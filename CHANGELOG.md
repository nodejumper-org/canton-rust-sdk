# Changelog

All notable changes to the Canton Rust SDK are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generated protobuf types (the `canton-proto` crate and the `proto` re-exports)
are **exempt from SemVer** — see the stability policy in `canton-proto`'s docs.

## [Unreleased]

### Added — Milestone 2: type-safe codegen from DARs

- **`canton-lf`** — native Rust Daml-LF reader: DAR container (zip +
  `MANIFEST.MF`) and an LF 2.x protobuf decoder (vendored `daml_lf2.proto`),
  including the LF 2.dev additions — the explicit package-import table and
  curried type application (`TApp`) — used by SDK 3.5 DARs. Decodes a DAR's whole
  dependency closure (`decode_all`).
- **`canton-daml`** — the runtime generated code depends on (as `rt`): the Daml
  primitive types (`Party`, `ContractId<T>`, `Numeric`, `Timestamp`, `Date`,
  `TextMap`, `GenMap`, `NestedOpt`, `Unit`), the `ToValue`/`FromValue` gRPC
  codecs and serde JSON conventions (LF-JSON: `Int64` and `Numeric` as strings —
  accepting a number on input — the nested-optional list form, `Unit` as `{}`,
  adjacently-tagged variants, …), the `Contract`/`Template`/`Interface`/
  `WithKey`/`Choice` traits, and the `create_command`/`exercise_command`/
  `exercise_by_key_command` builders.
- **`canton-codegen`** — DAR → typed Rust: a decoder-agnostic IR, the documented
  Daml-LF → Rust type mapping (see `docs/daml-lf-type-mapping.md`), and emission
  of records, variants, enums, **templates** (payload + typed choices + on-ledger
  id), **interfaces** (marker + view + choices), and **contract keys**, with JSON
  and gRPC codecs. Output is a fully-qualified `pub mod` tree
  (`crate::<package>::<module>::<Type>`) so cross-package references resolve and
  names never collide; template ids use the SCU-friendly `#<package-name>` form.
- **`canton-codegen-cli`** — the `dpm-codegen-rust` binary (a `dpm codegen-rust`
  component / build-script tool) that writes a self-contained bindings crate from
  a DAR.
- **Pre-built bindings crates** (checked-in generated output, drift-guarded):
  `canton-splice-amulet`, `canton-splice-wallet`, `canton-splice-wallet-payments`,
  and `canton-quickstart-licensing`.
- **`canton-sample`** — reference app: builds a typed `AppInstallRequest` from
  the bindings, round-trips both codecs, and runs the full verification loop
  (submit → observe transaction → query ACS) over gRPC and JSON.
- **Verified** on a live cn-quickstart participant: the whole-DAR closure
  compiles for all corpus DARs, and the sample's typed create commits and is read
  back from the ACS on both transports.

> **M2 release checklist (before publishing the M2 crates):** the M2 crates are
> `publish = false`. Before release, flip them to publishable, then publish
> `canton-daml` first; the `dpm-codegen-rust` default `--runtime-version` (`0.1`)
> only resolves once `canton-daml` is on crates.io (until then, generated crates
> use `--runtime-path`). Also decide the crate-version ↔ DAR-version convention
> for the `canton-splice-*` crates.

### Added — Milestone 1: core Ledger API client, auth & PoC

- **`canton`** — the SDK entry point: a thin facade (re-exports only, no
  logic) over the crate family — `canton::ledger` / `canton::auth` /
  `canton::admin`, the shared `Config`/`Error`/TLS/retry types at the root,
  and the `ws`/`otel` features forwarded (ADR-0007).

- **`canton-ledger`** — async Ledger API v2 client over gRPC:
  - Command submission: fire-and-forget `submit`, `submit_and_wait`
    (update id), and `submit_and_wait_for_transaction`; change-ID
    de-duplication; completion stream + `await_completion` recovery; the
    `Submit` builder (`act_as`/multi-party, `user_id`, `read_as`,
    `workflow_id`, `synchronizer_id`, de-duplication period).
  - Reads & streams: ACS snapshot + paging (+ **resumable** paged ACS),
    updates stream + paging + reverse-order (+ **resumable** from the last
    offset), event query by contract id, node health
    (`grpc.health.v1`).
  - JSON transport: command submission, bounded ACS/update reads, and —
    behind the `ws` feature — WebSocket streaming (updates, ACS,
    completions, **resumable** updates), TLS-aware.
  - TLS/mTLS on gRPC, HTTP, and WebSocket from one shared `TlsConfig`.
- **`canton-core`** — shared kernel: typed `Error` with
  retriable/non-retriable classification and structured
  `google.rpc.ErrorInfo` details; `Config` (endpoint, auth, TLS, timeout);
  opt-in retry with per-attempt timeouts and jittered exponential backoff;
  telemetry (`tracing` spans + `metrics` counters per method/transport,
  optional OTLP export + W3C trace-context propagation behind `otel`).
- **`canton-auth`** — OIDC client-credentials `TokenProvider` with caching,
  refresh, bounded fetch, credential-rejection surfacing (`Error::Auth`),
  and Keycloak/Auth0/Okta presets.
- **`canton-admin`** — party allocation/management, user self-inspect,
  packages read, and topology read (party→participant mappings, namespace
  delegations, vetted packages) over the Canton admin API.
- **`canton-proto`** — generated types/stubs from vendored protos pinned to
  Canton 3.5.7 (Ledger API v2, Canton admin-api topology read, gRPC health).
- Runnable examples (`version_and_health`, `submit_and_read`), CI
  (fmt/clippy/tests on Linux+macOS+Windows/docs/deny/MSRV/feature matrix).
