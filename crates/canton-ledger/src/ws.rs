//! WebSocket streaming for the JSON transport (feature `ws`).
//!
//! Canton's JSON Ledger API streams over WebSocket. The client opens a socket
//! (bearer auth in an `Authorization` header, plus the required `daml.ws.auth`
//! subprotocol), sends exactly one subscription frame — the same JSON as the
//! equivalent HTTP `POST` body — then receives response frames. A bounded
//! request completes with a normal WS close (end-of-stream); a `JsCantonError`
//! frame ends the stream with `Err`.

use canton_core::{Auth, Error, Result, TlsConfig};
use futures_core::Stream;
use futures_util::{SinkExt as _, StreamExt as _};
use serde_json::Value;
use tokio_tungstenite::Connector;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::ClientRequestBuilder;
use tokio_tungstenite::tungstenite::http::Uri;

/// The subprotocol Canton requires on the JSON-API WebSocket handshake.
const WS_SUBPROTOCOL: &str = "daml.ws.auth";

/// Build the TLS connector for the WS handshake from the shared [`TlsConfig`]:
/// a custom CA (server-side TLS against a private/self-signed server) and/or a
/// client identity (mutual TLS). `None` config keeps tungstenite's default
/// (native roots for `wss`).
pub(crate) fn build_connector(tls: Option<&TlsConfig>) -> Result<Option<Connector>> {
    use rustls_pki_types::pem::PemObject as _;
    use rustls_pki_types::{CertificateDer, PrivateKeyDer};

    let Some(tls) = tls else { return Ok(None) };

    // Root store: the custom CA when given, else the platform's native roots.
    let mut roots = rustls::RootCertStore::empty();
    match &tls.ca_certificate_pem {
        Some(ca) => {
            for cert in CertificateDer::pem_slice_iter(ca) {
                let cert = cert
                    .map_err(|e| Error::InvalidRequest(format!("invalid CA certificate: {e}")))?;
                roots
                    .add(cert)
                    .map_err(|e| Error::InvalidRequest(format!("rejected CA certificate: {e}")))?;
            }
            if roots.is_empty() {
                return Err(Error::InvalidRequest(
                    "the CA PEM contained no certificates".to_string(),
                ));
            }
        }
        None => {
            // Best-effort like tungstenite's own default: unloadable
            // individual certs are skipped.
            for cert in rustls_native_certs::load_native_certs().certs {
                let _ = roots.add(cert);
            }
        }
    }

    // Pin the ring provider explicitly: several rustls crypto backends may be
    // enabled across the dependency graph, which would make the implicit
    // default ambiguous.
    let builder = rustls::ClientConfig::builder_with_provider(
        rustls::crypto::ring::default_provider().into(),
    )
    .with_safe_default_protocol_versions()
    .map_err(|e| Error::InvalidRequest(format!("tls protocol setup failed: {e}")))?
    .with_root_certificates(roots);

    let config = match &tls.client_identity_pem {
        Some((cert_pem, key_pem)) => {
            let certs = CertificateDer::pem_slice_iter(cert_pem)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|e| Error::InvalidRequest(format!("invalid client certificate: {e}")))?;
            let key = PrivateKeyDer::from_pem_slice(key_pem)
                .map_err(|e| Error::InvalidRequest(format!("invalid client key: {e}")))?;
            builder
                .with_client_auth_cert(certs, key)
                .map_err(|e| Error::InvalidRequest(format!("invalid client identity: {e}")))?
        }
        None => builder.with_no_client_auth(),
    };

    Ok(Some(Connector::Rustls(std::sync::Arc::new(config))))
}

/// Rewrite an `http(s)` base URL to its `ws(s)` form.
fn ws_url(base_url: &str, path: &str) -> String {
    let base = base_url.trim_end_matches('/');
    let base = if let Some(rest) = base.strip_prefix("https://") {
        format!("wss://{rest}")
    } else if let Some(rest) = base.strip_prefix("http://") {
        format!("ws://{rest}")
    } else {
        base.to_string()
    };
    format!("{base}{path}")
}

