//! Splice Scan's HTTP API, asked directly or through a validator's scan-proxy.
//!
//! Scan is the service each Super Validator runs to publish what the DSO
//! decided and what happened on the network: the DSO party, the Amulet rules,
//! the mining rounds, traffic balances, the update history, snapshots of the
//! active contracts. The paths and payloads here come from Splice's OpenAPI
//! documents, `scan.yaml` and `scan-proxy.yaml` (vendored under
//! `testdata/openapi/`), and the methods are named after what they return.
//!
//! # Two ways in
//!
//! - [`ScanClient::new`] asks a Scan itself, at `<scan>/api/scan`. It serves
//!   everything here.
//! - [`ScanClient::through_validator`] asks a validator's scan-proxy, at
//!   `<validator>/api/validator`. The validator reads from several Super
//!   Validators' Scans and answers when enough of them agree, and it
//!   authenticates every request, so this route needs a token
//!   ([`with_token`](ScanClient::with_token),
//!   [`with_token_source`](ScanClient::with_token_source)). It serves the
//!   part of the API a wallet or an application backend needs, not the
//!   history and snapshot reads; those are refused before anything is sent.
//!
//! A Scan asked directly is one Super Validator's word. This client does not
//! compare several Scans; the scan-proxy is the route that does.
//!
//! # Why an application needs it
//!
//! Some choices take contracts the DSO owns: the `AmuletRules`, the open
//! mining round, a provider's `FeaturedAppRight`. Creating a
//! `TransferPreapproval` is one such choice. The submitting party does not
//! see those contracts, so they are passed as disclosed contracts, and Scan
//! is where they are published. [`ScanClient::amulet_rules`],
//! [`ScanClient::latest_open_mining_round`] and
//! [`ScanClient::featured_app_right`] return them with
//! [`disclosed`](ContractWithState::disclosed), which is what a submission
//! attaches.
//!
//! # Splice versions
//!
//! Written against the documents of Splice 0.10.3. Two things differ on an
//! older Scan or validator: [`dso`](ScanClient::dso) falls back to the
//! `/v0/dso` every release serves, and the two snapshot page reads
//! ([`acs_snapshot`](ScanClient::acs_snapshot),
//! [`holdings_state`](ScanClient::holdings_state)) need Splice 0.8.4, which
//! is where their `/v2` paths appeared. Everything else is served back to
//! Splice 0.6.
//!
//! # Stability
//!
//! Splice tags each Scan endpoint `external` or `internal`. The external ones
//! are its public API. The internal ones may change in any Splice release,
//! and three of them are the only source of contracts applications need:
//! [`amulet_rules`](ScanClient::amulet_rules),
//! [`featured_app_right`](ScanClient::featured_app_right) and
//! [`transfer_preapproval_by_party`](ScanClient::transfer_preapproval_by_party).
//! Asked of a Scan directly, these are refused unless the client was built
//! with [`with_internal_endpoints`](ScanClient::with_internal_endpoints),
//! which is the caller accepting that. Through a validator's scan-proxy they
//! are part of the proxy's own API and need no opt-in.
//!
//! Contract payloads, choice arguments and results stay JSON
//! ([`serde_json::Value`]): they are Daml values of whatever template the
//! contract has, in the compact encoding the Ledger JSON API uses.

use std::collections::BTreeMap;
use std::time::Duration;

use base64::Engine as _;
use canton_core::{Auth, Error, Result, TokenSource};
use canton_daml::Timestamp;
use canton_proto::com::daml::ledger::api::v2 as pb;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::registry::{
    ERROR_BODY_CAP, ErrorResponse, MAX_RESPONSE_BYTES, read_body_from, transport_error, truncate,
};

/// How Scan is named in an error.
const SCAN: &str = "Scan";

/// How long to wait on a Scan that accepts the connection and then says
/// nothing. `reqwest`'s own default is unbounded.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// The most contracts a history or snapshot page may ask for.
const MAX_PAGE_SIZE: u32 = 1000;

/// Where the client's base URL points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Route {
    /// A Scan: `<scan>/api/scan`, paths `/<version>/…`.
    Scan,
    /// A validator: `<validator>/api/validator`, paths
    /// `/<version>/scan-proxy/…`.
    ScanProxy,
}

/// A client for one Scan, or for one validator's scan-proxy.
#[derive(Clone, Debug)]
pub struct ScanClient {
    base: reqwest::Url,
    http: reqwest::Client,
    auth: Auth,
    route: Route,
    internal_endpoints: bool,
}

/// A contract as Scan publishes it.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ScanContract {
    /// `<package id>:<Module>:<Entity>`.
    pub template_id: String,
    /// The contract id.
    pub contract_id: String,
    /// The contract's arguments, as Daml JSON.
    pub payload: Value,
    /// The created-event blob, base64 as JSON requires. This is what a
    /// participant needs to accept the contract as disclosed.
    pub created_event_blob: String,
    /// When the contract was created.
    pub created_at: String,
}

/// A contract and the synchronizer it is assigned to.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ContractWithState {
    /// The contract.
    pub contract: ScanContract,
    /// The synchronizer the contract is assigned to. `None` while it is
    /// being reassigned from one synchronizer to another.
    #[serde(rename = "domain_id", default)]
    pub synchronizer_id: Option<String>,
}

impl ScanContract {
    /// The contract as a disclosed contract for a gRPC submission, on
    /// `synchronizer_id`.
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if the template id is not
    /// `<package>:<Module>:<Entity>` or the blob is not base64.
    pub fn disclosed(&self, synchronizer_id: &str) -> Result<pb::DisclosedContract> {
        let created_event_blob = base64::engine::general_purpose::STANDARD
            .decode(&self.created_event_blob)
            .map_err(|e| {
                Error::UnexpectedResponse(format!(
                    "Scan published contract `{}` with a created-event blob that is not \
                     base64: {e}",
                    self.contract_id
                ))
            })?;
        Ok(pb::DisclosedContract {
            template_id: Some(identifier(&self.template_id)?),
            contract_id: self.contract_id.clone(),
            created_event_blob,
            synchronizer_id: synchronizer_id.to_string(),
        })
    }

    /// The contract as a disclosed contract for the JSON Ledger API, which
    /// takes the blob as the base64 Scan already sent.
    #[must_use]
    pub fn disclosed_json(&self, synchronizer_id: &str) -> Value {
        serde_json::json!({
            "templateId": self.template_id,
            "contractId": self.contract_id,
            "createdEventBlob": self.created_event_blob,
            "synchronizerId": synchronizer_id,
        })
    }
}

impl ContractWithState {
    /// The contract as a disclosed contract for a gRPC submission, on the
    /// synchronizer it is assigned to.
    ///
    /// # Errors
    /// As [`ScanContract::disclosed`], plus [`Error::Connection`], which is
    /// retriable, if the contract is between synchronizers and so has none to
    /// be disclosed on. That passes in seconds.
    pub fn disclosed(&self) -> Result<pb::DisclosedContract> {
        self.contract.disclosed(self.assigned()?)
    }

    /// The contract as a disclosed contract for the JSON Ledger API.
    ///
    /// # Errors
    /// [`Error::Connection`] if the contract is between synchronizers.
    pub fn disclosed_json(&self) -> Result<Value> {
        Ok(self.contract.disclosed_json(self.assigned()?))
    }

    fn assigned(&self) -> Result<&str> {
        self.synchronizer_id.as_deref().ok_or_else(|| {
            Error::Connection(format!(
                "contract `{}` is being reassigned between synchronizers, so there is none to \
                 disclose it on yet; retry shortly",
                self.contract.contract_id
            ))
        })
    }
}

