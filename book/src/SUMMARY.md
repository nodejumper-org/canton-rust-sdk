# Summary

[Introduction](introduction.md)

# Using the SDK

- [Getting started](getting-started.md)
- [Local development and testing](local-development.md)
- [Reading the ledger](guides/reading-the-ledger.md)
- [Typed bindings from a DAR](guides/codegen.md)
- [The token standard](guides/token-standard.md)
- [Interactive submission and external signing](guides/interactive-submission.md)
- [Querying PQS](guides/pqs.md)
- [Admin API and telemetry](guides/admin-and-telemetry.md)
- [API reference (docs.rs)](api-reference.md)

# Reference

- [Compatibility matrix](reference/compatibility-matrix.md)
- [Daml-LF to Rust type mapping](reference/daml-lf-type-mapping.md)
- [Regenerating bindings on a DAR upgrade](reference/scu-regeneration.md)
- [Architecture decisions](reference/adr/index.md)
  - [0001 Vendored protos](reference/adr/0001-vendored-protos-pinned-to-a-canton-release.md)
  - [0002 Proto types are protocol-stable](reference/adr/0002-proto-types-are-protocol-stable.md)
  - [0003 Connection kernel](reference/adr/0003-connection-kernel-in-canton-core.md)
  - [0004 Streams use precise capturing](reference/adr/0004-streams-use-precise-capturing.md)
  - [0005 Lockstep versioning](reference/adr/0005-lockstep-versioning.md)
  - [0006 Telemetry](reference/adr/0006-telemetry-emits-consumer-exports.md)
  - [0007 Thin facade crate](reference/adr/0007-thin-facade-crate.md)
  - [0008 Native Daml-LF decoder](reference/adr/0008-native-lf-decoder.md)
  - [0009 Daml-LF version support](reference/adr/0009-lf-version-support.md)
  - [0010 Decisions deferred to 1.0](reference/adr/0010-api-decisions-deferred-to-1.0.md)
- [Security review scope](reference/audit-scope.md)
- [Live runs on record](reference/live-runs.md)

# Project

- [Maintenance](project/maintenance.md)
- [Security policy](project/security.md)
- [Contributing](project/contributing.md)
- [Releasing](project/releasing.md)
- [Changelog](project/changelog.md)
