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
| `Int64` | `i64` | number | `Int64` |
| `Numeric n` | `rt::Numeric` | decimal **string** | `Numeric` (string) |
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
| reference to a named type | `crate::<package>::<module>::<Type>` | per that type | per that type |
| directly-recursive reference | `Box<…>` | transparent | transparent |

### Notes on the subtle ones

- **`Unit`** is the empty object `{}` in LF-JSON, **not** JSON `null` — so it maps
  to a dedicated `rt::Unit`, not Rust `()` (whose serde form is `null`).
- **Nested `Optional`.** The top-level `Optional` is `null`/value; every
  `Optional` nested inside another uses the LF-JSON *list* form so `Some None`
  (`[]`) stays distinct from `None` (`null`). Codegen wraps each nested layer in
  `rt::NestedOpt`; the gRPC form is an ordinary proto `Optional`.
- **`Numeric`** travels as a decimal string on both wires to avoid binary-float
  rounding.
- **References** are always fully qualified (`crate::<package>::<module>::<Type>`),
  so cross-package/cross-module references resolve and names from different
  modules never collide. The package segment is `<name>_<version>` (Smart
  Contract Upgrade keeps versions distinct).
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
surfaced as a `LowerError` rather than emitted incorrectly.