/// `<package>:<Module>:<Entity>` as the Ledger API's three fields. A module
/// name has dots and never colons, so exactly three parts is the only
/// well-formed shape.
fn identifier(template_id: &str) -> Result<pb::Identifier> {
    let mut parts = template_id.split(':');
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(package_id), Some(module_name), Some(entity_name), None)
            if !package_id.is_empty() && !module_name.is_empty() && !entity_name.is_empty() =>
        {
            Ok(pb::Identifier {
                package_id: package_id.to_string(),
                module_name: module_name.to_string(),
                entity_name: entity_name.to_string(),
            })
        }
        _ => Err(Error::UnexpectedResponse(format!(
            "Scan published a contract with template id `{template_id}`, which is not \
             `<package>:<Module>:<Entity>`"
        ))),
    }
}

/// What the DSO is and the contracts that say how it runs.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct DsoInfo {
    /// The party that represents the DSO as a whole.
    pub dso_party_id: String,
    /// How many Super Validators must agree for a vote to pass.
    pub voting_threshold: u32,
    /// The `OpenMiningRound` with the highest round number. It may not have
    /// opened yet: see [`MiningRounds::latest_open`] for the one to use.
    pub latest_mining_round: ContractWithState,
    /// The `AmuletRules` contract.
    pub amulet_rules: ContractWithState,
    /// The `DsoRules` contract.
    pub dso_rules: ContractWithState,
    /// One `SvNodeState` contract per Super Validator in the DSO rules.
    pub sv_node_states: Vec<ContractWithState>,
    /// The round the network started from, where Scan reports it.
    #[serde(default)]
    pub initial_round: Option<String>,
}

/// The Scans the Super Validators run for one synchronizer.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SynchronizerScans {
    /// The synchronizer.
    #[serde(rename = "domainId")]
    pub synchronizer_id: String,
    /// Its Scans.
    pub scans: Vec<ScanInfo>,
}

/// One Super Validator's Scan.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ScanInfo {
    /// The Scan's public URL, without `/api/scan`.
    pub public_url: String,
    /// The Super Validator that runs it.
    pub sv_name: String,
}

/// The mining rounds that rewards can be registered in, and the ones whose
/// rewards are being computed.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct MiningRounds {
    /// `OpenMiningRound` contracts. Several are open at once, and the newest
    /// may not have reached its opening time.
    pub open: Vec<ContractWithState>,
    /// `IssuingMiningRound` contracts.
    pub issuing: Vec<ContractWithState>,
}

/// The two fields of an `OpenMiningRound` that decide whether it is the one
/// to use.
#[derive(Deserialize)]
struct OpenRound {
    round: Round,
    #[serde(rename = "opensAt")]
    opens_at: Timestamp,
}

#[derive(Deserialize)]
struct Round {
    #[serde(deserialize_with = "int64")]
    number: i64,
}

/// A Daml `Int` in JSON: a string, since a JSON number cannot hold every
/// `i64`, though a number is accepted too.
fn int64<'de, D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<i64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Int {
        Text(String),
        Number(i64),
    }
    match Int::deserialize(deserializer)? {
        Int::Text(text) => text.parse().map_err(serde::de::Error::custom),
        Int::Number(number) => Ok(number),
    }
}

impl MiningRounds {
    /// The open round to pass to a choice at `now`: of the rounds whose
    /// opening time has come, the one with the highest number.
    ///
    /// This is the round Splice's own applications pick
    /// (`selectLatestOpenMiningRound`). The round with the lowest number is
    /// open too, but it is the next to be archived, and a transaction built
    /// on it fails if the archival lands first.
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if a round's payload has no `round` or
    /// `opensAt`, and [`Error::Connection`], which is retriable, if no round
    /// has opened yet.
    pub fn latest_open(&self, now: Timestamp) -> Result<&ContractWithState> {
        let mut latest: Option<(i64, &ContractWithState)> = None;
        for round in &self.open {
            let fields: OpenRound = serde_json::from_value(round.contract.payload.clone())
                .map_err(|e| {
                    Error::UnexpectedResponse(format!(
                        "Scan published open mining round `{}` with a payload that is not an \
                         OpenMiningRound: {e}",
                        round.contract.contract_id
                    ))
                })?;
            if fields.opens_at > now {
                continue;
            }
            if latest.is_none_or(|(number, _)| fields.round.number > number) {
                latest = Some((fields.round.number, round));
            }
        }
        latest.map(|(_, round)| round).ok_or_else(|| {
            Error::Connection(format!(
                "none of the {} open mining rounds Scan published has reached its opening time; \
                 retry shortly",
                self.open.len()
            ))
        })
    }
}

/// How much traffic a member has bought and used on a synchronizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TrafficStatus {
    /// Bytes of extra traffic the member has consumed, as the sequencers
    /// account it.
    pub total_consumed: i64,
    /// Bytes of extra traffic the member may consume in total, as the
    /// sequencers account it.
    pub total_limit: i64,
    /// Bytes of extra traffic the member has purchased, as the DSO records
    /// it. The limit catches up with this once the purchase is processed.
    pub total_purchased: i64,
}

/// An entry in the Amulet Name Service.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AnsEntry {
    /// The entry's contract, where it has one. Entries the DSO defines for
    /// itself and the Super Validators have none.
    #[serde(default)]
    pub contract_id: Option<String>,
    /// The party the name belongs to.
    pub user: String,
    /// The name.
    pub name: String,
    /// The URL the owner gave.
    pub url: String,
    /// The description the owner gave.
    pub description: String,
    /// When the entry expires, where it does.
    #[serde(default)]
    pub expires_at: Option<String>,
}

/// A point in Scan's history: everything up to and including it has been
/// read. Pass it back to read on from there.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct HistoryPosition {
    /// The migration the record time belongs to. Record times start over
    /// after a hard synchronizer migration, so one alone does not order
    /// history.
    #[serde(rename = "after_migration_id")]
    pub migration_id: i64,
    /// The record time.
    #[serde(rename = "after_record_time")]
    pub record_time: String,
}

/// One entry of the update history.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ScanUpdate {
    /// A transaction.
    Transaction(ScanTransaction),
    /// A contract leaving one synchronizer or arriving on another.
    Reassignment(ScanReassignment),
}

impl ScanUpdate {
    /// Where this update is in history, to read on from.
    #[must_use]
    pub fn position(&self) -> HistoryPosition {
        match self {
            Self::Transaction(transaction) => HistoryPosition {
                migration_id: transaction.migration_id,
                record_time: transaction.record_time.clone(),
            },
            Self::Reassignment(reassignment) => HistoryPosition {
                migration_id: match &reassignment.event {
                    ReassignmentEvent::Assignment(assignment) => assignment.migration_id,
                    ReassignmentEvent::Unassignment(unassignment) => unassignment.migration_id,
                },
                record_time: reassignment.record_time.clone(),
            },
        }
    }

    /// The update id.
    #[must_use]
    pub fn update_id(&self) -> &str {
        match self {
            Self::Transaction(transaction) => &transaction.update_id,
            Self::Reassignment(reassignment) => &reassignment.update_id,
        }
    }
}

/// The API gives the two kinds of update no tag to tell them by: a
/// transaction is the one with events, a reassignment the one with an event.
impl<'de> Deserialize<'de> for ScanUpdate {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = Value::deserialize(deserializer)?;
        if value.get("events_by_id").is_some() {
            serde_json::from_value(value)
                .map(Self::Transaction)
                .map_err(D::Error::custom)
        } else if value.get("event").is_some() {
            serde_json::from_value(value)
                .map(Self::Reassignment)
                .map_err(D::Error::custom)
        } else {
            Err(D::Error::custom(
                "an update with neither `events_by_id` (a transaction) nor `event` (a \
                 reassignment)",
            ))
        }
    }
}

