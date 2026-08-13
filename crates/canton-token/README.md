# canton-token

Canton Network **token-standard** workflows (CIP-56): the registry client,
choice contexts, and the transfer and allocation flows.

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
`allocation-instruction-v1`. Every path and payload here comes from them, and
the method names follow their `operationId`s so the two can be read side by
side.

## The encoding that makes this work

The standard specifies that a factory's `choiceArguments` are encoded "using the
Daml JSON API, with the `extraArgs.context` and `extraArgs.meta` fields set to
the empty object" — and the context comes back in the same encoding. That is
exactly what `canton-daml` implements, so a generated choice type serializes
straight onto the wire and the returned context deserializes straight into the
generated `ChoiceContext`. No hand-written JSON anywhere in the path.

## Licence

Apache-2.0. See `LICENSE`.
