# Smart Contract Upgrade (SCU) and regeneration

How the generated bindings stay usable across a package version bump, and the
runbook to demonstrate it.

## What the codegen does for SCU

- **Version in the module name.** A package's types live under
  `crate::<name>_<version>::<module>::<Type>` (e.g.
  `crate::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstallRequest`).
  So two versions of one package produce two distinct module trees and never
  collide — a DAR that bundles both under SCU generates both.
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
package **name**, and regenerating against v2 yields the same type/module/field
names (only the version-suffixed package module changes).

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

**Expected:** the only differences are the package-module name
(`quickstart_licensing_0_0_1` → `..._0_0_2`) and the internal references to it;
every template/interface/choice/field name, and every `#<package-name>` template
id, is unchanged — so a consumer's code compiles and its commands still work
against the upgraded package. That is "a version bump regenerates compatible
code."

> Not yet run in this repo (no second-version DAR is checked in, and building one
> needs the Daml toolchain). The mechanics above are verified; this runbook is the
> end-to-end demonstration for the M2 review.