/// A transaction in the update history.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ScanTransaction {
    /// The update id.
    pub update_id: String,
    /// The migration the transaction was recorded in.
    pub migration_id: i64,
    /// The workflow id the submitter gave, empty when it gave none.
    pub workflow_id: String,
    /// When the sequencer recorded it.
    pub record_time: String,
    /// The synchronizer it was sequenced on.
    pub synchronizer_id: String,
    /// The ledger effective time.
    pub effective_at: String,
    /// The ids of the root events, in execution order.
    pub root_event_ids: Vec<String>,
    /// Every event of the transaction by its id.
    pub events_by_id: BTreeMap<String, ScanEvent>,
    /// The hash an external party signed, for an externally signed
    /// transaction.
    #[serde(default)]
    pub external_transaction_hash: Option<String>,
}

/// An event of a transaction.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "event_type")]
#[non_exhaustive]
pub enum ScanEvent {
    /// A contract was created.
    #[serde(rename = "created_event")]
    Created(ScanCreatedEvent),
    /// A choice was exercised.
    #[serde(rename = "exercised_event")]
    Exercised(ScanExercisedEvent),
}

/// A contract's creation.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ScanCreatedEvent {
    /// The event id.
    pub event_id: String,
    /// The contract id.
    pub contract_id: String,
    /// `<package id>:<Module>:<Entity>`.
    pub template_id: String,
    /// The name of the package the template is in.
    pub package_name: String,
    /// The contract's arguments, as Daml JSON.
    pub create_arguments: Value,
    /// When the contract was created.
    pub created_at: String,
    /// The contract's signatories.
    pub signatories: Vec<String>,
    /// The contract's observers.
    pub observers: Vec<String>,
}

/// A choice exercised on a contract.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ScanExercisedEvent {
    /// The event id.
    pub event_id: String,
    /// The contract the choice was exercised on.
    pub contract_id: String,
    /// `<package id>:<Module>:<Entity>`.
    pub template_id: String,
    /// The name of the package the template is in.
    pub package_name: String,
    /// The choice.
    pub choice: String,
    /// The choice's argument, as Daml JSON.
    pub choice_argument: Value,
    /// The ids of the events the choice caused, in execution order.
    pub child_event_ids: Vec<String>,
    /// What the choice returned, as Daml JSON.
    pub exercise_result: Value,
    /// Whether the choice archived the contract.
    pub consuming: bool,
    /// The parties that exercised it.
    pub acting_parties: Vec<String>,
    /// The interface the choice was exercised through, where it was.
    #[serde(default)]
    pub interface_id: Option<String>,
}

/// A contract leaving one synchronizer or arriving on another.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ScanReassignment {
    /// The update id.
    pub update_id: String,
    /// The offset on the Scan's own participant. It means nothing to another
    /// participant.
    pub offset: String,
    /// When the sequencer recorded it.
    pub record_time: String,
    /// Which half of the reassignment this is.
    pub event: ReassignmentEvent,
}

/// The two halves of a reassignment.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ReassignmentEvent {
    /// The contract arrived on the target synchronizer.
    Assignment(ScanAssignment),
    /// The contract left the source synchronizer.
    Unassignment(ScanUnassignment),
}

/// As [`ScanUpdate`]: untagged in the API, and told apart by the one field
/// each has that the other does not.
impl<'de> Deserialize<'de> for ReassignmentEvent {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = Value::deserialize(deserializer)?;
        if value.get("created_event").is_some() {
            serde_json::from_value(value)
                .map(Self::Assignment)
                .map_err(D::Error::custom)
        } else if value.get("contract_id").is_some() {
            serde_json::from_value(value)
                .map(Self::Unassignment)
                .map_err(D::Error::custom)
        } else {
            Err(D::Error::custom(
                "a reassignment event with neither `created_event` (an assignment) nor \
                 `contract_id` (an unassignment)",
            ))
        }
    }
}

/// A contract arriving on a synchronizer.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ScanAssignment {
    /// The party that submitted the reassignment.
    pub submitter: String,
    /// The synchronizer the contract left.
    pub source_synchronizer: String,
    /// The synchronizer it arrived on.
    pub target_synchronizer: String,
    /// The migration it was recorded in.
    pub migration_id: i64,
    /// The id that ties this half to the unassignment.
    pub unassign_id: String,
    /// The contract, as created on the target synchronizer.
    pub created_event: ScanCreatedEvent,
    /// How many times the contract has been reassigned.
    pub reassignment_counter: i64,
}

/// A contract leaving a synchronizer.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ScanUnassignment {
    /// The party that submitted the reassignment.
    pub submitter: String,
    /// The synchronizer the contract left.
    pub source_synchronizer: String,
    /// The migration it was recorded in.
    pub migration_id: i64,
    /// The synchronizer it is going to.
    pub target_synchronizer: String,
    /// The id that ties this half to the assignment.
    pub unassign_id: String,
    /// How many times the contract has been reassigned.
    pub reassignment_counter: i64,
    /// The contract.
    pub contract_id: String,
}

/// One entry of the event history: an update, the mediator's verdict on it,
/// or both.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ScanEventItem {
    /// The update, where Scan's participant saw it. A transaction the
    /// mediator rejected has a verdict and no update.
    #[serde(default)]
    pub update: Option<ScanUpdate>,
    /// The mediator's verdict, as Scan publishes it.
    #[serde(default)]
    pub verdict: Option<Value>,
    /// The traffic the update cost, where Scan reports it.
    #[serde(default)]
    pub traffic_summary: Option<Value>,
    /// The application activity records the update produced, where Scan
    /// reports them.
    #[serde(default)]
    pub app_activity_records: Option<Value>,
}

/// Which snapshot of the active contracts to read, and which part of it.
///
/// Scan takes a snapshot every few hours.
/// [`ScanClient::acs_snapshot_timestamp`] gives the record time of the one
/// before a moment; [`at_or_before`](Self::at_or_before) lets Scan find it
/// instead.
#[derive(Clone, Debug)]
pub struct SnapshotRequest {
    migration_id: i64,
    record_time: String,
    at_or_before: bool,
    after: Option<String>,
    page_size: u32,
    parties: Vec<String>,
    templates: Vec<String>,
}

impl SnapshotRequest {
    /// The snapshot taken at exactly `record_time` in `migration_id`.
    pub fn at(migration_id: i64, record_time: impl Into<String>) -> Self {
        Self {
            migration_id,
            record_time: record_time.into(),
            at_or_before: false,
            after: None,
            page_size: 100,
            parties: Vec::new(),
            templates: Vec::new(),
        }
    }

    /// Take the most recent snapshot at or before the record time, where the
    /// default is to require one at exactly that time.
    #[must_use]
    pub fn at_or_before(mut self) -> Self {
        self.at_or_before = true;
        self
    }

    /// How many contracts a page holds. Defaults to 100; Scan allows 1000.
    #[must_use]
    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    /// Read the page after the one that returned this token.
    #[must_use]
    pub fn after(mut self, next_page_token: impl Into<String>) -> Self {
        self.after = Some(next_page_token.into());
        self
    }

    /// Only contracts these parties are stakeholders of. For the holdings
    /// reads these are the owners, and at least one is required.
    #[must_use]
    pub fn for_parties<I, S>(mut self, parties: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.parties = parties.into_iter().map(Into::into).collect();
        self
    }

    /// Only contracts of these templates, each as
    /// `<package name>:<Module>:<Entity>`. Ignored by the holdings reads.
    #[must_use]
    pub fn for_templates<I, S>(mut self, templates: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.templates = templates.into_iter().map(Into::into).collect();
        self
    }

    fn record_time_match(&self) -> &'static str {
        if self.at_or_before {
            "at_or_before"
        } else {
            "exact"
        }
    }

    fn owners(&self, read: &str) -> Result<&[String]> {
        if self.parties.is_empty() {
            return Err(Error::InvalidRequest(format!(
                "{read} needs at least one owner party (SnapshotRequest::for_parties)"
            )));
        }
        Ok(&self.parties)
    }
}

