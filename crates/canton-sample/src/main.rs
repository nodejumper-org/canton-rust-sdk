//! Reference app for the Canton Rust SDK — the typed codegen (M2) and the Ledger
//! API client (M1) end to end.
//!
//! It builds a **typed** `FeaturedAppRight` from the generated
//! `canton-splice-amulet` bindings, turns it into a Ledger API create command
//! with `canton_daml::create_command` (which resolves the template id in the
//! upgrade-friendly `#<package-name>` form), and round-trips it through both
//! codecs. If the ledger environment variables are set it also submits the
//! command to a running participant (e.g. LocalNet) and reads it back from the
//! ACS.
//!
//! Offline (no ledger) it demonstrates the codegen + codecs and always runs.
//! Set `LEDGER_ENDPOINT`, `LEDGER_TOKEN`, and `LEDGER_PARTY` to submit for real.

use canton_daml as rt;
use canton_ledger::{CantonClient, Config, Submit};
use canton_splice_amulet::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppRight;

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // A typed contract payload from the generated Splice bindings. The provider
    // party would normally be the app operator; the DSO is the Amulet operator.
    let party = std::env::var("LEDGER_PARTY").unwrap_or_else(|_| "provider::example".to_string());
    let right = FeaturedAppRight {
        dso: rt::Party::new("DSO::example"),
        provider: rt::Party::new(party.clone()),
    };

    demonstrate_codecs(&right)?;

    // Typed payload → Ledger API `CreateCommand` (M2 codegen + command builder).
    let command = rt::create_command(&right);
    let template_id = <FeaturedAppRight as rt::Contract>::template_id();
    println!(
        "\ncreate command → template id {}:{}:{} (upgrade-friendly package-name form)",
        template_id.package_id, template_id.module_name, template_id.entity_name,
    );

    match ledger_config() {
        Some(config) => submit(config, &party, command).await?,
        None => println!(
            "\n(offline) set LEDGER_ENDPOINT, LEDGER_TOKEN, and LEDGER_PARTY to submit \
             this command to a running participant."
        ),
    }
    Ok(())
}

/// Show that a generated type round-trips through both wire codecs.
fn demonstrate_codecs(right: &FeaturedAppRight) -> Result<(), Error> {
    // JSON (Ledger API JSON) — the Daml labels are the keys.
    let json = serde_json::to_string_pretty(right)?;
    println!("FeaturedAppRight as LF-JSON:\n{json}");
    let from_json: FeaturedAppRight = serde_json::from_str(&json)?;
    assert_eq!(&from_json, right, "JSON round-trip");

    // gRPC (Ledger API protobuf `Value`).
    let value = rt::ToValue::to_value(right);
    let from_value = <FeaturedAppRight as rt::FromValue>::from_value(&value)?;
    assert_eq!(&from_value, right, "gRPC Value round-trip");

    println!("codec round-trip OK (JSON + gRPC)");
    Ok(())
}

/// The ledger connection config, if the environment provides an endpoint.
fn ledger_config() -> Option<Config> {
    let endpoint = std::env::var("LEDGER_ENDPOINT").ok()?;
    let mut config = Config::new(endpoint);
    if let Ok(token) = std::env::var("LEDGER_TOKEN") {
        config = config.with_token(token);
    }
    Some(config)
}

/// Submit the command to the participant and report the update id.
async fn submit(
    config: Config,
    party: &str,
    command: canton_ledger::proto::Command,
) -> Result<(), Error> {
    let client = CantonClient::connect_lazy(config)?;
    let user_id = std::env::var("LEDGER_USER").unwrap_or_else(|_| "canton-sample".to_string());
    let request = Submit::new(party)
        .add_command(command)
        .with_user_id(user_id);
    // `submit` submits and returns the update id; the ACS/updates streams on
    // `CantonClient` (e.g. `active_contracts`) read the created contract back.
    let update_id = client.submit(request).await?;
    println!("\nsubmitted — update id: {update_id}");
    Ok(())
}
