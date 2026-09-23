# Daml-LF → Rust type mapping

The mapping `canton-codegen` applies when generating Rust from a DAR. It is
implemented in [`canton-codegen/src/map.rs`](../crates/canton-codegen/src/map.rs)
(types) and [`emit.rs`](../crates/canton-codegen/src/emit.rs) (declarations); this
document is the human-readable specification of that deliverable. Generated code
depends on the `canton-daml` runtime, referenced below as `rt`.

## Primitive and container types

| Daml-LF type | Rust type | JSON (LF-JSON) | gRPC (`Value`) |
|---|---|---|---|
| `Unit` | `rt::Unit` | `{}` (empty object) | `Unit` |
| `Bool` | `bool` | `true` / `false` | `Bool` |
| `Int64` | `rt::Int64` | **string** (number also accepted on input) | `Int64` |
| `Numeric n` | `rt::Numeric` | **string** (number also accepted on input) | `Numeric` (string) |
| `Text` | `String` | string | `Text` |
| `Party` | `rt::Party` | string | `Party` |
| `Timestamp` | `rt::Timestamp` | RFC 3339 string | `Timestamp` (micros) |
| `Date` | `rt::Date` | `YYYY-MM-DD` string | `Date` (days) |
| `ContractId t` | `rt::ContractId<T>` | string | `ContractId` |
| `List t` / `[t]` | `Vec<T>` | array | `List` |
| `Optional t` | `Option<T>` | `null` / value | `Optional` |
| `Optional (Optional t)` … | `Option<rt::NestedOpt<T>>` … | nested **list** form (`[]` / `[x]`) | `Optional` |
| `TextMap t` | `rt::TextMap<T>` (`BTreeMap<String, T>`) | object | `TextMap` |
| `GenMap k v` | `rt::GenMap<K, V>` | array of `[k, v]` pairs | `GenMap` |
| type variable `a` | type parameter `A` (upper-camel) | — | — |
| reference to a named type | `crate::<package>::<module>::<Type>`, or `::<other_crate>::<package>::<module>::<Type>` when the package is generated into a crate of its own | per that type | per that type |
| directly-recursive reference | `Box<…>` | transparent | transparent |

### Notes on the subtle ones

- **`Unit`** is the empty object `{}` in LF-JSON, **not** JSON `null` — so it maps
  to a dedicated `rt::Unit`, not Rust `()` (whose serde form is `null`).
- **Nested `Optional`.** The top-level `Optional` is `null`/value; every
  `Optional` **directly** inside another uses the LF-JSON *list* form so
  `Some None` (`[]`) stays distinct from `None` (`null`). Only a directly nested
  one: `Optional [Optional t]` is `Option<Vec<Option<T>>>`, because the list
  element boundary already separates the two. Codegen wraps each directly nested
  layer in `rt::NestedOpt`; the gRPC form is an ordinary proto `Optional`.
- **`Int64`** is LF-JSON-encoded **as a string** (`encodeInt64AsString`, to
  survive JavaScript's 53-bit precision); it maps to `rt::Int64` (a newtype over
  `i64`) which serialises to a string and accepts a string *or* a number on
  input, matching the Ledger API. Its gRPC form is a plain `Int64`.
- **`Numeric`** travels as a decimal string on both wires to avoid binary-float
  rounding; on JSON input `rt::Numeric` also accepts a number (high-precision
  values should still use the string form).
- **References** are always fully qualified. Within one crate that is
`crate::<package>::<module>::<Type>`; where a package is generated into a crate
of its own — which is how every `canton-splice-*` crate in this repository is
built, so that a `Holding` is one Rust type rather than one per crate — the
reference is absolute and names that crate instead:

```rust
pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
pub expires_at: ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
```

Which packages are external is the caller's choice, passed as an
`ExternalPackages` map; anything not in it is emitted into the crate being
generated,
  so cross-package/cross-module references resolve and names from different
  modules never collide. The package segment is the package **name**; the
  version is appended only to separate two packages that would otherwise share a
  module name, which is how a DAR bundling two versions under Smart Contract
  Upgrade keeps them distinct (see
  [docs/scu-regeneration.md](scu-regeneration.md)).
- **Recursion.** Daml permits a type to contain itself directly; Rust needs
  indirection, so a direct self-reference is wrapped in `Box`.

## Declarations

| Daml declaration | Generated Rust |
|---|---|
| record | `struct` with `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]` + `ToValue`/`FromValue`; fields snake-cased, `#[serde(rename)]` keeps the Daml label; a phantom type parameter (declared but unused) becomes a `PhantomData` field |
| variant | adjacently-tagged `enum` (`{"tag": <ctor>, "value": <payload>}`) + codecs; a **nullary** constructor carries `rt::Unit` so its wire form is `{"tag":<c>,"value":{}}` |
| enum | fieldless `enum` (also `Copy`); JSON form is the bare constructor-name string |
| template | payload `struct` + `impl rt::Contract` (on-ledger id) + `impl rt::Template` + a typed `impl rt::Choice<Self>` per choice; a keyed template also gets `impl rt::WithKey` |
| interface | a marker `struct` + `impl rt::Contract` + `impl rt::Interface` (its view type) + a typed `impl rt::Choice<Marker>` per interface choice |
| contract key | `type Key` on the template's `rt::WithKey` impl |

