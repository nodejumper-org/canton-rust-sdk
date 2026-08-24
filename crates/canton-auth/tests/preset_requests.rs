//! What each advertised provider preset actually puts on the wire.
//!
//! A preset's job is to produce its provider's *normal* client-credentials
//! request. The token URL is only part of that: Okta expects the credentials in
//! an `Authorization: Basic` header and rejects them in the body as
//! `invalid_client`, while Auth0 needs an `audience` or it issues a token for
//! its own userinfo endpoint that no participant will accept. Neither shows up
//! in an assertion about the URL.
//!
//! Each test builds the preset, carries its settings onto a local capture
//! server (a preset's own `https://` host cannot be pointed at one), and reads
//! the request off the socket.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use canton_auth::{ClientAuth, OidcConfig, TokenProvider};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpListener;

const SECRET: &str = "s";

/// A token endpoint that answers one request and hands back what it received.
async fn capture_endpoint() -> (String, Arc<Mutex<Option<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let seen = Arc::new(Mutex::new(None));
    let captured = seen.clone();
    tokio::spawn(async move {
        let Ok((mut socket, _)) = listener.accept().await else {
            return;
        };
        let mut buf = vec![0u8; 4096];
        let read = socket.read(&mut buf).await.unwrap_or(0);
        *captured.lock().unwrap() = Some(String::from_utf8_lossy(&buf[..read]).into_owned());
        let body = r#"{"access_token":"t","expires_in":300}"#;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = socket.write_all(response.as_bytes()).await;
        let _ = socket.shutdown().await;
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    (format!("http://localhost:{port}/token"), seen)
}

/// The preset's request, aimed at a local endpoint: same credential placement,
/// same audience, different host. If a preset stops setting one of those, the
/// request asserted below changes with it.
async fn request_of(preset: &OidcConfig) -> String {
    let (url, seen) = capture_endpoint().await;
    let mut config =
        OidcConfig::new(url, preset.client_id(), SECRET).with_client_auth(preset.client_auth());
    if let Some(audience) = preset.audience() {
        config = config.with_audience(audience);
    }
    TokenProvider::new(config)
        .token()
        .await
        .expect("the capture endpoint issues a token");
    let request = seen.lock().unwrap().clone();
    request.expect("the endpoint saw a request")
}

#[tokio::test]
async fn keycloak_sends_its_credentials_in_the_body() {
    let preset = OidcConfig::keycloak("http://kc:8082/", "AppProvider", "c", SECRET);
    assert_eq!(
        preset.token_url(),
        "http://kc:8082/realms/AppProvider/protocol/openid-connect/token"
    );
    assert_eq!(preset.client_auth(), ClientAuth::Post);

    let request = request_of(&preset).await;
    assert!(request.contains("client_id=c"), "{request}");
    assert!(request.contains("client_secret=s"), "{request}");
    assert!(
        !request.to_lowercase().contains("authorization:"),
        "the body form is not accompanied by a Basic header: {request}"
    );
}

#[tokio::test]
async fn auth0_asks_for_the_audience_it_was_given() {
    let preset = OidcConfig::auth0("my.eu.auth0.com", "https://ledger.example", "c", SECRET);
    assert_eq!(preset.token_url(), "https://my.eu.auth0.com/oauth/token");
    assert_eq!(preset.audience(), Some("https://ledger.example"));

    let request = request_of(&preset).await;
    assert!(
        request.contains("audience=https%3A%2F%2Fledger.example"),
        "Auth0's request carries the audience, url-encoded: {request}"
    );
    assert!(request.contains("client_secret=s"), "{request}");
}

#[tokio::test]
async fn okta_presents_its_credentials_as_basic_auth() {
    let preset = OidcConfig::okta("my.okta.com", "default", "c", SECRET);
    assert_eq!(
        preset.token_url(),
        "https://my.okta.com/oauth2/default/v1/token"
    );
    assert_eq!(preset.client_auth(), ClientAuth::Basic);

    let request = request_of(&preset).await;
    // base64("c:s") — spelled out rather than computed, so the test fails if
    // the header stops being what Okta reads.
    assert!(
        request.contains("authorization: Basic Yzpz")
            || request.contains("Authorization: Basic Yzpz"),
        "Okta's credentials belong in the header: {request}"
    );
    assert!(
        !request.contains("client_secret="),
        "and must not also be in the body: {request}"
    );
    assert!(
        request.contains("grant_type=client_credentials"),
        "{request}"
    );
}
