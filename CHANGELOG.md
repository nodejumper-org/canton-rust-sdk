# Changelog

All notable changes to the Canton Rust SDK are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generated protobuf types (the `canton-proto` crate and the `proto` re-exports)
are **exempt from SemVer** — see the stability policy in `canton-proto`'s docs.

## [0.2.0] — unreleased

All `canton-*` crates release in lockstep, so the M1 crates move to 0.2.0 with
the rest. Everything the 0.1.x line gained after the M1 submission — the read
request builders, the full `Commands` surface, Canton-native error
classification (see 0.1.2 below) and the documentation fixes of 0.1.3/0.1.4 —
is included; nothing from it was removed or changed in signature.

### Added — type-safe codegen from DARs

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
  The package segment is the package **name** — a version bump does not rename
  the paths a consumer imports — with the version appended only to separate two
  packages that would otherwise share a module name. Each template's docs carry
  its on-ledger id and its choices with their consuming flag.
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

### Changed

- **`canton-core` gains `Error::Payload`** — the one addition to the surface
  published in 0.1.x. It carries a `canton-daml` codec failure as its `source`,
  so a typed decode and a transport failure land in the same `Result` and an
  application can stay on `canton::Error` end to end. `Error` is
  `#[non_exhaustive]`, so the new variant does not break a `match`.
  `canton-daml` supplies the bridge (`impl From<ValueError> for
  canton_core::Error`), and `ValueError` is structured rather than a string:
  it carries the field **path** it failed at and the message separately, so the
  path survives into the error chain instead of being formatted away.
- The `canton` facade re-exports the codegen runtime as [`canton::daml`], so
  `cargo add canton` gets a version-locked runtime for generated bindings. The
  generator stays out of the facade on purpose: it is a build-time tool
  (`cargo install canton-codegen-cli`, or depend on `canton-codegen` from a
  build script).
- The codegen pipeline (`generate`, `Options`, `Runtime`, `Stats`) lives in
  `canton-codegen`, not in the `-cli` crate, so a build script can call it
  without pulling in a CLI. `canton-codegen-cli` is binary-only.
- Codegen errors are typed: `GenerateError` and `CodegenError` replace
  `Box<dyn Error>` and `syn::Error`, and `SkippedType` (with `module()` /
  `reason()`) replaces a bare string. No `syn`/`proc-macro2` type appears in a
  public signature.
- `Options` uses the same consuming-builder style as M1's `Config`, and
  `Options`/`Stats`/`Runtime` are `#[non_exhaustive]`.
- Generated crates get a publishable `Cargo.toml` (description, no forced
  `[workspace]` stanza) and their default name drops the DAR's version suffix,
  so a DAR bump no longer renames the crate a caller depends on.
- Every published crate now ships the Apache-2.0 licence text, and integration
  tests whose fixtures live outside the package are no longer packaged.

> **Publish order:** `canton-proto` → `canton-core` → `canton-auth` →
> `canton-lf` → `canton-daml` → `canton-ledger` → `canton-admin` →
> `canton-codegen` → `canton-codegen-cli` → `canton` → the `canton-splice-*`
> bindings. `canton-sample` and `canton-quickstart-licensing` stay unpublished
> (reference material).
## [0.1.4] - 2026-08-05

Documentation and developer-experience fixes from the Canton Foundation review
of the M1 submission. No library code changes; `0.1.3` and `0.1.4` are the same
SDK.

### Fixed

- **Examples:** `version_and_health` and `submit_and_read` read the same
  `CANTON_TEST_*` variables as the live tests (the unprefixed `CANTON_*`
  spellings still work and take precedence). They previously required a second,
  undocumented set of six names, so following the README's own setup and then
  running the example it links to failed on `set CANTON_TOKEN_URL`.
- **Live tests:** the ACS token-paging test no longer caps the walk at 500
  pages. At page size 1 that bounded the ledger rather than the loop, so on a
  participant with a few hundred contracts a healthy walk reported "paging did
  not terminate". It now fails when the page token stops advancing, which is
  the condition it was meant to catch.

### Documentation

- **README:** the live-test variables are documented under their real names
  (every one is `CANTON_TEST_`-prefixed) in a table that says what each gates,
  with a copy-pasteable export block. `CANTON_TEST_JSON_ENDPOINT` (the JSON and
  WebSocket tests) and `CANTON_TEST_SYNC_ID` were previously undocumented, and
  six names were listed without their prefix.
