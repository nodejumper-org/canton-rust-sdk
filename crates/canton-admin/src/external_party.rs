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
        if response.topology_transactions.is_empty() {
            return Err(canton_core::Error::UnexpectedResponse(
                "the participant returned no onboarding transactions, so there would be \
                 nothing to submit and the multi-hash would cover nothing"
                    .to_string(),
            ));
        }
        // A party id is `<hint>::<namespace>`, and for an external party the
        // namespace *is* the fingerprint of the key being registered. The SDK
        // cannot yet recompute the multi-hash — that needs Canton's
        // hash-purpose scheme, which this workspace does not implement — so
        // this is the one relation between the returned pieces that can be
        // checked at all, and it is free.
        //
        // What it catches is the ordinary case rather than the adversarial
        // one: a response assembled for a different key, or a participant that
        // onboarded a party under a namespace the caller does not hold. The
        // caller would otherwise learn this by signing, submitting, and being
        // told the signature is invalid, with nothing pointing at why.
        match response.party_id.rsplit_once("::") {
            Some((_, namespace)) if namespace == response.public_key_fingerprint => {}
            Some((_, namespace)) => {
                return Err(canton_core::Error::UnexpectedResponse(format!(
                    "the participant offered party `{}`, whose namespace `{namespace}` is not the \
                     fingerprint `{}` of the key being registered — nothing this caller holds \
                     could sign as it",
                    response.party_id, response.public_key_fingerprint
                )));
            }
            None => {
                return Err(canton_core::Error::UnexpectedResponse(format!(
                    "the participant returned `{}`, which is not a party id",
                    response.party_id
                )));
            }
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::ExternalPartyTopology;
    use canton_proto::com::daml::ledger::api::v2::admin as pb;

    fn response(party_id: &str, fingerprint: &str) -> pb::GenerateExternalPartyTopologyResponse {
        pb::GenerateExternalPartyTopologyResponse {
            party_id: party_id.to_string(),
            public_key_fingerprint: fingerprint.to_string(),
            topology_transactions: vec![vec![1, 2, 3]],
            multi_hash: vec![0x12, 0x20, 0xaa],
        }
    }

    /// The ordinary case: the namespace of the party id is the fingerprint of
    /// the key being registered, so this caller can sign as it.
    #[test]
    fn a_party_whose_namespace_is_the_registered_key_is_accepted() {
        let topology = ExternalPartyTopology::from_response(response("alice::1220ab", "1220ab"))
            .expect("the namespace matches the key");
        assert_eq!(topology.party_id(), "alice::1220ab");
        assert_eq!(topology.public_key_fingerprint(), "1220ab");
    }

    /// And the case worth catching: a party under a namespace this caller's key
    /// does not control. Signing and submitting would be the only other way to
    /// find out, and the participant's answer would name the signature rather
    /// than the mismatch.
    #[test]
    fn a_party_under_a_namespace_this_key_does_not_control_is_refused() {
        let error = ExternalPartyTopology::from_response(response("alice::1220ff", "1220ab"))
            .expect_err("the namespace is another key's");
        let message = error.to_string();
        assert!(message.contains("1220ff"), "name the namespace: {message}");
        assert!(message.contains("1220ab"), "and the key: {message}");
    }

    /// Something that is not a party id at all.
    #[test]
    fn a_response_that_is_not_a_party_id_is_refused() {
        let error = ExternalPartyTopology::from_response(response("alice", "1220ab"))
            .expect_err("`alice` is not a party id");
        assert!(error.to_string().contains("not a party id"), "{error}");
    }
}
