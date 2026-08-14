//! What a signature is on the wire, and what a key is.
//!
//! These mirror the Ledger API's `crypto.proto` rather than re-using its
//! generated types, so that implementing [`Signer`](crate::Signer) for an HSM
//! does not mean matching on protobuf enums. The conversion is written once,
//! at the bottom of this file, and a test asserts each variant maps to the
//! proto variant it claims to.

use canton_proto::com::daml::ledger::api::v2 as pb;

/// The algorithm a signature was produced with.
///
/// Mirrors `SigningAlgorithmSpec`. Canton verifies against the algorithm named
/// here, so it must be the one actually used — not the one the key *could*
/// have used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SigningAlgorithm {
    /// EdDSA over Curve25519 with SHA-512 — what [`Ed25519Key`] produces.
    ///
    /// [`Ed25519Key`]: crate::Ed25519Key
    Ed25519,
    /// ECDSA with SHA-256.
    EcDsaSha256,
    /// ECDSA with SHA-384.
    EcDsaSha384,
    /// ML-DSA-65. `KeySpec` could already name this key; without the matching
    /// algorithm a signer holding one had to declare a signature it did not
    /// produce.
    MlDsa65,
}

/// How a signature's bytes are laid out.
///
/// Mirrors `SignatureFormat`. This is not cosmetic: Canton parses the bytes
/// according to it, and `Concat` for an ECDSA signature (or `Der` for an EdDSA
/// one) fails verification rather than being reinterpreted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SignatureFormat {
    /// `r || s`, as RFC 8032 §3.3 defines for EdDSA. 64 bytes for Ed25519.
    Concat,
    /// ASN.1 + DER encoding of `r` and `s`, as ECDSA uses.
    Der,
}

/// How a public key's bytes are laid out.
///
/// Mirrors `CryptoKeyFormat`, minus the formats Canton keeps only for
/// migration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum KeyFormat {
    /// The raw key — 32 bytes for Ed25519.
    Raw,
    /// ASN.1 + DER of an X.509 `SubjectPublicKeyInfo` (RFC 5280 §4.1).
    DerX509SubjectPublicKeyInfo,
}

/// Which curve or scheme a key belongs to.
///
/// Mirrors `SigningKeySpec`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum KeySpec {
    /// Curve25519.
    EcCurve25519,
    /// NIST P-256 (secp256r1).
    EcP256,
    /// NIST P-384 (secp384r1).
    EcP384,
    /// SECG secp256k1.
    EcSecp256k1,
    /// ML-DSA-65.
    MlDsa65,
}

/// A public key, in the shape the participant registers it in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicKey {
    format: KeyFormat,
    data: Vec<u8>,
    spec: KeySpec,
}

impl PublicKey {
    /// A key from its parts.
    #[must_use]
    pub fn new(format: KeyFormat, data: Vec<u8>, spec: KeySpec) -> Self {
        Self { format, data, spec }
    }

    /// How [`data`](Self::data) is encoded.
    #[must_use]
    pub fn format(&self) -> KeyFormat {
        self.format
    }

    /// The key bytes.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Which curve or scheme the key belongs to.
    #[must_use]
    pub fn spec(&self) -> KeySpec {
        self.spec
    }
}

/// A signature over a hash the participant produced.
///
/// `signed_by` is the **fingerprint** the participant assigned to the key, not
/// anything derivable from the key material here: Canton computes it, and
/// `GenerateExternalPartyTopology` is where a caller learns it. Sending a
/// fingerprint Canton did not issue fails verification, which is why
/// [`Ed25519Key`](crate::Ed25519Key) cannot become a [`Signer`](crate::Signer)
/// until it has one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    format: SignatureFormat,
    bytes: Vec<u8>,
    signed_by: String,
    algorithm: SigningAlgorithm,
}

/// The length of an Ed25519 signature in `r || s` form.
///
/// A property of the algorithm, so it is defined here rather than beside the
/// in-memory implementation — the validation in [`Signature::new`] has to hold
/// for a caller who took only the trait.
pub(crate) const ED25519_SIGNATURE_LEN: usize = 64;

