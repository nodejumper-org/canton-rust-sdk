//! Phase-A smoke test: connect to a running participant's gRPC Ledger API and
//! call `VersionService.GetLedgerApiVersion`. Proves the whole pipeline —
//! vendored protos -> generated client -> real call to the node — works.
//!
//! Run against LocalNet's App Provider participant:
//!   CANTON_TEST_ENDPOINT=http://localhost:3901 cargo run -p canton-proto --example smoke_version

use canton_proto::com::daml::ledger::api::v2::GetLedgerApiVersionRequest;
use canton_proto::com::daml::ledger::api::v2::version_service_client::VersionServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = std::env::var("CANTON_TEST_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:3901".to_string());

    println!("connecting to {endpoint} ...");
    // An `https://` endpoint (a DevNet validator) needs TLS with the system
    // roots; a LocalNet participant speaks plain HTTP/2.
    let mut channel = tonic::transport::Channel::from_shared(endpoint.clone())?;
    if endpoint.starts_with("https://") {
        channel =
            channel.tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())?;
    }
    let mut client = VersionServiceClient::new(channel.connect().await?);

    let response = client
        .get_ledger_api_version(GetLedgerApiVersionRequest {})
        .await?
        .into_inner();

    println!("OK — Ledger API version: {}", response.version);
    Ok(())
}
