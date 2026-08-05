//! Quickstart with OIDC auth: submit an `AppInstallRequest` create and read the
//! resulting transaction back.
//!
//! Reads the same variables as the live tests, so one export set runs both:
//!   CANTON_TEST_ENDPOINT=http://localhost:3901 \
//!   CANTON_TEST_TOKEN_URL=http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token \
//!   CANTON_TEST_CLIENT_ID=app-provider-backend CANTON_TEST_CLIENT_SECRET=… \
//!   CANTON_TEST_PARTY=… CANTON_TEST_LICENSING_PKG='#quickstart-licensing' \
//!     cargo run -p canton-ledger --example submit_and_read
//!
//! The unprefixed spellings (`CANTON_ENDPOINT`, …) are accepted too and win
//! when both are set.

use canton_auth::{OidcConfig, TokenProvider};
use canton_ledger::{CantonClient, Config, Error, Submit, create, identifier, record, value};

/// A setting, from `CANTON_<NAME>` or the live tests' `CANTON_TEST_<NAME>`.
/// Two spellings for one value is a papercut for anyone following the README,
/// so the example answers to both rather than making them guess which.
fn env(name: &str) -> canton_ledger::Result<String> {
    let plain = format!("CANTON_{name}");
    let test = format!("CANTON_TEST_{name}");
    std::env::var(&plain)
        .or_else(|_| std::env::var(&test))
        .map_err(|_| Error::InvalidRequest(format!("set {plain} (or {test})")))
}

#[tokio::main]
async fn main() -> canton_ledger::Result<()> {
    let oidc = OidcConfig::new(env("TOKEN_URL")?, env("CLIENT_ID")?, env("CLIENT_SECRET")?);
    let party = env("PARTY")?;
    let pkg = env("LICENSING_PKG")?;

    let client = CantonClient::connect_lazy(
        Config::new(env("ENDPOINT")?).with_oidc(TokenProvider::new(oidc)),
    )?;

    // Build an AppInstallRequest create acting as `party`.
    let arguments = record(vec![
        ("provider", value::party(&party)),
        ("user", value::party(&party)),
        (
            "meta",
            value::record(record(vec![("values", value::empty_text_map())])),
        ),
    ]);
    let command = create(
        identifier(&pkg, "Licensing.AppInstall", "AppInstallRequest"),
        arguments,
    );

    let transaction = client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(command))
        .await?;

    println!(
        "committed transaction {} at offset {} with {} event(s)",
        transaction.update_id,
        transaction.offset,
        transaction.events.len()
    );
    Ok(())
}
