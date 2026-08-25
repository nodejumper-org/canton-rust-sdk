# ADR-0005: All `canton-*` crates version in lockstep

**Status:** Accepted (pre-first-release) · **Date:** 2026-07

## Context

The workspace ships five interdependent crates (`canton-proto`, `canton-core`,
`canton-auth`, `canton-ledger`, `canton-admin`; later `canton-codegen`,
`canton-token`, `canton-pqs`, and the `canton-splice-*` family). Independent
per-crate versions would force a compatibility matrix between our own crates
and make "which versions work together?" a real user question.

## Decision

One version for the whole family, driven by `workspace.package.version`:

- Every `canton-*` crate is released **in lockstep** with the same version,
  even when a crate has no changes.
- Internal dependencies name the release's own version in the workspace
  dependency table (`version = "x.y.z"`), which Cargo reads as a caret
  requirement. **This constrains the minor, not the patch:** with `0.2.0`
  published, a consumer whose lockfile pairs `canton-ledger 0.2.0` with
  `canton-core 0.2.1` resolves and builds. Crossing a minor does not — that is
  where the guarantee lies, and it is the boundary this project breaks API at
  before 1.0.
- Mixed *patch* versions are therefore possible and are expected to work: the
  release train publishes them together, and a patch is by definition
  non-breaking. Pinning `=x.y.z` between our own crates would forbid the
  resolution rather than the misbehaviour, and would make a security patch to
  one crate un-installable without re-releasing all of them.
- A breaking change in any crate bumps the shared version's SemVer-major
  (post-1.0; pre-1.0 the minor plays that role).
- The `canton-splice-*` generated crates (M2+) additionally encode the DAR
  version in their own version metadata but follow the same release train.

## Consequences

- Users hold exactly one version number; the README compatibility table maps
  it to the supported Canton release.
- Some releases re-publish unchanged crates — accepted cost for a trivially
  correct compatibility story (same trade the C# SDK made).
