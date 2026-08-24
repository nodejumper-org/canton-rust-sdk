# Releasing

All `canton-*` crates release in lockstep on one version
([ADR-0005](docs/adr/0005-lockstep-versioning.md)). Publishing is the one step
here that cannot be undone — a crates.io version is permanent, and half a
family published in the wrong order leaves the rest unpublishable until the
next version.

## Order

`cargo publish` builds the package to verify it, which resolves **dev**
dependencies as well as normal ones, so a crate cannot go before anything it
depends on either way. This order is derived from the manifests, not from
memory:

1. `canton-core`
2. `canton-lf`
3. `canton-proto`
4. `canton-auth`
5. `canton-codegen`
6. `canton-admin`  ← dev-depends on `canton-auth`, which is why it is not third
7. `canton-codegen-cli`
8. `canton-daml`
9. `canton-ledger`
10. `canton`
11. `canton-splice-amulet`
12. `canton-splice-wallet`
13. `canton-splice-wallet-payments`

`canton-quickstart-licensing` and `canton-sample` are `publish = false`: the
first is generated from a DAR built from source, the second is the reference
app.

Re-derive the order after adding a crate:

```sh
cargo metadata --format-version 1 --no-deps >/dev/null   # manifests parse
tools/publish-order.sh                                    # prints the list above
```

Each step waits for the index: `cargo publish -p <crate>` then let it appear
before the next (crates.io is usually seconds; `cargo publish` will fail fast
with "no matching package" if it is not there yet).

## Before publishing

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features`
- `cargo deny check all`
- `cargo hack clippy --workspace --feature-powerset --no-dev-deps -- -D warnings`
- `cargo semver-checks -p canton-core -p canton-auth -p canton-ledger -p canton-admin -p canton --all-features`
  (only crates with a published baseline; add each new one after its first release)
- The live suites against a participant, with skips made fatal:
  `CANTON_TEST_REQUIRE_LIVE=1 cargo test -p canton-ledger --features ws --test live`
  and the same for `-p canton-admin`
- The reference app end to end on both transports: `cargo run -p canton-sample`
- `CHANGELOG.md`: turn `— unreleased` into the date
- `README.md`: the status line names what is released
- The tree is clean and tagged `v<version>`

## After publishing

- GitHub release on the tag, notes from the changelog section
- `docs/compatibility-matrix.md` (M3 onward) names the released version
