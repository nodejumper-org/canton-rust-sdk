//! Traits the generated template and interface types implement.
//!
//! A [`Contract`] is anything with an on-ledger identity that choices can be
//! exercised on — a template *or* an interface. [`Template`] narrows that to a
//! concrete template (it also has a payload codec), [`Interface`] to an
//! interface (it has a view type), and [`WithKey`] to a keyed template.

use canton_proto::com::daml::ledger::api::v2::Identifier;

use crate::value::{FromValue, ToValue, ValueError};

/// Something with an on-ledger identity that choices can be exercised on: a
/// template or an interface. Its template id is `PACKAGE:MODULE:ENTITY`.
pub trait Contract {
    /// The id (hash) of the package that defines it. Pins the exact version;
    /// prefer [`Contract::template_id`], which uses the package **name**.
    const PACKAGE_ID: &'static str;
    /// The Daml package **name** (e.g. `splice-amulet`).
    const PACKAGE_NAME: &'static str;
    /// The Daml module it is defined in, dotted (e.g. `Splice.Amulet`).
    const MODULE_NAME: &'static str;
    /// The template/interface entity name (e.g. `AmuletRules`, `Holding`).
    const ENTITY_NAME: &'static str;

    /// The on-ledger id. Uses the upgrade-friendly package-**name** form
    /// (`#<package-name>`), so the participant resolves the version vetted under
    /// Smart Contract Upgrade rather than pinning one package id. Falls back to
    /// the package id when no name is known.
    #[must_use]
    fn template_id() -> Identifier {
        let package = if Self::PACKAGE_NAME.is_empty() {
            Self::PACKAGE_ID.to_string()
        } else {
            format!("#{}", Self::PACKAGE_NAME)
        };
        Identifier {
            package_id: package,
            module_name: Self::MODULE_NAME.to_string(),
            entity_name: Self::ENTITY_NAME.to_string(),
        }
    }
}

/// A Daml template: a [`Contract`] with a payload codec. Generated code
/// implements this on each template's payload struct.
pub trait Template: Contract + ToValue + FromValue {
    /// The payload as a Ledger API `Record` — the shape a create command
    /// carries. A template payload is always a record, so this is total;
    /// encoding that in the trait is what keeps `create_command` panic-free.
    fn to_record(&self) -> canton_proto::com::daml::ledger::api::v2::Record;

    /// Decode a `CreatedEvent` (from a transaction stream, an ACS snapshot, or
    /// a `submit-and-wait` response) into this template's typed payload — the
    /// typed **read** path:
    ///
    /// ```ignore
    /// let install: AppInstall = AppInstall::from_created_event(&event)?;
    /// ```
    ///
    /// When the event carries a template id, its module/entity are checked
    /// first, so decoding an event of the wrong template fails with a clear
    /// error instead of a field-shape mismatch. The package id is deliberately
    /// **not** compared: under Smart Contract Upgrade the ledger may report any
    /// vetted version of the package.
    ///
    /// # Errors
    /// Returns [`ValueError`] if the event is a different template, carries no
    /// payload, or its payload does not decode as `Self`.
    fn from_created_event(
        event: &canton_proto::com::daml::ledger::api::v2::CreatedEvent,
    ) -> Result<Self, ValueError> {
        if let Some(id) = &event.template_id
            && (id.entity_name != Self::ENTITY_NAME || id.module_name != Self::MODULE_NAME)
        {
            return Err(ValueError::new(format!(
                "event is {}:{}, expected {}:{}",
                id.module_name,
                id.entity_name,
                Self::MODULE_NAME,
                Self::ENTITY_NAME,
            )));
        }
        let record = event
            .create_arguments
            .as_ref()
            .ok_or_else(|| ValueError::new("created event carries no create_arguments payload"))?;
        crate::value::from_record(record)
    }

