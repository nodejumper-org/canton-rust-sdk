//! Typed request builders for the client's read paths.
//!
//! The plain client methods ([`CantonClient::updates`],
//! [`CantonClient::completions`]) cover the common case — wildcard filters,
//! ledger-effects shape, an unbounded stream — with a two-argument call.
//! These builders open up the rest of the request surface without touching
//! those signatures: a bounded stream for catch-up reads (`until`), template
//! and interface filters, the ACS-delta shape, created-event blobs, and the
//! completion stream's `user_id`.
//!
//! Defaults match the plain methods exactly, so
//! `client.updates_with(UpdatesRequest::new(parties, 0))` and
//! `client.updates(parties, 0)` issue the same wire request.
//!
//! ```no_run
//! # async fn run(client: canton_ledger::CantonClient, party: String)
//! #     -> canton_ledger::Result<()> {
//! use canton_ledger::request::UpdatesRequest;
//! use tokio_stream::StreamExt as _;
//!
//! // A bounded catch-up read: everything between two offsets, one template.
//! let stream = client
//!     .updates_with(
//!         UpdatesRequest::new(vec![party], 0)
//!             .until(41_000)
//!             .for_templates(["#my-app:My.Module:Asset"])?,
//!     )
//!     .await?;
//! tokio::pin!(stream);
//! while let Some(update) = stream.next().await {
//!     println!("{:?}", update?);
//! }
//! # Ok(()) }
//! ```
//!
//! [`CantonClient::updates`]: crate::CantonClient::updates
//! [`CantonClient::completions`]: crate::CantonClient::completions

use canton_core::{Error, Result};
use canton_proto::com::daml::ledger::api::v2 as pb;

/// The shape of transactions in an update stream — which events are returned
/// and who has to see them (see the Ledger API's `TransactionShape`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransactionShape {
    /// Created and archived events describing the net change to the active
    /// contract set; a requesting party must be a **stakeholder** of an event.
    AcsDelta,
    /// Create and (non-)consuming exercise events as executed; a requesting
    /// party must be a **witness** of an event. The default, matching
    /// [`CantonClient::updates`](crate::CantonClient::updates).
    #[default]
    LedgerEffects,
}

impl TransactionShape {
    pub(crate) fn as_grpc(self) -> pb::TransactionShape {
        match self {
            Self::AcsDelta => pb::TransactionShape::AcsDelta,
            Self::LedgerEffects => pb::TransactionShape::LedgerEffects,
        }
    }
}

/// A builder for [`CantonClient::updates_with`] /
/// [`CantonClient::updates_resumable_with`]: the update stream with the full
/// request surface exposed.
///
/// [`CantonClient::updates_with`]: crate::CantonClient::updates_with
/// [`CantonClient::updates_resumable_with`]: crate::CantonClient::updates_resumable_with
#[derive(Clone, Debug)]
#[must_use = "a request does nothing until passed to a client method"]
// The bools mirror four genuinely independent wire flags of `UpdateFormat`;
// folding them into state enums would obscure the 1:1 proto mapping.
#[allow(clippy::struct_excessive_bools)]
pub struct UpdatesRequest {
    pub(crate) parties: Vec<String>,
    pub(crate) begin_exclusive: i64,
    end_inclusive: Option<i64>,
    shape: TransactionShape,
    /// Parsed template filters (empty + empty interfaces ⇒ wildcard).
    templates: Vec<pb::Identifier>,
    /// Parsed interface filters.
    interfaces: Vec<pb::Identifier>,
    include_created_event_blobs: bool,
    include_reassignments: bool,
    include_topology_events: bool,
    verbose: bool,
    any_party: bool,
    descending: bool,
}

