//! The WebSocket handshake carries the caller's trace context.
//!
//! A WebSocket makes exactly one request — the upgrade — so that request is the
//! only place trace context can be attached. Miss it and the streaming JSON
//! lane sits outside the trace every other call in the SDK joins, which is
//! precisely where a long-lived subscription's failures live.
//!
//! The server here captures the real upgrade request the client sends, opened
//! inside an active OpenTelemetry span.
#![cfg(all(feature = "ws", feature = "otel"))]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::large_futures,
    clippy::result_large_err
)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use canton_ledger::JsonClient;
use opentelemetry::trace::TracerProvider as _;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt as _;
use tracing_subscriber::layer::SubscriberExt as _;

type Headers = Arc<Mutex<Vec<(String, String)>>>;

/// Accept one handshake, record its headers, then close the socket.
async fn capture_upgrade(stream: TcpStream, headers: Headers) {
    use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
    use tokio_tungstenite::tungstenite::http::HeaderValue;

    let socket =
        tokio_tungstenite::accept_hdr_async(stream, |req: &Request, mut resp: Response| {
            *headers.lock().unwrap() = req
                .headers()
                .iter()
                .map(|(name, value)| {
                    (
                        name.as_str().to_ascii_lowercase(),
                        value.to_str().unwrap_or_default().to_string(),
                    )
                })
                .collect();
            resp.headers_mut().insert(
                "Sec-WebSocket-Protocol",
                HeaderValue::from_static("daml.ws.auth"),
            );
            Ok(resp)
        })
        .await;
    drop(socket);
}

#[tokio::test(flavor = "multi_thread")]
async fn the_websocket_handshake_carries_the_callers_trace_context() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let headers: Headers = Arc::new(Mutex::new(Vec::new()));
    let captured = headers.clone();
    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            capture_upgrade(stream, captured).await;
        }
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    // A real tracer, so there is a real trace context to propagate. Nothing is
    // exported; the span only has to exist and be sampled.
    let provider = opentelemetry_sdk::trace::TracerProvider::builder().build();
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("ws-trace-test")));
    let _guard = tracing::subscriber::set_default(subscriber);

    let client = JsonClient::new(format!("http://127.0.0.1:{port}")).with_token("t");
    let root = tracing::info_span!("caller");
    let _entered = root.enter();
    // The stream is expected to fail — the server closes right after the
    // handshake. The handshake is what this test is about.
    if let Ok(stream) = client.ws_updates(vec!["alice".to_string()], 0, None).await {
        tokio::pin!(stream);
        let _ = tokio::time::timeout(Duration::from_millis(500), stream.next()).await;
    }

    let headers = headers.lock().unwrap().clone();
    assert!(
        !headers.is_empty(),
        "the server should have seen an upgrade request"
    );
    let traceparent = headers
        .iter()
        .find(|(name, _)| name == "traceparent")
        .map(|(_, value)| value.clone());
    let traceparent = traceparent.expect(
        "the upgrade request must carry W3C trace context; \
         a WebSocket has no other request to attach it to",
    );
    // `00-<32 hex trace id>-<16 hex span id>-<flags>`
    let parts: Vec<&str> = traceparent.split('-').collect();
    assert_eq!(parts.len(), 4, "malformed traceparent: {traceparent}");
    assert_eq!(parts[1].len(), 32, "trace id: {traceparent}");
    assert_eq!(parts[2].len(), 16, "span id: {traceparent}");
    assert!(
        headers.iter().any(|(name, _)| name == "authorization"),
        "and the credentials the handshake already carried are still there"
    );
}
