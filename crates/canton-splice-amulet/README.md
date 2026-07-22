# canton-splice-amulet

Pre-built, typed Rust bindings for the Splice **`splice-amulet`** DAR — part of
the Canton Rust SDK (Milestone 2).

`src/lib.rs` is **generated, checked-in output** — do not edit by hand. It is a
`pub mod` tree (`crate::<package>::<module>::<Type>`) covering the whole DAR
dependency closure, with JSON + gRPC codecs, typed `Choice`/`Template`/
`Interface` impls, and SCU-aware template ids. It depends only on the
`canton-daml` runtime.

## Provenance

| | |
|---|---|
| Source DAR | `splice-amulet-0.1.14.dar` |
| Daml SDK | 3.3.0 |
| Generator | `dpm-codegen-rust` (`canton-codegen-cli`) |

## Regenerate

From the workspace root, with the DAR available:

```bash
cargo run -p canton-codegen-cli --bin dpm-codegen-rust -- \
  --dar /path/to/splice-amulet-0.1.14.dar \
  --out crates/canton-splice-amulet \
  --name canton-splice-amulet
# then restore the workspace-member Cargo.toml (the CLI writes a standalone one)
git checkout crates/canton-splice-amulet/Cargo.toml
```

An env-gated test in `canton-codegen-cli` (`CANTON_SPLICE_AMULET_DAR=/path/to.dar`)
regenerates and diffs against the checked-in `src/lib.rs`, so the committed
bindings cannot silently drift from the DAR.