impl UpdatesRequest {
    /// Refuse a request the participant is certain to refuse.
    ///
    /// An offset is a position on the ledger, so a negative one is not a range
    /// the participant can answer — it comes back as `INVALID_ARGUMENT` after a
    /// round trip, reading like a server-side problem. So does an inverted
    /// range, and a request naming no parties returns nothing at all while
    /// looking like a working subscription.
    pub(crate) fn validate(&self) -> crate::Result<()> {
        use canton_core::Error;
        if self.parties.is_empty() && !self.any_party {
            return Err(Error::InvalidRequest(
                "a read needs at least one party (or filters_for_any_party)".to_string(),
            ));
        }
        if self.begin_exclusive < 0 {
            return Err(Error::InvalidRequest(format!(
                "begin offset must not be negative, got {}",
                self.begin_exclusive
            )));
        }
        if let Some(end) = self.end_inclusive {
            if end < 0 {
                return Err(Error::InvalidRequest(format!(
                    "end offset must not be negative, got {end}"
                )));
            }
            if !self.descending && end < self.begin_exclusive {
                return Err(Error::InvalidRequest(format!(
                    "end offset {end} is before the begin offset {}",
                    self.begin_exclusive
                )));
            }
        }
        Ok(())
    }

    /// Updates visible to `parties`, starting after `begin_exclusive`. The
    /// defaults beyond that are the plain [`updates`] ones: unbounded,
    /// ledger-effects shape, all templates, reassignments included, verbose
    /// (labelled) records.
    ///
    /// [`updates`]: crate::CantonClient::updates
    pub fn new(parties: Vec<String>, begin_exclusive: i64) -> Self {
        Self {
            parties,
            begin_exclusive,
            end_inclusive: None,
            shape: TransactionShape::LedgerEffects,
            templates: Vec::new(),
            interfaces: Vec::new(),
            include_created_event_blobs: false,
            include_reassignments: true,
            include_topology_events: false,
            verbose: true,
            any_party: false,
            descending: false,
        }
    }

    /// Bound the stream: only updates at offsets `<= end_inclusive`, after
    /// which the stream ends. This is the catch-up/sync-tool form — without
    /// it the stream is live and never terminates.
    pub fn until(mut self, end_inclusive: i64) -> Self {
        self.end_inclusive = Some(end_inclusive);
        self
    }

    /// Return updates in descending offset order (newest first). Requires a
    /// bounded request ([`Self::until`]); rejected on the resumable stream,
    /// whose position tracking assumes ascending order.
    pub fn descending(mut self) -> Self {
        self.descending = true;
        self
    }

    /// Additionally apply this request's filter set for **any** party the
    /// token can read for (`filters_for_any_party`) — the full-ledger
    /// ingestion form. Requires wildcard read authorization on the
    /// participant; the explicit `parties` list (which may be empty) is still
    /// applied on top.
    pub fn for_any_party(mut self) -> Self {
        self.any_party = true;
        self
    }

    /// Select the transaction shape (default: ledger effects).
    pub fn with_shape(mut self, shape: TransactionShape) -> Self {
        self.shape = shape;
        self
    }

    /// Only events of these templates. Each id is `package:Module:Entity`,
    /// where `package` is a package id or a `#package-name` reference (the
    /// SCU-friendly form). May be combined with [`Self::for_interfaces`];
    /// each call adds to the filter.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] on a malformed id.
    pub fn for_templates<I, S>(mut self, template_ids: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for id in template_ids {
            self.templates.push(parse_identifier(id.as_ref())?);
        }
        Ok(self)
    }

