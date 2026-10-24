# Maintenance

How the Canton Rust SDK is maintained, what that covers, and what it does not.
The SDK is maintained by [Nodejumper](https://nodejumper.io); its development
is funded by the Canton Dev Fund
([proposal](https://github.com/canton-foundation/canton-dev-fund/blob/main/proposals/2026-06-Nodejumper-rust-sdk.md)).

## What is maintained

Every `canton-*` crate published from this repository, and the
`dpm-codegen-rust` binary (`canton-codegen-cli`). They release in lockstep on
one version ([ADR-0005](docs/adr/0005-lockstep-versioning.md)), so "the
current release" always means one version of the whole family.

## What maintenance means here

**Security issues come first.** Reports go through [SECURITY.md](SECURITY.md).
A confirmed vulnerability is fixed in a patch release of the current minor
before other work continues, and is credited in the release notes.

**Critical bugs come next.** A bug that loses or corrupts data, submits
something the caller did not ask for, or blocks a supported workflow on a
supported Canton version is fixed in a patch release of the current minor.

**New Canton releases are followed.** When a Canton or Splice release changes
the Ledger API, the Daml-LF schema or the token standard, the SDK re-vendors the
schema and ships a compatible release: a minor when the generated types change
([ADR-0002](docs/adr/0002-proto-types-are-protocol-stable.md)), a patch when
they do not. The [compatibility matrix](docs/compatibility-matrix.md) states
which Canton versions each release was verified against. A version is listed
there only after the live suites ran against it.

**Other issues are triaged.** Bug reports, questions and feature requests on
GitHub get an answer, a label, and a decision: fixed, scheduled, or declined
with the reason.

## How releases are made

- **Versioning.** Semantic versioning, applied pre-1.0 as Cargo does. A patch
  never breaks the public API, which `cargo semver-checks` holds every release
  to. Breaking changes wait for the next minor and are listed first in the
  [changelog](CHANGELOG.md). Generated protobuf types are exempt, per the
  stability policy in `canton-proto`.
- **Before 1.0, only the latest minor is maintained.** Fixes land there and are
  not backported. Changes already decided for 1.0 are recorded in
  [ADR-0010](docs/adr/0010-api-decisions-deferred-to-1.0.md).
- **Every release** follows [RELEASING.md](RELEASING.md). That covers the
  publish order, the fresh-install check from outside the workspace, and the
  docs.rs check.

## What CI holds every change to

- Format, clippy with `-D warnings` on every feature combination, and the test
  suite on Linux, macOS and Windows.
- The declared MSRV, docs built the way docs.rs builds them, `cargo deny`
  (RustSec advisories, licences, sources), `cargo semver-checks`, and a build
  with every dependency at its newest compatible version. That last one catches
  a dependency release that would break a user's fresh install before the user
  hits it.
- The Daml-LF decoder against the official JVM reader (the conformance oracle).
- Nightly and on demand, the live suites against a disposable LocalNet.

## What is not promised

- **No guaranteed response or fix times.** The one exception is the
  acknowledgement of a security report in [SECURITY.md](SECURITY.md). The
  priorities above set the order of work, not a deadline.
- **No backports** to earlier minors before 1.0.
- **No support for Canton versions older than the compatibility matrix lists,
  or for unreleased ones.**
- **No date for 1.0.** It ships when the API has settled through the
  independent security review and feedback from teams using the SDK.

## During and after the grant

While the Canton Dev Fund grant runs, maintenance is part of the funded work.

After the grant, Nodejumper keeps maintaining the SDK on the terms above:
- security fixes;
- critical bug fixes;
- compatibility releases for new Canton versions;
- issue triage.

**Extended maintenance is planned as a separate maintenance and support
proposal to the Canton Dev Fund.** That covers committed response times, a
scheduled release cadence, longer support for earlier versions, and new
features. If it is agreed, its terms will be published here.

If maintenance ever stops, this file and the README will say so plainly,
rather than letting the repository go quiet. We will also help hand the crates
over to a maintainer willing to take them on.

## Getting help

- Bugs, questions and feature requests: [GitHub issues](https://github.com/nodejumper-org/canton-rust-sdk/issues).
- Vulnerabilities: privately, as [SECURITY.md](SECURITY.md) describes.
- Contributions: [CONTRIBUTING.md](CONTRIBUTING.md).