/// True if a frame is a `JsCantonError` rather than a success payload. Success
/// frames (`{"update": …}` / `{"contractEntry": …}`) never carry `errorCategory`.
fn is_error_frame(value: &Value) -> bool {
    value.get("errorCategory").is_some() && value.get("code").is_some()
}

/// Map a `JsCantonError` frame to an [`Error`].
fn error_frame(value: &Value) -> Error {
    let code = value
        .get("code")
        .and_then(Value::as_str)
        .unwrap_or("UNKNOWN")
        .to_string();
    let message = value
        .get("cause")
        .and_then(Value::as_str)
        .unwrap_or("ws stream error")
        .to_string();
    Error::CommandRejected { code, message }
}

/// The ledger offset carried by an update frame (transaction / reassignment /
/// topology / checkpoint), for resumable-stream position tracking. Every tag
/// wraps its payload as `{"<Tag>": {"value": {"offset": N}}}`.
pub(crate) fn update_offset(value: &Value) -> Option<i64> {
    value
        .get("update")?
        .as_object()?
        .values()
        .next()?
        .get("value")?
        .get("offset")?
        .as_i64()
}

/// True if a frame is an `OffsetCheckpoint` heartbeat (in either the update or
/// completion envelope) rather than a real update/completion.
pub(crate) fn is_offset_checkpoint(value: &Value) -> bool {
    ["update", "completionResponse"].iter().any(|key| {
        value
            .get(key)
            .and_then(Value::as_object)
            .is_some_and(|obj| obj.contains_key("OffsetCheckpoint"))
    })
}

/// Drop `OffsetCheckpoint` heartbeat frames from a stream (matching the gRPC
/// client's `updates`/`completions`, which surface only real items).
pub(crate) fn filter_checkpoints(
    inner: impl Stream<Item = Result<Value>> + Send,
) -> impl Stream<Item = Result<Value>> + Send {
    async_stream::try_stream! {
        tokio::pin!(inner);
        while let Some(item) = inner.next().await {
            let frame = item?;
            if !is_offset_checkpoint(&frame) {
                yield frame;
            }
        }
    }
}

