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
//! Add `CANTON_TOKEN` (a bearer token as issued) or `CANTON_TEST_TOKEN_URL` /
//! `CANTON_TEST_CLIENT_ID` / `CANTON_TEST_CLIENT_SECRET` (plus
//! `CANTON_TEST_AUDIENCE` where the issuer wants one) where the participant
//! wants authentication, and set
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

/// Attach whatever credentials the environment offers: a bearer token as
/// issued (`CANTON_TOKEN`), or OIDC client credentials to obtain one, with
/// `CANTON_TEST_AUDIENCE` where the issuer wants it. An unauthenticated
/// LocalNet needs neither.
fn authenticate(config: Config) -> Config {
    match (
        std::env::var("CANTON_TOKEN"),
        std::env::var("CANTON_TEST_TOKEN_URL"),
        std::env::var("CANTON_TEST_CLIENT_ID"),
        std::env::var("CANTON_TEST_CLIENT_SECRET"),
    ) {
        (Ok(token), ..) => config.with_token(token),
        (_, Ok(url), Ok(id), Ok(secret)) => {
            let mut oidc = OidcConfig::new(url, id, secret);
            if let Ok(audience) = std::env::var("CANTON_TEST_AUDIENCE") {
                oidc = oidc.with_audience(audience);
            }
            config.with_oidc(TokenProvider::new(oidc))
        }
        _ => config,
    }
}

/// The holdings to spend: `CANTON_TOKEN_HOLDINGS` (comma-separated contract
/// ids) if set, otherwise every *unlocked* holding of the instrument the
/// sender has, read from the ledger through `canton_token::holdings`.
///
/// The standard permits a registry to select holdings itself, so an empty list
/// is legal — but Splice's reference registry refuses one, and a locked
/// holding named as an input fails the same late way (`Lock.expiresAt`), after
/// the whole registry round-trip has succeeded. Reading and filtering here is
/// what makes a run against a real registry succeed on the first try.
async fn pick_holdings<T>(
    client: &CantonClient,
    sender: &str,
    instrument_id: &str,
) -> Result<Vec<rt::ContractId<T>>, Box<dyn std::error::Error>> {
    let raw = std::env::var("CANTON_TOKEN_HOLDINGS").unwrap_or_default();
    if !raw.trim().is_empty() {
        return Ok(raw
            .split(',')
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .map(|id| rt::ContractId::new(id.to_string()))
            .collect());
    }
    let spendable = canton_token::holdings::spendable(client, sender, instrument_id).await?;
    if spendable.is_empty() {
        return Err(format!("{sender} has no unlocked holdings of {instrument_id}").into());
    }
    println!(
        "holdings:       {} unlocked holding(s) of {instrument_id} read from the ledger",
        spendable.len()
    );
    Ok(spendable.iter().map(HoldingSummaryExt::typed).collect())
}

/// Local alias so `pick_holdings` reads the same in every example.
trait HoldingSummaryExt {
    fn typed<T>(&self) -> rt::ContractId<T>;
}
impl HoldingSummaryExt for canton_token::holdings::HoldingSummary {
    fn typed<T>(&self) -> rt::ContractId<T> {
        canton_token::holdings::HoldingSummary::typed(self)
    }
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
    let client =
        CantonClient::connect_lazy(authenticate(Config::new(var("CANTON_TEST_ENDPOINT")?)))?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_micros();
    let now = i64::try_from(now)?;
    let inputs = pick_holdings(&client, sender.as_str(), &instrument_id).await?;
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
        input_holding_cids: inputs,
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
