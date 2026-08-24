//! Every public future is `Send`.
//!
//! An async API whose futures are not `Send` cannot be `tokio::spawn`ed, which
//! makes it unusable in the servers this SDK exists for. Rust's API guidelines
//! call this C-SEND-SYNC, and it is the kind of property that breaks by
//! accident — one `std::sync::MutexGuard` or `Rc` held across an `.await`
//! anywhere inside a call chain is enough, and nothing else in the build
//! notices.
//!
//! These assertions are compile-time; the futures are never polled.
#![allow(dead_code, unused_must_use)]

use std::time::Duration;

use canton_ledger::{CantonClient, JsonClient, JsonCommands, Submit};

fn assert_send<F: Send>(_: F) {}

/// Never called: instantiating the futures is the whole test, and the compiler
/// checks it whether or not this runs.
fn public_futures_are_send(grpc: CantonClient, json: JsonClient) {
    let parties = vec!["alice".to_string()];
    let commands = JsonCommands::new(parties.clone());

    // JSON lane. Its request helpers are generic over the body type, which is
    // where a `Send` bound is easiest to lose.
    assert_send(json.version());
    assert_send(json.ledger_end());
    assert_send(json.submit(&commands));
    assert_send(json.submit_and_wait(&commands));
    assert_send(json.submit_and_wait_for_transaction(&commands));
    assert_send(json.events_by_contract_id("cid", parties.clone()));
    assert_send(json.active_contracts(parties.clone(), 0, None));
    assert_send(json.updates(parties.clone(), 0, Some(1), None));

    // gRPC lane.
    assert_send(grpc.version());
    assert_send(grpc.ledger_end());
    assert_send(grpc.health_check());
    assert_send(grpc.submit(Submit::new("alice")));
    assert_send(grpc.submit_and_wait(Submit::new("alice")));
    assert_send(grpc.submit_and_wait_for_transaction(Submit::new("alice")));
    assert_send(grpc.acs_entries(parties.clone(), 0));
    assert_send(grpc.acs_page(parties.clone(), 0, 10, None));
    assert_send(grpc.completions(parties.clone(), 0));
    assert_send(grpc.updates(parties.clone(), 0));

    // Both recovery handles, including `recover` — the one a caller reaches for
    // from an error path, often inside a spawned task.
    let json_submission = json.submission(commands);
    assert_send(json_submission.submit());
    assert_send(json_submission.submit_and_wait());
    assert_send(json_submission.recover(0, Duration::from_secs(1)));

    let grpc_submission = grpc.submission(Submit::new("alice"));
    assert_send(grpc_submission.submit());
    assert_send(grpc_submission.submit_and_wait());
    assert_send(grpc_submission.recover(0, Duration::from_secs(1)));
}

/// The clients themselves cross threads, which is what makes a spawned future
/// able to hold one.
#[test]
fn the_clients_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CantonClient>();
    assert_send_sync::<JsonClient>();
    assert_send_sync::<canton_ledger::Submission>();
    assert_send_sync::<canton_ledger::JsonSubmission>();
}
