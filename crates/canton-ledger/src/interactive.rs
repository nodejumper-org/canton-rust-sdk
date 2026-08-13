//! Interactive submission: prepare, sign elsewhere, execute.
//!
//! An ordinary submission asks the participant to interpret *and* authorize a
//! command, which means the participant holds the party's key. Interactive
//! submission splits that: the participant interprets and returns the
//! transaction with its hash, the key signs the hash wherever it lives, and the
//! signature comes back to be executed.
//!
//! The three types here are the three states of that flow, and each one only
//! offers what is possible in it:
//!
//! * [`Prepare`] — what to interpret. Goes to
//!   [`prepare_submission`](crate::CantonClient::prepare_submission).
//! * [`Prepared`] — the interpreted transaction and its hash. Sign it.
//! * [`Executable`] — signed, and therefore submittable.
//!
//! # The window
//!
//! A prepared transaction does not stay executable indefinitely. If it depends
//! on time, it must be executed within the ledger-time tolerance of the ledger
//! time assigned during preparation — usually about a minute. If it does not,
//! the bound is the much larger preparation-time skew, in the order of hours.
//! Holding a `Prepared` across a long signing ceremony is a design decision to
//! make deliberately, not an accident to discover in production.

use canton_core::{Error, Result};
use canton_proto::com::daml::ledger::api::v2 as pb;
// The interactive submission service has a package of its own; `pb` stays
// the root, where `Command`, `Signature` and the transaction types live.
use canton_proto::com::daml::ledger::api::v2::interactive as ipb;
use canton_signer::Signer;

/// A submission to be interpreted but not authorized: the *prepare* half of
/// interactive submission.
///
/// This carries what `PrepareSubmissionRequest` carries, and nothing else.
/// [`Submit`](crate::Submit) is deliberately not reused: it has a
/// `deduplication` period and a `workflow_id` that preparation has no field
/// for, and accepting them here would silently drop them. The de-duplication
/// period, the transaction shape and the submission id belong to *execution*
/// and are set on [`Executable`]. In the other direction, preparation has four
/// fields a plain submission does not: `verbose_hashing`, `max_record_time`,
/// the hashing scheme, and traffic-cost estimation.
#[derive(Clone, Debug)]
pub struct Prepare {
    act_as: Vec<String>,
    commands: Vec<pb::Command>,
    command_id: Option<String>,
    user_id: Option<String>,
    read_as: Vec<String>,
    synchronizer_id: Option<String>,
    disclosed_contracts: Vec<pb::DisclosedContract>,
    package_id_selection_preference: Vec<String>,
    prefetch_contract_keys: Vec<pb::PrefetchContractKey>,
    min_ledger_time_abs: Option<prost_types::Timestamp>,
    min_ledger_time_rel: Option<std::time::Duration>,
    verbose_hashing: bool,
    max_record_time: Option<prost_types::Timestamp>,
    hashing_scheme_version: Option<i32>,
    taps_max_passes: Option<u32>,
    estimate_traffic_cost: bool,
}

impl Prepare {
    /// Prepare a submission acting as a single party.
    pub fn new(act_as: impl Into<String>) -> Self {
        Self::new_multi(vec![act_as.into()])
    }

    /// Prepare a submission acting as several parties.
    ///
    /// Each of them signs: the participant requires a signature per party in
    /// `act_as`, so a transaction prepared for two parties needs two.
    #[must_use]
    pub fn new_multi(act_as: Vec<String>) -> Self {
        Self {
            act_as,
            commands: Vec::new(),
            command_id: None,
            user_id: None,
            read_as: Vec::new(),
            synchronizer_id: None,
            disclosed_contracts: Vec::new(),
            package_id_selection_preference: Vec::new(),
            prefetch_contract_keys: Vec::new(),
            min_ledger_time_abs: None,
            min_ledger_time_rel: None,
            verbose_hashing: false,
            max_record_time: None,
            hashing_scheme_version: None,
            taps_max_passes: None,
            // The participant estimates by default; only an explicit opt-out
            // is sent, so leaving this alone changes nothing.
            estimate_traffic_cost: true,
        }
    }

