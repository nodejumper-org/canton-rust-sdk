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

pub mod allocation;
mod context;
mod registry;
pub mod transfer;
pub mod v2;

pub use context::{ChoiceContext, ChoiceContextRequest, WireChoiceContext, WireDisclosedContract};
pub use registry::{
    AllocationChoice, AllocationInstructionChoice, FactoryWithContext, Instrument, RegistryClient,
    RegistryInfo, TransferInstructionChoice, TransferKind,
};
/// The V1 (CIP-56) workflow, re-exported at the root because it is the one most
/// networks run today. The V2 (CIP-0112) workflow keeps the same names and so
/// stays behind [`v2`] — `token::transfer` and `token::v2::transfer` are
/// different standards, and the path is what says which you meant.
pub use transfer::{TokenCommand, accept, reject, transfer, withdraw};

pub mod types {
    //! The generated token-standard types this crate's API is written in terms
    //! of, re-exported so they can be named without depending on each
    //! generated crate directly.
    //!
    //! This is not a convenience. `transfer` takes a
    //! [`Transfer`](v1::transfer_instruction::Transfer) and `v2::transfer`
    //! takes a [`Transfer`](v2::transfer_instruction::Transfer) — so without
    //! these a `cargo add canton` user could reach the *functions* through the
    //! facade and be unable to name their arguments. The whole token
    //! deliverable was unreachable that way, which is the same defect the
    //! crate's own conformance suite states the rule against: a capability
    //! that works only via a member crate is not one `cargo add canton`
    //! delivers.
    //!
    //! Whole modules rather than a hand-listed set of types, because a list
    //! goes stale the first time the standard adds a record and nothing
    //! notices.

    /// Metadata — `ExtraArgs`, `ChoiceContext`, `Metadata`. Shared by both
    /// standards: there is no `metadata-v2`.
    pub use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as metadata;

    /// CIP-56 — the V1 token standard.
    pub mod v1 {
        pub use canton_splice_api_token_allocation_instruction_v1::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1 as allocation_instruction;
        pub use canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1 as allocation;
        pub use canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1 as holding;
        pub use canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1 as transfer_instruction;
    }

    /// CIP-0112 — the V2 token standard.
    pub mod v2 {
        pub use canton_splice_api_token_allocation_instruction_v2::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2 as allocation_instruction;
        pub use canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2 as allocation;
        pub use canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2 as holding;
        pub use canton_splice_api_token_transfer_events_v2::splice_api_token_transfer_events_v2::Splice_Api_Token_TransferEventsV2 as transfer_events;
        pub use canton_splice_api_token_transfer_instruction_v2::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2 as transfer_instruction;
    }
}

/// The V1 `Holding` interface, at the root because it is the type a caller
/// reaches for first. Everything else is under [`types`].
pub use canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding;
