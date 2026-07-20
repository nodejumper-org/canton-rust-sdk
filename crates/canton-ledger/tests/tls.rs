//! End-to-end TLS + mutual-TLS tests against an in-process tonic gRPC server.
//!
//! Self-contained (no live node, no env): server stubs come from the dev-only
//! `canton-proto/server` feature, and certificates are generated with `rcgen`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use canton_ledger::{CantonClient, Config, TlsConfig};
use canton_proto::com::daml::ledger::api::v2 as pb;
use pb::version_service_server::{VersionService, VersionServiceServer};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

/// Minimal in-process `VersionService` that always answers `"test-version"`.
#[derive(Default)]
struct MockVersion;

#[tonic::async_trait]
impl VersionService for MockVersion {
    async fn get_ledger_api_version(
        &self,
        _request: Request<pb::GetLedgerApiVersionRequest>,
    ) -> Result<Response<pb::GetLedgerApiVersionResponse>, Status> {
        Ok(Response::new(pb::GetLedgerApiVersionResponse {
            version: "test-version".to_string(),
            ..Default::default()
        }))
    }
}

/// A self-signed cert for `localhost`; the cert doubles as its own CA/trust anchor.
struct Certs {
    cert_pem: String,
    key_pem: String,
}

fn gen_certs() -> Certs {
    let ck = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    Certs {
        cert_pem: ck.cert.pem(),
        key_pem: ck.key_pair.serialize_pem(),
    }
}

/// Start an in-process TLS `VersionService` and return its `https://localhost:PORT`.
/// When `require_client_ca` is `Some`, mutual TLS is enforced against that CA.
async fn start_tls_server(certs: &Certs, require_client_ca: Option<String>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let incoming = TcpListenerStream::new(listener);

    let mut server_tls = ServerTlsConfig::new().identity(Identity::from_pem(
        certs.cert_pem.clone(),
        certs.key_pem.clone(),
    ));
    if let Some(ca) = require_client_ca {
        server_tls = server_tls.client_ca_root(Certificate::from_pem(ca));
    }

    tokio::spawn(async move {
        Server::builder()
            .tls_config(server_tls)
            .unwrap()
            .serve_with_incoming(VersionServiceServer::new(MockVersion), incoming)
            .await
            .unwrap();
    });

    // Let the spawned server start accepting before the client connects.
    tokio::time::sleep(Duration::from_millis(200)).await;
    format!("https://localhost:{port}")
}

#[tokio::test]
async fn server_side_tls_succeeds_when_ca_is_trusted() {
    let certs = gen_certs();
    let url = start_tls_server(&certs, None).await;

    let client = CantonClient::connect_lazy(
        Config::new(url).with_tls(
            TlsConfig::new()
                .with_ca_certificate(certs.cert_pem.clone().into_bytes())
                .with_domain_name("localhost"),
        ),
    )
    .unwrap();

    let version = client.version().await.expect("TLS call should succeed");
    assert_eq!(version, "test-version");
}

#[tokio::test]
async fn server_side_tls_fails_when_server_cert_is_untrusted() {
    let certs = gen_certs();
    let url = start_tls_server(&certs, None).await;

    // https endpoint with no custom CA → native roots → self-signed is untrusted.
    let client = CantonClient::connect_lazy(Config::new(&url)).unwrap();
    let result = client.version().await;
    assert!(
        result.is_err(),
        "an untrusted self-signed server must be rejected, got {result:?}"
    );
}

#[tokio::test]
async fn mutual_tls_requires_a_client_identity() {
    let certs = gen_certs();
    let url = start_tls_server(&certs, Some(certs.cert_pem.clone())).await;

    // No client identity → the mTLS server rejects the handshake.
    let no_identity = CantonClient::connect_lazy(
        Config::new(&url).with_tls(
            TlsConfig::new()
                .with_ca_certificate(certs.cert_pem.clone().into_bytes())
                .with_domain_name("localhost"),
        ),
    )
    .unwrap();
    assert!(
        no_identity.version().await.is_err(),
        "an mTLS server must reject a client that presents no identity"
    );

    // With a client identity (trusted as the client CA) → the call succeeds.
    let with_identity = CantonClient::connect_lazy(
        Config::new(&url).with_tls(
            TlsConfig::new()
                .with_ca_certificate(certs.cert_pem.clone().into_bytes())
                .with_domain_name("localhost")
                .with_client_identity(
                    certs.cert_pem.clone().into_bytes(),
                    certs.key_pem.clone().into_bytes(),
                ),
        ),
    )
    .unwrap();
    let version = with_identity
        .version()
        .await
        .expect("mTLS with a client identity should succeed");
    assert_eq!(version, "test-version");
}
