# Interactive submission and external signing

An ordinary submission asks the participant to interpret a command **and** to
authorize it with the party's key, so the participant holds that key.
Interactive submission splits the two. The participant interprets the command
and returns the transaction with a hash of it. The party's key signs the hash
wherever it is kept (an HSM, a cloud KMS, an offline machine), and the
participant executes the signed transaction without ever seeing the key.

Three crates take part. Interactive submission is on the gRPC client,
`CantonClient`; the JSON client does not offer it.

| Crate | Its part |
|---|---|
| [`canton-admin`](https://docs.rs/canton-admin/latest/canton_admin/) | Onboards the external party: [`AdminClient::generate_external_party_topology`](https://docs.rs/canton-admin/latest/canton_admin/struct.AdminClient.html#method.generate_external_party_topology) and [`allocate_external_party`](https://docs.rs/canton-admin/latest/canton_admin/struct.AdminClient.html#method.allocate_external_party). |
| [`canton-ledger`](https://docs.rs/canton-ledger/latest/canton_ledger/) | Prepare, sign, execute: [`Prepare`](https://docs.rs/canton-ledger/latest/canton_ledger/interactive/struct.Prepare.html), [`Prepared`](https://docs.rs/canton-ledger/latest/canton_ledger/interactive/struct.Prepared.html), [`Executable`](https://docs.rs/canton-ledger/latest/canton_ledger/interactive/struct.Executable.html), and the [`CantonClient`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html) methods that send them. |
| [`canton-signer`](https://docs.rs/canton-signer/latest/canton_signer/) | The [`Signer`](https://docs.rs/canton-signer/latest/canton_signer/trait.Signer.html) trait, and [`Ed25519Key`](https://docs.rs/canton-signer/latest/canton_signer/struct.Ed25519Key.html), an in-memory implementation. |

## Dependencies and the `ed25519` feature

`cargo add canton` brings all three, as `canton::ledger`, `canton::admin` and
`canton::signer`, with the in-memory Ed25519 key on by default. That key sits in
process memory and is not zeroed on drop: fine for development, tests, and
deployments that accept the risk. Production keys belong in an HSM or KMS
behind your own `Signer`. To build without the in-memory key type (`ring` itself stays in the tree,
because the TLS stack uses it):

```toml
canton = { version = "0.3", default-features = false }
```

The trait, the signature types and their validation do not depend on the
feature. On the individual crates, `canton-signer` has `ed25519` on by default;
`canton-ledger` and `canton-admin` need only the trait, and their `ed25519`
feature turns on `canton-signer`'s.

## Onboard an external party

The participant does not hold an external party's key, so allocating the party
takes two calls with a signature in between:

1. `generate_external_party_topology(synchronizer, party_hint, &public_key)`
   returns an
   [`ExternalPartyTopology`](https://docs.rs/canton-admin/latest/canton_admin/struct.ExternalPartyTopology.html):
   the party id the participant would assign, the **fingerprint** it computed
   for the key, the onboarding topology transactions, and one multi-hash over
   all of them. Nothing is submitted yet.
2. `allocate_external_party(synchronizer, &topology, &signer, user_id)` signs
   the multi-hash and submits the transactions with the signature. It returns
   the party id once the allocation has reached the synchronizer, so the party
   is usable straight away.

Canton computes the fingerprint, and every signature carries it in
`signed_by`, so an `Ed25519Key` cannot sign for the ledger until
[`into_signer(fingerprint)`](https://docs.rs/canton-signer/latest/canton_signer/struct.Ed25519Key.html#method.into_signer)
turns it into an
[`Ed25519Signer`](https://docs.rs/canton-signer/latest/canton_signer/struct.Ed25519Signer.html).

From the live test
([`interactive_live.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-ledger/tests/interactive_live.rs)),
where `ledger` is a `CantonClient`, `admin` an `AdminClient`, `hint` the party
hint and `user_id` the ledger user that will submit:

```rust,ignore
{{#include ../../../crates/canton-ledger/tests/interactive_live.rs:181:211}}
```

- Both calls go to the Ledger API port, and allocating needs a token whose user
  has `ParticipantAdmin` (the live test makes both calls as that admin). A
  LocalNet without authentication needs nothing beyond the endpoint.
- `user_id` gets `act_as` rights on the new party; an empty string grants none.
  Canton derives the ledger user from the token's `sub`, which in a
  client-credentials flow is a service-account id, not the OIDC client id.
  `AdminClient::current_user()` under the submitting user's token returns it.
- The SDK reads the onboarding set before it signs for it. It recomputes the
  multi-hash and decodes each transaction, and accepts only the key's own
  namespace delegation, a mapping of the party to that one key, and hosting
  with confirmation or observation permission. It also refuses a party id whose
  namespace is not the key's fingerprint, and an allocation of any party other
  than the one signed for.
- `allocate_external_party` is not retried: the request has no idempotency key.
  If the response is lost, look the party up with `AdminClient::get_parties`
  before trying again.

## Prepare, sign, execute

Each type is one state of the flow and offers only what is possible in it.
`Prepare` says what to interpret and goes to
[`CantonClient::prepare_submission`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.prepare_submission).
`Prepared` is the interpreted transaction and its hash. `Executable` is signed,
and so it can be executed. The round trip from the live test, with `party` and
`signer` from the onboarding above and `command` a create that only `party`
signs:

```rust,ignore
{{#include ../../../crates/canton-ledger/tests/interactive_live.rs:294:318}}
```

### `Prepare`

`Prepare::new(party)` acts as one party, `Prepare::new_multi(parties)` as
several. Add commands with `add_command` / `add_commands`, built with
`canton_ledger::create` / `exercise` or from generated bindings. Optional:

| Method | Effect |
|---|---|
| `with_command_id` | The command component of the change ID. Left unset, a UUID is generated once at prepare time and carried through execution. |
| `with_user_id`, `with_read_as`, `with_synchronizer_id` | As on an ordinary submission. |
| `with_disclosed_contracts` | Explicitly disclosed contracts. A token-standard [`TokenCommand`](https://docs.rs/canton-token/latest/canton_token/transfer/struct.TokenCommand.html) builds the whole `Prepare`, disclosures included, with `into_prepare(party)`; see [The token standard](token-standard.md). |
| `with_min_ledger_time_abs`, `with_min_ledger_time_rel` | Lower bound for ledger time. Exclusive; the later call wins. |
| `with_max_record_time` | The record time after which the participant refuses the prepared transaction. |
| `with_hashing_scheme_version`, `trust_participant_hash`, `with_verbose_hashing` | See [Hashing schemes](#hashing-schemes-and-the-hash-check). |
| `with_package_id_selection_preference`, `with_prefetch_contract_keys`, `with_taps_max_passes`, `without_traffic_cost_estimation` | The remaining `PrepareSubmissionRequest` fields. |

De-duplication and the submission id belong to execution, on `Executable`.
With `Config::with_retry`, `prepare_submission` is retried like a read.

### `Prepared`

`hash()` is the bytes to sign: the participant's hash of the transaction,
already checked by the SDK, and not a message to hash again. `transaction()` is
the interpreted transaction. Show it to whoever authorizes the signature before
signing; the hash proves what is signed, not that it was meant. `act_as()`,
`command_id()`, `change_id()`, `cost_estimation()`, `hashing_scheme_version()`
and `hashing_details()` report the rest. `sign_with(&signer)` signs as the only
acting party; with several, use `sign_as(party, &signer)`.

### `Executable`

`with_deduplication_duration` or `with_deduplication_offset` sets the
de-duplication period. `with_submission_id` fixes the submission id; left
unset, each attempt gets a fresh one. `with_transaction_shape` picks the events
the returned transaction carries. A missing or wrong signature is rejected by
the participant, not caught locally. Execute it one of three ways:

| `CantonClient` method | Returns |
|---|---|
| [`execute_submission`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.execute_submission) | `()` once the participant has accepted the transaction. Acceptance is not commitment: the outcome arrives on the completion stream. |
| [`execute_submission_and_wait`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.execute_submission_and_wait) | The update id and the completion offset, once the transaction has committed. |
| [`execute_submission_and_wait_for_transaction`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.execute_submission_and_wait_for_transaction) | The committed transaction, its events shaped by [`TransactionShape`](https://docs.rs/canton-ledger/latest/canton_ledger/request/enum.TransactionShape.html) and filtered for the acting and reading parties. |

### How long a prepared transaction stays executable

A transaction that depends on time must be executed within the ledger-time
tolerance of the ledger time assigned at preparation, usually about a minute.
One that does not is bounded by the preparation-time skew, on the order of
hours, so hold a `Prepared` across a long signing ceremony only on purpose.
Past `with_max_record_time` the participant is certain to refuse it, which is
what makes preparing the same intent again safe.

## Several acting parties

The participant requires a signature from every party in `act_as`:

```rust,ignore
let prepared = client
    .prepare_submission(Prepare::new_multi(vec![alice.clone(), bob.clone()]).add_command(command))
    .await?;
let executable = prepared.sign_as(&alice, &alice_signer).await?;
let executable = executable.and_sign_as(&bob, &bob_signer).await?;
assert!(executable.unsigned_parties().is_empty());
let transaction = client.execute_submission_and_wait_for_transaction(executable).await?;
```

`sign_with` on a multi-party submission returns `Error::InvalidRequest`,
since it is ambiguous which party the signature is for, and so does signing as
a party that is not acting. A party whose topology sets a signing threshold
above one calls `and_sign_as` again with its other keys; the signatures share
that party's entry on the wire. `Executable::with_signature(party, &signature)`
attaches a
[`Signature`](https://docs.rs/canton-signer/latest/canton_signer/struct.Signature.html)
produced elsewhere, such as an offline ceremony. It is on `Executable`, so at
least one party has to sign through a `Signer` first; when the only signature
comes from such a process, wrap the process in a `Signer`.

## Hashing schemes and the hash check

The signature covers the hash, so the hash decides what is authorized. A
participant that returned your transaction next to the hash of a *different*
one would get a valid signature over the other one. The SDK therefore does not
take the hash from the response: it recomputes it from the returned
transaction and refuses a response whose hash differs (V2 since 0.3.1, V3 since
0.3.2; see the [changelog](../project/changelog.md)).

`Prepare` asks for hashing scheme V2 explicitly instead of leaving the choice
to the participant. The scheme is echoed in the response and sent back with
the signature, so the participant verifies under the scheme it hashed with.

| Scheme | What the SDK does | Use it for |
|---|---|---|
| V2 | The default. Recomputes the hash and refuses a mismatch. | Transactions without contract keys. |
| V3 | Recomputes the hash and refuses a mismatch. | Transactions that use contract keys, which V2 cannot hash. |
| Any other (V4 is development-protocol-only) | Refuses the response unless you set `Prepare::trust_participant_hash()`. | Only with a preparing participant you trust. |

Ask for V3 with
[`with_hashing_scheme_version`](https://docs.rs/canton-ledger/latest/canton_ledger/interactive/struct.Prepare.html#method.with_hashing_scheme_version),
passing the generated proto enum `canton_ledger::proto::interactive::HashingSchemeVersion`
(protocol-stable, not SemVer-stable: [ADR-0002](../reference/adr/0002-proto-types-are-protocol-stable.md)):

```rust,ignore
{{#include ../../../crates/canton-ledger/tests/interactive_live.rs:352:360}}
```

`prepare_submission` returns `Error::UnexpectedResponse`, and nothing is
signed, when the response has no transaction or no hash; is hashed under an
unknown scheme or one other than requested; has a hash that is not 32 bytes
(every scheme is SHA-256); under V2 or V3, carries a hash that is not the
returned transaction's, or a transaction that cannot be hashed under that
scheme; or uses any other scheme without the opt-in.

[`trust_participant_hash()`](https://docs.rs/canton-ledger/latest/canton_ledger/interactive/struct.Prepare.html#method.trust_participant_hash)
signs the participant's hash as returned and logs a warning. An unchecked hash
authorizes whatever that participant hashed. The opt-in does not turn off the
check under V2 and V3.

The hashing is public in
[`canton_ledger::hashing`](https://docs.rs/canton-ledger/latest/canton_ledger/hashing/index.html):
[`hash_prepared_transaction(prepared, version)`](https://docs.rs/canton-ledger/latest/canton_ledger/hashing/fn.hash_prepared_transaction.html),
`hash_prepared_transaction_v2` and `hash_prepared_transaction_v3` return the
32-byte hash or a `HashingError`. Use them when the service that holds the key
is not the process that prepared: hand it the `PreparedTransaction` and it can
check the hash itself before signing. To debug a mismatch, prepare with
`with_verbose_hashing(true)` and read `Prepared::hashing_details()`. The
participant may refuse, and the format is not stable: read it, do not parse it.

## Implementing `Signer` for an HSM or KMS

`Signer` has three methods: `public_key()`, `fingerprint()` and an async
`sign(hash)`. It is object-safe, so you can hold an `Arc<dyn Signer>` and pass
`&*signer` where a `&dyn Signer` is expected. The rules:

- `sign` receives a hash the participant produced: `prepared_transaction_hash`
  for a submission, the multi-hash for onboarding. Sign those bytes as they
  are; do not hash them again.
- `fingerprint` is the one Canton assigned, from
  `ExternalPartyTopology::public_key_fingerprint()`. For onboarding, describe
  the key with `PublicKey::new(KeyFormat, data, KeySpec)`.
- Declare the algorithm and format the key actually used
  (`SigningAlgorithm::{Ed25519, EcDsaSha256, EcDsaSha384, MlDsa65}`,
  `SignatureFormat::{Concat, Der}`). Canton parses the bytes by the format, so
  `Concat` for ECDSA, or `Der` for EdDSA, fails verification.
- `Signature::new` refuses empty bytes and an Ed25519 `Concat` signature that
  is not 64 bytes, so a truncated reply from a signing service fails there with
  a clear message rather than as a verification failure.

A sketch for an HSM that holds an Ed25519 key, with `HsmClient` standing in for
your own client. `Debug` is required; like `Ed25519Signer`, it shows only the
fingerprint, never key material or credentials:

```rust,ignore
use std::{fmt, future::Future, pin::Pin};

use canton_ledger::{Error, Result};
use canton_signer::{PublicKey, Signature, SignatureFormat, Signer, SigningAlgorithm};

struct HsmSigner {
    hsm: HsmClient,
    key_label: String,
    public_key: PublicKey,
    fingerprint: String,
}

impl fmt::Debug for HsmSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HsmSigner").field("fingerprint", &self.fingerprint).finish_non_exhaustive()
    }
}

impl Signer for HsmSigner {
    fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    fn sign(&self, hash: &[u8]) -> Pin<Box<dyn Future<Output = Result<Signature>> + Send + '_>> {
        let hash = hash.to_vec();
        Box::pin(async move {
            let bytes = self
                .hsm
                .sign_ed25519(&self.key_label, &hash)
                .await
                .map_err(|e| Error::Connection(format!("HSM signing failed: {e}")))?;
            Signature::new(
                SignatureFormat::Concat,
                bytes,
                self.fingerprint.clone(),
                SigningAlgorithm::Ed25519,
            )
        })
    }
}
```

## Recovering an ambiguous execute

The change ID (`user_id`, `act_as`, `command_id`) is fixed at preparation, so
executing the same `Executable` twice is de-duplicated rather than committed
twice, and the de-duplication covers ordinary submissions of the same command
too. Executing consumes the `Executable`, so take the change ID first, and the
offset to read completions from:

```rust,ignore
let begin = client.ledger_end().await?;
let executable = prepared.sign_with(&signer).await?;
let change_id = executable.change_id();
client.execute_submission(executable).await?;
let completion = client
    .await_completion(&change_id, begin, std::time::Duration::from_secs(60))
    .await?;
```

[`await_completion`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.CantonClient.html#method.await_completion)
matches on the whole
[`ChangeId`](https://docs.rs/canton-ledger/latest/canton_ledger/struct.ChangeId.html),
and returns `Error::CommandRejected` if the ledger rejected the transaction and
`Error::Timeout` if the completion does not arrive in time. With retry enabled,
a retried `execute_submission` answered with `ALREADY_EXISTS` is reported as
success, because the earlier attempt is the one that landed;
`execute_submission_and_wait` reports it as the rejection it is, so recover the
outcome with `await_completion`, as ordinary submissions do in the
[`recover_a_submission`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-ledger/examples/recover_a_submission.rs)
example.

## Running it against a participant

The live suite
[`interactive_live.rs`](https://github.com/nodejumper-org/canton-rust-sdk/blob/main/crates/canton-ledger/tests/interactive_live.rs)
runs this page against a real participant: it onboards a party, runs the round
trip under V2 and V3, creates a keyed contract and exercises it by key under
V3, and checks that a signature from the wrong key is rejected.

```sh
# unauthenticated LocalNet
CANTON_TEST_ENDPOINT=http://localhost:3901 \
  cargo test -p canton-ledger --test interactive_live -- --nocapture
```

Against a participant that authenticates, also set `CANTON_TEST_TOKEN_URL`,
`CANTON_TEST_ADMIN_CLIENT_ID` / `CANTON_TEST_ADMIN_CLIENT_SECRET` (a client with
`ParticipantAdmin`) and `CANTON_TEST_CLIENT_ID` / `CANTON_TEST_CLIENT_SECRET`.
The contract-key test needs `CANTON_TEST_CONTRACT_KEYS_PKG`, after uploading
`testdata/contract-keys/contractkeys.dar`. `CANTON_TEST_REQUIRE_LIVE=1` turns a
skip into a failure. [Local development](../local-development.md) covers
bringing up a participant.

The threat model is in the [security review scope](../reference/audit-scope.md),
and the Canton versions this is verified on are in the
[compatibility matrix](../reference/compatibility-matrix.md).
