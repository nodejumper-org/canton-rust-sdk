# canton-token

Canton Network **token-standard** workflows: the registry client, choice
contexts, and the transfer and allocation flows. CIP-56 (V1) at the crate root
and CIP-0112 (V2) under `v2`, which keeps the same function names one module
down — `token::transfer` and `token::v2::transfer` are different standards, and
the path is what says which you meant.

Part of the [Canton Rust SDK](https://github.com/nodejumper-org/canton-rust-sdk).

## What this crate is, and is not

The token-standard **types** are generated from the Splice DARs and live in the
`canton-splice-api-token-*` crates. This crate does not re-declare any of them —
that would duplicate the code generator and go stale on the next Splice release.
The `Holding` named here is the same Rust type an application reads out of the
ledger.

What is missing from generated types is the *workflow*: resolving a factory,
asking a registry for a choice context, attaching the contracts it says to
disclose, and exercising the choice. That is this crate.

## Why a registry is involved

A token-standard choice is exercised through an interface, and only the registry
knows what reference data that choice needs — its own configuration contracts,
the rules for an instrument, the current round. So the client asks for a **choice
context** and gets back two things: the data to pass into the choice, and the
contracts the participant must be shown for it to resolve.

Those contracts are usually invisible to the submitting party. That is what
explicit disclosure is for, and attaching them correctly is most of what this
crate does.

The registry API is off-ledger HTTP, specified by the token standard's OpenAPI
documents — `token-metadata-v1`, `transfer-instruction-v1`, `allocation-v1`,
`allocation-instruction-v1`, and their V2 counterparts `transfer-instruction-v2`,
`allocation-v2`, `allocation-instruction-v2`. Every path and payload
here comes from them (vendored under `testdata/openapi/` at the Splice release the
tests pin), and
the method names follow their `operationId`s so the two can be read side by
side.

## Scan and the validator's scan-proxy

`canton_token::scan::ScanClient` is a typed client for Splice Scan, asked
directly (`ScanClient::new`, at `<scan>/api/scan`) or through a validator's
scan-proxy (`ScanClient::through_validator`, at `<validator>/api/validator`,
with a token). It returns the contracts an application discloses to use Amulet
(the `AmuletRules`, the open mining round, a provider's `FeaturedAppRight`)
ready to attach to a submission, and reads the DSO, a member's traffic, the
update history, the snapshots of the active contracts and the name service.
Paths and payloads come from Splice's `scan.yaml` and `scan-proxy.yaml`,
vendored under `testdata/openapi/`.

`RegistryClient` and `ScanClient` both take a bearer token (`with_token`,
`with_token_source`), never follow a redirect, bound the size of a reply and
percent-encode every path segment.

## The encoding that makes this work

The standard specifies that a factory's `choiceArguments` are encoded "using the
Daml JSON API, with the `extraArgs.context` and `extraArgs.meta` fields set to
the empty object" — and the context comes back in the same encoding. That is
exactly what `canton-daml` implements, so a generated choice type serializes
straight onto the wire and the returned context deserializes straight into the
generated `ChoiceContext`. No hand-written JSON anywhere in the path.

## Licence

Apache-2.0. See `LICENSE`.
