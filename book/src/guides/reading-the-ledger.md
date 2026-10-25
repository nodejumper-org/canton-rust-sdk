# Reading the ledger

There are three ways to read a participant's view of the ledger: the **active
contract set** (ACS) as of an offset, the **update stream** after an offset,
and **point reads** of a single update by id or offset. A typical reader takes
a snapshot at the ledger end, streams updates from that same offset, and keeps
the offset of the last update it processed so it can resume.

[`CantonClient`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html)
(gRPC) and
[`JsonClient`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.JsonClient.html)
(JSON Ledger API) both offer these reads. The differences are listed
[at the end of this page](#grpc-and-json-differences-that-matter). Connecting
is covered in [Getting started](../getting-started.md), so the snippets take a
connected client. Through the `canton` facade the crates are `canton::ledger`
and `canton::daml`. Streams are `futures_core::Stream`s, and the snippets pin
them with `tokio::pin!` and call `next()` through `tokio_stream::StreamExt`.

## Offsets

An offset is an `i64` position on the participant's ledger.
[`ledger_end()`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.ledger_end)
returns the current end (`0` for an empty view). Update reads cover
`(begin, end]`: begin is exclusive and the optional end inclusive. To continue
exactly where a snapshot left off, use the snapshot's offset as the begin.

## The active contract set

[`active_contracts(parties, offset)`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.active_contracts)
streams the contracts active for `parties` at `offset` as
`canton_ledger::proto::ActiveContract`. Use
[`ActiveContractsRequest`](https://docs.rs/canton-ledger/latest/canton_ledger/request/struct.ActiveContractsRequest.html)
with `active_contracts_with` to narrow the snapshot or add to it:

| Builder method | Effect |
|---|---|
| `for_templates(ids)` | Only these templates. Each id is `package:Module:Entity`, where `package` can be a `#package-name` reference that keeps working across upgrades. |
| `for_interfaces(ids)` | Only contracts implementing these interfaces. The created events carry the interface views. |
| `with_created_event_blobs()` | Include each contract's `created_event_blob`, for disclosure in later commands. |
| `non_verbose()` | Leave out record labels. Typed decoding handles both forms. |
| `for_any_party()` | Apply the filter for every party the token can read for. Needs wildcard read rights. |

A snapshot item is a proto message. Decode its created event with the
generated type's
[`Template::from_created_event`](https://docs.rs/canton-daml/latest/canton_daml/trait.Template.html#method.from_created_event)
([Typed bindings from a DAR](codegen.md)); here, the pre-built
`canton-splice-amulet` bindings:

```rust,ignore
use canton_daml::Template as _;
use canton_ledger::{ActiveContractsRequest, CantonClient, Result};
use canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet;
use tokio_stream::StreamExt as _;

/// The party's amulets, and the offset they were read at.
async fn amulets(client: &CantonClient, party: String) -> Result<(i64, Vec<Amulet>)> {
    let offset = client.ledger_end().await?;
    let request = ActiveContractsRequest::new(vec![party], offset)
        .for_templates(["#splice-amulet:Splice.Amulet:Amulet"])?;
    let stream = client.active_contracts_with(request).await?;
    tokio::pin!(stream);

    let mut amulets = Vec::new();
    while let Some(active) = stream.next().await {
        if let Some(event) = active?.created_event {
            amulets.push(Amulet::from_created_event(&event)?);
        }
    }
    Ok((offset, amulets))
}
```

`from_created_event` checks the event's module, entity and package **name**
before it decodes the payload. It does not compare the package id, so a
contract created by an upgraded package version still decodes.

### Incomplete reassignments

`active_contracts` returns only active contracts. With several synchronizers,
a snapshot can also hold a contract that is unassigned from one and not yet
assigned to the other, and a reader that skips those entries loses it until
the reassignment completes. If your application spans synchronizers, use
[`acs_entries`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.acs_entries)
or `acs_entries_with`, which return every entry as an
[`AcsEntry`](https://docs.rs/canton-ledger/latest/canton_ledger/enum.AcsEntry.html):
`Active`, `IncompleteUnassigned` or `IncompleteAssigned`.

### Large snapshots

A streamed snapshot that fails partway through has to start over.
`active_contracts_resumable(parties, offset, max_page_size)` and its
`_with` and `acs_entries_resumable*` forms read the snapshot page by page and
retry a failed page from the last continuation token. To manage pages
yourself, `active_contracts_page` and `acs_page` (and `_with` forms) return one
page and the token for the next, or `None` after the last page.

## Update streams

[`updates(parties, begin)`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.updates)
is a live stream of the transactions and reassignments visible to `parties`
after `begin`. It never ends on its own, drops the participant's
`OffsetCheckpoint` messages and requests no topology events.
[`UpdatesRequest`](https://docs.rs/canton-ledger/latest/canton_ledger/request/struct.UpdatesRequest.html)
with the `_with` methods sets more. It has the five builder methods of the ACS
table above, plus:

| Builder method | Effect |
|---|---|
| `until(end)` | Stop after offset `end`. The stream ends once it gets there, which is the form to use for catching up. |
| `with_shape(shape)` | Choose a [`TransactionShape`](https://docs.rs/canton-ledger/latest/canton_ledger/request/enum.TransactionShape.html) (see below). |
| `descending()` | Newest first. Requires `until`. The resumable stream refuses it. |
| `without_reassignments()` | Drop reassignments, which are included by default. |
| `with_topology_events()` | Include participant-authorization topology events for the requested parties. |

The shape decides which events a transaction carries:

- `TransactionShape::LedgerEffects` (the default) gives create events and
  exercise events, both consuming and non-consuming, as they were executed. A
  requesting party must be a **witness** of an event.
- `TransactionShape::AcsDelta` gives created and archived events, the net
  change to the active contract set. A requesting party must be a
  **stakeholder** of an event. Use it to keep a copy of the ACS up to date.

To page through a bounded range instead of streaming it, use `updates_page` or
`updates_page_with`. The request passed to them must have `until`.

## Typed updates

The reads above return `canton_ledger::proto::get_updates_response::Update` on
gRPC and `serde_json::Value` on JSON, so code written against one transport
cannot read the other, and code matching on the proto types sees every
re-vendored proto change with no SemVer signal. New in 0.3.2,
[`canton_ledger::update`](https://docs.rs/canton-ledger/latest/canton_ledger/update/index.html)
defines SDK types that both transports produce:

- [`Update`](https://docs.rs/canton-ledger/latest/canton_ledger/update/enum.Update.html)
  is `Transaction`, `Reassignment`, `TopologyTransaction` or
  `OffsetCheckpoint`, with `offset()`, `update_id()` and `as_transaction()`.
- [`Transaction`](https://docs.rs/canton-ledger/latest/canton_ledger/update/struct.Transaction.html)
  carries `update_id`, `command_id`, `offset`, `synchronizer_id`,
  `effective_at`, `record_time`, `events` and more, and iterates one kind of
  event with `created()`, `archived()` and `exercised()`.
- [`Event`](https://docs.rs/canton-ledger/latest/canton_ledger/update/enum.Event.html)
  is `Created`, `Archived` or `Exercised`.
  [`CreatedEvent::decode`](https://docs.rs/canton-ledger/latest/canton_ledger/update/struct.CreatedEvent.html#method.decode)
  decodes the payload as a generated template type, with the same identity
  checks as `from_created_event`.
  [`ExercisedEvent`](https://docs.rs/canton-ledger/latest/canton_ledger/update/struct.ExercisedEvent.html)
  has `decode_argument` and `decode_result`.
- For data these types do not model, the original wire form is still
  available through `as_grpc()` and `as_json()` on `CreatedEvent`,
  `Reassignment` and `TopologyTransaction`. A reassignment's or topology
  transaction's events are only available that way.

Three reads return these types:

| Method | Returns |
|---|---|
| `CantonClient::updates_typed_with(request)` | `async`. A stream of `Result<Update>`. |
| `CantonClient::updates_resumable_typed_with(request)` | Not `async`. A resumable stream of `Result<Update>`. |
| `JsonClient::updates_typed_with(&request, limit)` | `async`. A `Vec<Update>` from a bounded read. |

All three drop checkpoints. To convert anything else, use `Update::from_grpc`
(a gRPC stream element), `Update::from_grpc_point` (a gRPC point read, or the
`update` of an `updates_page` element) or `Update::from_json` (a JSON element
or WebSocket frame).

This example follows a party's amulets from a saved offset:

```rust,ignore
use canton_ledger::update::{Event, Update};
use canton_ledger::{CantonClient, Result, TransactionShape, UpdatesRequest};
use canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet;
use tokio_stream::StreamExt as _;

async fn follow(client: &CantonClient, party: String, after: i64) -> Result<()> {
    let request = UpdatesRequest::new(vec![party], after)
        .with_shape(TransactionShape::AcsDelta)
        .for_templates(["#splice-amulet:Splice.Amulet:Amulet"])?;
    let stream = client.updates_resumable_typed_with(request);
    tokio::pin!(stream);

    while let Some(update) = stream.next().await {
        let update = update?;
        if let Update::Transaction(tx) = &update {
            for event in &tx.events {
                match event {
                    Event::Created(created) => {
                        let amulet: Amulet = created.decode()?;
                        println!("{} created for {}", created.contract_id, amulet.owner);
                    }
                    Event::Archived(archived) => println!("{} archived", archived.contract_id),
                    _ => {}
                }
            }
        }
        // Persist update.offset() here once the update is processed.
    }
    Ok(())
}
```

Both transports take the same `UpdatesRequest` and return the same types. In
this excerpt from
[`crates/canton-ledger/tests/live.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-ledger/tests/live.rs),
both reads return the same single transaction:

```rust,ignore
{{#include ../../../crates/canton-ledger/tests/live.rs:1084:1109}}
```

[ADR-0010 §4](../reference/adr/0010-api-decisions-deferred-to-1.0.md) records
the plan for 1.0: the typed reads take the plain names (`updates`,
`updates_with`, `updates_resumable`, the point reads), and the proto- and
JSON-returning reads move to `*_raw`.

## Resumable streams

A plain stream yields the error when its connection drops and does not
reconnect.
[`updates_resumable`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.updates_resumable)
and its `_with` and `_typed_with` forms reconnect instead:

- When an error is retriable
  ([`Error::is_retriable`](https://docs.rs/canton-core/latest/canton_core/enum.Error.html#method.is_retriable)),
  the stream reconnects from the **last offset it observed**. That includes the
  participant's `OffsetCheckpoint` messages, which it reads but does not yield.
  On a stream with no activity for your parties, checkpoints are the only thing
  that moves the resume point forward.
- The wait between attempts grows linearly. The attempt budget and the step
  are the client's
  [`RetryConfig`](https://docs.rs/canton-core/latest/canton_core/retry/struct.RetryConfig.html)
  `max_attempts` and `initial_backoff` (set with `Config::with_retry`), or 5
  reconnects and 250 ms without one. The count resets when an item arrives.
- When the attempts run out, the stream yields **the error that caused the
  last reconnect**, with the participant's status and details, and ends.
- A non-retriable error is yielded at once and ends the stream. With `until`,
  the stream ends normally when the participant closes it at the end offset.
- A request that fails validation, or one set to `descending()`, comes back as
  an `Error::InvalidRequest` item. These methods are not `async`, so the stream
  is the only place to report it.

To survive a process restart, persist the offset of the last update you
processed and pass it as `begin` next time. Checkpoints are not yielded, so on
a quiet stream that offset can trail the stream's own resume point, and if the
participant prunes past it the restart is refused (next section).

## Pruned offsets

A participant can prune old history, and it refuses any read that starts
inside the pruned range.
[`latest_pruned_offsets()`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.latest_pruned_offsets),
available on both clients, reports how far pruning has gone as a
[`PrunedOffsets`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.PrunedOffsets.html).
Updates and completions at or below `participant_pruned_up_to_inclusive` are
gone. `all_divulged_contracts_pruned_up_to_inclusive` covers divulged
contracts and may lag behind.

When a read is refused for this reason,
[`Error::pruned_earliest_offset()`](https://docs.rs/canton-core/latest/canton_core/enum.Error.html#method.pruned_earliest_offset)
returns the earliest offset the participant can still serve, on either
transport, and `None` for any other error. A resumable stream does not retry
this refusal; it yields it:

```rust,ignore
// Before resuming from a saved offset:
let pruned = client.latest_pruned_offsets().await?;
let history_lost = saved < pruned.participant_pruned_up_to_inclusive;

// On an error from a stream or a page:
if let Some(earliest) = err.pruned_earliest_offset() {
    // Readable data begins at `earliest`; the updates before it are gone.
}
```

The SDK never moves a resume point forward on its own, because both ways out
change what your application has seen: resume from the earliest offset and
accept that the pruned updates are lost, or rebuild from a fresh ACS snapshot
at the ledger end and stream from there.

## Point reads

Point reads fetch one update without scanning a stream for it: a stored
offset, or a transaction whose id came from a submission's completion. All of
them exist on both clients:

- `update_by_offset(offset, parties)` and `update_by_id(update_id, parties)`
  use the default format of `UpdatesRequest::new`.
- `update_by_offset_with(offset, &request)` and
  `update_by_id_with(update_id, &request)` take their format from an
  `UpdatesRequest`: parties, filters, shape, blobs and so on. They ignore the
  request's offsets.

If no update with that offset or id is visible to the parties, the result is
`Ok(None)`, not an error. A request that names no party is refused before it is
sent. The gRPC client returns
`Option<canton_ledger::proto::get_update_response::Update>` and the JSON client
returns `Option<serde_json::Value>`. Convert either one to an `Update`:

```rust,ignore
use canton_ledger::update::Update;

if let Some(raw) = client.update_by_id(id.clone(), vec![party.clone()]).await? {
    let update = Update::from_grpc_point(raw)?; // gRPC
    println!("{id} is at offset {}", update.offset());
}
if let Some(raw) = json.update_by_id(id, vec![party]).await? {
    let update = Update::from_json(&raw)?; // JSON
}
```

For a contract's create and archive events by contract id, use
`events_by_contract_id(contract_id, parties)` on either client.

## gRPC and JSON: differences that matter

| | gRPC (`CantonClient`) | JSON (`JsonClient`) |
|---|---|---|
| Update reads | Streams. Live unless the request has `until`. | `updates`, `updates_with` and `updates_typed_with` return a `Vec` from one HTTP response. |
| Size limit | Streams are not subject to the JSON list limit. | The node's `http-list-max-elements-limit`. Past it the read fails with `Error::Http` status `413`, so pass `until` and/or a `limit`. |
| Live tail | `updates*` | `ws_updates`, `ws_updates_with` (feature `ws`). |
| Resumable updates | `updates_resumable*`, with any `UpdatesRequest`. | `ws_updates_resumable(parties, begin)`. Takes no builder, yields raw JSON, and also reconnects after a clean close. |
| ACS | Streamed `active_contracts*`, `acs_entries*` with incomplete reassignments, and page-based resumable reads. | `active_contracts`, `active_contracts_with` (a `Vec`, with `limit`). `ws_active_contracts*` streams it (feature `ws`), and `ws_active_contracts_resumable` resumes from the continuation token. |
| Explicit paging | `updates_page*`, `active_contracts_page*`, `acs_page*` | None. |
| Checkpoints | Dropped by every `updates*` method. | Included in the raw `updates` and `updates_with` arrays. Dropped by `updates_typed_with` and `ws_updates*`. |
| Point reads | `Option<proto::get_update_response::Update>` | `Option<serde_json::Value>` |

JSON ACS elements look like `{"workflowId": …, "contractEntry":
{"JsActiveContract": {"createdEvent": …}}}`. Decode the active ones with
[`Template::from_json_created_event`](https://docs.rs/canton-daml/latest/canton_daml/trait.Template.html#method.from_json_created_event):

```rust,ignore
for element in json.active_contracts_with(&request, Some(500)).await? {
    if let Some(active) = element["contractEntry"].get("JsActiveContract") {
        amulets.push(Amulet::from_json_created_event(active)?);
    }
}
```

The `ws` feature is off by default. Turn it on as `canton-ledger/ws`, or as `ws`
on the `canton` facade. On JSON, resumable streams take their reconnect
settings from `JsonClient::with_retry`, the same way as on gRPC.

## See also

- [Typed bindings from a DAR](codegen.md) for the generated types used above.
- [Querying PQS](pqs.md) to filter contracts in a database instead.
- [The changelog](../project/changelog.md) for what 0.3.2 added.
- [The compatibility matrix](../reference/compatibility-matrix.md) for verified Canton versions.
