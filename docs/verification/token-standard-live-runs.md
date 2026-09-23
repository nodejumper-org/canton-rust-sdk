# Token-standard examples: live runs

Three runs are on record: a cn-quickstart LocalNet (below), the Canton
Network DevNet (further down, dated 2026-09-22), and the reviewer's path on a
Canton Builder Tool LocalNet (at the end, same day).

The verification clause of the milestone-3 proposal asks for a V1 transfer and
a V2 `Account`-based transfer/allocation exercised end to end. The three
examples under [`crates/canton-token/examples`](../../crates/canton-token/examples)
are that exercise, and this file is their output from the most recent run,
kept verbatim so the offsets and update ids can be checked against the
participant that produced them.

## Environment

| | |
|---|---|
| Date | 2026-09-07 |
| Network | cn-quickstart LocalNet (Splice 0.6.11 images), `SV_PROFILE=on` |
| Participant | Canton **3.5.7**, the App Provider node: gRPC `:3901`, JSON `:3975` |
| Registry | the super-validator's scan on `:5012`, reached through a `socat` forward |
| Instrument | **Amulet** (Canton Coin), which declares every V1 and V2 token-standard API |
| Sender | the App Provider party (`app_provider_quickstart-…`) |
| Receiver | the App User party (`app_user_quickstart-…`) |
| Executor (allocation only) | the super-validator party (`sv::…`) |
| Authentication | Keycloak OIDC client credentials, as the live tests use |

Every example read its input holdings from the ledger through
`canton_token::holdings::spendable` — no contract id was supplied by hand — and
submitted with exactly the contracts the registry named for disclosure.

## Commands

```sh
export CANTON_TEST_ENDPOINT=http://localhost:3901
export CANTON_TEST_TOKEN_URL=http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token
export CANTON_TEST_CLIENT_ID=app-provider-backend CANTON_TEST_CLIENT_SECRET=…
export CANTON_TOKEN_REGISTRY_URL=http://localhost:5012
export CANTON_TOKEN_SENDER='app_provider_quickstart-…::1220…'
export CANTON_TOKEN_RECEIVER='app_user_quickstart-…::1220…'
export CANTON_TOKEN_EXECUTOR='sv::1220…'
export CANTON_TOKEN_INSTRUMENT=Amulet CANTON_TOKEN_AMOUNT=1.0

cargo run -p canton-token --example v1_transfer
cargo run -p canton-token --example v2_transfer
cargo run -p canton-token --example v2_allocate
```

And the JSON-only package read that issue #2 asked for, against the same
participant with nothing but its JSON port:

```sh
CANTON_JSON_ENDPOINT=http://localhost:3975 \
CANTON_TEST_TOKEN_URL=… CANTON_TEST_CLIENT_ID=… CANTON_TEST_CLIENT_SECRET=… \
  cargo run -p canton-ledger --example json_packages
```

## Output

### `v1_transfer` — V1 transfer (CIP-56)

`2026-09-07T19:52:14Z`, exit 0

```text
registry admin: DSO::1220c684772525a9bbf71cfe5669f24944d2ee9cbb34427f3d74ea3a404320168d96
instrument:     Canton Coin (CC), 10 decimals
holdings:       1 unlocked holding(s) of Amulet read from the ledger
kind:           offer — this will NOT settle until the receiver accepts it
disclosing:     4 contract(s) the registry named
committed 1220a2f8c620e1ec325c2194da01c700a87013a465d03371dc8e3765f7dc6848bbcb at offset 67684 with 6 event(s)
```

### `v2_transfer` — V2 transfer over accounts (CIP-0112)

`2026-09-07T19:52:41Z`, exit 0

```text
registry admin: DSO::1220c684772525a9bbf71cfe5669f24944d2ee9cbb34427f3d74ea3a404320168d96
instrument:     Canton Coin (CC), 10 decimals
from account:   "" / to account: ""
holdings:       1 unlocked holding(s) of Amulet read from the ledger
kind:           offer — this will NOT settle until the receiver accepts it
disclosing:     4 contract(s) the registry named
committed 122024fb9a369c0d2556058ed3033c8334ae4800673a73b01af1bc1b2d77b03a7e1c at offset 67690 with 6 event(s)
  holdings change on 00f709626156fc630d8e5278b8c9b514e2511609ae117e747a29be08e80ded64e9ca1212203b018402c21cf5e15e4c01c8800e446a1e96456d6a1de77756fa6153207ce301 (node 9): 1 spent, 2 produced, 2 leg(s)
```

