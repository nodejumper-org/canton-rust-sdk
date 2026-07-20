# Architecture Decision Records

Each significant, hard-to-reverse decision gets a numbered record: the context
at the time, the decision, and its consequences. Records are immutable — a
reversal is a new ADR that supersedes the old one.

| ADR | Title | Status |
|---|---|---|
| [0001](0001-vendored-protos-pinned-to-a-canton-release.md) | Vendored protos, pinned to a Canton release | Accepted |
| [0002](0002-proto-types-are-protocol-stable.md) | Generated proto types are protocol-stable, not SemVer-stable | Accepted |
| [0003](0003-connection-kernel-in-canton-core.md) | Shared connection kernel lives in `canton-core` | Accepted |
| [0004](0004-streams-use-precise-capturing.md) | Returned streams are detached from the client lifetime (`use<>`) | Accepted |
| [0005](0005-lockstep-versioning.md) | All `canton-*` crates version in lockstep | Accepted |
| [0006](0006-telemetry-emits-consumer-exports.md) | Telemetry: the SDK emits, the application exports | Accepted |
