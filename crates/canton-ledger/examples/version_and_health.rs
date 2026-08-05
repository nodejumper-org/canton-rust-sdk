//! Minimal quickstart: connect to a participant's gRPC Ledger API and print its
//! version and health. No authentication required.
//!
//! Run against LocalNet's App Provider participant:
//!   CANTON_ENDPOINT=http://localhost:3901 \
//!     cargo run -p canton-ledger --example version_and_health
//!
//! `CANTON_TEST_ENDPOINT` (the live tests' name) works too, and the default is
//! `http://localhost:3901`, so on LocalNet neither is needed.

use canton_ledger::{CantonClient, Config};

#[tokio::main]
async fn main() -> canton_ledger::Result<()> {
    let endpoint = std::env::var("CANTON_ENDPOINT")
        .or_else(|_| std::env::var("CANTON_TEST_ENDPOINT"))
        .unwrap_or_else(|_| "http://localhost:3901".to_string());

    // `connect_lazy` returns immediately; the connection opens on the first RPC.
    let client = CantonClient::connect_lazy(Config::new(endpoint))?;

    println!("ledger api version: {}", client.version().await?);
    println!("node health:        {:?}", client.health_check().await?);
    Ok(())
}
