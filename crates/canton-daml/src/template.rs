//! The `Template` trait that generated template payload types implement.

use crate::value::{FromValue, ToValue};

/// A Daml template: its on-ledger identity (the template id) and payload codec.
///
/// Generated code implements this on each template's payload struct, so a
/// contract of the template can be identified — its template id is
/// `PACKAGE_ID:MODULE_NAME:ENTITY_NAME` — and its payload moved to and from the
/// Ledger API `Value` (via the [`ToValue`]/[`FromValue`] supertraits) when
/// building create and exercise commands.
pub trait Template: ToValue + FromValue {
    /// The id (hash) of the package that defines the template.
    const PACKAGE_ID: &'static str;
    /// The Daml module the template is defined in, dotted (e.g. `Splice.Amulet`).
    const MODULE_NAME: &'static str;
    /// The template's entity name (e.g. `AmuletRules`).
    const ENTITY_NAME: &'static str;
}