/// A page of a snapshot of the active contracts.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AcsSnapshot {
    /// The record time of the snapshot that was read.
    pub record_time: String,
    /// The migration it belongs to.
    pub migration_id: i64,
    /// The contracts on this page.
    pub created_events: Vec<ActiveContract>,
    /// The token for the next page, absent on the last one.
    #[serde(default)]
    pub next_page_token: Option<String>,
}

/// A contract in a snapshot.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ActiveContract {
    /// The update that created it, where Scan reports it.
    #[serde(default)]
    pub created_in_update_id: Option<String>,
    /// The contract id.
    pub contract_id: String,
    /// `<package id>:<Module>:<Entity>`.
    pub template_id: String,
    /// The name of the package the template is in.
    pub package_name: String,
    /// The contract's arguments, as Daml JSON.
    pub create_arguments: Value,
    /// When the contract was created.
    pub created_at: String,
    /// The contract's signatories.
    pub signatories: Vec<String>,
    /// The contract's observers.
    pub observers: Vec<String>,
}

/// What some parties held at a snapshot.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HoldingsSummaries {
    /// The record time of the snapshot that was read.
    pub record_time: String,
    /// The migration it belongs to.
    pub migration_id: i64,
    /// One summary per owner that held anything.
    pub summaries: Vec<HoldingsSummary>,
}

/// What one party held at a snapshot. Amounts are decimal strings, as Daml
/// JSON writes a `Decimal`.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HoldingsSummary {
    /// The owner.
    pub party_id: String,
    /// The Amulet the party could spend.
    pub total_unlocked_coin: String,
    /// The Amulet locked in its name.
    pub total_locked_coin: String,
    /// Both together.
    pub total_coin_holdings: String,
}

impl ScanClient {
    /// A client for the Scan served at `base_url`, which ends in `/api/scan`.
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if `base_url` is not a URL, or an HTTP client
    /// cannot be built.
    pub fn new(base_url: &str) -> Result<Self> {
        Self::with_http_client(base_url, default_http_client()?)
    }

    /// [`new`](Self::new) with an HTTP client the caller has configured:
    /// timeouts, proxies, a custom TLS root store. Configure it with
    /// `reqwest::redirect::Policy::none()`, as [`new`](Self::new) does: what
    /// Scan answers goes into commands the caller signs, so it must come from
    /// the Scan the caller named and not from wherever a 3xx points.
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if `base_url` is not a URL.
    pub fn with_http_client(base_url: &str, http: reqwest::Client) -> Result<Self> {
        Self::build(base_url, http, Route::Scan)
    }

    /// A client for the scan-proxy of the validator served at `base_url`,
    /// which ends in `/api/validator`.
    ///
    /// The proxy authenticates every request: give the client a token with
    /// [`with_token`](Self::with_token) or
    /// [`with_token_source`](Self::with_token_source).
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if `base_url` is not a URL, or an HTTP client
    /// cannot be built.
    pub fn through_validator(base_url: &str) -> Result<Self> {
        Self::through_validator_with_http_client(base_url, default_http_client()?)
    }

    /// [`through_validator`](Self::through_validator) with an HTTP client the
    /// caller has configured. See [`with_http_client`](Self::with_http_client).
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if `base_url` is not a URL.
    pub fn through_validator_with_http_client(
        base_url: &str,
        http: reqwest::Client,
    ) -> Result<Self> {
        Self::build(base_url, http, Route::ScanProxy)
    }

    fn build(base_url: &str, http: reqwest::Client, route: Route) -> Result<Self> {
        // A trailing slash matters to `Url::join`, and the paths below are
        // relative, so it is normalised here once.
        let normalised = format!("{}/", base_url.trim_end_matches('/'));
        let base = reqwest::Url::parse(&normalised)
            .map_err(|e| Error::InvalidRequest(format!("`{base_url}` is not a Scan URL: {e}")))?;
        Ok(Self {
            base,
            http,
            auth: Auth::None,
            route,
            internal_endpoints: false,
        })
    }

