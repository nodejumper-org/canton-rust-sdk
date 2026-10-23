//! Updates as the SDK's own types, the same whichever transport read them.
//!
//! The update reads hand out what the wire carries: `prost` messages on gRPC,
//! JSON values on the JSON Ledger API. Code written against one does not read
//! the other, and code that matches on the `prost` types follows every
//! re-vendored proto without a version signal (ADR-0002, ADR-0010 §4). The
//! types here are the envelope both transports describe, built from either:
//!
//! ```no_run
//! # async fn run(client: canton_ledger::CantonClient, party: String)
//! #     -> canton_ledger::Result<()> {
//! use canton_ledger::update::{Event, Update};
//! use tokio_stream::StreamExt as _;
//!
//! let stream = client
//!     .updates_typed_with(canton_ledger::UpdatesRequest::new(vec![party], 0))
//!     .await?;
//! tokio::pin!(stream);
//! while let Some(update) = stream.next().await {
//!     if let Update::Transaction(tx) = update? {
//!         for event in &tx.events {
//!             if let Event::Created(created) = event {
//!                 println!("{} created {}", tx.update_id, created.contract_id);
//!             }
//!         }
//!     }
//! }
//! # Ok(()) }
//! ```
//!
//! A payload is decoded into a generated type with
//! [`CreatedEvent::decode`], which runs the identity checks of
//! [`Template::from_created_event`](canton_daml::Template::from_created_event)
//! on either transport. The wire form stays reachable
//! ([`CreatedEvent::as_grpc`], [`CreatedEvent::as_json`]) for what these
//! types do not model.

use canton_core::{Error, Result};
use canton_daml as rt;
use canton_proto::com::daml::ledger::api::v2 as pb;
use serde_json::Value;

/// A template, interface or choice-owner id: `package:Module:Entity`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Identifier {
    /// The package id (or a `#<package-name>` reference).
    pub package_id: String,
    /// The dotted module name.
    pub module_name: String,
    /// The entity name.
    pub entity_name: String,
}

impl Identifier {
    fn from_grpc(id: &pb::Identifier) -> Self {
        Self {
            package_id: id.package_id.clone(),
            module_name: id.module_name.clone(),
            entity_name: id.entity_name.clone(),
        }
    }

    /// `package:Module:Entity`, as the JSON API spells it. Module names are
    /// dotted and entity names carry no colon, so the package is what precedes
    /// the last two parts.
    fn from_json(text: &str) -> Result<Self> {
        let mut parts = text.rsplitn(3, ':');
        match (parts.next(), parts.next(), parts.next()) {
            (Some(entity), Some(module), Some(package))
                if !entity.is_empty() && !module.is_empty() && !package.is_empty() =>
            {
                Ok(Self {
                    package_id: package.to_string(),
                    module_name: module.to_string(),
                    entity_name: entity.to_string(),
                })
            }
            _ => Err(malformed(format!(
                "`{text}` is not a package:Module:Entity id"
            ))),
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.package_id, self.module_name, self.entity_name
        )
    }
}

/// One update, as a stream, a page or a point read returns it.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Update {
    /// A committed transaction.
    Transaction(Transaction),
    /// A contract moving between synchronizers.
    Reassignment(Reassignment),
    /// A topology change affecting the subscribed parties.
    TopologyTransaction(TopologyTransaction),
    /// A position the participant names as safe to resume from; carries no
    /// change.
    OffsetCheckpoint(OffsetCheckpoint),
}

impl Update {
    /// The ledger offset of the update.
    #[must_use]
    pub fn offset(&self) -> i64 {
        match self {
            Self::Transaction(t) => t.offset,
            Self::Reassignment(r) => r.offset,
            Self::TopologyTransaction(t) => t.offset,
            Self::OffsetCheckpoint(c) => c.offset,
        }
    }

    /// The update id; a checkpoint has none.
    #[must_use]
    pub fn update_id(&self) -> Option<&str> {
        match self {
            Self::Transaction(t) => Some(&t.update_id),
            Self::Reassignment(r) => Some(&r.update_id),
            Self::TopologyTransaction(t) => Some(&t.update_id),
            Self::OffsetCheckpoint(_) => None,
        }
    }

    /// The transaction, when this update is one.
    #[must_use]
    pub fn as_transaction(&self) -> Option<&Transaction> {
        match self {
            Self::Transaction(t) => Some(t),
            _ => None,
        }
    }

