# canton-signer

Pluggable transaction signing for Canton's **interactive submission** —
HSM/KMS-compatible.

Part of the [Canton Rust SDK](https://github.com/nodejumper-org/canton-rust-sdk).

## What it is for

Interactive submission splits a command in two. The participant *prepares* a
transaction and returns its hash; the party's key signs that hash; the signature
comes back with an *execute* call. The participant never holds the key — which
is the point, and which is why the signing step has to be somebody else's.

`Signer` is that somebody:

```rust,ignore
pub trait Signer: Send + Sync + Debug {
    fn public_key(&self) -> PublicKey;
    fn fingerprint(&self) -> &str;
    fn sign(&self, hash: &[u8]) -> BoxFuture<'_, Result<Signature>>;
}
```

It is object-safe, so the SDK holds an `Arc<dyn Signer>` without knowing which
implementation it is — the same shape as `canton_core::TokenSource`. `sign` is
async because the interesting implementations are a network call away; the
in-memory one returns a ready future.

## A key is not yet an identity

Canton addresses a key by a **fingerprint it computes itself**, and every
signature carries that fingerprint in `signed_by`. It is not derivable from key
material: you learn it from `GenerateExternalPartyTopology`, which also returns
the party id built from it.

So `Ed25519Key` holds key material and *cannot* sign for the ledger.
`into_signer(fingerprint)` returns something that can. The order that enforces
is the real one — you cannot sign as a party before the participant has told
you which party you are:

```rust,ignore
let key = Ed25519Key::generate()?;
let topology = admin
    .generate_external_party_topology(synchronizer, "alice", &key.public_key())
    .await?;

let signer = key.into_signer(&topology.public_key_fingerprint);
let party = admin.allocate_external_party(synchronizer, &topology, &signer).await?;

let prepared = client.prepare_submission(Prepare::new(&party).add_command(cmd)).await?;
let tx = client.execute_submission_and_wait_for_transaction(
    prepared.sign_with(&signer).await?,
).await?;
```

## Features

| Feature | Default | What it adds |
|---|---|---|
| `ed25519` | on | `Ed25519Key` / `Ed25519Signer`, an in-memory key built on `ring` (already in this workspace's tree via rustls, so no second crypto stack). |

`default-features = false` brings only the trait and the signature types — what
an HSM or KMS implementation needs.

## Security

`Ed25519Key` holds a private key in process memory. It is meant for
development, tests, and deployments where that is an accepted risk: it zeroes
nothing on drop and offers no protection against a process that can read its own
memory. Its `Debug` deliberately prints no key material, since a `Debug` derive
on a key type is a real way to leak one.

Production keys belong in an HSM or a KMS behind `Signer`.

## Licence

Apache-2.0. See `LICENSE`.
