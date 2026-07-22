//! Reference app for the Canton Rust SDK — the typed codegen (M2) and the Ledger
//! API client (M1) end to end, against the cn-quickstart licensing app.
//!
//! It builds a **typed** `AppInstallRequest` from the generated
//! `canton-quickstart-licensing` bindings, round-trips it through both codecs,
//! turns it into a Ledger API create command with `canton_daml::create_command`
//! (which resolves the template id in the upgrade-friendly `#<package-name>`
//! form), and — when the ledger environment variables are set — submits it to a
//! running participant (e.g. cn-quickstart LocalNet) and reads back the
//! committed transaction.
//!
//! Offline (no ledger) it demonstrates the codegen + codecs and always runs. Set
//! `LEDGER_ENDPOINT`, `LEDGER_TOKEN`, and `LEDGER_PARTY` to submit for real.

use canton_daml as rt;
use canton_ledger::{CantonClient, Config, Submit};
use canton_quickstart_licensing::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstallRequest;
use canton_quickstart_licensing::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata;

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // The party acts as both provider and user of the app-install request — the
    // shape the cn-quickstart app-provider can submit itself.
    let party =
        std::env::var("LEDGER_PARTY").unwrap_or_else(|_| "app_provider::example".to_string());
    let request = AppInstallRequest {
        provider: rt::Party::new(party.clone()),
        user: rt::Party::new(party.clone()),
        meta: Metadata {
            values: rt::TextMap::new(),
        },
    };

    demonstrate_codecs(&request)?;

    // Typed payload → Ledger API `CreateCommand` (M2 codegen + command builder).
    let command = rt::create_command(&request);
    let template_id = <AppInstallRequest as rt::Contract>::template_id();
    println!(
        "\ncreate command → template id {}:{}:{}",
        template_id.package_id, template_id.module_name, template_id.entity_name,
    );

    match ledger_config() {
        // `Box::pin`: the submit future is large (a whole gRPC call graph).
        Some(config) => Box::pin(submit(config, &party, command)).await?,
        None => println!(
            "\n(offline) set LEDGER_ENDPOINT, LEDGER_TOKEN, and LEDGER_PARTY to submit \
             this command to a running participant."
        ),
    }
    Ok(())
}

/// Show that a generated type round-trips through both wire codecs.
fn demonstrate_codecs(request: &AppInstallRequest) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(request)?;
    println!("AppInstallRequest as LF-JSON:\n{json}");
    let from_json: AppInstallRequest = serde_json::from_str(&json)?;
    assert_eq!(&from_json, request, "JSON round-trip");

    let value = rt::ToValue::to_value(request);
    let from_value = <AppInstallRequest as rt::FromValue>::from_value(&value)?;
    assert_eq!(&from_value, request, "gRPC Value round-trip");

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

/// Submit the command and read back the committed transaction.
async fn submit(
    config: Config,
    party: &str,
    command: canton_ledger::proto::Command,
) -> Result<(), Error> {
    let client = CantonClient::connect_lazy(config)?;
    let mut request = Submit::new(party).add_command(command);
    // Only override the user id when asked; the default matches the token's user.
    if let Ok(user_id) = std::env::var("LEDGER_USER") {
        request = request.with_user_id(user_id);
    }
    let transaction = client.submit_and_wait_for_transaction(request).await?;
    println!(
        "\nsubmitted — update id {} committed with {} event(s) at offset {}",
        transaction.update_id,
        transaction.events.len(),
        transaction.offset,
    );
    Ok(())
}
