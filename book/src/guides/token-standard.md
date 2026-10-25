# The token standard

[`canton-token`](https://docs.rs/canton-token/latest/canton_token/) runs the
Canton Network token-standard workflows: CIP-56 (V1) at the crate root and
CIP-0112 (V2) under [`canton_token::v2`](https://docs.rs/canton-token/latest/canton_token/v2/index.html).
The two standards use the same function names, so the module path tells you
which standard a call uses: `canton_token::transfer` is a V1 transfer and
`canton_token::v2::transfer` is a V2 one.

The crate declares no Daml types; those are generated from the Splice DARs and
ship in the `canton-splice-api-token-*` crates. `canton-token` adds the
workflow: it resolves a factory, asks the registry for a choice context,
attaches the contracts the registry says to disclose, and builds the exercise.
Submitting it is up to you.

Add it with `cargo add canton-token`, or use it through the facade crate as
`canton::token`. For what has been verified against which networks, see the
[compatibility matrix](../reference/compatibility-matrix.md#token-standard).

## Why a registry is involved

A token-standard choice is exercised through an interface. Only the registry
knows what reference data the choice needs, such as its configuration
contracts, the rules for an instrument or the current round. So a factory
workflow (transfer, allocate, settle) takes two steps, and the first one is an
HTTP call to the registry:

1. Send the registry the choice you are about to exercise, with empty
   `extraArgs` as the standard requires. The registry replies with the factory
   contract id, the choice context, and the contracts the participant must be
   shown.
2. Exercise the choice on that factory with the context filled in, and disclose
   the contracts the registry named. The submitting party usually cannot see
   them, which is what explicit disclosure is for.

### The registry client

[`RegistryClient::new`](https://docs.rs/canton-token/latest/canton_token/struct.RegistryClient.html)
takes the registry's base URL. On a Canton Network the registry for Amulet is
the network's Scan. The constructor parses the URL right away, so a typo shows
up as a configuration error instead of a connection failure. The client gives
up on a request after 30 seconds and never follows redirects, because the
registry's answer goes into commands you sign. To supply your own
`reqwest::Client` (proxy, TLS roots), use `RegistryClient::with_http_client`
and configure that client with `reqwest::redirect::Policy::none()`.

`info()` returns a
[`RegistryInfo`](https://docs.rs/canton-token/latest/canton_token/struct.RegistryInfo.html)
with `admin_id` (the party that administers the instruments) and
`supported_apis`. `list_instruments(page_size, page_token)` pages through the
[`Instrument`](https://docs.rs/canton-token/latest/canton_token/struct.Instrument.html)s
the registry issues, and `instrument(id)` returns `Ok(None)` for one it does
not issue. A 404 that is not the registry's own, from a wrong base URL or a
gateway, is an error rather than `None`.

### Choice contexts and disclosed contracts

The workflow functions call the registry for you. The lower-level methods are
public too: `transfer_factory`, `allocation_factory`,
`transfer_instruction_context`, `allocation_context`, and the V2 methods named
the same with a `_v2` suffix, plus `settlement_factory_v2` and
`allocation_instruction_context_v2`. A context for an existing contract is
requested per choice, named by
[`TransferInstructionChoice`](https://docs.rs/canton-token/latest/canton_token/enum.TransferInstructionChoice.html),
[`AllocationChoice`](https://docs.rs/canton-token/latest/canton_token/enum.AllocationChoice.html)
or
[`AllocationInstructionChoice`](https://docs.rs/canton-token/latest/canton_token/enum.AllocationInstructionChoice.html),
with a
[`ChoiceContextRequest`](https://docs.rs/canton-token/latest/canton_token/struct.ChoiceContextRequest.html)
(by default it asks the registry to leave out its debug fields). Never reuse a
context for another choice: the standard allows it to be choice-specific.

What comes back is a
[`ChoiceContext`](https://docs.rs/canton-token/latest/canton_token/struct.ChoiceContext.html):

- `decode::<T>()` decodes the context data into a generated type. The registry
  encodes the data with the Daml JSON API rules that `canton-daml` implements,
  so no hand-written mapping is involved.
- `disclosed_contracts()` returns the contracts as gRPC `DisclosedContract`s.
  The base64 `createdEventBlob` is decoded to bytes, and the
  `<package>:<Module>:<Entity>` template id is split into the three API fields.
  A blob that is not base64, or a malformed template id, is rejected here as
  `UnexpectedResponse` and never reaches the participant.
- `disclosed_contracts_json()` returns the same contracts in the registry's
  wire form (blob still base64), for a caller building a JSON Ledger API
  submission.

### What a workflow returns

Every workflow function returns a
[`TokenCommand`](https://docs.rs/canton-token/latest/canton_token/transfer/struct.TokenCommand.html):
the exercise command together with the contracts it needs disclosed, so one
cannot be submitted without the other. `into_submit(act_as)` builds a
[`canton_ledger::Submit`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.Submit.html)
and `into_prepare(act_as)` a
[`canton_ledger::interactive::Prepare`](https://docs.rs/canton-ledger/latest/canton_ledger/interactive/struct.Prepare.html)
(for a party whose key the participant does not hold, see
[Interactive submission and external signing](interactive-submission.md)),
both with the disclosures attached. `into_parts()` splits them for a
multi-command submission, and `transfer_kind()` reports how the registry will
carry out a transfer.

## Picking input holdings

The standard lets a request name no input holdings, but Splice's reference
registry refuses an empty list, and naming a **locked** holding fails too. Both
failures come from the Daml interpreter, after the registry round trip has
succeeded. So read the holdings first:
[`holdings::holdings(ledger, owner, Some(instrument_id))`](https://docs.rs/canton-token/latest/canton_token/holdings/fn.holdings.html)
reads the `Holding` interface views from the active contract set, and
[`holdings::spendable(ledger, owner, instrument_id)`](https://docs.rs/canton-token/latest/canton_token/holdings/fn.spendable.html)
returns the unlocked subset to name as inputs. Each
[`HoldingSummary`](https://docs.rs/canton-token/latest/canton_token/holdings/struct.HoldingSummary.html)
has `is_locked()`, `instrument_id()`, `amount()` and `typed::<T>()`. The V1 and
V2 `Holding` interfaces name the same contracts, so `typed` gives you the id as
whichever `ContractId<Holding>` your workflow needs. `v1_transfer`,
`v2_transfer` and the sender's side of `v2_allocate` call `spendable` when
`CANTON_TOKEN_HOLDINGS` is not set.

## Running the examples

The five examples are in
[`crates/canton-token/examples`](https://github.com/nodejumper-org/canton-rust-sdk/tree/main/crates/canton-token/examples).
Run them with `cargo run -p canton-token --example <name>`. They submit real
commands, so set `CANTON_TOKEN_DRY_RUN=1` on the first run: the example then
builds the command and stops before submitting it.

| Variable | Read by | Meaning |
|---|---|---|
| `CANTON_TEST_ENDPOINT` | all | The participant's gRPC Ledger API, e.g. `http://localhost:3901` |
| `CANTON_TOKEN_REGISTRY_URL` | all | The registry's base URL |
| `CANTON_TOKEN` | all | A bearer token as issued. If unset, `CANTON_TEST_TOKEN_URL`, `CANTON_TEST_CLIENT_ID` and `CANTON_TEST_CLIENT_SECRET` (plus `CANTON_TEST_AUDIENCE` if the issuer requires one) fetch one over OIDC. An unauthenticated LocalNet needs neither |
| `CANTON_TOKEN_DRY_RUN` | all | When set, build the command and do not submit it |
| `CANTON_TOKEN_SENDER` | `v1_transfer`, `v2_transfer`, `v2_allocate`, `v2_withdraw_allocation` | The sending party |
| `CANTON_TOKEN_RECEIVER` | `v1_transfer`, `v2_transfer`, `v2_allocate` | The receiving party |
| `CANTON_TOKEN_AMOUNT`, `CANTON_TOKEN_INSTRUMENT` | `v1_transfer`, `v2_transfer`, `v2_allocate` | Amount and instrument id, e.g. `Amulet` |
| `CANTON_TOKEN_HOLDINGS` | `v1_transfer`, `v2_transfer`, `v2_allocate` | Comma-separated holding contract ids. Optional: when unset, the sender's unlocked holdings are read from the ledger (the receiver's side of `v2_allocate` names none) |
| `CANTON_TOKEN_SENDER_ACCOUNT`, `CANTON_TOKEN_RECEIVER_ACCOUNT` | `v2_transfer` | V2 account ids. Default to the empty id, which fits a registry with one account per party |
| `CANTON_TOKEN_EXECUTOR` | `v2_allocate`, `v2_settle` | The party that settles |
| `CANTON_TOKEN_SIDE` | `v2_allocate` | `sender` (default) or `receiver` |
| `CANTON_TOKEN_SETTLEMENT_ID` | `v2_allocate` | The settlement to join. Required on the receiver's side. The sender's side generates `dvp-<micros>` when it is unset |
| `CANTON_TOKEN_SETTLEMENT` | `v2_settle` | Only settle this settlement id |
| `CANTON_TOKEN_ALLOCATION` | `v2_withdraw_allocation` | Only withdraw this allocation contract id |

Note the two settlement variables: `v2_allocate` reads `CANTON_TOKEN_SETTLEMENT_ID`,
`v2_settle` reads `CANTON_TOKEN_SETTLEMENT`.

## CIP-56 (V1) transfers

[`canton_token::transfer(&registry, &expected_admin, transfer)`](https://docs.rs/canton-token/latest/canton_token/transfer/fn.transfer.html)
takes a generated V1 `Transfer` and returns the `TransferFactory_Transfer`
exercise. `expected_admin` is required because naming the administrator in the
choice is what prevents a registry from substituting a different one. The
examples read it from `registry.info()` to stay short, but that only proves the
registry gave the same answer twice. In an application, take it from your own
configuration.

From
[`v1_transfer.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-token/examples/v1_transfer.rs)
(the `authenticate` and `pick_holdings` helpers are earlier in the file):

```rust,ignore
{{#include ../../../crates/canton-token/examples/v1_transfer.rs:106:198}}
```

The registry, not the caller, decides the
[`TransferKind`](https://docs.rs/canton-token/latest/canton_token/enum.TransferKind.html):
`Direct` (the receiver pre-approved direct transfers, so it completes on
submission), `SelfTransfer`, or `Offer`. An offer creates a
`TransferInstruction` and settles only when the receiver accepts it, so do not
report it as done. The enum is `#[non_exhaustive]` with an `Unknown` variant,
so a kind added later does not break decoding. The receiver completes an offer
with
[`canton_token::accept`](https://docs.rs/canton-token/latest/canton_token/transfer/fn.accept.html)
or `reject`; the sender can `withdraw` it. Each takes the registry and the
instruction's `ContractId<TransferInstruction>`.

### V1 allocations

[`canton_token::allocation`](https://docs.rs/canton-token/latest/canton_token/allocation/index.html)
covers the V1 allocation flow. `allocate(registry, expected_admin, allocation,
requested_at, input_holding_cids)` reserves holdings for one leg of a
settlement. On the resulting allocation, the executor calls `execute_transfer`
to settle it or `cancel` to drop it, and the sender calls `withdraw`. Each of
these choices fetches its own context.

## CIP-0112 (V2)

V2 builds commands the same way as V1. If you are porting from V1, these are
the changes:

- **Accounts, not parties.** A V2 transfer moves between two `Account`s
  (`owner`, `provider`, `id`). A party can hold several accounts, and both
  `owner` and `provider` are optional.
- **Every choice names its actors.** Each V2 function takes
  `actors: Vec<Party>`, the parties exercising the choice. V1 took them from
  the submitting party.
- **Settlement goes through a factory.** A V2 allocation is settled as part of
  a batch, so both legs of a delivery-versus-payment move in one transaction.
- **Allocation instructions have choices of their own:**
  `accept_allocation_instruction` and `withdraw_allocation_instruction`.

### Transfers over accounts

[`v2::transfer(&registry, transfer, actors)`](https://docs.rs/canton-token/latest/canton_token/v2/fn.transfer.html)
resolves the V2 transfer factory. It needs no `expected_admin` argument because
V2's `TransferFactory_Transfer` has no such field; the instrument's `admin` is
part of the transfer itself. From
[`v2_transfer.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-token/examples/v2_transfer.rs),
where the `account` helper builds an `Account` with `owner` set and `provider`
left as `None`:

```rust,ignore
{{#include ../../../crates/canton-token/examples/v2_transfer.rs:146:195}}
```

An offered V2 transfer is completed with `v2::accept`, `v2::reject` or
`v2::withdraw`. They take the same arguments as the V1 versions plus `actors`.

### Reading what moved

The create and archive events in a transaction show holdings appearing and
disappearing, but not which transfer they belonged to. A V2 registry records
that by exercising `EventLog_HoldingsChange`.
[`v2::events::holdings_changes(&transaction.events)`](https://docs.rs/canton-token/latest/canton_token/v2/events/fn.holdings_changes.html)
returns one
[`HoldingsChange`](https://docs.rs/canton-token/latest/canton_token/v2/events/struct.HoldingsChange.html)
per exercise, with `offset`, `node_id`, `contract_id` and the decoded `change`.

```rust,ignore
{{#include ../../../crates/canton-token/examples/v2_transfer.rs:220:243}}
```

The event is matched on the `EventLog` interface's module and entity name, not
its package id, which changes with every upgrade of the standard. A holdings
change that does not decode is an error, not a skip, so "could not read it"
never looks like "nothing moved".

### Allocation and settlement

An allocation reserves holdings for a settlement that a third party, the
**executor**, completes later. The **sender** allocates its holdings and is the
only party that can withdraw them before settlement. The **receiver** consents
to be paid with an allocation of its own, because a leg settles only when both
of its sides are authorised. The executor then settles the batch.

[`v2::allocate(&registry, settlement, allocation, requested_at, input_holding_cids, actors)`](https://docs.rs/canton-token/latest/canton_token/v2/fn.allocate.html)
names the `SettlementInfo` when the allocation is created. That is what lets
the settlement factory check that every allocation in a batch was made for
this settlement. From
[`v2_allocate.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-token/examples/v2_allocate.rs),
first the leg sides each run authorises:

```rust,ignore
{{#include ../../../crates/canton-token/examples/v2_allocate.rs:166:197}}
```

then the settlement, the allocation specification and the call:

```rust,ignore
{{#include ../../../crates/canton-token/examples/v2_allocate.rs:229:274}}
```

Set `settlement_deadline`. Without it the reservation never expires. Leave
`committed` as `false` if the sender should still be able to withdraw.

The executor settles with
[`v2::settle_batch(&registry, settlement, transfer_legs, allocations, actors)`](https://docs.rs/canton-token/latest/canton_token/v2/fn.settle_batch.html),
which goes through the registry's settlement factory. `v2_settle` reads every
allocation that names the executor from the active contract set and groups them
by settlement id. It then rebuilds each `TransferLeg` from the allocations'
`TransferLegSide`s (the `legs_of` helper) and settles each group as one batch.
From
[`v2_settle.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-token/examples/v2_settle.rs):

```rust,ignore
{{#include ../../../crates/canton-token/examples/v2_settle.rs:133:199}}
```

V2 has no execute-transfer context. `RegistryClient::allocation_context_v2`
refuses `AllocationChoice::ExecuteTransfer` locally with `InvalidRequest` and
points you to the settlement factory. The registry would only have answered
with a 404.

A three-party run takes one participant per role, each with its own
credentials. The sender runs `v2_allocate`, which prints the settlement id; the
receiver runs it again with `CANTON_TOKEN_SIDE=receiver` and
`CANTON_TOKEN_SETTLEMENT_ID` set to that id; the executor runs `v2_settle`.
That sequence has been run on a LocalNet with three participants on one
synchronizer. The commands, output and read-back are in
[Live runs on record](../reference/live-runs.md#v2-settlement-between-three-parties-on-three-participants-2026-10-13).

### Withdrawing an allocation

If the executor never settles, the allocation holds the sender's holdings
until its deadline. The sender releases them with
[`v2::withdraw_allocation(&registry, &allocation_id, actors)`](https://docs.rs/canton-token/latest/canton_token/v2/fn.withdraw_allocation.html).
The executor's counterpart is `v2::cancel`. `v2_withdraw_allocation` lists the
sender's active allocations, or only the one named in
`CANTON_TOKEN_ALLOCATION`, and withdraws each one in its own submission. From
[`v2_withdraw_allocation.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-token/examples/v2_withdraw_allocation.rs):

```rust,ignore
{{#include ../../../crates/canton-token/examples/v2_withdraw_allocation.rs:111:135}}
```

## Which crate holds which types

[`canton_token::types`](https://docs.rs/canton-token/latest/canton_token/types/index.html)
re-exports the generated modules the workflow functions take as arguments.
With it you can name every argument without adding each generated crate as a
dependency:

| Path in `canton_token::types` | Generated crate |
|---|---|
| `metadata` (`ExtraArgs`, `ChoiceContext`, `Metadata`) | `canton-splice-api-token-metadata-v1`, shared by V1 and V2 |
| `v1::holding` | `canton-splice-api-token-holding-v1` |
| `v1::transfer_instruction` | `canton-splice-api-token-transfer-instruction-v1` |
| `v1::allocation` | `canton-splice-api-token-allocation-v1` |
| `v1::allocation_instruction` | `canton-splice-api-token-allocation-instruction-v1` |
| `v2::holding` (`Account`, `Holding`) | `canton-splice-api-token-holding-v2` |
| `v2::transfer_instruction` | `canton-splice-api-token-transfer-instruction-v2` |
| `v2::allocation` (`SettlementInfo`, `TransferLeg`, `FinalizedAllocation`) | `canton-splice-api-token-allocation-v2` |
| `v2::allocation_instruction` | `canton-splice-api-token-allocation-instruction-v2` |
| `v2::transfer_events` (`EventLog`) | `canton-splice-api-token-transfer-events-v2` |

The V1 `Holding` interface is also available at the crate root as
`canton_token::Holding`. The allocation-request, burn/mint and featured-app
packages are not re-exported; depend on their crates directly. For the full
crate-by-crate table, see the
[compatibility matrix](../reference/compatibility-matrix.md#token-standard).

## Testing

CI drives every workflow function against an in-process registry stub that
records each request
([`tests/inprocess.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-token/tests/inprocess.rs)).
The live suite
([`tests/live.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-token/tests/live.rs))
runs against a real registry:

```sh
CANTON_TOKEN_REGISTRY_URL=http://localhost:5012 \
  cargo test -p canton-token --test live -- --nocapture
```

Without `CANTON_TOKEN_REGISTRY_URL` the live tests skip. Set
`CANTON_TEST_REQUIRE_LIVE=1` to make a skip fail the run. To get a participant
and a registry running locally, see
[Local development and testing](../local-development.md).
