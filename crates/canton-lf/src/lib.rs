//! Daml-LF archive reader and decoder.
//!
//! **Milestone 2.** This crate is the codegen's front-end: it reads a `.dar`
//! and decodes its Daml-LF packages into the AST that `canton-codegen` lowers
//! to IR and turns into Rust.
//!
//! - [`Dar`] — the DAR **container** layer (zip + `MANIFEST.MF` → the raw
//!   Daml-LF bytes of the main package and its dependencies).
//! - [`decode_all`] / [`decode_main_package`] — Daml-LF protobuf decode
//!   (LF 2.x → typed AST) with the interned-table resolvers.
//!
//! The decode is built **on the official `daml-lf-archive` schema**: the
//! vendored `daml_lf.proto` / `daml_lf2.proto` under `proto/` are Digital
//! Asset's own archive definitions (pinned to a Daml 3.3 snapshot), so the
//! bytes→AST step is machine-generated from the authoritative source rather
//! than hand-written. The interpretation on top (version dispatch, interning,
//! reference resolution) is held to the official **JVM `daml-lf-archive`
//! reader** by a conformance oracle: `tests/oracle.rs` renders the decoded
//! type-signature surface and asserts byte-for-byte agreement with
//! `tools/lf-oracle/LfOracle.scala`, which reads the same DAR through
//! `com.daml:daml-lf-archive-reader` (the decoder Canton itself uses). No JVM
//! is required to *use* the SDK or the codegen — the oracle is dev/CI-only.

mod dar;
mod decode;

pub use dar::{Dar, DarError};
pub use decode::{
    DecodeError, decode_all, decode_main_package, decode_package, imported_package_id,
    interned_dotted_name, interned_str, interned_type, package_name, package_version,
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
