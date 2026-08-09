//! Talk to a local development network with no configuration in the program.
//!
//! Start a network and export it, then run this:
//!
//! ```text
//! canton-devkit localnet up demo          # or: dpm localnet up demo
//! eval "$(canton-devkit localnet env demo)"
//! cargo run -p canton-ledger --example localnet
//! ```
//!
//! Everything below comes out of the environment `localnet env` set: the
//! endpoints, the tokens, and the party ids. Nothing here names a host, a port
//! or a credential, which is the point — the same binary runs against a
//! colleague's instance, or a second instance on a different port base, with no
//! edit.
//!
//! It also works against any environment that sets the same variables, and
//! `CANTON_ENDPOINT` / `CANTON_TOKEN` override them for one that does not.

use canton_core::{Config, localnet};
use canton_ledger::{CantonClient, JsonClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let (Some(instance), Some(version)) = (localnet::instance(), localnet::splice_version()) {
        println!("network `{instance}`, Splice {version}");
    }

    // The gRPC lane. One line, and the vhost hostname and scheme-less
    // `host:port` the exporter emits are handled on the way through.
    let grpc = CantonClient::connect_lazy(Config::from_env()?)?;
    println!(
        "gRPC   {}  ->  Ledger API {}",
        endpoint()?,
        grpc.version().await?
    );

    // The JSON lane reads the same environment.
    let json = JsonClient::from_env()?;
    println!("JSON   ->  Ledger API {}", json.version().await?);

    // Both participants are exported, so a test can drive two sides of a
    // workflow without a second configuration file.
    match Config::from_env_for("app-user") {
        Ok(config) => {
            let user = CantonClient::connect_lazy(config)?;
            println!("app-user  ->  Ledger API {}", user.version().await?);
        }
        Err(_) => println!("app-user  ->  not exported by this network"),
    }

    // Party ids, which is what an application actually needs to submit. These
    // are the on-ledger ids, not the Ledger API user names.
    for alias in ["app-provider", "app-user", "sv"] {
        match localnet::party(alias) {
            Some(party) => println!("party {alias:<13} {party}"),
            None => println!("party {alias:<13} (not resolved yet)"),
        }
    }

    let offset = grpc.ledger_end().await?;
    println!("ledger end at offset {offset}");
    Ok(())
}

/// Only for the log line above — the client does not need to be asked.
fn endpoint() -> Result<String, canton_core::Error> {
    localnet::grpc_endpoint(None).ok_or_else(|| {
        canton_core::Error::InvalidRequest("no endpoint in the environment".to_string())
    })
}
