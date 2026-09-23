//! What `cargo add canton` actually delivers.
//!
//! Separate from `conformance.rs` on purpose: that file is exactly one test per
//! row of the Ledger Client Standard, and CI asserts the count matches the
//! registry exactly. A test that is not a capability row does not belong in it.
//!
//! What these assert is the rule `conformance.rs` states and cannot itself
//! check — that a capability reachable only through a member crate, or only
//! under a feature combination nobody can select, is not one the facade
//! delivers. Every entry here is a defect that was found, not a hypothetical.
#![allow(clippy::unwrap_used, clippy::expect_used)]
/// The HSM/KMS build is a configuration the proposal names, so it has to be
/// reachable — and it was not. Three crates took `canton-signer` at its
/// defaults, and feature unification is additive, so `default-features = false`
/// on the facade did nothing: `ring` and the in-memory private-key type came
/// back through `canton-ledger`, which is not optional because
/// `prepare_submission` lives there.
///
/// Named without `__` so the completeness guard does not read it as a claim on
/// a capability row.
#[test]
fn the_signer_trait_is_usable_without_the_in_memory_key() {
    // Everything an HSM caller needs is unconditional: the trait, the wire
    // types, and the validation on them.
    let _: fn(
        canton::signer::SignatureFormat,
        Vec<u8>,
        &str,
        canton::signer::SigningAlgorithm,
    ) -> canton::Result<canton::signer::Signature> =
        |f, b, s, a| canton::signer::Signature::new(f, b, s, a);
    let _ = canton::signer::PublicKey::new;
    // And the length guard holds with the feature off — it is a property of
    // the algorithm, not of this crate's implementation of it.
    assert!(
        canton::signer::Signature::new(
            canton::signer::SignatureFormat::Concat,
            vec![0; 8],
            "1220ab",
            canton::signer::SigningAlgorithm::Ed25519,
        )
        .is_err(),
        "an 8-byte Ed25519 concat signature must be refused"
    );
}
