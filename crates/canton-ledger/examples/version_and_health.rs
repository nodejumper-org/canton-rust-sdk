//! Minimal quickstart: connect to a participant's gRPC Ledger API and print its
//! version and health. No authentication required.
//!
//! Run against LocalNet's App Provider participant:
//!   CANTON_ENDPOINT=http://localhost:3901 \
//!     cargo run -p canton-ledger --example version_and_health

use canton_ledger::{CantonClient, Config};

#[tokio::main]
async fn main() -> canton_ledger::Result<()> {
    let endpoint =
        std::env::var("CANTON_ENDPOINT").unwrap_or_else(|_| "http://localhost:3901".to_string());

    // `connect_lazy` returns immediately; the connection opens on the first RPC.
    let client = CantonClient::connect_lazy(Config::new(endpoint))?;

    println!("ledger api version: {}", client.version().await?);
    println!("node health:        {:?}", client.health_check().await?);
    Ok(())
}
