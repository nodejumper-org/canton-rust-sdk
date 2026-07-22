//! Traits the generated template and interface types implement.
//!
//! A [`Contract`] is anything with an on-ledger identity that choices can be
//! exercised on — a template *or* an interface. [`Template`] narrows that to a
//! concrete template (it also has a payload codec), [`Interface`] to an
//! interface (it has a view type), and [`WithKey`] to a keyed template.

use canton_proto::com::daml::ledger::api::v2::Identifier;

use crate::value::{FromValue, ToValue};

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
pub trait Template: Contract + ToValue + FromValue {}

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
