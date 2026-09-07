//! Which packages a participant has, over the JSON Ledger API only — no gRPC,
//! no admin port. The check a JSON-only deployment wants before submitting a
//! command (issue #2).
//!
//! Run against LocalNet's App Provider participant with a bearer token:
//!   CANTON_JSON_ENDPOINT=http://localhost:3975 CANTON_TOKEN=… \
//!     cargo run -p canton-ledger --example json_packages
//!
//! Or with OIDC client credentials, as the live tests do:
//!   CANTON_JSON_ENDPOINT=http://localhost:3975 \
//!   CANTON_TEST_TOKEN_URL=http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token \
//!   CANTON_TEST_CLIENT_ID=app-provider-backend CANTON_TEST_CLIENT_SECRET=… \
//!     cargo run -p canton-ledger --example json_packages
//!
//! Pass a package id as the first argument to ask about that one instead of
//! the first listed.

use canton_ledger::JsonClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = std::env::var("CANTON_JSON_ENDPOINT")
        .or_else(|_| std::env::var("CANTON_TEST_JSON_ENDPOINT"))
        .unwrap_or_else(|_| "http://localhost:3975".to_string());

    let mut client = JsonClient::new(&endpoint);
    if let Ok(token) = std::env::var("CANTON_TOKEN") {
        client = client.with_token(token);
    } else if let (Ok(url), Ok(id), Ok(secret)) = (
        std::env::var("CANTON_TEST_TOKEN_URL"),
        std::env::var("CANTON_TEST_CLIENT_ID"),
        std::env::var("CANTON_TEST_CLIENT_SECRET"),
    ) {
        client = client.with_oidc(canton_auth::TokenProvider::new(
            canton_auth::OidcConfig::new(url, id, secret),
        ));
    }

    println!(
        "participant:    {endpoint} (JSON Ledger API {})",
        client.version().await?
    );

    let ids = client.list_packages().await?;
    println!("packages known: {}", ids.len());

    let Some(first) = ids.first() else {
        println!("nothing to ask about: the participant has no packages");
        return Ok(());
    };
    let asked = std::env::args().nth(1).unwrap_or_else(|| first.clone());
    let status = client.package_status(&asked).await?;
    println!("{asked}: {status:?}");
    Ok(())
}
