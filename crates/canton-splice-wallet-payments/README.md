# canton-splice-wallet-payments

Pre-built typed Rust bindings for the Splice **`splice-wallet-payments`** DAR — part of the
Canton Rust SDK (Milestone 2). `src/lib.rs` is generated, checked-in output —
do not edit by hand.

| | |
|---|---|
| Source DAR | `splice-wallet-payments-0.1.14.dar` |
| Splice release | 0.6.11 |
| Daml SDK | 3.3.0 |
| Generator | `dpm-codegen-rust` (`canton-codegen-cli`) |

## Regenerate

```bash
cargo run -p canton-codegen-cli --bin dpm-codegen-rust -- \
  --dar /path/to/splice-wallet-payments-0.1.14.dar --out crates/canton-splice-wallet-payments --name canton-splice-wallet-payments
git checkout crates/canton-splice-wallet-payments/Cargo.toml   # CLI writes a standalone one
cargo fmt -p canton-splice-wallet-payments
```