    /// An element of a gRPC update stream or page.
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if the update is malformed.
    pub fn from_grpc(update: pb::get_updates_response::Update) -> Result<Self> {
        use pb::get_updates_response::Update as U;
        Ok(match update {
            U::Transaction(t) => Self::Transaction(Transaction::from_grpc(t)?),
            U::Reassignment(r) => Self::Reassignment(Reassignment::from_grpc(r)),
            U::TopologyTransaction(t) => {
                Self::TopologyTransaction(TopologyTransaction::from_grpc(t))
            }
            U::OffsetCheckpoint(c) => Self::OffsetCheckpoint(OffsetCheckpoint::from_grpc(&c)),
        })
    }

    /// The result of a gRPC point read
    /// ([`update_by_id`](crate::CantonClient::update_by_id),
    /// [`update_by_offset`](crate::CantonClient::update_by_offset)).
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if the update is malformed.
    pub fn from_grpc_point(update: pb::get_update_response::Update) -> Result<Self> {
        use pb::get_update_response::Update as U;
        Ok(match update {
            U::Transaction(t) => Self::Transaction(Transaction::from_grpc(t)?),
            U::Reassignment(r) => Self::Reassignment(Reassignment::from_grpc(r)),
            U::TopologyTransaction(t) => {
                Self::TopologyTransaction(TopologyTransaction::from_grpc(t))
            }
        })
    }

    /// An element as the JSON API returns it: `{"update": {"Transaction":
    /// {"value": …}}}` from [`updates`](crate::JsonClient::updates) and the
    /// point reads, or the inner `{"Transaction": {"value": …}}` a WebSocket
    /// frame carries.
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if the element is not an update the JSON
    /// API describes.
    pub fn from_json(element: &Value) -> Result<Self> {
        let update = element.get("update").unwrap_or(element);
        let (kind, body) = update
            .as_object()
            .and_then(|object| object.iter().next())
            .ok_or_else(|| malformed("an update element with no update in it"))?;
        let value = body.get("value").unwrap_or(body);
        Ok(match kind.as_str() {
            "Transaction" => Self::Transaction(Transaction::from_json(value)?),
            "Reassignment" => Self::Reassignment(Reassignment::from_json(value)?),
            "TopologyTransaction" => {
                Self::TopologyTransaction(TopologyTransaction::from_json(value)?)
            }
            "OffsetCheckpoint" => Self::OffsetCheckpoint(OffsetCheckpoint::from_json(value)?),
            other => return Err(malformed(format!("an update of unknown kind `{other}`"))),
        })
    }
}

/// A committed transaction.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Transaction {
    /// The update id.
    pub update_id: String,
    /// The command id, visible to the submitting parties only.
    pub command_id: String,
    /// The workflow id, if one was set.
    pub workflow_id: String,
    /// The ledger offset.
    pub offset: i64,
    /// The synchronizer the transaction was committed on.
    pub synchronizer_id: String,
    /// Ledger effective time.
    pub effective_at: Option<rt::Timestamp>,
    /// The synchronizer's record time.
    pub record_time: Option<rt::Timestamp>,
    /// The events visible to the subscribed parties, in node order.
    pub events: Vec<Event>,
}

impl Transaction {
    /// A gRPC transaction, as a stream, a point read or `submit-and-wait`
    /// returns it.
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if an event is malformed.
    pub fn from_grpc(tx: pb::Transaction) -> Result<Self> {
        let events = tx
            .events
            .into_iter()
            .filter_map(|event| event.event)
            .map(Event::from_grpc)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            update_id: tx.update_id,
            command_id: tx.command_id,
            workflow_id: tx.workflow_id,
            offset: tx.offset,
            synchronizer_id: tx.synchronizer_id,
            effective_at: tx.effective_at.as_ref().map(timestamp),
            record_time: tx.record_time.as_ref().map(timestamp),
            events,
        })
    }

    /// A JSON transaction (the `value` of a `Transaction` update, or the
    /// `transaction` of a `submit-and-wait-for-transaction` response).
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if a field is malformed.
    pub fn from_json(tx: &Value) -> Result<Self> {
        let events = tx
            .get("events")
            .and_then(Value::as_array)
            .map(|events| {
                events
                    .iter()
                    .map(Event::from_json)
                    .collect::<Result<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();
        Ok(Self {
            update_id: text(tx, "updateId")?,
            command_id: text_or_empty(tx, "commandId"),
            workflow_id: text_or_empty(tx, "workflowId"),
            offset: int(tx, "offset")?,
            synchronizer_id: text_or_empty(tx, "synchronizerId"),
            effective_at: json_time(tx, "effectiveAt")?,
            record_time: json_time(tx, "recordTime")?,
            events,
        })
    }

    /// The created events.
    pub fn created(&self) -> impl Iterator<Item = &CreatedEvent> {
        self.events.iter().filter_map(|e| match e {
            Event::Created(c) => Some(c),
            _ => None,
        })
    }

    /// The archived events.
    pub fn archived(&self) -> impl Iterator<Item = &ArchivedEvent> {
        self.events.iter().filter_map(|e| match e {
            Event::Archived(a) => Some(a),
            _ => None,
        })
    }

    /// The exercised events (present under the ledger-effects shape).
    pub fn exercised(&self) -> impl Iterator<Item = &ExercisedEvent> {
        self.events.iter().filter_map(|e| match e {
            Event::Exercised(x) => Some(x),
            _ => None,
        })
    }
}

/// An event of a transaction.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Event {
    /// A contract was created.
    Created(CreatedEvent),
    /// A contract was archived (the ACS-delta shape).
    Archived(ArchivedEvent),
    /// A choice was exercised (the ledger-effects shape).
    Exercised(ExercisedEvent),
}

