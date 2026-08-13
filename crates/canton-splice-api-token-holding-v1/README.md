# canton-splice-api-token-holding-v1

Pre-built typed Rust bindings for the Token Standard holding API (`splice-api-token-holding-v1`), generated from
`splice-api-token-holding-v1-1.0.0.dar` by [`canton-codegen`](https://crates.io/crates/canton-codegen).

```sh
cargo add canton-splice-api-token-holding-v1
```

`src/lib.rs` is generated output and is not edited by hand. The packages this
DAR shares with the other `canton-splice-*` crates are **referenced** from
their own crates rather than copied, so a `ContractId<Holding>` obtained
through any of them is the same type everywhere.

Part of the [Canton Rust SDK](https://github.com/nodejumper-org/canton-rust-sdk).
Licensed under Apache-2.0.
