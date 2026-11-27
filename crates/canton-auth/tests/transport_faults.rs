//! What a token fetch that never got an answer is reported as.
//!
//! The verdict is read off `reqwest`'s error text, since it exposes no typed
//! TLS error, so these go through a real failed request rather than through
//! the strings: what the text says is exactly what is being tested.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_auth::{OidcConfig, TokenProvider};
use canton_core::Error;

/// A refused connection is a transport fault, and retriable, whatever the
/// token URL happens to spell. `reqwest` prints the URL into its own message,
/// and "certificate" there is not a certificate failure.
#[tokio::test]
async fn a_refused_connection_is_retriable_whatever_the_token_url_spells() {
    for url in [
        "http://127.0.0.1:1/token",
        "http://127.0.0.1:1/realms/certificate-service/protocol/openid-connect/token",
    ] {
        let provider = TokenProvider::new(OidcConfig::new(url, "client", "secret"));
        let error = provider.token().await.expect_err("nothing listens there");
        assert!(matches!(error, Error::Connection(_)), "{url}: {error:?}");
        assert!(error.is_retriable(), "{url}: {error}");
    }
}

/// A peer that accepts the connection and hangs up before the TLS handshake
/// completes is restarting, or shedding connections. That passes, so it is
/// not reported as a certificate that cannot be verified, which never does.
#[tokio::test]
async fn a_peer_that_hangs_up_during_the_tls_handshake_is_retriable() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a local port");
    let port = listener.local_addr().expect("the local address").port();
    tokio::spawn(async move {
        while let Ok((socket, _)) = listener.accept().await {
            drop(socket);
        }
    });

    let provider = TokenProvider::new(OidcConfig::new(
        format!("https://127.0.0.1:{port}/token"),
        "client",
        "secret",
    ));
    let error = provider.token().await.expect_err("the peer hung up");
    assert!(matches!(error, Error::Connection(_)), "{error:?}");
    assert!(error.is_retriable(), "{error}");
}