    /// Add a command — build them with
    /// [`create`](crate::create) / [`exercise`](crate::exercise), or from
    /// generated bindings.
    #[must_use]
    pub fn add_command(mut self, command: pb::Command) -> Self {
        self.commands.push(command);
        self
    }

    /// Add several commands, executed atomically.
    #[must_use]
    pub fn add_commands(mut self, commands: impl IntoIterator<Item = pb::Command>) -> Self {
        self.commands.extend(commands);
        self
    }

    /// Set the command id, one component of the change ID the Ledger API
    /// de-duplicates on (`user_id`, `act_as`, `command_id`).
    ///
    /// Left unset, a fresh UUID is generated at prepare time and *carried
    /// through execution*, so re-executing the same [`Executable`] is
    /// de-duplicated rather than committed twice.
    #[must_use]
    pub fn with_command_id(mut self, command_id: impl Into<String>) -> Self {
        self.command_id = Some(command_id.into());
        self
    }

    /// Set the user id. Defaults to the one derived from the bearer token.
    #[must_use]
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Parties to read as, in addition to those acting.
    #[must_use]
    pub fn with_read_as(mut self, read_as: Vec<String>) -> Self {
        self.read_as = read_as;
        self
    }

    /// Pin the submission to a synchronizer.
    #[must_use]
    pub fn with_synchronizer_id(mut self, synchronizer_id: impl Into<String>) -> Self {
        self.synchronizer_id = Some(synchronizer_id.into());
        self
    }

    /// Attach explicitly disclosed contracts — the `created_event_blob` path
    /// for contracts the submitting party cannot see on its own.
    #[must_use]
    pub fn with_disclosed_contracts(mut self, contracts: Vec<pb::DisclosedContract>) -> Self {
        self.disclosed_contracts = contracts;
        self
    }

    /// Prefer these package ids when several satisfy a package-name reference.
    #[must_use]
    pub fn with_package_id_selection_preference(mut self, package_ids: Vec<String>) -> Self {
        self.package_id_selection_preference = package_ids;
        self
    }

    /// Fetch these contract keys into the participant's cache before
    /// interpretation.
    #[must_use]
    pub fn with_prefetch_contract_keys(mut self, keys: Vec<pb::PrefetchContractKey>) -> Self {
        self.prefetch_contract_keys = keys;
        self
    }

    /// Lower bound for the transaction's ledger time, absolute.
    ///
    /// Mutually exclusive with [`with_min_ledger_time_rel`](Self::with_min_ledger_time_rel);
    /// the participant rejects both at once.
    #[must_use]
    pub fn with_min_ledger_time_abs(mut self, time: prost_types::Timestamp) -> Self {
        self.min_ledger_time_abs = Some(time);
        self
    }

    /// Lower bound for the transaction's ledger time, relative to when the
    /// participant receives the request.
    ///
    /// Mutually exclusive with [`with_min_ledger_time_abs`](Self::with_min_ledger_time_abs).
    #[must_use]
    pub fn with_min_ledger_time_rel(mut self, duration: std::time::Duration) -> Self {
        self.min_ledger_time_rel = Some(duration);
        self
    }

    /// Ask the participant to return the hashing details it used.
    ///
    /// For debugging a hash mismatch. The participant may refuse — it is
    /// configurable server-side — and what comes back describes internal
    /// structure, so it is off by default.
    #[must_use]
    pub fn with_verbose_hashing(mut self, verbose: bool) -> Self {
        self.verbose_hashing = verbose;
        self
    }

    /// Bound how long the prepared transaction can be executed for.
    ///
    /// Past this record time the participant will certainly refuse it, which is
    /// what makes preparing a fresh transaction for the same intent safe again:
    /// without a bound, "it might still commit" and "prepare another" are both
    /// true at once.
    #[must_use]
    pub fn with_max_record_time(mut self, time: prost_types::Timestamp) -> Self {
        self.max_record_time = Some(time);
        self
    }

    /// Choose the hashing scheme (defaults to the participant's, `V2`).
    ///
    /// The scheme decides what the hash covers, so it is echoed in the response
    /// and sent back with the signature — the participant must verify under the
    /// same scheme it hashed with.
    #[must_use]
    pub fn with_hashing_scheme_version(mut self, version: ipb::HashingSchemeVersion) -> Self {
        self.hashing_scheme_version = Some(version as i32);
        self
    }

