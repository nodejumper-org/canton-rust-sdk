# Compatibility matrix

What this SDK is tested against, and what it therefore claims. Every row here
is exercised in CI; nothing is listed on the strength of "it should work".

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
| **CIP-0112** (`-v2`) | generated: holding (with `Account`), transfer instruction, transfer events (`EventLog`), allocation (with `SettlementFactory`, `Allocation_Settle`), allocation instruction, allocation request | `canton_token::v2`: the V2 registry paths, `TransferFactory_Transfer` over accounts, accept/reject/withdraw, allocate, `settle_batch`, allocation and allocation-instruction choices |

V2 reuses `splice-api-token-metadata-v1`; there is no `metadata-v2`.

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
| bindings drift, all nineteen generated crates | yes | the pinned DARs, fetched and checksummed; the V2 and stdlib crates need nothing |
| Daml-LF conformance oracle | yes | a JVM |
| live Ledger API, interactive submission | no | a Canton participant |
| live PQS | no | a Scribe store |
| token-standard registry | no | a Splice scan; a LocalNet running only a validator has none |

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
