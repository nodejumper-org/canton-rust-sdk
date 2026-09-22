# Compatibility matrix

What this SDK is tested against, and what it therefore claims. Nothing is
listed on the strength of "it should work" — but not everything is checked by
CI, and the table under [What CI runs, and what it cannot](#what-ci-runs-and-what-it-cannot)
says exactly which rows rest on a developer-run live suite instead.

Version **0.3.0** (unreleased).

## Rust

| Toolchain | Status | Where it is checked |
|---|---|---|
| 1.88 (MSRV) | supported | the `msrv` CI job builds the whole workspace with it |
| stable | supported | every other CI job |
| beta / nightly | untested | — |

The MSRV is `workspace.package.rust-version`. Raising it is a minor version
bump, not a patch.

## Platform

| Target | Status | Where it is checked |
|---|---|---|
| Linux (`ubuntu-latest`) | supported | the `test` job's matrix |
| macOS (`macos-latest`) | supported | the `test` job's matrix |
| Windows (`windows-latest`) | supported | the `test` job's matrix |

The whole test suite runs on all three. The live suites need a Canton
participant and are run by a developer, not by CI — see below.

## Canton and the Ledger API

| | |
|---|---|
| Ledger API | **v2** |
| Canton | **3.5.7** — the release the `.proto` files are vendored from |
| Verified against | a Canton 3.5.7 participant (LocalNet): submission, streaming, recovery, TLS/mTLS, auth, typed end-to-end on both transports, and interactive submission with an externally-signed party. A Canton **3.5.17** participant on the Canton Network **DevNet** (Splice validator 0.8.1): the token-standard workflows, JSON package and party reads, gRPC over TLS with a Keycloak token |

Moving the supported Canton range re-vendors the protos in a new SDK minor. See
the stability policy in [`canton-proto`](../crates/canton-proto/src/lib.rs) and
[ADR-0002](adr/0002-proto-types-are-protocol-stable.md).

## Daml-LF

| Version | Status |
|---|---|
| LF 2.1 | supported |
| LF 2.2 | supported |
| LF 2.x, other minors | refused, naming the minor and this build's range |
| LF 1.x | refused, naming the **major** |

An LF 1 archive carries minors like `15`, so judging it by the minor gate would
report "Daml-LF 2.15 is unsupported" and send a reader to upgrade an SDK that
will never read it. The major is checked first for that reason.

The decoder is held to the official JVM reader by a conformance oracle — the
`lf-conformance` CI job.

## Token standard

| Version | Types | Workflow |
|---|---|---|
| **CIP-56** (`-v1`) | generated: holding, metadata, transfer instruction, allocation, allocation instruction, allocation request, burn/mint, featured app | `canton-token`: registry client, choice contexts with disclosure, `TransferFactory_Transfer`, accept/reject/withdraw, allocate + execute/withdraw/cancel |
| **CIP-0112** (`-v2`) | generated: holding (with `Account`), transfer instruction, transfer events (`EventLog`), allocation (with `SettlementFactory`, `Allocation_Settle`), allocation instruction, allocation request | `canton_token::v2`: the V2 registry paths, `TransferFactory_Transfer` over accounts, accept/reject/withdraw, allocate, `settle_batch`, allocation and allocation-instruction choices, and `events::holdings_changes` for reading what moved |

V2 reuses `splice-api-token-metadata-v1`; there is no `metadata-v2`.

Which standard each crate targets, crate by crate:

| Crate | Standard | What it holds |
|---|---|---|
| `canton-splice-api-token-metadata-v1` | V1 and V2 (shared) | `ChoiceContext`, `ExtraArgs`, `Metadata`, `AnyValue` |
| `canton-splice-api-token-holding-v1` | CIP-56 V1 | `Holding`, `InstrumentId`, `Lock` |
| `canton-splice-api-token-transfer-instruction-v1` | CIP-56 V1 | `TransferFactory`, `TransferInstruction` |
| `canton-splice-api-token-allocation-v1` | CIP-56 V1 | `Allocation`, `SettlementInfo`, `TransferLeg` |
| `canton-splice-api-token-allocation-instruction-v1` | CIP-56 V1 | `AllocationFactory`, `AllocationInstruction` |
| `canton-splice-api-token-allocation-request-v1` | CIP-56 V1 | `AllocationRequest` |
| `canton-splice-api-token-burn-mint-v1` | CIP-56 V1 | `BurnMintFactory` |
| `canton-splice-api-featured-app-v1` | V1 (featured-app API) | `FeaturedAppRight`, activity markers |
| `canton-splice-api-token-holding-v2` | CIP-0112 V2 | `Holding`, `Account`, `InstrumentId`, `Lock` |
| `canton-splice-api-token-transfer-instruction-v2` | CIP-0112 V2 | `TransferFactory`, `TransferInstruction` over accounts |
| `canton-splice-api-token-transfer-events-v2` | CIP-0112 V2 | `EventLog`, the transfer events `events::holdings_changes` reads |
| `canton-splice-api-token-allocation-v2` | CIP-0112 V2 | `Allocation`, `SettlementFactory`, `SettlementInfo`, `TransferLegSide` |
| `canton-splice-api-token-allocation-instruction-v2` | CIP-0112 V2 | `AllocationFactory`, `AllocationInstruction` |
| `canton-splice-api-token-allocation-request-v2` | CIP-0112 V2 | `AllocationRequest` |
| `canton-splice-amulet`, `canton-splice-wallet`, `canton-splice-wallet-payments` | V1 and V2 — Amulet implements both | the Splice application packages, referencing the crates above rather than copying them |
| `canton-token` | V1 (`canton_token::*`) and V2 (`canton_token::v2`) | the workflows |

**Verified against two live registries** — the Splice scan of a cn-quickstart
LocalNet (Splice 0.6.11), and the public Scan of the Canton Network **DevNet**
(Splice 0.8.1). The instrument on both is **Amulet (Canton Coin)**, which
declares both standards (`splice-api-token-transfer-instruction-v1` and
`-v2`). That is a V2 implementation, exercised as one. The registry's OpenAPI
documents at 0.8.1 differ from the vendored 0.6.11 copies by one additive
field, which the client already reads.

**On the "V2 reference token".** Splice documented a separate *Token Standard
V2 DevNet* — a temporary, single-SV network run by Digital Asset for
organizations validating the V2 standard, with a public scan and a weekly
Monday reset. It appears to have been wound down: the document's version line
was bumped weekly through 23 June 2026 and has not moved since, and the
network's hostnames are absent from the certificate its address now presents
(a valid certificate covering the ordinary DevNet names and no
`token-std-v2-dev` name at all — a retired ingress rather than an IP
allowlist, which would answer 403 over a good handshake). V2 has meanwhile
landed in the ordinary stack, which is why a plain LocalNet's Amulet declares
every V2 API. So the runs below are against a V2 implementation on a normal
network, which is the target that exists.

| What | LocalNet (Canton 3.5.7) | DevNet (Canton 3.5.17) |
|---|---|---|
| V1 transfer, end to end | committed at offset 67684, 6 events, kind `offer` | committed at offset 3316298, kind **`direct`** — settled on submission |
| V2 `Account`-based transfer | committed at offset 67690, 6 events | committed at offset 3316304, `direct` |
| V2 event parsing (`events::holdings_changes`) on a committed transaction | one holdings change: 1 spent, 2 produced, 2 legs | two holdings changes: 1 spent / 1 produced, 0 spent / 1 produced |
| V2 **allocation**, three distinct parties (sender / receiver / executor) | allocated at offset 67693, 6 events, 3 contracts created | allocated at offset 3316319, executor the validator's party |
| V2 allocation **withdrawn** by the sender | — | withdrawn at offset 3316364; the reserved coin released |

Both runs are kept verbatim, with their environment and commands, in
[`docs/verification/token-standard-live-runs.md`](verification/token-standard-live-runs.md);
the DevNet update ids were read back from the participant by id after the run.

The allocation is the half that needs more than two parties: the sender
reserves holdings for a settlement a third party — the executor — completes
later, which is what delivery-versus-payment is built from. Settling is not
run here, and deliberately: it belongs to the executor, from their own
participant with their own credentials, and one process holding both sides
would demonstrate nothing.

Each run resolved the factory against the registry and submitted with the
contracts it named for disclosure, so the choice-context and
`createdEventBlob` → `DisclosedContract` path are proven against a real
registry rather than a stub.

The stub has its place too, and CI runs it: every workflow function in
`canton-token` — V1 and V2 transfer, the instruction choices, allocation,
`settle_batch`, the allocation and allocation-instruction choices — is driven
against an in-process registry that answers what the OpenAPI documents say and
records what it was asked, and the holdings read is driven against an
in-process `StateService`. What those pin is the request each workflow sends
and that the registry's context lands *inside* the exercise argument, which is
the part a mutation run found nothing else guarding.

The V1 packages come from cn-quickstart at a pinned commit, fetched in CI and
checked against their SHA-256. The V2 packages ship as no DAR anyone publishes,
so they were taken from a participant and committed under
[`testdata/token-standard-v2`](../testdata/token-standard-v2/README.md), where
each file name ends with the package id that hashes its bytes.

## PQS

| | |
|---|---|
| Scribe | **3.5.4** |
| Verified against | a running store: 969 active contracts read as typed payloads, payload and party-column predicates, containment, lookup by id, pinned-offset reads |

## What CI runs, and what it cannot

| Suite | CI | Needs |
|---|---|---|
| unit, in-process, TLS, WebSocket | yes | nothing |
| submit → observe → query, on gRPC and on JSON, as one flow | yes, against an in-process participant (`submit_observe_query_*` in `canton-ledger`'s tests) | nothing; the same flow runs against a real participant in the live suites (`create_contract_and_read_transaction`, `json_submit_and_read_back`) |
| conformance (`canton-conformance`) | yes | nothing |
| bindings drift, eighteen of the nineteen generated crates | yes | the pinned DARs, fetched and checksummed; the V2 and stdlib crates need nothing. `canton-quickstart-licensing` is the exception — its DAR is built from source, so it is guarded locally and not in CI |
| Daml-LF conformance oracle | yes | a JVM |
| live Ledger API, interactive submission | no | a Canton participant |
| live PQS | no | a Scribe store |
| token-standard registry | no | a Splice scan — the **super-validator** runs one, so a LocalNet with `SV_PROFILE=on` has it (cn-quickstart serves it on `:5012` and does not publish that port to the host) |

A suite CI cannot run is gated on an environment variable and **fails rather
than skips** when that variable is set — a connection failure and an
unconfigured machine used to look the same in a green run, and they are not the
same thing.

## Conformance to the Ledger Client Standard

[`conformance/capabilities.toml`](../conformance/capabilities.toml) lists every
capability of the standard this SDK claims, and `canton-conformance` has one
test named for each. A completeness guard asserts the two agree in both
directions: no capability without a test, and no test claiming a capability the
registry does not list.

One row is worth reading carefully. **Contract keys** are generated and
exercisable by key, but no template in this repository's corpus declares a key —
none of the Splice DARs does — so the conformance test asserts the mechanism is
present rather than exercising a round trip. The emission itself is covered by
`canton-codegen`'s own tests.
