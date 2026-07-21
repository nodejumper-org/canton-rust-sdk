//! Daml-LF archive reader and decoder.
//!
//! **Milestone 2, work in progress.** This crate is the codegen's front-end: it
//! reads a `.dar` and (eventually) decodes its Daml-LF packages into the
//! decoder-agnostic IR that `canton-codegen` turns into Rust.
//!
//! Status:
//! - [`Dar`] — the DAR **container** layer (zip + `MANIFEST.MF` → the raw
//!   Daml-LF bytes of the main package and its dependencies). Done.
//! - Daml-LF protobuf decode (LF 2.x → typed AST) and lowering to
//!   `canton_codegen::ir` — next. Per the milestone plan this is a native Rust
//!   decoder (no JVM), targeting the LF version Canton 3.x ships, blueprinted on
//!   the design of fujiapple's `daml-lf` but re-vendoring the LF 2.x protobufs.

mod dar;
mod decode;

pub use dar::{Dar, DarError};
pub use decode::{
    DecodeError, decode_main_package, decode_package, interned_str, package_name, package_version,
};

/// The generated Daml-LF archive types, from the vendored `.proto` files:
/// `pb::daml_lf_dev` (the `Archive`/`ArchivePayload` wrapper) and
/// `pb::daml_lf_2` (the LF 2 AST — `Package`, `Module`, `DefDataType`, …).
pub mod pb {
    #![allow(
        clippy::all,
        clippy::pedantic,
        missing_docs,
        unreachable_pub,
        clippy::doc_markdown
    )]
    include!(concat!(env!("OUT_DIR"), "/_daml_lf.rs"));
}
