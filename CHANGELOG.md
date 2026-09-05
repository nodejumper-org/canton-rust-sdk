# Changelog

All notable changes to the Canton Rust SDK are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generated protobuf types (the `canton-proto` crate and the `proto` re-exports)
are **exempt from SemVer** — see the stability policy in `canton-proto`'s docs.

## [0.3.0] — unreleased

### Changed — one crate per Daml package (**breaking**)

- The eight token-standard and featured-app packages are now crates of their
  own: `canton-splice-api-token-metadata-v1`, `-holding-v1`, `-allocation-v1`,
  `-allocation-instruction-v1`, `-allocation-request-v1`, `-burn-mint-v1`,
  `-transfer-instruction-v1` and `canton-splice-api-featured-app-v1`.
  `canton-splice-amulet`, `-wallet`, `-wallet-payments` and
  `canton-quickstart-licensing` reference them instead of carrying copies.
- **Why it is breaking:** those packages previously existed once inside *each*
  crate that depended on them, and Rust treats the copies as unrelated types.
  A `ContractId<Holding>` read through `canton-splice-amulet` did not typecheck
  against the `Holding` of `canton-splice-wallet`, so no program could use both
  crates together. They are now one type.
- **Migration:** a path that went through the containing crate now goes through
  the package's own crate — `canton_splice_amulet::splice_api_token_holding_v1::…`
  becomes `canton_splice_api_token_holding_v1::splice_api_token_holding_v1::…`,
  and the new crate is added to `Cargo.toml`. Nothing changes on the ledger:
  the package ids, template ids and wire encodings are identical.
- `splice-util` is unchanged — it ships no DAR of its own, so it stays inside
  `canton-splice-amulet`, and `canton-splice-wallet` reaches it from there.