    /// Send a bearer token with every request. A validator's scan-proxy
    /// requires one; a Scan does not.
    #[must_use]
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Auth::Static(token.into());
        self
    }

    /// Send a bearer token from a source that refreshes it, such as
    /// `canton_auth::TokenProvider`.
    #[must_use]
    pub fn with_token_source(mut self, source: impl TokenSource + 'static) -> Self {
        self.auth = Auth::Dynamic(std::sync::Arc::new(source));
        self
    }

    /// Allow the calls that reach endpoints Splice tags `internal` when the
    /// client asks a Scan directly:
    /// [`amulet_rules`](Self::amulet_rules),
    /// [`featured_app_right`](Self::featured_app_right) and
    /// [`transfer_preapproval_by_party`](Self::transfer_preapproval_by_party).
    ///
    /// Splice may change an internal endpoint in any release, and these
    /// methods change with it. Through a validator's scan-proxy the same
    /// calls are part of the proxy's API, and this is not needed.
    #[must_use]
    pub fn with_internal_endpoints(mut self) -> Self {
        self.internal_endpoints = true;
        self
    }

    /// Build a URL from an API version and path segments, under the
    /// scan-proxy prefix where the client goes through a validator. Every
    /// segment is percent-encoded, so a party or a name cannot change which
    /// resource is addressed.
    fn url(&self, version: &str, segments: &[&str]) -> Result<reqwest::Url> {
        let mut url = self.base.clone();
        {
            let mut path = url.path_segments_mut().map_err(|()| {
                Error::InvalidRequest("the Scan URL cannot have a path".to_string())
            })?;
            path.pop_if_empty().push(version);
            if self.route == Route::ScanProxy {
                path.push("scan-proxy");
            }
            path.extend(segments);
        }
        Ok(url)
    }

    /// Refuse a read the validator's scan-proxy does not serve.
    fn scan_only(&self, read: &str) -> Result<()> {
        if self.route == Route::ScanProxy {
            return Err(Error::InvalidRequest(format!(
                "a validator's scan-proxy does not serve {read}; ask a Scan for it \
                 (ScanClient::new)"
            )));
        }
        Ok(())
    }

    /// Refuse an internal endpoint of a Scan the caller did not opt into.
    fn internal(&self, read: &str) -> Result<()> {
        if self.route == Route::Scan && !self.internal_endpoints {
            return Err(Error::InvalidRequest(format!(
                "{read} is an endpoint Scan tags internal, which Splice may change in any \
                 release; opt in with ScanClient::with_internal_endpoints(), or go through a \
                 validator's scan-proxy (ScanClient::through_validator), which serves it as \
                 part of its API"
            )));
        }
        Ok(())
    }

    /// The party that represents the DSO.
    ///
    /// # Errors
    /// As any HTTP call, plus [`Error::UnexpectedResponse`] on a body that is
    /// not what Splice specifies.
    pub async fn dso_party_id(&self) -> Result<String> {
        #[derive(Deserialize)]
        struct Reply {
            dso_party_id: String,
        }
        let reply: Reply = self.get(self.url("v0", &["dso-party-id"])?).await?;
        Ok(reply.dso_party_id)
    }

    /// The DSO: its party, its voting threshold, and the contracts that hold
    /// its rules.
    ///
    /// Asked at `/v1/dso`, which Splice 0.10 added. A Scan or a validator
    /// older than that answers `404` there and serves `/v0/dso`, which
    /// carries the same contracts, so that is asked next.
    ///
    /// # Errors
    /// As [`dso_party_id`](Self::dso_party_id).
    pub async fn dso(&self) -> Result<DsoInfo> {
        match self.get(self.url("v1", &["dso"])?).await {
            Err(Error::Http { status: 404, .. }) => self.get(self.url("v0", &["dso"])?).await,
            answer => answer,
        }
    }

    /// Every Super Validator's Scan, per synchronizer.
    ///
    /// # Errors
    /// As [`dso_party_id`](Self::dso_party_id), and
    /// [`Error::InvalidRequest`] through a validator's scan-proxy, which does
    /// not serve it.
    pub async fn scans(&self) -> Result<Vec<SynchronizerScans>> {
        #[derive(Deserialize)]
        struct Reply {
            scans: Vec<SynchronizerScans>,
        }
        self.scan_only("the list of Scans")?;
        let reply: Reply = self.get(self.url("v0", &["scans"])?).await?;
        Ok(reply.scans)
    }

    /// The open and the issuing mining rounds.
    ///
    /// # Errors
    /// As [`dso_party_id`](Self::dso_party_id).
    pub async fn open_and_issuing_mining_rounds(&self) -> Result<MiningRounds> {
        // A Scan answers with only the rounds the caller does not say it has
        // cached, keyed by contract id; the proxy answers with them all.
        #[derive(Serialize)]
        struct Cached {
            cached_open_mining_round_contract_ids: [String; 0],
            cached_issuing_round_contract_ids: [String; 0],
        }
        #[derive(Deserialize)]
        struct MaybeCached {
            #[serde(default)]
            contract: Option<ScanContract>,
            #[serde(default)]
            domain_id: Option<String>,
        }
        #[derive(Deserialize)]
        struct FromScan {
            open_mining_rounds: BTreeMap<String, MaybeCached>,
            issuing_mining_rounds: BTreeMap<String, MaybeCached>,
        }
        #[derive(Deserialize)]
        struct FromProxy {
            open_mining_rounds: Vec<ContractWithState>,
            issuing_mining_rounds: Vec<ContractWithState>,
        }
        fn all(rounds: BTreeMap<String, MaybeCached>) -> Result<Vec<ContractWithState>> {
            rounds
                .into_iter()
                .map(|(contract_id, round)| {
                    let contract = round.contract.ok_or_else(|| {
                        Error::UnexpectedResponse(format!(
                            "Scan left out mining round `{contract_id}` as cached, where this \
                             client told it nothing was"
                        ))
                    })?;
                    Ok(ContractWithState {
                        contract,
                        synchronizer_id: round.domain_id,
                    })
                })
                .collect()
        }

        let url = self.url("v0", &["open-and-issuing-mining-rounds"])?;
        match self.route {
            Route::Scan => {
                let reply: FromScan = self
                    .post(
                        url,
                        &Cached {
                            cached_open_mining_round_contract_ids: [],
                            cached_issuing_round_contract_ids: [],
                        },
                    )
                    .await?;
                Ok(MiningRounds {
                    open: all(reply.open_mining_rounds)?,
                    issuing: all(reply.issuing_mining_rounds)?,
                })
            }
            Route::ScanProxy => {
                let reply: FromProxy = self.get(url).await?;
                Ok(MiningRounds {
                    open: reply.open_mining_rounds,
                    issuing: reply.issuing_mining_rounds,
                })
            }
        }
    }

    /// The open mining round to pass to a choice now: see
    /// [`MiningRounds::latest_open`].
    ///
    /// # Errors
    /// As [`open_and_issuing_mining_rounds`](Self::open_and_issuing_mining_rounds)
    /// and [`MiningRounds::latest_open`].
    pub async fn latest_open_mining_round(&self) -> Result<ContractWithState> {
        let rounds = self.open_and_issuing_mining_rounds().await?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .and_then(|since| i64::try_from(since.as_micros()).ok())
            .ok_or_else(|| {
                Error::InvalidRequest("the system clock is not a time after 1970".to_string())
            })?;
        rounds.latest_open(Timestamp(now)).cloned()
    }

    /// The closed mining rounds that have not been archived yet.
    ///
    /// # Errors
    /// As [`scans`](Self::scans).
    pub async fn closed_rounds(&self) -> Result<Vec<ScanContract>> {
        #[derive(Deserialize)]
        struct Reply {
            rounds: Vec<ScanContract>,
        }
        self.scan_only("the closed mining rounds")?;
        let reply: Reply = self.get(self.url("v0", &["closed-rounds"])?).await?;
        Ok(reply.rounds)
    }

    /// The `AmuletRules` contract.
    ///
    /// On a Scan this is an internal endpoint: see
    /// [`with_internal_endpoints`](Self::with_internal_endpoints).
    ///
    /// # Errors
    /// As [`dso_party_id`](Self::dso_party_id), and
    /// [`Error::InvalidRequest`] on a Scan without the opt-in.
    pub async fn amulet_rules(&self) -> Result<ContractWithState> {
        #[derive(Deserialize)]
        struct MaybeCached {
            #[serde(default)]
            contract: Option<ScanContract>,
            #[serde(default)]
            domain_id: Option<String>,
        }
        #[derive(Deserialize)]
        struct FromScan {
            amulet_rules_update: MaybeCached,
        }
        #[derive(Deserialize)]
        struct FromProxy {
            amulet_rules: ContractWithState,
        }
        self.internal("the Amulet rules")?;
        let url = self.url("v0", &["amulet-rules"])?;
        match self.route {
            Route::Scan => {
                // The body names the contract the caller has cached, so that
                // Scan can leave it out. None is named here.
                let reply: FromScan = self.post(url, &serde_json::json!({})).await?;
                let contract = reply.amulet_rules_update.contract.ok_or_else(|| {
                    Error::UnexpectedResponse(
                        "Scan left out the Amulet rules as cached, where this client told it \
                         nothing was"
                            .to_string(),
                    )
                })?;
                Ok(ContractWithState {
                    contract,
                    synchronizer_id: reply.amulet_rules_update.domain_id,
                })
            }
            Route::ScanProxy => {
                let reply: FromProxy = self.get(url).await?;
                Ok(reply.amulet_rules)
            }
        }
    }

    /// A provider's `FeaturedAppRight` contract, or `None` if the provider is
    /// not a featured application.
    ///
    /// The contract comes without a synchronizer: disclose it on the one the
    /// Amulet rules are on ([`ScanContract::disclosed`]).
    ///
    /// On a Scan this is an internal endpoint: see
    /// [`with_internal_endpoints`](Self::with_internal_endpoints).
    ///
    /// # Errors
    /// As [`amulet_rules`](Self::amulet_rules).
    pub async fn featured_app_right(&self, provider: &str) -> Result<Option<ScanContract>> {
        #[derive(Deserialize)]
        struct Reply {
            #[serde(default)]
            featured_app_right: Option<ScanContract>,
        }
        self.internal("a featured application right")?;
        let reply: Reply = self
            .get(self.url("v0", &["featured-apps", provider])?)
            .await?;
        Ok(reply.featured_app_right)
    }

    /// The `TransferPreapproval` contract that lets `party` receive Amulet
    /// without accepting each transfer, or `None` if it has none.
    ///
    /// On a Scan this is an internal endpoint: see
    /// [`with_internal_endpoints`](Self::with_internal_endpoints).
    ///
    /// # Errors
    /// As [`amulet_rules`](Self::amulet_rules).
    pub async fn transfer_preapproval_by_party(
        &self,
        party: &str,
    ) -> Result<Option<ContractWithState>> {
        #[derive(Deserialize)]
        struct Reply {
            transfer_preapproval: ContractWithState,
        }
        self.internal("a transfer preapproval")?;
        let url = self.url("v0", &["transfer-preapprovals", "by-party", party])?;
        let reply: Option<Reply> = self.get_optional(url).await?;
        Ok(reply.map(|reply| reply.transfer_preapproval))
    }

    /// How much traffic `member` has bought and used on a synchronizer. A
    /// member is a participant or a mediator, by its id (`PAR::…`, `MED::…`).
    ///
    /// # Errors
    /// As [`scans`](Self::scans).
    pub async fn member_traffic_status(
        &self,
        synchronizer_id: &str,
        member_id: &str,
    ) -> Result<TrafficStatus> {
        #[derive(Deserialize)]
        struct Reply {
            traffic_status: Status,
        }
        #[derive(Deserialize)]
        struct Status {
            actual: Actual,
            target: Target,
        }
        #[derive(Deserialize)]
        struct Actual {
            total_consumed: i64,
            total_limit: i64,
        }
        #[derive(Deserialize)]
        struct Target {
            total_purchased: i64,
        }
        self.scan_only("a member's traffic status")?;
        let url = self.url(
            "v0",
            &[
                "domains",
                synchronizer_id,
                "members",
                member_id,
                "traffic-status",
            ],
        )?;
        let reply: Reply = self.get(url).await?;
        Ok(TrafficStatus {
            total_consumed: reply.traffic_status.actual.total_consumed,
            total_limit: reply.traffic_status.actual.total_limit,
            total_purchased: reply.traffic_status.target.total_purchased,
        })
    }

    /// The Amulet Name Service entry of `party`, or `None` if it has none.
    ///
    /// # Errors
    /// As [`dso_party_id`](Self::dso_party_id).
    pub async fn ans_entry_by_party(&self, party: &str) -> Result<Option<AnsEntry>> {
        self.ans_entry(&["ans-entries", "by-party", party]).await
    }

    /// The Amulet Name Service entry for `name`, or `None` if there is none.
    ///
    /// # Errors
    /// As [`dso_party_id`](Self::dso_party_id).
    pub async fn ans_entry_by_name(&self, name: &str) -> Result<Option<AnsEntry>> {
        self.ans_entry(&["ans-entries", "by-name", name]).await
    }

    async fn ans_entry(&self, segments: &[&str]) -> Result<Option<AnsEntry>> {
        #[derive(Deserialize)]
        struct Reply {
            entry: AnsEntry,
        }
        let reply: Option<Reply> = self.get_optional(self.url("v0", segments)?).await?;
        Ok(reply.map(|reply| reply.entry))
    }

    /// Up to `page_size` unexpired Amulet Name Service entries, those whose
    /// name starts with `name_prefix` where one is given.
    ///
    /// # Errors
    /// As [`dso_party_id`](Self::dso_party_id).
    pub async fn ans_entries(
        &self,
        name_prefix: Option<&str>,
        page_size: u32,
    ) -> Result<Vec<AnsEntry>> {
        #[derive(Deserialize)]
        struct Reply {
            entries: Vec<AnsEntry>,
        }
        let mut url = self.url("v0", &["ans-entries"])?;
        {
            let mut query = url.query_pairs_mut();
            if let Some(name_prefix) = name_prefix {
                query.append_pair("name_prefix", name_prefix);
            }
            query.append_pair("page_size", &page_size.to_string());
        }
        let reply: Reply = self.get(url).await?;
        Ok(reply.entries)
    }

    /// Up to `page_size` updates after `after`, oldest first, or from the
    /// beginning of history where `after` is `None`.
    ///
    /// The last update's [`position`](ScanUpdate::position) is the `after` of
    /// the next page, and fewer than `page_size` updates means the end of
    /// what Scan has so far. Every Scan returns the same updates in the same
    /// order.
    ///
    /// # Errors
    /// As [`scans`](Self::scans).
    pub async fn updates(
        &self,
        after: Option<&HistoryPosition>,
        page_size: u32,
    ) -> Result<Vec<ScanUpdate>> {
        #[derive(Deserialize)]
        struct Reply {
            transactions: Vec<ScanUpdate>,
        }
        self.scan_only("the update history")?;
        let reply: Reply = self
            .post(
                self.url("v2", &["updates"])?,
                &history_request(after, page_size)?,
            )
            .await?;
        Ok(reply.transactions)
    }

    /// The update with this id, or `None` if Scan has none.
    ///
    /// # Errors
    /// As [`scans`](Self::scans).
    pub async fn update(&self, update_id: &str) -> Result<Option<ScanUpdate>> {
        self.scan_only("an update")?;
        self.get_optional(self.url("v2", &["updates", update_id])?)
            .await
    }

    /// Up to `page_size` entries of the event history after `after`: each
    /// update with the mediator's verdict on it. Paged as
    /// [`updates`](Self::updates).
    ///
    /// # Errors
    /// As [`scans`](Self::scans).
    pub async fn events(
        &self,
        after: Option<&HistoryPosition>,
        page_size: u32,
    ) -> Result<Vec<ScanEventItem>> {
        #[derive(Deserialize)]
        struct Reply {
            events: Vec<ScanEventItem>,
        }
        self.scan_only("the event history")?;
        let reply: Reply = self
            .post(
                self.url("v0", &["events"])?,
                &history_request(after, page_size)?,
            )
            .await?;
        Ok(reply.events)
    }

    /// The record time of the most recent snapshot of the active contracts
    /// taken before `before` (an RFC 3339 time) in `migration_id`, or `None`
    /// if there is none.
    ///
    /// # Errors
    /// As [`scans`](Self::scans).
    pub async fn acs_snapshot_timestamp(
        &self,
        before: &str,
        migration_id: i64,
    ) -> Result<Option<String>> {
        #[derive(Deserialize)]
        struct Reply {
            record_time: String,
        }
        self.scan_only("the snapshot times")?;
        let mut url = self.url("v0", &["state", "acs", "snapshot-timestamp"])?;
        url.query_pairs_mut()
            .append_pair("before", before)
            .append_pair("migration_id", &migration_id.to_string());
        let reply: Option<Reply> = self.get_optional(url).await?;
        Ok(reply.map(|reply| reply.record_time))
    }

    /// A page of a snapshot of the active contracts.
    ///
    /// Splice 0.8.4 and later: an older Scan has no `/v2/state/acs` and
    /// answers `404`.
    ///
    /// # Errors
    /// As [`scans`](Self::scans), and [`Error::InvalidRequest`] for a page
    /// size Scan does not allow.
    pub async fn acs_snapshot(&self, request: &SnapshotRequest) -> Result<AcsSnapshot> {
        self.scan_only("the snapshots of the active contracts")?;
        let mut body = serde_json::json!({
            "migration_id": request.migration_id,
            "record_time": request.record_time,
            "record_time_match": request.record_time_match(),
            "page_size": page_size(request.page_size)?,
        });
        if let Some(after) = &request.after {
            set(&mut body, "after", Value::from(after.as_str()));
        }
        if !request.parties.is_empty() {
            set(&mut body, "party_ids", Value::from(request.parties.clone()));
        }
        if !request.templates.is_empty() {
            set(
                &mut body,
                "templates",
                Value::from(request.templates.clone()),
            );
        }
        self.post(self.url("v2", &["state", "acs"])?, &body).await
    }

    /// A page of the Amulet and locked Amulet contracts the request's parties
    /// owned at a snapshot. Splice 0.8.4 and later, as
    /// [`acs_snapshot`](Self::acs_snapshot).
    ///
    /// # Errors
    /// As [`acs_snapshot`](Self::acs_snapshot), and
    /// [`Error::InvalidRequest`] if the request names no party.
    pub async fn holdings_state(&self, request: &SnapshotRequest) -> Result<AcsSnapshot> {
        self.scan_only("the holdings at a snapshot")?;
        let mut body = serde_json::json!({
            "migration_id": request.migration_id,
            "record_time": request.record_time,
            "record_time_match": request.record_time_match(),
            "page_size": page_size(request.page_size)?,
            "owner_party_ids": request.owners("a holdings read")?,
        });
        if let Some(after) = &request.after {
            set(&mut body, "after", Value::from(after.as_str()));
        }
        self.post(self.url("v2", &["holdings", "state"])?, &body)
            .await
    }

    /// The Amulet totals the request's parties held at a snapshot. The page
    /// size, page token and templates of the request are not used.
    ///
    /// # Errors
    /// As [`dso_party_id`](Self::dso_party_id), and
    /// [`Error::InvalidRequest`] if the request names no party.
    pub async fn holdings_summary(&self, request: &SnapshotRequest) -> Result<HoldingsSummaries> {
        let body = serde_json::json!({
            "migration_id": request.migration_id,
            "record_time": request.record_time,
            "record_time_match": request.record_time_match(),
            "owner_party_ids": request.owners("a holdings summary")?,
        });
        self.post(self.url("v1", &["holdings", "summary"])?, &body)
            .await
    }

    /// `GET` a path this client has no method for, and return the JSON as it
    /// came. `version` is the first path segment (`v0`, `v1`, …).
    ///
    /// For the rest of Splice's API: the same transport as every other call
    /// here (the token, no redirects, a bounded reply, encoded segments) and
    /// nothing checked about what comes back. Through a validator the path is
    /// taken under its `scan-proxy` prefix.
    ///
    /// # Errors
    /// As any HTTP call.
    pub async fn get_json(
        &self,
        version: &str,
        segments: &[&str],
        query: &[(&str, &str)],
    ) -> Result<Value> {
        let mut url = self.url(version, segments)?;
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }
        self.get(url).await
    }

    /// `POST` a JSON body to a path this client has no method for, and return
    /// the JSON as it came. See [`get_json`](Self::get_json).
    ///
    /// # Errors
    /// As any HTTP call.
    pub async fn post_json(&self, version: &str, segments: &[&str], body: &Value) -> Result<Value> {
        self.post(self.url(version, segments)?, body).await
    }

    /// Send a request with the bearer token, when one is configured.
    async fn send(&self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let request = match self.auth.bearer().await? {
            Some(token) => request.bearer_auth(token),
            None => request,
        };
        request.send().await.map_err(|e| transport_error(&e, SCAN))
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, url: reqwest::Url) -> Result<T> {
        let response = self.send(self.http.get(url.clone())).await?;
        decode(response, url.as_str()).await
    }

    async fn post<T: serde::de::DeserializeOwned>(
        &self,
        url: reqwest::Url,
        body: &(impl Serialize + ?Sized),
    ) -> Result<T> {
        let response = self.send(self.http.post(url.clone()).json(body)).await?;
        decode(response, url.as_str()).await
    }

    /// A `GET` whose `404` is an answer: there is no such entry.
    ///
    /// Only when Scan is the one answering. A `404` whose body is not JSON did
    /// not come from a Scan handler: it is a base URL one path component off,
    /// or a gateway in front of one, and reporting that as "no such entry"
    /// would be a steady, quiet `None` where every other call fails loudly.
    async fn get_optional<T: serde::de::DeserializeOwned>(
        &self,
        url: reqwest::Url,
    ) -> Result<Option<T>> {
        let response = self.send(self.http.get(url.clone())).await?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            let body = read_body_from(response, ERROR_BODY_CAP, url.as_str(), SCAN).await?;
            let trimmed = body.trim();
            if serde_json::from_str::<ErrorResponse>(trimmed).is_ok() {
                return Ok(None);
            }
            tracing::warn!(%url, "a 404 that is not Scan's own — check the base URL");
            return Err(Error::http_at(404, truncate(trimmed), url.as_str()));
        }
        Ok(Some(decode(response, url.as_str()).await?))
    }
}