    /// Only events of contracts implementing these interfaces (the returned
    /// created events carry the interface views). Same id format and
    /// accumulation as [`Self::for_templates`].
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] on a malformed id.
    pub fn for_interfaces<I, S>(mut self, interface_ids: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for id in interface_ids {
            self.interfaces.push(parse_identifier(id.as_ref())?);
        }
        Ok(self)
    }

    /// Include each created event's `created_event_blob`, for use as a
    /// disclosed contract in later commands.
    pub fn with_created_event_blobs(mut self) -> Self {
        self.include_created_event_blobs = true;
        self
    }

    /// Exclude reassignment events (included by default).
    pub fn without_reassignments(mut self) -> Self {
        self.include_reassignments = false;
        self
    }

    /// Include participant-authorization topology events for the requested
    /// parties (excluded by default).
    pub fn with_topology_events(mut self) -> Self {
        self.include_topology_events = true;
        self
    }

    /// Omit record labels from the returned values (verbose is the default).
    /// Non-verbose payloads are smaller; typed decoding handles both.
    pub fn non_verbose(mut self) -> Self {
        self.verbose = false;
        self
    }

    /// Restart marker for the resumable stream: same request, new begin.
    pub(crate) fn resume_after(&self, offset: i64) -> Self {
        let mut request = self.clone();
        request.begin_exclusive = offset;
        request
    }

    /// The bounds, for the paged read.
    pub(crate) fn bounds(&self) -> (i64, Option<i64>) {
        (self.begin_exclusive, self.end_inclusive)
    }

    /// Whether descending order was requested.
    pub(crate) fn is_descending(&self) -> bool {
        self.descending
    }

    /// The wire `UpdateFormat` (shared by the streaming and paged reads).
    pub(crate) fn update_format(&self) -> pb::UpdateFormat {
        let filters = build_filters(
            &self.templates,
            &self.interfaces,
            self.include_created_event_blobs,
        );
        let event_format = |verbose: bool| pb::EventFormat {
            filters_by_party: self
                .parties
                .iter()
                .map(|party| (party.clone(), filters.clone()))
                .collect(),
            filters_for_any_party: self.any_party.then(|| filters.clone()),
            verbose,
        };
        pb::UpdateFormat {
            include_transactions: Some(pb::TransactionFormat {
                event_format: Some(event_format(self.verbose)),
                transaction_shape: self.shape.as_grpc() as i32,
            }),
            // Reassignment events are always ACS-delta shaped and never
            // verbose-labelled, but share the party filters.
            include_reassignments: self
                .include_reassignments
                .then(|| event_format(self.verbose)),
            include_topology_events: self.include_topology_events.then(|| pb::TopologyFormat {
                include_participant_authorization_events: Some(
                    pb::ParticipantAuthorizationTopologyFormat {
                        parties: self.parties.clone(),
                    },
                ),
            }),
        }
    }

    /// The wire request.
    pub(crate) fn into_grpc(self) -> pb::GetUpdatesRequest {
        pb::GetUpdatesRequest {
            begin_exclusive: self.begin_exclusive,
            end_inclusive: self.end_inclusive,
            descending_order: self.descending,
            update_format: Some(self.update_format()),
        }
    }

    /// The JSON API request body (`POST /v2/updates` and the WS lane) — the
    /// same query as [`Self::update_format`], in the JSON transport's shape.
    pub(crate) fn json_body(&self) -> serde_json::Value {
        let event_format = || {
            event_format_json(
                &self.parties,
                &self.templates,
                &self.interfaces,
                self.include_created_event_blobs,
                self.verbose,
                self.any_party,
            )
        };
        let shape = match self.shape {
            TransactionShape::AcsDelta => "TRANSACTION_SHAPE_ACS_DELTA",
            TransactionShape::LedgerEffects => "TRANSACTION_SHAPE_LEDGER_EFFECTS",
        };
        let mut update_format = serde_json::json!({
            "includeTransactions": {
                "eventFormat": event_format(),
                "transactionShape": shape,
            }
        });
        if self.include_reassignments {
            update_format["includeReassignments"] = event_format();
        }
        if self.include_topology_events {
            update_format["includeTopologyEvents"] = serde_json::json!({
                "includeParticipantAuthorizationEvents": { "parties": self.parties }
            });
        }
        let mut body = serde_json::json!({
            "beginExclusive": self.begin_exclusive,
            "updateFormat": update_format,
        });
        if let Some(end) = self.end_inclusive {
            body["endInclusive"] = serde_json::json!(end);
        }
        if self.descending {
            body["descendingOrder"] = serde_json::json!(true);
        }
        body
    }
}

