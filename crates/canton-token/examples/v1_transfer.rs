//! A CIP-56 transfer, end to end.
//!
//! Shows the whole shape: ask the registry who administers the instrument,
//! build the transfer from *generated* types, let the registry resolve the
//! factory and supply the choice context, and submit with the contracts it
//! said to disclose.
//!
//! ```sh
//! CANTON_TEST_ENDPOINT=http://localhost:3901 \
//! CANTON_TOKEN_REGISTRY_URL=https://scan.example.com \
//! CANTON_TOKEN_SENDER='alice::1220…' \
//! CANTON_TOKEN_RECEIVER='bob::1220…' \
//! CANTON_TOKEN_AMOUNT=10.0 \
//! CANTON_TOKEN_INSTRUMENT=Amulet \
//! CANTON_TOKEN_HOLDINGS='00abc…,00def…' \
//!   cargo run -p canton-token --example v1_transfer
//! ```
//!
//! Add `CANTON_TEST_TOKEN_URL` / `CANTON_TEST_CLIENT_ID` /
//! `CANTON_TEST_CLIENT_SECRET` where the participant wants OIDC, and set
//! `CANTON_TOKEN_DRY_RUN=1` to stop after building the command rather than
//! submitting it — a transfer moves real assets, so that is the default worth
//! reaching for first.

use canton_auth::{OidcConfig, TokenProvider};
use canton_daml as rt;
use canton_ledger::{CantonClient, Config};
use canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1 as h;
use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as md;
use canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1 as ti;
use canton_token::{RegistryClient, TransferKind};

fn var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("set {name}"))
}

/// The holdings to spend, from `CANTON_TOKEN_HOLDINGS` (comma-separated
/// contract ids).
///
/// Read them from the ledger's active contracts filtered to the `Holding`
/// interface, or from PQS.
///
/// The standard permits a registry to select holdings itself, so an empty list
/// is legal — but Splice's reference registry refuses one, and the refusal
/// arrives from the Daml interpreter after the whole registry round-trip has
/// succeeded. So this warns rather than defaulting quietly: against most
/// registries an unset variable is a run that fails at the last step for a
/// reason nothing earlier hinted at.
fn holdings<T>() -> Vec<rt::ContractId<T>> {
    let raw = std::env::var("CANTON_TOKEN_HOLDINGS").unwrap_or_default();
    if raw.trim().is_empty() {
        eprintln!(
            "warning: CANTON_TOKEN_HOLDINGS is unset, so no holdings are named. This is legal \
             only against a registry that selects them itself; Splice's refuses with \"At least \
             one holding must be provided\" once the transfer reaches the interpreter."
        );
    }
    raw.split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(|id| rt::ContractId::new(id.to_string()))
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = RegistryClient::new(&var("CANTON_TOKEN_REGISTRY_URL")?)?;

    // Who administers the instrument. Read from the registry here for brevity,
    // which only checks that it did not change between this call and the
    // factory call. Naming the administrator in the choice protects against a
    // registry substituting one — but only when the value comes from somewhere
    // the caller trusts, such as configuration, rather than from the registry
    // being checked.
    let info = registry.info().await?;
    let admin = rt::Party::parse(&info.admin_id)?;
    println!("registry admin: {}", info.admin_id);

    let instrument_id = var("CANTON_TOKEN_INSTRUMENT")?;
    match registry.instrument(&instrument_id).await? {
        Some(instrument) => println!(
            "instrument:     {} ({}), {} decimals",
            instrument.name, instrument.symbol, instrument.decimals
        ),
        None => println!("instrument:     {instrument_id} — this registry does not issue it"),
    }

    // The transfer is a generated type. Nothing here re-declares it.
    let sender = rt::Party::parse(&var("CANTON_TOKEN_SENDER")?)?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_micros();
    let now = i64::try_from(now)?;
    let transfer = ti::Transfer {
        sender: sender.clone(),
        receiver: rt::Party::parse(&var("CANTON_TOKEN_RECEIVER")?)?,
        amount: var("CANTON_TOKEN_AMOUNT")?.parse()?,
        instrument_id: h::InstrumentId {
            admin: admin.clone(),
            id: instrument_id,
        },
        requested_at: rt::Timestamp(now),
        // The window in which this may execute. A registry rejects a transfer
        // whose deadline has passed, so it is not decoration.
        execute_before: rt::Timestamp(now + 10 * 60 * 1_000_000),
        // Which holdings to spend. The standard allows a registry to choose
        // them itself, but it does not require one to: Splice's reference
        // registry refuses an empty list outright — the transfer reaches the
        // Daml interpreter and fails with "At least one holding must be
        // provided". So this is not optional in practice, and naming them also
        // pins exactly which are spent.
        input_holding_cids: holdings(),
        meta: md::Metadata {
            values: rt::TextMap::new(),
        },
    };

    // Two steps in one call: resolve the factory, take its context.
    let command = canton_token::transfer(&registry, &admin, transfer).await?;

    match command.transfer_kind() {
        Some(TransferKind::Direct) => println!("kind:           direct — completes on submission"),
        Some(TransferKind::SelfTransfer) => {
            println!("kind:           self — sender and receiver match");
        }
        Some(TransferKind::Offer) => {
            println!("kind:           offer — this will NOT settle until the receiver accepts it");
        }
        None => println!("kind:           the registry did not say"),
        // `TransferKind` is `#[non_exhaustive]`: a kind added later reaches
        // this arm instead of failing to compile here, which is the point.
        Some(other) => println!("kind:           {other:?} — a kind this build does not know"),
    }
    println!(
        "disclosing:     {} contract(s) the registry named",
        command.disclosed_contracts().len()
    );

    if std::env::var_os("CANTON_TOKEN_DRY_RUN").is_some() {
        println!("dry run: the command is built and not submitted");
        return Ok(());
    }

    let config = Config::new(var("CANTON_TEST_ENDPOINT")?);
    let config = match (
        std::env::var("CANTON_TEST_TOKEN_URL"),
        std::env::var("CANTON_TEST_CLIENT_ID"),
        std::env::var("CANTON_TEST_CLIENT_SECRET"),
    ) {
        (Ok(url), Ok(id), Ok(secret)) => {
            config.with_oidc(TokenProvider::new(OidcConfig::new(url, id, secret)))
        }
        _ => config,
    };
    let client = CantonClient::connect_lazy(config)?;

    let transaction = client
        .submit_and_wait_for_transaction(command.into_submit(sender.as_str()))
        .await?;
    println!(
        "committed {} at offset {} with {} event(s)",
        transaction.update_id,
        transaction.offset,
        transaction.events.len()
    );
    Ok(())
}
