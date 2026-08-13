//! Allocating a party whose key the participant does not hold.
//!
//! An ordinary party is allocated in one call and the participant signs for it
//! from then on. An *external* party keeps its key elsewhere — which is the
//! whole point of interactive submission — so allocation is two calls with a
//! signature in between:
//!
//! 1. [`generate_external_party_topology`] hands the participant a public key.
//!    It replies with the party id it would assign, the **fingerprint** it
//!    computed for the key, the onboarding topology transactions, and one hash
//!    over all of them.
//! 2. The key signs that hash — proving the party controls it — and
//!    [`allocate_external_party`] submits the transactions with the signature.
//!
//! The fingerprint from step 1 is what makes a `canton_signer::Ed25519Key` into
//! a `Signer`: Canton computes it, so a caller cannot know it before asking.
//!
//! [`generate_external_party_topology`]: crate::AdminClient::generate_external_party_topology
//! [`allocate_external_party`]: crate::AdminClient::allocate_external_party

use canton_proto::com::daml::ledger::api::v2::admin as pb;

/// What the participant will do with an external party, once it is asked to.
///
/// The result of [`generate_external_party_topology`], and the input to
/// [`allocate_external_party`]. Nothing has been submitted yet: this is a
/// proposal, and it becomes real only when the key signs
/// [`multi_hash`](Self::multi_hash).
///
/// [`generate_external_party_topology`]: crate::AdminClient::generate_external_party_topology
/// [`allocate_external_party`]: crate::AdminClient::allocate_external_party
#[derive(Clone, Debug)]
pub struct ExternalPartyTopology {
    party_id: String,
    public_key_fingerprint: String,
    transactions: Vec<Vec<u8>>,
    multi_hash: Vec<u8>,
}

impl ExternalPartyTopology {
    pub(crate) fn from_response(
        response: pb::GenerateExternalPartyTopologyResponse,
    ) -> canton_core::Result<Self> {
        if response.multi_hash.is_empty() {
            return Err(canton_core::Error::UnexpectedResponse(
                "the participant returned no multi-hash to sign".to_string(),
            ));
        }
        if response.public_key_fingerprint.is_empty() {
            return Err(canton_core::Error::UnexpectedResponse(
                "the participant returned no key fingerprint, so nothing could \
                 sign as this party"
                    .to_string(),
            ));
        }
        Ok(Self {
            party_id: response.party_id,
            public_key_fingerprint: response.public_key_fingerprint,
            transactions: response.topology_transactions,
            multi_hash: response.multi_hash,
        })
    }

    /// The party id the participant will assign — `<hint>::<namespace>`.
    ///
    /// Known before allocation because it is derived from the key, so a caller
    /// can prepare against it while the signature is still being obtained.
    #[must_use]
    pub fn party_id(&self) -> &str {
        &self.party_id
    }

    /// The fingerprint Canton computed for the key.
    ///
    /// This is what a signature carries in `signed_by`, and what
    /// `canton_signer::Ed25519Key::into_signer` needs.
    #[must_use]
    pub fn public_key_fingerprint(&self) -> &str {
        &self.public_key_fingerprint
    }

    /// The hash to sign: one hash over every onboarding transaction, so one
    /// signature authorizes them all.
    #[must_use]
    pub fn multi_hash(&self) -> &[u8] {
        &self.multi_hash
    }

    /// The serialized onboarding topology transactions.
    ///
    /// Exposed for a caller that wants to inspect what it is about to
    /// authorize; [`allocate_external_party`] submits them.
    ///
    /// [`allocate_external_party`]: crate::AdminClient::allocate_external_party
    #[must_use]
    pub fn transactions(&self) -> &[Vec<u8>] {
        &self.transactions
    }
}
