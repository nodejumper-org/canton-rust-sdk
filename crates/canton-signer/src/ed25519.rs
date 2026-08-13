//! An Ed25519 key held in process memory.
//!
//! For development, tests, and deployments that accept a private key in the
//! application's address space. Production keys belong in an HSM or KMS behind
//! [`Signer`].

use std::fmt;
use std::future::Future;
use std::pin::Pin;

use canton_core::{Error, Result};
use ring::signature::{Ed25519KeyPair, KeyPair as _};

use crate::{KeyFormat, KeySpec, PublicKey, Signature, SignatureFormat, Signer, SigningAlgorithm};

/// The length of an Ed25519 seed, and of its public key.
const SEED_LEN: usize = 32;

/// An Ed25519 key pair, before the participant has told it who it is.
///
/// It can produce its public key — which is what
/// `GenerateExternalPartyTopology` needs — but it cannot sign for the ledger,
/// because a ledger signature carries a fingerprint only Canton issues. Call
/// [`into_signer`](Self::into_signer) with that fingerprint to get something
/// that can.
pub struct Ed25519Key {
    pair: Ed25519KeyPair,
}

impl fmt::Debug for Ed25519Key {
    /// Deliberately says nothing about the key.
    ///
    /// A private key that reaches a log through a `Debug` derive is a real way
    /// to lose one, and this type exists to hold private keys.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ed25519Key").finish_non_exhaustive()
    }
}

impl Ed25519Key {
    /// A fresh key from the system's randomness.
    ///
    /// # Errors
    /// If the platform's random source is unavailable.
    pub fn generate() -> Result<Self> {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).map_err(|_| {
            Error::InvalidRequest("the platform random source is unavailable".to_string())
        })?;
        Self::from_pkcs8(pkcs8.as_ref())
    }

    /// A key from its 32-byte seed — the private scalar, as RFC 8032 defines.
    ///
    /// # Errors
    /// If `seed` is not 32 bytes, or is not a valid Ed25519 seed.
    pub fn from_seed(seed: &[u8]) -> Result<Self> {
        if seed.len() != SEED_LEN {
            return Err(Error::InvalidRequest(format!(
                "an Ed25519 seed is {SEED_LEN} bytes, got {}",
                seed.len()
            )));
        }
        // `_unchecked` names the fact that this does not verify the seed
        // against a public key, because there is none to check against here.
        Ed25519KeyPair::from_seed_unchecked(seed)
            .map(|pair| Self { pair })
            .map_err(|_| Error::InvalidRequest("not a valid Ed25519 seed".to_string()))
    }

    /// A key from a PKCS#8 v2 document, as [`generate`](Self::generate)
    /// produces internally and as most tooling exports.
    ///
    /// # Errors
    /// If the document is not a PKCS#8 Ed25519 key pair.
    pub fn from_pkcs8(document: &[u8]) -> Result<Self> {
        Ed25519KeyPair::from_pkcs8(document)
            .map(|pair| Self { pair })
            .map_err(|_| Error::InvalidRequest("not a PKCS#8 Ed25519 key pair".to_string()))
    }

    /// A key from a file holding either a 32-byte raw seed or a PKCS#8
    /// document.
    ///
    /// Which one it is, is decided by length: a raw seed is exactly 32 bytes
    /// and a PKCS#8 document never is.
    ///
    /// # Errors
    /// If the file cannot be read, or holds neither form.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).map_err(|e| {
            Error::InvalidRequest(format!("cannot read the key at {}: {e}", path.display()))
        })?;
        if bytes.len() == SEED_LEN {
            Self::from_seed(&bytes)
        } else {
            Self::from_pkcs8(&bytes)
        }
    }

    /// The public key, in the raw 32-byte form Canton registers Curve25519
    /// keys in.
    #[must_use]
    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(
            KeyFormat::Raw,
            self.pair.public_key().as_ref().to_vec(),
            KeySpec::EcCurve25519,
        )
    }

    /// Pair this key with the fingerprint Canton assigned it, giving a
    /// [`Signer`].
    ///
    /// The fingerprint comes from `GenerateExternalPartyTopology`'s
    /// `public_key_fingerprint`, alongside the party id built from it. It is
    /// not computed here: Canton's fingerprint is its own hash of the key, and
    /// a value this crate invented would be one the participant rejects.
    #[must_use]
    pub fn into_signer(self, fingerprint: impl Into<String>) -> Ed25519Signer {
        Ed25519Signer {
            key: self,
            fingerprint: fingerprint.into(),
        }
    }

    /// Sign `hash` without a ledger identity — the raw 64-byte `r || s`.
    ///
    /// Present for signing something that is not a ledger submission. For a
    /// prepared transaction or an onboarding multi-hash, go through
    /// [`Signer`].
    #[must_use]
    pub fn sign_raw(&self, hash: &[u8]) -> Vec<u8> {
        self.pair.sign(hash).as_ref().to_vec()
    }
}