/// A builder for the Active Contract Set reads
/// ([`CantonClient::active_contracts_with`],
/// [`CantonClient::active_contracts_page_with`],
/// [`CantonClient::active_contracts_resumable_with`]): the snapshot with the
/// full request surface exposed.
///
/// [`CantonClient::active_contracts_with`]: crate::CantonClient::active_contracts_with
/// [`CantonClient::active_contracts_page_with`]: crate::CantonClient::active_contracts_page_with
/// [`CantonClient::active_contracts_resumable_with`]: crate::CantonClient::active_contracts_resumable_with
#[derive(Clone, Debug)]
#[must_use = "a request does nothing until passed to a client method"]
pub struct ActiveContractsRequest {
    pub(crate) parties: Vec<String>,
    pub(crate) active_at_offset: i64,
    templates: Vec<pb::Identifier>,
    interfaces: Vec<pb::Identifier>,
    include_created_event_blobs: bool,
    verbose: bool,
    any_party: bool,
}

impl ActiveContractsRequest {
    /// Refuse a snapshot request the participant is certain to refuse: an
    /// offset it cannot have reached, or a filter naming nobody.
    pub(crate) fn validate(&self) -> crate::Result<()> {
        use canton_core::Error;
        if self.parties.is_empty() && !self.any_party {
            return Err(Error::InvalidRequest(
                "an ACS read needs at least one party (or filters_for_any_party)".to_string(),
            ));
        }
        if self.active_at_offset < 0 {
            return Err(Error::InvalidRequest(format!(
                "active_at_offset must not be negative, got {}",
                self.active_at_offset
            )));
        }
        Ok(())
    }

    /// The ACS visible to `parties` as of `active_at_offset` (typically the
    /// current ledger end). The defaults beyond that are the plain
    /// [`active_contracts`] ones: all templates, verbose (labelled) records.
    ///
    /// [`active_contracts`]: crate::CantonClient::active_contracts
    pub fn new(parties: Vec<String>, active_at_offset: i64) -> Self {
        Self {
            parties,
            active_at_offset,
            templates: Vec::new(),
            interfaces: Vec::new(),
            include_created_event_blobs: false,
            verbose: true,
            any_party: false,
        }
    }

    /// Additionally apply this request's filter set for **any** party the
    /// token can read for (`filters_for_any_party`) — the full-ledger
    /// ingestion form. Requires wildcard read authorization on the
    /// participant; the explicit `parties` list (which may be empty) is still
    /// applied on top.
    pub fn for_any_party(mut self) -> Self {
        self.any_party = true;
        self
    }

    /// Only contracts of these templates. Same id format and accumulation as
    /// [`UpdatesRequest::for_templates`].
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] on a malformed id.
    pub fn for_templates<I, S>(mut self, template_ids: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for id in template_ids {
            self.templates.push(parse_identifier(id.as_ref())?);
        }
        Ok(self)
    }

    /// Only contracts implementing these interfaces (the returned created
    /// events carry the interface views). Same id format and accumulation as
    /// [`UpdatesRequest::for_interfaces`].
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] on a malformed id.
    pub fn for_interfaces<I, S>(mut self, interface_ids: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for id in interface_ids {
            self.interfaces.push(parse_identifier(id.as_ref())?);
        }
        Ok(self)
    }

    /// Include each contract's `created_event_blob`, for use as a disclosed
    /// contract in later commands.
    pub fn with_created_event_blobs(mut self) -> Self {
        self.include_created_event_blobs = true;
        self
    }

    /// Omit record labels from the returned values (verbose is the default).
    pub fn non_verbose(mut self) -> Self {
        self.verbose = false;
        self
    }

    /// The wire `EventFormat` (shared by the streaming and paged reads).
    pub(crate) fn event_format(&self) -> pb::EventFormat {
        let filters = build_filters(
            &self.templates,
            &self.interfaces,
            self.include_created_event_blobs,
        );
        pb::EventFormat {
            filters_by_party: self
                .parties
                .iter()
                .map(|party| (party.clone(), filters.clone()))
                .collect(),
            filters_for_any_party: self.any_party.then(|| filters.clone()),
            verbose: self.verbose,
        }
    }

    /// The JSON API request body (`POST /v2/state/active-contracts` and the
    /// WS lane) — the same query as [`Self::event_format`], in the JSON
    /// transport's shape.
    pub(crate) fn json_body(&self) -> serde_json::Value {
        serde_json::json!({
            "activeAtOffset": self.active_at_offset,
            "eventFormat": event_format_json(
                &self.parties,
                &self.templates,
                &self.interfaces,
                self.include_created_event_blobs,
                self.verbose,
                self.any_party,
            ),
        })
    }
}

