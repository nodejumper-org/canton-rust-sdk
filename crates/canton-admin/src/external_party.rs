//! Allocating a party whose key the participant does not hold.
//!
//! An ordinary party is allocated in one call and the participant signs for it
//! from then on. An *external* party keeps its key elsewhere — which is the
//! whole point of interactive submission — so allocation is two calls with a
//! signature in between:
//!
//! 1. [`generate_external_party_topology`] hands the participant a public key.
//!    It replies with the party id it would assign, the **fingerprint** of the
//!    key, the onboarding topology transaction, and the hash to sign.
//! 2. The key signs that hash — proving the party controls it — and
//!    [`allocate_external_party`] submits the transaction with the signature.
//!
//! The fingerprint from step 1 is what makes a `canton_signer::Ed25519Key` into
//! a `Signer`.
//!
//! # What is checked before anything is signed
//!
//! The signature authorizes whatever the reply contains, and the reply comes
//! from the participant. So the reply is held to what Canton generates for
//! the request this SDK sends, and to nothing else:
//!
//! - the **fingerprint** is recomputed from the caller's key, not taken from
//!   the reply, and the party must be `<hint>::<that fingerprint>`;
//! - there is exactly **one** transaction, and it adds one
//!   `PartyToParticipant` mapping for that party;
//! - the party is **hosted** by one participant, with confirmation permission
//!   and a threshold of one;
//! - the party's **signing keys** are the caller's key alone, with a
//!   threshold of one, registered for the uses an external party's key has;
//! - the transaction is the party's **first** hosting: serial 1;
//! - the transaction holds **nothing this build does not read**: it must
//!   re-encode to the bytes that would be signed.
//!
//! Canton writes a topology transaction in one of two formats, and names the
//! format beside the bytes: 30 up to protocol version 36, and 31 from protocol
//! version 37 (Canton 3.7), where a host carries a state instead of an
//! onboarding flag. Both are read; a format this build does not know is
//! refused.
//!
//! Canton before 3.4.9 answered with three transactions (a namespace
//! delegation, a party-to-key mapping and the hosting). That shape is refused:
//! this SDK supports Canton 3.5 and later.
//!
//! [`generate_external_party_topology`]: crate::AdminClient::generate_external_party_topology
//! [`allocate_external_party`]: crate::AdminClient::allocate_external_party

// This module runs on bytes a participant chose, before anything is signed.
// Arithmetic that could overflow or wrap on them has to say what it does then.
#![warn(clippy::arithmetic_side_effects)]

use canton_proto::com::daml::ledger::api::v2::admin as pb;
use canton_proto::com::digitalasset::canton::crypto::v30 as crypto;
use canton_proto::com::digitalasset::canton::protocol::v30 as topo;
use canton_signer::PublicKey;
use prost::Message as _;
use sha2::{Digest as _, Sha256};

/// Canton's `UntypedVersionedMessage`, the envelope every serialized topology
/// transaction arrives in: the transaction bytes and the version of the format
/// they are written in. Two fields, so it is declared here rather than vendored.
#[derive(Clone, PartialEq, prost::Message)]
struct UntypedVersionedMessage {
    #[prost(bytes = "vec", tag = "1")]
    data: Vec<u8>,
    #[prost(int32, tag = "2")]
    version: i32,
}

/// The topology transaction format of protocol versions up to 36:
/// `com.digitalasset.canton.protocol.v30.TopologyTransaction`.
const TOPOLOGY_FORMAT_30: i32 = 30;
/// The format of protocol version 37 and later:
/// `com.digitalasset.canton.protocol.v31.TopologyTransaction`.
const TOPOLOGY_FORMAT_31: i32 = 31;

/// Canton's `com.digitalasset.canton.protocol.v31` topology messages, as far
/// as onboarding reads them.
///
/// Declared here because the protos this SDK vendors are Canton 3.5.7's, which
/// predate the format. It differs from format 30 in the hosting mapping only:
/// a host carries a `state` in place of the `onboarding` flag, and the party's
/// signing keys are field 4 rather than 6. Enumerations are plain `int32`,
/// which is what they are on the wire.
mod v31 {
    use canton_proto::com::digitalasset::canton::crypto::v30 as crypto;

    #[derive(Clone, PartialEq, prost::Message)]
    pub(super) struct TopologyTransaction {
        /// `v30.Enums.TopologyChangeOp`.
        #[prost(int32, tag = "1")]
        pub(super) operation: i32,
        #[prost(uint32, tag = "2")]
        pub(super) serial: u32,
        #[prost(message, optional, tag = "3")]
        pub(super) mapping: Option<TopologyMapping>,
    }

