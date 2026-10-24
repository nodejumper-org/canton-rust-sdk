# Typed bindings from a DAR

Codegen turns a DAR into a Rust crate: templates become structs, choices become
typed argument structs linked to their template, and every generated type
carries both the JSON and the gRPC codec. Three crates are involved:

- [`canton-codegen-cli`](https://crates.io/crates/canton-codegen-cli) provides
  the `dpm-codegen-rust` binary, which writes a complete bindings crate from a
  DAR. Most projects need only this.
- [`canton-codegen`](https://docs.rs/canton-codegen/latest/canton_codegen/) is
  the library behind the binary, for tooling that generates crates itself.
- [`canton-daml`](https://docs.rs/canton-daml/latest/canton_daml/) is the
  runtime the generated code imports as `rt`: the Daml primitive types, the
  `Template`/`Choice` traits and the command builders.

If the DARs you need are Splice's, skip generation and use the
[pre-built crates](#pre-built-splice-crates).

## Install the generator

```sh
cargo install canton-codegen-cli --version 0.3.2 --locked
dpm-codegen-rust --version
```

Install the same release as the SDK crates your application uses. All
`canton-*` crates release in lockstep
([ADR-0005](../reference/adr/0005-lockstep-versioning.md)), and a generated
crate asks for `canton-daml` at the generator's own major and minor version
(`0.3` for this release). `--locked` builds the binary with the lockfile
published with the release instead of the newest dependency versions.

The binary is named for dpm's subcommand convention: dpm resolves
`dpm codegen-rust` to a `dpm-codegen-rust` executable, the way git and cargo
resolve theirs. With Cargo's bin directory on your `PATH`, both spellings run
the same binary and take the same flags.

## Generate a bindings crate

```sh
dpm-codegen-rust --dar dars/my-app-0.1.0.dar --out my-app-bindings --name my-app-bindings
```

This writes two files, `my-app-bindings/Cargo.toml` and
`my-app-bindings/src/lib.rs`, covering the DAR's whole dependency closure. It
prints a summary line and, as warnings, any declaration it could not lower.

| Flag | Meaning |
|---|---|
| `--dar <PATH>` | The input DAR. Required. |
| `--out <DIR>` | The output crate directory, created if absent. Required. |
| `--name <NAME>` | The generated package name. Without it the name comes from the DAR file name with its version dropped: `my-app-0.1.0.dar` gives `my-app`. |
| `--runtime-path <PATH>` | Depend on `canton-daml` by path instead of from crates.io. |
| `--runtime-version <VER>` | The `canton-daml` version requirement. Defaults to the generator's major and minor version (`0.3`). |
| `--force` | Overwrite a `Cargo.toml` or `src/lib.rs` this tool did not write. |

Pass `--name` when the package name has to match the dependency key you will
use. The default name leaves the version out so that a DAR version bump does
not rename the crate; the crate's `version` is taken from the DAR's main
package instead. `--runtime-path` and `--runtime-version` both
set the `canton-daml` dependency, so passing both is an error.

The generated manifest declares edition 2021 and leaves `license` and
`repository` commented out for you to fill in before publishing. Regeneration
rewrites both files, so keep hand edits out of them.

### Commit the crate and depend on it by path

```toml
[dependencies]
my-app-bindings = { path = "my-app-bindings" }
canton          = "0.3"
```

Committing the output keeps every change to the generated code reviewable in a
diff, and it is how the `canton-splice-*` crates in this repository are built.
The bindings crate depends on `canton-daml` itself; your code can name the
runtime as `canton::daml` through the facade, or depend on `canton-daml`
directly. At matching versions it is one crate.

Use `--runtime-path` in a monorepo, or to try bindings against an unreleased
checkout of the SDK. When the crate depends on `canton-daml` from crates.io,
the tool prints a note pointing to `--runtime-path` in case that dependency
does not resolve.

### Regenerating over the tool's own output

The generator recognises its own files by a marker in their first lines and
refuses to overwrite a `Cargo.toml` or `src/lib.rs` without it. Regenerating into a directory it
wrote needs no `--force`, and the same DAR produces byte-identical output.
`--force` exists for overwriting files this tool did **not** write: a thing to
do on purpose, not a flag to keep in a script.

The output is formatted with `prettyplease`, not rustfmt. If your workspace's
`cargo fmt` reaches the bindings crate, run `cargo fmt -p my-app-bindings` after
every regeneration, including the one in CI below, or the check will report
formatting as drift.

## Regenerate in CI and fail on a diff

To keep the committed crate in step with the DAR, regenerate in CI and fail
when anything changed:

```sh
cargo install canton-codegen-cli --version 0.3.2 --locked
dpm-codegen-rust --dar dars/my-app-0.1.0.dar --out my-app-bindings --name my-app-bindings
git diff --exit-code my-app-bindings
```

The step fails when the DAR changed and nobody regenerated, or when someone
edited generated code by hand. When you move to a new SDK release, bump the
`--version` here together with the SDK dependencies.

## Why not a build script

Generating during the build does not work, for two reasons:

- Cargo resolves path dependencies before it runs build scripts. On a clean
  checkout `my-app-bindings` does not exist yet, so the build fails before the
  script that would create it runs.
- The `prost-build` arrangement (write into `OUT_DIR`, then `include!` it) does
  not apply either. The emitted tree spans several Daml packages and refers
  between them by `crate::`-qualified paths, which resolve to the root of the
  *including* crate, not to the module the code was placed in.

Generate ahead of time, commit, and let CI check it.

## Using the bindings

### Where the types are

A generated crate is a module tree, `crate::<package>::<module>::<Type>`. The
package segment is the Daml package **name** with hyphens turned into
underscores (`quickstart-licensing` becomes `quickstart_licensing`); the
version is appended only when two packages in the DAR would otherwise share a
module name. The module segment is the Daml module with dots replaced by
underscores (`Licensing.AppInstall` becomes `Licensing_AppInstall`). Each
choice's argument type is named after the choice. The crate-level docs list the packages
it contains, and each template's docs carry its on-ledger id and its choices
with their consuming flag.

The reference app,
[`crates/canton-sample`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-sample/src/main.rs),
uses bindings generated from the cn-quickstart licensing DAR. That bindings
crate, `canton-quickstart-licensing`, lives in the repository and is not
published:

```rust,ignore
{{#include ../../../crates/canton-sample/src/main.rs:18:27}}
```

The full mapping from Daml-LF types and declarations to Rust is in
[Daml-LF to Rust type mapping](../reference/daml-lf-type-mapping.md).

### Create, read back, exercise

[`create_command`](https://docs.rs/canton-daml/latest/canton_daml/fn.create_command.html)
turns a template payload into a Ledger API `Command`.
[`Template::from_created_event`](https://docs.rs/canton-daml/latest/canton_daml/trait.Template.html#method.from_created_event)
decodes a `CreatedEvent` back into the payload type; it is a trait method, so
bring `rt::Template` into scope. The builders do no I/O: the client submits
the command.

```rust,ignore
{{#include ../../../crates/canton-sample/src/main.rs:113:136}}
```

[`exercise_command`](https://docs.rs/canton-daml/latest/canton_daml/fn.exercise_command.html)
takes a typed `ContractId<T>` and a choice argument. The compiler checks that
the choice belongs to the template: the argument type must implement
[`Choice<T>`](https://docs.rs/canton-daml/latest/canton_daml/trait.Choice.html),
and the generator implements it only for the templates and interfaces that
declare that choice.

```rust,ignore
{{#include ../../../crates/canton-sample/src/main.rs:188:202}}
```

`exercise_command` also takes a `ContractId` of an interface, so a choice
can be exercised through an interface without knowing the concrete template.

On the JSON Ledger API, the payload's serde form is the create argument and the
template id is the same identifier written out as a string:

```rust,ignore
{{#include ../../../crates/canton-sample/src/main.rs:235:240}}
```

### Reading events

There are three ways to decode an event into a generated type, and all three
run the same identity check:

- [`Template::from_created_event`](https://docs.rs/canton-daml/latest/canton_daml/trait.Template.html#method.from_created_event)
  takes the gRPC `CreatedEvent`.
- [`Template::from_json_created_event`](https://docs.rs/canton-daml/latest/canton_daml/trait.Template.html#method.from_json_created_event)
  takes a JSON created event, either bare or in the wrappers the JSON API uses
  (`{"CreatedEvent": …}` in a transaction, `createdEvent` in an ACS entry,
  `{"created": {"createdEvent": …}}` from an event query).
- [`canton_ledger::update::CreatedEvent::decode`](https://docs.rs/canton-ledger/latest/canton_ledger/update/struct.CreatedEvent.html#method.decode)
  works on the SDK's transport-neutral update types. `ExercisedEvent` has
  `decode_argument` and `decode_result` for the choice argument and its result.
  See [Reading the ledger](reading-the-ledger.md).

The check compares the event's module, entity and package **name** with the
template's. The package **id** is deliberately not compared, because under
Smart Contract Upgrade the ledger may report any vetted version of the package.
An event from a different package that defines the same `Module:Entity` is
refused, even when its payload has the same shape. An event with no template
id, or with a package id and no package name, is refused rather than decoded on
trust.

The package-name comparison is new in 0.3.2 and can turn a decode that used to
succeed into an error. Events a participant returns carry both fields; a
`CreatedEvent` built by hand in a test may need `package_name` set. See the
[changelog](../project/changelog.md).

### Exercise by key

A template that declares a contract key also implements
[`WithKey`](https://docs.rs/canton-daml/latest/canton_daml/trait.WithKey.html),
whose `Key` type is the key's generated type.
[`exercise_by_key_command`](https://docs.rs/canton-daml/latest/canton_daml/fn.exercise_by_key_command.html)
takes that key in place of a contract id. With bindings generated from the
keyed fixture in
[`testdata/contract-keys`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/testdata/contract-keys/README.md),
where `Account` has a record key:

```rust,ignore
use canton_daml as rt;
// Generated from contractkeys.dar without --name, so the crate is `contractkeys`.
use contractkeys::contractkeys_h61a17c769e62::ContractKeys::{Account, AccountKey, Credit};

let key = AccountKey {
    custodian: rt::Party::new("bank::1220cd"),
    label: "savings".to_string(),
};
let command = rt::exercise_by_key_command::<Account, Credit>(&key, &Credit { delta: rt::Int64(5) });
```

The fixture is a Daml-LF 2.3 package; no Splice DAR declares a key. A
transaction that uses keys is hashed under scheme V3 for interactive
submission, which the SDK recomputes; see
[Interactive submission](interactive-submission.md).

## Template ids and Smart Contract Upgrade

Every template and interface implements
[`Contract`](https://docs.rs/canton-daml/latest/canton_daml/trait.Contract.html),
which carries `PACKAGE_ID`, `PACKAGE_NAME`, `MODULE_NAME` and `ENTITY_NAME`.
[`Contract::template_id`](https://docs.rs/canton-daml/latest/canton_daml/trait.Contract.html#method.template_id)
returns the id in the `#<package-name>` form, for example
`#quickstart-licensing:Licensing.AppInstall:AppInstallRequest`, and every
command builder uses it. The participant then resolves the package version it
has vetted, so an application built against one version of a package keeps
working when the participant upgrades to a compatible one.

When you need the exact build the bindings were generated from,
[`PACKAGE_ID`](https://docs.rs/canton-daml/latest/canton_daml/trait.Contract.html#associatedconstant.PACKAGE_ID)
pins it:

```rust,ignore
let pinned: &str = <AppInstallRequest as rt::Contract>::PACKAGE_ID;
```

Module paths do not carry the package version, so regenerating from a bumped
DAR leaves every path, type, field and `#<package-name>` id unchanged and moves
only the `PACKAGE_ID` constants. The runbook and a recorded run are in
[Regenerating bindings on a DAR upgrade](../reference/scu-regeneration.md).

## Pre-built Splice crates

For the Splice DARs, depend on the published crates instead of generating your
own:

- `canton-splice-amulet`, `canton-splice-wallet`, `canton-splice-wallet-payments`
  for the Splice application DARs.
- `canton-splice-api-token-*-v1` and `-v2` for the token-standard packages
  (CIP-56 and CIP-0112), and `canton-splice-api-featured-app-v1`.
- `canton-daml-stdlib` for the Daml standard library packages every DAR carries.

They reference each other instead of copying shared packages, so a type such as
`canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding`
is one Rust type whichever crate you reach it through. They release with the
rest of the SDK, so keep them at the same version. The
[API reference](../api-reference.md) links each one on docs.rs, and
[The token standard](token-standard.md) shows them in use.

A crate you generate with the CLI embeds its own copy of every package in the
DAR's closure. If your DAR depends on a Splice package, its copy of, say,
`Holding` is a different Rust type from the one in
`canton-splice-api-token-holding-v1`. To reference the published crate instead,
use the library and name the package as external, as below.

## `canton_codegen` as a library

[`canton_codegen::generate`](https://docs.rs/canton-codegen/latest/canton_codegen/fn.generate.html)
is what the CLI runs. Call it from a small tool of your own when the CLI's flags
are not enough, for example to reference packages that are already published
as crates:

```rust,ignore
use canton_codegen::{Options, generate};

let options = Options::new("dars/my-app-0.1.0.dar", "my-app-bindings")
    .with_crate_name("my-app-bindings")
    .with_external_package("splice-api-token-holding-v1", "canton_splice_api_token_holding_v1");
let stats = generate(&options)?;
for skipped in &stats.skipped {
    eprintln!("warning: skipped {skipped}");
}
```

- [`Options`](https://docs.rs/canton-codegen/latest/canton_codegen/struct.Options.html)
  mirrors the CLI flags: `with_crate_name`, `with_runtime` (taking a
  [`Runtime`](https://docs.rs/canton-codegen/latest/canton_codegen/enum.Runtime.html),
  `Version` or `Path`) and `with_force`.
- [`with_external_package`](https://docs.rs/canton-codegen/latest/canton_codegen/struct.Options.html#method.with_external_package)
  is keyed by package name (preferred, since it survives a version bump) or
  package id. The generated manifest lists each such crate with the version
  `"*"` and a `# pin this` comment; pin it to your SDK version.
- [`Stats`](https://docs.rs/canton-codegen/latest/canton_codegen/struct.Stats.html)
  reports the packages, modules and items written and the declarations skipped.
  Failures are a
  [`GenerateError`](https://docs.rs/canton-codegen/latest/canton_codegen/enum.GenerateError.html).

For finer control,
[`lower_dar`](https://docs.rs/canton-codegen/latest/canton_codegen/fn.lower_dar.html)
produces the public [`ir`](https://docs.rs/canton-codegen/latest/canton_codegen/ir/index.html)
and [`generate_crate`](https://docs.rs/canton-codegen/latest/canton_codegen/fn.generate_crate.html)
turns it into the source of `lib.rs` alone. This repository regenerates its own
bindings crates that way, so their curated manifests are left alone; see
[`regenerate_splice.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-codegen/examples/regenerate_splice.rs).
[`lower_packages_selecting`](https://docs.rs/canton-codegen/latest/canton_codegen/fn.lower_packages_selecting.html)
does the same from packages already decoded rather than from a DAR, such as
packages downloaded from a participant with the
[`fetch_packages`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-admin/examples/fetch_packages.rs)
example.

## Limits

- The decoder reads Daml-LF 2.1 to 2.3 and refuses other versions with an
  error naming them. The current range is in the
  [compatibility matrix](../reference/compatibility-matrix.md).
- A declaration the generator cannot represent correctly is skipped and
  reported, not emitted with a wrong encoding. The one known case, a nested
  `Optional` behind a type parameter, is described in the
  [type mapping](../reference/daml-lf-type-mapping.md).
