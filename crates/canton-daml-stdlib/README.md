# canton-daml-stdlib

Pre-built typed Rust bindings for the Daml standard library — `daml-stdlib`,
`daml-prim`, `ghc-stdlib` and their per-module packages.

Part of the [Canton Rust SDK](https://github.com/nodejumper-org/canton-rust-sdk).

## Why this crate exists

Every DAR carries the standard library in its dependency closure. Generating a
DAR whole therefore gives each generated crate *its own* `RelTime`, `Set`,
`NonEmpty` and so on — and Rust treats those copies as unrelated types. A
`RelTime` read through one bindings crate then does not typecheck against the
`RelTime` of another, though both name the same Daml type in the same package:

```text
error[E0308]: mismatched types
note: `RelTime` is defined in crate `canton_splice_api_token_metadata_v1`
note: `RelTime` is defined in crate `canton_splice_api_token_holding_v1`
```

This crate owns those packages so that every other crate can reference them.
One `RelTime`, whichever crate you reach it through.

## Generated, not written

`src/lib.rs` is checked-in generated output. Do not edit it by hand; a drift
guard in `canton-codegen` regenerates it and fails if it no longer matches.

The standard library ships no DAR of its own, so this crate is generated from a
*selection* of packages taken out of a DAR that carries them — the one
committed to this repository, which means this crate's drift guard needs no
external checkout and runs in CI with the rest.

## Versioning

The package ids here are those of the Daml SDK the corpus DARs were built with.
A DAR built against a different SDK carries a standard library with *different*
package ids, which is a genuinely different set of Daml packages — not a
mismatch to paper over. Bindings for such a DAR keep their own copy unless a
crate exists for that version too.

## Licence

Apache-2.0. See `LICENSE`.
