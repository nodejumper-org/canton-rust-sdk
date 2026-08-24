# ADR-0009: Daml-LF version-support policy for codegen

**Status:** Accepted (M2) · **Date:** 2026-07

## Context

The codegen's LF decoder (ADR-0008) is built against a **specific vendored LF
protobuf schema**, mirroring how the Ledger API protos are pinned to a Canton
release (ADR-0001). Three version numbers are currently in play and must not be
conflated:

- **Canton release** the SDK targets (M1 pins Canton **3.5.7** for the Ledger
  API protos).
- **Daml-LF schema** the codegen decodes (vendored from daml
  `v3.3.0-snapshot.20250507.0`, i.e. **LF 2.x**).
- **DAR SDK-version** — the `Sdk-Version` in a DAR's manifest (test corpus DAR
  is `3.3.0`).

These differ today (a DAR built with SDK 3.3 decodes fine while the Ledger API
is pinned to 3.5.7) because **LF 2.x is stable across this range** — the wire
schema does not change between these minor releases. But "it happens to work"
is not a policy, and a reviewer will ask what range is actually supported.

## Decision

- **Codegen supports the Daml-LF 2.x wire schema** as vendored, independent of
  the Canton release the *client* crates target. LF-version compatibility is a
  property of the LF major/minor, not of the Canton patch version.
- The vendored LF schema's provenance (daml ref + LF version) is recorded in
  `canton-lf` (as the Ledger API protos are in `canton-proto`), and re-vendoring
  to a newer LF is a deliberate, changelogged commit.
- The **supported DAR SDK-version range** is stated in the README compatibility
  table alongside the Canton/Ledger-API/MSRV row.
- The test corpus must include a **golden DAR from the pinned Canton line
  (3.5.x)**, not only the 3.3 DAR currently used, so decode is exercised against
  the same release the client targets. *(Open task — the 3.5.x golden DAR still
  needs to be sourced and added to the env-gated decode test.)*

## Consequences

- Clear, defensible answer to "which DARs does codegen accept": any LF 2.x DAR
  in the supported range, decoupled from the client's Canton pin.
- A single documented place (README table + `canton-lf` provenance) to update on
  each re-vendor, mirroring ADR-0001.
- One follow-up owed: add the 3.5.x golden DAR to the corpus.