- **README:** node setup lists the
  [Canton Builder Tool](https://canton-network-devs.github.io/Canton-Builder-Tool/#part-builder)
  and [Splice LocalNet](https://docs.sync.global/app_dev/testing/localnet.html)
  alongside `cn-quickstart`, with what each one actually provides: Splice
  LocalNet is unauthenticated, so the OIDC-gated tests skip there as well as the
  command-submission ones; the Builder Tool's `canton builder deploy` uploads a
  DAR, which is how to get `CANTON_TEST_LICENSING_PKG` without cn-quickstart.

## [0.1.3] - 2026-08-04

### Fixed

- **`canton-ledger`:** `CompletionsRequest`'s JSON body builder is now gated on
  the `ws` feature, which is the only thing that reads it — the JSON Ledger API
  has no POST endpoint for completions. Without the gate the method was dead
  code in any build without `ws`, which failed the workspace's own
  `-D warnings` on the feature powerset. **Consumers of 0.1.2 are unaffected**:
  Cargo caps lints in registry dependencies, so a `default-features = false`
  build of the published crate compiles either way.

## [0.1.2] - 2026-08-04

Additive (semver-compatible) API growth from external M1 review feedback:
request builders for the read streams, and Canton-native error classification.

### Added

- **`canton-ledger` (request builders):** `UpdatesRequest`,
  `ActiveContractsRequest`, and `CompletionsRequest` (module `request`,
  re-exported at the root) open up the request surface the plain methods
  hard-code — on **every read path and both transports**: gRPC
  (`updates_with`, `updates_resumable_with`, `updates_page_with`,
  `active_contracts_with`, `active_contracts_page_with`,
  `active_contracts_resumable_with`, `completions_with`), JSON POST
  (`JsonClient::updates_with`, `active_contracts_with`), and WebSocket
  (`ws_updates_with`, `ws_active_contracts_with`, `ws_completions_with`):
  - bounded streams (`until(end_inclusive)`) for catch-up/sync reads that
    terminate at an offset instead of staying live;
  - template and interface filters (`for_templates` / `for_interfaces`,
    accepting `package:Module:Entity` ids with `#package-name` references)
    instead of the wildcard filter;
  - transaction shape selection (`TransactionShape::AcsDelta` /
    `LedgerEffects`), created-event blobs, topology events, dropping
    reassignments, and non-verbose records;
  - `UpdatesRequest::descending` — bounded newest-first reads on the stream
    and page paths, both transports (rejected up front on the resumable
    stream, whose position tracking assumes ascending order);
  - `for_any_party` (updates and ACS) — the `filters_for_any_party` /
    `filtersForAnyParty` full-ledger ingestion form, for tokens with
    wildcard read authorization;
  - `CompletionsRequest::with_user_id` to scope the completion stream to a
    submitting user id (pairs with `Submit::with_user_id`) — required for
    tokens that do not carry a user id.

  Defaults match the plain methods exactly (`updates_with(UpdatesRequest::
  new(p, o))` ≡ `updates(p, o)`), whose signatures are unchanged — the plain
  methods now delegate to the builders, so there is a single body/request
  producer per query on each transport.
- **`canton-ledger` (submit):** `Submit::with_transaction_shape` selects the
  shape of the transaction returned by `submit_and_wait_for_transaction`
  (ledger effects — the default, as before — or ACS delta), removing the last
  hard-coded `TransactionShape`.
- **`canton-ledger` (submit, full `Commands` surface):** every remaining
  `Commands` field is now settable on both transports —
  `Submit`/`JsonCommands` gain `with_submission_id`, `add_disclosed_contract`
  (the explicit-disclosure counterpart to the created-event blobs the read
  builders expose), `with_package_id_selection_preference` (the SCU upgrade
  pin), and `with_min_ledger_time_abs`/`with_min_ledger_time_rel`; `Submit`
  additionally gains `with_prefetch_contract_keys` and `with_taps_max_passes`,
  and `JsonCommands` gains `with_deduplication_period` (raw JSON, mirroring
  `Submit::with_deduplication_duration`/`_offset`).
- **`canton-ledger` (JSON retries):** `JsonClient::with_retry(RetryConfig)` —
  the same opt-in retry pipeline as the gRPC client (category-first
  classification of the participant's error body, exponential backoff,
  server-recommended delays honoured; de-duplication-safe for submits since
  the command id stays fixed across attempts).
- **`canton-core` (error classification):** errors now expose the precise
  retry semantics the Ledger API encodes, beyond the transport status code —
  on **both transports**:
  - `ErrorCategory` — Canton's error categories (from
    `ErrorInfo.metadata["category"]` on gRPC statuses, or the
    `errorCategory` field of JSON API error bodies) with the retryability the
    error-code docs define; surfaced as `Error::category()`.
  - `Error::retry_delay()` — the server-recommended pause, from the
    `google.rpc.RetryInfo` detail (gRPC) or the `retryInfo` field (JSON).
  - `Error::correlation_id()` — from the `google.rpc.RequestInfo` detail
    (gRPC) or `correlationId`/`traceId` (JSON), for quoting to the
    participant operator.
  - `Error::is_retriable` is now **category-first** for both `Error::Status`
    and `Error::Http`: the category's verdict wins over the gRPC/HTTP status
    code (e.g. a category-12 "seek after ledger end" on HTTP 400 is
    retryable; a category-10 "resource exists" on `ABORTED` is not), a bare
    `RetryInfo` counts as retryable, and the previous code-based
    classification remains as the fallback for non-Canton errors.
    **Behaviour change** under opt-in retry: HTTP errors that Canton marks
    retryable now retry where they previously failed fast — including the
    413 `JSON_API_MAXIMUM_LIST_ELEMENTS_NUMBER_EXCEEDED` (category 2 with a
    1-second `retryInfo`), even though shrinking the request `limit` is the
    actual remedy. The SDK follows the server's verdict rather than
    special-casing codes; callers that want to fail fast on it can match on
    `Error::Http { status: 413, .. }` or handle it before retrying.
  - `retry::run_with_retry` honours the server's `retry_delay` when it
    exceeds the local backoff.

### Fixed

- **`canton-ledger` (WebSocket error frames):** a `JsCantonError` frame on a
  WS stream is now surfaced as `Error::Http` (with the gRPC code's canonical
  HTTP status and the full error body), instead of `Error::CommandRejected`.
  The frame is the same error object the JSON API returns in HTTP error
  bodies, so it now carries its `category()`/`retry_delay()`/
  `correlation_id()` — and, **behaviour change**, `ws_updates_resumable` now
  correctly reconnects on transient participant errors (categories 1/2/…),
  which the never-retriable `CommandRejected` mapping previously turned into
  a dead stream.

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
