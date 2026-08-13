//! What PQS returns, typed.
//!
//! The payload is stored in the Daml JSON encoding — which is what
//! `canton-daml` implements — so it deserializes into the same generated type
//! a transaction stream yields. The columns around it are the ledger metadata
//! PQS records: who signed, when it was created, at which offset, and when it
//! was archived if it has been.

use canton_core::{Error, Result};
use canton_daml::ContractId;

/// A contract as PQS holds it: the typed payload, plus the ledger facts about
/// it.
// `contract_id` reads as a repetition of the type name, but it is what the
// ledger calls this and what the PQS column is called. Shortening it to `id`
// would make the accessor say less than the domain does.
#[allow(clippy::struct_field_names)]
#[derive(Clone, Debug)]
pub struct Contract<T> {
    contract_id: ContractId<T>,
    payload: T,
    created_at_offset: i64,
    archived_at_offset: Option<i64>,
    created_effective_at: Option<time::OffsetDateTime>,
    archived_effective_at: Option<time::OffsetDateTime>,
    signatories: Vec<String>,
    observers: Vec<String>,
    witnesses: Vec<String>,
    package_name: String,
    package_version: String,
    package_id: String,
}

impl<T> Contract<T> {
    /// The contract id, typed.
    #[must_use]
    pub fn contract_id(&self) -> &ContractId<T> {
        &self.contract_id
    }

    /// The payload.
    #[must_use]
    pub fn payload(&self) -> &T {
        &self.payload
    }

    /// Take the payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }

    /// The offset the contract was created at.
    #[must_use]
    pub fn created_at_offset(&self) -> i64 {
        self.created_at_offset
    }

    /// The offset it was archived at, if it has been archived at all.
    ///
    /// **Being set does not mean the contract was inactive for the read that
    /// returned it.** `active_at(o)` selects contracts whose lifetime *covers*
    /// `o`, so a contract that was live then and archived afterwards comes back
    /// with this set — filtering those out would drop exactly the rows the
    /// query asked for. Compare it against the offset that was read at:
    /// [`was_active_at`](Self::was_active_at) does that.
    #[must_use]
    pub fn archived_at_offset(&self) -> Option<i64> {
        self.archived_at_offset
    }

    /// Whether the contract was active at `offset`.
    ///
    /// There is no offset-free answer to "is this active": it depends on when
    /// you are asking about. A contract read by `active_at(o)` was active at
    /// `o` whatever this says about later offsets.
    #[must_use]
    pub fn was_active_at(&self, offset: i64) -> bool {
        self.created_at_offset <= offset
            && self
                .archived_at_offset
                .is_none_or(|archived| archived > offset)
    }

    /// Ledger effective time of the creation.
    #[must_use]
    pub fn created_effective_at(&self) -> Option<time::OffsetDateTime> {
        self.created_effective_at
    }

    /// Ledger effective time of the archival.
    #[must_use]
    pub fn archived_effective_at(&self) -> Option<time::OffsetDateTime> {
        self.archived_effective_at
    }

    /// The parties that signed it.
    #[must_use]
    pub fn signatories(&self) -> &[String] {
        &self.signatories
    }

    /// The parties that observe it.
    #[must_use]
    pub fn observers(&self) -> &[String] {
        &self.observers
    }

    /// Every party that witnessed it — the column `Predicate::witness` filters
    /// on, so selecting by it and then not being able to see it made no sense.
    #[must_use]
    pub fn witnesses(&self) -> &[String] {
        &self.witnesses
    }

    /// The Daml package name — stable across an upgrade.
    #[must_use]
    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    /// The package version.
    #[must_use]
    pub fn package_version(&self) -> &str {
        &self.package_version
    }

    /// The package id — the exact build, which a version does not pin.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }
}

/// An exercised choice as PQS holds it.
#[derive(Clone, Debug)]
pub struct Exercise<C> {
    contract_id: String,
    choice: String,
    consuming: bool,
    argument: C,
    result: serde_json::Value,
    exercised_at_offset: i64,
    exercised_effective_at: Option<time::OffsetDateTime>,
    controllers: Vec<String>,
}

