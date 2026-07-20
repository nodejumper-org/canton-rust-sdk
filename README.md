# Canton Rust SDK

A production-grade, async **Rust SDK for the [Canton Network](https://www.canton.network/)** Ledger API — the Rust member of Canton's language-binding set, funded by Canton dev-fund proposal [#407](https://github.com/canton-foundation/canton-dev-fund/pull/407). Apache-2.0.

Built on `tonic`/`prost`/`tokio`. Talks the **Ledger API v2** over gRPC (primary) and JSON (HTTP + WebSocket), with correct change-ID de-duplication, command recovery, resilient/resumable streaming, TLS/mTLS on every transport, JWT/OIDC auth, and built-in telemetry.

> **Status: Milestone 1 code-complete.** Every M1 deliverable is implemented and verified — 112 no-node tests (unit, in-process gRPC/WS mock servers, TLS) plus a full live suite against a Canton **3.5.7** LocalNet participant, all green under `-D warnings` on every feature combination. The remaining step before the M1 submission is the first crates.io release.

## Crates

| Crate | What it is |
|---|---|
| `canton` | The SDK entry point: a thin facade re-exporting the whole family (`canton::ledger`, `canton::auth`, `canton::admin` + the shared `Config`/`Error` at the root) with the `ws`/`otel` features forwarded. `cargo add canton` gets everything below as one version-locked set. |
| `canton-core` | Shared foundation: the `Error`/`Result` model (retriable classification, structured `ErrorInfo` details), the connection kernel (`Config`, `Auth`/`TokenSource`, `TlsConfig`, jittered retry with per-attempt timeouts), and telemetry (`tracing` spans + `metrics`, optional OTLP via `otel`). |
| `canton-proto` | Generated gRPC types + client stubs from vendored protos (Ledger API v2, Canton admin API topology read, gRPC health), pinned to a Canton release. Internal. |
| `canton-auth` | JWT/OIDC authentication: client-credentials `TokenProvider` with caching + refresh + bounded fetch, and Keycloak/Auth0/Okta presets. |
| `canton-ledger` | The async Ledger API client. gRPC: `submit` / `submitAndWait` / `submitAndWaitForTransaction`, completions + recovery, ACS/update streaming (+ paging, reverse-order, event query, offset-resumable), node health. JSON: command submission, bounded reads, and WebSocket streaming (incl. resumable) behind the `ws` feature. |
| `canton-admin` | Admin surface: party allocation/management, user self-inspect, packages read, and topology read (party→participant mappings, namespace delegations, vetted packages) over the Canton admin API. |

## Compatibility

| SDK version | Canton version | Ledger API | Rust (MSRV) |
|---|---|---|---|
| 0.1.x (unreleased) | 3.5.7 (pinned protos) | v2 | 1.88 |

The vendored `.proto` files are pinned to the Canton release above; moving the
supported Canton range re-vendors them in a new SDK minor (see the stability
policy in [`canton-proto`](crates/canton-proto/src/lib.rs) and
[ADR-0002](docs/adr/0002-proto-types-are-protocol-stable.md)). All `canton-*`
crates release in **lockstep** — mix only equal versions
([ADR-0005](docs/adr/0005-lockstep-versioning.md)).

## Feature flags

| Feature | Crate | What it adds |
|---|---|---|
| `ws` | `canton-ledger` | WebSocket streaming for the JSON transport (`ws_updates`, `ws_active_contracts`, `ws_completions`, `ws_updates_resumable`), TLS-aware. |
| `otel` | `canton-core`, `canton-ledger` | OTLP span export (`telemetry::otel::otlp_tracer`) and automatic W3C trace-context injection into outgoing gRPC metadata + JSON headers. |

The `canton` facade forwards both: `canton = { version = "0.1", features = ["ws", "otel"] }`.

Telemetry follows the standard Rust model: the SDK **emits** (`tracing` spans, `metrics` counters labelled by method + transport); the application installs the subscriber/recorder of its choice.

## Quickstart

```sh
cargo add canton            # the whole SDK, one crate
# or pick pieces: cargo add canton-ledger canton-auth
```

```rust
use canton::ledger::{CantonClient, Config};

# async fn run() -> canton::Result<()> {
let client = CantonClient::connect_lazy(Config::new("http://localhost:3901"))?;
println!("ledger api version: {}", client.version().await?);
println!("node health:        {:?}", client.health_check().await?);
# Ok(())
# }
```

With OIDC auth and a command:

```rust
use canton::auth::{OidcConfig, TokenProvider};
use canton::ledger::{CantonClient, Config, Submit, create, identifier, record, value};

# async fn run(party: &str, pkg: &str) -> canton::Result<()> {
let auth = TokenProvider::new(OidcConfig::keycloak(
    "http://keycloak.localhost:8082", "AppProvider", "client-id", "client-secret",
));
let client = CantonClient::connect_lazy(
    Config::new("http://localhost:3901").with_oidc(auth),
)?;

let tx = client
    .submit_and_wait_for_transaction(
        Submit::new(party).add_command(create(
            identifier(pkg, "Licensing.AppInstall", "AppInstallRequest"),
            record(vec![
                ("provider", value::party(party)),
                ("user", value::party(party)),
                ("meta", value::record(record(vec![("values", value::empty_text_map())]))),
            ]),
        )),
    )
    .await?;
println!("committed {} at offset {}", tx.update_id, tx.offset);
# Ok(())
# }
```

Runnable examples: [`version_and_health`](crates/canton-ledger/examples/version_and_health.rs) (no auth) and [`submit_and_read`](crates/canton-ledger/examples/submit_and_read.rs) (OIDC auth + a create). Run with `cargo run -p canton-ledger --example version_and_health`. See also the integration tests in [`crates/canton-ledger/tests/`](crates/canton-ledger/tests/) and [`crates/canton-admin/tests/`](crates/canton-admin/tests/).

## Testing

- **Unit / in-process / TLS / WS tests** run with no external services: `cargo test --workspace --all-features`.
- **Live integration tests** run against a participant node when `CANTON_TEST_ENDPOINT` (and, for authenticated tests, `CANTON_TEST_TOKEN_URL` / `CLIENT_ID` / `CLIENT_SECRET` / `PARTY` / `LICENSING_PKG`; for admin, `CANTON_TEST_ADMIN_ENDPOINT` / `ADMIN_CLIENT_ID` / `ADMIN_CLIENT_SECRET`) are set; otherwise they skip. Bring up a node with [`cn-quickstart`](https://github.com/digital-asset/cn-quickstart) LocalNet.

CI enforces `rustfmt`, `clippy -D warnings` (all features), the full test suite on Linux/macOS/Windows, rustdoc `-D warnings`, `cargo-deny`, and the MSRV build.

## MSRV

Rust **1.88** (bounded by `tonic` 0.14). Policy: the MSRV tracks what our
pinned major dependencies require; a bump is a minor (not breaking) change,
announced in the [CHANGELOG](CHANGELOG.md), and CI always builds the declared
MSRV.

## Roadmap

M1 (this milestone) is the core client. Coming next per the proposal: **M2** — type-safe code generation from DAR packages (`daml-lf-archive`-based, SCU-aware) with a `dpm codegen-rust` component and the first prebuilt `canton-splice-*` crates; **M3** — token-standard support (CIP-56 V1 + CIP-0112 V2), interactive submission with a pluggable signer, a typed PQS client, and the Ledger-Client-Standard conformance suite.

## Contributing & security

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and
[SECURITY.md](SECURITY.md) for private vulnerability reporting. Notable
changes are tracked in [CHANGELOG.md](CHANGELOG.md).

## License

Apache-2.0. See [LICENSE](LICENSE).
