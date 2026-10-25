# Canton Rust SDK

An async Rust SDK for the [Canton Network](https://www.canton.network/) Ledger
API. It covers:

- **gRPC and JSON transports.** It talks Ledger API v2 over gRPC and over JSON
  (HTTP and WebSocket), with the same core operations on both. Interactive
  submission, health checks and explicit paging are gRPC-only.
- **Type-safe codegen from DARs.** Daml templates become Rust types.
- **The token standard,** CIP-56 and CIP-0112.
- **Interactive submission** with a pluggable signer.
- **A typed client for the Participant Query Store.**

The SDK is open source under Apache-2.0. Its development is funded by the
Canton Dev Fund ([proposal #407](https://github.com/canton-foundation/canton-dev-fund/pull/407)).

```sh
cargo add canton
```

## What is in it

`canton` is a thin facade over the family, so one dependency gets all of it at
one version. The crates behind it can also be used one at a time:

| Crate | For |
|---|---|
| [`canton-ledger`](https://docs.rs/canton-ledger) | The Ledger API client:<br>• submit, recover an ambiguous submission<br>• read the active contract set<br>• stream updates, resumably<br>• point reads<br>• interactive submission |
| [`canton-auth`](https://docs.rs/canton-auth) | OIDC client-credentials tokens, cached and refreshed, with Keycloak, Auth0 and Okta presets. |
| [`canton-daml`](https://docs.rs/canton-daml) | The runtime under generated bindings:<br>• Daml types, `Template` and `Choice`<br>• command builders<br>• the JSON and gRPC value codecs |
| [`canton-codegen-cli`](https://crates.io/crates/canton-codegen-cli) | `dpm-codegen-rust`: turns a DAR into a typed bindings crate. |
| [`canton-token`](https://docs.rs/canton-token) | Token-standard workflows over the registry API: transfers, allocations, settlement. |
| [`canton-signer`](https://docs.rs/canton-signer) | The `Signer` trait for external signing, with an Ed25519 implementation. |
| [`canton-admin`](https://docs.rs/canton-admin) | Parties, packages and users over the Ledger API's admin services, and topology reads over the Canton admin API. |
| [`canton-pqs`](https://docs.rs/canton-pqs) | Typed queries against a PQS (Scribe) store. |
| `canton-splice-*` | Pre-built bindings for the Splice and token-standard DARs. |

The full list, including the internal crates, is in the
[README](https://github.com/nodejumper-org/canton-rust-sdk#crates).

## Where to start

- New to the SDK: [Getting started](getting-started.md), then
  [Local development and testing](local-development.md) to get a participant
  to talk to.
- Building against your own Daml model:
  [Typed bindings from a DAR](guides/codegen.md).
- Moving tokens: [The token standard](guides/token-standard.md).
- Keys held outside the participant:
  [Interactive submission and external signing](guides/interactive-submission.md).
- Which Canton, Daml-LF and Rust versions are supported:
  [the compatibility matrix](reference/compatibility-matrix.md).
- How the SDK is maintained, what that covers and what it does not:
  [Maintenance](project/maintenance.md).

The API reference for every crate is on [docs.rs](api-reference.md).

## Status

Releases are on [crates.io](https://crates.io/crates/canton). Before 1.0,
breaking changes go into a new minor version and are listed in the [changelog](project/changelog.md). The
changes already decided for 1.0 are recorded in
[ADR-0010](reference/adr/0010-api-decisions-deferred-to-1.0.md).

Bugs and questions go to
[GitHub issues](https://github.com/nodejumper-org/canton-rust-sdk/issues).
Vulnerabilities go privately, as the [security policy](project/security.md)
describes.