    /// Cap the topology-aware package selection passes (defaults to the
    /// participant's configured value).
    #[must_use]
    pub fn with_taps_max_passes(mut self, passes: u32) -> Self {
        self.taps_max_passes = Some(passes);
        self
    }

    /// Turn off the traffic-cost estimate the participant returns by default.
    #[must_use]
    pub fn without_traffic_cost_estimation(mut self) -> Self {
        self.estimate_traffic_cost = false;
        self
    }

    /// The parties this will act as.
    #[must_use]
    pub fn act_as(&self) -> &[String] {
        &self.act_as
    }

    /// Build the wire request, returning the command id that went into it.
    ///
    /// The id is returned rather than read back off the request so it survives
    /// into [`Executable`]: the change ID it belongs to is what makes a repeated
    /// execution a duplicate instead of a second transaction.
    pub(crate) fn into_request(self) -> (String, Option<String>, ipb::PrepareSubmissionRequest) {
        let command_id = self
            .command_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let user_id = self.user_id;
        let min_ledger_time = min_ledger_time(self.min_ledger_time_abs, self.min_ledger_time_rel);
        let request = ipb::PrepareSubmissionRequest {
            user_id: user_id.clone().unwrap_or_default(),
            command_id: command_id.clone(),
            commands: self.commands,
            min_ledger_time,
            act_as: self.act_as,
            read_as: self.read_as,
            disclosed_contracts: self.disclosed_contracts,
            synchronizer_id: self.synchronizer_id.unwrap_or_default(),
            package_id_selection_preference: self.package_id_selection_preference,
            verbose_hashing: self.verbose_hashing,
            prefetch_contract_keys: self.prefetch_contract_keys,
            max_record_time: self.max_record_time,
            hashing_scheme_version: self.hashing_scheme_version,
            taps_max_passes: self.taps_max_passes,
            estimate_traffic_cost: (!self.estimate_traffic_cost).then_some(
                ipb::CostEstimationHints {
                    disabled: true,
                    // Only meaningful when estimation is on, and it is off here.
                    expected_signatures: Vec::new(),
                },
            ),
        };
        (command_id, user_id, request)
    }
}

/// A `std::time::Duration` as protobuf carries one.
///
/// Saturating rather than wrapping: a duration past `i64::MAX` seconds is a
/// caller mistake, and the participant refusing a very long period is a better
/// outcome than a negative one it accepts.
fn duration(value: std::time::Duration) -> prost_types::Duration {
    prost_types::Duration {
        seconds: i64::try_from(value.as_secs()).unwrap_or(i64::MAX),
        nanos: i32::try_from(value.subsec_nanos()).unwrap_or(0),
    }
}

/// `MinLedgerTime` is a `oneof`, so at most one of the two can be set.
///
/// Both set is rejected by the participant; expressing it as a `oneof` here
/// means the later call simply wins rather than producing a request that
/// cannot succeed.
fn min_ledger_time(
    abs: Option<prost_types::Timestamp>,
    rel: Option<std::time::Duration>,
) -> Option<ipb::MinLedgerTime> {
    use ipb::min_ledger_time::Time;
    let time = match (abs, rel) {
        (Some(abs), _) => Time::MinLedgerTimeAbs(abs),
        (None, Some(rel)) => Time::MinLedgerTimeRel(duration(rel)),
        (None, None) => return None,
    };
    Some(ipb::MinLedgerTime { time: Some(time) })
}

/// An interpreted transaction, waiting for signatures.
///
/// [`hash`](Self::hash) is what gets signed — it is the participant's hash of
/// the transaction, not a message to hash again.
#[derive(Clone, Debug)]
pub struct Prepared {
    transaction: Option<ipb::PreparedTransaction>,
    hash: Vec<u8>,
    hashing_scheme_version: i32,
    hashing_details: Option<String>,
    cost_estimation: Option<ipb::CostEstimation>,
    act_as: Vec<String>,
    command_id: String,
    user_id: Option<String>,
}

