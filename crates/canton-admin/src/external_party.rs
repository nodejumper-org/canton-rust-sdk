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
use canton_proto::com::digitalasset::canton::crypto::v30 as crypto;
use canton_proto::com::digitalasset::canton::protocol::v30 as topo;
use canton_signer::PublicKey;
use prost::Message as _;
use sha2::{Digest as _, Sha256};

/// Canton's `UntypedVersionedMessage`, the envelope every serialized topology
/// transaction arrives in: the transaction bytes and the protocol version they
/// were written under. Two fields, so it is declared here rather than vendored.
#[derive(Clone, PartialEq, prost::Message)]
struct UntypedVersionedMessage {
    #[prost(bytes = "vec", tag = "1")]
    data: Vec<u8>,
    #[prost(int32, tag = "2")]
    version: i32,
}

/// Canton's `HashPurpose.TopologyTransactionSignature`: the purpose a single
/// topology transaction is hashed under.
const HASH_PURPOSE_TOPOLOGY_TRANSACTION_SIGNATURE: u32 = 11;
/// Canton's `HashPurpose.MultiTopologyTransaction`: the purpose the hash over
/// a set of transactions is built under, which is what one signature covers.
const HASH_PURPOSE_MULTI_TOPOLOGY_TRANSACTION: u32 = 55;
/// A Canton hash on the wire is a multihash: SHA-256's code and length, then
/// the digest.
const MULTIHASH_SHA256_PREFIX: [u8; 2] = [0x12, 0x20];

fn evidence(digest: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(MULTIHASH_SHA256_PREFIX.len() + digest.len());
    out.extend_from_slice(&MULTIHASH_SHA256_PREFIX);
    out.extend_from_slice(digest);
    out
}

/// The hash of one serialized topology transaction, as Canton computes it:
/// `SHA-256(purpose ‖ bytes)`, multihash-encoded.
fn transaction_hash(serialized: &[u8]) -> Vec<u8> {
    let mut digest = Sha256::new();
    digest.update(HASH_PURPOSE_TOPOLOGY_TRANSACTION_SIGNATURE.to_be_bytes());
    digest.update(serialized);
    evidence(&digest.finalize())
}

/// The hash one signature covers for a set of transactions, as Canton's
/// `MultiTransactionSignature.computeCombinedHash` builds it: the purpose,
/// the number of transactions, then each transaction's hash (length-prefixed)
/// in ascending order. This is what the participant returns as `multi_hash`,
/// and recomputing it is what ties the signature to the transactions the
/// caller can read rather than to whatever the participant chose to hash.
pub(crate) fn multi_hash(transactions: &[Vec<u8>]) -> Vec<u8> {
    let mut covered: Vec<Vec<u8>> = transactions.iter().map(|t| transaction_hash(t)).collect();
    covered.sort();
    covered.dedup();
    let mut digest = Sha256::new();
    digest.update(HASH_PURPOSE_MULTI_TOPOLOGY_TRANSACTION.to_be_bytes());
    digest.update(
        u32::try_from(covered.len())
            .unwrap_or(u32::MAX)
            .to_be_bytes(),
    );
    for hash in &covered {
        digest.update(u32::try_from(hash.len()).unwrap_or(u32::MAX).to_be_bytes());
        digest.update(hash);
    }
    evidence(&digest.finalize())
}

fn decode_transaction(serialized: &[u8]) -> canton_core::Result<topo::TopologyTransaction> {
    let envelope = UntypedVersionedMessage::decode(serialized).map_err(|e| {
        canton_core::Error::UnexpectedResponse(format!(
            "an onboarding transaction is not a versioned message: {e}"
        ))
    })?;
    topo::TopologyTransaction::decode(envelope.data.as_slice()).map_err(|e| {
        canton_core::Error::UnexpectedResponse(format!(
            "an onboarding transaction (protocol version {}) does not decode: {e}",
            envelope.version
        ))
    })
}

fn refuse(what: impl std::fmt::Display) -> canton_core::Error {
    canton_core::Error::UnexpectedResponse(format!(
        "the participant's onboarding transactions would authorize more than this party's \
         own key: {what}; refusing to sign them"
    ))
}

