# Smart Contract Upgrade (SCU) and regeneration

How the generated bindings stay usable across a package version bump, and the
runbook to demonstrate it.

## What the codegen does for SCU

- **The module path does not carry the version.** A package's types live under
  `crate::<name>::<module>::<Type>` (e.g.
  `crate::quickstart_licensing::Licensing_AppInstall::AppInstallRequest`), so a
  version bump does not rename every path a consumer imports.
  The version is appended only where it has to disambiguate — a DAR that bundles
  two versions of one package under SCU generates `my_app_1_0_0` and
  `my_app_2_0_0`, and both are reachable. If even that repeats (a rebuilt
  package keeps its metadata but gets a new id) the package-id prefix is
  appended too. Covered by unit tests in
  [`lower.rs`](../crates/canton-codegen/src/lower.rs).
- **References resolve by package id.** Every cross-package reference is resolved
  through the package **id hash** (self / imported-by-hash / import-table), so a
  reference always resolves to the exact version it named — verified by the
  18-DAR corpus compiling with all cross-package references intact.
- **Version-independent template id.** `Contract::template_id()` uses the
  `#<package-name>` form, **not** a pinned package-id hash. A create/exercise
  command therefore targets *whatever version the participant has vetted*, which
  is the point of SCU — verified live: `canton-sample` commits
  `#quickstart-licensing:Licensing.AppInstall:AppInstallRequest` and the
  participant resolves the vetted version.

Together these are the SCU mechanics: a consumer built against v1's types keeps
working when the participant upgrades to v2, because commands are addressed by
package **name**, and regenerating against v2 yields the same module, type and
field names — the consumer's `use` paths and call sites are untouched.

## Demo: regenerate on a minor version bump (~30 min)

Requires the Daml toolchain (the `daml` CLI used by cn-quickstart) to build a
second version of a package. Example with the licensing package:

```bash
# 1. Bump the version and rebuild the DAR.
cd .../quickstart/daml/licensing
sed -i '' 's/^version: 0.0.1/version: 0.0.2/' daml.yaml
daml build                      # → .daml/dist/quickstart-licensing-0.0.2.dar

# 2. Regenerate the bindings from the bumped DAR.
cd <sdk-repo>
cargo run -p canton-codegen-cli --bin dpm-codegen-rust -- \
  --dar .../quickstart-licensing-0.0.2.dar --out /tmp/licensing-0.0.2 \
  --name canton-quickstart-licensing

# 3. Diff against the checked-in v0.0.1 bindings.
diff <(cargo run -q -p canton-codegen-cli --bin dpm-codegen-rust -- \
         --dar .../quickstart-licensing-0.0.1.dar --out /tmp/l1 && cat /tmp/l1/src/lib.rs) \
     /tmp/licensing-0.0.2/src/lib.rs
```

**Expected:** the only difference is the `PACKAGE_ID` constant on each
template/interface (the new archive hashes differently). Every module path,
template/interface/choice/field name, and every `#<package-name>` template id is
unchanged — so a consumer's code compiles untouched and its commands still work
against the upgraded package. That is "a version bump regenerates compatible
code."

If the DAR bundles *both* versions (the SCU deployment shape), the two package
modules appear side by side as `quickstart_licensing_0_0_1` and
`quickstart_licensing_0_0_2` instead.

> Not yet run in this repo (no second-version DAR is checked in, and building one
> needs the Daml toolchain). The mechanics above are verified; this runbook is the
> end-to-end demonstration for the M2 review.
