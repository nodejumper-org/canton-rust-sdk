//! Canton Network token-standard workflows (CIP-56).
//!
//! The token-standard **types** are generated from the Splice DARs and live in
//! the `canton-splice-api-token-*` crates. This crate is the *workflow* over
//! them: resolving a factory, asking a registry for a choice context,
//! attaching the contracts it says to disclose, and exercising the choice.
//!
//! Nothing here re-declares a Daml type. That would duplicate the code
//! generator and go stale on the next Splice release; the `Holding` this crate
//! names is the same Rust type an application reads out of the ledger.
//!
//! # Why a registry is involved at all
//!
//! A transfer is exercised through an interface, and only the registry knows
//! what reference data that choice needs — its own configuration contracts,
//! the rules for an instrument, the current round. So the client asks for a
//! choice context and gets back both the data to pass in and the contracts the
//! participant must be shown. Those contracts are usually invisible to the
//! submitting party, which is exactly what explicit disclosure is for.
//!
//! The registry's API is off-ledger HTTP, specified by the token standard's
//! OpenAPI documents.

#![forbid(unsafe_code)]

mod context;
mod registry;

pub use context::{ChoiceContext, ChoiceContextRequest, WireChoiceContext, WireDisclosedContract};
pub use registry::{
    AllocationChoice, FactoryWithContext, Instrument, RegistryClient, RegistryInfo,
    TransferInstructionChoice, TransferKind,
};
