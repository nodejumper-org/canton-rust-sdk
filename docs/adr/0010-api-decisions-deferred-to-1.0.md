# ADR-0010: API changes deliberately deferred to 1.0

## Status

Accepted (2026-08-24). Nothing here changes before the 1.0 release; all of it
changes *at* 1.0 unless re-argued.

## Context

An audit of the public surface against the Rust API Guidelines found two
deviations whose fix is a breaking change. Pre-1.0 the family is at 0.x, where
breaking is allowed but still costs every early adopter a migration. Neither
deviation is dangerous enough to charge that cost now; both are wrong enough
that 1.0 — the last cheap moment — should pay it. Recording the decision is the
point: an unrecorded "later" is how a wart becomes permanent.

## Decisions

**1. `PartyParseError` / `NumericParseError` become `ParsePartyError` /
`ParseNumericError` (C-WORD-ORDER).** The std convention is verb-object-error
(`ParseIntError`); ours inverts it, and it is the exact example the guideline
names. At 1.0 the types are renamed with deprecated type aliases kept for one
minor release.

**2. `Party` loses its unvalidated `From<&str>` / `From<String>` impls.** Today
the most idiomatic spelling (`"x".into()`) routes to the *unchecked* wire-path
constructor while `"x".parse()` validates — the two spellings a reader expects
to be equivalent differ in exactly the property that matters. The impls also
block adding a validating `TryFrom<&str>`: the std blanket
`impl<T, U: From<T>> TryFrom<T> for U` makes `TryFrom` and `From` mutually
exclusive for the same pair of types, so the checked conversion cannot even be
offered while the unchecked one holds the slot. At 1.0 the `From` impls go,
`TryFrom<&str>` arrives with `PartyParseError`, and `Party::new` remains the
documented unchecked door for wire values.

**3. `TlsConfig::client_identity_pem` becomes a private field behind a typed
accessor.** Today it is a `pub` field of type `Option<(Vec<u8>, Vec<u8>)>` — the
mutual-TLS certificate and **private key** as an untyped pair, with "which
element is the key" an undocumented invariant. The `#[non_exhaustive]` attribute
and the `with_client_identity` builder already steer callers to the safe path,
and `Debug` redacts the key, so nothing leaks today; but a `pub` field carrying
key material, tuple-ordered, is a shape 1.0 should not freeze. At 1.0 the field
goes private and the identity is read through a small typed accessor. Deferred
rather than done now for the same reason as the others: it is a breaking change,
and 0.x users should migrate once.

**4. Update reads return the SDK's own types; the `prost` ones become the
escape hatch.** ADR-0002 promised that raw proto types would leave the default
public path, and for update reads they had not: `updates_resumable` yields
`pb::get_updates_response::Update` and the JSON client returns raw JSON, so
code written against one transport did not read the other, and a re-vendored
proto reached callers who matched on it without a version signal
(nodejumper-org/canton-rust-sdk#11). The types now exist and are additive:
[`canton_ledger::update`](../../crates/canton-ledger/src/update.rs) (`Update`,
`Transaction`, `Event` with created, archived and exercised events,
`Reassignment`, `TopologyTransaction`, `OffsetCheckpoint`), built from either
transport, with payloads decoded by the generated type and the wire form still
reachable. `updates_typed_with` and `updates_resumable_typed_with` (gRPC) and
`updates_typed_with` (JSON) return them, and `Update::from_grpc`,
`from_grpc_point` and `from_json` convert anything else. At 1.0 the typed reads
take the plain names (`updates`, `updates_with`, `updates_resumable`, the point
reads) and the proto- and JSON-returning ones move to `*_raw`, so the default
path is transport-neutral and the wire types are something a caller opts into.
Deferred for the reason the others are: renaming now would break every 0.x
reader of updates, and they should migrate once.

## Consequences

- These changes ride the 1.0 release train, listed in its migration notes.
- Until then the current shapes stay: churning them at 0.x would break users
  twice for the same lesson.