/// Open a WS subscription at `path` with `request` as the single subscription
/// frame, and yield each response frame as JSON.
///
/// # Errors
/// Returns an [`Error`] if the URL is invalid, auth fails, or the handshake
/// fails. The returned stream yields `Err` on a participant error frame or a
/// transport failure.
pub(crate) async fn subscribe(
    base_url: &str,
    auth: &Auth,
    tls: Option<&TlsConfig>,
    path: &str,
    request: Value,
) -> Result<impl Stream<Item = Result<Value>> + Send + use<>> {
    let url = ws_url(base_url, path);
    let uri: Uri = url
        .parse()
        .map_err(|e| Error::InvalidRequest(format!("invalid ws url {url}: {e}")))?;

    let mut builder = ClientRequestBuilder::new(uri).with_sub_protocol(WS_SUBPROTOCOL);
    if let Some(token) = auth.bearer().await? {
        builder = builder.with_header("Authorization", format!("Bearer {token}"));
    }

    let connector = build_connector(tls)?;
    let (mut socket, _response) =
        tokio_tungstenite::connect_async_tls_with_config(builder, None, false, connector)
            .await
            .map_err(|e| Error::Connection(format!("ws connect to {url} failed: {e}")))?;

    // A single subscription frame (same JSON as the equivalent HTTP POST body).
    socket
        .send(Message::text(request.to_string()))
        .await
        .map_err(|e| Error::Connection(format!("ws send to {url} failed: {e}")))?;

    Ok(async_stream::try_stream! {
        while let Some(message) = socket.next().await {
            let message = message.map_err(|e| Error::Connection(format!("ws recv failed: {e}")))?;
            match message {
                Message::Text(text) => {
                    let value: Value = serde_json::from_str(text.as_str()).map_err(Error::from)?;
                    if is_error_frame(&value) {
                        Err(error_frame(&value))?;
                    }
                    yield value;
                }
                // A normal close ends the stream (bounded requests close when done).
                Message::Close(_) => break,
                _ => {} // ping / pong / binary / raw frame — ignore
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_url_rewrites_the_scheme() {
        assert_eq!(
            ws_url("http://localhost:3975", "/v2/updates"),
            "ws://localhost:3975/v2/updates"
        );
        assert_eq!(
            ws_url("https://host/", "/v2/state/active-contracts"),
            "wss://host/v2/state/active-contracts"
        );
        // An already-ws(s) or unknown scheme passes through unchanged.
        assert_eq!(ws_url("ws://host:9", "/p"), "ws://host:9/p");
    }

    #[test]
    fn error_frames_are_distinguished_from_success() {
        let error = serde_json::json!({
            "code": "JSON_API_X", "cause": "boom", "errorCategory": 2, "grpcCodeValue": 7
        });
        assert!(is_error_frame(&error));
        match error_frame(&error) {
            Error::CommandRejected { code, message } => {
                assert_eq!(code, "JSON_API_X");
                assert_eq!(message, "boom");
            }
            other => panic!("expected CommandRejected, got {other:?}"),
        }

        let success = serde_json::json!({ "update": { "Transaction": {} } });
        assert!(!is_error_frame(&success));
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn build_connector_honours_the_tls_config() {
        // No TLS config → default connector (native roots), no error.
        assert!(build_connector(None).unwrap().is_none());

        // Custom CA + client identity (mTLS) → a rustls connector.
        let ck = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let cert_pem = ck.cert.pem().into_bytes();
        let key_pem = ck.key_pair.serialize_pem().into_bytes();
        let tls = TlsConfig::new()
            .with_ca_certificate(cert_pem.clone())
            .with_client_identity(cert_pem, key_pem);
        assert!(matches!(
            build_connector(Some(&tls)).unwrap(),
            Some(Connector::Rustls(_))
        ));

        // A CA PEM with no certificates is rejected, not silently trusted.
        let empty_ca = TlsConfig::new().with_ca_certificate(b"not a pem".to_vec());
        assert!(matches!(
            build_connector(Some(&empty_ca)),
            Err(Error::InvalidRequest(_))
        ));

        // A garbage client key is rejected as an InvalidRequest.
        let ck2 = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let bad_identity = TlsConfig::new()
            .with_client_identity(ck2.cert.pem().into_bytes(), b"not a key".to_vec());
        assert!(matches!(
            build_connector(Some(&bad_identity)),
            Err(Error::InvalidRequest(_))
        ));
    }

    #[test]
    fn update_offset_reads_any_tag() {
        let tx = serde_json::json!({ "update": { "Transaction": { "value": { "offset": 42 } } } });
        assert_eq!(update_offset(&tx), Some(42));
        let cp =
            serde_json::json!({ "update": { "OffsetCheckpoint": { "value": { "offset": 7 } } } });
        assert_eq!(update_offset(&cp), Some(7));
        let acs = serde_json::json!({ "contractEntry": {} });
        assert_eq!(update_offset(&acs), None);
    }

    #[test]
    fn offset_checkpoints_are_recognized_in_both_envelopes() {
        let update_cp = serde_json::json!({ "update": { "OffsetCheckpoint": { "value": {} } } });
        let completion_cp =
            serde_json::json!({ "completionResponse": { "OffsetCheckpoint": { "value": {} } } });
        let real = serde_json::json!({ "update": { "Transaction": { "value": {} } } });
        assert!(is_offset_checkpoint(&update_cp));
        assert!(is_offset_checkpoint(&completion_cp));
        assert!(!is_offset_checkpoint(&real));
    }
}