/// A builder for [`CantonClient::completions_with`]: the command-completion
/// stream with the full request surface exposed.
///
/// [`CantonClient::completions_with`]: crate::CantonClient::completions_with
#[derive(Clone, Debug)]
#[must_use = "a request does nothing until passed to a client method"]
pub struct CompletionsRequest {
    pub(crate) parties: Vec<String>,
    pub(crate) begin_exclusive: i64,
    user_id: Option<String>,
}

impl CompletionsRequest {
    /// Refuse a completion subscription the participant is certain to refuse.
    pub(crate) fn validate(&self) -> crate::Result<()> {
        use canton_core::Error;
        if self.parties.is_empty() {
            return Err(Error::InvalidRequest(
                "a completion subscription needs at least one party".to_string(),
            ));
        }
        if self.begin_exclusive < 0 {
            return Err(Error::InvalidRequest(format!(
                "begin offset must not be negative, got {}",
                self.begin_exclusive
            )));
        }
        Ok(())
    }

    /// Completions of commands acted on by `parties`, starting after
    /// `begin_exclusive`.
    pub fn new(parties: Vec<String>, begin_exclusive: i64) -> Self {
        Self {
            parties,
            begin_exclusive,
            user_id: None,
        }
    }

    /// Only completions of commands submitted with this `user_id` (pair it
    /// with [`Submit::with_user_id`]). Without it, the participant uses the
    /// user id carried by the access token — which fails for tokens that do
    /// not carry one (custom-claims or admin tokens).
    ///
    /// [`Submit::with_user_id`]: crate::Submit::with_user_id
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// The wire request.
    pub(crate) fn into_grpc(self) -> pb::CompletionStreamRequest {
        pb::CompletionStreamRequest {
            user_id: self.user_id.unwrap_or_default(),
            parties: self.parties,
            begin_exclusive: self.begin_exclusive,
        }
    }

    /// The JSON API request body. Gated on `ws` because the JSON Ledger API
    /// has no POST endpoint for completions — the WS lane is its only reader,
    /// so without the feature this body has nobody to build it for.
    #[cfg(feature = "ws")]
    pub(crate) fn json_body(&self) -> serde_json::Value {
        let mut body = serde_json::json!({
            "parties": self.parties,
            "beginExclusive": self.begin_exclusive,
        });
        if let Some(user_id) = &self.user_id {
            body["userId"] = serde_json::json!(user_id);
        }
        body
    }
}

/// The per-party `Filters`: wildcard when no template or interface filter was
/// added, else the union of the requested filters (per the API, each entry
/// widens the match).
fn build_filters(
    templates: &[pb::Identifier],
    interfaces: &[pb::Identifier],
    include_created_event_blobs: bool,
) -> pb::Filters {
    use pb::cumulative_filter::IdentifierFilter;

    if templates.is_empty() && interfaces.is_empty() {
        return pb::Filters {
            cumulative: vec![pb::CumulativeFilter {
                identifier_filter: Some(IdentifierFilter::WildcardFilter(pb::WildcardFilter {
                    include_created_event_blob: include_created_event_blobs,
                })),
            }],
        };
    }

    let template_filters = templates.iter().map(|id| {
        IdentifierFilter::TemplateFilter(pb::TemplateFilter {
            template_id: Some(id.clone()),
            include_created_event_blob: include_created_event_blobs,
        })
    });
    let interface_filters = interfaces.iter().map(|id| {
        IdentifierFilter::InterfaceFilter(pb::InterfaceFilter {
            interface_id: Some(id.clone()),
            include_interface_view: true,
            include_created_event_blob: include_created_event_blobs,
        })
    });
    pb::Filters {
        cumulative: template_filters
            .chain(interface_filters)
            .map(|filter| pb::CumulativeFilter {
                identifier_filter: Some(filter),
            })
            .collect(),
    }
}