- **`canton-daml-stdlib`** (new) owns the Daml standard library — `daml-stdlib`,
  `daml-prim`, `ghc-stdlib` and their per-module packages — and every bindings
  crate references it. The same defect applied to those: `RelTime` appears in
  public field types (`HoldingView`'s lock expiry, `AnyValue::AV_RelTime`), and
  each crate declared its own. The standard library ships no DAR, so this crate
  is generated from a *selection* of packages out of the DAR committed to this
  repository — which means its drift guard needs no external checkout.
- Every crate shrank by the standard library it no longer carries:
  `canton-splice-api-token-metadata-v1` 134 KB → 12 KB,
  `canton-splice-wallet` 310 KB → 189 KB, `canton-splice-amulet` 556 KB → 435 KB.

### Added — interactive submission with a pluggable signer

- **`canton-signer`** (new) — `Signer`, an object-safe async trait for signing a
  prepared transaction's hash, so an HSM or KMS fits behind it. `Ed25519Key` is
  the in-memory implementation, on `ring`, behind a default `ed25519` feature an
  HSM implementer can turn off.
- A key is not yet an identity: Canton addresses a key by a fingerprint it
  computes itself, so `Ed25519Key` cannot sign for the ledger until
  `into_signer(fingerprint)` gives it one. The ordering the types enforce is the
  real one.
- **`canton-ledger`** — `prepare_submission`, `execute_submission`,
  `execute_submission_and_wait`, `execute_submission_and_wait_for_transaction`,
  with the flow as a type per stage: `Prepare` → `Prepared` → `Executable`.
  Nothing unsigned can be executed, because there is no such value. Signing
  refuses a party that is not acting, and `unsigned_parties` answers "who is
  still missing" without a round trip.
- **`canton-ledger`** — `connected_synchronizers`, so "which synchronizer?" is a
  question the participant answers rather than configuration.
- **`canton-admin`** — `generate_external_party_topology` and
  `allocate_external_party`: onboarding a party whose key the participant does
  not hold, which is two calls with a signature over the onboarding multi-hash
  in between.
- **Verified live** on a Canton 3.5.7 participant: an external party is
  onboarded by signing its own topology, a command is prepared, signed off the
  participant and committed, and a signature from the wrong key is refused —
  `Received 0 valid signatures from distinct keys (1 invalid)`. That last one is
  the control: it is what shows the signature is carrying the authorization.

### Added — token standard (CIP-56)

- The `canton` facade re-exports the three new crates as `canton::signer`,
  `canton::token` and `canton::pqs`. Without that, `cargo add canton` — which
  the README describes as getting "everything below as one version-locked set" —
  delivered none of this milestone.
- **`canton-token`** (new) — the *workflow* over the generated token-standard
  types, which it does not re-declare: `RegistryClient` for the off-ledger API,
  choice contexts with their disclosures, `TransferFactory_Transfer`, and the
  allocate path with execute / withdraw / cancel.
- Every path and payload comes from the standard's OpenAPI documents. Two are
  vendored in cn-quickstart; the other two were taken from the pinned upstream
  commit those copies name.
- The standard specifies `choiceArguments` as the choice "encoded using the Daml
  JSON API, with `extraArgs.context` and `extraArgs.meta` set to the empty
  object", and returns the context the same way — which is what `canton-daml`
  implements. So a generated choice serializes straight onto the wire and the
  reply deserializes straight into the generated `ChoiceContext`.
- `TokenCommand` keeps a command and its disclosures together and converts into
  either an ordinary or an *interactive* submission, so a token transfer can be
  signed by a party whose key the participant does not hold.
- Each choice on an allocation fetches its own context: the standard says a
  context may be specific to the choice, so sharing one is a bug that works
  until a registry starts distinguishing them.
- The **V2 workflow** lives in `canton_token::v2`, keeping the same function
  names one module down: `token::transfer` and `token::v2::transfer` are
  different standards, and the path is what says which you meant.

### Added — conformance to the Ledger Client Standard

- **`canton-conformance`** (new, not published) — one test per capability of the
  standard, named for the row it answers, so a reviewer can read the two side by
  side. `conformance/capabilities.toml` is the machine-readable checklist,
  derived from the capability matrix DA published.
- A completeness guard asserts the two agree **in both directions**: no
  capability claimed without a test, and no test claiming a capability the
  registry does not list. Checked against the suite's source rather than a run
  of it, so an ignored test does not count as coverage.
- The suite exercises the SDK **through the `canton` facade**, which is how it
  found that `canton::telemetry` was not re-exported — the metric names and
  transport labels an application builds a dashboard from were unreachable from
  `cargo add canton`. They are exported now.
- One row is honest about its limits rather than quietly weaker: **contract
  keys** are generated and exercisable by key, but no template in this corpus
  declares a key, so the test asserts the mechanism is present and says so.
- [`docs/compatibility-matrix.md`](docs/compatibility-matrix.md) — toolchain,
  platform, Canton release, Daml-LF minors, token-standard version, and a table
  of what CI checks against what needs a live node.
- A `conformance` CI job runs the suite and asserts the count matches the
  registry **exactly**, with nothing ignored. `>=` across every binary in the
  package left slack equal to the number of guard tests, and an `#[ignore]`d
  capability could hide in it — the name-based guard reads the suite's source,
  so it counts an ignored test as coverage.

### Fixed — a full M3 review, and what it turned up

Two independent reviews of the whole milestone: one against the proposal, one
hunting bugs and checking whether the problem classes found in M1 and M2 had
recurred. Every finding below was verified against the code before being acted
on, and two of the reviews' own claims did not survive that check.

**Retriability, which M1 got wrong in two places and M3 got wrong in two more.**

- `canton-pqs` classified *every* Postgres SQLSTATE as `InvalidRequest`, which
  is not retriable. A failover (`57P01`), a serialization failure (`40001`), a
  deadlock, `53300 too_many_connections` — all reported to the caller as "PQS
  rejected the query", pointing whoever read it at a predicate that was never
  wrong, and making an application give up on a five-second restart. Now
  classified by SQLSTATE *class*: `08`, `40`, `53`, `55` and `57` are transient,
  everything else is the caller's. `classify` had no test at all; it has three.
- `canton-token`'s registry client turned every transport failure into a
  retriable `Error::Connection` carrying only reqwest's outer sentence. A
  certificate the client cannot verify was retried forever, and the word
  "certificate" — which lives in the source chain — never reached the operator.
  The chain is now walked, and a certificate failure, a malformed URL and a
  redirect loop are reported as non-retriable.

**Two integrity checks that did not check.**

- `canton-admin::get_package` compared the participant's *own* `hash` field
  against the id that was asked for, and skipped the comparison entirely when
  that field was empty — while its doc claimed "a package id **is** the hash of
  its payload, so that check is what makes asking by id pin the content". It now
  hashes the payload.
- The committed V2 payload corpus is documented as needing no checksum file
  "because each file name ends with the id it hashes to". Nothing hashed them.
  The shared generation table now does, and rejects a name that does not carry a
  64-hex id — `rsplit('-').next()` returned the whole filename when there was no
  `-`, so `foo.lfpayload` yielded the package id `foo` and the error written for
  that shape was unreachable.

**A guard that was absent from the configuration it was written for.** The
Ed25519 signature-length check in `canton-signer` sat inside
`#[cfg(feature = "ed25519")]`, and the crate's docs tell HSM/KMS callers to
build with `default-features = false`. The length of an Ed25519 signature is a
property of the algorithm, not of the in-memory implementation, so the check is
now unconditional — and CI runs the tests with default features off, which the
feature-powerset job could not (it passes `--no-dev-deps`, so tests are never
compiled under a feature combination).

**A query that silently returned less than it should.** `Predicate::not_eq`
rendered as `<>`, which yields SQL NULL against a missing JSON path and drops
the row. Since a query matches on the package *name* so it survives a Smart
Contract Upgrade, a result set normally mixes payloads of different versions
with different field sets — so adding a field in v2 would have quietly dropped
every v1 contract from a `not_eq` on it. Now `IS DISTINCT FROM`. The ordered
comparisons keep NULL semantics, which is right, and now say so.

**`instrument()` reported a misconfigured base URL as an answer.** Any 404
became `Ok(None)` — "this registry does not issue it" — so a base URL one path
component off made a wallet's polling call return a steady, quiet `None` while
every other call on the same client failed loudly. A 404 whose body is not JSON
did not come from a registry handler and is now an error that says so.

**The conformance registry claimed less than the SDK does, and nothing could
tell.** `conformance/capabilities.toml` listed 39 rows; the Ledger Client
Standard has 49 in scope. The two existing guards compare the registry to the
suite, so a row missing from *both* was invisible to them — and ten were,
including **Explicit disclosure**, an M3 row and a named sub-item of the
token-standard deliverable. Meanwhile the CI job derived its threshold from that
same short file, so the gate grew easier as the registry shrank.

- The ten rows are added, each with a test: explicit disclosure, interface
  subscriptions, node health, gRPC package management, vetting, the three
  topology reads, and user self-inspect.
- One of them is **not** added as a claim. "JSON package mgmt" is an M1 row this
  SDK does not implement — `JsonClient` covers version, ledger end, commands,
  ACS and updates, and has no package endpoint — so it is recorded as a
  `[[gap]]` with a reason. A capability nobody can name a test for is not one
  this SDK has, and writing a test that named it anyway is precisely the fake
  coverage the mechanism exists to prevent.
- A third guard asserts capabilities + gaps account for every in-scope row, so
  dropping one now fails rather than passing quietly. Removing "Explicit
  disclosure" was used to check the guard actually fires.
- The CI counter requires an exact match with **zero ignored**, and counts
  `[[capability]]` blocks rather than `id` lines — gaps carry an `id` too.

**Reachability through the facade.** `canton-pqs`'s `tls` feature was not
forwarded, so `PqsClient::connect_tls` did not exist for a `cargo add canton`
user; reaching a PQS behind TLS meant a second, separately-versioned dependency
— the version skew the facade exists to prevent. Forwarded as `pqs-tls`, and the
conformance suite now names the method, which is the file that states the rule.

**The registry stub did not check the HTTP verb.** It recorded path and body
only, and answered `200` to any request line, so a factory issued as a `GET`
passed every assertion — `reqwest` sends the body either way. The method is
recorded and asserted.

**Documentation that claimed more than the code did.** The compatibility matrix
said bindings drift covers "all nineteen" generated crates (CI covers eighteen;
`canton-quickstart-licensing` builds its DAR from source and is guarded
locally), and opened by saying every row is exercised in CI while its own table
lists four that are not. The capability registry cited `canton-kernel` and
`canton-daml-runtime`, neither of which exists — they are `canton-core` and
`canton-daml`. The facade's landing-page table omitted all three M3 crates and
still described the token crate as CIP-56 only. Three intra-doc links were
broken, two of them predating this work.

**Reviewer claims that did not survive checking**, recorded because a review is
evidence, not a verdict: the count of missing capability rows was first reported
against a 49-row total I could not reproduce until I found that three sections
of the map use a different column order; and "all ten are implemented" was wrong
for JSON package management, which is why that one became a gap rather than a
test.

### Verified — both token standards, against a live registry

The registry half of `canton-token` had been exercised only by
`tests/inprocess.rs`, because the environment was recorded as having no
registry. That was wrong: a LocalNet's registry is the **scan**, which the
super-validator runs, and cn-quickstart runs one under `SV_PROFILE=on` on port
5012 — it simply does not publish that port to the host.

Against it, with the Amulet instrument declaring both standards
(`splice-api-token-transfer-instruction-v1` **and** `-v2`):

- **A V1 transfer settles end to end** — factory resolved against the registry,
  submitted with the four contracts it named for disclosure, committed at offset
  39643.
- **A V2 `Account`-based transfer against Amulet on a LocalNet, exercised as a
  V2 implementation** — committed at offset 39646. The proposal's verification
  clause names the V2 *reference token*; that network is retired (see the
  compatibility matrix), so this is the closest available target, and the
  first time the `/v2/` paths, the `Account` model and the `actors` field have
  met a real registry rather than a transcription of its specification.
- **V2 event parsing on a committed transaction** — `events::holdings_changes`
  read back one holdings change: one holding spent, two produced, two transfer
  legs. The interface is matched on its qualified name rather than its package
  id, and this is the first evidence that a *real* registry's event satisfies
  that match.
- `tests/live.rs` (new, env-gated on `CANTON_TOKEN_REGISTRY_URL`) covers the
  registry read path: the admin party parses, instruments decode with the
  optional fields genuinely absent, a not-issued instrument is `None` rather
  than an error, page tokens round-trip, and the declared API versions are
  readable. As with the other live suites, a set variable that cannot be reached
  **fails rather than skips**.

**What the first real submission found.** Both examples passed an empty
`inputHoldingCids`, with a comment that a registry may select holdings itself.
Splice's reference registry does not: the transfer reached the Daml interpreter
and failed with `At least one holding must be provided`. An end-to-end example
that cannot complete against the reference implementation is not end to end, so
both examples now take `CANTON_TOKEN_HOLDINGS` and say why it is not optional in
practice. The V2 example also reads its own committed transaction back through
`holdings_changes`, which is where a reader would look for it.

### Fixed — a three-milestone review, and the fixes that had siblings

Three reviews: M1+M2 against their proposal text, M3 exhaustively, and a sweep
for fixes applied in one place while the same pattern survived elsewhere. The
third was the one worth running.

**The cause chain, now in one place.** The previous round fixed how the registry
client reports a transport failure. Four other sites had the same two defects —
a message that dropped everything under the outer error, and a verdict of
retriable `Error::Connection` for conditions no amount of waiting fixes. Two of
them, `canton-ledger`'s JSON client and `canton-auth`'s token fetch, sit *inside*
`run_with_retry`, which the registry client did not: an IdP or participant behind
a certificate the client cannot verify ran the full retry schedule and then
reported `error sending request for url (…)`, with the word *certificate* one
level down in a chain nobody read. `canton_core::chain` is now the single walker
— three crates had each grown a private copy before anyone noticed the fourth had
none — and each site classifies for itself, because what is permanent differs:
a bad token endpoint is `Auth`, a bad JSON body is `UnexpectedResponse`.

**`PqsClient::connect` had the bug the same file's `classify` had just lost.**
Fixed by the SQLSTATE class rule already written twelve lines below it. Verified
against the running store: a wrong password is now
`invalid request: … the store refused the connection (28P01): … FATAL: password
authentication failed`, non-retriable, instead of a retriable `db error`.

**The token deliverable could not be *called* through the facade.** Every entry
point takes a generated type — `transfer` takes a `Transfer` — and the facade
depends on no `canton-splice-*` crate, so a `cargo add canton` user could reach
the functions and not name their arguments. `canton_token::types` re-exports the
modules (`types::v1::transfer_instruction`, `types::v2::holding`, `types::metadata`
…), whole modules rather than a hand-listed set that goes stale on the next
record the standard adds. This is the `pqs-tls` defect one level up, and larger:
that one hid a feature, this one hid the milestone.

**External-party onboarding now checks the one relation it can.** A party id is
`<hint>::<namespace>`, and for an external party the namespace *is* the
fingerprint of the key being registered — so a response assembled for a
different key is now refused before anything signs it. The multi-hash still
cannot be recomputed (that needs Canton's hash-purpose scheme, which this
workspace does not implement); this is the free check that was missing beside it.

**Half-applied fixes from the previous round, completed:**

- The HTTP-verb assertion reached the six POST endpoints and none of the GET
  ones — in the file whose own new comment names both.
- `CANTON_TOKEN_HOLDINGS` was added to both examples with a doc that still
  carried the pre-fix reassurance, contradicting the call-site comment eight
  lines below it. The helper now warns when the variable is unset rather than
  defaulting quietly into a failure that arrives from the Daml interpreter.
- `pqs-tls` reached the manifest and the module table but not the facade's own
  canonical feature list, which reads as complete.
- `get_package`'s doc still described the check the hash fix removed — that the
  SDK trusts the server's `hash` field.

**Documentation that claimed more than the code did.** The published matrix
called a run "against the V2 reference token" six lines after naming the
instrument as Amulet; it now says what it is — a V2 implementation exercised as
one — and points at what the clause still needs. `docs/daml-lf-type-mapping.md`,
the M2 deliverable document, said references are "always"
`crate::<package>::<module>::<Type>`; every one of the nineteen generated crates
in this repository disproves it, and the cross-crate form is now documented with
the emitted code beside it. ADR-0005 claimed the bindings crates encode the DAR
version in their version metadata (they do not). The CHANGELOG said thirteen
generated crates in the same entry where it said nineteen. Two CI comments
described a four-guard shape that no longer exists, and `canton-quickstart-licensing`
pointed at the wrong crate for its own drift guard.

**The CI step gating the conformance count did not check that it ran.**
`cargo test` exits 0 on "0 passed", so deleting every completeness guard would
have turned green the step that everything downstream depends on. Its sibling
steps already assert their own markers.

### Added — the V2 token standard (CIP-0112)

- Six new bindings crates: `canton-splice-api-token-holding-v2` (which carries
  the **`Account`** model), `-transfer-instruction-v2`, `-transfer-events-v2`
  (the `EventLog` a V2 transfer is parsed from), `-allocation-v2` (with
  `SettlementFactory`, `Allocation_Settle` and `FinalizedAllocation` — executor
  settlement), `-allocation-instruction-v2` and `-allocation-request-v2`.
- V2 reuses `splice-api-token-metadata-v1`: there is no `metadata-v2`.
- **Where the packages came from.** V2 ships as no DAR anyone publishes —
  cn-quickstart carries only the V1 set, the Splice repository holds Daml
  sources rather than built artefacts, and there are no release assets. It is
  live on the network, so it was taken from a participant through the Ledger
  API's `GetPackage` and committed under `testdata/token-standard-v2`. A package
  id is the SHA-256 of its own bytes and each file name ends with that id, so
  the corpus is pinned by construction and needs no checksum file.
- The V2 crates reference `canton-daml-stdlib` and the metadata crate rather
  than copying them, exactly as the V1 crates do — the `ghc-stdlib` and
  `daml-stdlib` packages they depend on carry the same ids the V1 corpus uses,
  which is why one stdlib crate serves both.
- The drift guard covers all nineteen generated crates, and the seven generated
  from committed packages are guarded in CI with no external checkout.
- **`canton_token::v2`** — the workflow over those types: transfer with
  accept/reject/withdraw, allocate, `settle_batch`, and the allocation and
  allocation-instruction choices. Four differences from V1 a caller meets at
  once, each of which the module documents: a transfer moves between
  **accounts** rather than parties (and both the owner and the provider of an
  account are optional); every choice names its **actors**, which V1 left
  implicit in the submitting party; a V2 allocation names the settlement it
  belongs to when it is *created*; and settlement moved to a **batch** on the
  settlement factory, which is what lets both legs of a delivery-versus-payment
  settle together. Asking for a V1 `execute-transfer` context is refused before
  it reaches the network, naming the settlement factory as its replacement —
  a registry would answer 404 with nothing that says why.
- The V2 paths were taken from the OpenAPI documents rather than derived from
  V1: the collections are singular (`transfer-instruction`,
  `allocation-instruction`, `allocation`) *except* the choice contexts on an
  allocation, which are under the plural `allocations`. A test pins each one,
  because a client that regularises the odd one out gets a 404 from a registry
  that is working correctly.
- **`canton_token::v2::events`** — `holdings_changes` reads what actually moved
  off a transaction. A V2 registry records it by exercising
  `EventLog_HoldingsChange` on the `EventLog` interface, and that is the only
  complete answer: the creates and archives show holdings appearing and
  disappearing without saying which transfer they belonged to. Events are
  matched on the interface's **qualified name**, not its package id — pinning
  the id would make a client stop seeing events the day a network upgraded the
  standard, and that failure reads as "nothing moved" rather than as an error.
  The choice name alone is not enough either: it is not reserved. An argument
  that does not decode is reported rather than skipped, for the same reason.
- `examples/v2_transfer.rs` is the V1 example's counterpart, written to be read
  beside it, and `examples/v2_allocate.rs` is the allocation flow — the half
  that needs three parties rather than two. Run against the live registry with
  app-provider as sender, app-user as receiver and the super-validator as
  executor: allocated at offset 40176, six events. It stops before settling on
  purpose; that is the executor's move from their own participant, and one
  process holding both sides would prove nothing about a pattern whose whole
  point is that the principals differ.
- The allocation example is also the first thing written entirely against
  `canton_token::types` — it names `SettlementInfo`, `AllocationSpecification`,
  `TransferLegSide` and `Account` with no direct dependency on any
  `canton-splice-*` crate, which is what the re-export was for.

### Added — reading packages from a participant

- **The same reads over JSON.** `JsonClient::list_packages` (`GET
  /v2/packages`) and `JsonClient::package_status` (`GET
  /v2/packages/{package-id}/status`), returning the gRPC path's
  `PackageStatus` so a caller switches transports without re-learning the
  vocabulary. Asked for in [issue #2](https://github.com/nodejumper-org/canton-rust-sdk/issues/2)
  by a JSON-only deployment; the conformance registry's one declared gap
  (`packages__json_package_mgmt`) becomes a capability with it. DAR upload
  (`POST /v2/dars`) is deliberately not included — an operator write with its
  own authorization story, tracked separately.

- **`canton-admin`** — `get_package` downloads a package's `ArchivePayload`
  bytes and checks the hash the participant returns against the id that was
  asked for. With `list_packages`, that is enough to generate bindings from
  what a network has actually vetted rather than from a file that has to be
  found — and for a package that exists only on a network, there may be no file.
- **`canton-lf`** — `decode_payload` reads that shape. A DAR entry is a whole
  `Archive` (payload, hash and hash function together) and `decode_package`
  reads *that*; the Ledger API returns the three as separate fields. Mistaking
  one for the other fails with a protobuf error about a `hash` field that says
  nothing about the cause.
- **`canton-codegen`** — `lower_packages_selecting` generates from decoded
  packages rather than only from a DAR.

### Added — the Participant Query Store

- **`canton-pqs`** (new) — a typed read client for PQS/Scribe. A query names its
  template by *type*: the qname is `PACKAGE_NAME:MODULE_NAME:ENTITY_NAME`, read
  off the generated `Contract`, and it is the package **name**, so a query
  survives an upgrade instead of pinning one build.
- PQS stores payloads in the Daml JSON encoding, which `canton-daml` implements,
  so a contract read from Postgres deserializes into the same generated type a
  transaction stream yields. Two ways in, one set of types.
- Predicates compile to parameterized statements. Every value is a parameter and
  so is every JSON field path — bound as `text[]` and applied with `#>` — so the
  statement text depends on a query's shape and never on its data.
- Ordered comparisons on numbers are numeric: LF-JSON carries `Int64` and
  `Numeric` as strings, so comparing lexically would sort `"9"` after `"10"`.
- `tls` feature for `connect_tls`; off by default, since PQS is usually inside a
  trust boundary.
- **Verified live** against a running Scribe 3.5.4 store: 969 active contracts
  read as typed payloads, payload and party-column predicates filtering in the
  database, containment, lookup by id, and a pinned-offset read.

### Added — codegen

- `Selection` and `lower_dar_selecting`: generate a crate from part of a DAR.
  A reference that leaves the selection is reported as a skipped type rather
  than emitted, since the path would name a module the crate does not have —
  it would compile where it is generated and fail in the consumer.
- `Selection::and_prefixed` and `ExternalPackages::with_prefixed` take package
  **name prefixes**, for a family that arrives as many packages. The standard
  library is some thirty of them, and which ones a DAR carries depends on what
  its Daml source touched, so an exact list is right for one DAR and wrong for
  the next. A prefix matches at a `-` boundary: `daml-prim` does not also match
  a `daml-primary`.

> **Publish order — this replaces 0.2.0's, which no longer works.**
> `canton-ledger` and `canton-admin` now depend on `canton-signer`, so it has to
> go out before them rather than not at all:
>
> `canton-proto` → `canton-core` → `canton-auth` → `canton-lf` → `canton-daml`
> → **`canton-signer`** → `canton-ledger` → `canton-admin` → `canton-codegen` →
> `canton-codegen-cli` → **`canton-pqs`**
> → `canton-daml-stdlib` (every binding references it)
> → `canton-splice-api-token-metadata-v1`, `canton-splice-api-featured-app-v1`
> → `-holding-v1` → `-allocation-v1` → `-allocation-instruction-v1`,
> `-allocation-request-v1`, `-burn-mint-v1`, `-transfer-instruction-v1`
> → `canton-splice-api-token-holding-v2` → `-transfer-instruction-v2`,
> `-transfer-events-v2`, `-allocation-v2` → `-allocation-instruction-v2`,
> `-allocation-request-v2`
> → `canton-splice-amulet` → `canton-splice-wallet-payments` →
> `canton-splice-wallet` → **`canton-token`** → `canton` (last: the facade
> re-exports everything above).
>
> `canton-sample` and `canton-quickstart-licensing` stay unpublished.

### Changed — CI

- The packaging check derives its crate list from `cargo metadata` rather than
  a hand-written one, so a crate added to the workspace cannot be left out of
  it — the previous list silently stopped covering the eight crates above.
- The bindings drift guard checks all nineteen generated crates (it checked
  three), reading the crate/DAR/external-package table from the same file the
  regeneration example uses so the two cannot disagree. `canton-daml-stdlib` is
  generated from the DAR committed here, so that one is guarded in CI with no
  checkout at all.

## [0.2.3] — 2026-08-25

### Fixed

- **A gRPC stream or submit did not retry a connection that died in flight.**
  tonic surfaces a dropped connection mid-call as a `Status` with code
  `Unknown` (message "transport error" / "h2 protocol error"), not as a
  `transport::Error` — that variant is only for connection *establishment*. So
  `Error::is_retriable()` treated the most ordinary failure there is as
  terminal: `updates_resumable` gave up on the first blip and `run_with_retry`
  never fired. Data integrity was never at risk (no loss, no duplicate — the
  failure surfaced as an error), but the resilience did not engage. A
  category-less `Unknown`/`Internal` status whose source chain is populated is
  now retriable; a bare server `Unknown` (decoded from trailers, no source)
  stays terminal. Found by killing a real TCP connection under load through a
  chaos proxy — in-process mocks had emitted `Status::unavailable`, which is
  why it never showed. The WebSocket resumable lane was already correct (it
  treats a socket close as a reconnect trigger, not via this predicate).
- **`Template::from_json_created_event` rejected the `events-by-contract-id`
  response shape.** That endpoint wraps the created event as
  `{"created": {"createdEvent": …}}`; the helper peeled only the inner key and
  errored with "carries no createArgument". It now unwraps both layers, so all
  four JSON shapes the API uses decode.

- **A test broke `cargo test` without `--all-features`.** `futures_are_spawnable`
  asserted `JsonSubmission::recover` is `Send`, but that method is `ws`-gated
  while the assertion was not — so the default-feature build failed to compile.
  Invisible because CI ran tests only with `--all-features`; a second
  `cargo test --workspace` job (default features) now guards the whole class.
  Found by running the suite under every feature combination.
- **`tokio-stream`'s declared lower bound was `0.1`, but the `net` feature it
  uses landed in `0.1.1`** — a minimal-versions resolve does not build against
  `0.1.0`. Corrected to `0.1.1`. Found by `cargo +nightly build
  -Z direct-minimal-versions`.

## [0.2.2] — 2026-08-24

### Added

- **Conversion and hashing polish from a Rust API Guidelines audit** (16
  observations, none blocking; the audit's two breaking items are *decided*
  rather than fixed — [ADR-0010](docs/adr/0010-api-decisions-deferred-to-1.0.md)
  defers the `*ParseError` renames and the removal of `Party`'s unvalidated
  `From<&str>`/`From<String>` to 1.0, the latter blocked from gaining a checked
  `TryFrom` today by std's blanket impl).
  - `ChangeId` derives `Hash` — it is the key the docs tell applications to
    track submissions by, and it could not be a `HashMap` key. `Store`,
    `TransactionShape` and `ClientAuth` follow.
  - `canton-ledger` and `canton-admin` re-export `Auth`, `ErrorInfo` and
    `ResourceInfo` (and admin gains `ErrorCategory`): these appear in the
    signatures of `Error::error_info`/`resource_info` and `Config::auth`, so a
    user of one client crate could not previously name the types their own
    functions return without adding a `canton-core` dependency.
  - `ErrorCategory` implements `TryFrom<i32>` and `From<ErrorCategory> for i32`
    alongside the inherent methods.
  - `#[must_use]` on the constructors that lacked it; docs.rs feature badges on
    all seven remaining `ws`-gated methods (two of nine had them).
- **A compile-time proof that every public future is `Send`**
  (`tests/futures_are_spawnable.rs`) — an async API that cannot be
  `tokio::spawn`ed is unusable in a server, and the property breaks silently.
- **Hermetic coverage for the paths only live tests exercised**, found by
  measuring coverage rather than reading it: the three `TopologyClient` list
  reads (36% file coverage, all three public methods untested without a node —
  the same shape of gap the M1 review named about live tests), both recovery
  handles' `recover`, and `Submission::submit_and_wait`. Hand-written code
  measures 87.7% line coverage; `cargo udeps` reports no unused dependencies.

### Fixed

- The in-process duplicate-submission mocks answer synchronously on the
  fire-and-forget lane; a live Canton 3.5.7 participant does not — it accepts
  the RPC and reports the rejection on the completion stream. Established by
  adversarial probes run with the published 0.2.1 crates against a real node
  (which also confirmed the JSON duplicate answer is exactly the
  HTTP 409 + `DUPLICATE_COMMAND` the `is_duplicate_submission` predicate was
  written against, and that OIDC refresh survives a real token expiry). The
  mocks stay as pins of the synchronous case; their comments now say where
  reality differs.

## [0.2.1] — 2026-08-24

### Fixed

- **`canton-ledger` documented as failed on docs.rs.** Two
  `#[cfg_attr(docsrs, doc(cfg(feature = "ws")))]` attributes went in without the
  crate declaring `#![cfg_attr(docsrs, feature(doc_cfg))]`, which that attribute
  requires. `cargo doc` never sets `docsrs`, so the attributes compiled out and
  every gate stayed green; docs.rs does set it, on nightly, and the published
  crate's documentation page said the build failed. The crates themselves
  installed and built correctly throughout — this only ever affected the
  rendered documentation, and it cannot be fixed for a version already
  published, hence this patch.
- A `docsrs` CI job now builds the workspace the way docs.rs will, so the class
  of error that produced this cannot reach a release again.
- **The MSRV job never checked the MSRV.** `rust-toolchain.toml` pins the
  channel for anything run inside the checkout and wins over the toolchain the
  CI action installs — rustup says so in its own log, then uses stable. So a job
  that asked for 1.88 ran on stable, and `rust-version = "1.88"` went unverified
  every time it was claimed. It happens to be true (1.88 checks clean), but
  nothing was establishing that. Both jobs that want a specific toolchain now
  set `RUSTUP_TOOLCHAIN`, which is the override that beats the file.
- The `canton` facade declared `all-features` for docs.rs without the matching
  `rustdoc-args`, the same inconsistency one step behind.
- `cargo-semver-checks` covered 5 crates of 11. The M2 crates had no published
  baseline to compare against until 0.2.0, and the job's own comment said they
  should join afterwards — this is afterwards. `canton-proto` stays out by
  ADR-0002 and `canton-codegen-cli` has no library API.

### Changed

- `RELEASING.md` carries the post-publish verification the pre-publish gate
  cannot do: `cargo package` runs `--no-verify` because verifying needs the
  siblings to already be on crates.io, so the packaged artefact is first built
  by whoever installs it. The runbook now says to be that person first.
- The README compatibility table listed 0.2 as unreleased.

## [0.2.0] — 2026-08-24


All `canton-*` crates release in lockstep, so the M1 crates move to 0.2.0 with
the rest. Everything the 0.1.x line gained after the M1 submission — the read
request builders, the full `Commands` surface, Canton-native error
classification (see 0.1.2 below) and the documentation fixes of 0.1.3/0.1.4 —
is included.

### Breaking, for code on 0.1.x

Two signatures change, both because the old one could not express a correct
call. Everything else from 0.1.x compiles unchanged.

- **`OidcConfig::auth0(domain, client_id, secret)` → `auth0(domain, audience,
  client_id, secret)`.** Auth0 answers a client-credentials request without an
  `audience` by issuing a token for its own userinfo endpoint, which a
  participant rejects — so the old preset could not produce a working request.
  The audience identifies your Auth0 API and cannot be derived from the domain.

  ```diff
  - OidcConfig::auth0("my.eu.auth0.com", "client-id", "secret")
  + OidcConfig::auth0("my.eu.auth0.com", "https://ledger.example", "client-id", "secret")
  ```

- **`CantonClient::await_completion(command_id, parties, offset, timeout)` →
  `await_completion(&ChangeId, offset, timeout)`.** Canton identifies a command
  by (user, acting parties, command id), and matching on the command id alone
  can return another application's completion. The parties move inside the
  change ID.

  ```diff
  - client.await_completion(&command_id, vec![party.clone()], offset, timeout)
  + client.await_completion(&ChangeId::new("", vec![party.clone()], &command_id), offset, timeout)
  ```

  Better still, take the identity from the submission rather than rebuilding it:
  `let submission = client.submission(submit);` then
  `submission.recover(offset, timeout)`.

### Added — type-safe codegen from DARs

- **`canton-lf`** — native Rust Daml-LF reader: DAR container (zip +
  `MANIFEST.MF`) and an LF 2.x protobuf decoder (vendored `daml_lf2.proto`),
  including the LF 2.dev additions — the explicit package-import table and
  curried type application (`TApp`) — used by SDK 3.5 DARs. Decodes a DAR's whole
  dependency closure (`decode_all`).
- **`canton-daml`** — the runtime generated code depends on (as `rt`): the Daml
  primitive types (`Party`, `ContractId<T>`, `Numeric`, `Timestamp`, `Date`,
  `TextMap`, `GenMap`, `NestedOpt`, `Unit`), the `ToValue`/`FromValue` gRPC
  codecs and serde JSON conventions (LF-JSON: `Int64` and `Numeric` as strings —
  accepting a number on input — the nested-optional list form, `Unit` as `{}`,
  adjacently-tagged variants, …), the `Contract`/`Template`/`Interface`/
  `WithKey`/`Choice` traits, and the `create_command`/`exercise_command`/
  `exercise_by_key_command` builders.
- **`canton-codegen`** — DAR → typed Rust: a decoder-agnostic IR, the documented
  Daml-LF → Rust type mapping (see `docs/daml-lf-type-mapping.md`), and emission
  of records, variants, enums, **templates** (payload + typed choices + on-ledger
  id), **interfaces** (marker + view + choices), and **contract keys**, with JSON
  and gRPC codecs. Output is a fully-qualified `pub mod` tree
  (`crate::<package>::<module>::<Type>`) so cross-package references resolve and
  names never collide; template ids use the SCU-friendly `#<package-name>` form.
  The package segment is the package **name** — a version bump does not rename
  the paths a consumer imports — with the version appended only to separate two
  packages that would otherwise share a module name. Each template's docs carry
  its on-ledger id and its choices with their consuming flag.
- **`canton-codegen-cli`** — the `dpm-codegen-rust` binary (a `dpm codegen-rust`
  component / build-script tool) that writes a self-contained bindings crate from
  a DAR.
- **Pre-built bindings crates** (checked-in generated output, drift-guarded):
  `canton-splice-amulet`, `canton-splice-wallet`, `canton-splice-wallet-payments`,
  and `canton-quickstart-licensing`.
- **`canton-sample`** — reference app: builds a typed `AppInstallRequest` from
  the bindings, round-trips both codecs, and runs the full verification loop
  (submit → observe transaction → query ACS) over gRPC and JSON.
- **Verified** on a live cn-quickstart participant: the whole-DAR closure
  compiles for all corpus DARs, and the sample's typed create commits and is read
  back from the ACS on both transports.

### Added — a local network needs no configuration in your program

- **`canton_core::localnet`** reads a Splice LocalNet out of the environment —
  the variables [canton-devkit](https://github.com/bitdynamics-ab/canton-devkit)'s
  `localnet env` exports. After `eval "$(canton-devkit localnet env demo)"`,
  `Config::from_env()` is a working gRPC configuration and
  `JsonClient::from_env()` its JSON counterpart, with the participant's token
  attached; `Config::from_env_for("app-user")` reaches the other participant,
  and `localnet::party(alias)` the on-ledger id an application needs for
  `act_as`. `CANTON_ENDPOINT` / `CANTON_TOKEN` override the lot for an
  environment that is not a LocalNet. Runnable as the `localnet` example.
- **`Config` accepts a scheme-less `host:port`**, which is what a gRPC client
  dials and therefore what tooling hands out —
  `CANTON_GRPC_LEDGER_API_URL` is exactly this shape. Previously the missing
  scheme surfaced at the first RPC as an unexplained transport error; the
  scheme now defaults to the one TLS configuration implies.

### Security

- **`canton-lf` (zip bomb):** a DAR is now bounded **in total**, not only per
  entry. Every `.dalf` is read into memory, so an archive of many entries each
  legal on its own — a thousand at 256 MiB — passed every check and still asked
  for hundreds of gigabytes; zeros DEFLATE at roughly 1000:1, so the file
  carrying that request arrives small enough to go unnoticed. Each read is now
  capped by whichever of the per-entry ceiling and the remaining archive budget
  (2 GiB) is smaller, and the error says which limit was reached. The ceiling is
  ~50× the entire available corpus (41 MiB across 18 DARs), so it cannot fire on
  a real DAR. Found by reading canton-devkit's DAR reader, which has had the
  aggregate cap all along.

- **`canton-core`:** the mutual-TLS private key no longer reaches logs.
  `TlsConfig` derived `Debug` and holds `client_identity_pem`, so
  `format!("{config:?}")` — or one `tracing` field capturing a `Config` —
  printed the key byte by byte; `Config` leaked it too, along with any
  credentials in the endpoint URL. Both now have hand-written `Debug` that
  reports presence and length. Five error paths across `canton-core`,
  `canton-auth` and `canton-ledger` that quoted a URL verbatim now run it
  through `canton_core::redact_url`. Two more types held the same secret behind
  a `Debug` and were missed the first time: `JsonClient` derived one and printed
  its `base_url` — userinfo included — and `OidcConfig` redacted its
  `client_secret` field while printing a `token_url` that, for a provider taking
  client credentials as basic auth, *is* the secret. `TokenResponse` no longer
  derives `Debug` at all; it is one bearer token, and nothing should print it.
- **`canton-daml`:** a type mismatch no longer copies the offending value into
  the error message. `mismatch()` formatted it with prost's `Debug`, so
  decoding a payload as the wrong type put the whole record — parties, amounts,
  free text — into a string that travels into the application's traces and
  metrics. It now names the kind and stops.

- **`canton-core` (bearer token in `Debug`):** `AuthInterceptor` derived `Debug`
  over the token it injects into every gRPC request. The interceptor lives
  inside every client that holds a channel, so one `{:?}` on client state — a
  tracing field, a panic message, an error context — printed a live credential,
  and `SECURITY.md`'s "Debug output is redacted" was untrue as written. It now
  reports presence only. Reported privately by Equilibrium during their M1
  review, alongside the mutual-TLS key above.
- **`canton-core` (panic on a hostile retry hint):** `Error::retry_delay()`
  handed a server-supplied number to `Duration::from_secs_f64`, which panics on
  anything a `Duration` cannot hold — so a JSON error body carrying
  `"retryInfo": "1e300 seconds"` aborted the caller *inside error
  classification*, which the retry loop calls on every retriable failure. The
  conversion is fallible now and an out-of-range hint reads as no
  recommendation.

### Fixed — from Equilibrium's independent M1 review

An engineering review of the released 0.1.4 client by [Equilibrium](https://equilibrium.co),
carried out on the Development Fund milestone issue. Every finding is closed
here; two of them (the `Debug` leak and the retry-hint panic) are in
**Security** above.

- **Ambiguous submissions are recoverable.** A submission whose response is
  lost may still have committed, and the change ID is the only way back to the
  outcome — but the SDK generated the command id *inside* the call that failed
  and returned it only on success. `CantonClient::submission` and
  `JsonClient::submission` fix the identity first and hand back a `Submission`
  carrying its `ChangeId`, with `recover` reading the completion back. The
  existing client methods are thin wrappers over the same object.
- **Recovery matches the whole change ID.** `await_completion` compared
  `command_id` alone; Canton identifies a command by (user, acting parties,
  command id), and two applications on one participant may each use `daily-run`.
  It now takes a `&ChangeId`. A user id left to the bearer token is not
  compared (the participant resolved it and the client cannot know it), and a
  completion carrying no acting parties is not rejected on that ground.
- **The resumable update stream honours checkpoints.** `updates_with` dropped
  `OffsetCheckpoint` frames before the resumable wrapper could see them, so the
  resume point only advanced when a transaction arrived — on a quiet stream a
  reconnect went back to where the caller started, which after pruning fails
  outright. The resumable path now reads the unfiltered stream and filters for
  itself. Subscribers see no change.
- **A spent reconnect budget reports the participant's failure**, not
  `UnexpectedResponse("failed to resume after N reconnects")`, which threw away
  the status, the details, the correlation id and the retriable classification
  at the moment they were needed. Same fix in the resumable ACS read.
- **The JSON lane gained the four operations it was missing**: `submit`
  (`/v2/commands/async/submit`), `submit_and_wait`, `events_by_contract_id`,
  and recovery through `JsonClient::submission`. All four exist in Canton
  3.5.7's JSON API; this was the SDK stopping short.
- **`JsonClient::ws_active_contracts_resumable`** resubscribes from the last
  `streamContinuationToken` rather than restarting the snapshot, which the gRPC
  lane has done since M1.
- **The typed ACS read is lossless.** Every gRPC ACS method matched
  `ActiveContract` and dropped the rest, so a reassignment in flight at the
  snapshot offset — `IncompleteUnassigned` / `IncompleteAssigned` — vanished
  from a multi-synchronizer application's view. `AcsEntry` and the `acs_page` /
  `acs_entries` / `acs_entries_resumable` family are the lossless read; the
  active-only methods keep their names and are now that read with
  `into_active` applied.
- **The Auth0 and Okta presets produce their providers' normal requests.**
  Auth0 needs an `audience` (without one it issues a token for its own userinfo
  endpoint, which no participant accepts) — `auth0` now takes it, which is a
  **breaking** signature change. Okta reads the credentials from an
  `Authorization: Basic` header and rejects them in the body as
  `invalid_client`; the preset selects that, and `ClientAuth` exposes the
  choice for custom endpoints.
- **Telemetry covers a stream's life, not its opening.** `instrument_stream`
  counts errors that arrive after a subscription opens — previously a stream
  that failed an hour in had been recorded as a success and never revisited.
  The WebSocket upgrade carries `traceparent` (the only request a WS stream
  makes), structured events carry `trace_id`, and `otel::otlp_metrics` is a
  supported OTLP path for the counters, recorder and all.

- **A retry the participant de-duplicates is a success, not a failure.** This
  is the other half of the finding, and the half that was still open after the
  recovery handle was added. When the SDK retries a submission whose response
  was lost, the participant refuses the second attempt as `DUPLICATE_COMMAND` —
  because the first one was accepted. `submit` was reporting that rejection to
  the caller, which says the command did not happen at the exact moment it
  provably did. It now reports success, over both transports. A duplicate on
  the *first* attempt is untouched: nothing of ours is at the participant, so
  the caller reused a change id and needs to hear about it.

  The waiting variants cannot do this — their result is a transaction, and a
  de-duplicated retry does not carry one — so they surface the rejection and
  their documentation now says to recover through the handle rather than
  describing a caveat and leaving it there.

- **`examples/recover_a_submission.rs`** walks the finding's own scenario end
  to end: submit, submit the same change ID again (what a retry after a lost
  response looks like to the participant), watch it be rejected as
  `DUPLICATE_COMMAND`, and recover the original outcome. Verified live — the
  recovered `update_id` is the one the first submission committed. The README's
  other examples are backed by compiled example files; this one was not, and
  the newest API is the worst one to leave uncompiled.
- **The facade reaches what the documentation promises.** `canton::telemetry`
  was not re-exported, so `cargo add canton` could not see the metric names, the
  transport labels, or — now — the OTLP setup those metrics are meant to be
  exported through. The `otel` feature also reached `canton-core` only by way of
  `canton-ledger`, which was correct by accident. Both fixed, with tests that
  name the paths a reader of the README would try.

**From the same review's non-blocking list:**

- Requests the participant would certainly refuse are refused locally: a
  submission with no commands or no acting party, both minimum-ledger-time
  forms at once, a negative offset, an inverted range, a subscription filtered
  to nobody.
- `read_as` reaches the transaction filter of a submission's response, matching
  the Ledger API's own default; filtering to `act_as` alone returned a
  transaction quietly missing events.
- The idempotent reads — events-by-contract-id, the ACS page, the updates page
  — take the configured retry policy, which had applied only to `version`, the
  health check, `ledger_end` and submissions.
- Jitter never brings a retry back **before** a server-recommended delay; a
  `RetryInfo` is a minimum, and coming back early spends an attempt on a
  guaranteed rejection.
- The WebSocket streams take their reconnect budget and backoff from the
  client's `RetryConfig` instead of a hardcoded five-at-250ms.
- `list_known_parties` fails on a repeated page token instead of returning a
  prefix as if it were the whole list, and a topology response missing a
  required field fails the read rather than shrinking it.
- The vendored `.proto` files carry a provenance record and per-file SHA-256s,
  verified by a test, with `tools/vendor-protos.sh` for the refresh.
- `CANTON_TEST_REQUIRE_LIVE=1` turns a skipped live test into a failure, so a
  live run's result is a claim about a participant rather than about an empty
  environment.
- CI checks `aarch64-unknown-linux-gnu` and `x86_64-unknown-linux-musl`, and
  the `cargo-semver-checks` job is enabled now that a baseline exists.
- ADR-0005 no longer claims mixed installs "fail to resolve"; with caret
  requirements mixed *patch* versions resolve, which is intended.
- `canton-admin` documents that party management here is allocation and
  discovery, and why updates are out of scope for M1.
- The reference app reads its committed transaction back independently and
  matches the exact update id, on both transports.

### Fixed — from an external review of the M1 client

- **`canton-core` (message size):** the gRPC decode limit is raised off tonic's
  4 MiB default to 128 MiB, configurable via
  `Config::with_max_decoding_message_size`. One ACS page arrives as a single
  message and the participant permits page sizes up to 10 000, so the default
  was reachable in ordinary use and surfaced as a client-side `OUT_OF_RANGE`
  that reads like a server fault. Applied at all 24 service-client
  constructions across `canton-ledger` and `canton-admin` through one macro per
  crate, so a newly added RPC cannot pick the default back up. The WebSocket
  lane had the same problem one layer down and no way to fix it:
  `tungstenite`'s defaults cap a message at 64 MiB and a single **frame** at
  16 MiB. `JsonClient::with_max_decoding_message_size` now sets both, and
  defaults them to the same 128 MiB, so one transport cannot quietly be
  stricter than the other. (The HTTP lane needs nothing — `reqwest` puts no
  limit on a response body.)
- **`canton-core` (errors):** `Error::resource_info()` exposes the
  `google.rpc.ResourceInfo` Canton attaches to failures where "which one?" is
  the first question — `CONTRACT_NOT_FOUND` names the contract id. A `Vec`
  rather than an `Option`, because the JSON transport carries a list.
- **`canton-ledger` (unbounded waits):** the JSON lane had no timeout at all.
  `reqwest` applies none unless asked, so a participant that accepted the
  connection and then went quiet held the caller's task open for the life of
  the process — while the gRPC channel beside it had bounded the same call at
  30s since M1. `JsonClient::with_timeout` sets it, defaulting to that same
  30s, applied per request so it holds whatever order the builders were called
  in. The WebSocket **handshake** is bounded by the same value; the stream that
  follows is a live tail and deliberately is not.
- **`canton-core` (transport parity):** `Error::error_info()` and
  `Error::code()` now answer on the JSON transport too. Both returned `None`
  there while returning the real thing over gRPC — so an application that
  classified errors by error id, exactly as `error_info`'s documentation tells
  it to, silently fell back to string-matching the display text the moment it
  was pointed at the JSON lane. The body spells both (`code`/`context` and
  `grpcCodeValue`); a redacted error's literal `"NA"` is reported as no error
  id rather than as one. A live test now asserts that both lanes describe the
  same failure identically, so this cannot drift apart again unnoticed.
- **`canton-daml` (party ids):** `Party::parse` / `"…".parse::<Party>()`
  validate a party id a caller supplies — refusing empty, over-long, and
  characters Canton does not use — while `Party::new` still takes a wire value
  as-is, the same asymmetry `Numeric` already had. `FromStr` was `Infallible`,
  so `"".parse::<Party>()` succeeded and the failure surfaced only at the
  participant, as a `PermissionDenied` naming nothing. An empty party id is the
  shape `std::env::var` returns for `PARTY=`, which is how it happens in
  practice. Idea taken from zenith-network/canton-rs, which validates its
  identifier types.
- **`canton-codegen` (submit vs read):** the two encoders a generated template
  carries are now pinned to agree. The emitter writes the payload's field list
  twice — in `ToValue` and in `Template::to_record` — and they are different
  paths at runtime: `create_command` submits `to_record`, while a contract read
  back arrives through `from_value`. A template whose two lists drifted would
  write one shape to the ledger and expect another, and nothing compared them:
  every round-trip test goes `to_value` → `from_value`, which is the half the
  submit path does not use.
- **`canton-daml` (decode errors in containers):** a failure inside a `List`,
  `TextMap` or `GenMap` now names the element. Generated records attach the
  field name to every decode, but containers dropped everything below it, so
  one bad entry in a list of five hundred holdings reported that the list was
  bad and left the reader to find which. The paths compose: `holders.2`. A list
  reports the index, a `TextMap` the key (which locates an entry better than a
  position), a `GenMap` the position and which half of the entry — its keys are
  arbitrary values, so there is no name to point at.
- **`canton-daml` (dead API removed before it froze):** `record_field` and
  `record_value` are gone from the generated-code surface. The emitter never
  emitted either, nothing in the workspace called them, and the crate is about
  to publish — after which they would have to keep working forever.
- **`canton-codegen` (drift guard, published crates):** a CI job fetches the
  three DARs the published bindings were generated from — cn-quickstart tracks
  them in git — at a pinned commit, verifies their SHA-256, and runs the guards
  against them, asserting each one ran rather than skipped. Nothing before this
  checked on any push that the emitter still produces what is on crates.io.
  (`canton-quickstart-licensing` stays local-only: its DAR is built from source
  rather than committed, so there is nothing to pin.)
- **`canton-daml` (fixture drift):** the test fixture that claims to be written
  "exactly as the generator emits" is now checked against the real emitter
  instead of asserting it in a comment. It had drifted three times — the
  `.at(label)` on each field decode, the serde derives, and the per-field
  renames — and each drift quietly removed a path from coverage while every
  test stayed green.
- **`canton-codegen` (phantom type parameters):** a generated codec bounds only
  the type parameters it actually encodes. Daml permits a phantom parameter —
  declared but used in no field — and the emitter required `T: ToValue` for
  every declared one, so instantiating a phantom parameter with an **interface
  marker** produced Rust that does not compile. Markers carry no codec by
  design: they exist only as the tag of a `ContractId`. Valid Daml therefore
  generated invalid Rust, with the error landing in code the reader did not
  write. Four types across the published bindings carried the spurious bound
  and have been regenerated without it.
- **`canton-codegen` (IR semver):** every public IR struct is
  `#[non_exhaustive]`, with a constructor for each. The IR is documented as
  something a caller lowers and then post-processes, its fields are public, and
  it gained **forty fields during Milestone 2 alone** — so once the crate is on
  crates.io, one more field would be a breaking change for anyone who wrote a
  struct literal, and adding `#[non_exhaustive]` afterwards is itself breaking.
  Fields stay public, so reading and mutating a lowered IR is unchanged; only
  construction goes through `Record::new`, `Template::new` and the rest.
- **`canton-codegen` (a test that had stopped compiling):** the end-to-end test
  that generates a crate, builds it and round-trips both codecs was gated on
  `CODEGEN_COMPILE_TEST`, which nothing set — so it skipped on every push and
  went stale when the runtime made `Numeric` and `GenMap`'s fields private. It
  now builds its values through the public API, the way a consumer must, and CI
  sets the variable and asserts the test ran.
- **`canton-codegen` (manifest injection):** the DAR's package version is
  validated before it reaches the generated `Cargo.toml`. It was interpolated
  raw, so a version of the form `0.1.0"` + newline + `[dependencies.evil]` +
  `git = "…` closed the string and opened a table — an arbitrary git dependency
  in a manifest the caller then compiles, which is code execution from an
  archive. The archive-integrity guards do not stop it: the package id is the
  hash of whatever payload its author chose, so an authored DAR passes them
  all. The crate name beside it was already validated and the runtime path
  beside it already escaped; the version was the third field in the same
  manifest and the one that was missed.
- **`canton-codegen` (drift guard):** the guard that keeps the committed
  bindings honest now runs in CI. The four existing ones need a DAR from a
  Splice or cn-quickstart checkout, so on a machine without one they skip — and
  a skipped test reads exactly like a passing one, which meant the property had
  never been enforced anywhere but a developer's laptop, for three crates that
  are published. A fifth guard regenerates from the DAR committed to this
  repository and AST-compares against a committed fixture, and the CI job
  asserts it ran rather than skipped, the way the conformance-oracle job
  already did.
- **`canton-daml` (typed read on JSON):** `Template::from_json_created_event`
  is the JSON counterpart of `from_created_event`, which was gRPC-only. Both
  transports carry the same contract for the same bindings, so an application
  may write over one and read over the other — but on the JSON lane a caller
  had to reach into `event["CreatedEvent"]["createArgument"]` themselves, and
  nothing then checked that the event was that template at all. A party's
  stream carries every template it sees, and where two payloads share a field
  shape, decoding one as the other succeeds and is wrong. Accepts the event
  wrapped (`CreatedEvent` / `createdEvent`) or bare, and compares module and
  entity but not the package id, matching the gRPC path under Smart Contract
  Upgrade.
- **`canton-codegen` (shared packages):** a package already published as its own
  crate can be **referenced** instead of re-generated —
  `Options::with_external_package(name_or_id, crate)`, or `lower_dar_with` for
  the library path. A DAR's dependency closure is shared, so
  `splice-api-token-holding-v1` sits under amulet, wallet and wallet-payments
  alike; generating it into each gave each crate its own `Holding`, and Rust
  treats those as unrelated types. A program depending on two of the published
  binding crates failed to compile with "expected `Holding`, found a different
  `Holding`" — for the same interface, in the same package. Packages are keyed
  by **name** as well as id, and the name is the one to prefer: an id is the
  hash of one build, while the name survives a version bump, which is the point
  of addressing packages by name under Smart Contract Upgrade.
- **`canton-codegen` (hostile DAR):** a type that resolves to itself is
  refused instead of overflowing the stack. Interned types are a flat table of
  indices, so `interned_types[0] = Interned(0)` — or two entries pointing at
  each other — is not malformed to prost, which bounds nested *messages* and
  has no view of the table. Following it recursed until the stack ended, and a
  stack overflow aborts the process rather than failing the DAR: the one input
  in this reader that still killed the caller after the zip-bomb, entry-size,
  archive-size, package-id and LF-version guards. Bounded at 256 levels, where
  the deepest type in the 648-package corpus resolves in 15.
- **`canton-lf` (archive integrity):** a package's payload is hashed and checked
  against the id it declares before it is parsed, and a `hash_function` other
  than SHA-256 is refused rather than assumed. That id is embedded in generated
  bindings as `PACKAGE_ID` and is what cross-package references resolve
  through, so a DAR whose id does not match its bytes previously produced
  bindings naming a package its contents do not answer to.
- **`canton-lf` (LF version):** an archive declaring a Daml-LF 2.x minor this
  build was not compiled against is refused instead of decoded. prost drops
  fields from a schema it does not know, so a newer minor yielded a package
  quietly missing template fields. Accepts `2.1` and `2.2`, which is what the
  available corpus contains (648 packages across 18 DARs). The **major**
  version is decided first: an LF 1 archive — every DAR a Daml 2.x SDK built —
  was being reported as an unsupported *LF 2 minor*, telling the reader to
  upgrade an SDK that will never read it.
- **`canton-codegen` (decode errors):** generated `FromValue` bodies attach the
  field name to a failure, so a mismatch inside a nested record reports
  `meta.values` instead of a bare "expected Text". `ValueError::at` existed and
  was documented as being used by generated code; it was not.

### Fixed — pre-release hardening (M2 quality audit)

- **Recursion is now Rust-sound.** `Optional` self-recursion (`data Tree = Node
  { left : Optional Tree }`), mutual recursion (including cross-module), and
  recursion through a generic instantiation previously generated
  infinitely-sized structs (E0072); a crate-wide containment-cycle breaker now
  boxes every cycle-closing reference occurrence.
- **Record decode matches Canton's wire shapes.** Generated `FromValue` locates
  fields by label *or* declaration index (non-verbose output omits labels) and
  decodes an absent trailing `Optional` field as `None` (record normalization
  under Smart Contract Upgrade drops them).
- **`Numeric` compares numerically**, not textually — the ledger echoes `"1.5"`
  as `"1.5000000000"` and the two are now equal (`Eq`/`Ord`/`Hash` on the
  canonical decimal; `Numeric::parse` validates early).
- **Typed read path:** `Template::from_created_event` decodes a `CreatedEvent`
  into the payload struct (with a template-identity check);
  `ContractId::retag` re-tags an id for interface exercise.
- **Codegen robustness on arbitrary DARs:** per-entry decompression cap
  (zip-bomb guard), typed errors naming the offending file/package, `$`-named
  GHC-internal types skip instead of panicking the emitter, snake-case field
  collisions and duplicate package modules are detected, generated code spells
  std types fully qualified so same-named Daml types cannot shadow them.
- **CLI safety:** refuses to overwrite files it did not generate (`--force` to
  override); validates the crate name; the generated crate's version is the
  DAR package's version; `--runtime-path` is absolutized and TOML-escaped.
- Always-on fixture tests: `testdata/splice-api-token-holding-v1-1.0.0.dar`
  runs the full decode → lower → emit pipeline (plus determinism and CLI
  contract checks) in every `cargo test`, no external setup.

### Changed

- **`canton-core` gains `Error::Payload`** — the one addition to the surface
  published in 0.1.x. It carries a `canton-daml` codec failure as its `source`,
  so a typed decode and a transport failure land in the same `Result` and an
  application can stay on `canton::Error` end to end. `Error` is
  `#[non_exhaustive]`, so the new variant does not break a `match`.
  `canton-daml` supplies the bridge (`impl From<ValueError> for
  canton_core::Error`), and `ValueError` is structured rather than a string:
  it carries the field **path** it failed at and the message separately, so the
  path survives into the error chain instead of being formatted away.
- The `canton` facade re-exports the codegen runtime as [`canton::daml`], so
  `cargo add canton` gets a version-locked runtime for generated bindings. The
  generator stays out of the facade on purpose: it is a build-time tool
  (`cargo install canton-codegen-cli`, or depend on `canton-codegen` from a
  build script).
- The codegen pipeline (`generate`, `Options`, `Runtime`, `Stats`) lives in
  `canton-codegen`, not in the `-cli` crate, so a build script can call it
  without pulling in a CLI. `canton-codegen-cli` is binary-only.
- Codegen errors are typed: `GenerateError` and `CodegenError` replace
  `Box<dyn Error>` and `syn::Error`, and `SkippedType` (with `module()` /
  `reason()`) replaces a bare string. No `syn`/`proc-macro2` type appears in a
  public signature.
- `Options` uses the same consuming-builder style as M1's `Config`, and
  `Options`/`Stats`/`Runtime` are `#[non_exhaustive]`.
- Generated crates get a publishable `Cargo.toml` (description, no forced
  `[workspace]` stanza) and their default name drops the DAR's version suffix,
  so a DAR bump no longer renames the crate a caller depends on.
- Every published crate now ships the Apache-2.0 licence text, and integration
  tests whose fixtures live outside the package are no longer packaged.

> **Publish order:** `canton-proto` → `canton-core` → `canton-auth` →
> `canton-lf` → `canton-daml` → `canton-ledger` → `canton-admin` →
> `canton-codegen` → `canton-codegen-cli` → `canton` → the `canton-splice-*`
> bindings. `canton-sample` and `canton-quickstart-licensing` stay unpublished
> (reference material).
## [0.1.4] - 2026-08-05

Documentation and developer-experience fixes from the Canton Foundation review
of the M1 submission. No library code changes; `0.1.3` and `0.1.4` are the same
SDK.

### Fixed

- **Examples:** `version_and_health` and `submit_and_read` read the same
  `CANTON_TEST_*` variables as the live tests (the unprefixed `CANTON_*`
  spellings still work and take precedence). They previously required a second,
  undocumented set of six names, so following the README's own setup and then
  running the example it links to failed on `set CANTON_TOKEN_URL`.
- **Live tests:** the ACS token-paging test no longer caps the walk at 500
  pages. At page size 1 that bounded the ledger rather than the loop, so on a
  participant with a few hundred contracts a healthy walk reported "paging did
  not terminate". It now fails when the page token stops advancing, which is
  the condition it was meant to catch.

### Documentation

- **README:** the live-test variables are documented under their real names
  (every one is `CANTON_TEST_`-prefixed) in a table that says what each gates,
  with a copy-pasteable export block. `CANTON_TEST_JSON_ENDPOINT` (the JSON and
  WebSocket tests) and `CANTON_TEST_SYNC_ID` were previously undocumented, and
  six names were listed without their prefix.
- **README:** node setup lists the
  [Canton Builder Tool](https://canton-network-devs.github.io/Canton-Builder-Tool/#part-builder)
  and [Splice LocalNet](https://docs.sync.global/app_dev/testing/localnet.html)
  alongside `cn-quickstart`, with what each one actually provides: Splice
  LocalNet is unauthenticated, so the OIDC-gated tests skip there as well as the
  command-submission ones; the Builder Tool's `canton builder deploy` uploads a
  DAR, which is how to get `CANTON_TEST_LICENSING_PKG` without cn-quickstart.

## [0.1.3] - 2026-08-04

### Fixed

- **`canton-ledger`:** `CompletionsRequest`'s JSON body builder is now gated on
  the `ws` feature, which is the only thing that reads it — the JSON Ledger API
  has no POST endpoint for completions. Without the gate the method was dead
  code in any build without `ws`, which failed the workspace's own
  `-D warnings` on the feature powerset. **Consumers of 0.1.2 are unaffected**:
  Cargo caps lints in registry dependencies, so a `default-features = false`
  build of the published crate compiles either way.

## [0.1.2] - 2026-08-04

Additive (semver-compatible) API growth from external M1 review feedback:
request builders for the read streams, and Canton-native error classification.

### Added

- **`canton-ledger` (request builders):** `UpdatesRequest`,
  `ActiveContractsRequest`, and `CompletionsRequest` (module `request`,
  re-exported at the root) open up the request surface the plain methods
  hard-code — on **every read path and both transports**: gRPC
  (`updates_with`, `updates_resumable_with`, `updates_page_with`,
  `active_contracts_with`, `active_contracts_page_with`,
  `active_contracts_resumable_with`, `completions_with`), JSON POST
  (`JsonClient::updates_with`, `active_contracts_with`), and WebSocket
  (`ws_updates_with`, `ws_active_contracts_with`, `ws_completions_with`):
  - bounded streams (`until(end_inclusive)`) for catch-up/sync reads that
    terminate at an offset instead of staying live;
  - template and interface filters (`for_templates` / `for_interfaces`,
    accepting `package:Module:Entity` ids with `#package-name` references)
    instead of the wildcard filter;
  - transaction shape selection (`TransactionShape::AcsDelta` /
    `LedgerEffects`), created-event blobs, topology events, dropping
    reassignments, and non-verbose records;
  - `UpdatesRequest::descending` — bounded newest-first reads on the stream
    and page paths, both transports (rejected up front on the resumable
    stream, whose position tracking assumes ascending order);
  - `for_any_party` (updates and ACS) — the `filters_for_any_party` /
    `filtersForAnyParty` full-ledger ingestion form, for tokens with
    wildcard read authorization;
  - `CompletionsRequest::with_user_id` to scope the completion stream to a
    submitting user id (pairs with `Submit::with_user_id`) — required for
    tokens that do not carry a user id.

  Defaults match the plain methods exactly (`updates_with(UpdatesRequest::
  new(p, o))` ≡ `updates(p, o)`), whose signatures are unchanged — the plain
  methods now delegate to the builders, so there is a single body/request
  producer per query on each transport.
- **`canton-ledger` (submit):** `Submit::with_transaction_shape` selects the
  shape of the transaction returned by `submit_and_wait_for_transaction`
  (ledger effects — the default, as before — or ACS delta), removing the last
  hard-coded `TransactionShape`.
- **`canton-ledger` (submit, full `Commands` surface):** every remaining
  `Commands` field is now settable on both transports —
  `Submit`/`JsonCommands` gain `with_submission_id`, `add_disclosed_contract`
  (the explicit-disclosure counterpart to the created-event blobs the read
  builders expose), `with_package_id_selection_preference` (the SCU upgrade
  pin), and `with_min_ledger_time_abs`/`with_min_ledger_time_rel`; `Submit`
  additionally gains `with_prefetch_contract_keys` and `with_taps_max_passes`,
  and `JsonCommands` gains `with_deduplication_period` (raw JSON, mirroring
  `Submit::with_deduplication_duration`/`_offset`).
- **`canton-ledger` (JSON retries):** `JsonClient::with_retry(RetryConfig)` —
  the same opt-in retry pipeline as the gRPC client (category-first
  classification of the participant's error body, exponential backoff,
  server-recommended delays honoured; de-duplication-safe for submits since
  the command id stays fixed across attempts).
- **`canton-core` (error classification):** errors now expose the precise
  retry semantics the Ledger API encodes, beyond the transport status code —
  on **both transports**:
  - `ErrorCategory` — Canton's error categories (from
    `ErrorInfo.metadata["category"]` on gRPC statuses, or the
    `errorCategory` field of JSON API error bodies) with the retryability the
    error-code docs define; surfaced as `Error::category()`.
  - `Error::retry_delay()` — the server-recommended pause, from the
    `google.rpc.RetryInfo` detail (gRPC) or the `retryInfo` field (JSON).
  - `Error::correlation_id()` — from the `google.rpc.RequestInfo` detail
    (gRPC) or `correlationId`/`traceId` (JSON), for quoting to the
    participant operator.
  - `Error::is_retriable` is now **category-first** for both `Error::Status`
    and `Error::Http`: the category's verdict wins over the gRPC/HTTP status
    code (e.g. a category-12 "seek after ledger end" on HTTP 400 is
    retryable; a category-10 "resource exists" on `ABORTED` is not), a bare
    `RetryInfo` counts as retryable, and the previous code-based
    classification remains as the fallback for non-Canton errors.
    **Behaviour change** under opt-in retry: HTTP errors that Canton marks
    retryable now retry where they previously failed fast — including the
    413 `JSON_API_MAXIMUM_LIST_ELEMENTS_NUMBER_EXCEEDED` (category 2 with a
    1-second `retryInfo`), even though shrinking the request `limit` is the
    actual remedy. The SDK follows the server's verdict rather than
    special-casing codes; callers that want to fail fast on it can match on
    `Error::Http { status: 413, .. }` or handle it before retrying.
  - `retry::run_with_retry` honours the server's `retry_delay` when it
    exceeds the local backoff.

### Fixed

- **`canton-ledger` (WebSocket error frames):** a `JsCantonError` frame on a
  WS stream is now surfaced as `Error::Http` (with the gRPC code's canonical
  HTTP status and the full error body), instead of `Error::CommandRejected`.
  The frame is the same error object the JSON API returns in HTTP error
  bodies, so it now carries its `category()`/`retry_delay()`/
  `correlation_id()` — and, **behaviour change**, `ws_updates_resumable` now
  correctly reconnects on transient participant errors (categories 1/2/…),
  which the never-retriable `CommandRejected` mapping previously turned into
  a dead stream.

## [0.1.1] - 2026-07-22

Hardening patch over 0.1.0, from an adversarial pre-release review. Each fix has
a regression test. Semver-compatible, so `^0.1` resolves to it.

### Fixed

- **`canton-core` (TLS):** `Config::with_tls` no longer connects in plaintext on
  an `http://` endpoint. `tonic` selects the TLS handshake from the URI scheme,
  not from the presence of a `tls_config`, so TLS was silently ignored (no
  encryption, no server-certificate verification, no client certificate for
  mutual TLS). The scheme is now normalised to `https://` when TLS is
  configured, case-insensitively.
- **`canton-ledger` (JSON and WebSocket TLS):** the same normalisation on
  `JsonClient::with_tls`. An `http://` base URL previously sent plaintext HTTP
  with the configured certificates unused and opened a `ws://` socket; the base
  URL is now upgraded to `https://` so both the HTTP and WebSocket lanes use TLS.
- **`canton-ledger` (JSON updates):** the JSON update stream now requests
  `includeReassignments`, matching the gRPC transport. **Behaviour change:** a
  JSON update stream now surfaces the assigned/unassigned reassignment events it
  silently dropped before.
- **`canton-auth`:** the cached-token TTL is clamped to 30 days, so a token
  endpoint reporting an absurd `expires_in` can no longer overflow the cache
  deadline and panic.
- **`canton-core` (retry):** `Error::is_retriable` now treats the whole `5xx`
  range as retriable (plus `408` and `429`), instead of a hand-picked subset
  that missed codes such as `501`, `507`, `509`, `511`, and `520`.

## [0.1.0] - 2026-07-20

### Added — Milestone 1: core Ledger API client, auth & PoC

- **`canton`** — the SDK entry point: a thin facade (re-exports only, no
  logic) over the crate family — `canton::ledger` / `canton::auth` /
  `canton::admin`, the shared `Config`/`Error`/TLS/retry types at the root,
  and the `ws`/`otel` features forwarded (ADR-0007).

- **`canton-ledger`** — async Ledger API v2 client over gRPC:
  - Command submission: fire-and-forget `submit`, `submit_and_wait`
    (update id), and `submit_and_wait_for_transaction`; change-ID
    de-duplication; completion stream + `await_completion` recovery; the
    `Submit` builder (`act_as`/multi-party, `user_id`, `read_as`,
    `workflow_id`, `synchronizer_id`, de-duplication period).
  - Reads & streams: ACS snapshot + paging (+ **resumable** paged ACS),
    updates stream + paging + reverse-order (+ **resumable** from the last
    offset), event query by contract id, node health
    (`grpc.health.v1`).
  - JSON transport: command submission, bounded ACS/update reads, and —
    behind the `ws` feature — WebSocket streaming (updates, ACS,
    completions, **resumable** updates), TLS-aware.
  - TLS/mTLS on gRPC, HTTP, and WebSocket from one shared `TlsConfig`.
- **`canton-core`** — shared kernel: typed `Error` with
  retriable/non-retriable classification and structured
  `google.rpc.ErrorInfo` details; `Config` (endpoint, auth, TLS, timeout);
  opt-in retry with per-attempt timeouts and jittered exponential backoff;
  telemetry (`tracing` spans + `metrics` counters per method/transport,
  optional OTLP export + W3C trace-context propagation behind `otel`).
- **`canton-auth`** — OIDC client-credentials `TokenProvider` with caching,
  refresh, bounded fetch, credential-rejection surfacing (`Error::Auth`),
  and Keycloak/Auth0/Okta presets.
- **`canton-admin`** — party allocation/management, user self-inspect,
  packages read, and topology read (party→participant mappings, namespace
  delegations, vetted packages) over the Canton admin API.
- **`canton-proto`** — generated types/stubs from vendored protos pinned to
  Canton 3.5.7 (Ledger API v2, Canton admin-api topology read, gRPC health).
- Runnable examples (`version_and_health`, `submit_and_read`), CI
  (fmt/clippy/tests on Linux+macOS+Windows/docs/deny/MSRV/feature matrix).