impl Event {
    /// The offset of the transaction the event belongs to.
    #[must_use]
    pub fn offset(&self) -> i64 {
        match self {
            Self::Created(e) => e.offset,
            Self::Archived(e) => e.offset,
            Self::Exercised(e) => e.offset,
        }
    }

    /// The contract the event is about.
    #[must_use]
    pub fn contract_id(&self) -> &str {
        match self {
            Self::Created(e) => &e.contract_id,
            Self::Archived(e) => &e.contract_id,
            Self::Exercised(e) => &e.contract_id,
        }
    }

    /// The template of that contract.
    #[must_use]
    pub fn template_id(&self) -> &Identifier {
        match self {
            Self::Created(e) => &e.template_id,
            Self::Archived(e) => &e.template_id,
            Self::Exercised(e) => &e.template_id,
        }
    }

    fn from_grpc(event: pb::event::Event) -> Result<Self> {
        Ok(match event {
            pb::event::Event::Created(c) => Self::Created(CreatedEvent::from_grpc(c)?),
            pb::event::Event::Archived(a) => Self::Archived(ArchivedEvent::from_grpc(&a)?),
            pb::event::Event::Exercised(x) => Self::Exercised(ExercisedEvent::from_grpc(x)?),
        })
    }

    fn from_json(event: &Value) -> Result<Self> {
        if let Some(created) = event.get("CreatedEvent") {
            Ok(Self::Created(CreatedEvent::from_json(created)?))
        } else if let Some(archived) = event.get("ArchivedEvent") {
            Ok(Self::Archived(ArchivedEvent::from_json(archived)?))
        } else if let Some(exercised) = event.get("ExercisedEvent") {
            Ok(Self::Exercised(ExercisedEvent::from_json(exercised)?))
        } else {
            Err(malformed(
                "an event that is not created, archived or exercised",
            ))
        }
    }
}

/// The wire form an event was read from, kept so a payload can be decoded by
/// the codec of the transport that carried it.
#[derive(Clone, Debug)]
enum Wire<G> {
    Grpc(Box<G>),
    Json(Box<Value>),
}

/// A contract was created.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreatedEvent {
    /// The offset of the transaction.
    pub offset: i64,
    /// The node id within the transaction.
    pub node_id: i32,
    /// The new contract's id.
    pub contract_id: String,
    /// Its template.
    pub template_id: Identifier,
    /// The package name of its template.
    pub package_name: String,
    /// The subscribed parties the event is visible to.
    pub witness_parties: Vec<String>,
    /// The contract's signatories.
    pub signatories: Vec<String>,
    /// The contract's observers.
    pub observers: Vec<String>,
    /// When the contract was created.
    pub created_at: Option<rt::Timestamp>,
    /// The blob that lets another party submit with this contract disclosed;
    /// empty unless the read asked for it.
    pub created_event_blob: Vec<u8>,
    wire: Wire<pb::CreatedEvent>,
}

impl CreatedEvent {
    /// Decode the payload as the generated type `T`, checking first that the
    /// event is `T` (module, entity and package name; the package id is not
    /// compared, so an upgraded version decodes).
    ///
    /// # Errors
    /// [`ValueError`](rt::ValueError) if the event is another template or its
    /// payload does not decode as `T`.
    pub fn decode<T>(&self) -> std::result::Result<T, rt::ValueError>
    where
        T: rt::Template + serde::de::DeserializeOwned,
    {
        match &self.wire {
            Wire::Grpc(event) => T::from_created_event(event),
            Wire::Json(event) => T::from_json_created_event(event),
        }
    }

    /// The event as gRPC delivered it, if it came over gRPC.
    #[must_use]
    pub fn as_grpc(&self) -> Option<&pb::CreatedEvent> {
        match &self.wire {
            Wire::Grpc(event) => Some(event),
            Wire::Json(_) => None,
        }
    }

