//! A CIP-0112 **allocation** — the other half of the V2 workflow.
//!
//! A transfer moves value between two parties who both want it to happen. An
//! allocation is for the case where they do not, or cannot, act together: the
//! sender *reserves* holdings for a settlement that a third party — the
//! **executor** — will complete later, and neither side has to trust the other
//! to hold up their end. It is what delivery-versus-payment is built from.
//!
//! Three parties, and the roles matter:
//!
//! * the **sender**, who allocates, and who is the only one who can withdraw
//!   the allocation before it settles;
//! * the **receiver**, named as the other side of the transfer leg;
//! * the **executor**, who settles the batch and who is neither of the above.
//!
//! ```sh
//! CANTON_TOKEN_REGISTRY_URL=http://localhost:5012 \
//! CANTON_TEST_ENDPOINT=http://localhost:3901 \
//! CANTON_TOKEN_SENDER='app_provider…::1220…' \
//! CANTON_TOKEN_RECEIVER='app_user…::1220…' \
//! CANTON_TOKEN_EXECUTOR='sv::1220…' \
//! CANTON_TOKEN_HOLDINGS='00abc…' \
//! CANTON_TOKEN_AMOUNT=1.0 \
//! CANTON_TOKEN_INSTRUMENT=Amulet \
//!   cargo run -p canton-token --example v2_allocate
//! ```
//!
//! This runs the **sender's** half: it allocates. Settling is the executor's,
//! from their own participant with their own credentials — the point of the
//! pattern is that the two are different principals, so one process holding
//! both would be demonstrating nothing. The run prints the settlement id and
//! the allocation's contract id, which is what the other sides need.
//!
//! A leg settles only when *both* of its sides are authorised, so when the
//! receiver is a different party it consents with an allocation of its own:
//! `CANTON_TOKEN_SIDE=receiver`, run as the receiver with its own credentials,
//! with `CANTON_TOKEN_SETTLEMENT_ID` set to the id the sender's run printed.
//! Nothing of the receiver's is reserved, so it names no holdings. Then the
//! executor runs `v2_settle`, which settles both allocations as one batch.
//!
//! `CANTON_TOKEN_DRY_RUN=1` stops after building the command.

use canton_auth::{OidcConfig, TokenProvider};
use canton_daml as rt;
use canton_ledger::{CantonClient, Config};
use canton_token::RegistryClient;
use canton_token::types::metadata as md;
use canton_token::types::v2::{allocation as al, holding as h};

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

fn account(owner: rt::Party) -> h::Account {
    h::Account {
        owner: Some(owner),
        provider: None,
        id: String::new(),
    }
}