### `v2_allocate` — V2 allocation, three parties (CIP-0112)

`2026-09-07T19:52:48Z`, exit 0

```text
registry admin: DSO::1220c684772525a9bbf71cfe5669f24944d2ee9cbb34427f3d74ea3a404320168d96
sender:         app_provider_quickstart-dimakozoliy-1::12204a2925a78214d3a4c81f3b425f89894a42858b9e8f13a8427c68513232cdf640
receiver:       app_user_quickstart-dimakozoliy-1::12209a0cd86271e87116bd1f059b0392dd5a7b09b93681b488082ed8c5a808fc25e3
executor:       sv::1220932f0e5664f46da8c8fcc36a6b1f8df822df6d70188d0090878b7e4eed8f2088
holdings:       1 unlocked holding(s) of Amulet read from the ledger
disclosing:     2 contract(s) the registry named
allocated:      122011c124d3a1420d7ce72e172cc8b6934a006eb3dda43e4ea7a9ace9e391756265 at offset 67693 with 6 event(s)

the executor settles this batch with:
  settlement id: dvp-1788810771450019
  executor:      sv::1220932f0e5664f46da8c8fcc36a6b1f8df822df6d70188d0090878b7e4eed8f2088
  v2::settle_batch(&registry, settlement, transfer_legs, allocations, vec![executor])
  created:       00a352050a612e0175c58de115803585cec450a9d3c1cf913d648a8768de587dcfca12122077deccc45c651af83eb4567e5dd30d7f9916991301db8a71e5501a657a685858
  created:       00eac6bef2d86eed2ee4eb33362095ee5f1d09471e0341418d9080f8a3206e8fdeca121220e939f334b3bb9c3f71905bfc0d783f4ddfc2531591795dd13cffc7d9f8b14f16
  created:       00ba2dedc8f522a97a2a24c64f4854e45f321aa6ae55a408d49e11c328a9dc0a93ca121220eb6dea4ffba2706a8cc14afb6320131e8518003036939a1e734d66b090a0ecba
```

### `json_packages` — JSON-only package read (issue #2)

`2026-09-07T19:51:46Z`, exit 0

```text
participant:    http://localhost:3975 (JSON Ledger API 3.5.7)
packages known: 63
de2cc2f90eb523414ff54e899951dadd8789a4c07e0f71f6d6c9eaf57d412a54: Registered
```

## What the output shows

| Clause | Evidence |
|---|---|
| V1 transfer settles end to end | `v1_transfer`: the factory resolved against the registry, four disclosed contracts attached, **committed at offset 67684** with 6 events. The kind is `offer`, so the receiver's `accept` completes it — the receiver's move, from their own credentials |
| V2 `Account`-based transfer | `v2_transfer`: the same over accounts, **committed at offset 67690**; `events::holdings_changes` read the committed transaction back as one holdings change — 1 spent, 2 produced, 2 legs |
| V2 allocation | `v2_allocate`: sender, receiver and executor are three distinct parties; **allocated at offset 67693**, 3 contracts created, and the settlement the executor completes is printed with its id. Settling is deliberately not run from the sender's process |
| Choice context and disclosure | every run lists the contracts the registry named and submitted with them; the transaction would not have interpreted without them |

Amulet is a V2 implementation exercised as one. Whether that satisfies the
clause's words "V2 reference token" is recorded as an open question for the
subcommittee in the compatibility matrix, not decided here.


---

# DevNet, 2026-09-22

The same three examples, plus the withdrawal that releases the allocation,
against a validator on the **Canton Network DevNet** — the network the
proposal's verification clause names. Every submission was direct: the
receiving party holds a transfer pre-approval, so both transfers settled on
submission with no `accept` step, which is the clause's "settles end to end"
taken literally.

## Environment