fn is_the_registered_key(candidate: Option<&crypto::SigningPublicKey>, key: &PublicKey) -> bool {
    let Some(candidate) = candidate else {
        return false;
    };
    let format_matches = match key.format() {
        canton_signer::KeyFormat::Raw => candidate.format == crypto::CryptoKeyFormat::Raw as i32,
        // Any other format: the bytes decide.
        _ => true,
    };
    format_matches && candidate.public_key == key.data()
}

/// What an onboarding transaction may say, and nothing else: the key's own
/// namespace delegated to the key itself, the party bound to that one key, and
/// the party hosted for confirmation (never submission, which would let the
/// host act as the party without a signature).
#[allow(deprecated)]
fn check_transaction(
    transaction: &topo::TopologyTransaction,
    party_id: &str,
    fingerprint: &str,
    key: &PublicKey,
) -> canton_core::Result<()> {
    use topo::topology_mapping::Mapping;
    if transaction.operation != topo::enums::TopologyChangeOp::AddReplace as i32 {
        return Err(refuse(format!(
            "a transaction with operation {} rather than an addition",
            transaction.operation
        )));
    }
    let mapping = transaction
        .mapping
        .as_ref()
        .and_then(|m| m.mapping.as_ref())
        .ok_or_else(|| refuse("a transaction without a mapping"))?;
    match mapping {
        Mapping::NamespaceDelegation(delegation) => {
            if delegation.namespace != fingerprint {
                return Err(refuse(format!(
                    "a namespace delegation for `{}`, not this key's namespace `{fingerprint}`",
                    delegation.namespace
                )));
            }
            if !is_the_registered_key(delegation.target_key.as_ref(), key) {
                return Err(refuse(
                    "a namespace delegation to a key other than the one being registered",
                ));
            }
        }
        Mapping::PartyToKeyMapping(mapping) => {
            if mapping.party != party_id {
                return Err(refuse(format!(
                    "a key mapping for `{}`, not `{party_id}`",
                    mapping.party
                )));
            }
            if mapping.signing_keys.len() != 1
                || !is_the_registered_key(mapping.signing_keys.first(), key)
            {
                return Err(refuse(
                    "a key mapping naming a signing key other than the one being registered",
                ));
            }
            if mapping.threshold > 1 {
                return Err(refuse(format!(
                    "a key mapping with threshold {}",
                    mapping.threshold
                )));
            }
        }
        Mapping::PartyToParticipant(hosting) => {
            if hosting.party != party_id {
                return Err(refuse(format!(
                    "a hosting mapping for `{}`, not `{party_id}`",
                    hosting.party
                )));
            }
            if hosting.threshold > 1 {
                return Err(refuse(format!(
                    "a hosting mapping with threshold {}",
                    hosting.threshold
                )));
            }
            for host in &hosting.participants {
                let allowed = host.permission
                    == topo::enums::ParticipantPermission::Confirmation as i32
                    || host.permission == topo::enums::ParticipantPermission::Observation as i32;
                if !allowed {
                    return Err(refuse(format!(
                        "participant `{}` hosting the party with permission {} (only \
                         confirmation or observation is a hosted external party)",
                        host.participant_uid, host.permission
                    )));
                }
            }
        }
        _ => return Err(refuse("a mapping of a kind onboarding does not need")),
    }
    Ok(())
}

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
        public_key: &PublicKey,
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
        // namespace *is* the fingerprint of the key being registered.
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
        // The signature will cover `multi_hash`, so what the hash covers is
        // what the key authorizes. Recomputed from the transactions the caller
        // can read, and each of those read: a participant that hashed a
        // different set, or slipped a mapping of its own into this one (a
        // second host with submission rights, a delegation to its own key),
        // is not signed for.
        let recomputed = multi_hash(&response.topology_transactions);
        if recomputed != response.multi_hash {
            return Err(canton_core::Error::UnexpectedResponse(
                "the participant's multi-hash is not the hash of the onboarding transactions \
                 it returned; refusing to sign it"
                    .to_string(),
            ));
        }
        for serialized in &response.topology_transactions {
            let transaction = decode_transaction(serialized)?;
            check_transaction(
                &transaction,
                &response.party_id,
                &response.public_key_fingerprint,
                public_key,
            )?;
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
#[allow(clippy::unwrap_used, clippy::expect_used, deprecated)]
mod tests {
    use super::{ExternalPartyTopology, UntypedVersionedMessage, multi_hash};
    use canton_proto::com::daml::ledger::api::v2::admin as pb;
    use canton_proto::com::digitalasset::canton::crypto::v30 as crypto;
    use canton_proto::com::digitalasset::canton::protocol::v30 as topo;
    use canton_signer::{KeyFormat, KeySpec, PublicKey};
    use prost::Message as _;
    use topo::topology_mapping::Mapping;

    const FINGERPRINT: &str = "1220ab";
    const PARTY: &str = "alice::1220ab";

    fn key() -> PublicKey {
        PublicKey::new(KeyFormat::Raw, vec![7; 32], KeySpec::EcCurve25519)
    }

    fn signing_key(bytes: &[u8]) -> crypto::SigningPublicKey {
        crypto::SigningPublicKey {
            format: crypto::CryptoKeyFormat::Raw as i32,
            public_key: bytes.to_vec(),
            ..Default::default()
        }
    }

    fn transaction(mapping: Mapping, operation: topo::enums::TopologyChangeOp) -> Vec<u8> {
        let transaction = topo::TopologyTransaction {
            operation: operation as i32,
            serial: 1,
            mapping: Some(topo::TopologyMapping {
                mapping: Some(mapping),
            }),
        };
        UntypedVersionedMessage {
            data: transaction.encode_to_vec(),
            version: 30,
        }
        .encode_to_vec()
    }

    fn add(mapping: Mapping) -> Vec<u8> {
        transaction(mapping, topo::enums::TopologyChangeOp::AddReplace)
    }

    fn delegation(namespace: &str, to: &[u8]) -> Mapping {
        Mapping::NamespaceDelegation(topo::NamespaceDelegation {
            namespace: namespace.to_string(),
            target_key: Some(signing_key(to)),
            restriction: Some(topo::namespace_delegation::Restriction::CanSignAllMappings(
                topo::namespace_delegation::CanSignAllMappings {},
            )),
            ..Default::default()
        })
    }

    fn key_mapping(party: &str, keys: &[&[u8]]) -> Mapping {
        Mapping::PartyToKeyMapping(topo::PartyToKeyMapping {
            party: party.to_string(),
            threshold: 1,
            signing_keys: keys.iter().map(|k| signing_key(k)).collect(),
        })
    }

    fn hosting(party: &str, hosts: &[(&str, topo::enums::ParticipantPermission)]) -> Mapping {
        Mapping::PartyToParticipant(topo::PartyToParticipant {
            party: party.to_string(),
            threshold: 1,
            participants: hosts
                .iter()
                .map(
                    |(uid, permission)| topo::party_to_participant::HostingParticipant {
                        participant_uid: (*uid).to_string(),
                        permission: *permission as i32,
                        onboarding: None,
                    },
                )
                .collect(),
            ..Default::default()
        })
    }

    /// What a participant returns for an honest onboarding of `PARTY`.
    fn honest_set() -> Vec<Vec<u8>> {
        vec![
            add(delegation(FINGERPRINT, &[7; 32])),
            add(key_mapping(PARTY, &[&[7; 32]])),
            add(hosting(
                PARTY,
                &[(
                    "participant1::1220cc",
                    topo::enums::ParticipantPermission::Confirmation,
                )],
            )),
        ]
    }

    fn response(
        party_id: &str,
        fingerprint: &str,
        transactions: Vec<Vec<u8>>,
    ) -> pb::GenerateExternalPartyTopologyResponse {
        pb::GenerateExternalPartyTopologyResponse {
            party_id: party_id.to_string(),
            public_key_fingerprint: fingerprint.to_string(),
            multi_hash: multi_hash(&transactions),
            topology_transactions: transactions,
        }
    }

    fn refused(response: pb::GenerateExternalPartyTopologyResponse) -> String {
        let error = ExternalPartyTopology::from_response(response, &key()).expect_err("refused");
        assert!(
            matches!(error, canton_core::Error::UnexpectedResponse(_)),
            "{error:?}"
        );
        error.to_string()
    }

    /// The ordinary case: the namespace of the party id is the fingerprint of
    /// the key being registered, the hash is the hash of the transactions,
    /// and the transactions say what onboarding says.
    #[test]
    fn an_honest_onboarding_set_is_accepted() {
        let topology = ExternalPartyTopology::from_response(
            response(PARTY, FINGERPRINT, honest_set()),
            &key(),
        )
        .expect("the honest set");
        assert_eq!(topology.party_id(), PARTY);
        assert_eq!(topology.public_key_fingerprint(), FINGERPRINT);
        assert_eq!(topology.multi_hash().len(), 34);
        assert_eq!(&topology.multi_hash()[..2], &[0x12, 0x20]);
    }

    /// The multi-hash is Canton's: purpose 55, a count, then each transaction's
    /// purpose-11 hash length-prefixed in ascending order. Order of the input
    /// does not matter; the set does.
    #[test]
    fn the_multi_hash_is_over_the_sorted_set_of_transaction_hashes() {
        let set = honest_set();
        let mut reversed = set.clone();
        reversed.reverse();
        assert_eq!(multi_hash(&set), multi_hash(&reversed));
        assert_ne!(multi_hash(&set), multi_hash(&set[..2]));
        // A known vector: one empty transaction. SHA-256(BE32(11) ‖ "") is the
        // transaction hash; the multi-hash is SHA-256(BE32(55) ‖ BE32(1) ‖ BE32(34) ‖ 0x1220 ‖ that).
        let single = multi_hash(&[Vec::new()]);
        assert_eq!(single.len(), 34);
        assert_ne!(&single[2..], &multi_hash(&[vec![1]])[2..]);
    }

    #[test]
    fn a_multi_hash_that_is_not_the_hash_of_the_transactions_is_refused() {
        let mut response = response(PARTY, FINGERPRINT, honest_set());
        response.multi_hash[10] ^= 0xff;
        let message = refused(response);
        assert!(message.contains("multi-hash"), "{message}");
    }

    #[test]
    fn a_second_host_with_submission_rights_is_refused() {
        let mut set = honest_set();
        set.push(add(hosting(
            PARTY,
            &[(
                "attacker::1220ee",
                topo::enums::ParticipantPermission::Submission,
            )],
        )));
        let message = refused(response(PARTY, FINGERPRINT, set));
        assert!(
            message.contains("submission") || message.contains("permission 1"),
            "{message}"
        );
    }

    #[test]
    fn a_delegation_to_another_key_is_refused() {
        let mut set = honest_set();
        set.push(add(delegation(FINGERPRINT, &[9; 32])));
        let message = refused(response(PARTY, FINGERPRINT, set));
        assert!(message.contains("delegation"), "{message}");
        let set = vec![add(delegation("1220ff", &[7; 32]))];
        let message = refused(response(PARTY, FINGERPRINT, set));
        assert!(message.contains("1220ff"), "{message}");
    }

    #[test]
    fn a_key_mapping_naming_an_extra_key_or_another_party_is_refused() {
        let set = vec![add(key_mapping(PARTY, &[&[7; 32], &[9; 32]]))];
        assert!(refused(response(PARTY, FINGERPRINT, set)).contains("signing key"));
        let set = vec![add(key_mapping("mallory::1220ab", &[&[7; 32]]))];
        assert!(refused(response(PARTY, FINGERPRINT, set)).contains("mallory"));
    }

    #[test]
    fn a_mapping_onboarding_does_not_need_and_a_removal_are_refused() {
        let set = vec![add(
            Mapping::VettedPackages(topo::VettedPackages::default()),
        )];
        assert!(refused(response(PARTY, FINGERPRINT, set)).contains("kind"));
        let set = vec![transaction(
            delegation(FINGERPRINT, &[7; 32]),
            topo::enums::TopologyChangeOp::Remove,
        )];
        assert!(refused(response(PARTY, FINGERPRINT, set)).contains("operation"));
    }

    #[test]
    fn bytes_that_are_not_a_transaction_are_refused() {
        let message = refused(response(PARTY, FINGERPRINT, vec![vec![0xff, 0xff, 0xff]]));
        assert!(
            message.contains("versioned message") || message.contains("decode"),
            "{message}"
        );
    }

    /// And the checks that predate the decoding: a party under a namespace this
    /// caller's key does not control, and something that is not a party id.
    #[test]
    fn a_party_under_a_namespace_this_key_does_not_control_is_refused() {
        let message = refused(response("alice::1220ff", FINGERPRINT, honest_set()));
        assert!(
            message.contains("1220ff") && message.contains("1220ab"),
            "{message}"
        );
    }

    #[test]
    fn a_response_that_is_not_a_party_id_is_refused() {
        let message = refused(response("alice", FINGERPRINT, honest_set()));
        assert!(message.contains("not a party id"), "{message}");
    }
}
