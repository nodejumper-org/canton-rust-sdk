//! The provider the application installed is the one the SDK uses.
//!
//! Which provider a handshake ran on cannot be read back from a connection,
//! so the one installed here cannot be mistaken for `ring`: it has no cipher
//! suites. A TLS setup that uses it fails to configure; one that reached for
//! `ring` regardless would get as far as the socket.
//!
//! PQS only. The JSON client cannot be built at all in this process, since
//! `reqwest` takes the installed provider as well and panics on one it
//! cannot use, which is the same fact shown for the HTTP lane.

use std::time::Duration;

use canton_ledger::Error;
use canton_pqs::PqsClient;

#[tokio::test]
async fn pqs_tls_is_configured_with_the_installed_provider() {
    let unusable = rustls::crypto::CryptoProvider {
        cipher_suites: Vec::new(),
        ..rustls::crypto::aws_lc_rs::default_provider()
    };
    unusable
        .install_default()
        .expect("nothing was installed before this");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a local port");
    let port = listener.local_addr().expect("the local address").port();
    tokio::spawn(async move {
        while let Ok((socket, _)) = listener.accept().await {
            drop(socket);
        }
    });

    let config = format!("host=127.0.0.1 port={port} user=pqs dbname=pqs");
    let Err(error) = PqsClient::connect_tls_with_timeout(&config, Duration::from_secs(10)).await
    else {
        panic!("a server that hangs up answered as a Postgres store");
    };
    assert!(
        !error.to_string().contains("no native root certificates"),
        "this machine has no root certificates, so the TLS setup was never reached: {error}"
    );
    assert!(
        matches!(error, Error::InvalidRequest(_))
            && error.to_string().contains("tls protocol setup"),
        "the installed provider was not the one used: {error:?}"
    );
}
