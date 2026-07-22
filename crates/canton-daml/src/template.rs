//! The `Template` trait that generated template payload types implement.

use canton_proto::com::daml::ledger::api::v2::Identifier;

use crate::value::{FromValue, ToValue};

/// A Daml template: its on-ledger identity (the template id) and payload codec.
///
/// Generated code implements this on each template's payload struct, so a
/// contract of the template can be identified and its payload moved to and from
/// the Ledger API `Value` (via the [`ToValue`]/[`FromValue`] supertraits) when
/// building create and exercise commands.
pub trait Template: ToValue + FromValue {
    /// The id (hash) of the package that defines the template. Pins the exact
    /// version; prefer [`Template::template_id`], which uses the package **name**.
    const PACKAGE_ID: &'static str;
    /// The Daml package **name** (e.g. `splice-amulet`).
    const PACKAGE_NAME: &'static str;
    /// The Daml module the template is defined in, dotted (e.g. `Splice.Amulet`).
    const MODULE_NAME: &'static str;
    /// The template's entity name (e.g. `AmuletRules`).
    const ENTITY_NAME: &'static str;

    /// The on-ledger template id. Uses the upgrade-friendly package-**name** form
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

/// A template that declares a **contract key**. Generated code implements this
/// on the payload struct of any keyed template, exposing the key's type so a
/// contract can be looked up and choices exercised *by key* (see
/// [`crate::exercise_by_key_command`]).
pub trait WithKey: Template {
    /// The contract key's type (a serializable Daml type).
    type Key: ToValue;
}