impl Prepared {
    pub(crate) fn from_response(
        response: ipb::PrepareSubmissionResponse,
        act_as: Vec<String>,
        command_id: String,
        user_id: Option<String>,
    ) -> Result<Self> {
        if response.prepared_transaction_hash.is_empty() {
            return Err(Error::UnexpectedResponse(
                "prepare returned no transaction hash".to_string(),
            ));
        }
        Ok(Self {
            transaction: response.prepared_transaction,
            hash: response.prepared_transaction_hash,
            hashing_scheme_version: response.hashing_scheme_version,
            hashing_details: response.hashing_details,
            cost_estimation: response.cost_estimation,
            act_as,
            command_id,
            user_id,
        })
    }

    /// The hash to sign.
    #[must_use]
    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    /// The interpreted transaction, for a caller that wants to inspect what it
    /// is about to authorize before signing it.
    #[must_use]
    pub fn transaction(&self) -> Option<&ipb::PreparedTransaction> {
        self.transaction.as_ref()
    }

    /// The change ID's command component, carried from preparation.
    #[must_use]
    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    /// The parties that must sign.
    #[must_use]
    pub fn act_as(&self) -> &[String] {
        &self.act_as
    }

    /// What the participant says about how it hashed, when preparation asked
    /// for it with [`Prepare::with_verbose_hashing`].
    ///
    /// For troubleshooting a hash mismatch and nothing else: the proto is
    /// explicit that neither the format nor the content is stable, so this is
    /// a string to read, never one to parse.
    #[must_use]
    pub fn hashing_details(&self) -> Option<&str> {
        self.hashing_details.as_deref()
    }

    /// The participant's estimate of what executing this will cost in traffic,
    /// unless preparation turned the estimate off.
    #[must_use]
    pub fn cost_estimation(&self) -> Option<&ipb::CostEstimation> {
        self.cost_estimation.as_ref()
    }

    /// The hashing scheme the participant hashed under, echoed back with the
    /// signature so it verifies under the same one.
    #[must_use]
    pub fn hashing_scheme_version(&self) -> ipb::HashingSchemeVersion {
        ipb::HashingSchemeVersion::try_from(self.hashing_scheme_version)
            .unwrap_or(ipb::HashingSchemeVersion::Unspecified)
    }

    /// Sign as the single acting party.
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if the submission acts as more than one party
    /// — then it is ambiguous which one this signature is for, and
    /// [`sign_as`](Self::sign_as) says so explicitly.
    pub async fn sign_with(self, signer: &dyn Signer) -> Result<Executable> {
        let [party] = self.act_as.as_slice() else {
            return Err(Error::InvalidRequest(format!(
                "this submission acts as {} parties, so a signature must name \
                 one: use `sign_as`",
                self.act_as.len()
            )));
        };
        let party = party.clone();
        self.sign_as(&party, signer).await
    }

    /// Sign on behalf of `party`.
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if `party` is not among the acting parties —
    /// the participant would reject the execution, and catching it here says
    /// which party was meant. Otherwise, whatever the signer returns.
    pub async fn sign_as(self, party: &str, signer: &dyn Signer) -> Result<Executable> {
        let signature = self.sign_one(party, signer).await?;
        Ok(Executable {
            prepared: self,
            signatures: vec![signature],
            submission_id: None,
            transaction_shape: crate::request::TransactionShape::default(),
            deduplication: None,
        })
    }

    async fn sign_one(
        &self,
        party: &str,
        signer: &dyn Signer,
    ) -> Result<ipb::SinglePartySignatures> {
        if !self.act_as.iter().any(|p| p == party) {
            return Err(Error::InvalidRequest(format!(
                "`{party}` is not among the parties this submission acts as ({})",
                self.act_as.join(", ")
            )));
        }
        let signature = signer.sign(&self.hash).await?;
        Ok(ipb::SinglePartySignatures {
            party: party.to_string(),
            signatures: vec![pb::Signature::from(&signature)],
        })
    }
}

/// How long a change ID stays a duplicate.
///
/// On the wire this is a `oneof` per execute variant, so it is held in one
/// neutral form here and converted at the point of sending.
#[derive(Clone, Copy, Debug)]
enum Deduplication {
    Duration(std::time::Duration),
    Offset(i64),
}