/// The HTTP client every [`ScanClient`] starts from: a deadline, and no
/// redirects.
fn default_http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(DEFAULT_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| Error::InvalidRequest(format!("cannot build an HTTP client: {e}")))
}

fn page_size(page_size: u32) -> Result<u32> {
    if page_size == 0 || page_size > MAX_PAGE_SIZE {
        return Err(Error::InvalidRequest(format!(
            "a page of {page_size} is not one Scan serves: it takes 1 to {MAX_PAGE_SIZE}"
        )));
    }
    Ok(page_size)
}

/// Set a field of a JSON object built on the line above. `object[key] = …`
/// does the same and panics when `object` is not an object.
fn set(object: &mut Value, key: &str, value: Value) {
    if let Some(fields) = object.as_object_mut() {
        fields.insert(key.to_string(), value);
    }
}

/// The body of a history read: where to read on from, and how much.
fn history_request(after: Option<&HistoryPosition>, size: u32) -> Result<Value> {
    let mut body = serde_json::json!({ "page_size": page_size(size)? });
    if let Some(after) = after {
        let after = serde_json::to_value(after).map_err(|e| {
            Error::InvalidRequest(format!("a history position that is not JSON: {e}"))
        })?;
        set(&mut body, "after", after);
    }
    Ok(body)
}

