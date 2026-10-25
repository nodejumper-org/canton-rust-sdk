# Getting started

This page takes you from an empty crate to a submitted command, and to
recovering a submission whose response was lost. You need a participant:
[Local development and testing](local-development.md) shows how to start a
LocalNet. The snippets assume its App Provider participant, with the gRPC Ledger
API on `http://localhost:3901` and the JSON Ledger API on `http://localhost:3975`.

## Add the SDK

The `canton` crate gives you the whole SDK as one dependency at one version:

```sh
cargo add canton
cargo add tokio --features macros,rt-multi-thread
```

`canton` contains only re-exports ([ADR-0007](reference/adr/0007-thin-facade-crate.md)).
Each member crate is a module: `canton::ledger` is `canton-ledger`, and likewise
`auth`, `admin`, `daml`, `signer`, `token` and `pqs`. The shared `canton-core`
types (`Config`, `Error`, `Result`, `RetryConfig`, `TlsConfig`, `TokenSource`,
`ErrorCategory`) are at the root, next to `canton::localnet` and `canton::telemetry`. The code
generator runs ahead of time, not in your program (you commit what it
writes), so it is not re-exported. See
[Typed bindings from a DAR](guides/codegen.md).

If you only need part of the SDK, depend on the member crates directly:

```sh
cargo add canton-ledger canton-auth
```

All `canton-*` crates are released together, so use equal versions of them
([ADR-0005](reference/adr/0005-lockstep-versioning.md)). The snippets here use
facade paths (`canton::ledger::…`), and the included examples use the member
crates (`canton_ledger::…`). Both name the same items.

The MSRV is Rust 1.88, and the build vendors `protoc`, so you need no system
install. Supported Canton and Daml-LF versions are in the
[compatibility matrix](reference/compatibility-matrix.md).

### Feature flags

| Feature | Crate | What it adds |
|---|---|---|
| `ws` | `canton-ledger` | WebSocket streaming for the JSON transport (`ws_updates`, `ws_active_contracts`, `ws_completions`, `ws_updates_resumable`). You also need it to recover a JSON submission. |
| `otel` | `canton-core`, `canton-ledger` | OTLP span export (`telemetry::otel::otlp_tracer`), and W3C trace-context injection into outgoing gRPC metadata and JSON headers. |
| `ed25519` | `canton-signer` | The in-memory Ed25519 key (`Ed25519Key`), built on `ring`. On by default through the facade. |
| `tls` | `canton-pqs` | Connecting to a PQS store over TLS (`PqsClient::connect_tls`). |

The facade forwards all four, and names the PQS one `pqs-tls`:
`canton = { version = "0.3", features = ["ws", "otel", "pqs-tls"] }`. An HSM or
KMS deployment that must not hold a private key in process memory sets
`default-features = false`, which removes the in-memory `Ed25519Key`, and
implements `Signer` itself. (`ring` stays in the dependency tree either way:
the TLS stack uses it.) See
[Interactive submission and external signing](guides/interactive-submission.md).

## Connect over gRPC