## Identity and commands

A template/interface exposes its on-ledger id through `rt::Contract`
(`PACKAGE_ID` / `PACKAGE_NAME` / `MODULE_NAME` / `ENTITY_NAME`).
`Contract::template_id()` returns the id in the **upgrade-friendly
`#<package-name>` form**, so the participant resolves the version vetted under
Smart Contract Upgrade rather than pinning a package-id hash. The runtime turns a
typed payload/choice into a Ledger API command:

- `rt::create_command(&payload)` → `CreateCommand`;
- `rt::exercise_command(&contract_id, &choice_arg)` → `ExerciseCommand`
  (works on `ContractId<Template>` and `ContractId<Interface>`);
- `rt::exercise_by_key_command::<T, _>(&key, &choice_arg)` → `ExerciseByKeyCommand`.

## Not represented

Term-level Daml (expression bodies of choices/values), and LF type shapes that
are not serialisable (`Forall`, structural records, type synonyms as such,
function/`Update`/`Any` types) are not modelled — codegen only needs the
serialisable type skeleton. A type that cannot be lowered is skipped and
surfaced as a `SkippedType` rather than emitted incorrectly.

## Known limitation: a nested `Optional` behind a type parameter

LF-JSON encodes an `Optional` as `null`/value unless its **immediate** argument
is itself an `Optional`, in which case that inner one takes the list form —
which is why `Optional (Optional t)` maps to `Option<NestedOpt<T>>`. The rule is
local: a container or record boundary in between already tells an inner `None`
from an outer one, so `Optional [Optional t]` is the ordinary
`Option<Vec<Option<T>>>` with `null` elements.

That rewrite is structural, so it cannot see through a type parameter: for
`data Wrap a = Wrap with value : Optional a` instantiated at
`Wrap (Optional Text)`, the nesting is real on the wire but absent from the
spelled-out Rust type. Nor is the nesting always visible at the instantiation
site: `data Outer a = Outer with inner : Wrap a` nests its own parameter too,
through `Wrap`, so the property is computed as a fixpoint over the reference
graph rather than one declaration at a time.

Rather than emit `Option<Option<String>>` — which would serialize the inner
layer as `null` where the ledger expects `[]` — codegen **refuses** that
instantiation and reports it as a skipped declaration. Encoding it correctly
requires the instantiation site to know how the target uses its parameter, plus
a distinct IR node for "this `Optional` is nested"; until then the case is
rejected rather than mis-encoded. The refusal is deliberately narrow — it fires
only where the two `Optional`s really do meet, never across a `List`, `TextMap`,
`GenMap` or a `ContractId`'s phantom tag. It does not occur anywhere in the
Splice corpus. The gRPC codec is unaffected — both layers are a proto `Optional`
there.

## Conformance against the official `daml-lf-archive` reader

The decode itself is machine-generated from the official `daml-lf-archive`
protobuf schema (vendored verbatim in `canton-lf/proto/`), and the hand-written
interpretation above it — LF version dispatch, interned-table resolution,
package-reference resolution — is verified against the authoritative JVM
implementation. The oracle test
([`canton-lf/tests/oracle.rs`](../crates/canton-lf/tests/oracle.rs)) and its
JVM twin ([`tools/lf-oracle/LfOracle.scala`](../tools/lf-oracle/LfOracle.scala),
which reads the same DAR through `com.daml:daml-lf-archive-reader` — the decoder
Canton itself uses) each render the DAR's full type-signature surface (every
package: serialisable data types, templates with keys/implements/choices,
interfaces with views) to one canonical JSON document, and the test asserts the
two documents are equal. The JVM reader also re-computes and checks each
archive's SHA-256, so agreement covers package-id integrity too.

Run it with a JVM and [scala-cli](https://scala-cli.virtuslab.org) on `PATH`:

```sh
CANTON_LF_ORACLE_DAR=/path/to/x.dar cargo test -p canton-lf --test oracle
```

**CI runs it** on the vendored fixture (`testdata/splice-api-token-holding-v1-1.0.0.dar`,
31 packages) in the `lf-conformance` job, which installs a JVM and scala-cli and
fails if the test skips instead of comparing — a conformance claim is worth what
a reviewer can re-run. Locally the same test has been run across the whole
Splice surface (splice-amulet, splice-wallet, splice-wallet-payments, the
token-standard API DARs, quickstart-licensing): 245 package decodings, zero
disagreements. Like the other DAR-dependent suites, the test skips cleanly when
the env var or toolchain is absent.
