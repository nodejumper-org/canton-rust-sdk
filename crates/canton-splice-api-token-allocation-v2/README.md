# canton-splice-api-token-allocation-v2

Pre-built typed Rust bindings for the Splice `splice-api-token-allocation-v2` package
— the CIP-0112 (V2) token standard's allocations.

Part of the [Canton Rust SDK](https://github.com/nodejumper-org/canton-rust-sdk).

## Generated, not written

`src/lib.rs` is checked-in generated output. Do not edit it by hand; a drift
guard in `canton-codegen` regenerates it and fails if it no longer matches.

Packages this one depends on are **referenced** from their own crates rather
than copied, so a `Holding` is the same Rust type whichever crate reaches it.

## Where the package came from

The V2 token standard is published as no DAR: cn-quickstart carries only the V1
set, the Splice repository holds Daml sources rather than built artefacts, and
there are no release assets. It exists on the network, vetted by the
participants there, and the Ledger API's `GetPackage` is how it is obtained —
see `testdata/token-standard-v2/README.md`.

V2 reuses `splice-api-token-metadata-v1`; there is no `metadata-v2`.

## Licence

Apache-2.0. See `LICENSE`.