    /// The event as the JSON API delivered it, if it came over JSON.
    #[must_use]
    pub fn as_json(&self) -> Option<&Value> {
        match &self.wire {
            Wire::Grpc(_) => None,
            Wire::Json(event) => Some(event),
        }
    }

    fn from_grpc(event: pb::CreatedEvent) -> Result<Self> {
        let template_id = Identifier::from_grpc(
            event
                .template_id
                .as_ref()
                .ok_or_else(|| malformed("a created event without a template id"))?,
        );
        Ok(Self {
            offset: event.offset,
            node_id: event.node_id,
            contract_id: event.contract_id.clone(),
            template_id,
            package_name: event.package_name.clone(),
            witness_parties: event.witness_parties.clone(),
            signatories: event.signatories.clone(),
            observers: event.observers.clone(),
            created_at: event.created_at.as_ref().map(timestamp),
            created_event_blob: event.created_event_blob.clone(),
            wire: Wire::Grpc(Box::new(event)),
        })
    }

    fn from_json(event: &Value) -> Result<Self> {
        use base64::Engine as _;
        let blob = text_or_empty(event, "createdEventBlob");
        let created_event_blob = if blob.is_empty() {
            Vec::new()
        } else {
            base64::engine::general_purpose::STANDARD
                .decode(blob.as_bytes())
                .map_err(|e| malformed(format!("a createdEventBlob that is not base64: {e}")))?
        };
        Ok(Self {
            offset: int(event, "offset")?,
            node_id: node_id(event, "nodeId")?,
            contract_id: text(event, "contractId")?,
            template_id: Identifier::from_json(&text(event, "templateId")?)?,
            package_name: text_or_empty(event, "packageName"),
            witness_parties: strings(event, "witnessParties"),
            signatories: strings(event, "signatories"),
            observers: strings(event, "observers"),
            created_at: json_time(event, "createdAt")?,
            created_event_blob,
            wire: Wire::Json(Box::new(event.clone())),
        })
    }
}

/// A contract was archived.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ArchivedEvent {
    /// The offset of the transaction.
    pub offset: i64,
    /// The node id within the transaction.
    pub node_id: i32,
    /// The archived contract's id.
    pub contract_id: String,
    /// Its template.
    pub template_id: Identifier,
    /// The package name of its template.
    pub package_name: String,
    /// The subscribed parties the event is visible to.
    pub witness_parties: Vec<String>,
    /// The interfaces the template implements that the read filtered on.
    pub implemented_interfaces: Vec<Identifier>,
}

impl ArchivedEvent {
    fn from_grpc(event: &pb::ArchivedEvent) -> Result<Self> {
        Ok(Self {
            offset: event.offset,
            node_id: event.node_id,
            contract_id: event.contract_id.clone(),
            template_id: Identifier::from_grpc(
                event
                    .template_id
                    .as_ref()
                    .ok_or_else(|| malformed("an archived event without a template id"))?,
            ),
            package_name: event.package_name.clone(),
            witness_parties: event.witness_parties.clone(),
            implemented_interfaces: event
                .implemented_interfaces
                .iter()
                .map(Identifier::from_grpc)
                .collect(),
        })
    }

    fn from_json(event: &Value) -> Result<Self> {
        Ok(Self {
            offset: int(event, "offset")?,
            node_id: node_id(event, "nodeId")?,
            contract_id: text(event, "contractId")?,
            template_id: Identifier::from_json(&text(event, "templateId")?)?,
            package_name: text_or_empty(event, "packageName"),
            witness_parties: strings(event, "witnessParties"),
            implemented_interfaces: identifiers(event, "implementedInterfaces")?,
        })
    }
}

/// A Daml value as the transport carried it, decoded on demand.
#[derive(Clone, Debug)]
enum WireValue {
    Grpc(Box<pb::Value>),
    Json(Box<Value>),
}

impl WireValue {
    fn decode<T>(&self) -> std::result::Result<T, rt::ValueError>
    where
        T: rt::FromValue + serde::de::DeserializeOwned,
    {
        match self {
            Self::Grpc(value) => T::from_value(value),
            Self::Json(value) => serde_json::from_value((**value).clone())
                .map_err(|e| rt::ValueError::new(format!("value did not decode: {e}"))),
        }
    }
}

