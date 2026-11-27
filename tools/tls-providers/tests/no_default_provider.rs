//! An application that compiles in both rustls crypto providers and installs
//! neither.
//!
//! That is an ordinary application: the SDK asks rustls for `ring`, and
//! `aws-lc-rs` is a rustls default feature, so any other dependency that takes
//! rustls's defaults adds it. In such a process `rustls::ClientConfig::builder()`
//! panics, since it cannot tell which provider is meant. The SDK must therefore
//! never call it, directly or through a dependency left to its defaults.

use std::net::SocketAddr;
use std::time::Duration;

use canton_ledger::{Error, JsonClient};
use canton_pqs::PqsClient;

/// The condition under test holds in this process: both providers are
/// compiled in, none is installed, and rustls on its own cannot choose.
fn rustls_cannot_choose_a_provider() {
    let _ = rustls::crypto::ring::default_provider();
    let _ = rustls::crypto::aws_lc_rs::default_provider();
    assert!(
        rustls::crypto::CryptoProvider::get_default().is_none(),
        "a crypto provider is installed, so this test proves nothing"
    );
    let chosen = std::panic::catch_unwind(|| {
        let _ = rustls::ClientConfig::builder();
    });
    assert!(
        chosen.is_err(),
        "rustls chose a provider by itself, so this test proves nothing"
    );
}

/// A server that accepts a connection and hangs up. Far enough for a client
/// to get to its TLS setup, which is where the panic was: tungstenite builds
/// its connector only once the socket is open.
async fn a_server_that_hangs_up() -> SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a local port");
    let address = listener.local_addr().expect("the local address");
    tokio::spawn(async move {
        while let Ok((socket, _)) = listener.accept().await {
            drop(socket);
        }
    });
    address
}

#[tokio::test]
async fn a_wss_stream_with_no_tls_settings_fails_without_panicking() {
    rustls_cannot_choose_a_provider();
    let address = a_server_that_hangs_up().await;
    let client =
        JsonClient::new(format!("https://{address}")).with_timeout(Duration::from_secs(10));

    match client.ws_updates(vec!["alice".to_string()], 0, None).await {
        Ok(_) => panic!("a server that hangs up completed a TLS handshake"),
        // Without root certificates the client stops before any TLS setup,
        // on the old code as on the new, and the run would pass without
        // having reached the code it is about.
        Err(error) => assert!(
            matches!(error, Error::Connection(_)),
            "the TLS setup was not reached, or failed for another reason: {error:?}"
        ),
    }
    match client.ws_completions(vec!["alice".to_string()], 0).await {
        Ok(_) => panic!("a server that hangs up completed a TLS handshake"),
        Err(error) => assert!(matches!(error, Error::Connection(_)), "{error:?}"),
    }
    assert!(
        rustls::crypto::CryptoProvider::get_default().is_none(),
        "the SDK installed a process-wide provider, which is the application's to choose"
    );
}

#[tokio::test]
async fn a_pqs_tls_connection_fails_without_panicking() {
    rustls_cannot_choose_a_provider();
    let address = a_server_that_hangs_up().await;
    let config = format!("host=127.0.0.1 port={} user=pqs dbname=pqs", address.port());

    let Err(error) = PqsClient::connect_tls_with_timeout(&config, Duration::from_secs(10)).await
    else {
        panic!("a server that hangs up answered as a Postgres store");
    };
    // Without root certificates the client stops before any TLS setup, and
    // the run would pass without having reached the code it is about.
    assert!(
        !error.to_string().contains("no native root certificates"),
        "this machine has no root certificates, so the TLS setup was never reached: {error}"
    );
    assert!(
        rustls::crypto::CryptoProvider::get_default().is_none(),
        "the SDK installed a process-wide provider, which is the application's to choose"
    );
}
