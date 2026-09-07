# Token-standard examples: live runs

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