    /// [`Template::from_created_event`] for the **JSON** Ledger API: decode a
    /// created event from a JSON transaction, ACS entry, or `submit-and-wait`
    /// response into this template's typed payload.
    ///
    /// ```ignore
    /// for event in &response.transaction.events {
    ///     if let Ok(install) = AppInstall::from_json_created_event(event) {
    ///         println!("{}", install.provider);
    ///     }
    /// }
    /// ```
    ///
    /// Both transports carry the same contract for the same bindings, and until
    /// this existed only one of them could read one back as a type. On the JSON
    /// lane a caller had to reach into `event["CreatedEvent"]["createArgument"]`
    /// themselves — and, having done so, had nothing checking that the event was
    /// this template at all. A party's stream carries events for every template
    /// it sees; where two payloads happen to share a field shape, decoding one
    /// as the other succeeds and is wrong.
    ///
    /// Accepts the created event either bare or wrapped, because the API hands
    /// it out several ways: a transaction's `events` are
    /// `{"CreatedEvent": {…}}`, an ACS entry nests `createdEvent`, and the
    /// `events/events-by-contract-id` response nests it under
    /// `{"created": {"createdEvent": {…}}}`. There is no ambiguity — a created
    /// event does not itself have those keys.
    ///
    /// The module and entity of `templateId` are checked, and the package id is
    /// deliberately not, for the reason given on
    /// [`Template::from_created_event`].
    ///
    /// # Errors
    /// Returns [`ValueError`] if the value is not a created event, is a
    /// different template, carries no `createArgument`, or its payload does not
    /// deserialize as `Self`.
    fn from_json_created_event(event: &serde_json::Value) -> Result<Self, ValueError>
    where
        Self: serde::de::DeserializeOwned,
    {
        // Unwrap one or two layers: `events-by-contract-id` wraps the created
        // event as `{"created": {"createdEvent": …}}`, so peel `created` first
        // and then the created-event key inside it.
        let inner = event.get("created").unwrap_or(event);
        let created = ["CreatedEvent", "createdEvent"]
            .iter()
            .find_map(|key| inner.get(key))
            .unwrap_or(inner);

        if let Some(id) = created
            .get("templateId")
            .and_then(serde_json::Value::as_str)
        {
            // `<package>:<Module>:<Entity>` — the package part is whatever
            // version the ledger vetted, so only the last two are compared.
            let mut parts = id.rsplit(':');
            let entity = parts.next().unwrap_or_default();
            let module = parts.next().unwrap_or_default();
            if entity != Self::ENTITY_NAME || module != Self::MODULE_NAME {
                return Err(ValueError::new(format!(
                    "event is {module}:{entity}, expected {}:{}",
                    Self::MODULE_NAME,
                    Self::ENTITY_NAME,
                )));
            }
        }

        let payload = created.get("createArgument").ok_or_else(|| {
            // Singular: the gRPC field is `create_arguments` and this one is
            // not, which is the sort of thing only a real response settles.
            ValueError::new("created event carries no createArgument payload")
        })?;
        serde_json::from_value(payload.clone())
            // The serde error names the field it failed on but not the value,
            // which is the same bargain the gRPC codec strikes.
            .map_err(|error| ValueError::new(format!("payload did not decode: {error}")))
    }
}

/// A Daml interface: a [`Contract`] identified by a marker type (held via
/// `ContractId`), with a view type. Choices are exercised through the interface
/// id, so a `ContractId<Interface>` can be exercised without knowing the
/// concrete template.
pub trait Interface: Contract {
    /// The interface's view type — the record returned by an interface view.
    type View: FromValue;
}

/// A template that declares a **contract key**. Generated code implements this
/// on the payload struct of any keyed template, exposing the key's type so a
/// contract can be looked up and choices exercised *by key* (see
/// [`crate::exercise_by_key_command`]).
pub trait WithKey: Template {
    /// The contract key's type (a serializable Daml type).
    type Key: ToValue;
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod json_envelope_tests {
    use super::*;
    use canton_proto::com::daml::ledger::api::v2 as pb;
    use serde::Deserialize;

    // A minimal Template whose JSON payload is one field, enough to drive
    // `from_json_created_event` through each envelope shape the API uses.
    #[derive(Debug, PartialEq, Deserialize)]
    struct Widget {
        owner: String,
    }
    impl Contract for Widget {
        const PACKAGE_ID: &'static str = "pkg";
        const PACKAGE_NAME: &'static str = "app";
        const MODULE_NAME: &'static str = "M";
        const ENTITY_NAME: &'static str = "Widget";
    }
    impl ToValue for Widget {
        fn to_value(&self) -> pb::Value {
            crate::Unit.to_value()
        }
    }
    impl FromValue for Widget {
        fn from_value(_: &pb::Value) -> Result<Self, ValueError> {
            Err(ValueError::new("unused"))
        }
    }
    impl Template for Widget {
        fn to_record(&self) -> pb::Record {
            pb::Record::default()
        }
    }

    fn created() -> serde_json::Value {
        serde_json::json!({
            "templateId": "whatever-package:M:Widget",
            "createArgument": { "owner": "alice" }
        })
    }

    #[test]
    fn every_envelope_shape_the_json_api_uses_decodes() {
        let want = Widget {
            owner: "alice".to_string(),
        };

        // Bare, transaction-event `{"CreatedEvent": …}`, ACS-entry
        // `{"createdEvent": …}`, and the `events-by-contract-id` double wrap
        // `{"created": {"createdEvent": …}}` — the last one used to error with
        // "carries no createArgument" because only the inner key was peeled.
        let shapes = [
            created(),
            serde_json::json!({ "CreatedEvent": created() }),
            serde_json::json!({ "createdEvent": created() }),
            serde_json::json!({ "created": { "createdEvent": created() } }),
        ];
        for (i, shape) in shapes.iter().enumerate() {
            assert_eq!(
                Widget::from_json_created_event(shape).expect("shape decodes"),
                want,
                "envelope shape {i} did not decode"
            );
        }
    }
}