/// An [`Ed25519Key`] that knows which key Canton thinks it is.
pub struct Ed25519Signer {
    key: Ed25519Key,
    fingerprint: String,
}

impl fmt::Debug for Ed25519Signer {
    /// The fingerprint is public — it is in every signature — so it is shown.
    /// The key is not.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ed25519Signer")
            .field("fingerprint", &self.fingerprint)
            .finish_non_exhaustive()
    }
}

impl Ed25519Signer {
    /// The key this signs with.
    #[must_use]
    pub fn key(&self) -> &Ed25519Key {
        &self.key
    }
}

impl Signer for Ed25519Signer {
    fn public_key(&self) -> PublicKey {
        self.key.public_key()
    }

    fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    fn sign(&self, hash: &[u8]) -> Pin<Box<dyn Future<Output = Result<Signature>> + Send + '_>> {
        // Ed25519 signs the message it is given; the participant already
        // hashed the transaction, so this must not hash again.
        let signature = Signature::new(
            SignatureFormat::Concat,
            self.key.sign_raw(hash),
            self.fingerprint.clone(),
            SigningAlgorithm::Ed25519,
        );
        Box::pin(std::future::ready(Ok(signature)))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    const SEED: [u8; SEED_LEN] = [9; SEED_LEN];

    #[test]
    fn a_seed_gives_the_same_key_every_time() {
        let a = Ed25519Key::from_seed(&SEED).expect("valid seed");
        let b = Ed25519Key::from_seed(&SEED).expect("valid seed");
        assert_eq!(a.public_key(), b.public_key());
        assert_eq!(a.sign_raw(b"hash"), b.sign_raw(b"hash"));
    }

    #[test]
    fn a_generated_key_is_not_the_same_as_another() {
        let a = Ed25519Key::generate().expect("a random source");
        let b = Ed25519Key::generate().expect("a random source");
        assert_ne!(a.public_key(), b.public_key());
    }

