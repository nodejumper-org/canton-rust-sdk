//! Pluggable transaction signing for Canton's interactive submission.
//!
//! Interactive submission splits a command in two: the participant *prepares* a
//! transaction and returns its hash, the party's key signs that hash, and the
//! signature comes back with an *execute* call. The participant never holds the
//! key — which is the point, and which is why the signing step has to be
//! somebody else's.
//!
//! [`Signer`] is that somebody. It is object-safe and its `sign` is async, so a
//! hardware module or a cloud KMS fits behind it without this crate knowing
//! anything about them:
//!
//! ```rust,ignore
//! let signer: Arc<dyn Signer> = Arc::new(my_kms_signer);
//! let prepared = client.prepare_submission(prepare).await?;
//! let executed = client.execute_submission(prepared.sign_with(&*signer).await?).await?;
//! ```
//!
//! The in-memory implementation, [`Ed25519Key`], is behind the default
//! `ed25519` feature. Turn it off (`default-features = false`) to take only the
//! trait.
//!
//! # A key is not yet an identity
//!
//! Canton addresses a key by a **fingerprint** it computes itself, and a
//! signature carries that fingerprint in `signed_by`. It is not derivable from
//! the key material here; a caller learns it from
//! `GenerateExternalPartyTopology`, which also yields the party id built from
//! it. So [`Ed25519Key`] holds key material and cannot sign for the ledger:
//! [`Ed25519Key::into_signer`] takes the fingerprint and returns something that
//! can. That ordering is the real one — you cannot sign as a party before the
//! participant has told you which party you are.
//!
//! # Security
//!
//! [`Ed25519Key`] holds a private key in process memory and is meant for
//! development, tests, and deployments where that is an accepted risk. It zeroes
//! nothing on drop and offers no protection against a process that can read its
//! own memory. Production keys belong in an HSM or KMS behind [`Signer`].

#![forbid(unsafe_code)]

#[cfg(feature = "ed25519")]
mod ed25519;
mod signature;

use std::fmt;
use std::future::Future;
use std::pin::Pin;

use canton_core::Result;

#[cfg(feature = "ed25519")]
pub use ed25519::{Ed25519Key, Ed25519Signer};
pub use signature::{KeyFormat, KeySpec, PublicKey, Signature, SignatureFormat, SigningAlgorithm};

/// Something that can sign a prepared transaction's hash on a party's behalf.
///
/// Object-safe, so it is used as `Arc<dyn Signer>` — the same shape as
/// `canton_core::TokenSource`, and for the same reason: the SDK holds one
/// without knowing which implementation it is.
///
/// `sign` is async because the interesting implementations are a network call
/// away. The in-memory one simply returns a ready future.
pub trait Signer: Send + Sync + fmt::Debug {
    /// The public key, in the shape the participant registers.
    fn public_key(&self) -> PublicKey;

    /// The fingerprint Canton assigned to this key — a signature's `signed_by`.
    fn fingerprint(&self) -> &str;

    /// Sign `hash`, which is a participant-produced hash and **not** a message
    /// to be hashed again.
    ///
    /// For `prepare_submission` this is `prepared_transaction_hash`; for
    /// external party allocation it is the response's `multi_hash`.
    fn sign(&self, hash: &[u8]) -> Pin<Box<dyn Future<Output = Result<Signature>> + Send + '_>>;
}

/// A `Signer` shared by reference signs exactly as the thing it points at.
///
/// Without this, `Arc<dyn Signer>` satisfies a `&dyn Signer` parameter but not
/// an `impl Signer` one, and callers end up writing `&*signer` at each site.
impl<T: Signer + ?Sized> Signer for &T {
    fn public_key(&self) -> PublicKey {
        (**self).public_key()
    }

    fn fingerprint(&self) -> &str {
        (**self).fingerprint()
    }

    fn sign(&self, hash: &[u8]) -> Pin<Box<dyn Future<Output = Result<Signature>> + Send + '_>> {
        (**self).sign(hash)
    }
}

impl<T: Signer + ?Sized> Signer for std::sync::Arc<T> {
    fn public_key(&self) -> PublicKey {
        (**self).public_key()
    }

    fn fingerprint(&self) -> &str {
        (**self).fingerprint()
    }

    fn sign(&self, hash: &[u8]) -> Pin<Box<dyn Future<Output = Result<Signature>> + Send + '_>> {
        (**self).sign(hash)
    }
}