| | |
|---|---|
| Date | 2026-09-22, 07:32–07:34 UTC |
| Network | Canton Network **DevNet**, synchronizer `global-domain::1220be58…` |
| Participant | `nodejumper-dev-1`, Canton **3.5.17**, Splice validator **0.8.1** (Nodejumper's node) |
| Ledger API | gRPC over TLS on `:443`; JSON Ledger API over HTTPS |
| Authentication | Keycloak, realm `canton-devnet`: a user token with `actAs` on the sender party and nothing else |
| Registry | the public DevNet Scan of SV-1, `https://scan.sv-1.dev.global.canton.network.sync.global`, unauthenticated |
| Instrument | **Amulet** (Canton Coin); the DevNet registry declares all ten V1 and V2 APIs |
| Sender | `8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd…`, a wallet user on that node, funded by a tap |
| Receiver and executor | `nodejumper-dev-1::1220a5cd…`, the validator's own party |
| Amount | 1 CC per operation |

The registry's OpenAPI documents at Splice 0.8.1 were diffed against the
vendored 0.6.11 copies before the run: six of seven are byte-identical and
`token-metadata-v1` differs only by the additive `accountInputFieldsToShow`
field, which `Instrument` already carries.

## Commands

```sh
export CANTON_TEST_ENDPOINT=https://ledger-grpc.validator.devnet.canton.nodejumper.io
export CANTON_TEST_JSON_ENDPOINT=https://ledger-api.validator.devnet.canton.nodejumper.io
export CANTON_TOKEN_REGISTRY_URL=https://scan.sv-1.dev.global.canton.network.sync.global
export CANTON_TOKEN=…                # a bearer token for the sender's user
export CANTON_TOKEN_SENDER='8f2bb33b-…::1220a5cd…'
export CANTON_TOKEN_RECEIVER='nodejumper-dev-1::1220a5cd…'
export CANTON_TOKEN_EXECUTOR='nodejumper-dev-1::1220a5cd…'
export CANTON_TOKEN_INSTRUMENT=Amulet CANTON_TOKEN_AMOUNT=1.0

cargo run -p canton-token --example v1_transfer
cargo run -p canton-token --example v2_transfer
cargo run -p canton-token --example v2_allocate
cargo run -p canton-token --example v2_withdraw_allocation
```

Each was run first with `CANTON_TOKEN_DRY_RUN=1` — the command built against
the real registry, with its context and disclosures, and not submitted — and
then for real.

## Output

### `v1_transfer` — V1 transfer (CIP-56), direct

`2026-09-22T07:32:26Z`, exit 0

```text
registry admin: DSO::1220be58c29e65de40bf273be1dc2b266d43a9a002ea5b18955aeef7aac881bb471a
instrument:     Canton Coin (CC), 10 decimals
holdings:       1 unlocked holding(s) of Amulet read from the ledger
kind:           direct — completes on submission
disclosing:     6 contract(s) the registry named
committed 12203d9608d50269325b406975fc6445c1956acd845dc30185bab22db21b3979fb3d at offset 3316298 with 7 event(s)
```

### `v2_transfer` — V2 transfer over accounts (CIP-0112), direct

`2026-09-22T07:32:33Z`, exit 0

```text
registry admin: DSO::1220be58c29e65de40bf273be1dc2b266d43a9a002ea5b18955aeef7aac881bb471a
instrument:     Canton Coin (CC), 10 decimals
from account:   "" / to account: ""
holdings:       1 unlocked holding(s) of Amulet read from the ledger
kind:           direct — completes on submission
disclosing:     6 contract(s) the registry named
committed 12202e8ddcd542ff100bd16850a6d2b97f4cced6cf25246ea1c55fb77baf2fe5579e at offset 3316304 with 7 event(s)
  holdings change on 00435bc9becd27689198d8123677d28c1b23061da6b0716fc3c24e02deb657c3cbca1212206d0cde3b136ecb322fc15ff0c110882c11910cdd31009451eacdb5cb72434ecb (node 11): 1 spent, 1 produced, 1 leg(s)
  holdings change on 00435bc9becd27689198d8123677d28c1b23061da6b0716fc3c24e02deb657c3cbca1212206d0cde3b136ecb322fc15ff0c110882c11910cdd31009451eacdb5cb72434ecb (node 12): 0 spent, 1 produced, 1 leg(s)
```

### `v2_allocate` — V2 allocation (CIP-0112)

`2026-09-22T07:32:44Z`, exit 0

```text
registry admin: DSO::1220be58c29e65de40bf273be1dc2b266d43a9a002ea5b18955aeef7aac881bb471a
sender:         8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd
receiver:       nodejumper-dev-1::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd
executor:       nodejumper-dev-1::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd
holdings:       1 unlocked holding(s) of Amulet read from the ledger
disclosing:     2 contract(s) the registry named
allocated:      12201e70b86ba0ef478fc6738c62d85d13a38e2df1318671db6b0ae560e751afe3fd at offset 3316319 with 6 event(s)

the executor settles this batch with:
  settlement id: dvp-1790062365710383
  executor:      nodejumper-dev-1::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd
  v2::settle_batch(&registry, settlement, transfer_legs, allocations, vec![executor])
  created:       00f536ec3917d4ab54acefc4aad06ad72cfd9690fda1c157468475041016c63b72ca121220776cab3d3e2c5bea1d1fe570310fb30ec1483b00a1d50b1825c5716d2eb49af9
  created:       009badac1d7fc8a59777ae65c71d9d319a16598b20ec4321c9b148cd7c2cfc57a8ca12122086e787c4fc08e546ebe24d6d88f9879382ede8fa443a8d44ebe8b8fe7897f925
  created:       00594cb3a8b1a5d3ecdbfca3c40a20c37eb5e3bf748d4ebd75235a0ffbdb0ae555ca121220ca0f4d05081cc81706f2df0879af716cc2db3460e358f70dcacc8c7887334b7e
```

### `v2_withdraw_allocation` — V2 allocation withdrawn by the sender

`2026-09-22T07:33:39Z`, exit 0

```text
sender:      8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd
allocations: 1 active
  00594cb3a8b1a5d3ecdbfca3c40a20c37eb5e3bf748d4ebd75235a0ffbdb0ae555ca121220ca0f4d05081cc81706f2df0879af716cc2db3460e358f70dcacc8c7887334b7e
    settlement dvp-1790062365710383 by nodejumper-dev-1::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd, 1 holding(s) reserved
withdrawing 00594cb3a8b1a5d3 (settlement dvp-1790062365710383): disclosing 4 contract(s) the registry named
withdrawn:   12209b5d69f163eca284c4f2c04cfcbb030de8792012a5e37c3c8b64294f3a7a51a3 at offset 3316364 with 7 event(s)
```

## Read back from the participant by update id

Every update id above was read back from the node over the JSON Ledger API
(`POST /v2/updates/update-by-id`) after the run. The record times are the
synchronizer's.

| Update id | Offset | Record time | Events visible to the sender |
|---|---|---|---|
| `12203d9608d50269325b…` | 3316298 | 2026-09-22T07:32:29.748296Z | 2 |
| `12202e8ddcd542ff100b…` | 3316304 | 2026-09-22T07:32:34.873070Z | 2 |
| `12201e70b86ba0ef478f…` | 3316319 | 2026-09-22T07:32:47.179492Z | 4 |
| `12209b5d69f163eca284…` | 3316364 | 2026-09-22T07:33:42.266115Z | 3 |

The public Scan's transaction API answers 403 to unauthenticated callers on
DevNet, so the ids are verifiable from any DevNet participant rather than from
a browser. The sender's holdings afterwards: 837.2655239056 CC and the 1 CC the
withdrawal released, both unlocked — 840 minus the two transfers, nothing left
reserved.


## Executor settlement, the same day

The allocation above was withdrawn, not settled: the executor is the
validator's party and this run holds no token for it. `settle_batch` — the
executor's move, and the proposal's "allocation/executor settlement" — was
then exercised with the sender as executor, in two attempts that together say
what a V2 settlement requires.

**First attempt, refused by the settlement factory.** An allocation naming the
sender as executor, one leg from the sender to the validator's party.
Amulet's `SettlementFactory_SettleBatch` refused it at interpretation:

```text
executor:    8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd
allocations: 1 naming it as executor
settlement dvp-1790063359274031: 1 allocation(s), 1 leg(s)
  leg leg-1: 8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd -> nodejumper-dev-1::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd : 1.0000000000 Amulet
  disclosing 3 contract(s) the registry named
Error: Status(Status { code: FailedPrecondition, message: "DAML_FAILURE(9,5085fbf8): Interpretation error: Error: User failure: UNHANDLED_EXCEPTION/DA.Exception.GeneralError:GeneralError (error category 9): 'missing authorizations' is not equal to 'empty set'.\nmissing authorizations: Set [(Account {owner = Some 'nodejumper-dev-1::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd', provider = None, id = \"\"},TransferLegSide {transferLegId = \"leg-1\", side = ReceiverSide, otherside = Account {owner = Some '8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd...", details: … }
```

That is the standard, not a bug: a V2 leg needs *both* sides authorised — the
receiver consents with a `ReceiverSide` allocation of its own — and a
transfer pre-approval does not stand in for it. This allocation was withdrawn
(update `1220f4a7…`, offset 3317173).

**Second attempt, settled.** A leg whose sender, receiver and executor are the
same party, so one allocation carries both sides and one token can execute.
Degenerate as a delivery-versus-payment, but it drives the whole path: the
allocation factory, the settlement factory's context and disclosures, and
`settle_batch` committing on the DevNet synchronizer.

### `v2_allocate` — both sides of the leg in one allocation

`2026-09-22T07:50:32Z`, exit 0

```text
receiver:       8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd
receiver = sender: authorising both sides of the leg in this allocation
allocated:      1220a959f224d586e432b0e7684cd04b6d204356b33f6ecf3d334d0ecbbb8e59ea5b at offset 3317197 with 2 event(s)
  settlement id: dvp-1790063434475117
```

### `v2_settle` — the executor settles the batch

`2026-09-22T07:50:54Z`, exit 0

```text
executor:    8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd
allocations: 1 naming it as executor
settlement dvp-1790063434475117: 1 allocation(s), 1 leg(s)
  leg leg-1: 8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd -> 8f2bb33b-699f-440e-a861-508ee6a0472d::1220a5cd222348403b3db750ba80ddbd0c3f18a5692b425273d150c1efff5ceb63bd : 1.0000000000 Amulet
  disclosing 2 contract(s) the registry named
  settled: 1220b829f7b412aedd5215d77f006b4d6fdd54696cfaf21fe1093c18754327299d69 at offset 3317221 with 4 event(s)
```

Read back by update id:

| Update id | Offset | Record time | Events visible to the sender |
|---|---|---|---|
| `1220424177cbc89173b0…` | 3317129 | 2026-09-22T07:49:20.440389Z | 4 |
| `1220f4a77863cb1ad1ea…` | 3317173 | 2026-09-22T07:50:15.185034Z | 3 |
| `1220a959f224d586e432…` | 3317197 | 2026-09-22T07:50:35.177075Z | 1 |
| `1220b829f7b412aedd52…` | 3317221 | 2026-09-22T07:50:57.874686Z | 1 |

The sender's holdings afterwards: one unlocked holding of 838.2655239056 CC —
840 minus the two transfers to the validator, the settled coin back with its
owner, nothing reserved.

## Live suites against the same node

With `CANTON_TEST_REQUIRE_LIVE=1`, so a suite that could not reach the node
fails rather than skips:

- `canton-token` live suite against the DevNet registry: 5 passed — the
  registry describes itself and its instruments as the standard says, and
  paging works against a real Scan.
- `canton-ledger` live suite, the subset a node without the test DAR can run:
  6 passed — version and health over gRPC, version and ledger end over JSON,
  the JSON-only package read (288 packages), the JSON party reads (the
  participant id, the sender's own party), a bad token is an HTTP error.

The submission-based ledger and interactive-submission suites need the
`quickstart-licensing` package on the participant and were not run here;
they run against the LocalNet above.

## What the run shows against the clause

| Clause | Evidence |
|---|---|
| A V1 transfer settles end to end **on DevNet** | committed at offset 3316298, kind `direct`, read back with record time 07:32:29 UTC |
| A V2 `Account`-based transfer/allocation exercised against **Canton Coin's V2 path on DevNet** | transfer at 3316304 with the holdings change parsed from the committed transaction; allocation at 3316319 naming the validator as executor, withdrawn at 3316364; allocation at 3317197 **settled by its executor** at 3317221 |
| Allocation/**executor settlement** | `settle_batch` committed through Amulet's settlement factory at 3317221, after the factory had refused a one-sided leg — the refusal is on record above |
| Choice context and disclosure, against a production registry | six disclosed contracts per transfer, two per allocation, four per withdrawal, all named by the DevNet Scan |

The clause names two V2 targets, joined by "and": the *V2 reference token*,
and Canton Coin's V2 path on DevNet. This run covers the second. The first is
**not covered here**: the reference token is `TestTokenV2` from Splice's
`splice-token-standard-v2-test` package, which Splice exercises by Daml script
against a simulated registry; it has no deployment on DevNet, and the
separate V2 DevNet that once hosted it is retired. Closing that half means
either standing the reference token up ourselves — its DAR on a participant,
with a registry for it — or the subcommittee agreeing that the deployed V2
implementation stands in for a reference token that is not deployed anywhere.


---

# The reviewer's path: Canton Builder Tool, 2026-09-22

The "How to verify" steps of the milestone comment, run verbatim from a fresh
clone of the branch (`git clone`, then the commands as written) against a
[Canton Builder Tool](https://canton-network-devs.github.io/Canton-Builder-Tool/#part-builder)
LocalNet — the environment the Foundation's reviewer used for the earlier
milestones. Nothing beyond `canton builder start`, one tap in the App Provider
wallet, and the commands.

| | |
|---|---|
| LocalNet | Canton Builder Tool, Splice 0.6.11 images, Canton 3.5.7; auth `self-signed` (HS256 tokens from `canton builder token`, whose user holds `ParticipantAdmin`) |
| Registry | the LocalNet's own Scan, `http://scan.localhost:4000` |
| Party | `app_provider_builder-localnet-1::122050a3…`, sender, receiver and executor at once (`kind: self`) |
| External signing | the built-in `Ping` template every participant ships, created by the freshly onboarded external party |
| PQS | Scribe 3.5.4 from [`tools/pqs/compose.yaml`](../../tools/pqs/compose.yaml), reading as the same token, after [`wait-ready.sh`](../../tools/pqs/wait-ready.sh) |

```text
### step 2: cargo test --workspace --all-features
test lower::tests::a_daml_name_rust_cannot_spell_is_skipped_not_panicked_on ... ok
passed: 600 failed: 0
### step 3: conformance
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 5 tests
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 51 tests
test result: ok. 51 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
capabilities: 51  gaps: 0
### step 4: external signing
committed 122070b591b0a0b519fa1d06c4809003ae6f0d063b48f72186ba309276041f4f31e1 at offset 303 with 1 event(s)
test external_commands__prepare_sign_execute_commits_a_transaction ... ok
test signing__an_external_party_is_onboarded_by_signing_its_own_topology ... ok
rejected as expected: grpc status Client specified an invalid argument: FAILED_TO_EXECUTE_TRANSACTION(8,bbc5eeef): The participant failed to execute the transaction: Received 0 valid signatures from d
test signing__a_signature_from_another_key_is_rejected ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.91s
### step 5: json_packages
participant:    http://localhost:3975 (JSON Ledger API 3.5.7)
packages known: 60
de2cc2f90eb523414ff54e899951dadd8789a4c07e0f71f6d6c9eaf57d412a54: Registered
### step 6: token standard
test the_registry_says_who_administers_its_instruments ... ok
test paging_through_instruments_works_against_a_real_registry ... ok
test the_registry_declares_which_token_standard_versions_it_serves ... ok
test an_instrument_this_registry_does_not_issue_is_none_rather_than_an_error ... ok
test the_instruments_decode_as_the_standard_describes_them ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
--- v1_transfer
registry admin: DSO::1220da56976bc0f405bfffaf268f075cfb2d0d0224f2115b214a38cf03db1b2cedad
instrument:     Amulet (AMT), 10 decimals
holdings:       1 unlocked holding(s) of Amulet read from the ledger
kind:           self — sender and receiver match
disclosing:     4 contract(s) the registry named
committed 1220eeabaf4c53d837e1ee9ecd24e69e6c480b1c8e26763f3bd7f83685277d520705 at offset 311 with 5 event(s)
exit=0
--- v2_transfer
registry admin: DSO::1220da56976bc0f405bfffaf268f075cfb2d0d0224f2115b214a38cf03db1b2cedad
instrument:     Amulet (AMT), 10 decimals
from account:   "" / to account: ""
holdings:       2 unlocked holding(s) of Amulet read from the ledger
kind:           self — sender and receiver match
disclosing:     4 contract(s) the registry named
committed 122021d543f64f15dca59f5d0db3c6a923b622825b904ea06e266c9e7e91dfcdbed8 at offset 314 with 6 event(s)
  holdings change on 00bb34aac538c58ba3ca4a08d848d709d17dec3a0f3cf2ec32c124d01db1c2d357ca121220423f5b875fe0a0747bf01aaef96096929245507ad6b089128fd66028710e9af6 (node 12): 2 spent, 2 produced, 2 leg(s)
exit=0
--- v2_allocate
registry admin: DSO::1220da56976bc0f405bfffaf268f075cfb2d0d0224f2115b214a38cf03db1b2cedad
sender:         app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
receiver:       app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
executor:       app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
receiver = sender: authorising both sides of the leg in this allocation
holdings:       2 unlocked holding(s) of Amulet read from the ledger
disclosing:     2 contract(s) the registry named
allocated:      1220ea0fae9279d586d46b78a5382a195af5aa315ee8168a26cf1c3b8a6183aa7a1b at offset 317 with 2 event(s)

the executor settles this batch with:
  settlement id: dvp-1790073600615421
  executor:      app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
  v2::settle_batch(&registry, settlement, transfer_legs, allocations, vec![executor])
  created:       00b55f1fb85f23d86f6816e6ecbb1ca75bd35b052497575d1273a26c0ededaec3eca121220332df3d5975bcf7c4f8fceca7abeed58adf50c249a71cd342b4d3fa27700803e
exit=0
--- v2_withdraw_allocation
sender:      app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
allocations: 1 active
  00b55f1fb85f23d86f6816e6ecbb1ca75bd35b052497575d1273a26c0ededaec3eca121220332df3d5975bcf7c4f8fceca7abeed58adf50c249a71cd342b4d3fa27700803e
    settlement dvp-1790073600615421 by app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e, 0 holding(s) reserved
withdrawing 00b55f1fb85f23d8 (settlement dvp-1790073600615421): disclosing 0 contract(s) the registry named
withdrawn:   1220b6c66e3c202d0ac7b55fa693c1e95e5486302408c41422e73cbe9d9eafa4190b at offset 320 with 2 event(s)
exit=0
--- v2_allocate
registry admin: DSO::1220da56976bc0f405bfffaf268f075cfb2d0d0224f2115b214a38cf03db1b2cedad
sender:         app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
receiver:       app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
executor:       app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
receiver = sender: authorising both sides of the leg in this allocation
holdings:       2 unlocked holding(s) of Amulet read from the ledger
disclosing:     2 contract(s) the registry named
allocated:      12206f7aeb40c08e1dbf9f512c8dfaec2ee02c5648b0309478834984f56b500189a6 at offset 323 with 2 event(s)

the executor settles this batch with:
  settlement id: dvp-1790073604041114
  executor:      app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
  v2::settle_batch(&registry, settlement, transfer_legs, allocations, vec![executor])
  created:       00a85b1e5b537e01969d0f19cad1355c584cbdb341594c7f8b91538d22be0f0a88ca121220c6e240b34c502e153e2da2d363c2b6ed423bd8634284ee731eb3890f79426ad6
exit=0
--- v2_settle
executor:    app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e
allocations: 1 naming it as executor
settlement dvp-1790073604041114: 1 allocation(s), 1 leg(s)
  leg leg-1: app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e -> app_provider_builder-localnet-1::122050a3b417325869df2a2248607ef4d408abdd45da8d7c4e774f96c090e32cd11e : 1.0000000000 Amulet
  disclosing 2 contract(s) the registry named
  settled: 12206f9fa56dbc7208b1534bfed792427d3d7314cb22db4bbaa1bf2f5ba69aeb9092 at offset 326 with 4 event(s)
exit=0
### step 7: pqs
 Container pqs-scribe-1 Started 
pqs ready at offset 326
test a_refused_connection_is_not_retried_forever ... ok
test pqs_reports_how_far_it_has_ingested ... ok
test an_ordered_comparison_is_a_statement_postgres_accepts ... ok
active Amulet: 2
test a_contract_read_from_postgres_is_the_generated_type ... ok
test a_contract_is_found_by_id ... ok
test containment_and_party_columns_work_against_the_real_schema ... ok
test the_acs_can_be_read_as_of_an_offset ... ok
test a_payload_predicate_filters_in_the_database ... ok
test a_connection_the_store_closes_is_replaced_on_the_next_read ... ok
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s
 Network pqs_default Removed
```

Every step passed as written. Step 4 is the one to read closely: the
onboarded party committed a create and the participant refused a signature
from another key. An earlier run of the same day reported the same three
tests green while two of them had quietly returned for want of an application
package; that silent return is gone, which is what `CANTON_TEST_REQUIRE_LIVE`
exists to guarantee.