impl Signature {
    /// A signature from its parts.
    ///
    /// # Errors
    /// [`canton_core::Error::InvalidRequest`] if the bytes cannot be what they claim: empty
    /// — which the Ledger API marks as required non-empty — or the wrong length
    /// for an Ed25519 `Concat` signature. A truncated reply from a signing
    /// service is worth catching here rather than as a verification failure
    /// with nothing to point at.
    pub fn new(
        format: SignatureFormat,
        bytes: Vec<u8>,
        signed_by: impl Into<String>,
        algorithm: SigningAlgorithm,
    ) -> canton_core::Result<Self> {
        if bytes.is_empty() {
            return Err(canton_core::Error::InvalidRequest(
                "a signature cannot be empty".to_string(),
            ));
        }
        // Deliberately *not* behind the `ed25519` feature. The length of an
        // Ed25519 signature is a property of the algorithm, not of this crate's
        // in-memory implementation of it — and the case this check exists for
        // is a signing service returning a truncated or DER-shaped reply, which
        // is precisely what an HSM caller building with
        // `default-features = false` would hit. Gating it on the feature
        // removed it from the only configuration that needed it.
        if algorithm == SigningAlgorithm::Ed25519
            && format == SignatureFormat::Concat
            && bytes.len() != ED25519_SIGNATURE_LEN
        {
            return Err(canton_core::Error::InvalidRequest(format!(
                "an Ed25519 `r || s` signature is {ED25519_SIGNATURE_LEN} bytes, got {}",
                bytes.len()
            )));
        }
        Ok(Self {
            format,
            bytes,
            signed_by: signed_by.into(),
            algorithm,
        })
    }

    /// How [`bytes`](Self::bytes) is laid out.
    #[must_use]
    pub fn format(&self) -> SignatureFormat {
        self.format
    }

    /// The signature bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The fingerprint of the key that produced it, as Canton assigned it.
    #[must_use]
    pub fn signed_by(&self) -> &str {
        &self.signed_by
    }

    /// The algorithm it was produced with.
    #[must_use]
    pub fn algorithm(&self) -> SigningAlgorithm {
        self.algorithm
    }
}

impl From<SigningAlgorithm> for pb::SigningAlgorithmSpec {
    fn from(value: SigningAlgorithm) -> Self {
        match value {
            SigningAlgorithm::Ed25519 => Self::Ed25519,
            SigningAlgorithm::EcDsaSha256 => Self::EcDsaSha256,
            SigningAlgorithm::EcDsaSha384 => Self::EcDsaSha384,
            SigningAlgorithm::MlDsa65 => Self::MlDsa65,
        }
    }
}

impl From<SignatureFormat> for pb::SignatureFormat {
    fn from(value: SignatureFormat) -> Self {
        match value {
            SignatureFormat::Concat => Self::Concat,
            SignatureFormat::Der => Self::Der,
        }
    }
}

impl From<KeyFormat> for pb::CryptoKeyFormat {
    fn from(value: KeyFormat) -> Self {
        match value {
            KeyFormat::Raw => Self::Raw,
            KeyFormat::DerX509SubjectPublicKeyInfo => Self::DerX509SubjectPublicKeyInfo,
        }
    }
}

impl From<KeySpec> for pb::SigningKeySpec {
    fn from(value: KeySpec) -> Self {
        match value {
            KeySpec::EcCurve25519 => Self::EcCurve25519,
            KeySpec::EcP256 => Self::EcP256,
            KeySpec::EcP384 => Self::EcP384,
            KeySpec::EcSecp256k1 => Self::EcSecp256k1,
            KeySpec::MlDsa65 => Self::MlDsa65,
        }
    }
}

impl From<&PublicKey> for pb::SigningPublicKey {
    fn from(value: &PublicKey) -> Self {
        Self {
            format: pb::CryptoKeyFormat::from(value.format) as i32,
            key_data: value.data.clone(),
            key_spec: pb::SigningKeySpec::from(value.spec) as i32,
        }
    }
}