/// One place where a non-success status becomes an error and a body becomes a
/// value, so every call reports failure the same way.
async fn decode<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
    url: &str,
) -> Result<T> {
    let status = response.status();
    let cap = if status.is_success() {
        MAX_RESPONSE_BYTES
    } else {
        ERROR_BODY_CAP
    };
    let body = read_body_from(response, cap, url, SCAN).await?;
    if !status.is_success() {
        // Splice's error body is `{"error": "…"}`; anything else is passed
        // through. The body stays the body, since the shared error model
        // reads it; the URL is in the log line.
        tracing::warn!(%url, status = status.as_u16(), "Scan answered with an error");
        let message = serde_json::from_str::<ErrorResponse>(&body)
            .map(|e| e.error)
            .unwrap_or(body);
        return Err(Error::http_at(status.as_u16(), truncate(&message), url));
    }
    serde_json::from_str(&body).map_err(|e| {
        Error::UnexpectedResponse(format!(
            "{url} returned a body this does not understand: {e}"
        ))
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn contract(contract_id: &str, payload: Value) -> ContractWithState {
        ContractWithState {
            contract: ScanContract {
                template_id: "pkg:Splice.Round:OpenMiningRound".to_string(),
                contract_id: contract_id.to_string(),
                payload,
                created_event_blob: base64::engine::general_purpose::STANDARD.encode([1, 2, 3]),
                created_at: "2026-11-26T10:00:00Z".to_string(),
            },
            synchronizer_id: Some("global-domain::1220aa".to_string()),
        }
    }

    fn round(number: i64, opens_at: &str) -> ContractWithState {
        contract(
            &format!("00round{number}"),
            serde_json::json!({
                "round": { "number": number.to_string() },
                "opensAt": opens_at,
                "targetClosesAt": "2026-11-26T12:00:00Z",
            }),
        )
    }

    fn at(time: &str) -> Timestamp {
        serde_json::from_value(Value::from(time)).unwrap()
    }

    /// Several rounds are open at once and the newest has usually not reached
    /// its opening time. The one to use is the highest that has: the lowest
    /// is archived next, and the one above the highest is not usable yet.
    #[test]
    fn the_latest_open_round_is_the_highest_one_whose_opening_time_has_come() {
        let rounds = MiningRounds {
            open: vec![
                round(11, "2026-11-26T10:10:00.000000Z"),
                round(10, "2026-11-26T10:00:00.000000Z"),
                round(12, "2026-11-26T10:20:00.000000Z"),
            ],
            issuing: Vec::new(),
        };
        let pick = |now: &str| {
            rounds
                .latest_open(at(now))
                .map(|round| round.contract.contract_id.clone())
        };
        assert_eq!(pick("2026-11-26T10:15:00Z").unwrap(), "00round11");
        assert_eq!(pick("2026-11-26T10:05:00Z").unwrap(), "00round10");
        // The opening time itself counts as open.
        assert_eq!(pick("2026-11-26T10:20:00Z").unwrap(), "00round12");
        assert_eq!(pick("2026-11-27T00:00:00Z").unwrap(), "00round12");

        // Before any round opens there is none, and that passes by itself.
        let error = pick("2026-11-26T09:00:00Z").unwrap_err();
        assert!(error.is_retriable(), "{error:?}");
        assert!(
            error.to_string().contains("3 open mining rounds"),
            "{error}"
        );
    }

    /// A round number is a Daml `Int`, which JSON writes as a string, and
    /// compares as a number: round 9 is before round 10.
    #[test]
    fn round_numbers_compare_as_numbers_whether_written_as_text_or_not() {
        let mut as_number = round(9, "2026-11-26T09:50:00Z");
        as_number.contract.payload["round"]["number"] = Value::from(9);
        let rounds = MiningRounds {
            open: vec![round(10, "2026-11-26T10:00:00Z"), as_number],
            issuing: Vec::new(),
        };
        let latest = rounds.latest_open(at("2026-11-26T11:00:00Z")).unwrap();
        assert_eq!(latest.contract.contract_id, "00round10");
    }

    #[test]
    fn a_round_that_is_not_an_open_mining_round_is_an_unexpected_response() {
        let rounds = MiningRounds {
            open: vec![contract("00odd", serde_json::json!({ "round": {} }))],
            issuing: Vec::new(),
        };
        let error = rounds.latest_open(at("2026-11-26T11:00:00Z")).unwrap_err();
        assert!(matches!(error, Error::UnexpectedResponse(_)), "{error:?}");
        assert!(error.to_string().contains("00odd"), "{error}");
    }

    /// gRPC takes the blob as bytes and the JSON API as the base64 Scan sent,
    /// and both name the synchronizer the contract is on.
    #[test]
    fn a_contract_is_disclosed_as_bytes_for_grpc_and_base64_for_json() {
        let rules = contract("00rules", serde_json::json!({}));
        let disclosed = rules.disclosed().unwrap();
        assert_eq!(disclosed.contract_id, "00rules");
        assert_eq!(disclosed.created_event_blob, [1, 2, 3]);
        assert_eq!(disclosed.synchronizer_id, "global-domain::1220aa");
        let template = disclosed.template_id.unwrap();
        assert_eq!(
            (
                template.package_id.as_str(),
                template.module_name.as_str(),
                template.entity_name.as_str()
            ),
            ("pkg", "Splice.Round", "OpenMiningRound")
        );

        let json = rules.disclosed_json().unwrap();
        assert_eq!(json["createdEventBlob"], "AQID");
        assert_eq!(json["synchronizerId"], "global-domain::1220aa");
        assert_eq!(json["templateId"], "pkg:Splice.Round:OpenMiningRound");
    }

    #[test]
    fn a_contract_that_cannot_be_disclosed_says_why() {
        let mut in_transit = contract("00moving", serde_json::json!({}));
        in_transit.synchronizer_id = None;
        let error = in_transit.disclosed().unwrap_err();
        assert!(error.is_retriable(), "{error:?}");
        assert!(in_transit.disclosed_json().is_err());

        for template_id in ["pkg:Module", "pkg:Module:Entity:Extra", "::", ""] {
            let mut odd = contract("00odd", serde_json::json!({}));
            odd.contract.template_id = template_id.to_string();
            let error = odd.disclosed().unwrap_err();
            assert!(matches!(error, Error::UnexpectedResponse(_)), "{error:?}");
        }

        let mut odd = contract("00blob", serde_json::json!({}));
        odd.contract.created_event_blob = "not base64!!".to_string();
        let error = odd.disclosed().unwrap_err();
        assert!(error.to_string().contains("00blob"), "{error}");
    }

    /// The two kinds of update carry no tag: a transaction is the one with
    /// events, a reassignment the one with an event.
    #[test]
    fn updates_are_told_apart_by_their_fields() {
        let transaction: ScanUpdate = serde_json::from_value(serde_json::json!({
            "update_id": "1220tx",
            "migration_id": 4,
            "workflow_id": "",
            "record_time": "2026-11-26T10:00:00.000000Z",
            "synchronizer_id": "global-domain::1220aa",
            "effective_at": "2026-11-26T09:59:59.000000Z",
            "root_event_ids": ["1220tx:0"],
            "events_by_id": {
                "1220tx:0": {
                    "event_type": "exercised_event",
                    "event_id": "1220tx:0",
                    "contract_id": "00rules",
                    "template_id": "pkg:Splice.AmuletRules:AmuletRules",
                    "package_name": "splice-amulet",
                    "choice": "AmuletRules_Transfer",
                    "choice_argument": {},
                    "child_event_ids": ["1220tx:1"],
                    "exercise_result": {},
                    "consuming": false,
                    "acting_parties": ["alice::1220"],
                },
                "1220tx:1": {
                    "event_type": "created_event",
                    "event_id": "1220tx:1",
                    "contract_id": "00amulet",
                    "template_id": "pkg:Splice.Amulet:Amulet",
                    "package_name": "splice-amulet",
                    "create_arguments": { "owner": "bob::1220" },
                    "created_at": "2026-11-26T09:59:59.000000Z",
                    "signatories": ["DSO::1220"],
                    "observers": [],
                },
            },
        }))
        .unwrap();
        assert_eq!(transaction.update_id(), "1220tx");
        assert_eq!(
            transaction.position(),
            HistoryPosition {
                migration_id: 4,
                record_time: "2026-11-26T10:00:00.000000Z".to_string(),
            }
        );
        let ScanUpdate::Transaction(transaction) = transaction else {
            panic!("a transaction");
        };
        assert!(matches!(
            transaction.events_by_id["1220tx:0"],
            ScanEvent::Exercised(_)
        ));
        assert!(matches!(
            transaction.events_by_id["1220tx:1"],
            ScanEvent::Created(_)
        ));

        let unassigned: ScanUpdate = serde_json::from_value(serde_json::json!({
            "update_id": "1220out",
            "offset": "000a",
            "record_time": "2026-11-26T10:01:00.000000Z",
            "event": {
                "submitter": "alice::1220",
                "source_synchronizer": "one::1220",
                "target_synchronizer": "two::1220",
                "migration_id": 5,
                "unassign_id": "u1",
                "reassignment_counter": 1,
                "contract_id": "00moving",
            },
        }))
        .unwrap();
        assert_eq!(unassigned.position().migration_id, 5);
        assert!(matches!(
            unassigned,
            ScanUpdate::Reassignment(ScanReassignment {
                event: ReassignmentEvent::Unassignment(_),
                ..
            })
        ));

        let neither = serde_json::from_value::<ScanUpdate>(serde_json::json!({
            "update_id": "1220odd",
        }))
        .unwrap_err();
        assert!(neither.to_string().contains("events_by_id"), "{neither}");
    }

    /// The position serializes as the `after` object the history reads take.
    #[test]
    fn a_history_position_is_the_after_of_the_next_page() {
        let after = HistoryPosition {
            migration_id: 4,
            record_time: "2026-11-26T10:00:00.000000Z".to_string(),
        };
        assert_eq!(
            history_request(Some(&after), 50).unwrap(),
            serde_json::json!({
                "page_size": 50,
                "after": {
                    "after_migration_id": 4,
                    "after_record_time": "2026-11-26T10:00:00.000000Z",
                },
            })
        );
        assert_eq!(
            history_request(None, 1).unwrap(),
            serde_json::json!({ "page_size": 1 })
        );
        for size in [0, 1001] {
            let error = history_request(None, size).unwrap_err();
            assert!(matches!(error, Error::InvalidRequest(_)), "{error:?}");
        }
    }
}