impl<C> Exercise<C> {
    /// The contract the choice was exercised on.
    #[must_use]
    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }

    /// The choice name.
    #[must_use]
    pub fn choice(&self) -> &str {
        &self.choice
    }

    /// Whether exercising it archived the contract.
    #[must_use]
    pub fn consuming(&self) -> bool {
        self.consuming
    }

    /// The typed argument.
    #[must_use]
    pub fn argument(&self) -> &C {
        &self.argument
    }

    /// The result, untyped.
    ///
    /// A choice's return type is not the argument's, and PQS does not say which
    /// it is — so this stays JSON rather than pretending to a type it cannot
    /// know. Decode it with [`result_as`](Self::result_as).
    #[must_use]
    pub fn result(&self) -> &serde_json::Value {
        &self.result
    }

    /// Decode the result into a type the caller knows it to be.
    ///
    /// # Errors
    /// [`Error::Payload`] if it does not match.
    pub fn result_as<R: serde::de::DeserializeOwned>(&self) -> Result<R> {
        serde_json::from_value(self.result.clone()).map_err(|e| Error::Payload(Box::new(e)))
    }

    /// The offset it was exercised at.
    #[must_use]
    pub fn exercised_at_offset(&self) -> i64 {
        self.exercised_at_offset
    }

    /// Ledger effective time of the exercise.
    #[must_use]
    pub fn exercised_effective_at(&self) -> Option<time::OffsetDateTime> {
        self.exercised_effective_at
    }

    /// The parties that exercised it.
    #[must_use]
    pub fn controllers(&self) -> &[String] {
        &self.controllers
    }
}

/// Read a column by name, saying which one was missing rather than panicking.
///
/// PQS's row shape is a contract of its own; a rename upstream should say so
/// by name, not as an index that no longer exists.
fn column<'a, T: tokio_postgres::types::FromSql<'a>>(
    row: &'a tokio_postgres::Row,
    name: &str,
) -> Result<T> {
    row.try_get(name).map_err(|e| {
        Error::UnexpectedResponse(format!("PQS returned no usable `{name}` column: {e}"))
    })
}

impl<T: serde::de::DeserializeOwned> Contract<T> {
    /// Decode one row.
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if a column is missing or of another type,
    /// [`Error::Payload`] if the payload does not match `T`.
    pub fn from_row(row: &tokio_postgres::Row) -> Result<Self> {
        let payload: serde_json::Value = column(row, "payload")?;
        let contract_id: String = column(row, "contract_id")?;
        Ok(Self {
            payload: serde_json::from_value(payload).map_err(|e| {
                Error::Payload(Box::new(std::io::Error::other(format!(
                    "contract {contract_id}: {e}"
                ))))
            })?,
            contract_id: ContractId::new(contract_id),
            created_at_offset: column(row, "created_at_offset")?,
            archived_at_offset: column(row, "archived_at_offset")?,
            created_effective_at: column(row, "created_effective_at")?,
            archived_effective_at: column(row, "archived_effective_at")?,
            signatories: column(row, "signatories")?,
            observers: column(row, "observers")?,
            witnesses: column(row, "witnesses")?,
            package_name: column(row, "package_name")?,
            package_version: column(row, "package_version")?,
            package_id: column(row, "package_id")?,
        })
    }
}

impl<C: serde::de::DeserializeOwned> Exercise<C> {
    /// Decode one row.
    ///
    /// # Errors
    /// As [`Contract::from_row`].
    pub fn from_row(row: &tokio_postgres::Row) -> Result<Self> {
        let argument: serde_json::Value = column(row, "argument")?;
        let choice: String = column(row, "choice")?;
        Ok(Self {
            argument: serde_json::from_value(argument).map_err(|e| {
                Error::Payload(Box::new(std::io::Error::other(format!(
                    "choice {choice}: {e}"
                ))))
            })?,
            choice,
            contract_id: column(row, "contract_id")?,
            consuming: column(row, "consuming")?,
            result: column::<Option<serde_json::Value>>(row, "result")?
                .unwrap_or(serde_json::Value::Null),
            exercised_at_offset: column(row, "exercised_at_offset")?,
            exercised_effective_at: column(row, "exercised_effective_at")?,
            controllers: column(row, "controllers")?,
        })
    }
}
