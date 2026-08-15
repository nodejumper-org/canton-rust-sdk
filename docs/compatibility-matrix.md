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
| Verified against | a Canton 3.5.7 participant: submission, streaming, recovery, TLS/mTLS, auth, typed end-to-end on both transports, and interactive submission with an externally-signed party |

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

**Verified against a live registry** — the Splice scan of a cn-quickstart
LocalNet. The instrument is **Amulet (Canton Coin)**, which declares both
standards (`splice-api-token-transfer-instruction-v1` and `-v2`). That is a V2
implementation, exercised as one.

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

| What | Result |
|---|---|
| V1 transfer, end to end | committed at offset 39643, 5 events |
| V2 `Account`-based transfer | committed at offset 39646, 5 events |
| V2 event parsing (`events::holdings_changes`) on a committed transaction | one holdings change: 1 holding spent, 2 produced, 2 transfer legs |
| V2 **allocation**, three distinct parties (sender / receiver / executor) | allocated at offset 40176, 6 events, 3 contracts created |

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