fn no_meta() -> md::Metadata {
    md::Metadata {
        values: rt::TextMap::new(),
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
    let named = named_holdings();
    if !named.is_empty() {
        return Ok(named);
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

/// The holdings `CANTON_TOKEN_HOLDINGS` names (comma-separated contract ids),
/// or none.
fn named_holdings<T>() -> Vec<rt::ContractId<T>> {
    std::env::var("CANTON_TOKEN_HOLDINGS")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(|id| rt::ContractId::new(id.to_string()))
        .collect()
}

/// Which side of the leg this run allocates: `CANTON_TOKEN_SIDE=receiver` for
/// the receiver's consent, otherwise the sender's reservation.
fn receiver_side() -> Result<bool, String> {
    match std::env::var("CANTON_TOKEN_SIDE").as_deref() {
        Err(_) | Ok("sender") => Ok(false),
        Ok("receiver") => Ok(true),
        Ok(other) => Err(format!(
            "CANTON_TOKEN_SIDE is `sender` or `receiver`, not `{other}`"
        )),
    }
}

/// Both sides of a leg must name the same settlement, so the receiver joins
/// the one the sender's run created rather than starting its own.
fn settlement_id(receiving: bool, now: i64) -> Result<String, String> {
    match std::env::var("CANTON_TOKEN_SETTLEMENT_ID") {
        Ok(id) => Ok(id),
        Err(_) if receiving => Err("the receiver's side joins the sender's settlement: set \
                                    CANTON_TOKEN_SETTLEMENT_ID to the id the sender's run printed"
            .to_string()),
        Err(_) => Ok(format!("dvp-{now}")),
    }
}

/// The sides of the one leg this allocation authorises.
///
/// `SenderSide` says these are the authorizer's holdings being reserved for the
/// other account; `ReceiverSide` is the receiver's consent to be paid. Settling
/// needs *both* sides of a leg authorised, and a transfer pre-approval does not
/// stand in for the receiver's. When the sender is also the receiver, one
/// allocation carries both sides.
fn leg_sides(
    receiving: bool,
    sender: &rt::Party,
    receiver: &rt::Party,
    amount: &rt::Numeric,
    instrument_id: &str,
) -> Vec<al::TransferLegSide> {
    let side = |side, otherside: &rt::Party| al::TransferLegSide {
        transfer_leg_id: "leg-1".to_string(),
        side,
        otherside: account(otherside.clone()),
        amount: amount.clone(),
        instrument_id: instrument_id.to_string(),
        meta: no_meta(),
    };
    if receiving {
        return vec![side(al::TransferSide::ReceiverSide, sender)];
    }
    let mut sides = vec![side(al::TransferSide::SenderSide, receiver)];
    if receiver == sender {
        sides.push(side(al::TransferSide::ReceiverSide, sender));
        println!("receiver = sender: authorising both sides of the leg in this allocation");
    }
    sides
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = RegistryClient::new(&var("CANTON_TOKEN_REGISTRY_URL")?)?;
    let info = registry.info().await?;
    let admin = rt::Party::parse(&info.admin_id)?;
    println!("registry admin: {}", info.admin_id);

    let instrument_id = var("CANTON_TOKEN_INSTRUMENT")?;
    let sender = rt::Party::parse(&var("CANTON_TOKEN_SENDER")?)?;
    let client =
        CantonClient::connect_lazy(authenticate(Config::new(var("CANTON_TEST_ENDPOINT")?)))?;
    let receiver = rt::Party::parse(&var("CANTON_TOKEN_RECEIVER")?)?;
    let executor = rt::Party::parse(&var("CANTON_TOKEN_EXECUTOR")?)?;
    let receiving = receiver_side()?;
    // The party whose allocation this is, and who submits it.
    let authorizer = if receiving { &receiver } else { &sender };
    println!("sender:         {sender}");
    println!("receiver:       {receiver}");
    println!("executor:       {executor}");
    println!(
        "side:           {}",
        if receiving { "receiver" } else { "sender" }
    );

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_micros();
    let now = i64::try_from(now)?;
    let deadline = rt::Timestamp(now + 60 * 60 * 1_000_000);

    // What the allocation belongs to. A V2 allocation names its settlement when
    // it is *created*, which is what lets the settlement factory later check
    // that every leg of a batch was allocated for this one and not another.
    let settlement = al::SettlementInfo {
        executors: vec![executor.clone()],
        id: settlement_id(receiving, now)?,
        // A contract standing for the off-ledger agreement, when there is one.
        cid: None,
        meta: no_meta(),
    };

    let amount: rt::Numeric = var("CANTON_TOKEN_AMOUNT")?.parse()?;
    let transfer_leg_sides = leg_sides(receiving, &sender, &receiver, &amount, &instrument_id);

    let allocation = al::AllocationSpecification {
        admin: admin.clone(),
        // Whose allocation this is: the sender's holdings back the sender's
        // side, and the receiver's side is the receiver's consent.
        authorizer: account(authorizer.clone()),
        transfer_leg_sides,
        // After this the executor can no longer settle and the sender's
        // holdings are theirs again. Absent means no deadline, which is a
        // reservation with no way out — so it is set here on purpose.
        settlement_deadline: Some(deadline),
        next_iteration_funding: None,
        // `false`: the sender may still withdraw. Committing gives that up.
        committed: false,
        meta: no_meta(),
    };

    // The receiver reserves nothing, so it names no holdings unless told to.
    let holdings: Vec<rt::ContractId<h::Holding>> = if receiving {
        named_holdings()
    } else {
        pick_holdings(&client, sender.as_str(), &instrument_id).await?
    };

    let command = canton_token::v2::allocate(
        &registry,
        settlement.clone(),
        allocation,
        rt::Timestamp(now),
        holdings,
        vec![authorizer.clone()],
    )
    .await?;
    println!(
        "disclosing:     {} contract(s) the registry named",
        command.disclosed_contracts().len()
    );

    if std::env::var_os("CANTON_TOKEN_DRY_RUN").is_some() {
        println!("dry run: the allocation is built and not submitted");
        return Ok(());
    }

    let transaction = client
        .submit_and_wait_for_transaction(command.into_submit(authorizer.as_str()))
        .await?;
    println!(
        "allocated:      {} at offset {} with {} event(s)",
        transaction.update_id,
        transaction.offset,
        transaction.events.len()
    );

    // What the executor needs, printed rather than done: settling is theirs,
    // from their participant with their credentials.
    println!("\nthe executor settles this batch with:");
    println!("  settlement id: {}", settlement.id);
    println!("  executor:      {executor}");
    println!(
        "  v2::settle_batch(&registry, settlement, transfer_legs, allocations, vec![executor])"
    );
    for event in &transaction.events {
        if let Some(canton_ledger::proto::event::Event::Created(created)) = &event.event {
            println!("  created:       {}", created.contract_id);
        }
    }
    Ok(())
}
