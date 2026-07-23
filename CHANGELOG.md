# Changelog

All notable changes to the Canton Rust SDK are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generated protobuf types (the `canton-proto` crate and the `proto` re-exports)
are **exempt from SemVer** — see the stability policy in `canton-proto`'s docs.

## [Unreleased]

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