/// A choice was exercised.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ExercisedEvent {
    /// The offset of the transaction.
    pub offset: i64,
    /// The node id within the transaction.
    pub node_id: i32,
    /// The contract the choice was exercised on.
    pub contract_id: String,
    /// Its template.
    pub template_id: Identifier,
    /// The interface the choice was exercised through, if any.
    pub interface_id: Option<Identifier>,
    /// The choice.
    pub choice: String,
    /// Whether the choice consumed the contract.
    pub consuming: bool,
    /// The parties that exercised it.
    pub acting_parties: Vec<String>,
    /// The subscribed parties the event is visible to.
    pub witness_parties: Vec<String>,
    /// The last node of the exercise's subtree.
    pub last_descendant_node_id: i32,
    /// The package name of the template.
    pub package_name: String,
    argument: Option<WireValue>,
    result: Option<WireValue>,
}

impl ExercisedEvent {
    /// Decode the choice argument as `A` (the generated choice type).
    ///
    /// # Errors
    /// [`ValueError`](rt::ValueError) if there is no argument or it does not
    /// decode as `A`.
    pub fn decode_argument<A>(&self) -> std::result::Result<A, rt::ValueError>
    where
        A: rt::FromValue + serde::de::DeserializeOwned,
    {
        self.argument
            .as_ref()
            .ok_or_else(|| rt::ValueError::new("exercised event carries no choice argument"))?
            .decode()
    }

    /// Decode the choice's result as `R`; `None` when the event carries none.
    #[must_use]
    pub fn decode_result<R>(&self) -> Option<std::result::Result<R, rt::ValueError>>
    where
        R: rt::FromValue + serde::de::DeserializeOwned,
    {
        self.result.as_ref().map(WireValue::decode)
    }

    fn from_grpc(event: pb::ExercisedEvent) -> Result<Self> {
        Ok(Self {
            offset: event.offset,
            node_id: event.node_id,
            template_id: Identifier::from_grpc(
                event
                    .template_id
                    .as_ref()
                    .ok_or_else(|| malformed("an exercised event without a template id"))?,
            ),
            interface_id: event.interface_id.as_ref().map(Identifier::from_grpc),
            contract_id: event.contract_id,
            choice: event.choice,
            consuming: event.consuming,
            acting_parties: event.acting_parties,
            witness_parties: event.witness_parties,
            last_descendant_node_id: event.last_descendant_node_id,
            package_name: event.package_name,
            argument: event.choice_argument.map(|v| WireValue::Grpc(Box::new(v))),
            result: event.exercise_result.map(|v| WireValue::Grpc(Box::new(v))),
        })
    }

    fn from_json(event: &Value) -> Result<Self> {
        let interface_id = match event.get("interfaceId").and_then(Value::as_str) {
            Some(id) if !id.is_empty() => Some(Identifier::from_json(id)?),
            _ => None,
        };
        let present = |key: &str| {
            event
                .get(key)
                .filter(|v| !v.is_null())
                .map(|v| WireValue::Json(Box::new(v.clone())))
        };
        Ok(Self {
            offset: int(event, "offset")?,
            node_id: node_id(event, "nodeId")?,
            contract_id: text(event, "contractId")?,
            template_id: Identifier::from_json(&text(event, "templateId")?)?,
            interface_id,
            choice: text(event, "choice")?,
            consuming: event
                .get("consuming")
                .and_then(Value::as_bool)
                .unwrap_or_default(),
            acting_parties: strings(event, "actingParties"),
            witness_parties: strings(event, "witnessParties"),
            last_descendant_node_id: node_id(event, "lastDescendantNodeId")?,
            package_name: text_or_empty(event, "packageName"),
            argument: present("choiceArgument"),
            result: present("exerciseResult"),
        })
    }
}

/// A contract moving between synchronizers. Its events are reachable in the
/// wire form ([`Reassignment::as_grpc`], [`Reassignment::as_json`]).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Reassignment {
    /// The update id.
    pub update_id: String,
    /// The command id, visible to the submitting parties only.
    pub command_id: String,
    /// The workflow id, if one was set.
    pub workflow_id: String,
    /// The ledger offset.
    pub offset: i64,
    /// The synchronizer's record time.
    pub record_time: Option<rt::Timestamp>,
    wire: Wire<pb::Reassignment>,
}

impl Reassignment {
    /// The reassignment as gRPC delivered it, if it came over gRPC.
    #[must_use]
    pub fn as_grpc(&self) -> Option<&pb::Reassignment> {
        match &self.wire {
            Wire::Grpc(r) => Some(r),
            Wire::Json(_) => None,
        }
    }

    /// The reassignment as the JSON API delivered it, if it came over JSON.
    #[must_use]
    pub fn as_json(&self) -> Option<&Value> {
        match &self.wire {
            Wire::Grpc(_) => None,
            Wire::Json(r) => Some(r),
        }
    }

