# ADR-0007: A thin `canton` facade crate is the documented entry point

**Status:** Accepted (pre-first-release) · **Date:** 2026-07

## Context

The proposal's distribution model is per-crate (`cargo add canton-ledger`),
and the SDK works fully that way. But a family of five crates leaves three
gaps: there is no single "start here" crate on crates.io/docs.rs; feature
flags live in different crates (`ws` in `canton-ledger`, `otel` in two
places); and the proposal's adoption gate counts *crates.io downloads*, which
are tracked per crate with no family aggregation — transitive dependencies
inflate the foundation crates' counters, so no member crate is a clean
measure of "installs of the SDK".

## Decision

Ship a sixth crate, `canton`, that is a **thin facade**: re-exports only
(`canton::ledger`, `canton::auth`, `canton::admin`, plus the shared
`canton-core` types at the root), the `ws`/`otel` features forwarded, no
logic of its own. Documentation and quickstarts lead with `cargo add canton`;
per-crate installation stays supported and documented as the "pick pieces"
path.

The facade must stay thin: no wrapper types, no helpers, no prelude until
M2's typed bindings show what users actually need (a "thick" facade would
add a maintenance surface that can drift from the member crates).

## Consequences

- `cargo add canton` gives the whole SDK as one lockstep-versioned set
  (ADR-0005), with one place to enable features.
- The `canton` download counter becomes a clean, conservative adoption
  metric: nobody pulls the facade transitively, so its count ≈ deliberate
  SDK installs. (A lower bound — direct per-crate users bypass it.)
- One more crate in the release train — re-exports only, so the publish cost
  is a single extra `cargo publish`.
- The most discoverable name in the ecosystem hosts the official entry point
  instead of being left to squatters.