/// A prepared transaction with the signatures that authorize it.
#[derive(Clone, Debug)]
pub struct Executable {
    prepared: Prepared,
    signatures: Vec<ipb::SinglePartySignatures>,
    submission_id: Option<String>,
    transaction_shape: crate::request::TransactionShape,
    deduplication: Option<Deduplication>,
}

impl Executable {
    /// Add another party's signature — for a multi-party submission, where the
    /// participant requires one per acting party.
    ///
    /// # Errors
    /// As [`Prepared::sign_as`].
    pub async fn and_sign_as(mut self, party: &str, signer: &dyn Signer) -> Result<Self> {
        let signature = self.prepared.sign_one(party, signer).await?;
        self.signatures.push(signature);
        Ok(self)
    }

    /// Attach a signature produced elsewhere — an offline ceremony, or a
    /// service that returns signatures rather than implementing
    /// [`Signer`].
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if `party` is not among the acting parties.
    pub fn with_signature(
        mut self,
        party: &str,
        signature: &canton_signer::Signature,
    ) -> Result<Self> {
        if !self.prepared.act_as.iter().any(|p| p == party) {
            return Err(Error::InvalidRequest(format!(
                "`{party}` is not among the parties this submission acts as ({})",
                self.prepared.act_as.join(", ")
            )));
        }
        self.signatures.push(ipb::SinglePartySignatures {
            party: party.to_string(),
            signatures: vec![pb::Signature::from(signature)],
        });
        Ok(self)
    }

    /// Set the de-duplication period as a wall-clock duration: an execution
    /// with the same change ID within this window is rejected as a duplicate.
    ///
    /// The change ID comes from preparation (`user_id`, `act_as`,
    /// `command_id`), so this de-duplicates against ordinary submissions of the
    /// same command too, not only against other executions.
    #[must_use]
    pub fn with_deduplication_duration(mut self, duration: std::time::Duration) -> Self {
        self.deduplication = Some(Deduplication::Duration(duration));
        self
    }

    /// Set the de-duplication period as a ledger offset: executions with the
    /// same change ID since that offset are rejected as duplicates.
    #[must_use]
    pub fn with_deduplication_offset(mut self, offset: i64) -> Self {
        self.deduplication = Some(Deduplication::Offset(offset));
        self
    }

    /// Set the submission id. Left unset, a fresh UUID is generated.
    #[must_use]
    pub fn with_submission_id(mut self, submission_id: impl Into<String>) -> Self {
        self.submission_id = Some(submission_id.into());
        self
    }

    /// Which events the returned transaction carries — only meaningful for
    /// [`execute_submission_and_wait_for_transaction`].
    ///
    /// [`execute_submission_and_wait_for_transaction`]:
    ///     crate::CantonClient::execute_submission_and_wait_for_transaction
    #[must_use]
    pub fn with_transaction_shape(mut self, shape: crate::request::TransactionShape) -> Self {
        self.transaction_shape = shape;
        self
    }

    /// The parties that have signed so far.
    #[must_use]
    pub fn signed_by(&self) -> Vec<&str> {
        self.signatures.iter().map(|s| s.party.as_str()).collect()
    }

    /// Every acting party that has not signed yet.
    ///
    /// The participant requires a signature per acting party; this is that
    /// check, before the round trip that would report it as a rejection.
    #[must_use]
    pub fn unsigned_parties(&self) -> Vec<&str> {
        self.prepared
            .act_as
            .iter()
            .filter(|party| !self.signatures.iter().any(|s| &&s.party == party))
            .map(String::as_str)
            .collect()
    }

    /// The parts every execute variant shares.
    fn parts(
        self,
    ) -> (
        Option<ipb::PreparedTransaction>,
        Option<ipb::PartySignatures>,
        String,
        String,
        i32,
        crate::request::TransactionShape,
        Option<Deduplication>,
    ) {
        let submission_id = self
            .submission_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        (
            self.prepared.transaction,
            Some(ipb::PartySignatures {
                signatures: self.signatures,
            }),
            submission_id,
            self.prepared.user_id.unwrap_or_default(),
            self.prepared.hashing_scheme_version,
            self.transaction_shape,
            self.deduplication,
        )
    }

