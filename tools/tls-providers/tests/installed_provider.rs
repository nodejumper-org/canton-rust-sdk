//! An application that compiles in both rustls crypto providers and installs
//! one for the process. The SDK uses the installed one rather than putting
//! its own `ring` beside it, as its gRPC and HTTP transports already do.

use std::time::Duration;

use canton_ledger::{Error, JsonClient};
use canton_pqs::PqsClient;

#[tokio::test]
async fn tls_setup_uses_the_provider_the_application_installed() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("nothing was installed before this");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a local port");
    let address = listener.local_addr().expect("the local address");
    tokio::spawn(async move {
        while let Ok((socket, _)) = listener.accept().await {
            drop(socket);
        }
    });

    let client =
        JsonClient::new(format!("https://{address}")).with_timeout(Duration::from_secs(10));
    match client.ws_updates(vec!["alice".to_string()], 0, None).await {
        Ok(_) => panic!("a server that hangs up completed a TLS handshake"),
        Err(error) => assert!(matches!(error, Error::Connection(_)), "{error:?}"),
    }

    let config = format!("host=127.0.0.1 port={} user=pqs dbname=pqs", address.port());
    assert!(
        PqsClient::connect_tls_with_timeout(&config, Duration::from_secs(10))
            .await
            .is_err()
    );
}
