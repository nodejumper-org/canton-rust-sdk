//! Recovering a submission whose outcome was lost.
//!
//! A submission that fails is not a submission that did not happen. A dropped
//! connection, a timeout, or a retry the participant de-duplicated all produce
//! an error over a command that may have committed perfectly well. The way back
//! to the answer is the change ID — which is why it exists before the send
//! rather than being returned by it.
//!
//! This deliberately submits *twice* with the same change ID. The second is
//! what a retry after a lost response looks like from the participant's side:
//! it is rejected as a duplicate, and the recovery then reads the original
//! command's real outcome out of the completion stream.
//!
//! Run against LocalNet's App Provider participant:
//!   LEDGER_ENDPOINT=http://localhost:3901 \
//!     LEDGER_TOKEN=$(...) LEDGER_PARTY='app_provider::1220…' \
//!     LEDGER_PKG='#quickstart-licensing' \
//!     cargo run -p canton-ledger --example recover_a_submission

use std::time::Duration;

use canton_ledger::{CantonClient, Config, Submit, create, identifier, record, value};

#[tokio::main]
async fn main() -> canton_ledger::Result<()> {
    let endpoint =
        std::env::var("LEDGER_ENDPOINT").unwrap_or_else(|_| "http://localhost:3901".to_string());
    let Ok(party) = std::env::var("LEDGER_PARTY") else {
        eprintln!("set LEDGER_PARTY to a party this participant hosts");
        return Ok(());
    };
    let package =
        std::env::var("LEDGER_PKG").unwrap_or_else(|_| "#quickstart-licensing".to_string());

    let mut config = Config::new(endpoint);
    if let Ok(token) = std::env::var("LEDGER_TOKEN") {
        config = config.with_token(token);
    }
    let client = CantonClient::connect_lazy(config)?;

    let command = create(
        identifier(&package, "Licensing.AppInstall", "AppInstallRequest"),
        record(vec![
            ("provider", value::party(&party)),
            ("user", value::party(&party)),
            (
                "meta",
                value::record(record(vec![("values", value::empty_text_map())])),
            ),
        ]),
    );

    // Where to read completions from, taken *before* submitting: a completion
    // that has already gone past cannot be read again.
    let offset = client.ledger_end().await?;

    let submission = client.submission(Submit::new(&party).add_command(command));
    let change_id = submission.change_id().clone();
    println!(
        "change id: command={} parties={:?}",
        change_id.command_id(),
        change_id.act_as()
    );

    let first = submission.submit_and_wait().await?;
    println!("committed: update_id={}", first.update_id);

    // The same submission again — what the SDK itself sends when a response is
    // lost and the request is retried. The participant de-duplicates on the
    // change ID, so this is rejected even though nothing is wrong.
    match submission.submit_and_wait().await {
        Ok(_) => println!("second submission accepted (no de-duplication window)"),
        Err(error) => {
            println!("second submission rejected as expected: {error}");

            // This is the moment the handle exists for. The match is on the
            // whole change ID — user, acting parties, command id — because a
            // command id alone is not unique across a participant's users.
            let completion = submission.recover(offset, Duration::from_secs(30)).await?;
            println!(
                "recovered the original outcome: update_id={} offset={}",
                completion.update_id, completion.offset
            );
            assert_eq!(
                completion.update_id, first.update_id,
                "recovery must find the command that actually committed"
            );
        }
    }

    Ok(())
}