    pub(crate) fn into_execute_request(self) -> ipb::ExecuteSubmissionRequest {
        use ipb::execute_submission_request::DeduplicationPeriod as P;
        let (transaction, party_signatures, submission_id, user_id, hashing, _, dedup) =
            self.parts();
        ipb::ExecuteSubmissionRequest {
            prepared_transaction: transaction,
            party_signatures,
            deduplication_period: dedup.map(|d| match d {
                Deduplication::Duration(d) => P::DeduplicationDuration(duration(d)),
                Deduplication::Offset(o) => P::DeduplicationOffset(o),
            }),
            submission_id,
            user_id,
            hashing_scheme_version: hashing,
            min_ledger_time: None,
        }
    }

    pub(crate) fn into_execute_and_wait_request(self) -> ipb::ExecuteSubmissionAndWaitRequest {
        use ipb::execute_submission_and_wait_request::DeduplicationPeriod as P;
        let (transaction, party_signatures, submission_id, user_id, hashing, _, dedup) =
            self.parts();
        ipb::ExecuteSubmissionAndWaitRequest {
            prepared_transaction: transaction,
            party_signatures,
            deduplication_period: dedup.map(|d| match d {
                Deduplication::Duration(d) => P::DeduplicationDuration(duration(d)),
                Deduplication::Offset(o) => P::DeduplicationOffset(o),
            }),
            submission_id,
            user_id,
            hashing_scheme_version: hashing,
            min_ledger_time: None,
        }
    }

    pub(crate) fn into_execute_and_wait_for_transaction_request(
        self,
        event_format: pb::EventFormat,
    ) -> ipb::ExecuteSubmissionAndWaitForTransactionRequest {
        use ipb::execute_submission_and_wait_for_transaction_request::DeduplicationPeriod as P;
        let (transaction, party_signatures, submission_id, user_id, hashing, shape, dedup) =
            self.parts();
        ipb::ExecuteSubmissionAndWaitForTransactionRequest {
            prepared_transaction: transaction,
            party_signatures,
            deduplication_period: dedup.map(|d| match d {
                Deduplication::Duration(d) => P::DeduplicationDuration(duration(d)),
                Deduplication::Offset(o) => P::DeduplicationOffset(o),
            }),
            submission_id,
            user_id,
            hashing_scheme_version: hashing,
            min_ledger_time: None,
            transaction_format: Some(pb::TransactionFormat {
                event_format: Some(event_format),
                transaction_shape: shape.as_grpc() as i32,
            }),
        }
    }

