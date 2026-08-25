# Changelog

All notable changes to the Canton Rust SDK are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generated protobuf types (the `canton-proto` crate and the `proto` re-exports)
are **exempt from SemVer** — see the stability policy in `canton-proto`'s docs.

## [Unreleased]

### Fixed

- **A gRPC stream or submit did not retry a connection that died in flight.**
  tonic surfaces a dropped connection mid-call as a `Status` with code
  `Unknown` (message "transport error" / "h2 protocol error"), not as a
  `transport::Error` — that variant is only for connection *establishment*. So
  `Error::is_retriable()` treated the most ordinary failure there is as
  terminal: `updates_resumable` gave up on the first blip and `run_with_retry`
  never fired. Data integrity was never at risk (no loss, no duplicate — the
  failure surfaced as an error), but the resilience did not engage. A
  category-less `Unknown`/`Internal` status whose source chain is populated is
  now retriable; a bare server `Unknown` (decoded from trailers, no source)
  stays terminal. Found by killing a real TCP connection under load through a
  chaos proxy — in-process mocks had emitted `Status::unavailable`, which is
  why it never showed. The WebSocket resumable lane was already correct (it
  treats a socket close as a reconnect trigger, not via this predicate).
- **`Template::from_json_created_event` rejected the `events-by-contract-id`
  response shape.** That endpoint wraps the created event as
  `{"created": {"createdEvent": …}}`; the helper peeled only the inner key and
  errored with "carries no createArgument". It now unwraps both layers, so all
  four JSON shapes the API uses decode.

- **A test broke `cargo test` without `--all-features`.** `futures_are_spawnable`
  asserted `JsonSubmission::recover` is `Send`, but that method is `ws`-gated
  while the assertion was not — so the default-feature build failed to compile.
  Invisible because CI ran tests only with `--all-features`; a second
  `cargo test --workspace` job (default features) now guards the whole class.
  Found by running the suite under every feature combination.
- **`tokio-stream`'s declared lower bound was `0.1`, but the `net` feature it
  uses landed in `0.1.1`** — a minimal-versions resolve does not build against
  `0.1.0`. Corrected to `0.1.1`. Found by `cargo +nightly build
  -Z direct-minimal-versions`.

## [0.2.2] — 2026-08-24

### Added

