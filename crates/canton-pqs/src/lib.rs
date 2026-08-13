//! A typed read client for the Daml **Participant Query Store** (PQS, served by
//! Scribe).
//!
//! PQS streams a participant's contracts into Postgres and exposes them through
//! a small set of functions — `active`, `creates`, `archives`, `exercises`,
//! `lookup_contract`. This crate reads them typed: a query names its template
//! by *type*, and the row comes back as the generated payload.
//!
//! # No hand-written SQL, and no interpolation
//!
//! Predicates compile to parameterized statements. Every caller-supplied value
//! is a parameter, and so is every JSON field path — sent as `text[]` and
//! applied with `#>`. The statement text depends on the *shape* of a query and
//! never on its data, so a field name taken from user input is data Postgres
//! never parses.
//!
//! # The payload is the same type the Ledger API gives you
//!
//! PQS stores payloads in the Daml JSON encoding, which is what `canton-daml`
//! implements — so a contract read here deserializes into the same generated
//! type as one read from a transaction stream. That is the point of the crate:
//! two ways in, one set of types.

#![forbid(unsafe_code)]

mod client;
mod query;
mod row;

pub use client::{PqsClient, active_signed_by};
pub use query::{IntoPath, Op, Param, Predicate, Query, Source, Sql};
pub use row::{Contract, Exercise};
