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
//! both would be demonstrating nothing. `CANTON_TOKEN_SETTLE=1` prints what the
//! executor would exercise and the allocation's own contract id, which is what
//! their side needs.
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = RegistryClient::new(&var("CANTON_TOKEN_REGISTRY_URL")?)?;
    let info = registry.info().await?;
    let admin = rt::Party::parse(&info.admin_id)?;
    println!("registry admin: {}", info.admin_id);

    let instrument_id = var("CANTON_TOKEN_INSTRUMENT")?;
    let sender = rt::Party::parse(&var("CANTON_TOKEN_SENDER")?)?;
    let receiver = rt::Party::parse(&var("CANTON_TOKEN_RECEIVER")?)?;
    let executor = rt::Party::parse(&var("CANTON_TOKEN_EXECUTOR")?)?;
    println!("sender:         {sender}");
    println!("receiver:       {receiver}");
    println!("executor:       {executor}");

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
        id: format!("dvp-{now}"),
        // A contract standing for the off-ledger agreement, when there is one.
        cid: None,
        meta: no_meta(),
    };

    // The sender's side of one leg: this much of this instrument, to that
    // account. `TransferSide::SenderSide` is what says these are *our* holdings
    // being reserved rather than someone else's being expected.
    let leg = al::TransferLegSide {
        transfer_leg_id: "leg-1".to_string(),
        side: al::TransferSide::SenderSide,
        otherside: account(receiver),
        amount: var("CANTON_TOKEN_AMOUNT")?.parse()?,
        instrument_id: instrument_id.clone(),
        meta: no_meta(),
    };

    let allocation = al::AllocationSpecification {
        admin: admin.clone(),
        // Whose holdings back this. The sender authorizes, so it is theirs.
        authorizer: account(sender.clone()),
        transfer_leg_sides: vec![leg],
        // After this the executor can no longer settle and the sender's
        // holdings are theirs again. Absent means no deadline, which is a
        // reservation with no way out — so it is set here on purpose.
        settlement_deadline: Some(deadline),
        next_iteration_funding: None,
        // `false`: the sender may still withdraw. Committing gives that up.
        committed: false,
        meta: no_meta(),
    };

    let holdings: Vec<rt::ContractId<h::Holding>> = std::env::var("CANTON_TOKEN_HOLDINGS")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(|id| rt::ContractId::new(id.to_string()))
        .collect();
    if holdings.is_empty() {
        eprintln!("warning: CANTON_TOKEN_HOLDINGS is unset — Splice's registry refuses that");
    }

    let command = canton_token::v2::allocate(
        &registry,
        settlement.clone(),
        allocation,
        rt::Timestamp(now),
        holdings,
        vec![sender.clone()],
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