    fn from_grpc(r: pb::Reassignment) -> Self {
        Self {
            update_id: r.update_id.clone(),
            command_id: r.command_id.clone(),
            workflow_id: r.workflow_id.clone(),
            offset: r.offset,
            record_time: r.record_time.as_ref().map(timestamp),
            wire: Wire::Grpc(Box::new(r)),
        }
    }

    fn from_json(r: &Value) -> Result<Self> {
        Ok(Self {
            update_id: text(r, "updateId")?,
            command_id: text_or_empty(r, "commandId"),
            workflow_id: text_or_empty(r, "workflowId"),
            offset: int(r, "offset")?,
            record_time: json_time(r, "recordTime")?,
            wire: Wire::Json(Box::new(r.clone())),
        })
    }
}

/// A topology change affecting the subscribed parties. Its events are
/// reachable in the wire form.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct TopologyTransaction {
    /// The update id.
    pub update_id: String,
    /// The ledger offset.
    pub offset: i64,
    /// The synchronizer the change was sequenced on.
    pub synchronizer_id: String,
    /// The synchronizer's record time.
    pub record_time: Option<rt::Timestamp>,
    wire: Wire<pb::TopologyTransaction>,
}

impl TopologyTransaction {
    /// The topology transaction as gRPC delivered it, if it came over gRPC.
    #[must_use]
    pub fn as_grpc(&self) -> Option<&pb::TopologyTransaction> {
        match &self.wire {
            Wire::Grpc(t) => Some(t),
            Wire::Json(_) => None,
        }
    }

    /// The topology transaction as the JSON API delivered it, if it came over
    /// JSON.
    #[must_use]
    pub fn as_json(&self) -> Option<&Value> {
        match &self.wire {
            Wire::Grpc(_) => None,
            Wire::Json(t) => Some(t),
        }
    }

    fn from_grpc(t: pb::TopologyTransaction) -> Self {
        Self {
            update_id: t.update_id.clone(),
            offset: t.offset,
            synchronizer_id: t.synchronizer_id.clone(),
            record_time: t.record_time.as_ref().map(timestamp),
            wire: Wire::Grpc(Box::new(t)),
        }
    }

    fn from_json(t: &Value) -> Result<Self> {
        Ok(Self {
            update_id: text(t, "updateId")?,
            offset: int(t, "offset")?,
            synchronizer_id: text_or_empty(t, "synchronizerId"),
            record_time: json_time(t, "recordTime")?,
            wire: Wire::Json(Box::new(t.clone())),
        })
    }
}

/// A position the participant names as safe to resume from.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OffsetCheckpoint {
    /// The offset.
    pub offset: i64,
    /// The record time each synchronizer has reached.
    pub synchronizer_times: Vec<SynchronizerTime>,
}

/// A synchronizer's record time at a checkpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SynchronizerTime {
    /// The synchronizer.
    pub synchronizer_id: String,
    /// Its record time.
    pub record_time: Option<rt::Timestamp>,
}

impl OffsetCheckpoint {
    fn from_grpc(c: &pb::OffsetCheckpoint) -> Self {
        Self {
            offset: c.offset,
            synchronizer_times: c
                .synchronizer_times
                .iter()
                .map(|t| SynchronizerTime {
                    synchronizer_id: t.synchronizer_id.clone(),
                    record_time: t.record_time.as_ref().map(timestamp),
                })
                .collect(),
        }
    }

