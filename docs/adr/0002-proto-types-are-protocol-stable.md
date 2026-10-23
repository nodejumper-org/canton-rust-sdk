# ADR-0002: Generated proto types are protocol-stable, not SemVer-stable

**Status:** Accepted (M1) · **Date:** 2026-07

## Context

Until the M2 code generator produces typed Daml bindings, the client's dynamic
path necessarily exposes generated protobuf types (`canton_ledger::proto`,
`pb::Transaction`, `pb::Completion`, …). Thousands of generated types in the
public surface cannot realistically carry a hand-maintained SemVer promise,
and a `prost`/`tonic` major bump or a Canton re-vendor changes them wholesale.

## Decision

Split the stability promise in two, documented in `canton-proto` and on every
`proto` re-export:

- **Hand-written SDK types** (`Config`, `Error`, builders, clients) carry the
  normal SemVer guarantee.
- **Generated proto types** are *protocol-stable*: they track the vendored
  protos pinned to the Canton release (ADR-0001) and are explicitly **exempt**
  from SemVer. They may change in any release that re-vendors protos or bumps
  `prost`/`tonic`.

## Consequences

- Honest, checkable contract for design partners; `cargo-semver-checks` is
  scoped to the hand-written surface.
- Consumers needing insulation from wire types use the generated bindings for
  payloads and `canton_ledger::update` for the update envelope. The proto-typed
  update reads stay public until 1.0, when they move behind `*_raw` names
  (ADR-0010 §4).
