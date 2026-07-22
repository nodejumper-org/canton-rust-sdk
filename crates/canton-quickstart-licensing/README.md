# canton-quickstart-licensing

Pre-built typed Rust bindings for the cn-quickstart **`quickstart-licensing`**
DAR — the bindings the reference app (`canton-sample`) uses. Part of the Canton
Rust SDK (Milestone 2).

`src/lib.rs` is **generated, checked-in output** — do not edit by hand.

## Provenance

| | |
|---|---|
| Source DAR | `quickstart-licensing-0.0.1.dar` |
| Daml SDK | 3.5.2 |
| Generator | `dpm-codegen-rust` (`canton-codegen-cli`) |

## Regenerate

```bash
cargo run -p canton-codegen-cli --bin dpm-codegen-rust -- \
  --dar /path/to/quickstart-licensing-0.0.1.dar \
  --out crates/canton-quickstart-licensing \
  --name canton-quickstart-licensing
git checkout crates/canton-quickstart-licensing/Cargo.toml  # CLI writes a standalone one
cargo fmt -p canton-quickstart-licensing
```

A drift guard in `canton-codegen-cli` (`CANTON_LICENSING_DAR=/path/to.dar`)
regenerates and compares at the AST level against the checked-in `src/lib.rs`.