    /// The `mapping` oneof, with the party's hosting as its only member. Any
    /// other mapping is then a field this message does not declare, which the
    /// decoder drops and the re-encoding check in `decode_transaction` catches.
    #[derive(Clone, PartialEq, prost::Message)]
    pub(super) struct TopologyMapping {
        #[prost(message, optional, tag = "9")]
        pub(super) party_to_participant: Option<PartyToParticipant>,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub(super) struct PartyToParticipant {
        #[prost(string, tag = "1")]
        pub(super) party: String,
        #[prost(uint32, tag = "2")]
        pub(super) threshold: u32,
        #[prost(message, repeated, tag = "3")]
        pub(super) participants: Vec<HostingParticipant>,
        #[prost(message, optional, tag = "4")]
        pub(super) party_signing_keys: Option<crypto::SigningKeysWithThreshold>,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub(super) struct HostingParticipant {
        #[prost(string, tag = "1")]
        pub(super) participant_uid: String,
        /// `v30.Enums.ParticipantPermission`.
        #[prost(int32, tag = "2")]
        pub(super) permission: i32,
        /// `HostingState`: nominated, freezing, onboarding, active.
        #[prost(int32, tag = "3")]
        pub(super) state: i32,
    }

    /// `HOSTING_STATE_ACTIVE`: the participant hosts the party. The other
    /// states are steps towards hosting it.
    pub(super) const HOSTING_STATE_ACTIVE: i32 = 4;
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
/// What Canton registers an external party's key for, in ascending order of
/// `SigningKeyUsage`: the party's namespace, signing its transactions, and
/// proving it owns the key.
const EXTERNAL_PARTY_KEY_USAGE: [i32; 3] = [
    crypto::SigningKeyUsage::Namespace as i32,
    crypto::SigningKeyUsage::Protocol as i32,
    crypto::SigningKeyUsage::ProofOfOwnership as i32,
];
/// `SIGNING_KEY_SPEC_ML_DSA_65` in Canton's `crypto.proto`. The enum vendored
/// here (Canton 3.5.7) predates it.
const SIGNING_KEY_SPEC_ML_DSA_65: i32 = 5;

fn evidence(digest: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(MULTIHASH_SHA256_PREFIX.len().saturating_add(digest.len()));
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

/// A signing key from a topology transaction, as the type the caller's key
/// has, so the two can be compared as keys
/// ([`PublicKey::is_same_key`]) rather than as bytes: Canton stores an
/// Ed25519 key as a DER `SubjectPublicKeyInfo` also when it was handed over
/// raw, and `canton_signer::Ed25519Key` hands it over raw. `None` for a spec
/// or a format this build does not know.
fn registered_key(key: &crypto::SigningPublicKey) -> Option<PublicKey> {
    use canton_signer::{KeyFormat, KeySpec};
    let spec = match key.key_spec {
        spec if spec == crypto::SigningKeySpec::EcCurve25519 as i32 => KeySpec::EcCurve25519,
        spec if spec == crypto::SigningKeySpec::EcP256 as i32 => KeySpec::EcP256,
        spec if spec == crypto::SigningKeySpec::EcP384 as i32 => KeySpec::EcP384,
        spec if spec == crypto::SigningKeySpec::EcSecp256k1 as i32 => KeySpec::EcSecp256k1,
        SIGNING_KEY_SPEC_ML_DSA_65 => KeySpec::MlDsa65,
        _ => return None,
    };
    let format = match key.format {
        format if format == crypto::CryptoKeyFormat::Raw as i32 => KeyFormat::Raw,
        format if format == crypto::CryptoKeyFormat::DerX509SubjectPublicKeyInfo as i32 => {
            KeyFormat::DerX509SubjectPublicKeyInfo
        }
        _ => return None,
    };
    Some(PublicKey::new(format, key.public_key.clone(), spec))
}

/// Which participant the caller expects to host the party.
///
/// The onboarding transaction names one host, and the checks require it to be
/// a single participant with confirmation permission. Which participant it is
/// the SDK cannot check by itself: it knows the endpoint it called, not which
/// participant answers there, and asking that participant for its own id is
/// asking the party being checked. A caller that knows its participant's id,
/// from its configuration, can have the host held to it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExpectedHosting {
    participant: Option<String>,
}

impl ExpectedHosting {
    /// One confirming host, whichever participant it is. This is what
    /// [`generate_external_party_topology`] checks.
    ///
    /// [`generate_external_party_topology`]: crate::AdminClient::generate_external_party_topology
    #[must_use]
    pub fn any_participant() -> Self {
        Self::default()
    }

    /// One confirming host, and it is this participant.
    ///
    /// `participant_uid` is the participant's unique id,
    /// `<name>::<namespace>`. The member form with its `PAR::` prefix is
    /// taken too.
    #[must_use]
    pub fn participant(participant_uid: impl Into<String>) -> Self {
        Self {
            participant: Some(participant_uid.into()),
        }
    }

    /// Whether `participant_uid`, the host a transaction names, is the one
    /// expected.
    fn is_met_by(&self, participant_uid: &str) -> bool {
        // The UID as given, or its member form. Not "strip the prefix and
        // compare": a participant may itself be named `PAR`.
        self.participant.as_deref().is_none_or(|expected| {
            expected == participant_uid || expected.strip_prefix("PAR::") == Some(participant_uid)
        })
    }
}

fn refuse(what: impl std::fmt::Display) -> canton_core::Error {
    canton_core::Error::UnexpectedResponse(format!(
        "the participant's onboarding reply is not what onboarding this party's own key \
         takes: {what}; refusing to sign it"
    ))
}

/// An onboarding transaction as either format writes it, reduced to what is
/// checked.
struct Onboarding {
    operation: i32,
    serial: u32,
    /// `None` when the mapping is not the party's hosting.
    hosting: Option<Hosting>,
}

struct Hosting {
    party: String,
    threshold: u32,
    hosts: Vec<Host>,
    signing_keys: Option<crypto::SigningKeysWithThreshold>,
}

struct Host {
    participant_uid: String,
    permission: i32,
    /// The participant hosts the party, rather than being on the way to:
    /// no onboarding flag in format 30, the active state in format 31.
    active: bool,
}

impl From<topo::TopologyTransaction> for Onboarding {
    fn from(transaction: topo::TopologyTransaction) -> Self {
        use topo::topology_mapping::Mapping;
        let hosting = match transaction.mapping.and_then(|mapping| mapping.mapping) {
            Some(Mapping::PartyToParticipant(hosting)) => Some(Hosting {
                party: hosting.party,
                threshold: hosting.threshold,
                hosts: hosting
                    .participants
                    .into_iter()
                    .map(|host| Host {
                        participant_uid: host.participant_uid,
                        permission: host.permission,
                        active: host.onboarding.is_none(),
                    })
                    .collect(),
                signing_keys: hosting.party_signing_keys,
            }),
            _ => None,
        };
        Self {
            operation: transaction.operation,
            serial: transaction.serial,
            hosting,
        }
    }
}

impl From<v31::TopologyTransaction> for Onboarding {
    fn from(transaction: v31::TopologyTransaction) -> Self {
        let hosting = transaction
            .mapping
            .and_then(|mapping| mapping.party_to_participant)
            .map(|hosting| Hosting {
                party: hosting.party,
                threshold: hosting.threshold,
                hosts: hosting
                    .participants
                    .into_iter()
                    .map(|host| Host {
                        participant_uid: host.participant_uid,
                        permission: host.permission,
                        active: host.state == v31::HOSTING_STATE_ACTIVE,
                    })
                    .collect(),
                signing_keys: hosting.party_signing_keys,
            });
        Self {
            operation: transaction.operation,
            serial: transaction.serial,
            hosting,
        }
    }
}

/// The one onboarding transaction, decoded in the format its envelope names,
/// and only if this build read all of it.
///
/// The signature covers the bytes; the checks cover what is decoded from
/// them. A field this build does not know is dropped in decoding, so a
/// transaction carrying one would be checked as if it did not. Encoding what
/// was decoded gives the same bytes exactly when nothing was dropped.
fn decode_transaction(serialized: &[u8]) -> canton_core::Result<Onboarding> {
    fn undecodable(format: i32, error: &prost::DecodeError) -> canton_core::Error {
        canton_core::Error::UnexpectedResponse(format!(
            "an onboarding transaction (topology format {format}) does not decode: {error}"
        ))
    }
    let envelope = UntypedVersionedMessage::decode(serialized).map_err(|e| {
        canton_core::Error::UnexpectedResponse(format!(
            "an onboarding transaction is not a versioned message: {e}"
        ))
    })?;
    let data = envelope.data.as_slice();
    let (reencoded, onboarding) = match envelope.version {
        TOPOLOGY_FORMAT_30 => {
            let transaction = topo::TopologyTransaction::decode(data)
                .map_err(|e| undecodable(envelope.version, &e))?;
            (transaction.encode_to_vec(), Onboarding::from(transaction))
        }
        TOPOLOGY_FORMAT_31 => {
            let transaction = v31::TopologyTransaction::decode(data)
                .map_err(|e| undecodable(envelope.version, &e))?;
            (transaction.encode_to_vec(), Onboarding::from(transaction))
        }
        other => {
            return Err(refuse(format!(
                "a transaction in topology format {other}, where this SDK reads \
                 {TOPOLOGY_FORMAT_30} and {TOPOLOGY_FORMAT_31}"
            )));
        }
    };
    if reencoded != envelope.data || envelope.encode_to_vec() != serialized {
        return Err(refuse(
            "a transaction that carries something this SDK does not read",
        ));
    }
    Ok(onboarding)
}

/// What the onboarding transaction may say, and nothing else: `party_id`
/// hosted for confirmation by one participant, the expected one where the
/// caller named it, and signing with `key` alone. Returns that participant's
/// UID.
fn check_transaction(
    transaction: Onboarding,
    party_id: &str,
    key: &PublicKey,
    expected: &ExpectedHosting,
) -> canton_core::Result<String> {
    if transaction.operation != topo::enums::TopologyChangeOp::AddReplace as i32 {
        return Err(refuse(format!(
            "a transaction with operation {} rather than an addition",
            transaction.operation
        )));
    }
    // A party's first hosting has serial 1, which is what Canton generates
    // and what it requires again when the transaction is submitted.
    if transaction.serial != 1 {
        return Err(refuse(format!(
            "a transaction with serial {}, where a party's first hosting has serial 1",
            transaction.serial
        )));
    }
    let hosting = transaction.hosting.ok_or_else(|| {
        refuse("a mapping other than the party's hosting, which is the only one onboarding takes")
    })?;
    if hosting.party != party_id {
        return Err(refuse(format!(
            "a hosting mapping for `{}`, not `{party_id}`",
            hosting.party
        )));
    }

    // One host, confirming. A second host would receive the party's
    // transactions, and one with submission rights could act as the party
    // without a signature.
    if hosting.threshold != 1 {
        return Err(refuse(format!(
            "a hosting mapping with threshold {}",
            hosting.threshold
        )));
    }
    let [host] = hosting.hosts.as_slice() else {
        return Err(refuse(format!(
            "the party hosted by {} participants, where one was asked for",
            hosting.hosts.len()
        )));
    };
    if host.permission != topo::enums::ParticipantPermission::Confirmation as i32 {
        return Err(refuse(format!(
            "participant `{}` hosting the party with permission {} rather than confirmation",
            host.participant_uid, host.permission
        )));
    }
    if !host.active {
        return Err(refuse(format!(
            "participant `{}` not yet hosting the party, only on the way to",
            host.participant_uid
        )));
    }
    if host.participant_uid.is_empty() {
        return Err(refuse("a hosting participant without an id"));
    }
    if !expected.is_met_by(&host.participant_uid) {
        return Err(refuse(format!(
            "the party hosted by participant `{}`, where `{}` was expected",
            host.participant_uid,
            expected.participant.as_deref().unwrap_or_default()
        )));
    }

    // The caller's key and no other. Any key listed here can submit as the
    // party.
    let signing = hosting
        .signing_keys
        .as_ref()
        .ok_or_else(|| refuse("a hosting mapping without the party's signing keys"))?;
    if signing.threshold != 1 {
        return Err(refuse(format!(
            "a signing-key threshold of {}",
            signing.threshold
        )));
    }
    let [registered] = signing.keys.as_slice() else {
        return Err(refuse(format!(
            "{} signing keys for the party, where its one key was registered",
            signing.keys.len()
        )));
    };
    if !registered_key(registered).is_some_and(|registered| registered.is_same_key(key)) {
        return Err(refuse(
            "a signing key for the party other than the one being registered",
        ));
    }
    // The rest of what the key's entry says, so that nothing in the bytes
    // being signed is left unread: the uses Canton registers an external
    // party's key for, and nothing in the field Canton no longer writes.
    let mut usage = registered.usage.clone();
    usage.sort_unstable();
    usage.dedup();
    #[allow(deprecated)]
    let scheme = registered.scheme;
    if usage != EXTERNAL_PARTY_KEY_USAGE || scheme != 0 {
        return Err(refuse(
            "the party's key registered for uses other than the three an external party's key \
             has (namespace, protocol, proof of ownership)",
        ));
    }
    Ok(host.participant_uid.clone())
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
    hosting_participant: String,
    transactions: Vec<Vec<u8>>,
    multi_hash: Vec<u8>,
}

impl ExternalPartyTopology {
    pub(crate) fn from_response(
        response: pb::GenerateExternalPartyTopologyResponse,
        party_hint: &str,
        public_key: &PublicKey,
        expected: &ExpectedHosting,
    ) -> canton_core::Result<Self> {
        // The fingerprint is the key's, so it is computed from the key the
        // caller holds. Compared with the one in the reply instead, a
        // participant could name a key of its own, put the party in that
        // key's namespace, and have both agree with each other.
        let fingerprint = public_key.fingerprint()?;
        if response.multi_hash.is_empty() {
            return Err(canton_core::Error::UnexpectedResponse(
                "the participant returned no multi-hash to sign".to_string(),
            ));
        }
        if response.public_key_fingerprint != fingerprint {
            return Err(canton_core::Error::UnexpectedResponse(format!(
                "the participant returned the fingerprint `{}` for a key whose fingerprint is \
                 `{fingerprint}`; refusing to sign for a key that is not this caller's",
                response.public_key_fingerprint
            )));
        }
        // A party id is `<hint>::<namespace>`, and for an external party the
        // namespace *is* the fingerprint of the key being registered.
        let party_id = format!("{party_hint}::{fingerprint}");
        if response.party_id != party_id {
            return Err(canton_core::Error::UnexpectedResponse(format!(
                "the participant offered party `{}`, where hint `{party_hint}` and the key \
                 being registered give `{party_id}` — nothing this caller holds could sign as \
                 any other",
                response.party_id
            )));
        }
        // The signature will cover `multi_hash`, so what the hash covers is
        // what the key authorizes. Recomputed from the transaction the caller
        // can read, and that transaction read: a participant that hashed
        // something else, or put a mapping of its own into this one (a second
        // host, a second signing key), is not signed for.
        let topology = Self::checked(
            party_id,
            fingerprint,
            response.topology_transactions,
            public_key,
            expected,
        )?;
        if topology.multi_hash != response.multi_hash {
            return Err(canton_core::Error::UnexpectedResponse(
                "the participant's multi-hash is not the hash of the onboarding transaction \
                 it returned; refusing to sign it"
                    .to_string(),
            ));
        }
        Ok(topology)
    }

    /// Rebuild a topology that was stored between
    /// [`generate_external_party_topology`] and [`allocate_external_party`],
    /// running the checks the first of them ran on what was stored.
    ///
    /// When the signature comes later (a user approving in a wallet, an HSM
    /// queue, a restart in between) the topology has to be kept somewhere,
    /// and what was kept is input again by the time it is read back. Store
    /// [`party_id`](Self::party_id) and [`transactions`](Self::transactions),
    /// and pass them here with the key and the host that were expected. The
    /// fingerprint and the multi-hash are recomputed, never read back, so
    /// what gets signed is the hash of the transaction that was checked.
    ///
    /// One thing cannot be checked again: the hint. The party must be in the
    /// key's namespace and be the party the transaction hosts, but its hint
    /// is whatever the stored party id says. A caller that needs the hint to
    /// be the one it asked for compares the party id with its own record.
    ///
    /// [`generate_external_party_topology`]: crate::AdminClient::generate_external_party_topology
    /// [`allocate_external_party`]: crate::AdminClient::allocate_external_party
    ///
    /// # Errors
    /// [`canton_core::Error::InvalidRequest`] if `public_key` is not a key its
    /// fingerprint can be computed for, and
    /// [`canton_core::Error::UnexpectedResponse`], as from
    /// `generate_external_party_topology`, if the party is not
    /// `<hint>::<the key's fingerprint>` or the transactions are anything but
    /// the onboarding of that key on the expected host.
    pub fn from_parts(
        party_id: &str,
        transactions: Vec<Vec<u8>>,
        public_key: &PublicKey,
        expected: &ExpectedHosting,
    ) -> canton_core::Result<Self> {
        let fingerprint = public_key.fingerprint()?;
        match party_id.split_once("::") {
            Some((hint, namespace)) if !hint.is_empty() && namespace == fingerprint => {}
            _ => {
                return Err(canton_core::Error::UnexpectedResponse(format!(
                    "`{party_id}` is not a party of the key being registered, whose parties are \
                     `<hint>::{fingerprint}` — nothing this caller holds could sign as it"
                )));
            }
        }
        Self::checked(
            party_id.to_string(),
            fingerprint,
            transactions,
            public_key,
            expected,
        )
    }

    /// The topology for `party_id`, if `transactions` are its onboarding and
    /// nothing else, with the multi-hash computed from them.
    fn checked(
        party_id: String,
        fingerprint: String,
        transactions: Vec<Vec<u8>>,
        public_key: &PublicKey,
        expected: &ExpectedHosting,
    ) -> canton_core::Result<Self> {
        let serialized = match transactions.as_slice() {
            [serialized] => serialized,
            [] => {
                return Err(canton_core::Error::UnexpectedResponse(
                    "there is no onboarding transaction, so there would be nothing to submit \
                     and the multi-hash would cover nothing"
                        .to_string(),
                ));
            }
            several => {
                return Err(refuse(format!(
                    "{} transactions, where onboarding takes one (Canton before 3.4.9 sent \
                     three, and this SDK supports 3.5 and later)",
                    several.len()
                )));
            }
        };
        let transaction = decode_transaction(serialized)?;
        let hosting_participant = check_transaction(transaction, &party_id, public_key, expected)?;
        Ok(Self {
            party_id,
            public_key_fingerprint: fingerprint,
            hosting_participant,
            multi_hash: multi_hash(&transactions),
            transactions,
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

    /// The key's fingerprint, as Canton computes it and as this SDK computed
    /// it from the key to check the participant's reply.
    ///
    /// This is what a signature carries in `signed_by`, and what
    /// `canton_signer::Ed25519Key::into_signer` needs.
    #[must_use]
    pub fn public_key_fingerprint(&self) -> &str {
        &self.public_key_fingerprint
    }

    /// The participant that will host the party, by its UID
    /// (`<name>::<namespace>`): the one host the onboarding transaction
    /// names, with confirmation permission.
    ///
    /// From an honest participant this is the participant that was asked. The
    /// SDK cannot tell: it knows the endpoint it called, not which participant
    /// answers there. A caller that knows its participant's id can have it
    /// checked, with [`ExpectedHosting::participant`].
    #[must_use]
    pub fn hosting_participant(&self) -> &str {
        &self.hosting_participant
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
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::arithmetic_side_effects,
    deprecated
)]
mod tests {
    use super::{
        ExpectedHosting, ExternalPartyTopology, TOPOLOGY_FORMAT_30, TOPOLOGY_FORMAT_31,
        UntypedVersionedMessage, multi_hash, v31,
    };
    use canton_proto::com::daml::ledger::api::v2::admin as pb;
    use canton_proto::com::digitalasset::canton::crypto::v30 as crypto;
    use canton_proto::com::digitalasset::canton::protocol::v30 as topo;
    use canton_signer::{KeyFormat, KeySpec, PublicKey};
    use prost::Message as _;
    use topo::enums::ParticipantPermission::{self, Confirmation, Observation, Submission};
    use topo::enums::TopologyChangeOp;
    use topo::topology_mapping::Mapping;

    /// The public key of RFC 8032 §7.1 TEST 1.
    const KEY: [u8; 32] = [
        0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64, 0x07,
        0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68, 0xf7, 0x07,
        0x51, 0x1a,
    ];
    /// Its fingerprint: `SHA-256(BE32(12) ‖ KEY)` as a multihash.
    const FINGERPRINT: &str =
        "1220035a791d845bbb8195615e7eccd7a57c2ec4075c2e88695f194bab4e0194a889";
    /// A key that is not the caller's.
    const OTHER_KEY: [u8; 32] = [9; 32];
    /// What precedes an Ed25519 key in its DER `SubjectPublicKeyInfo`.
    const ED25519_SPKI_PREFIX: [u8; 12] = [
        0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
    ];
    const HINT: &str = "alice";
    const HOST: &str = "participant1::1220cc";
    /// `PartyToParticipant`'s tag in `TopologyMapping`'s oneof, in both formats.
    const PARTY_TO_PARTICIPANT_TAG: u32 = 9;
    /// Field 15 as a varint: defined in none of the messages read here.
    const UNKNOWN_FIELD: [u8; 2] = [0x78, 0x01];

    /// The two formats Canton writes a topology transaction in.
    const FORMATS: [i32; 2] = [TOPOLOGY_FORMAT_30, TOPOLOGY_FORMAT_31];

    fn party() -> String {
        format!("{HINT}::{FINGERPRINT}")
    }

    fn spki(key: &[u8]) -> Vec<u8> {
        [ED25519_SPKI_PREFIX.as_slice(), key].concat()
    }

    /// The key as `canton_signer::Ed25519Key` hands it over.
    fn raw_key() -> PublicKey {
        PublicKey::new(KeyFormat::Raw, KEY.to_vec(), KeySpec::EcCurve25519)
    }

    /// The same key as an HSM exports it.
    fn der_key() -> PublicKey {
        PublicKey::new(
            KeyFormat::DerX509SubjectPublicKeyInfo,
            spki(&KEY),
            KeySpec::EcCurve25519,
        )
    }

    /// An Ed25519 key as Canton writes it into the transaction: a DER
    /// `SubjectPublicKeyInfo`, whichever format the request carried.
    fn canton_key(key: &[u8]) -> crypto::SigningPublicKey {
        crypto::SigningPublicKey {
            format: crypto::CryptoKeyFormat::DerX509SubjectPublicKeyInfo as i32,
            public_key: spki(key),
            usage: vec![
                crypto::SigningKeyUsage::Namespace as i32,
                crypto::SigningKeyUsage::Protocol as i32,
                crypto::SigningKeyUsage::ProofOfOwnership as i32,
            ],
            key_spec: crypto::SigningKeySpec::EcCurve25519 as i32,
            ..Default::default()
        }
    }

    /// Where a participant stands in hosting a party. Format 30 can say only
    /// the last two.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum State {
        Nominated = 1,
        Freezing = 2,
        Onboarding = 3,
        Active = 4,
    }

    #[derive(Clone, Debug)]
    struct Host {
        uid: String,
        permission: ParticipantPermission,
        state: State,
    }

    fn host(uid: &str, permission: ParticipantPermission) -> Host {
        Host {
            uid: uid.to_string(),
            permission,
            state: State::Active,
        }
    }

    /// A hosting mapping, written in either format.
    #[derive(Clone, Debug)]
    struct Hosting {
        party: String,
        threshold: u32,
        hosts: Vec<Host>,
        signing_keys: Option<crypto::SigningKeysWithThreshold>,
    }

    impl Hosting {
        /// What Canton's `generateExternalPartyTopology` builds for the
        /// request this SDK sends: the party hosted by the participant that
        /// was asked, confirming, and signing with the one key it was given.
        fn honest() -> Self {
            Self {
                party: party(),
                threshold: 1,
                hosts: vec![host(HOST, Confirmation)],
                signing_keys: Some(crypto::SigningKeysWithThreshold {
                    keys: vec![canton_key(&KEY)],
                    threshold: 1,
                }),
            }
        }

        fn in_format_30(&self) -> topo::PartyToParticipant {
            topo::PartyToParticipant {
                party: self.party.clone(),
                threshold: self.threshold,
                participants: self
                    .hosts
                    .iter()
                    .map(|host| topo::party_to_participant::HostingParticipant {
                        participant_uid: host.uid.clone(),
                        permission: host.permission as i32,
                        onboarding: match host.state {
                            State::Active => None,
                            State::Onboarding => {
                                Some(topo::party_to_participant::hosting_participant::Onboarding {})
                            }
                            other => panic!("format 30 cannot say {other:?}"),
                        },
                    })
                    .collect(),
                party_signing_keys: self.signing_keys.clone(),
            }
        }

        fn in_format_31(&self) -> v31::PartyToParticipant {
            v31::PartyToParticipant {
                party: self.party.clone(),
                threshold: self.threshold,
                participants: self
                    .hosts
                    .iter()
                    .map(|host| v31::HostingParticipant {
                        participant_uid: host.uid.clone(),
                        permission: host.permission as i32,
                        state: host.state as i32,
                    })
                    .collect(),
                party_signing_keys: self.signing_keys.clone(),
            }
        }

        /// The mapping's own bytes.
        fn encoded(&self, format: i32) -> Vec<u8> {
            match format {
                TOPOLOGY_FORMAT_30 => self.in_format_30().encode_to_vec(),
                TOPOLOGY_FORMAT_31 => self.in_format_31().encode_to_vec(),
                other => panic!("no format {other}"),
            }
        }

        /// The transaction carrying it, without its envelope.
        fn transaction(&self, format: i32, operation: TopologyChangeOp) -> Vec<u8> {
            match format {
                TOPOLOGY_FORMAT_30 => topo::TopologyTransaction {
                    operation: operation as i32,
                    serial: 1,
                    mapping: Some(topo::TopologyMapping {
                        mapping: Some(Mapping::PartyToParticipant(self.in_format_30())),
                    }),
                }
                .encode_to_vec(),
                TOPOLOGY_FORMAT_31 => v31::TopologyTransaction {
                    operation: operation as i32,
                    serial: 1,
                    mapping: Some(v31::TopologyMapping {
                        party_to_participant: Some(self.in_format_31()),
                    }),
                }
                .encode_to_vec(),
                other => panic!("no format {other}"),
            }
        }

        /// The serialized transaction that adds it, as a participant returns it.
        fn added(&self, format: i32) -> Vec<u8> {
            envelope(
                format,
                self.transaction(format, TopologyChangeOp::AddReplace),
            )
        }
    }

    fn envelope(format: i32, transaction: Vec<u8>) -> Vec<u8> {
        UntypedVersionedMessage {
            data: transaction,
            version: format,
        }
        .encode_to_vec()
    }

    /// A format-30 transaction adding a mapping that is not a hosting.
    fn other_mapping(mapping: Mapping) -> Vec<u8> {
        topo::TopologyTransaction {
            operation: TopologyChangeOp::AddReplace as i32,
            serial: 1,
            mapping: Some(topo::TopologyMapping {
                mapping: Some(mapping),
            }),
        }
        .encode_to_vec()
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

    /// A reply for the caller's party and key carrying these transactions.
    fn carrying(transactions: Vec<Vec<u8>>) -> pb::GenerateExternalPartyTopologyResponse {
        response(&party(), FINGERPRINT, transactions)
    }

    /// The honest reply in `format`, with one thing about the hosting changed.
    fn hosting_with(
        format: i32,
        change: impl FnOnce(&mut Hosting),
    ) -> pb::GenerateExternalPartyTopologyResponse {
        let mut hosting = Hosting::honest();
        change(&mut hosting);
        carrying(vec![hosting.added(format)])
    }

    fn honest(format: i32) -> pb::GenerateExternalPartyTopologyResponse {
        hosting_with(format, |_| {})
    }

    fn refused(response: pb::GenerateExternalPartyTopologyResponse) -> String {
        let error = ExternalPartyTopology::from_response(
            response,
            HINT,
            &raw_key(),
            &ExpectedHosting::default(),
        )
        .expect_err("refused");
        assert!(
            matches!(error, canton_core::Error::UnexpectedResponse(_)),
            "{error:?}"
        );
        error.to_string()
    }

    /// The ordinary case, in both formats, for a key handed over raw and for
    /// the same key handed over as DER: Canton answers both with the DER form,
    /// and both are the caller's key.
    #[test]
    fn an_honest_reply_is_accepted_in_either_format_for_a_raw_key_and_a_der_one() {
        for format in FORMATS {
            for key in [raw_key(), der_key()] {
                let topology = ExternalPartyTopology::from_response(
                    honest(format),
                    HINT,
                    &key,
                    &ExpectedHosting::default(),
                )
                .expect("the honest reply");
                assert_eq!(topology.party_id(), party());
                assert_eq!(topology.public_key_fingerprint(), FINGERPRINT);
                assert_eq!(topology.hosting_participant(), HOST);
                assert_eq!(topology.transactions().len(), 1);
                assert_eq!(topology.multi_hash().len(), 34);
                assert_eq!(&topology.multi_hash()[..2], &[0x12, 0x20]);
            }
        }
    }

    /// Canton's fingerprint of an Ed25519 key is over the 32-byte key in
    /// either format; of any other key, over its DER.
    #[test]
    fn the_fingerprint_is_cantons_and_does_not_depend_on_the_key_format() {
        use sha2::{Digest as _, Sha256};
        use std::fmt::Write as _;

        for key in [raw_key(), der_key()] {
            assert_eq!(key.fingerprint().unwrap(), FINGERPRINT);
        }

        let der = vec![0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86];
        let p256 = PublicKey::new(
            KeyFormat::DerX509SubjectPublicKeyInfo,
            der.clone(),
            KeySpec::EcP256,
        );
        let mut digest = Sha256::new();
        digest.update(12u32.to_be_bytes());
        digest.update(&der);
        let mut expected = "1220".to_string();
        for byte in digest.finalize() {
            write!(expected, "{byte:02x}").unwrap();
        }
        assert_eq!(p256.fingerprint().unwrap(), expected);
    }

    /// A key the fingerprint cannot be computed for is the caller's mistake,
    /// found before the participant's reply is looked at.
    #[test]
    fn a_key_that_is_not_what_it_says_is_an_invalid_request() {
        for key in [
            PublicKey::new(KeyFormat::Raw, vec![7; 31], KeySpec::EcCurve25519),
            PublicKey::new(
                KeyFormat::DerX509SubjectPublicKeyInfo,
                KEY.to_vec(),
                KeySpec::EcCurve25519,
            ),
            PublicKey::new(KeyFormat::Raw, vec![7; 65], KeySpec::EcP256),
        ] {
            let error = ExternalPartyTopology::from_response(
                honest(TOPOLOGY_FORMAT_30),
                HINT,
                &key,
                &ExpectedHosting::default(),
            )
            .expect_err("a malformed key");
            assert!(
                matches!(error, canton_core::Error::InvalidRequest(_)),
                "{error:?}"
            );
        }
    }

    /// The multi-hash is Canton's: purpose 55, a count, then each transaction's
    /// purpose-11 hash length-prefixed in ascending order. Order of the input
    /// does not matter; the set does.
    #[test]
    fn the_multi_hash_is_over_the_sorted_set_of_transaction_hashes() {
        let set = vec![vec![1], vec![2], vec![3]];
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
    fn a_multi_hash_that_is_not_the_hash_of_the_transaction_is_refused() {
        for format in FORMATS {
            let mut response = honest(format);
            response.multi_hash[10] ^= 0xff;
            let message = refused(response);
            assert!(message.contains("multi-hash"), "{message}");

            let mut response = honest(format);
            response.multi_hash.clear();
            assert!(refused(response).contains("no multi-hash"));
        }
    }

    /// The fingerprint in the reply is not what the party's namespace is
    /// checked against: the key's own fingerprint is. A participant that
    /// names a key of its own, puts the party under that key's fingerprint
    /// and registers that key agrees with itself in every field.
    #[test]
    fn a_fingerprint_that_is_not_this_keys_is_refused_though_the_reply_agrees_with_itself() {
        let other = PublicKey::new(KeyFormat::Raw, OTHER_KEY.to_vec(), KeySpec::EcCurve25519);
        let other_fingerprint = other.fingerprint().unwrap();
        let other_party = format!("{HINT}::{other_fingerprint}");
        for format in FORMATS {
            let mut hosting = Hosting::honest();
            hosting.party.clone_from(&other_party);
            hosting.signing_keys.as_mut().unwrap().keys = vec![canton_key(&OTHER_KEY)];
            let consistent = response(
                &other_party,
                &other_fingerprint,
                vec![hosting.added(format)],
            );
            // It is a reply this SDK accepts for the key it is about.
            ExternalPartyTopology::from_response(
                consistent.clone(),
                HINT,
                &other,
                &ExpectedHosting::default(),
            )
            .expect("consistent, for the other key");

            let message = refused(consistent);
            assert!(
                message.contains(&other_fingerprint) && message.contains(FINGERPRINT),
                "{message}"
            );
        }

        let mut response = honest(TOPOLOGY_FORMAT_31);
        response.public_key_fingerprint.clear();
        assert!(refused(response).contains("fingerprint"));
    }

    #[test]
    fn a_party_that_is_not_the_hint_under_this_keys_fingerprint_is_refused() {
        for format in FORMATS {
            for offered in [
                format!("mallory::{FINGERPRINT}"),
                format!("{HINT}::1220ff"),
                format!("{HINT}::{FINGERPRINT}::{FINGERPRINT}"),
                HINT.to_string(),
                String::new(),
            ] {
                let mut response = honest(format);
                response.party_id.clone_from(&offered);
                let message = refused(response);
                assert!(message.contains(&party()), "{offered}: {message}");
            }
            // And the party the reply names is the one the transaction hosts.
            let message = refused(hosting_with(format, |hosting| {
                hosting.party = format!("mallory::{FINGERPRINT}");
            }));
            assert!(message.contains("mallory"), "{message}");
        }
    }

    /// Any key among the party's signing keys can submit as the party.
    #[test]
    fn signing_keys_other_than_the_callers_one_key_are_refused() {
        for format in FORMATS {
            let message = refused(hosting_with(format, |hosting| {
                let signing = hosting.signing_keys.as_mut().unwrap();
                signing.keys.push(canton_key(&OTHER_KEY));
            }));
            assert!(message.contains("2 signing keys"), "{message}");

            let message = refused(hosting_with(format, |hosting| {
                hosting.signing_keys.as_mut().unwrap().keys = vec![canton_key(&OTHER_KEY)];
            }));
            assert!(message.contains("other than the one"), "{message}");

            let message = refused(hosting_with(format, |hosting| {
                hosting.signing_keys.as_mut().unwrap().keys.clear();
            }));
            assert!(message.contains("0 signing keys"), "{message}");

            let message = refused(hosting_with(format, |hosting| {
                hosting.signing_keys = None;
            }));
            assert!(
                message.contains("without the party's signing keys"),
                "{message}"
            );

            for threshold in [0, 2] {
                let message = refused(hosting_with(format, |hosting| {
                    hosting.signing_keys.as_mut().unwrap().threshold = threshold;
                }));
                assert!(message.contains("signing-key threshold"), "{message}");
            }
        }
    }

    /// Every field of the transaction is read. The serial is 1, as Canton
    /// generates it and requires it again at submission, and the key is
    /// registered for exactly the uses an external party's key has.
    #[test]
    fn a_serial_or_a_key_usage_canton_does_not_generate_is_refused() {
        for format in FORMATS {
            for serial in [0, 2, 7] {
                // Operation 1, then the serial, then the honest mapping.
                let honest = Hosting::honest().transaction(format, TopologyChangeOp::AddReplace);
                assert_eq!(&honest[..4], [0x08, 0x01, 0x10, 0x01]);
                let mut bytes = if serial == 0 {
                    vec![0x08, 0x01]
                } else {
                    vec![0x08, 0x01, 0x10, serial]
                };
                bytes.extend_from_slice(&honest[4..]);
                let message = refused(carrying(vec![envelope(format, bytes)]));
                assert!(message.contains("serial"), "{serial}: {message}");
            }

            let usages: [&[crypto::SigningKeyUsage]; 4] = [
                &[],
                &[crypto::SigningKeyUsage::Protocol],
                &[
                    crypto::SigningKeyUsage::Namespace,
                    crypto::SigningKeyUsage::Protocol,
                ],
                &[
                    crypto::SigningKeyUsage::Namespace,
                    crypto::SigningKeyUsage::Protocol,
                    crypto::SigningKeyUsage::ProofOfOwnership,
                    crypto::SigningKeyUsage::SequencerAuthentication,
                ],
            ];
            for usage in usages {
                let message = refused(hosting_with(format, |hosting| {
                    hosting.signing_keys.as_mut().unwrap().keys[0].usage =
                        usage.iter().map(|usage| *usage as i32).collect();
                }));
                assert!(
                    message.contains("registered for uses"),
                    "{usage:?}: {message}"
                );
            }
            let message = refused(hosting_with(format, |hosting| {
                hosting.signing_keys.as_mut().unwrap().keys[0].scheme = 1;
            }));
            assert!(message.contains("registered for uses"), "{message}");

            // The same three uses in another order are the same uses.
            let reordered = hosting_with(format, |hosting| {
                hosting.signing_keys.as_mut().unwrap().keys[0]
                    .usage
                    .reverse();
            });
            ExternalPartyTopology::from_response(
                reordered,
                HINT,
                &raw_key(),
                &ExpectedHosting::default(),
            )
            .expect("the same uses");
        }
    }

    /// The caller's key is recognized by its spec and its bytes, in the raw
    /// form or the DER one, and in nothing else.
    #[test]
    fn a_key_that_only_resembles_the_callers_is_refused() {
        let resembling: [fn(&mut crypto::SigningPublicKey); 4] = [
            // The same bytes, claimed for another curve.
            |key| key.key_spec = crypto::SigningKeySpec::EcP256 as i32,
            // The caller's key with something after it.
            |key| key.public_key.push(0),
            // A format this build does not read a key from.
            |key| key.format = crypto::CryptoKeyFormat::Der as i32,
            // DER that is not an Ed25519 key's.
            |key| key.public_key[8] ^= 1,
        ];
        for format in FORMATS {
            for change in resembling {
                let message = refused(hosting_with(format, |hosting| {
                    change(&mut hosting.signing_keys.as_mut().unwrap().keys[0]);
                }));
                assert!(message.contains("other than the one"), "{message}");
            }

            // The caller's key written raw is still the caller's key.
            let raw = hosting_with(format, |hosting| {
                let key = &mut hosting.signing_keys.as_mut().unwrap().keys[0];
                key.format = crypto::CryptoKeyFormat::Raw as i32;
                key.public_key = KEY.to_vec();
            });
            ExternalPartyTopology::from_response(
                raw,
                HINT,
                &der_key(),
                &ExpectedHosting::default(),
            )
            .expect("the same key, raw");
        }
    }

    /// One host was asked for. A second one receives the party's transactions
    /// whatever its permission, and one that can submit acts as the party.
    #[test]
    fn hosting_other_than_by_one_confirming_participant_is_refused() {
        for format in FORMATS {
            for extra in [Confirmation, Observation, Submission] {
                let message = refused(hosting_with(format, |hosting| {
                    hosting.hosts.push(host("attacker::1220ee", extra));
                }));
                assert!(message.contains("hosted by 2 participants"), "{message}");
            }
            let message = refused(hosting_with(format, |hosting| hosting.hosts.clear()));
            assert!(message.contains("hosted by 0 participants"), "{message}");

            for permission in [Submission, Observation] {
                let message = refused(hosting_with(format, |hosting| {
                    hosting.hosts = vec![host(HOST, permission)];
                }));
                assert!(message.contains("rather than confirmation"), "{message}");
            }

            for threshold in [0, 2] {
                let message = refused(hosting_with(format, |hosting| {
                    hosting.threshold = threshold;
                }));
                assert!(
                    message.contains("hosting mapping with threshold"),
                    "{message}"
                );
            }

            let message = refused(hosting_with(format, |hosting| {
                hosting.hosts[0].uid.clear();
            }));
            assert!(message.contains("without an id"), "{message}");
        }
    }

    /// The host must be hosting the party, not on the way to. Format 30 says
    /// so with a flag; format 31 with a state, of which only one is "hosts".
    #[test]
    fn a_host_that_is_not_yet_hosting_the_party_is_refused() {
        let message = refused(hosting_with(TOPOLOGY_FORMAT_30, |hosting| {
            hosting.hosts[0].state = State::Onboarding;
        }));
        assert!(message.contains("not yet hosting"), "{message}");

        for state in [State::Nominated, State::Freezing, State::Onboarding] {
            let message = refused(hosting_with(TOPOLOGY_FORMAT_31, |hosting| {
                hosting.hosts[0].state = state;
            }));
            assert!(message.contains("not yet hosting"), "{state:?}: {message}");
        }

        // No state at all, and a state this build does not know.
        for state in [0, 5] {
            let mut hosting = Hosting::honest().in_format_31();
            hosting.participants[0].state = state;
            let transaction = v31::TopologyTransaction {
                operation: TopologyChangeOp::AddReplace as i32,
                serial: 1,
                mapping: Some(v31::TopologyMapping {
                    party_to_participant: Some(hosting),
                }),
            };
            let message = refused(carrying(vec![envelope(
                TOPOLOGY_FORMAT_31,
                transaction.encode_to_vec(),
            )]));
            assert!(message.contains("not yet hosting"), "{state}: {message}");
        }
    }

    /// Onboarding is one transaction. More than one is something else riding
    /// along, including the three Canton sent before 3.4.9.
    #[test]
    fn more_than_one_transaction_is_refused() {
        for format in FORMATS {
            let extra_host = Hosting {
                hosts: vec![host("attacker::1220ee", Submission)],
                ..Hosting::honest()
            };
            let message = refused(carrying(vec![
                Hosting::honest().added(format),
                extra_host.added(format),
            ]));
            assert!(message.contains("2 transactions"), "{message}");
        }

        let legacy = vec![
            envelope(
                TOPOLOGY_FORMAT_30,
                other_mapping(Mapping::NamespaceDelegation(topo::NamespaceDelegation {
                    namespace: FINGERPRINT.to_string(),
                    target_key: Some(canton_key(&KEY)),
                    ..Default::default()
                })),
            ),
            envelope(
                TOPOLOGY_FORMAT_30,
                other_mapping(Mapping::PartyToKeyMapping(topo::PartyToKeyMapping {
                    party: party(),
                    threshold: 1,
                    signing_keys: vec![canton_key(&KEY)],
                })),
            ),
            Hosting::honest().added(TOPOLOGY_FORMAT_30),
        ];
        let message = refused(carrying(legacy));
        assert!(
            message.contains("3 transactions") && message.contains("3.4.9"),
            "{message}"
        );

        let message = refused(carrying(Vec::new()));
        assert!(message.contains("no onboarding transaction"), "{message}");
    }

    #[test]
    fn a_mapping_onboarding_does_not_take_and_a_removal_are_refused() {
        let others = || {
            [
                Mapping::VettedPackages(topo::VettedPackages::default()),
                Mapping::NamespaceDelegation(topo::NamespaceDelegation {
                    namespace: FINGERPRINT.to_string(),
                    target_key: Some(canton_key(&KEY)),
                    ..Default::default()
                }),
                Mapping::PartyToKeyMapping(topo::PartyToKeyMapping {
                    party: party(),
                    threshold: 1,
                    signing_keys: vec![canton_key(&KEY)],
                }),
            ]
        };
        for mapping in others() {
            let message = refused(carrying(vec![envelope(
                TOPOLOGY_FORMAT_30,
                other_mapping(mapping),
            )]));
            assert!(
                message.contains("other than the party's hosting"),
                "{message}"
            );
        }
        // In format 31 these mappings keep their field numbers, and the
        // hosting is the only one this build declares: the rest are refused
        // as something it does not read.
        for mapping in others() {
            let message = refused(carrying(vec![envelope(
                TOPOLOGY_FORMAT_31,
                other_mapping(mapping),
            )]));
            assert!(message.contains("does not read"), "{message}");
        }

        for format in FORMATS {
            let removal = envelope(
                format,
                Hosting::honest().transaction(format, TopologyChangeOp::Remove),
            );
            let message = refused(carrying(vec![removal]));
            assert!(message.contains("operation"), "{message}");

            // A transaction with no mapping at all: the operation and serial
            // alone, which both formats write the same way.
            let empty = envelope(format, vec![0x08, 0x01, 0x10, 0x01]);
            let message = refused(carrying(vec![empty]));
            assert!(
                message.contains("other than the party's hosting"),
                "{message}"
            );
        }
    }

    /// The format is the one the envelope names. Bytes written in one and
    /// labelled as the other do not pass as either, and a format this build
    /// does not know is not guessed at.
    #[test]
    fn a_transaction_is_read_in_the_format_its_envelope_names_and_no_other() {
        let hosting = Hosting::honest();
        let as_31 = envelope(
            TOPOLOGY_FORMAT_31,
            hosting.transaction(TOPOLOGY_FORMAT_30, TopologyChangeOp::AddReplace),
        );
        assert!(refused(carrying(vec![as_31])).contains("does not read"));

        let as_30 = envelope(
            TOPOLOGY_FORMAT_30,
            hosting.transaction(TOPOLOGY_FORMAT_31, TopologyChangeOp::AddReplace),
        );
        assert!(refused(carrying(vec![as_30])).contains("does not decode"));

        for unknown in [0, 29, 32, -1] {
            let bytes = envelope(
                unknown,
                hosting.transaction(TOPOLOGY_FORMAT_31, TopologyChangeOp::AddReplace),
            );
            let message = refused(carrying(vec![bytes]));
            assert!(
                message.contains(&format!("topology format {unknown}")),
                "{message}"
            );
        }
    }

    /// The transaction's fields written out by hand around `mapping`, the
    /// hosting mapping's bytes: operation 1, serial 1, then the mapping oneof
    /// holding it. The same layout in both formats.
    fn by_hand(mapping: &[u8]) -> Vec<u8> {
        let mut oneof = Vec::new();
        prost::encoding::encode_key(
            PARTY_TO_PARTICIPANT_TAG,
            prost::encoding::WireType::LengthDelimited,
            &mut oneof,
        );
        prost::encoding::encode_varint(mapping.len() as u64, &mut oneof);
        oneof.extend_from_slice(mapping);
        let mut bytes = vec![0x08, 0x01, 0x10, 0x01, 0x1a];
        prost::encoding::encode_varint(oneof.len() as u64, &mut bytes);
        bytes.extend_from_slice(&oneof);
        bytes
    }

    /// The hand-written layout is the one prost produces, so the test that
    /// uses it fails for the unknown field and not for the layout.
    #[test]
    fn the_hand_written_transaction_layout_is_prosts() {
        for format in FORMATS {
            let hosting = Hosting::honest();
            assert_eq!(
                by_hand(&hosting.encoded(format)),
                hosting.transaction(format, TopologyChangeOp::AddReplace),
                "format {format}"
            );
        }
    }

    /// The signature covers the bytes and the checks cover what is decoded
    /// from them, so the two must be the same thing. A field this build does
    /// not know is dropped in decoding: Canton, which may know it, would act
    /// on it, and the checks above would never have seen it.
    #[test]
    fn a_transaction_carrying_something_this_build_does_not_read_is_refused() {
        for format in FORMATS {
            let hosting = Hosting::honest();
            let honest = || hosting.transaction(format, TopologyChangeOp::AddReplace);

            // In the transaction.
            let mut bytes = honest();
            bytes.extend_from_slice(&UNKNOWN_FIELD);
            let message = refused(carrying(vec![envelope(format, bytes)]));
            assert!(message.contains("does not read"), "{message}");

            // In the envelope around it.
            let mut bytes = envelope(format, honest());
            bytes.extend_from_slice(&UNKNOWN_FIELD);
            let message = refused(carrying(vec![bytes]));
            assert!(message.contains("does not read"), "{message}");

            // Inside the hosting mapping, where a later Canton would add one.
            let mut mapping = hosting.encoded(format);
            mapping.extend_from_slice(&UNKNOWN_FIELD);
            let message = refused(carrying(vec![envelope(format, by_hand(&mapping))]));
            assert!(message.contains("does not read"), "{message}");

            // The same field twice, where Canton reads the last: a second
            // hosting after the honest one.
            let attacker = Hosting {
                hosts: vec![host("attacker::1220ee", Submission)],
                ..Hosting::honest()
            };
            let mut bytes = honest();
            bytes.extend_from_slice(&by_hand(&attacker.encoded(format))[4..]);
            let message = refused(carrying(vec![envelope(format, bytes)]));
            assert!(
                message.contains("does not read") || message.contains("attacker"),
                "{message}"
            );
        }
    }

    /// The SDK cannot know which participant answers at the endpoint it
    /// called, so the host is held to a participant only when the caller
    /// names one.
    #[test]
    fn the_host_is_held_to_the_participant_the_caller_expects() {
        for format in FORMATS {
            for expected in [
                ExpectedHosting::any_participant(),
                ExpectedHosting::participant(HOST),
                ExpectedHosting::participant(format!("PAR::{HOST}")),
            ] {
                let topology = ExternalPartyTopology::from_response(
                    honest(format),
                    HINT,
                    &raw_key(),
                    &expected,
                )
                .expect("the expected host");
                assert_eq!(topology.hosting_participant(), HOST);
            }

            for elsewhere in [
                "participant2::1220cc",
                "participant1::1220dd",
                "PAR::",
                "",
                "PAR::PAR::participant1::1220cc",
            ] {
                let error = ExternalPartyTopology::from_response(
                    honest(format),
                    HINT,
                    &raw_key(),
                    &ExpectedHosting::participant(elsewhere),
                )
                .expect_err("hosted by another participant than expected");
                assert!(
                    matches!(error, canton_core::Error::UnexpectedResponse(_)),
                    "{error:?}"
                );
                let message = error.to_string();
                assert!(
                    message.contains(HOST) && message.contains("was expected"),
                    "{message}"
                );
            }
        }
    }

    /// The expected host is a UID, or the same in its member form. A
    /// participant that is itself named `PAR` is one like any other.
    #[test]
    fn a_participant_named_par_is_matched_in_either_form() {
        for format in FORMATS {
            let named_par = hosting_with(format, |hosting| {
                hosting.hosts = vec![host("PAR::1220cc", Confirmation)];
            });
            for expected in ["PAR::1220cc", "PAR::PAR::1220cc"] {
                let topology = ExternalPartyTopology::from_response(
                    named_par.clone(),
                    HINT,
                    &raw_key(),
                    &ExpectedHosting::participant(expected),
                )
                .expect("the participant named PAR");
                assert_eq!(topology.hosting_participant(), "PAR::1220cc");
            }
            ExternalPartyTopology::from_response(
                named_par,
                HINT,
                &raw_key(),
                &ExpectedHosting::participant("1220cc"),
            )
            .expect_err("a bare namespace is not the participant");
        }
    }

    /// A topology kept between generating it and allocating it is input
    /// again when it is read back. Rebuilding it runs the same checks, and
    /// computes the hash to sign from the transaction that passed them.
    #[test]
    fn a_stored_topology_is_checked_again_when_it_is_rebuilt() {
        for format in FORMATS {
            let generated = ExternalPartyTopology::from_response(
                honest(format),
                HINT,
                &raw_key(),
                &ExpectedHosting::participant(HOST),
            )
            .expect("the honest reply");
            let (party_id, transactions) = (
                generated.party_id().to_string(),
                generated.transactions().to_vec(),
            );

            for key in [raw_key(), der_key()] {
                let rebuilt = ExternalPartyTopology::from_parts(
                    &party_id,
                    transactions.clone(),
                    &key,
                    &ExpectedHosting::participant(HOST),
                )
                .expect("what was stored, unchanged");
                assert_eq!(rebuilt.party_id(), generated.party_id());
                assert_eq!(rebuilt.public_key_fingerprint(), FINGERPRINT);
                assert_eq!(rebuilt.hosting_participant(), HOST);
                assert_eq!(rebuilt.multi_hash(), generated.multi_hash());
                assert_eq!(rebuilt.transactions(), generated.transactions());
            }

            let refused = |party_id: &str,
                           transactions: Vec<Vec<u8>>,
                           key: &PublicKey,
                           expected: &ExpectedHosting| {
                let error =
                    ExternalPartyTopology::from_parts(party_id, transactions, key, expected)
                        .expect_err("not what was generated");
                assert!(
                    matches!(error, canton_core::Error::UnexpectedResponse(_)),
                    "{error:?}"
                );
                error.to_string()
            };
            let any = ExpectedHosting::any_participant();
            let other = PublicKey::new(KeyFormat::Raw, OTHER_KEY.to_vec(), KeySpec::EcCurve25519);

            // Tampered transactions: a second signing key, a second host.
            let mut tampered = Hosting::honest();
            tampered
                .signing_keys
                .as_mut()
                .unwrap()
                .keys
                .push(canton_key(&OTHER_KEY));
            let message = refused(&party_id, vec![tampered.added(format)], &raw_key(), &any);
            assert!(message.contains("2 signing keys"), "{message}");
            let mut tampered = Hosting::honest();
            tampered.hosts.push(host("attacker::1220ee", Confirmation));
            let message = refused(&party_id, vec![tampered.added(format)], &raw_key(), &any);
            assert!(message.contains("hosted by 2 participants"), "{message}");
            let mut bytes = transactions[0].clone();
            bytes.extend_from_slice(&UNKNOWN_FIELD);
            assert!(refused(&party_id, vec![bytes], &raw_key(), &any).contains("does not read"));
            assert!(refused(&party_id, Vec::new(), &raw_key(), &any).contains("no onboarding"));

            // A different key, a different party id, a different host.
            let message = refused(&party_id, transactions.clone(), &other, &any);
            assert!(message.contains("not a party of the key"), "{message}");
            for party in [
                format!("mallory::{FINGERPRINT}"),
                format!("{HINT}::1220ff"),
                format!("::{FINGERPRINT}"),
                HINT.to_string(),
            ] {
                let message = refused(&party, transactions.clone(), &raw_key(), &any);
                assert!(
                    message.contains("mallory") || message.contains("not a party of the key"),
                    "{party}: {message}"
                );
            }
            let message = refused(
                &party_id,
                transactions.clone(),
                &raw_key(),
                &ExpectedHosting::participant("participant2::1220cc"),
            );
            assert!(message.contains("was expected"), "{message}");
        }

        // A key that is not one is the caller's mistake here too.
        let malformed = PublicKey::new(KeyFormat::Raw, vec![7; 31], KeySpec::EcCurve25519);
        let error = ExternalPartyTopology::from_parts(
            &party(),
            vec![Hosting::honest().added(TOPOLOGY_FORMAT_31)],
            &malformed,
            &ExpectedHosting::any_participant(),
        )
        .expect_err("a malformed key");
        assert!(
            matches!(error, canton_core::Error::InvalidRequest(_)),
            "{error:?}"
        );
    }

    /// No input makes the check panic or run away: it is the first thing to
    /// see bytes a participant chose. Each round takes the honest transaction
    /// and overwrites, cuts or extends it at a position a fixed generator
    /// picks, so a failure reproduces.
    #[test]
    fn no_mutation_of_an_onboarding_transaction_panics_the_check() {
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut accepted = 0;
        for format in FORMATS {
            let honest = Hosting::honest().added(format);
            for round in 0..4000 {
                let mut bytes = honest.clone();
                match round % 4 {
                    0 => {
                        let at = usize::try_from(next()).unwrap_or(0) % bytes.len();
                        bytes[at] = next().to_le_bytes()[0];
                    }
                    1 => bytes.truncate(usize::try_from(next()).unwrap_or(0) % bytes.len()),
                    2 => {
                        let extra = usize::try_from(next()).unwrap_or(0) % 16;
                        bytes
                            .extend(std::iter::repeat_with(|| next().to_le_bytes()[0]).take(extra));
                    }
                    _ => {
                        let length = usize::try_from(next()).unwrap_or(0) % 64;
                        bytes = std::iter::repeat_with(|| next().to_le_bytes()[0])
                            .take(length)
                            .collect();
                    }
                }
                let unchanged = bytes == honest;
                let outcome = ExternalPartyTopology::from_parts(
                    &party(),
                    vec![bytes],
                    &raw_key(),
                    &ExpectedHosting::participant(HOST),
                );
                // Whatever still passes is the honest transaction itself:
                // every byte of it is either checked or refused as unread.
                if outcome.is_ok() {
                    assert!(
                        unchanged,
                        "format {format}, round {round}: a mutation passed"
                    );
                    accepted += 1;
                }
            }
        }
        // Rounds that happened to write a byte back unchanged.
        assert!(
            accepted < 200,
            "{accepted} mutations left the bytes as they were"
        );
    }

    #[test]
    fn bytes_that_are_not_a_transaction_are_refused() {
        let message = refused(carrying(vec![vec![0xff, 0xff, 0xff]]));
        assert!(
            message.contains("versioned message") || message.contains("decode"),
            "{message}"
        );
        for format in FORMATS {
            let message = refused(carrying(vec![envelope(format, vec![0xff, 0xff, 0xff])]));
            assert!(message.contains("does not decode"), "{message}");
        }
    }
}
