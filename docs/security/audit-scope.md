# Independent security review: scope

**Status: draft**, for agreement with the Canton Foundation's Tech & Ops
security subcommittee before the review begins. The proposal (#407, milestone
3) makes the review a pass-through item: the auditor and this scope are agreed
with the subcommittee, this document is published before the review starts,
and the report and a remediation summary are published here when it is done.
Nothing below is final until the subcommittee has agreed it; the section
"Open with the subcommittee" lists what still needs their word.

## What is reviewed

The commit under review is the `v0.3.0` tag of
[`nodejumper-org/canton-rust-sdk`](https://github.com/nodejumper-org/canton-rust-sdk)
(Apache-2.0). The proposal names three groups — the client, the codegen, and
the token crates — which are these crates:

| Group | Crates | Hand-written Rust, lines | What it does |
|---|---|---|---|
| Client | `canton-core`, `canton-auth`, `canton-ledger`, `canton-admin`, `canton-signer`, `canton-pqs`, `canton` | ≈ 10,500 | TLS and mTLS, OAuth client-credentials and token refresh, retry and error classification, the gRPC and JSON Ledger API clients, interactive submission and the `Signer` trait with the in-memory Ed25519 key, the PQS query compiler and Postgres client, the admin and topology reads |
| Codegen | `canton-lf`, `canton-codegen`, `canton-codegen-cli`, `canton-daml` | ≈ 7,300 | the Daml-LF archive decoder, the lowering and emitter that turn a DAR into a Rust crate, the CLI, and the runtime the generated code links against (JSON and gRPC codecs, commands) |
| Token | `canton-token` | ≈ 2,100 | the token-standard registry client, choice contexts and disclosure, the V1 and V2 workflows |

Lines are source without comments and blank lines, `src/` only. The fourteen
`canton-splice-api-*` crates, `canton-daml-stdlib`, `canton-splice-amulet`,
`canton-splice-wallet` and `canton-splice-wallet-payments` are generated
output of `canton-codegen` from Splice's DARs, reproduced byte for byte by
CI's `bindings-drift` job; they are covered by reviewing the generator, not
by reading them.

## Threat model

The SDK runs inside an application that holds credentials and, with
interactive submission, may hold or reach a signing key. What an attacker can
control, and what the SDK must guarantee in each case:

| The attacker controls | The SDK must |
|---|---|
| Responses from a participant (gRPC or JSON): a compromised or impersonated node, a proxy | not panic, not loop, not allocate without bound on a hostile payload; classify errors so that a permanent failure is never retried forever; never let a response redirect a credential elsewhere |
| Responses from a token-standard registry (an HTTP service, possibly a third party's) | attach only the disclosed contracts the standard's context names, to the command the caller built; never let registry data change which factory, party or amount is exercised beyond what the standard specifies; enforce TLS verification, with certificate failures permanent |
| A DAR fed to the code generator (from a package repository, a participant, or a colleague) | decode it without panic or unbounded memory (the decompression caps and the fuzzed decoder); generate code that cannot execute anything at build time beyond compiling; not let a package name or identifier become code injection in the emitted crate |
| Rows in a PQS store, or the store's connection | bind every predicate as a parameter, never interpolate; require TLS when asked for it; decode a hostile row into an error, not a panic |
| The process's logs and `Debug` output | never print a bearer token, an OAuth secret, a private key, or a mutual-TLS key (the `Debug` implementations are hand-written for this and tested) |
| Time and the network (a lost response, a retried request) | never submit a command twice for one intent: change-id de-duplication on retry, and a retried execute of a signed transaction recognised as its own duplicate |

Interactive submission deserves its own line: the participant never holds the
key, so the guarantee is that what the `Signer` is asked to sign is exactly
the hash the participant computed for the transaction the caller built, and
that a signature is bound to the party and the hashing scheme it was made
under.

## What the auditor is asked for

1. A review of the crates above against the threat model: memory safety at
   the boundaries (`#![forbid(unsafe_code)]` holds across the workspace, so
   the question is logic, not UB), credential handling, TLS configuration,
   the retry and de-duplication logic, the code generator as a parser of
   untrusted input, and the token-standard workflows' handling of registry
   data.
2. Findings rated critical / high / medium / low / informational, each with
   a reproduction.
3. Critical and high findings are remediated before the report is published;
   medium and low are remediated or accepted with a written rationale.
4. A remediation summary, published in this directory under Apache-2.0 with
   the report, and linked from the milestone issue.

## Out of scope

- Canton, Splice, Scribe, Keycloak and the networks themselves.
- The generated binding crates as text (see above).
- The example programs and the test suites, except where a test double
  shapes production behaviour.
- Denial of service against a participant by a client that is itself
  compromised; the SDK is not a rate limiter.
- Cryptographic primitives: `ring` and `rustls` are used, not implemented.

## Inputs the auditor gets

- The repository at the tag, with CI green on Linux, macOS and Windows.
- This document, the compatibility matrix, the Ledger Client Standard map,
  and the verification records under [`docs/verification/`](../verification/).
- The previous independent review of milestone 1 by Equilibrium (posted on
  the milestone-1 issue), its regression branch, and the fixes, so that
  ground is not covered twice.
- A LocalNet recipe (the README's testing section) and, on request, a party
  on a DevNet validator.

## Open with the subcommittee

- The auditor. <status: e.g. a scope-based estimate has been requested from
  CertiK; alternatives welcome>
- The budget, pass-through and separate from the base grant, as the proposal
  sets out.
- Whether the review starts before or after the milestone-3 vote, given the
  hard deadline of 6 months from grant approval.
- Anything the subcommittee wants added to the threat model.