impl From<&Signature> for pb::Signature {
    fn from(value: &Signature) -> Self {
        Self {
            format: pb::SignatureFormat::from(value.format) as i32,
            signature: value.bytes.clone(),
            signed_by: value.signed_by.clone(),
            signing_algorithm_spec: pb::SigningAlgorithmSpec::from(value.algorithm) as i32,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// The mirror types exist so an HSM implementation never sees a protobuf
    /// enum. That only holds up if each variant maps to the one it claims to —
    /// a wrong mapping here is a signature Canton rejects, with nothing in the
    /// Rust types to suggest why.
    #[test]
    fn each_variant_maps_to_the_proto_variant_it_names() {
        assert_eq!(
            pb::SigningAlgorithmSpec::from(SigningAlgorithm::Ed25519),
            pb::SigningAlgorithmSpec::Ed25519
        );
        assert_eq!(
            pb::SigningAlgorithmSpec::from(SigningAlgorithm::EcDsaSha256),
            pb::SigningAlgorithmSpec::EcDsaSha256
        );
        assert_eq!(
            pb::SigningAlgorithmSpec::from(SigningAlgorithm::EcDsaSha384),
            pb::SigningAlgorithmSpec::EcDsaSha384
        );
        assert_eq!(
            pb::SigningAlgorithmSpec::from(SigningAlgorithm::MlDsa65),
            pb::SigningAlgorithmSpec::MlDsa65
        );
        assert_eq!(
            pb::SignatureFormat::from(SignatureFormat::Concat),
            pb::SignatureFormat::Concat
        );
        assert_eq!(
            pb::SignatureFormat::from(SignatureFormat::Der),
            pb::SignatureFormat::Der
        );
        assert_eq!(
            pb::CryptoKeyFormat::from(KeyFormat::Raw),
            pb::CryptoKeyFormat::Raw
        );
        assert_eq!(
            pb::CryptoKeyFormat::from(KeyFormat::DerX509SubjectPublicKeyInfo),
            pb::CryptoKeyFormat::DerX509SubjectPublicKeyInfo
        );
        assert_eq!(
            pb::SigningKeySpec::from(KeySpec::EcCurve25519),
            pb::SigningKeySpec::EcCurve25519
        );
        assert_eq!(
            pb::SigningKeySpec::from(KeySpec::EcP256),
            pb::SigningKeySpec::EcP256
        );
        assert_eq!(
            pb::SigningKeySpec::from(KeySpec::EcP384),
            pb::SigningKeySpec::EcP384
        );
        assert_eq!(
            pb::SigningKeySpec::from(KeySpec::EcSecp256k1),
            pb::SigningKeySpec::EcSecp256k1
        );
        assert_eq!(
            pb::SigningKeySpec::from(KeySpec::MlDsa65),
            pb::SigningKeySpec::MlDsa65
        );
    }

    /// None of them may land on the proto's zero value: `…_UNSPECIFIED` is what
    /// an unset field decodes to, so a mapping that produced it would be
    /// indistinguishable from having sent nothing at all.
    #[test]
    fn no_variant_maps_to_unspecified() {
        for algorithm in [
            SigningAlgorithm::Ed25519,
            SigningAlgorithm::EcDsaSha256,
            SigningAlgorithm::EcDsaSha384,
            SigningAlgorithm::MlDsa65,
        ] {
            assert_ne!(
                pb::SigningAlgorithmSpec::from(algorithm),
                pb::SigningAlgorithmSpec::Unspecified,
                "{algorithm:?}"
            );
        }
        for format in [SignatureFormat::Concat, SignatureFormat::Der] {
            assert_ne!(
                pb::SignatureFormat::from(format),
                pb::SignatureFormat::Unspecified,
                "{format:?}"
            );
        }
        for format in [KeyFormat::Raw, KeyFormat::DerX509SubjectPublicKeyInfo] {
            assert_ne!(
                pb::CryptoKeyFormat::from(format),
                pb::CryptoKeyFormat::Unspecified,
                "{format:?}"
            );
        }
        for spec in [
            KeySpec::EcCurve25519,
            KeySpec::EcP256,
            KeySpec::EcP384,
            KeySpec::EcSecp256k1,
            KeySpec::MlDsa65,
        ] {
            assert_ne!(
                pb::SigningKeySpec::from(spec),
                pb::SigningKeySpec::Unspecified,
                "{spec:?}"
            );
        }
    }

    /// The wire form carries the enums as `i32`, and a field left at its
    /// default is the same as unset — so the built message must not be one.
    #[test]
    fn the_wire_form_carries_every_field() {
        let signature = Signature::new(
            SignatureFormat::Concat,
            vec![7; 64],
            "1220abcd",
            SigningAlgorithm::Ed25519,
        )
        .expect("a well-formed signature");
        let wire = pb::Signature::from(&signature);
        assert_eq!(wire.signature, vec![7; 64]);
        assert_eq!(wire.signed_by, "1220abcd");
        assert_ne!(wire.format, 0, "format left unset");
        assert_ne!(wire.signing_algorithm_spec, 0, "algorithm left unset");

        let key = PublicKey::new(KeyFormat::Raw, vec![3; 32], KeySpec::EcCurve25519);
        let wire = pb::SigningPublicKey::from(&key);
        assert_eq!(wire.key_data, vec![3; 32]);
        assert_ne!(wire.format, 0, "key format left unset");
        assert_ne!(wire.key_spec, 0, "key spec left unset");
    }

    /// A signature that cannot be what it claims never reaches the wire: the
    /// Ledger API marks the bytes required non-empty, and a truncated reply
    /// from a signing service is better caught here than as a verification
    /// failure with nothing to point at.
    #[test]
    fn a_signature_that_cannot_be_what_it_claims_is_refused() {
        let err = Signature::new(
            SignatureFormat::Concat,
            Vec::new(),
            "1220abcd",
            SigningAlgorithm::Ed25519,
        )
        .expect_err("empty");
        assert!(err.to_string().contains("cannot be empty"), "{err}");

        let err = Signature::new(
            SignatureFormat::Concat,
            vec![7; 63],
            "1220abcd",
            SigningAlgorithm::Ed25519,
        )
        .expect_err("truncated");
        assert!(err.to_string().contains("64 bytes, got 63"), "{err}");

        // An ECDSA signature is DER and varies in length, so the check applies
        // only where a length is actually fixed.
        Signature::new(
            SignatureFormat::Der,
            vec![7; 63],
            "1220abcd",
            SigningAlgorithm::EcDsaSha256,
        )
        .expect("a DER signature has no fixed length");
    }
}
