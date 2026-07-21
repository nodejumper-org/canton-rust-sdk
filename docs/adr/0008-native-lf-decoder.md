# ADR-0008: A native Rust Daml-LF decoder (not the JVM `daml-lf-archive`)

**Status:** Accepted (M2) · **Date:** 2026-07 · **Supersedes the LF-decoder plan in proposal §M2**

## Context

M2 codegen has to turn a `.dar` into a typed Rust AST. The proposal committed
to decoding LF **through the JVM `daml-lf-archive`** via a thin JVM helper,
explicitly *"rather than re-implementing LF decoding natively in Rust … LF
decoding and version dispatch are non-trivial … wrapping it is materially less
risky than a hand-written Rust LF parser that would drift from the spec."*

Two things reframed that during implementation:

1. **We are not writing a bespoke parser.** The concern the proposal guarded
   against was a hand-rolled LF parser drifting from the spec. But the Daml-LF
   **protobuf schema is itself part of `daml-lf-archive`** (`daml_lf.proto` /
   `daml_lf2.proto` live in that directory). Vendoring that authoritative schema
   and decoding it with `prost` is *using daml-lf-archive's own definitions* —
   not a bespoke parser. Codegen also only needs the type-level **structure**
   (data types, templates, choices, interfaces, keys), never Daml expressions
   (those run on the participant), so the surface to decode is small and bounded.

2. **The JVM breaks the native-Rust premise.** Wrapping the JVM decoder puts a
   Java + jar build-time dependency (and a Rust↔JVM subprocess/serialization
   bridge) in front of every developer who generates bindings from their own
   DAR — the opposite of the prost-like, pure-`cargo` build-script workflow that
   design partners explicitly asked for.

Precedent (surveyed): the funded, public, JVM-free peer — Go's `noders-team/
go-daml` — decodes the LF protobuf **natively** (vendored `.pb.go`,
`proto.Unmarshal`, zero subprocess). The C# SDK and DA's own TypeScript codegen
are JVM-backed; K2F's Rust effort is mixed (native proto parsing + codegen
referencing daml-lf-archive). So native is a shipping, precedented approach for
non-JVM Canton codegen, though not the majority pattern.

## Decision

Decode Daml-LF **natively in Rust**: vendor `daml-lf-archive`'s own LF 2.x
`.proto` schema and decode packages with `prost` (`canton-lf`), resolving the
interning tables and recovering package id/version/PackageMap (the SCU inputs)
ourselves. No JVM at build time.

- SCU stays consumer-of-results, exactly as the proposal describes: we read the
  resolved package id/hash/version/PackageMap; we never re-implement
  upgrade-compatibility checks.
- The decoder sits **behind the IR seam** (ADR: IR is decoder-agnostic), so the
  choice is reversible — a JVM `daml-lf-archive` backend could be added behind
  the same IR without touching the generator, if ever needed.

## Consequences

- **Better DX / best-practice for a Rust SDK:** a `build.rs` reads a DAR and
  generates types with no JVM, no subprocess — the workflow Rust users want.
  Consumers of pre-built `canton-splice-*` crates never touch a decoder either
  way, so only the (rare) "generate from my own DAR" path is affected, and it is
  the path that benefits most.
- **We own the decode+lower maintenance:** re-vendor the LF proto per supported
  Daml/LF release and keep the lowering correct (bounded to type-level
  structure). Go demonstrates this is maintainable.
- **This deviates from the proposal's stated JVM path** — deliberately, as a
  product improvement (no JVM), using daml-lf-archive's own schema. To be raised
  with the committee at M2 submission rather than presented as-is; reversible via
  the IR seam if they prefer the JVM wrap.
