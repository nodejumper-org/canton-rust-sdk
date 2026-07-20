//! Quickstart with OIDC auth: submit an `AppInstallRequest` create and read the
//! resulting transaction back. Env-driven (mirrors the live-test setup).
//!
//! Run against LocalNet:
//!   CANTON_ENDPOINT=http://localhost:3901 \
//!   CANTON_TOKEN_URL=http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token \
//!   CANTON_CLIENT_ID=app-provider-backend CANTON_CLIENT_SECRET=… \
//!   CANTON_PARTY=… CANTON_LICENSING_PKG=… \
//!     cargo run -p canton-ledger --example submit_and_read

use canton_auth::{OidcConfig, TokenProvider};
use canton_ledger::{CantonClient, Config, Error, Submit, create, identifier, record, value};

fn env(key: &str) -> canton_ledger::Result<String> {
    std::env::var(key).map_err(|_| Error::InvalidRequest(format!("set {key}")))
}

#[tokio::main]
async fn main() -> canton_ledger::Result<()> {
    let oidc = OidcConfig::new(
        env("CANTON_TOKEN_URL")?,
        env("CANTON_CLIENT_ID")?,
        env("CANTON_CLIENT_SECRET")?,
    );
    let party = env("CANTON_PARTY")?;
    let pkg = env("CANTON_LICENSING_PKG")?;

    let client = CantonClient::connect_lazy(
        Config::new(env("CANTON_ENDPOINT")?).with_oidc(TokenProvider::new(oidc)),
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