    fn from_json(c: &Value) -> Result<Self> {
        let synchronizer_times = c
            .get("synchronizerTimes")
            .and_then(Value::as_array)
            .map(|times| {
                times
                    .iter()
                    .map(|t| {
                        Ok(SynchronizerTime {
                            synchronizer_id: text_or_empty(t, "synchronizerId"),
                            record_time: json_time(t, "recordTime")?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();
        Ok(Self {
            offset: int(c, "offset")?,
            synchronizer_times,
        })
    }
}

fn malformed(message: impl Into<String>) -> Error {
    Error::UnexpectedResponse(message.into())
}

fn timestamp(t: &prost_types::Timestamp) -> rt::Timestamp {
    rt::Timestamp(t.seconds * 1_000_000 + i64::from(t.nanos) / 1_000)
}

fn text(value: &Value, key: &str) -> Result<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| malformed(format!("a JSON update without `{key}`")))
}

fn text_or_empty(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn int(value: &Value, key: &str) -> Result<i64> {
    value
        .get(key)
        .and_then(Value::as_i64)
        .ok_or_else(|| malformed(format!("a JSON update without a numeric `{key}`")))
}

fn node_id(value: &Value, key: &str) -> Result<i32> {
    i32::try_from(int(value, key)?).map_err(|_| malformed(format!("`{key}` out of range")))
}

fn strings(value: &Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn identifiers(value: &Value, key: &str) -> Result<Vec<Identifier>> {
    value.get(key).and_then(Value::as_array).map_or_else(
        || Ok(Vec::new()),
        |items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(Identifier::from_json)
                .collect()
        },
    )
}

fn json_time(value: &Value, key: &str) -> Result<Option<rt::Timestamp>> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(time) => serde_json::from_value(time.clone())
            .map(Some)
            .map_err(|e| malformed(format!("`{key}` is not a timestamp: {e}"))),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A minimal template, enough to drive `decode` on both transports.
    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct Steward {
        steward: String,
        charter: String,
    }
    impl rt::Contract for Steward {
        const PACKAGE_ID: &'static str = "a50e65b2";
        const PACKAGE_NAME: &'static str = "contractkeys-h61a17c769e62";
        const MODULE_NAME: &'static str = "ContractKeys";
        const ENTITY_NAME: &'static str = "Steward";
    }
    impl rt::ToValue for Steward {
        fn to_value(&self) -> pb::Value {
            rt::Unit.to_value()
        }
    }
    impl rt::FromValue for Steward {
        fn from_value(value: &pb::Value) -> std::result::Result<Self, rt::ValueError> {
            let Some(pb::value::Sum::Record(record)) = &value.sum else {
                return Err(rt::ValueError::new("not a record"));
            };
            let text = |i: usize| match record.fields.get(i).and_then(|f| f.value.as_ref()) {
                Some(pb::Value {
                    sum: Some(pb::value::Sum::Party(p) | pb::value::Sum::Text(p)),
                }) => Ok(p.clone()),
                _ => Err(rt::ValueError::new("field")),
            };
            Ok(Self {
                steward: text(0)?,
                charter: text(1)?,
            })
        }
    }
    impl rt::Template for Steward {
        fn to_record(&self) -> pb::Record {
            pb::Record::default()
        }
    }

    /// The choice argument, decoded through the JSON codec in these tests.
    #[derive(Debug, serde::Deserialize)]
    struct Revise {
        revised: String,
    }
    impl rt::FromValue for Revise {
        fn from_value(_: &pb::Value) -> std::result::Result<Self, rt::ValueError> {
            Err(rt::ValueError::new("unused on the JSON path"))
        }
    }

    /// The shape the LocalNet's JSON API returned for a keyed exercise under
    /// the ledger-effects shape (offsets and ids shortened).
    fn json_transaction() -> Value {
        json!({"update": {"Transaction": {"value": {
            "updateId": "1220d4c7",
            "commandId": "cmd-1",
            "workflowId": "",
            "effectiveAt": "2026-10-23T14:52:28.949667Z",
            "offset": 16376,
            "synchronizerId": "global-domain::1220",
            "recordTime": "2026-10-23T14:52:29.100000Z",
            "events": [
                {"ExercisedEvent": {
                    "offset": 16376, "nodeId": 0, "contractId": "009d1f",
                    "templateId": "a50e65b2:ContractKeys:Steward",
                    "interfaceId": null, "choice": "Revise",
                    "choiceArgument": {"revised": "v2"},
                    "actingParties": ["alice::1220"], "consuming": true,
                    "witnessParties": ["alice::1220"], "lastDescendantNodeId": 1,
                    "exerciseResult": "0068d4", "packageName": "contractkeys-h61a17c769e62",
                    "implementedInterfaces": [], "acsDelta": true
                }},
                {"CreatedEvent": {
                    "offset": 16376, "nodeId": 1, "contractId": "0068d4",
                    "templateId": "a50e65b2:ContractKeys:Steward",
                    "contractKey": "alice::1220", "contractKeyHash": "",
                    "createArgument": {"steward": "alice::1220", "charter": "v2"},
                    "createdEventBlob": "", "interfaceViews": [],
                    "witnessParties": ["alice::1220"], "signatories": ["alice::1220"],
                    "observers": [], "createdAt": "2026-10-23T14:52:28.949667Z",
                    "packageName": "contractkeys-h61a17c769e62",
                    "representativePackageId": "a50e65b2", "acsDelta": true
                }},
                {"ArchivedEvent": {
                    "offset": 16376, "nodeId": 2, "contractId": "009d1f",
                    "templateId": "a50e65b2:ContractKeys:Steward",
                    "witnessParties": ["alice::1220"],
                    "packageName": "contractkeys-h61a17c769e62",
                    "implementedInterfaces": []
                }}
            ]
        }}}})
    }

    #[test]
    fn a_json_transaction_reads_into_the_sdk_types() {
        let update = Update::from_json(&json_transaction()).unwrap();
        assert_eq!(update.offset(), 16376);
        assert_eq!(update.update_id(), Some("1220d4c7"));
        let tx = update.as_transaction().unwrap();
        assert_eq!(tx.synchronizer_id, "global-domain::1220");
        assert!(tx.effective_at.is_some() && tx.record_time.is_some());

        let exercised = tx.exercised().next().unwrap();
        assert_eq!(exercised.choice, "Revise");
        assert!(exercised.consuming);
        assert_eq!(
            exercised.template_id.to_string(),
            "a50e65b2:ContractKeys:Steward"
        );
        let argument: Revise = exercised.decode_argument().unwrap();
        assert_eq!(argument.revised, "v2");

        let created = tx.created().next().unwrap();
        assert_eq!(created.contract_id, "0068d4");
        assert_eq!(created.package_name, "contractkeys-h61a17c769e62");
        assert_eq!(
            created.decode::<Steward>().unwrap(),
            Steward {
                steward: "alice::1220".to_string(),
                charter: "v2".to_string()
            }
        );
        assert!(created.as_json().is_some() && created.as_grpc().is_none());
        assert_eq!(tx.archived().count(), 1);
    }

    #[test]
    fn the_same_transaction_over_grpc_reads_into_the_same_types() {
        let party = |p: &str| pb::Value {
            sum: Some(pb::value::Sum::Party(p.to_string())),
        };
        let text = |t: &str| pb::Value {
            sum: Some(pb::value::Sum::Text(t.to_string())),
        };
        let id = pb::Identifier {
            package_id: "a50e65b2".to_string(),
            module_name: "ContractKeys".to_string(),
            entity_name: "Steward".to_string(),
        };
        let created = pb::CreatedEvent {
            offset: 16376,
            node_id: 1,
            contract_id: "0068d4".to_string(),
            template_id: Some(id.clone()),
            create_arguments: Some(pb::Record {
                record_id: None,
                fields: vec![
                    pb::RecordField {
                        label: "steward".to_string(),
                        value: Some(party("alice::1220")),
                    },
                    pb::RecordField {
                        label: "charter".to_string(),
                        value: Some(text("v2")),
                    },
                ],
            }),
            witness_parties: vec!["alice::1220".to_string()],
            package_name: "contractkeys-h61a17c769e62".to_string(),
            created_at: Some(prost_types::Timestamp {
                seconds: 1_792_853_548,
                nanos: 949_667_000,
            }),
            ..Default::default()
        };
        let tx = pb::Transaction {
            update_id: "1220d4c7".to_string(),
            offset: 16376,
            events: vec![pb::Event {
                event: Some(pb::event::Event::Created(created)),
            }],
            ..Default::default()
        };
        let update = Update::from_grpc(pb::get_updates_response::Update::Transaction(tx)).unwrap();
        let created = update.as_transaction().unwrap().created().next().unwrap();
        assert_eq!(
            created.template_id.to_string(),
            "a50e65b2:ContractKeys:Steward"
        );
        assert_eq!(
            created.created_at,
            Some(rt::Timestamp(1_792_853_548_949_667))
        );
        assert_eq!(created.decode::<Steward>().unwrap().charter, "v2");
        assert!(created.as_grpc().is_some());
    }

    #[test]
    fn a_checkpoint_and_the_bare_ws_shape_read_too() {
        let checkpoint = json!({"update": {"OffsetCheckpoint": {"value": {
            "offset": 99,
            "synchronizerTimes": [{"synchronizerId": "s::1", "recordTime": "2026-10-23T14:52:29Z"}]
        }}}});
        let Update::OffsetCheckpoint(c) = Update::from_json(&checkpoint).unwrap() else {
            panic!("a checkpoint");
        };
        assert_eq!(c.offset, 99);
        assert_eq!(c.synchronizer_times[0].synchronizer_id, "s::1");

        // A WebSocket frame carries the update without the outer `update` key.
        let bare = json_transaction()["update"].clone();
        assert_eq!(Update::from_json(&bare).unwrap().offset(), 16376);
    }

    #[test]
    fn a_malformed_element_is_an_error_not_a_default() {
        assert!(Update::from_json(&json!({"update": {"Mystery": {"value": {}}}})).is_err());
        assert!(Update::from_json(&json!({})).is_err());
        let mut no_id = json_transaction();
        no_id["update"]["Transaction"]["value"]["events"][1]["CreatedEvent"]["templateId"] =
            json!("not-an-id");
        assert!(Update::from_json(&no_id).is_err());
    }
}
