# Changelog

All notable changes to the Canton Rust SDK are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generated protobuf types (the `canton-proto` crate and the `proto` re-exports)
are **exempt from SemVer** — see the stability policy in `canton-proto`'s docs.

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