[`CantonClient`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html)
is the gRPC client. You build it from a
[`Config`](https://docs.rs/canton-core/latest/canton_core/config/struct.Config.html).
`connect_lazy` returns immediately and opens the connection on the first RPC,
so a participant that is down fails the call, not the construction. An invalid
endpoint URI returns `Error::InvalidRequest`. Clones of a client share one
connection and are cheap, so give each task its own.

This example prints the participant's Ledger API version and health:

```rust,ignore
{{#include ../../crates/canton-ledger/examples/version_and_health.rs}}
```

Run it from a checkout of the repository with
`cargo run -p canton-ledger --example version_and_health`. `health_check` calls
the standard `grpc.health.v1.Health` service: a healthy participant answers
`ServingStatus::Serving`, and an unreachable one returns a transport error.

`Config::new` starts with no authentication, no retries and a 30-second
per-request timeout. These builder methods change that:

| Method | Effect |
|---|---|
| `with_token(token)` | A fixed bearer token. See [Authenticate](#authenticate). |
| `with_oidc(source)` | A token source that refreshes, such as `canton::auth::TokenProvider` |
| `with_tls(TlsConfig)` | TLS or mutual TLS |
| `with_timeout(duration)` | The per-request timeout. Long `submitAndWait` calls under load may need more than 30 s. |
| `with_retry(RetryConfig)` | Retries for unary calls. See [Errors and retries](#errors-and-retries). |
| `with_max_decoding_message_size(bytes)` | The largest response the client decodes. The default is 128 MiB. |

An `https://` endpoint uses TLS with the platform's root certificates. For a
private CA, a different verified name or a client certificate, pass a
`TlsConfig` built with `with_ca_certificate`, `with_domain_name` or
`with_client_identity`. With `with_tls`, an `http://` endpoint is upgraded to
`https://`, so the connection is never silently plaintext.

On a LocalNet started with canton-devkit, `Config::from_env()` reads the
exported endpoint and token. See [Local development and testing](local-development.md).

## Connect over JSON

[`JsonClient`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.JsonClient.html)
talks to the JSON Ledger API over HTTP, with the same core operations. Its
builders are the counterparts of `Config`'s: `with_token`, `with_oidc`,
`with_retry`, `with_timeout` (30 s by default), `with_max_decoding_message_size`,
and `with_tls`, which takes a `&TlsConfig`, returns a `Result`, and goes last.

```rust,ignore
use canton::ledger::JsonClient;

let json = JsonClient::new("http://localhost:3975").with_token(token);
println!("ledger api version: {}", json.version().await?);
println!("ledger end:         {}", json.ledger_end().await?);
```

`health_check` is gRPC-only. WebSocket streaming and JSON recovery need the
`ws` feature. `JsonClient::from_env()` is the counterpart of `Config::from_env()`.

## Authenticate

A client sends no credentials (the default), a fixed bearer token, or a token
from a source that refreshes it.

**Fixed token.** `Config::with_token` and `JsonClient::with_token` send the
token as is and never refresh it. Use them for a LocalNet's development tokens,
or for a token your application already manages.

**OIDC client credentials.** [`canton-auth`](https://docs.rs/canton-auth/latest/canton_auth/)
implements the OAuth2 client-credentials grant.
[`TokenProvider`](https://docs.rs/canton-auth/latest/canton_auth/struct.TokenProvider.html)
fetches a token, caches it, and refreshes it 30 seconds before it expires.
While a refresh is in flight, other callers wait for that one fetch, which
times out after 30 seconds. Clones share the cache, so one provider can back a
gRPC client and a JSON client:

```rust,ignore
use canton::auth::{OidcConfig, TokenProvider};
use canton::ledger::{CantonClient, Config, JsonClient};

let provider = TokenProvider::new(OidcConfig::keycloak(
    "http://keycloak.localhost:8082", "AppProvider", "app-provider-backend", client_secret,
));
let grpc = CantonClient::connect_lazy(
    Config::new("http://localhost:3901").with_oidc(provider.clone()),
)?;
let json = JsonClient::new("http://localhost:3975").with_oidc(provider);
```

[`OidcConfig`](https://docs.rs/canton-auth/latest/canton_auth/struct.OidcConfig.html)
takes a token endpoint URL directly. The presets build the URL for you and
make each provider's usual token request:

| Constructor | Token endpoint | Notes |
|---|---|---|
| `OidcConfig::new(token_url, client_id, secret)` | `token_url` | Sends the credentials in the request body |
| `OidcConfig::keycloak(base_url, realm, client_id, secret)` | `{base_url}/realms/{realm}/protocol/openid-connect/token` | The LocalNet IdP |
| `OidcConfig::auth0(domain, audience, client_id, secret)` | `https://{domain}/oauth/token` | Sends `audience`, which Auth0 needs to issue a token the participant can verify |
| `OidcConfig::okta(domain, auth_server, client_id, secret)` | `https://{domain}/oauth2/{auth_server}/v1/token` | Sends HTTP Basic credentials, Okta's default |

To adjust a configuration, use `with_scope`, `with_audience` and
`with_client_auth(ClientAuth::Basic)` or `with_client_auth(ClientAuth::Post)`.

Rejected credentials (HTTP 401 or 403) return `Error::Auth`, which is not
retriable. An unreachable or slow endpoint returns `Error::Connection` or
`Error::Timeout`, which are. The SDK does not follow a redirect from the token
endpoint, because that would send the client secret elsewhere. If a participant
rejects a token, `TokenProvider::invalidate()` makes the next call fetch a new
one. `Debug` output never shows the client secret or a `Config`'s tokens.

**Your own token source.** `Config::with_oidc` accepts any
[`TokenSource`](https://docs.rs/canton-core/latest/canton_core/config/trait.TokenSource.html).
The trait has one method, `fetch_bearer`, which returns the current token, or
`None` for an unauthenticated call. Implement it to get tokens from a vault or
a sidecar. `JsonClient::with_oidc` only takes a `TokenProvider`.

## Submit a command and read it back

Without generated bindings, you build commands from the Ledger API's value
types. [`Submit`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.Submit.html)
names the acting party, `create` and `exercise` build the commands,
`identifier(package, module, entity)` names the template, and `record` and the
[`value`](https://docs.rs/canton-ledger/latest/canton_ledger/value/index.html)
constructors build the arguments.

This example authenticates with OIDC, creates an `AppInstallRequest` from the
cn-quickstart licensing package, and prints the committed transaction:

```rust,ignore
{{#include ../../crates/canton-ledger/examples/submit_and_read.rs}}
```

The example reads `ENDPOINT`, `TOKEN_URL`, `CLIENT_ID`, `CLIENT_SECRET`,
`PARTY` and `LICENSING_PKG`, each prefixed `CANTON_` or `CANTON_TEST_`. Run it
with `cargo run -p canton-ledger --example submit_and_read`. The package can be
given by id, or by `#` and its name, as in `#quickstart-licensing`.

There are three ways to submit. Each waits for something different:

| Method | Waits for | Returns |
|---|---|---|
| `submit` | Nothing | The command id, to find the completion with later |
| `submit_and_wait` | The completion | `update_id` and `completion_offset` |
| `submit_and_wait_for_transaction` | The transaction | The transaction, with `update_id`, `offset` and `events` |

The transaction is filtered to the submission's `act_as` and `read_as` parties,
so your created events are in it. If you do not set a command id, the SDK
generates a UUID. You can set your own with `Submit::with_command_id` and the
user with `with_user_id`, and `Submit::new_multi` acts as several parties.

To read a transaction again later, use its update id (or its offset, with
`update_by_offset`). You get `Ok(None)` if the given parties cannot see it:

```rust,ignore
use canton::ledger::proto::get_update_response::Update;

if let Some(Update::Transaction(tx)) = client.update_by_id(&update_id, vec![party.clone()]).await? {
    println!("{} events at offset {}", tx.events.len(), tx.offset);
}
```

Over JSON, `JsonCommands` takes the place of `Submit`, and you pass the
arguments as Daml-LF JSON. Your crate needs `serde_json` for that:

```rust,ignore
use canton::ledger::JsonCommands;

let commands = JsonCommands::new(vec![party.clone()]).add_create(
    format!("{pkg}:Licensing.AppInstall:AppInstallRequest"),
    serde_json::json!({ "provider": party, "user": party, "meta": { "values": {} } }),
);
let response = json.submit_and_wait_for_transaction(&commands).await?;
println!("committed {} at offset {}", response.transaction.update_id, response.transaction.offset);
```

To use your own templates without hand-written records, generate typed bindings
from the DAR. See [Typed bindings from a DAR](guides/codegen.md).

## Recover an ambiguous submission

An error from a submission does not mean the command failed. After a dropped
connection, a timeout, or a retry the participant de-duplicated, the command
may have committed. The way to find out is the command's change ID: the user,
the acting parties and the command id. Canton de-duplicates on all three,
because a command id alone is not unique across a participant's users.

So fix the change ID before you send. `CantonClient::submission` returns a
[`Submission`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.Submission.html)
handle that holds the change ID. If the send fails,
[`recover`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.Submission.html#method.recover)
reads the outcome from the completion stream:

```rust,ignore
use std::time::Duration;

// Take the offset before submitting: a completion that has gone past cannot be read again.
let offset = client.ledger_end().await?;
let submission = client.submission(Submit::new(&party).add_command(command));

if submission.submit_and_wait().await.is_err() {
    let completion = submission.recover(offset, Duration::from_secs(30)).await?;
    println!("committed after all: {}", completion.update_id);
}
```

If the ledger rejected the command, `recover` returns `Error::CommandRejected`.
If no completion arrives in time, it returns `Error::Timeout`. A timeout is the
correct answer for a command that never reached the participant.

The `recover_a_submission` example sends the same change ID twice on purpose.
The participant rejects the second send as a duplicate, and the example
recovers the first one's outcome:

```rust,ignore
{{#include ../../crates/canton-ledger/examples/recover_a_submission.rs:53:89}}
```

Set `LEDGER_PARTY` (and `LEDGER_TOKEN` if needed), then run
`cargo run -p canton-ledger --example recover_a_submission`. `LEDGER_ENDPOINT`
defaults to `http://localhost:3901` and `LEDGER_PKG` to `#quickstart-licensing`.

Over JSON, `JsonClient::submission` returns a
[`JsonSubmission`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.JsonSubmission.html),
whose `recover` reads completions over the WebSocket (feature `ws`) and returns
the completion as raw JSON.

## Errors and retries

Every crate returns one error type,
[`canton_core::Error`](https://docs.rs/canton-core/latest/canton_core/enum.Error.html)
(`canton::Error`, with the `canton::Result` alias). It is `#[non_exhaustive]`,
so a `match` on it needs a wildcard arm.

[`Error::is_retriable`](https://docs.rs/canton-core/latest/canton_core/enum.Error.html#method.is_retriable)
says whether trying again can help:

- `Timeout`, `Transport` and `Connection` are retriable.
- For a gRPC `Status` or a JSON `Http` error, Canton's
  [`ErrorCategory`](https://docs.rs/canton-core/latest/canton_core/enum.ErrorCategory.html)
  decides. Without a category, the gRPC codes `Unavailable`,
  `DeadlineExceeded`, `ResourceExhausted` and `Aborted` are retriable, as is a
  call whose connection dropped mid-flight (an `Unknown` or `Internal` status
  that tonic built from a transport error), and so are HTTP 408, 429 and 5xx.
- Nothing else is retriable, including `Auth`, `InvalidRequest`,
  `CommandRejected`, `Json`, `Payload`, not-found and already-exists.

These accessors work on both transports, so you never match on message text:

```rust,ignore
fn explain(error: &canton::Error) {
    eprintln!(
        "{error}: retriable={} category={:?} reason={:?} correlation_id={:?}",
        error.is_retriable(),
        error.category(),
        error.error_info().map(|info| info.reason), // e.g. DUPLICATE_COMMAND
        error.correlation_id(),                     // quote it to the participant's operator
    );
}
```

Also available: `retry_delay()`, `resource_info()` (the contract, party or
package concerned), `code()` (the gRPC code, on either transport) and
`pruned_earliest_offset()`.

Retries are off by default. Turn them on with `Config::with_retry` or
`JsonClient::with_retry`:

```rust,ignore
use std::time::Duration;
use canton::{Config, RetryConfig};

let config = Config::new("http://localhost:3901").with_retry(
    RetryConfig::default() // 3 attempts, 100 ms initial backoff, 5 s cap
        .with_max_attempts(5)
        .with_attempt_timeout(Duration::from_secs(20)),
);
```

The SDK retries unary calls on retriable errors only, with jittered
exponential backoff, and waits longer if the participant recommends it (up to
60 seconds). A retried submission keeps its command id, so the participant
de-duplicates it. If `submit` sees its own retry rejected as a duplicate, it
reports success. `submit_and_wait` and `submit_and_wait_for_transaction` cannot,
because a de-duplicated retry carries no result: the rejection reaches you, and
`Submission::recover` gets the outcome.

Streams are not covered by this policy. The resumable stream variants, such as
`updates_resumable`, reconnect on their own from the last offset they saw. See
[Reading the ledger](guides/reading-the-ledger.md).

## Next steps

- [Local development and testing](local-development.md): run a LocalNet and the live test suites.
- [Reading the ledger](guides/reading-the-ledger.md): the active contract set, update streams and point reads.
- [Typed bindings from a DAR](guides/codegen.md): replace hand-built records with generated Rust types.
- [The token standard](guides/token-standard.md): transfers, allocations and settlement.
- [Interactive submission and external signing](guides/interactive-submission.md): keys held outside the participant.
- [Querying PQS](guides/pqs.md): typed queries against the Participant Query Store.
- [Admin API and telemetry](guides/admin-and-telemetry.md): parties, packages, topology, spans and metrics.
- [API reference](api-reference.md): every crate on docs.rs.
