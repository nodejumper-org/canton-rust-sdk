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

## Consequences

- Both changes ride the 1.0 release train, listed in its migration notes.
- Until then the current shapes stay: churning them at 0.x would break users
  twice for the same lesson.