    /// The signature has to verify against the public key that is handed to the
    /// participant — if it did not, everything downstream would fail with
    /// Canton's error message rather than this one.
    #[test]
    fn the_signature_verifies_against_the_public_key_that_is_registered() {
        let key = Ed25519Key::from_seed(&SEED).expect("valid seed");
        let public = key.public_key();
        let signature = key.sign_raw(b"a prepared transaction hash");

        ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public.data())
            .verify(b"a prepared transaction hash", &signature)
            .expect("the signature must verify against the registered key");
    }

    #[test]
    fn a_signature_over_another_hash_does_not_verify() {
        let key = Ed25519Key::from_seed(&SEED).expect("valid seed");
        let signature = key.sign_raw(b"one hash");
        assert!(
            ring::signature::UnparsedPublicKey::new(
                &ring::signature::ED25519,
                key.public_key().data()
            )
            .verify(b"another hash", &signature)
            .is_err()
        );
    }

    /// Canton registers Curve25519 keys as 32 raw bytes; a DER-wrapped key
    /// under a `Raw` label is a key the participant cannot parse.
    #[test]
    fn the_public_key_is_raw_and_thirty_two_bytes() {
        let key = Ed25519Key::from_seed(&SEED).expect("valid seed");
        let public = key.public_key();
        assert_eq!(public.format(), KeyFormat::Raw);
        assert_eq!(public.spec(), KeySpec::EcCurve25519);
        assert_eq!(public.data().len(), SEED_LEN);
    }

    /// Ed25519 signatures are `r || s`, 64 bytes. Declaring `Der` here would
    /// make Canton parse 64 raw bytes as ASN.1 and reject them.
    #[tokio::test]
    async fn a_ledger_signature_declares_concat_and_ed25519() {
        let signer = Ed25519Key::from_seed(&SEED)
            .expect("valid seed")
            .into_signer("1220deadbeef");
        let signature = signer.sign(b"hash").await.expect("in-memory signing");

        assert_eq!(signature.format(), SignatureFormat::Concat);
        assert_eq!(signature.algorithm(), SigningAlgorithm::Ed25519);
        assert_eq!(signature.bytes().len(), 64);
        assert_eq!(signature.signed_by(), "1220deadbeef");
    }

    /// The hash is signed as given. Hashing it again would produce a signature
    /// over something the participant never computed, which verifies nowhere.
    #[tokio::test]
    async fn the_hash_is_signed_as_given_and_not_hashed_again() {
        let key = Ed25519Key::from_seed(&SEED).expect("valid seed");
        let expected = key.sign_raw(b"the participant's hash");
        let signer = key.into_signer("1220deadbeef");
        let signature = signer.sign(b"the participant's hash").await.expect("sign");
        assert_eq!(signature.bytes(), expected.as_slice());
    }

    #[test]
    fn a_seed_of_the_wrong_length_is_refused() {
        let err = Ed25519Key::from_seed(&[1, 2, 3]).expect_err("too short");
        assert!(
            err.to_string().contains("32 bytes"),
            "the error should say what was expected: {err}"
        );
    }

    #[test]
    fn a_file_holding_either_form_is_read() {
        let dir = std::env::temp_dir().join("canton-signer-key-forms");
        std::fs::create_dir_all(&dir).expect("temp dir");

        let seed_path = dir.join("seed.bin");
        std::fs::write(&seed_path, SEED).expect("write");
        let from_seed = Ed25519Key::from_file(&seed_path).expect("a raw seed is read");
        assert_eq!(
            from_seed.public_key(),
            Ed25519Key::from_seed(&SEED).expect("seed").public_key()
        );

        let rng = ring::rand::SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).expect("generate");
        let pkcs8_path = dir.join("key.pk8");
        std::fs::write(&pkcs8_path, pkcs8.as_ref()).expect("write");
        let from_pkcs8 = Ed25519Key::from_file(&pkcs8_path).expect("a PKCS#8 document is read");
        assert_eq!(
            from_pkcs8.public_key(),
            Ed25519Key::from_pkcs8(pkcs8.as_ref())
                .expect("pkcs8")
                .public_key()
        );

        std::fs::remove_dir_all(&dir).ok();
    }

    /// A `Debug` derive on a type holding a private key is how one reaches a
    /// log. Neither of these may print key material.
    #[test]
    fn debug_does_not_print_the_key() {
        let key = Ed25519Key::from_seed(&SEED).expect("valid seed");
        let hex = |bytes: &[u8]| -> String {
            use std::fmt::Write as _;
            bytes.iter().fold(String::new(), |mut out, b| {
                let _ = write!(out, "{b:02x}");
                out
            })
        };
        let seed_hex = hex(&SEED);
        let public_hex = hex(key.public_key().data());

        let rendered = format!("{key:?}");
        assert!(!rendered.contains(&seed_hex), "{rendered}");
        assert!(!rendered.contains(&public_hex), "{rendered}");

        let signer = key.into_signer("1220deadbeef");
        let rendered = format!("{signer:?}");
        assert!(!rendered.contains(&seed_hex), "{rendered}");
        assert!(
            rendered.contains("1220deadbeef"),
            "the fingerprint is public and identifies the signer: {rendered}"
        );
    }
}
