# Local development and testing

This page covers two things: pointing your own program at a Canton network on
your machine, and running the SDK's own test suites, with or without one.

## Choosing a local network

Any participant the [compatibility matrix](reference/compatibility-matrix.md)
covers will do. These are the usual ways to run one locally, least setup first:

| Tool | Start it | App Provider participant | Credentials |
|---|---|---|---|
| [canton-devkit](https://github.com/bitdynamics-ab/canton-devkit) | `canton-devkit localnet up demo` (or `dpm localnet up demo`) | its own ports; read them from the export, or choose them with `--port-base` | Splice's `unsafe-jwt-hmac-256`; tokens are exported ready-made |
| [Canton Builder Tool](https://canton-network-devs.github.io/Canton-Builder-Tool/#part-builder) | `canton builder start`; `canton builder status` prints the ports | `3901` gRPC, `3902` admin, `3975` JSON | `canton builder token --validator app-provider` prints a bearer token whose user carries `ParticipantAdmin` |
| [Splice LocalNet](https://docs.sync.global/app_dev/testing/localnet.html) | Docker Compose | `3901` gRPC, `3902` admin, `3975` JSON | none by default; the other profile, `unsafe-jwt-hmac-256`, has you sign tokens yourself |
| [cn-quickstart](https://github.com/digital-asset/cn-quickstart) | `make setup && make build && make start` | `3901` gRPC, `3902` admin, `3975` JSON | Keycloak: OIDC client credentials |

canton-devkit exports the network into your shell under the names the SDK
reads; the next section relies on that. cn-quickstart ships the licensing app
the SDK's command-submission tests use, and you can upload its DAR to the others
([Packages for the live suites](#packages-for-the-live-suites)). More tooling is
listed on the [Canton Dev Hub](https://dev-hub.canton.foundation/).

## A LocalNet with no configuration in your program

canton-devkit runs a Splice LocalNet (two participants and a super-validator)
and prints its endpoints, tokens and party ids as shell variables. The SDK reads
those variables directly, so nothing in your program names a host, a port or a
credential:

```sh
canton-devkit localnet up demo            # or: dpm localnet up demo
eval "$(canton-devkit localnet env demo)"
```

```rust,ignore
use canton::ledger::{CantonClient, JsonClient};
use canton::{Config, localnet};

let grpc = CantonClient::connect_lazy(Config::from_env()?)?; // app-provider, with its token
let json = JsonClient::from_env()?;                          // the same participant over JSON
let user = Config::from_env_for("app-user")?;                // the other participant
let party = localnet::party("app-provider");                 // Option<String>, for act_as
```

The runnable version is the
[`localnet` example](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-ledger/examples/localnet.rs):

```sh
cargo run -p canton-ledger --example localnet
```

```rust,ignore
{{#include ../../crates/canton-ledger/examples/localnet.rs:20:}}
```

### The variables

[`canton_core::localnet`](https://docs.rs/canton-core/latest/canton_core/localnet/index.html)
is the one place in the SDK that knows these names. The `canton` facade
re-exports it as `canton::localnet`.

| Variable | Meaning | Read by |
|---|---|---|
| `CANTON_GRPC_LEDGER_API_URL` | gRPC Ledger API, scheme-less `host:port` | [`Config::from_env`](https://docs.rs/canton-core/latest/canton_core/config/struct.Config.html#method.from_env), `localnet::grpc_endpoint(None)` |
| `CANTON_JSON_LEDGER_API_URL` | JSON Ledger API, with an `http(s)://` scheme | [`JsonClient::from_env`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.JsonClient.html#method.from_env), `localnet::json_endpoint(None)` |
| `CANTON_<ROLE>_JWT` | that role's bearer token | both of the above, `localnet::token` |
| `CANTON_<ALIAS>_PARTY` | an on-ledger party id | [`localnet::party`](https://docs.rs/canton-core/latest/canton_core/localnet/fn.party.html) |
| `CANTON_INSTANCE`, `CANTON_SPLICE_VERSION` | which network this is | `localnet::instance`, `localnet::splice_version` |
| `CANTON_ENDPOINT` | overrides the gRPC URL (not the JSON one) | `Config::from_env`, `localnet::grpc_endpoint` |
| `CANTON_TOKEN` | overrides the token, on both transports | `Config::from_env`, `JsonClient::from_env`, `localnet::token` |

The unqualified URL variables point at the **app-provider** participant, and
the token attached by default is `CANTON_APP_PROVIDER_JWT`. Other participants
are reached by role:
[`Config::from_env_for`](https://docs.rs/canton-core/latest/canton_core/config/struct.Config.html#method.from_env_for)
and
[`JsonClient::from_env_for`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.JsonClient.html#method.from_env_for)
read `CANTON_APP_USER_GRPC_LEDGER_API_URL`, `CANTON_SV_JWT` and so on. The role
is normalised the way the exporter does it (upper-cased, `-` becomes `_`), so
`"app-user"` and `"app_user"` name the same variables. A role with no JSON URL
exported is an error, not a fall-back to the default participant: talking to the
wrong node silently is worse than not connecting. `localnet::party` returns the
party **id** that goes in `act_as`, not the Ledger API user name; its alias is a
role (`"app-provider"`) or a name given to a party later (`"bob"`).

### Things this handles for you

Each of these otherwise fails late, with an error that hides the cause.

- **The gRPC URL has no scheme.** `host:port` is what a gRPC client dials, so
  that is what the exporter prints. `Config` keeps it as given and the channel
  builder adds `http://`, or `https://` when TLS is configured. Passed unchanged
  to a client that expects a scheme, it surfaces as a transport error at the
  first RPC.
- **The hosts are nginx virtual-host names**, such as
  `grpc-ledger-api.app-provider.demo.localhost`. The name routes the request, so
  it has to reach the `:authority` (gRPC) or `Host` (HTTP) header. Do not
  substitute `127.0.0.1`: that reaches the port and is refused by the virtual
  host. `*.localhost` resolves to loopback on macOS and on Linux with
  systemd-resolved; elsewhere, add an `/etc/hosts` entry.
- **A set-but-empty variable counts as unset.** An empty `CANTON_ENDPOINT` or
  `CANTON_TOKEN` left over from an earlier shell does not shadow the real
  LocalNet values.
- **A missing token is not an error.** A LocalNet started without
  authentication exports no JWT, and `from_env` then returns a configuration
  with no credentials.
- **A missing endpoint is.** `from_env` returns
  [`Error::InvalidRequest`](https://docs.rs/canton-core/latest/canton_core/enum.Error.html#variant.InvalidRequest)
  naming the variable it looked for and the `canton-devkit localnet env` command
  that sets it.

The exported tokens are development credentials from the LocalNet's own issuer:
long-lived and never revoked. Do not carry them into a deployment.

## Other local networks

On a network that does not export these variables, either configure the client
explicitly:

```rust,ignore
use canton::auth::{OidcConfig, TokenProvider};
use canton::ledger::CantonClient;
use canton::Config;

// Unauthenticated: Config::new("http://localhost:3901") is all there is to it.
// cn-quickstart: client credentials against its Keycloak.
let oidc = OidcConfig::new(
    "http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token",
    "app-provider-backend",
    client_secret,
);
let client = CantonClient::connect_lazy(
    Config::new("http://localhost:3901").with_oidc(TokenProvider::new(oidc)),
)?;
```

or set the names `from_env` reads yourself, so the same program runs on every
network. `from_env` takes a ready-made token only; for an OIDC issuer, use
`with_oidc` as above.

```sh
export CANTON_ENDPOINT=http://localhost:3901
export CANTON_JSON_LEDGER_API_URL=http://localhost:3975
export CANTON_TOKEN="$(canton builder token --validator app-provider)"   # only where a token is wanted
```

## Running the SDK's tests

### No-node tests

Unit tests, in-process gRPC and WebSocket mock servers, TLS handshakes and
wire-shape assertions need nothing installed or configured:

```sh
cargo test --workspace --all-features
cargo test --workspace
```

CI runs both lines, on Linux, macOS and Windows. The second one catches a test
that names a feature-gated item without being gated itself, which compiles under
`--all-features` and breaks a plain `cargo test`. The checks to run before a pull
request are listed in [Contributing](project/contributing.md).

### Live suites

Five test targets talk to real services. Each test skips when the variables it
needs are unset, which is why the commands above stay green on a machine with no
node.

| Suite | Command | Needs |
|---|---|---|
| Ledger API | `cargo test -p canton-ledger --all-features --test live` | a participant |
| Interactive submission | `cargo test -p canton-ledger --all-features --test interactive_live` | a participant; `ParticipantAdmin` to onboard an external party |
| Admin API | `cargo test -p canton-admin --all-features --test live` | a participant, and its admin port for topology reads |
| PQS | `cargo test -p canton-pqs --all-features --test live` | a Scribe store following the participant |
| Token standard | `cargo test -p canton-token --all-features --test live` | a token-standard registry (a Splice Scan) |

Add `-- --nocapture` to see what each test reports, including why it skipped.

A skipped test and a passing one are the same line in cargo's output, so a run
that reached no participant reads exactly like one that exercised everything.
Set **`CANTON_TEST_REQUIRE_LIVE=1`** on any run whose result you are going to
report: every test that would step aside for a missing variable fails instead.
In the PQS and token suites, a variable that is set but points at something
unreachable is a failure in any case, never a skip.

### Variables

The ledger, interactive and admin suites fall back to the canton-devkit export
for the endpoint and the token, and the ledger suite for the party too, so after
`eval "$(canton-devkit localnet env demo)"` most of the table below is already
covered. Nothing stands in for `CANTON_TEST_LICENSING_PKG` or
`CANTON_TEST_ADMIN_ENDPOINT`; set those yourself.

| Variable | What it gates | Falls back to |
|---|---|---|
| `CANTON_TEST_ENDPOINT` | every gRPC test in the ledger, interactive and admin suites | `CANTON_ENDPOINT`, then `CANTON_GRPC_LEDGER_API_URL` |
| `CANTON_TEST_JSON_ENDPOINT` | the JSON-transport and WebSocket tests | `CANTON_JSON_LEDGER_API_URL` |
| `CANTON_TEST_TOKEN_URL`, `CANTON_TEST_CLIENT_ID`, `CANTON_TEST_CLIENT_SECRET` | OIDC client credentials | the exported token: `CANTON_TOKEN`, then `CANTON_APP_PROVIDER_JWT` |
| `CANTON_TEST_PARTY` | command submission and read-back | `CANTON_APP_PROVIDER_PARTY` |
| `CANTON_TEST_LICENSING_PKG` | the tests that submit commands: the package they instantiate, `#quickstart-licensing` | nothing |
| `CANTON_TEST_ADMIN_CLIENT_ID`, `CANTON_TEST_ADMIN_CLIENT_SECRET` | party management and external-party onboarding, which need `ParticipantAdmin` | an exported token, after asking the participant whether it carries `ParticipantAdmin`; on an unauthenticated participant the interactive suite needs nothing |
| `CANTON_TEST_ADMIN_ENDPOINT` | `canton-admin` topology reads (`3902` on a LocalNet) | nothing |
| `CANTON_TEST_SYNC_ID` | optional: also assert vetted packages in the synchronizer store | nothing |
| `CANTON_TEST_CONTRACT_KEYS_PKG` | the interactive suite's contract-key test: `#contractkeys-h61a17c769e62` | nothing |
| `CANTON_PQS_URL` | the PQS suite: a `postgres://` URL or a libpq `host=… port=… user=… password=… dbname=…` string | nothing |
| `CANTON_TOKEN_REGISTRY_URL` | the token suite: `http://scan.localhost:4000` on a Splice LocalNet or Canton Builder Tool | nothing |
| `CANTON_TEST_REQUIRE_LIVE` | turns every skip into a failure | |

**Credentials.** The suites take whichever the environment offers: the OIDC
client-credentials flow where `CANTON_TEST_TOKEN_URL` and the client variables
are set, otherwise a ready-made bearer token: `CANTON_TOKEN` (on Canton Builder
Tool, `$(canton builder token --validator app-provider)`; on a Splice LocalNet's
`unsafe-jwt-hmac-256` profile, one you signed) or the one canton-devkit exports.
A test that attaches a credential skips when there is neither; the interactive
suite alone also runs with none against a participant that does not
authenticate. `ledger_end_with_oidc_auth` tests the OIDC flow itself and needs
a real issuer. The party-management tests need `ParticipantAdmin`, from the
admin OIDC client or an exported token the participant confirms carries it.

Against cn-quickstart, with its Keycloak:

```sh
export CANTON_TEST_ENDPOINT=http://localhost:3901
export CANTON_TEST_JSON_ENDPOINT=http://localhost:3975
export CANTON_TEST_TOKEN_URL=http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token
export CANTON_TEST_CLIENT_ID=app-provider-backend CANTON_TEST_CLIENT_SECRET=…
export CANTON_TEST_ADMIN_CLIENT_ID=app-provider-validator CANTON_TEST_ADMIN_CLIENT_SECRET=…
export CANTON_TEST_ADMIN_ENDPOINT=http://localhost:3902
export CANTON_TEST_PARTY='app_provider_quickstart-…::1220…'
export CANTON_TEST_LICENSING_PKG='#quickstart-licensing'
export CANTON_TEST_REQUIRE_LIVE=1
cargo test -p canton-ledger --all-features --test live -- --nocapture
```

### Packages for the live suites

The command-submission tests instantiate templates from the cn-quickstart
licensing DAR. cn-quickstart has it installed. For any other LocalNet, the DAR
the `canton-quickstart-licensing` bindings were generated from is committed at
[`testdata/live/quickstart-licensing-0.0.1.dar`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/testdata/live/quickstart-licensing-0.0.1.dar),
with its `.sha256` next to it. Upload it with `canton builder deploy <dar>`,
canton-devkit's `localnet dar upload`, or the JSON Ledger API, as the nightly
workflow does:

```sh
(cd testdata/live && sha256sum -c quickstart-licensing-0.0.1.dar.sha256)
curl -fsS -X POST "$CANTON_TEST_JSON_ENDPOINT/v2/packages" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/octet-stream" \
  --data-binary @testdata/live/quickstart-licensing-0.0.1.dar
export CANTON_TEST_LICENSING_PKG='#quickstart-licensing'
```

`$TOKEN` is an admin token (the workflow fetches one with the
`CANTON_TEST_ADMIN_CLIENT_*` credentials); leave the header out on an
unauthenticated participant. The contract-key test takes
[`testdata/contract-keys/contractkeys.dar`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/testdata/contract-keys/README.md)
the same way, with `CANTON_TEST_CONTRACT_KEYS_PKG='#contractkeys-h61a17c769e62'`.

### A PQS store: `tools/pqs`

The PQS suite reads from a Scribe store.
[`tools/pqs/compose.yaml`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/tools/pqs/compose.yaml)
runs one, Postgres plus Scribe, following the App Provider participant at
`host.docker.internal:3901`:

```sh
docker compose -f tools/pqs/compose.yaml up -d
# where the participant wants a token:
# SCRIBE_SOURCE_LEDGER_AUTH=OAuth CANTON_TOKEN=… docker compose -f tools/pqs/compose.yaml up -d
tools/pqs/wait-ready.sh
export CANTON_PQS_URL='postgres://pqs:pqs@localhost:5433/pqs'
CANTON_TEST_REQUIRE_LIVE=1 cargo test -p canton-pqs --all-features --test live -- --nocapture
docker compose -f tools/pqs/compose.yaml down -v
```

- Run `wait-ready.sh` before querying. Scribe applies its schema and ingests
  from the ledger's beginning first; the script waits, up to six minutes, until
  `latest_offset()` answers and holds still across two checks.
- The tests read `Amulet`, so the party the store follows must hold Canton Coin:
  tap in its wallet first (on a Splice LocalNet, the App Provider wallet at
  `http://wallet.localhost:3000`). An empty store fails the suite.
- `SCRIBE_SOURCE_LEDGER_HOST` and `SCRIBE_SOURCE_LEDGER_PORT` point Scribe at
  another participant; `SCRIBE_IMAGE` and `SCRIBE_VERSION` pick the image.
- cn-quickstart runs a store of its own. Point `CANTON_PQS_URL` at it instead
  (`postgres://cnadmin:…@localhost:5432/pqs-app-provider`).

Querying a store from your own code is covered in [Querying PQS](guides/pqs.md).

### The token registry

The registry the token suite and examples talk to is the super-validator's Scan.
A Splice LocalNet and Canton Builder Tool publish it at
`http://scan.localhost:4000`. cn-quickstart serves it only when started with
`SV_PROFILE=on`, on port `5012` of its `splice` container, without publishing
the port to the host; forward it:

```sh
docker run -d --name canton-rs-scan-forward \
  --network "$(docker network ls --filter name=quickstart --format '{{.Name}}' | head -1)" \
  -p 5012:5012 alpine/socat TCP-LISTEN:5012,fork,reuseaddr TCP:splice:5012
export CANTON_TOKEN_REGISTRY_URL=http://localhost:5012
cargo test -p canton-token --all-features --test live -- --nocapture
```

The token-standard examples take more variables, described in [The token
standard](guides/token-standard.md).

### Tests that need a DAR, not a node

A few code-generation and Daml-LF tests read a DAR path from the environment and
skip without one: `CANTON_TEST_DAR` (one DAR; with `CODEGEN_COMPILE_TEST=1` the
generated crate is compiled too), `CANTON_TEST_DAR_DIR` (every DAR in a
directory, with `CODEGEN_COMPILE_TEST=1`), and `CANTON_LF_ORACLE_DAR` (the
`canton-lf` `oracle` test, which compares the native decoder with the official
JVM reader and needs `scala-cli` on `PATH`). The committed
`testdata/live/quickstart-licensing-0.0.1.dar` serves for any of them.

## The nightly LocalNet workflow

[`.github/workflows/localnet.yml`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/.github/workflows/localnet.yml)
runs all five live suites against a disposable LocalNet every night at 03:00
UTC, on manual dispatch, and on pull requests that change the workflow or
`testdata/live/`. In order, it:

1. Compiles every live suite first, before the LocalNet takes the memory.
2. Boots a LocalNet with one validator, a PQS store and a party, using the
   [canton-localnet](https://github.com/peacefulstudio/canton-localnet) action
   pinned to a commit.
3. Maps the action's `CANTON_LOCALNET_*` exports to the `CANTON_TEST_*`,
   `CANTON_PQS_URL` and `CANTON_TOKEN_REGISTRY_URL` names above. A missing
   export fails this step, instead of reaching a suite as an empty value.
4. Checks the licensing and contract-keys DARs against their `.sha256` files and
   uploads both over the JSON API. It then confirms the participant holds the
   package id `canton-quickstart-licensing` pins, so the committed DAR and the
   committed bindings cannot drift apart.
5. Taps Amulet into the validator's wallet, for the PQS suite to read.
6. Runs each suite with `CANTON_TEST_REQUIRE_LIVE=1` and asserts its exact
   passed count and zero ignored tests. The PQS step also requires the line its
   Amulet comparison prints, since one PQS test would otherwise pass on a store
   with no Amulet.
7. Tears the LocalNet down, whatever happened before.

The expected counts live in the workflow file, so a change that adds or removes
a live test updates them there. What CI covers and what it cannot is set out in
the [compatibility matrix](reference/compatibility-matrix.md#what-ci-runs-and-what-it-cannot);
the DevNet runs, update ids included, are in [Live runs on
record](reference/live-runs.md).
