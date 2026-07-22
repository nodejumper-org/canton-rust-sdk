# canton-splice-wallet

Pre-built typed Rust bindings for the Splice **`splice-wallet`** DAR — part of the
Canton Rust SDK (Milestone 2). `src/lib.rs` is generated, checked-in output —
do not edit by hand.

| | |
|---|---|
| Source DAR | `splice-wallet-0.1.14.dar` |
| Daml SDK | 3.3.0 |
| Generator | `dpm-codegen-rust` (`canton-codegen-cli`) |

## Regenerate

```bash
cargo run -p canton-codegen-cli --bin dpm-codegen-rust -- \
  --dar /path/to/splice-wallet-0.1.14.dar --out crates/canton-splice-wallet --name canton-splice-wallet
git checkout crates/canton-splice-wallet/Cargo.toml   # CLI writes a standalone one
cargo fmt -p canton-splice-wallet
```

A drift guard in `canton-codegen-cli` regenerates and compares at the AST level
against the checked-in `src/lib.rs`.
