# Admin API and telemetry

Two things an application needs in production but not on the transaction
path: [`canton-admin`](https://docs.rs/canton-admin/latest/canton_admin/) for
packages, parties, users and topology, and the telemetry every client call
emits, with how to get it into your own tracing and metrics pipeline.

## The admin clients

Canton serves these operations on two different gRPC endpoints, so
`canton-admin` has two clients:

| Client | Endpoint | Covers |
|---|---|---|
| [`AdminClient`](https://docs.rs/canton-admin/latest/canton_admin/struct.AdminClient.html) | the **Ledger API** port (`3901` on LocalNet), with the same endpoint and auth as `canton-ledger` | parties, user self-inspect, package reads, external-party onboarding |
| [`TopologyClient`](https://docs.rs/canton-admin/latest/canton_admin/struct.TopologyClient.html) | the Canton **admin API** port (`3902` on LocalNet) | topology reads: party-to-participant mappings, namespace delegations, vetted packages |

Both take the shared [`Config`](https://docs.rs/canton-core/latest/canton_core/config/struct.Config.html)
(endpoint, auth, TLS, retry), re-exported as `canton_admin::Config`, and
`connect_lazy` connects on the first call. Through the facade they are
`canton::admin::AdminClient` and `canton::admin::TopologyClient`.

### Rights

| Calls | What the token needs |
|---|---|
| `current_user`, `current_user_rights`, `acting_parties`, `participant_id` | any authenticated user |
| `list_packages`, `get_package`, `get_package_status` | any authenticated user (the live tests run `list_packages` and `get_package_status` with an ordinary token; `get_package` runs in the `fetch_packages` example) |
| `allocate_party`, `list_known_parties`, `list_known_parties_page`, `get_parties` | `ParticipantAdmin` |
| `generate_external_party_topology`, `allocate_external_party` | `ParticipantAdmin` (how the live tests run them) |
| `get_user` / `list_user_rights` for another user's id | `ParticipantAdmin` or identity-provider admin. An empty id means your own user, which any token may read. |
| every `TopologyClient` call | whatever the admin API is configured to require. It is unauthenticated on LocalNet. Elsewhere, set a token with `Config::with_oidc` or `Config::with_token`. |

To fail early rather than with `PermissionDenied` halfway through a workflow,
check the right first, as the live tests do before a party RPC:

```rust,ignore
use canton_admin::proto::right::Kind;

let rights = admin.current_user_rights().await?;
let is_admin = rights
    .iter()
    .any(|right| matches!(right.kind, Some(Kind::ParticipantAdmin(_))));
```

`acting_parties()` returns only explicit `CanActAs` grants. A user with the
wildcard `CanActAsAnyParty` right gets an empty list, so an empty result does
not mean "cannot act". Read `current_user_rights()` when the wildcard matters.

## Packages

- [`list_packages`](https://docs.rs/canton-admin/latest/canton_admin/struct.AdminClient.html#method.list_packages)
  returns the ids of every package the participant knows. It returns ids only.
- [`get_package`](https://docs.rs/canton-admin/latest/canton_admin/struct.AdminClient.html#method.get_package)
  downloads one package as `ArchivePayload` bytes. A package id is the SHA-256
  of its payload, so the SDK hashes the bytes it receives. If they do not hash
  to the id you asked for, the call fails with `Error::UnexpectedResponse`.
  Asking by id therefore pins the content, and you need no separate checksum.
- [`get_package_status`](https://docs.rs/canton-admin/latest/canton_admin/struct.AdminClient.html#method.get_package_status)
  returns a [`PackageStatus`](https://docs.rs/canton-admin/latest/canton_admin/enum.PackageStatus.html):
  `Registered` for a known package. An unknown id is not an error; its status
  is something other than `Registered`.

The bytes from `get_package` are the payload alone: decode them with
`canton_lf::decode_payload`, not `canton_lf::decode_package`, which reads the
whole `Archive` in a DAR entry. A JSON-only deployment has
[`JsonClient::list_packages`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.JsonClient.html#method.list_packages)
and `JsonClient::package_status` in `canton-ledger` instead
([`json_packages.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-ledger/examples/json_packages.rs)).

### Example: download the packages a participant knows

Some packages exist only on a network and have no published DAR. The token
standard's V2 packages are one example. The participant is then the only
source, and it is also the most authoritative one. The
[`fetch_packages`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-admin/examples/fetch_packages.rs)
example downloads every package, or the ones whose name matches a filter:

```sh
CANTON_TEST_ENDPOINT=http://localhost:3901 \
CANTON_TEST_TOKEN_URL=… CANTON_TEST_CLIENT_ID=… CANTON_TEST_CLIENT_SECRET=… \
  cargo run -p canton-admin --example fetch_packages -- <out-dir> [name-filter…]
```

Without the `CANTON_TEST_*` variables it falls back to `canton_core::localnet`,
which reads the endpoint and token a LocalNet exports. `CANTON_ENDPOINT` and
`CANTON_TOKEN` override those.

```rust,ignore
{{#include ../../../crates/canton-admin/examples/fetch_packages.rs:21:}}
```

Each file is named `<name>-<version>-<package id>.lfpayload`, so the name
carries its own checksum. It is not `.dalf`, because a DAR entry is a whole
`Archive` and these are payload bytes only. A package in a Daml-LF minor
version this build cannot decode is skipped (`DecodeError::UnsupportedMinor`),
not fatal. The repository's
[`testdata/token-standard-v2`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/testdata/token-standard-v2/README.md)
corpus was downloaded this way, and its bindings are generated from that
directory; see [Typed bindings from a DAR](codegen.md).

### Uploading a DAR

`canton-admin` does not wrap uploads. Packages are only read. On LocalNet, the
tools in [Local development and testing](../local-development.md) upload DARs
for you. In code, call `PackageManagementService` through the generated types.
`canton_admin::proto` re-exports them, and they run on a channel built from the
same `Config`. The auth interceptor is `canton_core::auth::intercepted`, which
the `canton` facade does not re-export, so this needs a direct `canton-core`
dependency:

```rust,ignore
use canton_admin::proto::UploadDarFileRequest;
use canton_admin::proto::package_management_service_client::PackageManagementServiceClient;

let channel = config.connect_channel()?;
let authed = canton_core::auth::intercepted(&channel, config.auth()).await?;
PackageManagementServiceClient::new(authed)
    .upload_dar_file(UploadDarFileRequest {
        dar_file: std::fs::read("model.dar")?,
        ..Default::default()
    })
    .await?;
```

Generated types are protocol-stable rather than SemVer-stable: they change when
the vendored protos move to a new Canton release
([ADR-0002](../reference/adr/0002-proto-types-are-protocol-stable.md)).

## Parties

```rust,ignore
use canton_admin::{AdminClient, Config};

// A token with ParticipantAdmin.
let admin = AdminClient::connect_lazy(config)?;

let alice = admin.allocate_party(Some("alice")).await?; // party = "alice::<fingerprint>"
let everyone = admin.list_known_parties().await?;
let (page, next_token) = admin.list_known_parties_page(100, None).await?;
let found = admin.get_parties(vec![alice.party.clone()]).await?;
```

- `allocate_party` is **never retried**, even when a `RetryConfig` is set.
  Allocation is a topology change that is not idempotent, and a lost response
  must not allocate the party twice. If it fails ambiguously, look for a party
  starting with `<hint>::` before you allocate again.
- `list_known_parties` follows pagination for you. It fails instead of
  returning a partial list if the participant repeats a page token or keeps
  going past `canton_core::MAX_LIST_PAGES` pages. To page by hand, use
  `list_known_parties_page`, where a `page_size` of `0` means the server
  default.
- `get_parties` leaves unknown parties out of its result instead of failing.

Updating party details, the identity-provider fields and the participant-permission
options of `AllocateParty` are deliberately not wrapped; use the generated types.

## External parties

An external party's key stays outside the participant, so allocating one takes
two calls with a signature in between. The participant computes the key's
fingerprint, which means the signer cannot be built until the first call
returns:

```rust,ignore
use canton_signer::Ed25519Key;

// Any synchronizer the participant is connected to (canton-ledger's CantonClient).
let synchronizer = ledger
    .connected_synchronizers("")
    .await?
    .into_iter()
    .next()
    .ok_or("not connected to a synchronizer")?
    .synchronizer_id;

let key = Ed25519Key::generate()?;
let topology = admin
    .generate_external_party_topology(&synchronizer, "alice", &key.public_key())
    .await?;
let signer = key.into_signer(topology.public_key_fingerprint());
let party = admin
    .allocate_external_party(&synchronizer, &topology, &signer, &user_id)
    .await?;
assert_eq!(party, topology.party_id());
```

[`ExternalPartyTopology`](https://docs.rs/canton-admin/latest/canton_admin/struct.ExternalPartyTopology.html)
is a proposal: nothing is submitted until the key signs its `multi_hash()`.
Before returning it, the SDK recomputes that hash from the onboarding
transactions, decodes each one, and refuses (`Error::UnexpectedResponse`) a set
that would authorize more than this party's own key, such as a delegation to
another key, a mapping for another party, or a host with submission permission.
`allocate_external_party` grants `user_id` `act_as` on the new party (empty
grants nothing), waits until the allocation reaches the synchronizer, and like
`allocate_party` is never retried.

Signing as that party, including with an HSM or KMS behind the `Signer` trait,
is covered in [Interactive submission and external signing](interactive-submission.md).

## Topology reads

```rust,ignore
use canton_admin::{Config, Store, TopologyClient};

// LocalNet's admin API is unauthenticated.
let topology = TopologyClient::connect_lazy(Config::new("http://localhost:3902"))?;

let hosting = topology.list_party_to_participant(Store::Authorized, "", "").await?;
for entry in &hosting {
    println!("{} on {} participant(s)", entry.item.party, entry.item.participants.len());
}
let delegations = topology.list_namespace_delegations(Store::Authorized, "", "").await?;
// Vetted packages live in a synchronizer's store, addressed by its physical id.
let vetted = topology.list_vetted_packages(Store::Synchronizer(physical_id), "").await?;
```

- An empty filter string matches everything.
  [`Store`](https://docs.rs/canton-admin/latest/canton_admin/enum.Store.html)
  selects the store: `Authorized` (the node's own state),
  `Synchronizer(physical_id)`, or `Temporary(name)`. Vetted packages are stored
  per synchronizer, so querying them against `Store::Authorized` returns an
  empty list.
- Each result is an
  [`Entry`](https://docs.rs/canton-admin/latest/canton_admin/struct.Entry.html)
  with the mapping in `item` and its validity, serial and store in `context`.
  If a row lacks either half, the whole read fails. A list that is silently
  short would look like a true answer to "which participants host this party".
- Reads use the current head state and leave out proposals. `TopologyClient` is
  read-only.

## Telemetry

### What the SDK emits

Every RPC made by the `canton-ledger` clients (gRPC, JSON and WebSocket) and by
both `canton-admin` clients goes through
[`canton_core::telemetry`](https://docs.rs/canton-core/latest/canton_core/telemetry/index.html).
The PQS and token-registry clients do not.

| Span | Fields | Covers |
|---|---|---|
| `canton.rpc` | `method`, `transport` | one call, including every retry attempt |
| `canton.stream` | `method`, `transport` | the whole life of a stream. Each item is polled inside it. |

| Counter | Labels |
|---|---|
| `canton_client_requests_total` | `method`, `transport` |
| `canton_client_errors_total` | `method`, `transport`, `retriable` |

- `method` is the SDK method name, such as `allocate_party` or
  `list_packages`. A convenience call reports under the call it delegates to,
  so `current_user` reports as `get_user`. `transport` is `grpc` or `json`, and
  it tells the twins apart, since `list_packages` exists on both. The names are
  public constants: `METRIC_REQUESTS`, `METRIC_ERRORS`, `TRANSPORT_GRPC` and
  `TRANSPORT_JSON`.
- Successes are not counted separately: success = requests − errors. A retried
  call counts as one request.
- A stream counts as one request when it opens. Its errors are counted as they
  arrive, so a subscription that fails ten minutes in still shows up in
  `canton_client_errors_total`.
- Structured events go with the spans: `rpc completed` and `stream ended` at
  `debug`, `rpc failed` and `stream failed` at `warn`, with the error and
  `retriable`. All of them have the target `canton_core::telemetry`, so a
  filter such as `canton_core=debug` shows every call. Each event carries a
  `trace_id` field. It is empty unless an OpenTelemetry context is active (see
  below).

### The SDK emits, the application exports

The SDK installs nothing process-wide: no tracing subscriber, no metrics
recorder and no exporter
([ADR-0006](../reference/adr/0006-telemetry-emits-consumer-exports.md)). Until
your application installs them, everything above is a no-op. You choose the
log level, format and destination.

Spans and events go through `tracing` 0.1 and counters through the `metrics`
0.24 facade, so your subscriber and recorder must be on those major versions to
see them. Any `metrics` recorder works, a Prometheus exporter for example.

### The `otel` feature

`otel` adds what belongs on the SDK side of an OpenTelemetry setup:

- [`otel::otlp_tracer`](https://docs.rs/canton-core/latest/canton_core/telemetry/otel/fn.otlp_tracer.html)
  and [`otel::otlp_tracer_provider`](https://docs.rs/canton-core/latest/canton_core/telemetry/otel/fn.otlp_tracer_provider.html)
  build a batching OTLP/gRPC span exporter; take the provider to flush or shut it down.
- [`otel::otlp_metrics`](https://docs.rs/canton-core/latest/canton_core/telemetry/otel/fn.otlp_metrics.html)
  builds an OTLP metrics pipeline and installs it as the process's `metrics`
  recorder. It fails if a recorder is already installed.
- W3C trace context (`traceparent` and `tracestate`) is added to every
  outgoing gRPC request's metadata, to JSON request headers, and to the
  WebSocket upgrade request. This happens only when a valid OpenTelemetry span
  context is active. The WebSocket upgrade is the only request a WebSocket
  stream makes, so it is the only place the context can go.
- Under such a context the events' `trace_id` is filled in, so a log line can
  be joined to its trace.

| You depend on | Turn on |
|---|---|
| `canton` | `features = ["otel"]`, forwarded to `canton-ledger` and `canton-core` |
| `canton-ledger` | `features = ["otel"]`, forwarded to `canton-core` |
| `canton-admin` without the others | `canton-core` with `features = ["otel"]`. `canton-admin` has no `otel` feature of its own; its gRPC calls go through `canton-core`'s interceptor. |

### Wiring an exporter

The application owns the subscriber. The exporter types come from the
OpenTelemetry versions `canton-core` is built on, so match them: `opentelemetry`
0.27 and `tracing-opentelemetry` 0.28.

```toml
[dependencies]
canton = { version = "0.3", features = ["otel"] }
opentelemetry = "0.27"
tracing = "0.1"
tracing-opentelemetry = "0.28"
tracing-subscriber = "0.3"
```

```rust,ignore
use opentelemetry::trace::TracerProvider as _;
use tracing_subscriber::layer::SubscriberExt as _;

const SERVICE: &str = "my-service";
const COLLECTOR: &str = "http://localhost:4317";

// Counters: builds the pipeline and installs it as the `metrics` recorder.
let meters = canton::telemetry::otel::otlp_metrics(SERVICE, COLLECTOR)?;

// Spans: the SDK builds the exporter, the application owns the subscriber.
let spans = canton::telemetry::otel::otlp_tracer_provider(SERVICE, COLLECTOR)?;
let subscriber = tracing_subscriber::registry()
    .with(tracing_opentelemetry::layer().with_tracer(spans.tracer(SERVICE)));
tracing::subscriber::set_global_default(subscriber)?;

// ... run the application ...

// Spans go out in batches and metrics periodically. Flush before exit, or the last batch is lost.
spans.force_flush();
meters.shutdown()?;
```

Add your own log layer to the same registry, for example
`tracing_subscriber::fmt::layer()`. Keep `meters` alive for the whole process,
because dropping it early loses the last export interval.

Two tests check this path end to end.
[`otlp_export.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-core/tests/otlp_export.rs)
runs its own OTLP collector and checks that the `canton.rpc` span and both
counters arrive with their labels;
[`ws_trace.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-ledger/tests/ws_trace.rs)
checks that a WebSocket upgrade carries `traceparent` next to `authorization`.
