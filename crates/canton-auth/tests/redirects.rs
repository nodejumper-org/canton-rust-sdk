//! A token endpoint that redirects is refused: the request carries the client
//! secret, and following a 3xx would replay it at the redirect's address.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use canton_auth::{OidcConfig, TokenProvider};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpListener;

/// An endpoint answering one request with the given status line and headers,
/// recording what it received.
async fn endpoint(
    status: &'static str,
    extra_headers: String,
) -> (String, Arc<Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let captured = seen.clone();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 8192];
            let read = socket.read(&mut buf).await.unwrap_or(0);
            captured
                .lock()
                .unwrap()
                .push(String::from_utf8_lossy(&buf[..read]).into_owned());
            let body = r#"{"access_token":"attacker-token","expires_in":300}"#;
            let response = format!(
                "HTTP/1.1 {status}\r\n{extra_headers}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
        }
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    (format!("http://127.0.0.1:{port}"), seen)
}

#[tokio::test]
async fn a_redirecting_token_endpoint_is_refused_and_the_secret_stays_home() {
    let (collector, collected) = endpoint("200 OK", String::new()).await;
    let (issuer, seen) = endpoint(
        "307 Temporary Redirect",
        format!("Location: {collector}/collect\r\n"),
    )
    .await;

    let provider = TokenProvider::new(OidcConfig::new(
        format!("{issuer}/token"),
        "client-1",
        "TOP-SECRET",
    ));
    let err = provider.token().await.expect_err("a redirect is refused");

    assert!(
        matches!(err, canton_core::Error::Auth(_)),
        "not an auth error: {err:?}"
    );
    let message = err.to_string();
    assert!(message.contains("redirect"), "{message}");
    assert!(!message.contains("TOP-SECRET"), "{message}");
    // The issuer saw the credentials once; the redirect target saw nothing.
    assert_eq!(seen.lock().unwrap().len(), 1);
    assert!(seen.lock().unwrap()[0].contains("client_secret=TOP-SECRET"));
    assert!(
        collected.lock().unwrap().is_empty(),
        "the secret was forwarded"
    );
    // And no token was minted from the attacker's answer.
    assert!(!message.contains("attacker-token"));
}