/// The JSON-transport `EventFormat` for the same filter set as
/// [`build_filters`] — wildcard when no filter was added, else the union of
/// template and interface filters. The JSON API spells identifiers as
/// `package:Module:Entity` strings and wraps each oneof case in
/// `{"Case": {"value": …}}`.
fn event_format_json(
    parties: &[String],
    templates: &[pb::Identifier],
    interfaces: &[pb::Identifier],
    include_created_event_blobs: bool,
    verbose: bool,
    any_party: bool,
) -> serde_json::Value {
    use serde_json::json;

    let cumulative: Vec<serde_json::Value> = if templates.is_empty() && interfaces.is_empty() {
        vec![json!({
            "identifierFilter": {
                "WildcardFilter": {
                    "value": { "includeCreatedEventBlob": include_created_event_blobs }
                }
            }
        })]
    } else {
        let identifier = |id: &pb::Identifier| {
            format!("{}:{}:{}", id.package_id, id.module_name, id.entity_name)
        };
        templates
            .iter()
            .map(|id| {
                json!({
                    "identifierFilter": {
                        "TemplateFilter": {
                            "value": {
                                "templateId": identifier(id),
                                "includeCreatedEventBlob": include_created_event_blobs,
                            }
                        }
                    }
                })
            })
            .chain(interfaces.iter().map(|id| {
                json!({
                    "identifierFilter": {
                        "InterfaceFilter": {
                            "value": {
                                "interfaceId": identifier(id),
                                "includeInterfaceView": true,
                                "includeCreatedEventBlob": include_created_event_blobs,
                            }
                        }
                    }
                })
            }))
            .collect()
    };
    let filters_by_party: serde_json::Map<String, serde_json::Value> = parties
        .iter()
        .map(|party| (party.clone(), json!({ "cumulative": cumulative })))
        .collect();
    let mut format = json!({ "filtersByParty": filters_by_party, "verbose": verbose });
    if any_party {
        format["filtersForAnyParty"] = json!({ "cumulative": cumulative });
    }
    format
}

