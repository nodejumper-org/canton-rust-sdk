# Canton Rust SDK

A production-grade, async **Rust SDK for the [Canton Network](https://www.canton.network/)** Ledger API — the Rust member of Canton's language-binding set, funded by Canton dev-fund proposal [#407](https://github.com/canton-foundation/canton-dev-fund/pull/407). Apache-2.0.

Built on `tonic`/`prost`/`tokio`. Talks the **Ledger API v2** over gRPC (primary) and JSON (HTTP + WebSocket), with correct change-ID de-duplication, command recovery, resilient/resumable streaming, TLS/mTLS on every transport, JWT/OIDC auth, and built-in telemetry.

> **Status:** the Ledger API client is **released** on crates.io (0.1.x); the type-safe DAR codegen is **code-complete and not yet published**. Everything here is verified against a Canton **3.5.7** participant: hermetic tests plus a live suite (submit, streaming, recovery, TLS/mTLS, auth), and an end-to-end typed loop — generate bindings from a DAR, submit a typed create, read it back, exercise a choice — over gRPC and JSON. CI holds the whole workspace to `-D warnings` on every feature combination. Token-standard support (CIP-56 / CIP-0112), a PQS client, and external signing are next.

## Crates

| Crate | What it is |
|---|---|
| `canton` | The SDK entry point: a thin facade re-exporting the whole family (`canton::ledger`, `canton::auth`, `canton::admin`, `canton::daml` + the shared `Config`/`Error` at the root) with the `ws`/`otel` features forwarded. `cargo add canton` gets everything below as one version-locked set. |
| `canton-core` | Shared foundation: the `Error`/`Result` model (retriable classification, structured `ErrorInfo` details), the connection kernel (`Config`, `Auth`/`TokenSource`, `TlsConfig`, jittered retry with per-attempt timeouts), and telemetry (`tracing` spans + `metrics`, optional OTLP via `otel`). |
| `canton-proto` | Generated gRPC types + client stubs from vendored protos (Ledger API v2, Canton admin API topology read, gRPC health), pinned to a Canton release. Internal. |
| `canton-auth` | JWT/OIDC authentication: client-credentials `TokenProvider` with caching + refresh + bounded fetch, and Keycloak/Auth0/Okta presets. |
| `canton-ledger` | The async Ledger API client. gRPC: `submit` / `submitAndWait` / `submitAndWaitForTransaction`, completions + recovery, ACS/update streaming (+ paging, reverse-order, event query, offset-resumable), request builders (bounded/filtered/shaped streams, completion `user_id`), node health. JSON: command submission, bounded reads, and WebSocket streaming (incl. resumable) behind the `ws` feature. |
| `canton-admin` | Admin surface: party allocation/management, user self-inspect, packages read, and topology read (party→participant mappings, namespace delegations, vetted packages) over the Canton admin API. |
| `canton-daml` | The runtime under generated bindings: Daml primitive types (`Party`, `ContractId<T>`, `Numeric`, `Timestamp`, …), `Template`/`Interface`/`Choice` traits, command builders, and the JSON + gRPC value codecs. |
| `canton-codegen` / `canton-codegen-cli` | DAR → typed Rust. The CLI (`dpm-codegen-rust`, also `dpm codegen-rust`) writes a complete crate from any DAR; the library is the IR + emitter behind it. |
| `canton-lf` | Daml-LF archive reader/decoder (the codegen front-end), built on the official `daml-lf-archive` schema and held to the official JVM reader by a conformance oracle. Internal. |
| `canton-splice-amulet`, `canton-splice-wallet`, `canton-splice-wallet-payments` | Pre-built typed bindings for the Splice protocol DARs, regenerated per release ("DAR as a crate"). |
| `canton-splice-api-token-*`, `canton-splice-api-featured-app-v1` | The token-standard and featured-app packages, one crate each. The crates above reference these rather than copying them, so a `Holding` is the same Rust type whichever crate you reach it through. |
| `canton-quickstart-licensing` | The same, for the cn-quickstart licensing DAR. **Not published** — it backs the reference app and the end-to-end tests; generate your own with the CLI. |

## Compatibility

| SDK version | Canton version | Ledger API | Rust (MSRV) |
|---|---|---|---|
| 0.1.4 (released) | 3.5.7 (pinned protos) | v2 | 1.88 |
| 0.2.x (this branch, unreleased) | 3.5.7 (pinned protos) | v2 | 1.88 |

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

The `canton` facade forwards both: `canton = { version = "0.3", features = ["ws", "otel"] }`.

Telemetry follows the standard Rust model: the SDK **emits** (`tracing` spans, `metrics` counters labelled by method + transport); the application installs the subscriber/recorder of its choice.

## Quickstart

```sh
cargo add canton            # the whole SDK, one crate
# or pick pieces: cargo add canton-ledger canton-auth
```

```rust,ignore
use canton::ledger::{CantonClient, Config};

#[tokio::main]
async fn main() -> canton::Result<()> {
    let client = CantonClient::connect_lazy(Config::new("http://localhost:3901"))?;
    println!("ledger api version: {}", client.version().await?);
    println!("node health:        {:?}", client.health_check().await?);
    Ok(())
}
```

With OIDC auth and a command:

```rust,ignore
use canton::auth::{OidcConfig, TokenProvider};
use canton::ledger::{CantonClient, Config, Submit, create, identifier, record, value};

async fn submit(party: &str, pkg: &str) -> canton::Result<()> {
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
    Ok(())
}
```

Runnable examples: [`version_and_health`](crates/canton-ledger/examples/version_and_health.rs) (no auth, defaults to `http://localhost:3901`) and [`submit_and_read`](crates/canton-ledger/examples/submit_and_read.rs) (OIDC auth + a create). Both read the same `CANTON_TEST_*` variables as the live tests below, so one export set runs everything:

```sh
cargo run -p canton-ledger --example version_and_health
cargo run -p canton-ledger --example submit_and_read
```

See also the integration tests in [`crates/canton-ledger/tests/`](crates/canton-ledger/tests/) and [`crates/canton-admin/tests/`](crates/canton-admin/tests/).

## A local network, with no configuration in your program

[canton-devkit](https://github.com/bitdynamics-ab/canton-devkit) runs a Splice
LocalNet — two participants and a super-validator — and exports it into the
environment. The SDK reads that export directly, so nothing in the program names
a host, a port, or a credential:

```sh
canton-devkit localnet up demo            # or: dpm localnet up demo
eval "$(canton-devkit localnet env demo)"
```

```rust,ignore
use canton::ledger::{CantonClient, JsonClient};
use canton::{Config, localnet};

let grpc = CantonClient::connect_lazy(Config::from_env()?)?;   // app-provider
let json = JsonClient::from_env()?;                            // same network
let user = Config::from_env_for("app-user")?;                  // the other participant
let party = localnet::party("app-provider");                   // the id for `act_as`
```

Runnable: [`localnet`](crates/canton-ledger/examples/localnet.rs).

```sh
cargo run -p canton-ledger --example localnet
```

Two details this handles for you, both of which otherwise fail late and
unhelpfully. The exported gRPC URL has **no scheme** (`host:port` is what a gRPC
client dials) — passed to a client unchanged it produced an unexplained
transport error at the first RPC. And the URLs are nginx **virtual-host names**
(`grpc-ledger-api.app-provider.demo.localhost`), so the name has to reach the
`:authority` / `Host` header rather than be resolved away; substituting
`127.0.0.1` reaches the port and is refused by the vhost. `*.localhost` resolves
to loopback on macOS and on Linux with systemd-resolved — elsewhere, add an
`/etc/hosts` entry.

Nothing here is devkit-specific beyond the variable names, and `CANTON_ENDPOINT`
/ `CANTON_TOKEN` override them for an environment that is not a LocalNet. The
full contract is in [`canton_core::localnet`](crates/canton-core/src/localnet.rs).

Verified against a live Canton 3.5.7 participant reached through those exported
shapes — scheme-less gRPC URL, vhost hostnames, ready-made bearer token — on
both transports. What that does *not* yet cover is a `localnet up` of our own:
the contract is read and exercised, the orchestration around it is the devkit
project's to vouch for.

## Typed bindings from your DAR (codegen)

Turn any DAR into a typed crate — templates become structs, choices become
typed exercise impls, with JSON and gRPC codecs on everything:

```sh
cargo install canton-codegen-cli          # provides `dpm-codegen-rust`
dpm-codegen-rust --dar path/to/my-app-0.1.0.dar --out my-app-bindings
```

The output is a self-contained crate (`Cargo.toml` + `src/lib.rs`). **Commit it**
and depend on it by path — that is how the `canton-splice-*` crates in this
repository are built, and it keeps the generated code reviewable in a diff:

```toml
my-app-bindings = { path = "my-app-bindings" }
```

To keep it in step with the DAR, regenerate in CI and fail on a diff, rather
than generating during the build:

```sh
dpm-codegen-rust --dar dars/my-app-0.1.0.dar --out my-app-bindings
git diff --exit-code my-app-bindings
```

No `--force` is needed to regenerate over the tool's own output; it is there to
overwrite files this tool did **not** write, which is a thing to do on purpose
and not a flag to carry around.

Generating from a **build script** does not work, and it is worth saying why
rather than leaving it to be discovered: Cargo resolves path dependencies before
it runs build scripts, so on a clean checkout `my-app-bindings` does not exist
yet and the build fails before the script that would create it has run. The
`prost-build` arrangement — write into `OUT_DIR`, `include!` it — does not apply
either: the emitted tree spans several packages and refers between them by
`crate::`-qualified paths, which resolve to the *including* crate's root rather
than the module they were placed in.

`canton_codegen::generate` is a library call for exactly the CI step above, and
for tooling that produces a crate directory.

Then submit typed commands:

```rust,ignore
use my_app_bindings::my_app::My_Module::{Asset, Asset_Transfer};
use canton_daml as rt;
use rt::Template as _;

let payload = Asset { owner: rt::Party::new(party), name: "gem".into() };
let create = rt::create_command(&payload);                       // gRPC command
let created: Asset = Asset::from_created_event(&event)?;         // typed read
let exercise = rt::exercise_command(&contract_id, &Asset_Transfer {
    new_owner: rt::Party::new(other),
});
```

Template ids use the upgrade-friendly `#package-name` form, so the participant
resolves the version vetted under Smart Contract Upgrade (the pinned package id
is also available as `Asset::PACKAGE_ID`). For the Splice DARs, skip codegen and
use the pre-built `canton-splice-*` crates. The full Daml-LF → Rust type mapping
is documented in [docs/daml-lf-type-mapping.md](docs/daml-lf-type-mapping.md);
regeneration on a DAR version bump in
[docs/scu-regeneration.md](docs/scu-regeneration.md). A complete runnable flow
(typed create → read back → exercise, on gRPC and JSON) is
[`crates/canton-sample`](crates/canton-sample/src/main.rs).

## Testing

**No-node tests** — unit tests, in-process gRPC/WebSocket mock servers, TLS
handshakes, wire-shape assertions. Nothing to install or configure:

```sh
cargo test --workspace --all-features
```

**Live integration tests** run against a real participant when the variables
below are set, and skip otherwise (so the command above stays green without a
node). Every name is prefixed `CANTON_TEST_`:

| Variable | What it gates | Example (LocalNet App Provider) |
|---|---|---|
| `CANTON_TEST_ENDPOINT` | all gRPC live tests | `http://localhost:3901` |
| `CANTON_TEST_JSON_ENDPOINT` | the JSON-transport and WebSocket tests | `http://localhost:3975` |
| `CANTON_TEST_TOKEN_URL` | authenticated tests (OIDC client-credentials) | `http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token` |
| `CANTON_TEST_CLIENT_ID`, `CANTON_TEST_CLIENT_SECRET` | ditto | `app-provider-backend`, … |
| `CANTON_TEST_PARTY` | command submission and read-back | `app_provider_quickstart-…::1220…` |
| `CANTON_TEST_LICENSING_PKG` | ditto — the package the test commands instantiate | `#quickstart-licensing` |
| `CANTON_TEST_ADMIN_ENDPOINT` | `canton-admin` topology reads | `http://localhost:3902` |
| `CANTON_TEST_ADMIN_CLIENT_ID`, `CANTON_TEST_ADMIN_CLIENT_SECRET` | party-admin RPCs (need the `ParticipantAdmin` right) | `app-provider-validator`, … |
| `CANTON_TEST_SYNC_ID` | optional: also assert vetted packages in the synchronizer store | |

```sh
export CANTON_TEST_ENDPOINT=http://localhost:3901
export CANTON_TEST_JSON_ENDPOINT=http://localhost:3975
export CANTON_TEST_TOKEN_URL=http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token
export CANTON_TEST_CLIENT_ID=app-provider-backend CANTON_TEST_CLIENT_SECRET=…
export CANTON_TEST_PARTY='app_provider_quickstart-…::1220…'
export CANTON_TEST_LICENSING_PKG='#quickstart-licensing'
cargo test -p canton-ledger --all-features --test live -- --nocapture
```

**Credentials.** The suite takes whichever the environment offers: the OIDC
client-credentials flow where there is an issuer (`CANTON_TEST_TOKEN_URL` and
friends), otherwise a ready-made bearer token — which is what a Splice LocalNet
exports, having no issuer to exchange credentials with. Two tests genuinely need
an issuer (`ledger_end_with_oidc_auth`, and party management, which needs a
token carrying `ParticipantAdmin`); those say so when they step aside. The rest
run either way.

**Bringing up a node.** Any Canton 3.5 participant works; four paths, least
setup first:

- [canton-devkit](https://github.com/bitdynamics-ab/canton-devkit) — one binary:
  `canton-devkit localnet up demo`, then `eval "$(canton-devkit localnet env
  demo)"` exports endpoints, tokens and party ids under the names
  [`canton_core::localnet`](crates/canton-core/src/localnet.rs) reads, so the
  suite needs no `CANTON_TEST_*` at all. It allocates its own ports, so pass
  `--port-base` or read the exported values rather than assuming `3901`.
  Authentication is Splice's `unsafe-jwt-hmac-256`, so the two issuer-dependent
  tests skip; `localnet dar upload` supplies a package for the rest.
- [Canton Builder Tool](https://canton-network-devs.github.io/Canton-Builder-Tool/#part-builder)
  — the least to install: `canton builder start` brings up a LocalNet (its guide
  says about five minutes the first time, faster after), and
  `canton builder status` prints the port reference. Its App Provider
  participant is on `3901`/`3902`/`3975` like the others, and
  `canton builder deploy <dar>` uploads a DAR to both participants, which is how
  you get `CANTON_TEST_LICENSING_PKG` without cn-quickstart.
- [Splice LocalNet](https://docs.sync.global/app_dev/testing/localnet.html) —
  plain Docker Compose. Its App Provider participant is on the ports used above
  (`3901` gRPC, `3902` admin, `3975` JSON), so `CANTON_TEST_ENDPOINT` and
  `CANTON_TEST_JSON_ENDPOINT` need no changes. It runs **unauthenticated** by
  default — its only other profile is `unsafe-jwt-hmac-256`, an HMAC secret you
  sign tokens with yourself — so there is no OIDC token endpoint. Set the token
  you signed as `CANTON_TOKEN` and everything except the two issuer-dependent
  tests runs; the command-submission ones additionally want a package, which is
  what `cn-quickstart` supplies.
- [`cn-quickstart`](https://github.com/digital-asset/cn-quickstart)
  (`make setup && make build && make start`) — the same LocalNet plus the
  licensing sample app, which is where `CANTON_TEST_LICENSING_PKG` /
  `#quickstart-licensing` comes from; needed for the tests that submit commands.

More LocalNet tooling is catalogued on the
[Canton Dev Hub](https://dev-hub.canton.foundation/).

CI enforces `rustfmt`, `clippy -D warnings` (all features), the full test suite on Linux/macOS/Windows, rustdoc `-D warnings`, `cargo-deny`, and the MSRV build.

## MSRV

Rust **1.88** (bounded by `tonic` 0.14). Policy: the MSRV tracks what our
pinned major dependencies require; a bump is a minor (not breaking) change,
announced in the [CHANGELOG](CHANGELOG.md), and CI always builds the declared
MSRV.

## Roadmap

**Shipped:** the async Ledger API client (gRPC + JSON + WebSocket, auth, TLS,
retry, telemetry) and type-safe code generation from DAR packages — SCU-aware,
with a `dpm codegen-rust` component and prebuilt `canton-splice-*` crates. The
LF decoder is native Rust rather than a JVM wrapper around `daml-lf-archive`
([ADR-0008](docs/adr/0008-native-lf-decoder.md)); its output is held to the
official JVM reader by a conformance oracle.

**Next:** token-standard support (CIP-56 V1 + CIP-0112 V2), interactive
submission with a pluggable signer, a typed PQS client, and the
Ledger-Client-Standard conformance suite.

## Contributing & security

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and
[SECURITY.md](SECURITY.md) for private vulnerability reporting. Notable
changes are tracked in [CHANGELOG.md](CHANGELOG.md).

## Acknowledgements

Built on the Ledger API and Daml-LF work of the
[Canton](https://github.com/digital-asset/canton) and
[Splice](https://github.com/canton-network/splice) teams.

The local-development path reads the environment
[canton-devkit](https://github.com/bitdynamics-ab/canton-devkit) exports, and
reading its DAR container taught us that a per-entry decompression cap bounds
nothing on its own — an archive is now bounded in total as well.

## License

Apache-2.0. See [LICENSE](LICENSE).
