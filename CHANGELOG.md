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

### Fixed — pre-release hardening (M2 quality audit)

- **Recursion is now Rust-sound.** `Optional` self-recursion (`data Tree = Node
  { left : Optional Tree }`), mutual recursion (including cross-module), and
  recursion through a generic instantiation previously generated
  infinitely-sized structs (E0072); a crate-wide containment-cycle breaker now
  boxes every cycle-closing reference occurrence.
- **Record decode matches Canton's wire shapes.** Generated `FromValue` locates
  fields by label *or* declaration index (non-verbose output omits labels) and
  decodes an absent trailing `Optional` field as `None` (record normalization
  under Smart Contract Upgrade drops them).
- **`Numeric` compares numerically**, not textually — the ledger echoes `"1.5"`
  as `"1.5000000000"` and the two are now equal (`Eq`/`Ord`/`Hash` on the
  canonical decimal; `Numeric::parse` validates early).
- **Typed read path:** `Template::from_created_event` decodes a `CreatedEvent`
  into the payload struct (with a template-identity check);
  `ContractId::retag` re-tags an id for interface exercise.
- **Codegen robustness on arbitrary DARs:** per-entry decompression cap
  (zip-bomb guard), typed errors naming the offending file/package, `$`-named
  GHC-internal types skip instead of panicking the emitter, snake-case field
  collisions and duplicate package modules are detected, generated code spells
  std types fully qualified so same-named Daml types cannot shadow them.
- **CLI safety:** refuses to overwrite files it did not generate (`--force` to
  override); validates the crate name; the generated crate's version is the
  DAR package's version; `--runtime-path` is absolutized and TOML-escaped.
- Always-on fixture tests: `testdata/splice-api-token-holding-v1-1.0.0.dar`
  runs the full decode → lower → emit pipeline (plus determinism and CLI
  contract checks) in every `cargo test`, no external setup.

> **M2 release checklist:** publish in dependency order — `canton-lf` →
> `canton-daml` → `canton-codegen` → `canton-codegen-cli` → the
> `canton-splice-*` bindings (their crate version follows the SDK lockstep;
> each README states the DAR version it was generated from). The
> `dpm-codegen-rust` default `--runtime-version` (`0.1`) resolves once
> `canton-daml` is live. `canton-sample` and `canton-quickstart-licensing`
> stay unpublished (reference material).

## [0.1.1] - 2026-07-22

Hardening patch over 0.1.0, from an adversarial pre-release review. Each fix has
a regression test. Semver-compatible, so `^0.1` resolves to it.

### Fixed

- **`canton-core` (TLS):** `Config::with_tls` no longer connects in plaintext on
  an `http://` endpoint. `tonic` selects the TLS handshake from the URI scheme,
  not from the presence of a `tls_config`, so TLS was silently ignored (no
  encryption, no server-certificate verification, no client certificate for
  mutual TLS). The scheme is now normalised to `https://` when TLS is
  configured, case-insensitively.
- **`canton-ledger` (JSON and WebSocket TLS):** the same normalisation on
  `JsonClient::with_tls`. An `http://` base URL previously sent plaintext HTTP
  with the configured certificates unused and opened a `ws://` socket; the base
  URL is now upgraded to `https://` so both the HTTP and WebSocket lanes use TLS.
- **`canton-ledger` (JSON updates):** the JSON update stream now requests
  `includeReassignments`, matching the gRPC transport. **Behaviour change:** a
  JSON update stream now surfaces the assigned/unassigned reassignment events it
  silently dropped before.
- **`canton-auth`:** the cached-token TTL is clamped to 30 days, so a token
  endpoint reporting an absurd `expires_in` can no longer overflow the cache
  deadline and panic.
- **`canton-core` (retry):** `Error::is_retriable` now treats the whole `5xx`
  range as retriable (plus `408` and `429`), instead of a hand-picked subset
  that missed codes such as `501`, `507`, `509`, `511`, and `520`.

## [0.1.0] - 2026-07-20

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