/// Parse `package:Module:Entity` into an [`pb::Identifier`]. The package part
/// is a package id or a `#package-name` reference; module and entity are
/// dotted names (never containing `:`).
fn parse_identifier(id: &str) -> Result<pb::Identifier> {
    let mut parts = id.splitn(3, ':');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(package), Some(module), Some(entity))
            if !package.is_empty() && !module.is_empty() && !entity.is_empty() =>
        {
            Ok(pb::Identifier {
                package_id: package.to_string(),
                module_name: module.to_string(),
                entity_name: entity.to_string(),
            })
        }
        _ => Err(Error::InvalidRequest(format!(
            "malformed identifier `{id}`: expected `package:Module:Entity` \
             (package id, or `#package-name`)"
        ))),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_default_request_matches_the_plain_updates_call() {
        // `updates_with(UpdatesRequest::new(..))` must issue byte-identical
        // wire requests to `updates(..)` — the builder only *adds* surface.
        let request = UpdatesRequest::new(vec!["alice".to_string()], 7).into_grpc();

        assert_eq!(request.begin_exclusive, 7);
        assert_eq!(request.end_inclusive, None);
        assert!(!request.descending_order);
        let format = request.update_format.unwrap();
        let transactions = format.include_transactions.unwrap();
        assert_eq!(
            transactions.transaction_shape,
            pb::TransactionShape::LedgerEffects as i32
        );
        let events = transactions.event_format.unwrap();
        assert!(events.verbose);
        assert!(events.filters_for_any_party.is_none());
        let filters = &events.filters_by_party["alice"];
        assert_eq!(filters.cumulative.len(), 1);
        assert!(matches!(
            filters.cumulative[0].identifier_filter,
            Some(pb::cumulative_filter::IdentifierFilter::WildcardFilter(
                pb::WildcardFilter {
                    include_created_event_blob: false
                }
            ))
        ));
        assert!(format.include_reassignments.is_some());
        assert!(format.include_topology_events.is_none());
    }

    #[test]
    fn every_builder_knob_reaches_the_wire_request() {
        let request = UpdatesRequest::new(vec!["alice".to_string()], 0)
            .until(41)
            .with_shape(TransactionShape::AcsDelta)
            .for_templates(["#my-app:My.Mod:Asset"])
            .unwrap()
            .for_interfaces(["#my-app:My.Api:IAsset"])
            .unwrap()
            .with_created_event_blobs()
            .without_reassignments()
            .with_topology_events()
            .non_verbose()
            .into_grpc();

        assert_eq!(request.end_inclusive, Some(41));
        let format = request.update_format.unwrap();
        assert!(format.include_reassignments.is_none());
        assert_eq!(
            format
                .include_topology_events
                .unwrap()
                .include_participant_authorization_events
                .unwrap()
                .parties,
            vec!["alice".to_string()]
        );
        let transactions = format.include_transactions.unwrap();
        assert_eq!(
            transactions.transaction_shape,
            pb::TransactionShape::AcsDelta as i32
        );
        let events = transactions.event_format.unwrap();
        assert!(!events.verbose);
        let filters = &events.filters_by_party["alice"].cumulative;
        assert_eq!(filters.len(), 2, "one template + one interface filter");
        let Some(pb::cumulative_filter::IdentifierFilter::TemplateFilter(template)) =
            &filters[0].identifier_filter
        else {
            panic!("expected a template filter first");
        };
        assert_eq!(template.template_id.as_ref().unwrap().package_id, "#my-app");
        assert!(template.include_created_event_blob);
        let Some(pb::cumulative_filter::IdentifierFilter::InterfaceFilter(interface)) =
            &filters[1].identifier_filter
        else {
            panic!("expected an interface filter second");
        };
        assert_eq!(
            interface.interface_id.as_ref().unwrap().entity_name,
            "IAsset"
        );
        assert!(interface.include_interface_view);
    }

    #[test]
    fn identifiers_parse_and_malformed_ones_are_refused() {
        let id = parse_identifier("#pkg-name:Some.Dotted.Module:Entity").unwrap();
        assert_eq!(id.package_id, "#pkg-name");
        assert_eq!(id.module_name, "Some.Dotted.Module");
        assert_eq!(id.entity_name, "Entity");

        for bad in ["", "nope", "a:b", ":Mod:Ent", "pkg::Ent", "pkg:Mod:"] {
            assert!(parse_identifier(bad).is_err(), "`{bad}` should be refused");
        }
    }

    #[test]
    fn acs_request_knobs_reach_the_wire_and_json_bodies() {
        let request = ActiveContractsRequest::new(vec!["alice".to_string()], 42)
            .for_templates(["#app:Mod:Asset"])
            .unwrap()
            .for_interfaces(["#app:Api:IAsset"])
            .unwrap()
            .with_created_event_blobs()
            .non_verbose();

        // gRPC wire shape.
        let format = request.event_format();
        assert!(!format.verbose);
        let filters = &format.filters_by_party["alice"].cumulative;
        assert_eq!(filters.len(), 2);
        assert!(matches!(
            &filters[0].identifier_filter,
            Some(pb::cumulative_filter::IdentifierFilter::TemplateFilter(t))
                if t.include_created_event_blob
        ));

        // JSON body: same query, transport-shaped.
        let body = request.json_body();
        assert_eq!(body["activeAtOffset"], 42);
        let cumulative = &body["eventFormat"]["filtersByParty"]["alice"]["cumulative"];
        assert_eq!(
            cumulative[0]["identifierFilter"]["TemplateFilter"]["value"]["templateId"],
            "#app:Mod:Asset"
        );
        assert_eq!(
            cumulative[1]["identifierFilter"]["InterfaceFilter"]["value"]["interfaceId"],
            "#app:Api:IAsset"
        );
        assert_eq!(body["eventFormat"]["verbose"], false);

        // The default is a wildcard, verbose.
        let plain = ActiveContractsRequest::new(vec!["alice".to_string()], 42).json_body();
        assert!(plain["eventFormat"]["filtersByParty"]["alice"]["cumulative"][0]
            ["identifierFilter"]["WildcardFilter"]
            .is_object());
        assert_eq!(plain["eventFormat"]["verbose"], true);
    }

    #[test]
    fn updates_json_body_mirrors_the_grpc_query() {
        let body = UpdatesRequest::new(vec!["alice".to_string()], 5)
            .until(9)
            .with_shape(TransactionShape::AcsDelta)
            .for_templates(["#app:Mod:Asset"])
            .unwrap()
            .without_reassignments()
            .with_topology_events()
            .json_body();

        assert_eq!(body["beginExclusive"], 5);
        assert_eq!(body["endInclusive"], 9);
        let format = &body["updateFormat"];
        assert_eq!(
            format["includeTransactions"]["transactionShape"],
            "TRANSACTION_SHAPE_ACS_DELTA"
        );
        assert!(format.get("includeReassignments").is_none());
        assert_eq!(
            format["includeTopologyEvents"]["includeParticipantAuthorizationEvents"]["parties"][0],
            "alice"
        );
        assert_eq!(
            format["includeTransactions"]["eventFormat"]["filtersByParty"]["alice"]["cumulative"]
                [0]["identifierFilter"]["TemplateFilter"]["value"]["templateId"],
            "#app:Mod:Asset"
        );
    }

    #[test]
    fn descending_and_any_party_reach_both_wire_shapes() {
        let request = UpdatesRequest::new(vec!["alice".to_string()], 5)
            .until(9)
            .descending()
            .for_any_party();

        let body = request.clone().json_body();
        assert_eq!(body["descendingOrder"], true);
        let format = &body["updateFormat"]["includeTransactions"]["eventFormat"];
        assert!(format["filtersForAnyParty"]["cumulative"].is_array());
        assert!(format["filtersByParty"]["alice"].is_object());

        let grpc = request.into_grpc();
        assert!(grpc.descending_order);
        let Some(format) = grpc
            .update_format
            .and_then(|f| f.include_transactions)
            .and_then(|t| t.event_format)
        else {
            panic!("expected an event format");
        };
        assert!(format.filters_for_any_party.is_some());
        assert!(format.filters_by_party.contains_key("alice"));

        // Plain requests keep both off.
        let plain = UpdatesRequest::new(vec!["alice".to_string()], 5).json_body();
        assert!(plain.get("descendingOrder").is_none());
        assert!(
            plain["updateFormat"]["includeTransactions"]["eventFormat"]
                .get("filtersForAnyParty")
                .is_none()
        );
    }

    #[test]
    fn acs_any_party_reaches_both_wire_shapes() {
        let request = ActiveContractsRequest::new(vec![], 7).for_any_party();

        let body = request.json_body();
        assert!(body["eventFormat"]["filtersForAnyParty"]["cumulative"].is_array());

        let format = request.event_format();
        assert!(format.filters_for_any_party.is_some());
        assert!(format.filters_by_party.is_empty());
    }

    #[test]
    #[cfg(feature = "ws")] // the body exists only for the WS completions lane
    fn completions_json_body_carries_the_user_id_only_when_set() {
        let plain = CompletionsRequest::new(vec!["p".to_string()], 3).json_body();
        assert!(plain.get("userId").is_none());
        assert_eq!(plain["beginExclusive"], 3);

        let scoped = CompletionsRequest::new(vec!["p".to_string()], 3)
            .with_user_id("sync-tool")
            .json_body();
        assert_eq!(scoped["userId"], "sync-tool");
    }

    #[test]
    fn completions_request_carries_the_user_id() {
        let plain = CompletionsRequest::new(vec!["p".to_string()], 3).into_grpc();
        assert_eq!(plain.user_id, "");
        assert_eq!(plain.begin_exclusive, 3);

        let scoped = CompletionsRequest::new(vec!["p".to_string()], 3)
            .with_user_id("sync-tool")
            .into_grpc();
        assert_eq!(scoped.user_id, "sync-tool");
    }
}