    /// The acting parties, for building the event format an
    /// `…_and_wait_for_transaction` call needs.
    pub(crate) fn act_as(&self) -> Vec<String> {
        self.prepared.act_as.clone()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use canton_signer::Ed25519Key;

    fn signer(fingerprint: &str) -> canton_signer::Ed25519Signer {
        Ed25519Key::from_seed(&[4; 32])
            .expect("a valid seed")
            .into_signer(fingerprint)
    }

    fn prepared(act_as: &[&str]) -> Prepared {
        let act_as: Vec<String> = act_as.iter().map(ToString::to_string).collect();
        Prepared::from_response(
            ipb::PrepareSubmissionResponse {
                prepared_transaction: None,
                prepared_transaction_hash: vec![1, 2, 3],
                hashing_scheme_version: ipb::HashingSchemeVersion::V2 as i32,
                hashing_details: None,
                cost_estimation: None,
            },
            act_as,
            "command-1".to_string(),
            Some("user-1".to_string()),
        )
        .expect("a response carrying a hash")
    }

    /// A response with no hash leaves nothing to sign, so it is an error here
    /// rather than an empty signature the participant rejects later.
    #[test]
    fn a_prepare_response_without_a_hash_is_refused() {
        let err = Prepared::from_response(
            ipb::PrepareSubmissionResponse {
                prepared_transaction: None,
                prepared_transaction_hash: Vec::new(),
                hashing_scheme_version: 0,
                hashing_details: None,
                cost_estimation: None,
            },
            vec!["alice".to_string()],
            "c".to_string(),
            None,
        )
        .expect_err("no hash");
        assert!(err.to_string().contains("no transaction hash"), "{err}");
    }

    #[tokio::test]
    async fn signing_a_multi_party_submission_must_name_the_party() {
        let err = prepared(&["alice", "bob"])
            .sign_with(&signer("1220aa"))
            .await
            .expect_err("ambiguous");
        assert!(err.to_string().contains("sign_as"), "{err}");
    }

    #[tokio::test]
    async fn signing_as_a_party_that_is_not_acting_is_refused() {
        let err = prepared(&["alice"])
            .sign_as("mallory", &signer("1220aa"))
            .await
            .expect_err("not an acting party");
        assert!(err.to_string().contains("mallory"), "{err}");
        assert!(err.to_string().contains("alice"), "{err}");
    }

    /// The participant requires a signature per acting party. Asking locally
    /// costs nothing; finding out from a rejection costs a round trip.
    #[tokio::test]
    async fn a_multi_party_submission_reports_who_has_not_signed() {
        let executable = prepared(&["alice", "bob"])
            .sign_as("alice", &signer("1220aa"))
            .await
            .expect("alice signs");
        assert_eq!(executable.signed_by(), vec!["alice"]);
        assert_eq!(executable.unsigned_parties(), vec!["bob"]);

        let executable = executable
            .and_sign_as("bob", &signer("1220bb"))
            .await
            .expect("bob signs");
        assert!(executable.unsigned_parties().is_empty());
        assert_eq!(executable.signed_by(), vec!["alice", "bob"]);
    }

    /// The change ID is `(user_id, act_as, command_id)`. If the command id did
    /// not survive preparation, executing twice would commit twice instead of
    /// being de-duplicated — the failure mode the id exists to prevent.
    #[tokio::test]
    async fn the_change_id_survives_from_preparation_into_execution() {
        let (command_id, user_id, request) = Prepare::new("alice")
            .with_command_id("stable-id")
            .with_user_id("user-1")
            .into_request();
        assert_eq!(command_id, "stable-id");
        assert_eq!(request.command_id, "stable-id");
        assert_eq!(user_id.as_deref(), Some("user-1"));

        let executable = prepared(&["alice"])
            .sign_as("alice", &signer("1220aa"))
            .await
            .expect("sign");
        let wire = executable.into_execute_request();
        assert_eq!(wire.user_id, "user-1", "the user id is half the change ID");
    }

    /// Left unset it is still stable across a retry, because it is generated
    /// once at build time rather than per attempt.
    #[test]
    fn an_unset_command_id_is_generated_once() {
        let (id, _, request) = Prepare::new("alice").into_request();
        assert!(!id.is_empty());
        assert_eq!(id, request.command_id);
    }

    /// `MinLedgerTime` is a `oneof`: the participant rejects both at once, so
    /// the builder cannot produce both.
    #[test]
    fn the_two_min_ledger_times_are_mutually_exclusive() {
        use ipb::min_ledger_time::Time;

        let (_, _, request) = Prepare::new("alice")
            .with_min_ledger_time_rel(std::time::Duration::from_secs(30))
            .into_request();
        assert!(matches!(
            request.min_ledger_time.and_then(|t| t.time),
            Some(Time::MinLedgerTimeRel(_))
        ));

        let (_, _, request) = Prepare::new("alice")
            .with_min_ledger_time_rel(std::time::Duration::from_secs(30))
            .with_min_ledger_time_abs(prost_types::Timestamp {
                seconds: 42,
                nanos: 0,
            })
            .into_request();
        let time = request.min_ledger_time.and_then(|t| t.time);
        assert!(
            matches!(time, Some(Time::MinLedgerTimeAbs(_))),
            "one of the two, never both: {time:?}"
        );
    }

    /// The estimate is on by default and only an opt-out is sent, so an
    /// untouched builder must not carry the hints at all.
    #[test]
    fn traffic_cost_estimation_is_only_sent_when_turned_off() {
        let (_, _, request) = Prepare::new("alice").into_request();
        assert!(request.estimate_traffic_cost.is_none());

        let (_, _, request) = Prepare::new("alice")
            .without_traffic_cost_estimation()
            .into_request();
        assert_eq!(
            request.estimate_traffic_cost.map(|h| h.disabled),
            Some(true)
        );
    }

    /// A signature produced somewhere this crate cannot reach — an offline
    /// ceremony — reaches the wire under the party it was made for.
    #[tokio::test]
    async fn a_signature_made_elsewhere_reaches_the_wire() {
        let external = canton_signer::Signature::new(
            canton_signer::SignatureFormat::Concat,
            vec![8; 64],
            "1220cc",
            canton_signer::SigningAlgorithm::Ed25519,
        );
        let executable = prepared(&["alice", "bob"])
            .sign_as("alice", &signer("1220aa"))
            .await
            .expect("alice signs")
            .with_signature("bob", &external)
            .expect("bob's signature arrives from elsewhere");

        let wire = executable.into_execute_and_wait_request();
        let signatures = wire.party_signatures.expect("signatures").signatures;
        let bob = signatures
            .iter()
            .find(|s| s.party == "bob")
            .expect("bob's entry");
        assert_eq!(bob.signatures[0].signature, vec![8; 64]);
        assert_eq!(bob.signatures[0].signed_by, "1220cc");
    }

    #[tokio::test]
    async fn a_signature_for_a_party_that_is_not_acting_is_refused() {
        let external = canton_signer::Signature::new(
            canton_signer::SignatureFormat::Concat,
            vec![8; 64],
            "1220cc",
            canton_signer::SigningAlgorithm::Ed25519,
        );
        let err = prepared(&["alice"])
            .sign_as("alice", &signer("1220aa"))
            .await
            .expect("sign")
            .with_signature("mallory", &external)
            .expect_err("not an acting party");
        assert!(err.to_string().contains("mallory"), "{err}");
    }

    /// The scheme the participant hashed under has to come back with the
    /// signature: verifying under a different one fails.
    #[tokio::test]
    async fn the_hashing_scheme_is_echoed_back_to_the_participant() {
        let prepared = prepared(&["alice"]);
        assert_eq!(
            prepared.hashing_scheme_version(),
            ipb::HashingSchemeVersion::V2
        );

        let wire = prepared
            .sign_as("alice", &signer("1220aa"))
            .await
            .expect("sign")
            .into_execute_request();
        assert_eq!(
            wire.hashing_scheme_version,
            ipb::HashingSchemeVersion::V2 as i32
        );
    }

    /// The hash the signer sees is the participant's, unmodified.
    #[tokio::test]
    async fn the_signer_is_handed_the_participants_hash() {
        let prepared = prepared(&["alice"]);
        assert_eq!(prepared.hash(), &[1, 2, 3]);

        let key = Ed25519Key::from_seed(&[4; 32]).expect("seed");
        let expected = key.sign_raw(&[1, 2, 3]);
        let wire = prepared
            .sign_as("alice", &signer("1220aa"))
            .await
            .expect("sign")
            .into_execute_request();
        let signatures = wire.party_signatures.expect("signatures").signatures;
        assert_eq!(signatures[0].signatures[0].signature, expected);
    }

    /// De-duplication lives on execution, not preparation — the wire has it
    /// there, and a period set before signing would be silently lost.
    #[tokio::test]
    async fn the_deduplication_period_reaches_every_execute_variant() {
        use ipb::execute_submission_request::DeduplicationPeriod;

        let executable = prepared(&["alice"])
            .sign_as("alice", &signer("1220aa"))
            .await
            .expect("sign")
            .with_deduplication_duration(std::time::Duration::from_secs(60));

        let wire = executable.clone().into_execute_request();
        assert!(matches!(
            wire.deduplication_period,
            Some(DeduplicationPeriod::DeduplicationDuration(d)) if d.seconds == 60
        ));

        let wire = executable
            .with_deduplication_offset(99)
            .into_execute_and_wait_request();
        assert!(matches!(
            wire.deduplication_period,
            Some(
                ipb::execute_submission_and_wait_request::DeduplicationPeriod::DeduplicationOffset(
                    99
                )
            )
        ));
    }
}
