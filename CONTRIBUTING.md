# Contributing to the Canton Rust SDK

Thanks for your interest! Issues, bug reports, and pull requests are welcome.

## Development setup

- Rust **1.88+** (the workspace MSRV; `rustup toolchain install 1.88.0`).
- No system `protoc` needed — the build vendors one.
- Optional, for live integration tests: a local Canton participant via
  [`cn-quickstart`](https://github.com/digital-asset/cn-quickstart) LocalNet.

## Before you open a PR

All of these must pass (CI enforces them):

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features   # -D warnings via workspace lints
cargo test  --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
```

Guidelines:

- **Tests are not optional.** New behavior ships with a unit or in-process
  test; anything that talks to a node also gets an env-gated live test
  (`CANTON_TEST_*` variables — see the test files for the pattern).
- **No panics in library code** (`unwrap`/`expect`/indexing are denied by
  lints outside tests).
- **Errors**: map into `canton_core::Error`, keep the retriable
  classification correct, and document `# Errors` on public functions.
- **SemVer**: public types are `#[non_exhaustive]` where growth is expected;
  builder methods use `with_*`/`add_*` naming. Generated proto types are
  exempt (see the stability policy in `canton-proto`).
- Keep commits focused; update `CHANGELOG.md` under `[Unreleased]`.
- **Architectural decisions** (anything hard to reverse: crate boundaries,
  stability promises, wire-facing behavior) get an ADR in
  [`docs/adr/`](docs/adr/README.md) — add a new numbered record rather than
  editing an old one.
- **Versioning**: all `canton-*` crates release in lockstep with one shared
  version ([ADR-0005](docs/adr/0005-lockstep-versioning.md)).

## Security issues

Please do **not** open public issues for suspected vulnerabilities — see
[SECURITY.md](SECURITY.md).

## License

By contributing you agree that your contributions are licensed under the
Apache License 2.0, the license of this repository.
