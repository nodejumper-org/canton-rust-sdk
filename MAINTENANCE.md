# Maintenance

How the Canton Rust SDK is maintained, what that covers, and what it does not.
The SDK is maintained by [Nodejumper](https://nodejumper.io); its development
is funded by the Canton Dev Fund
([proposal](https://github.com/canton-foundation/canton-dev-fund/blob/main/proposals/2026-06-Nodejumper-rust-sdk.md)).

## What is maintained

Every `canton-*` crate published from this repository, and the
`dpm-codegen-rust` binary (`canton-codegen-cli`). They release in lockstep on
one version ([ADR-0005](docs/adr/0005-lockstep-versioning.md)).

## Priorities

Work is taken in this order. These are priorities, not deadlines.

1. **Security issues**, reported as [SECURITY.md](SECURITY.md) describes.
   Security fixes ship as patch releases of the current minor.
2. **Critical bugs**: bugs that lose or corrupt data, submit something the
   caller did not ask for, or block a documented workflow on a Canton version
   the compatibility matrix lists. Fixes ship as patch releases of the current
   minor.
3. **New Canton and Splice releases** that change the Ledger API, the Daml-LF
   schema or the token standard: re-vendoring the schema. Generated protobuf
   types may change in such a release, as
   [ADR-0002](docs/adr/0002-proto-types-are-protocol-stable.md) allows.
4. **Other issues.** Bug reports, questions and feature requests go to
   [GitHub issues](https://github.com/nodejumper-org/canton-rust-sdk/issues).
   Triage gives an issue a label and a decision: fixed, scheduled, or declined
   with the reason.

## How releases are made

- **Versioning.** Semantic versioning, applied pre-1.0 as Cargo does: breaking
  changes to the public API go into a new minor and are listed first in the
  [changelog](CHANGELOG.md), and a patch is meant to be compatible. CI's
  `semver` job runs `cargo semver-checks` against the published baseline for
  the crates it lists. Generated protobuf types are exempt, per the stability
  policy in `canton-proto`.
- **Before 1.0, only the latest minor is maintained.** Fixes are not
  backported. Changes already decided for 1.0 are recorded in
  [ADR-0010](docs/adr/0010-api-decisions-deferred-to-1.0.md).
- **Every release** follows [RELEASING.md](RELEASING.md).

## What CI checks

- Format, clippy with `-D warnings` on every feature combination, and the test
  suite on Linux, macOS and Windows.
- On each push and pull request:
  - the declared MSRV;
  - docs built the way docs.rs builds them;
  - `cargo deny` (RustSec advisories, licences, sources);
  - `cargo semver-checks` on the crates its job lists;
  - a build with every dependency at its newest compatible version.
- The Daml-LF decoder against the official JVM reader (the conformance oracle).
- Nightly and on demand, the live suites against a disposable LocalNet.

The [compatibility matrix](docs/compatibility-matrix.md) lists the Canton
versions each release was run against, and which suites ran on each.

## What is not promised

- **No guaranteed response or fix times.** The one exception, while the grant
  runs, is the acknowledgement of a security report in [SECURITY.md](SECURITY.md).
- **No backports** to earlier minors before 1.0.
- **No support for Canton versions older than the compatibility matrix lists,
  or for unreleased ones.**

## During and after the grant

While the Canton Dev Fund grant runs, maintenance is part of the funded work.
After it, the SDK stays open source and is maintained on a best-effort basis,
with no committed response times (the security acknowledgement included),
release dates or support windows. Extended maintenance may be funded
separately.

## Getting help

- Bugs, questions and feature requests: [GitHub issues](https://github.com/nodejumper-org/canton-rust-sdk/issues).
- Vulnerabilities: privately, as [SECURITY.md](SECURITY.md) describes.
- Contributions: [CONTRIBUTING.md](CONTRIBUTING.md).