- **Conversion and hashing polish from a Rust API Guidelines audit** (16
  observations, none blocking; the audit's two breaking items are *decided*
  rather than fixed — [ADR-0010](docs/adr/0010-api-decisions-deferred-to-1.0.md)
  defers the `*ParseError` renames and the removal of `Party`'s unvalidated
  `From<&str>`/`From<String>` to 1.0, the latter blocked from gaining a checked
  `TryFrom` today by std's blanket impl).
  - `ChangeId` derives `Hash` — it is the key the docs tell applications to
    track submissions by, and it could not be a `HashMap` key. `Store`,
    `TransactionShape` and `ClientAuth` follow.
  - `canton-ledger` and `canton-admin` re-export `Auth`, `ErrorInfo` and
    `ResourceInfo` (and admin gains `ErrorCategory`): these appear in the
    signatures of `Error::error_info`/`resource_info` and `Config::auth`, so a
    user of one client crate could not previously name the types their own
    functions return without adding a `canton-core` dependency.
  - `ErrorCategory` implements `TryFrom<i32>` and `From<ErrorCategory> for i32`
    alongside the inherent methods.
  - `#[must_use]` on the constructors that lacked it; docs.rs feature badges on
    all seven remaining `ws`-gated methods (two of nine had them).
- **A compile-time proof that every public future is `Send`**
  (`tests/futures_are_spawnable.rs`) — an async API that cannot be
  `tokio::spawn`ed is unusable in a server, and the property breaks silently.
- **Hermetic coverage for the paths only live tests exercised**, found by
  measuring coverage rather than reading it: the three `TopologyClient` list
  reads (36% file coverage, all three public methods untested without a node —
  the same shape of gap the M1 review named about live tests), both recovery
  handles' `recover`, and `Submission::submit_and_wait`. Hand-written code
  measures 87.7% line coverage; `cargo udeps` reports no unused dependencies.

### Fixed

- The in-process duplicate-submission mocks answer synchronously on the
  fire-and-forget lane; a live Canton 3.5.7 participant does not — it accepts
  the RPC and reports the rejection on the completion stream. Established by
  adversarial probes run with the published 0.2.1 crates against a real node
  (which also confirmed the JSON duplicate answer is exactly the
  HTTP 409 + `DUPLICATE_COMMAND` the `is_duplicate_submission` predicate was
  written against, and that OIDC refresh survives a real token expiry). The
  mocks stay as pins of the synchronous case; their comments now say where
  reality differs.

## [0.2.1] — 2026-08-24

### Fixed

- **`canton-ledger` documented as failed on docs.rs.** Two
  `#[cfg_attr(docsrs, doc(cfg(feature = "ws")))]` attributes went in without the
  crate declaring `#![cfg_attr(docsrs, feature(doc_cfg))]`, which that attribute
  requires. `cargo doc` never sets `docsrs`, so the attributes compiled out and
  every gate stayed green; docs.rs does set it, on nightly, and the published
  crate's documentation page said the build failed. The crates themselves
  installed and built correctly throughout — this only ever affected the
  rendered documentation, and it cannot be fixed for a version already
  published, hence this patch.
- A `docsrs` CI job now builds the workspace the way docs.rs will, so the class
  of error that produced this cannot reach a release again.
- **The MSRV job never checked the MSRV.** `rust-toolchain.toml` pins the
  channel for anything run inside the checkout and wins over the toolchain the
  CI action installs — rustup says so in its own log, then uses stable. So a job
  that asked for 1.88 ran on stable, and `rust-version = "1.88"` went unverified
  every time it was claimed. It happens to be true (1.88 checks clean), but
  nothing was establishing that. Both jobs that want a specific toolchain now
  set `RUSTUP_TOOLCHAIN`, which is the override that beats the file.
- The `canton` facade declared `all-features` for docs.rs without the matching
  `rustdoc-args`, the same inconsistency one step behind.
- `cargo-semver-checks` covered 5 crates of 11. The M2 crates had no published
  baseline to compare against until 0.2.0, and the job's own comment said they
  should join afterwards — this is afterwards. `canton-proto` stays out by
  ADR-0002 and `canton-codegen-cli` has no library API.

### Changed

- `RELEASING.md` carries the post-publish verification the pre-publish gate
  cannot do: `cargo package` runs `--no-verify` because verifying needs the
  siblings to already be on crates.io, so the packaged artefact is first built
  by whoever installs it. The runbook now says to be that person first.
- The README compatibility table listed 0.2 as unreleased.

## [0.2.0] — 2026-08-24

All `canton-*` crates release in lockstep, so the M1 crates move to 0.2.0 with
the rest. Everything the 0.1.x line gained after the M1 submission — the read
request builders, the full `Commands` surface, Canton-native error
classification (see 0.1.2 below) and the documentation fixes of 0.1.3/0.1.4 —
is included.

### Breaking, for code on 0.1.x

Two signatures change, both because the old one could not express a correct
call. Everything else from 0.1.x compiles unchanged.

- **`OidcConfig::auth0(domain, client_id, secret)` → `auth0(domain, audience,
  client_id, secret)`.** Auth0 answers a client-credentials request without an
  `audience` by issuing a token for its own userinfo endpoint, which a
  participant rejects — so the old preset could not produce a working request.
  The audience identifies your Auth0 API and cannot be derived from the domain.

  ```diff
  - OidcConfig::auth0("my.eu.auth0.com", "client-id", "secret")
  + OidcConfig::auth0("my.eu.auth0.com", "https://ledger.example", "client-id", "secret")
  ```

- **`CantonClient::await_completion(command_id, parties, offset, timeout)` →
  `await_completion(&ChangeId, offset, timeout)`.** Canton identifies a command
  by (user, acting parties, command id), and matching on the command id alone
  can return another application's completion. The parties move inside the
  change ID.

  ```diff
  - client.await_completion(&command_id, vec![party.clone()], offset, timeout)
  + client.await_completion(&ChangeId::new("", vec![party.clone()], &command_id), offset, timeout)
  ```

  Better still, take the identity from the submission rather than rebuilding it:
  `let submission = client.submission(submit);` then
  `submission.recover(offset, timeout)`.

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

### Added — a local network needs no configuration in your program

- **`canton_core::localnet`** reads a Splice LocalNet out of the environment —
  the variables [canton-devkit](https://github.com/bitdynamics-ab/canton-devkit)'s
  `localnet env` exports. After `eval "$(canton-devkit localnet env demo)"`,
  `Config::from_env()` is a working gRPC configuration and
  `JsonClient::from_env()` its JSON counterpart, with the participant's token
  attached; `Config::from_env_for("app-user")` reaches the other participant,
  and `localnet::party(alias)` the on-ledger id an application needs for
  `act_as`. `CANTON_ENDPOINT` / `CANTON_TOKEN` override the lot for an
  environment that is not a LocalNet. Runnable as the `localnet` example.
- **`Config` accepts a scheme-less `host:port`**, which is what a gRPC client
  dials and therefore what tooling hands out —
  `CANTON_GRPC_LEDGER_API_URL` is exactly this shape. Previously the missing
  scheme surfaced at the first RPC as an unexplained transport error; the
  scheme now defaults to the one TLS configuration implies.

### Security

- **`canton-lf` (zip bomb):** a DAR is now bounded **in total**, not only per
  entry. Every `.dalf` is read into memory, so an archive of many entries each
  legal on its own — a thousand at 256 MiB — passed every check and still asked
  for hundreds of gigabytes; zeros DEFLATE at roughly 1000:1, so the file
  carrying that request arrives small enough to go unnoticed. Each read is now
  capped by whichever of the per-entry ceiling and the remaining archive budget
  (2 GiB) is smaller, and the error says which limit was reached. The ceiling is
  ~50× the entire available corpus (41 MiB across 18 DARs), so it cannot fire on
  a real DAR. Found by reading canton-devkit's DAR reader, which has had the
  aggregate cap all along.

- **`canton-core`:** the mutual-TLS private key no longer reaches logs.
  `TlsConfig` derived `Debug` and holds `client_identity_pem`, so
  `format!("{config:?}")` — or one `tracing` field capturing a `Config` —
  printed the key byte by byte; `Config` leaked it too, along with any
  credentials in the endpoint URL. Both now have hand-written `Debug` that
  reports presence and length. Five error paths across `canton-core`,
  `canton-auth` and `canton-ledger` that quoted a URL verbatim now run it
  through `canton_core::redact_url`. Two more types held the same secret behind
  a `Debug` and were missed the first time: `JsonClient` derived one and printed
  its `base_url` — userinfo included — and `OidcConfig` redacted its
  `client_secret` field while printing a `token_url` that, for a provider taking
  client credentials as basic auth, *is* the secret. `TokenResponse` no longer
  derives `Debug` at all; it is one bearer token, and nothing should print it.
- **`canton-daml`:** a type mismatch no longer copies the offending value into
  the error message. `mismatch()` formatted it with prost's `Debug`, so
  decoding a payload as the wrong type put the whole record — parties, amounts,
  free text — into a string that travels into the application's traces and
  metrics. It now names the kind and stops.

- **`canton-core` (bearer token in `Debug`):** `AuthInterceptor` derived `Debug`
  over the token it injects into every gRPC request. The interceptor lives
  inside every client that holds a channel, so one `{:?}` on client state — a
  tracing field, a panic message, an error context — printed a live credential,
  and `SECURITY.md`'s "Debug output is redacted" was untrue as written. It now
  reports presence only. Reported privately by Equilibrium during their M1
  review, alongside the mutual-TLS key above.
- **`canton-core` (panic on a hostile retry hint):** `Error::retry_delay()`
  handed a server-supplied number to `Duration::from_secs_f64`, which panics on
  anything a `Duration` cannot hold — so a JSON error body carrying
  `"retryInfo": "1e300 seconds"` aborted the caller *inside error
  classification*, which the retry loop calls on every retriable failure. The
  conversion is fallible now and an out-of-range hint reads as no
  recommendation.

### Fixed — from Equilibrium's independent M1 review

An engineering review of the released 0.1.4 client by [Equilibrium](https://equilibrium.co),
carried out on the Development Fund milestone issue. Every finding is closed
here; two of them (the `Debug` leak and the retry-hint panic) are in
**Security** above.

- **Ambiguous submissions are recoverable.** A submission whose response is
  lost may still have committed, and the change ID is the only way back to the
  outcome — but the SDK generated the command id *inside* the call that failed
  and returned it only on success. `CantonClient::submission` and
  `JsonClient::submission` fix the identity first and hand back a `Submission`
  carrying its `ChangeId`, with `recover` reading the completion back. The
  existing client methods are thin wrappers over the same object.
- **Recovery matches the whole change ID.** `await_completion` compared
  `command_id` alone; Canton identifies a command by (user, acting parties,
  command id), and two applications on one participant may each use `daily-run`.
  It now takes a `&ChangeId`. A user id left to the bearer token is not
  compared (the participant resolved it and the client cannot know it), and a
  completion carrying no acting parties is not rejected on that ground.
- **The resumable update stream honours checkpoints.** `updates_with` dropped
  `OffsetCheckpoint` frames before the resumable wrapper could see them, so the
  resume point only advanced when a transaction arrived — on a quiet stream a
  reconnect went back to where the caller started, which after pruning fails
  outright. The resumable path now reads the unfiltered stream and filters for
  itself. Subscribers see no change.
- **A spent reconnect budget reports the participant's failure**, not
  `UnexpectedResponse("failed to resume after N reconnects")`, which threw away
  the status, the details, the correlation id and the retriable classification
  at the moment they were needed. Same fix in the resumable ACS read.
- **The JSON lane gained the four operations it was missing**: `submit`
  (`/v2/commands/async/submit`), `submit_and_wait`, `events_by_contract_id`,
  and recovery through `JsonClient::submission`. All four exist in Canton
  3.5.7's JSON API; this was the SDK stopping short.
- **`JsonClient::ws_active_contracts_resumable`** resubscribes from the last
  `streamContinuationToken` rather than restarting the snapshot, which the gRPC
  lane has done since M1.
- **The typed ACS read is lossless.** Every gRPC ACS method matched
  `ActiveContract` and dropped the rest, so a reassignment in flight at the
  snapshot offset — `IncompleteUnassigned` / `IncompleteAssigned` — vanished
  from a multi-synchronizer application's view. `AcsEntry` and the `acs_page` /
  `acs_entries` / `acs_entries_resumable` family are the lossless read; the
  active-only methods keep their names and are now that read with
  `into_active` applied.
- **The Auth0 and Okta presets produce their providers' normal requests.**
  Auth0 needs an `audience` (without one it issues a token for its own userinfo
  endpoint, which no participant accepts) — `auth0` now takes it, which is a
  **breaking** signature change. Okta reads the credentials from an
  `Authorization: Basic` header and rejects them in the body as
  `invalid_client`; the preset selects that, and `ClientAuth` exposes the
  choice for custom endpoints.
- **Telemetry covers a stream's life, not its opening.** `instrument_stream`
  counts errors that arrive after a subscription opens — previously a stream
  that failed an hour in had been recorded as a success and never revisited.
  The WebSocket upgrade carries `traceparent` (the only request a WS stream
  makes), structured events carry `trace_id`, and `otel::otlp_metrics` is a
  supported OTLP path for the counters, recorder and all.

- **A retry the participant de-duplicates is a success, not a failure.** This
  is the other half of the finding, and the half that was still open after the
  recovery handle was added. When the SDK retries a submission whose response
  was lost, the participant refuses the second attempt as `DUPLICATE_COMMAND` —
  because the first one was accepted. `submit` was reporting that rejection to
  the caller, which says the command did not happen at the exact moment it
  provably did. It now reports success, over both transports. A duplicate on
  the *first* attempt is untouched: nothing of ours is at the participant, so
  the caller reused a change id and needs to hear about it.

  The waiting variants cannot do this — their result is a transaction, and a
  de-duplicated retry does not carry one — so they surface the rejection and
  their documentation now says to recover through the handle rather than
  describing a caveat and leaving it there.

- **`examples/recover_a_submission.rs`** walks the finding's own scenario end
  to end: submit, submit the same change ID again (what a retry after a lost
  response looks like to the participant), watch it be rejected as
  `DUPLICATE_COMMAND`, and recover the original outcome. Verified live — the
  recovered `update_id` is the one the first submission committed. The README's
  other examples are backed by compiled example files; this one was not, and
  the newest API is the worst one to leave uncompiled.
- **The facade reaches what the documentation promises.** `canton::telemetry`
  was not re-exported, so `cargo add canton` could not see the metric names, the
  transport labels, or — now — the OTLP setup those metrics are meant to be
  exported through. The `otel` feature also reached `canton-core` only by way of
  `canton-ledger`, which was correct by accident. Both fixed, with tests that
  name the paths a reader of the README would try.

**From the same review's non-blocking list:**

- Requests the participant would certainly refuse are refused locally: a
  submission with no commands or no acting party, both minimum-ledger-time
  forms at once, a negative offset, an inverted range, a subscription filtered
  to nobody.
- `read_as` reaches the transaction filter of a submission's response, matching
  the Ledger API's own default; filtering to `act_as` alone returned a
  transaction quietly missing events.
- The idempotent reads — events-by-contract-id, the ACS page, the updates page
  — take the configured retry policy, which had applied only to `version`, the
  health check, `ledger_end` and submissions.
- Jitter never brings a retry back **before** a server-recommended delay; a
  `RetryInfo` is a minimum, and coming back early spends an attempt on a
  guaranteed rejection.
- The WebSocket streams take their reconnect budget and backoff from the
  client's `RetryConfig` instead of a hardcoded five-at-250ms.
- `list_known_parties` fails on a repeated page token instead of returning a
  prefix as if it were the whole list, and a topology response missing a
  required field fails the read rather than shrinking it.
- The vendored `.proto` files carry a provenance record and per-file SHA-256s,
  verified by a test, with `tools/vendor-protos.sh` for the refresh.
- `CANTON_TEST_REQUIRE_LIVE=1` turns a skipped live test into a failure, so a
  live run's result is a claim about a participant rather than about an empty
  environment.
- CI checks `aarch64-unknown-linux-gnu` and `x86_64-unknown-linux-musl`, and
  the `cargo-semver-checks` job is enabled now that a baseline exists.
- ADR-0005 no longer claims mixed installs "fail to resolve"; with caret
  requirements mixed *patch* versions resolve, which is intended.
- `canton-admin` documents that party management here is allocation and
  discovery, and why updates are out of scope for M1.
- The reference app reads its committed transaction back independently and
  matches the exact update id, on both transports.

### Fixed — from an external review of the M1 client

- **`canton-core` (message size):** the gRPC decode limit is raised off tonic's
  4 MiB default to 128 MiB, configurable via
  `Config::with_max_decoding_message_size`. One ACS page arrives as a single
  message and the participant permits page sizes up to 10 000, so the default
  was reachable in ordinary use and surfaced as a client-side `OUT_OF_RANGE`
  that reads like a server fault. Applied at all 24 service-client
  constructions across `canton-ledger` and `canton-admin` through one macro per
  crate, so a newly added RPC cannot pick the default back up. The WebSocket
  lane had the same problem one layer down and no way to fix it:
  `tungstenite`'s defaults cap a message at 64 MiB and a single **frame** at
  16 MiB. `JsonClient::with_max_decoding_message_size` now sets both, and
  defaults them to the same 128 MiB, so one transport cannot quietly be
  stricter than the other. (The HTTP lane needs nothing — `reqwest` puts no
  limit on a response body.)
- **`canton-core` (errors):** `Error::resource_info()` exposes the
  `google.rpc.ResourceInfo` Canton attaches to failures where "which one?" is
  the first question — `CONTRACT_NOT_FOUND` names the contract id. A `Vec`
  rather than an `Option`, because the JSON transport carries a list.
- **`canton-ledger` (unbounded waits):** the JSON lane had no timeout at all.
  `reqwest` applies none unless asked, so a participant that accepted the
  connection and then went quiet held the caller's task open for the life of
  the process — while the gRPC channel beside it had bounded the same call at
  30s since M1. `JsonClient::with_timeout` sets it, defaulting to that same
  30s, applied per request so it holds whatever order the builders were called
  in. The WebSocket **handshake** is bounded by the same value; the stream that
  follows is a live tail and deliberately is not.
- **`canton-core` (transport parity):** `Error::error_info()` and
  `Error::code()` now answer on the JSON transport too. Both returned `None`
  there while returning the real thing over gRPC — so an application that
  classified errors by error id, exactly as `error_info`'s documentation tells
  it to, silently fell back to string-matching the display text the moment it
  was pointed at the JSON lane. The body spells both (`code`/`context` and
  `grpcCodeValue`); a redacted error's literal `"NA"` is reported as no error
  id rather than as one. A live test now asserts that both lanes describe the
  same failure identically, so this cannot drift apart again unnoticed.
- **`canton-daml` (party ids):** `Party::parse` / `"…".parse::<Party>()`
  validate a party id a caller supplies — refusing empty, over-long, and
  characters Canton does not use — while `Party::new` still takes a wire value
  as-is, the same asymmetry `Numeric` already had. `FromStr` was `Infallible`,
  so `"".parse::<Party>()` succeeded and the failure surfaced only at the
  participant, as a `PermissionDenied` naming nothing. An empty party id is the
  shape `std::env::var` returns for `PARTY=`, which is how it happens in
  practice. Idea taken from zenith-network/canton-rs, which validates its
  identifier types.
- **`canton-codegen` (submit vs read):** the two encoders a generated template
  carries are now pinned to agree. The emitter writes the payload's field list
  twice — in `ToValue` and in `Template::to_record` — and they are different
  paths at runtime: `create_command` submits `to_record`, while a contract read
  back arrives through `from_value`. A template whose two lists drifted would
  write one shape to the ledger and expect another, and nothing compared them:
  every round-trip test goes `to_value` → `from_value`, which is the half the
  submit path does not use.
- **`canton-daml` (decode errors in containers):** a failure inside a `List`,
  `TextMap` or `GenMap` now names the element. Generated records attach the
  field name to every decode, but containers dropped everything below it, so
  one bad entry in a list of five hundred holdings reported that the list was
  bad and left the reader to find which. The paths compose: `holders.2`. A list
  reports the index, a `TextMap` the key (which locates an entry better than a
  position), a `GenMap` the position and which half of the entry — its keys are
  arbitrary values, so there is no name to point at.
- **`canton-daml` (dead API removed before it froze):** `record_field` and
  `record_value` are gone from the generated-code surface. The emitter never
  emitted either, nothing in the workspace called them, and the crate is about
  to publish — after which they would have to keep working forever.
- **`canton-codegen` (drift guard, published crates):** a CI job fetches the
  three DARs the published bindings were generated from — cn-quickstart tracks
  them in git — at a pinned commit, verifies their SHA-256, and runs the guards
  against them, asserting each one ran rather than skipped. Nothing before this
  checked on any push that the emitter still produces what is on crates.io.
  (`canton-quickstart-licensing` stays local-only: its DAR is built from source
  rather than committed, so there is nothing to pin.)
- **`canton-daml` (fixture drift):** the test fixture that claims to be written
  "exactly as the generator emits" is now checked against the real emitter
  instead of asserting it in a comment. It had drifted three times — the
  `.at(label)` on each field decode, the serde derives, and the per-field
  renames — and each drift quietly removed a path from coverage while every
  test stayed green.
- **`canton-codegen` (phantom type parameters):** a generated codec bounds only
  the type parameters it actually encodes. Daml permits a phantom parameter —
  declared but used in no field — and the emitter required `T: ToValue` for
  every declared one, so instantiating a phantom parameter with an **interface
  marker** produced Rust that does not compile. Markers carry no codec by
  design: they exist only as the tag of a `ContractId`. Valid Daml therefore
  generated invalid Rust, with the error landing in code the reader did not
  write. Four types across the published bindings carried the spurious bound
  and have been regenerated without it.
- **`canton-codegen` (IR semver):** every public IR struct is
  `#[non_exhaustive]`, with a constructor for each. The IR is documented as
  something a caller lowers and then post-processes, its fields are public, and
  it gained **forty fields during Milestone 2 alone** — so once the crate is on
  crates.io, one more field would be a breaking change for anyone who wrote a
  struct literal, and adding `#[non_exhaustive]` afterwards is itself breaking.
  Fields stay public, so reading and mutating a lowered IR is unchanged; only
  construction goes through `Record::new`, `Template::new` and the rest.
- **`canton-codegen` (a test that had stopped compiling):** the end-to-end test
  that generates a crate, builds it and round-trips both codecs was gated on
  `CODEGEN_COMPILE_TEST`, which nothing set — so it skipped on every push and
  went stale when the runtime made `Numeric` and `GenMap`'s fields private. It
  now builds its values through the public API, the way a consumer must, and CI
  sets the variable and asserts the test ran.
- **`canton-codegen` (manifest injection):** the DAR's package version is
  validated before it reaches the generated `Cargo.toml`. It was interpolated
  raw, so a version of the form `0.1.0"` + newline + `[dependencies.evil]` +
  `git = "…` closed the string and opened a table — an arbitrary git dependency
  in a manifest the caller then compiles, which is code execution from an
  archive. The archive-integrity guards do not stop it: the package id is the
  hash of whatever payload its author chose, so an authored DAR passes them
  all. The crate name beside it was already validated and the runtime path
  beside it already escaped; the version was the third field in the same
  manifest and the one that was missed.
- **`canton-codegen` (drift guard):** the guard that keeps the committed
  bindings honest now runs in CI. The four existing ones need a DAR from a
  Splice or cn-quickstart checkout, so on a machine without one they skip — and
  a skipped test reads exactly like a passing one, which meant the property had
  never been enforced anywhere but a developer's laptop, for three crates that
  are published. A fifth guard regenerates from the DAR committed to this
  repository and AST-compares against a committed fixture, and the CI job
  asserts it ran rather than skipped, the way the conformance-oracle job
  already did.
- **`canton-daml` (typed read on JSON):** `Template::from_json_created_event`
  is the JSON counterpart of `from_created_event`, which was gRPC-only. Both
  transports carry the same contract for the same bindings, so an application
  may write over one and read over the other — but on the JSON lane a caller
  had to reach into `event["CreatedEvent"]["createArgument"]` themselves, and
  nothing then checked that the event was that template at all. A party's
  stream carries every template it sees, and where two payloads share a field
  shape, decoding one as the other succeeds and is wrong. Accepts the event
  wrapped (`CreatedEvent` / `createdEvent`) or bare, and compares module and
  entity but not the package id, matching the gRPC path under Smart Contract
  Upgrade.
- **`canton-codegen` (shared packages):** a package already published as its own
  crate can be **referenced** instead of re-generated —
  `Options::with_external_package(name_or_id, crate)`, or `lower_dar_with` for
  the library path. A DAR's dependency closure is shared, so
  `splice-api-token-holding-v1` sits under amulet, wallet and wallet-payments
  alike; generating it into each gave each crate its own `Holding`, and Rust
  treats those as unrelated types. A program depending on two of the published
  binding crates failed to compile with "expected `Holding`, found a different
  `Holding`" — for the same interface, in the same package. Packages are keyed
  by **name** as well as id, and the name is the one to prefer: an id is the
  hash of one build, while the name survives a version bump, which is the point
  of addressing packages by name under Smart Contract Upgrade.
- **`canton-codegen` (hostile DAR):** a type that resolves to itself is
  refused instead of overflowing the stack. Interned types are a flat table of
  indices, so `interned_types[0] = Interned(0)` — or two entries pointing at
  each other — is not malformed to prost, which bounds nested *messages* and
  has no view of the table. Following it recursed until the stack ended, and a
  stack overflow aborts the process rather than failing the DAR: the one input
  in this reader that still killed the caller after the zip-bomb, entry-size,
  archive-size, package-id and LF-version guards. Bounded at 256 levels, where
  the deepest type in the 648-package corpus resolves in 15.
- **`canton-lf` (archive integrity):** a package's payload is hashed and checked
  against the id it declares before it is parsed, and a `hash_function` other
  than SHA-256 is refused rather than assumed. That id is embedded in generated
  bindings as `PACKAGE_ID` and is what cross-package references resolve
  through, so a DAR whose id does not match its bytes previously produced
  bindings naming a package its contents do not answer to.
- **`canton-lf` (LF version):** an archive declaring a Daml-LF 2.x minor this
  build was not compiled against is refused instead of decoded. prost drops
  fields from a schema it does not know, so a newer minor yielded a package
  quietly missing template fields. Accepts `2.1` and `2.2`, which is what the
  available corpus contains (648 packages across 18 DARs). The **major**
  version is decided first: an LF 1 archive — every DAR a Daml 2.x SDK built —
  was being reported as an unsupported *LF 2 minor*, telling the reader to
  upgrade an SDK that will never read it.
- **`canton-codegen` (decode errors):** generated `FromValue` bodies attach the
  field name to a failure, so a mismatch inside a nested record reports
  `meta.values` instead of a bare "expected Text". `ValueError::at` existed and
  was documented as being used by generated code; it was not.

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
