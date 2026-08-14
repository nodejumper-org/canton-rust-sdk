//! The Canton Network Rust SDK — one-stop entry point.
//!
//! This crate is a thin facade over the `canton-*` family: it contains no
//! logic of its own, only re-exports, so `cargo add canton` brings in the
//! whole SDK as one coherent, version-locked set. Each member crate can also
//! be used directly (`cargo add canton-ledger`) when only a subset is needed.
//!
//! | Module | Crate | What it is |
//! |---|---|---|
//! | [`ledger`] | `canton-ledger` | The async Ledger API v2 client (gRPC + JSON + WebSocket) |
//! | [`auth`] | `canton-auth` | JWT/OIDC authentication (client-credentials, caching, refresh) |
//! | [`admin`] | `canton-admin` | Party management, user self-inspect, topology read |
//! | [`daml`] | `canton-daml` | The runtime under generated bindings: Daml primitives, codecs, command builders |
//! | [`signer`] | `canton-signer` | Pluggable signing for interactive submission — Ed25519 built in, HSM/KMS via the trait |
//! | [`token`] | `canton-token` | Token-standard workflows: CIP-56 at the root, CIP-0112 under `token::v2` |
//! | [`pqs`] | `canton-pqs` | Typed reads from the Participant Query Store (`pqs-tls` for TLS) |
//!
//! The shared foundation (`canton-core`: [`Config`], [`Error`], TLS, retry)
//! is re-exported at the crate root — the same types `canton-ledger` itself
//! re-exports, so both entry points name identical items.
//!
//! # Typed bindings
//!
//! [`daml`] is the runtime half of the codegen story: generated bindings
//! depend on it (as `rt`) for [`daml::Party`], [`daml::ContractId`], the
//! `Template`/`Choice` traits, and the command builders. It is re-exported
//! here so a `cargo add canton` user gets a version-locked runtime without a
//! second dependency line.
//!
//! Generating the bindings themselves is a **build-time** step, so the
//! generator is deliberately *not* re-exported: install the CLI with
//! `cargo install canton-codegen-cli`, or depend on `canton-codegen` from a
//! build script.
//!
//! # Feature flags
//!
//! Features are forwarded to the crates that implement them:
//!
//! * `ws` — WebSocket streaming for the JSON transport (`canton-ledger/ws`).
//! * `otel` — OTLP export + W3C trace-context propagation (`canton-ledger/otel`).
//!
//! # Quickstart
//!
//! ```no_run
//! use canton::ledger::{CantonClient, Config};
//!
//! # async fn run() -> canton::Result<()> {
//! let client = CantonClient::connect_lazy(Config::new("http://localhost:3901"))?;
//! println!("ledger api version: {}", client.version().await?);
//! # Ok(())
//! # }
//! ```
//!
//! This SDK is a community project funded by Canton dev-fund proposal
//! [#407](https://github.com/canton-foundation/canton-dev-fund/pull/407);
//! it is not an official Digital Asset product.

pub use canton_admin as admin;
pub use canton_auth as auth;
pub use canton_daml as daml;
pub use canton_ledger as ledger;
/// Typed reads from the Participant Query Store.
pub use canton_pqs as pqs;
/// Pluggable signing for interactive submission (HSM/KMS-compatible).
pub use canton_signer as signer;
/// Token-standard workflows: CIP-56 (V1) at the root, CIP-0112 (V2) under `v2`.
pub use canton_token as token;

/// Reading a local development network out of the environment — the variables
/// `canton-devkit localnet env` exports, and the party ids that come with them.
pub use canton_core::localnet;
/// What the SDK emits: span and metric names, and the transport labels they
/// carry. An application building a dashboard needs these, and reaching them
/// meant depending on `canton-core` directly.
pub use canton_core::telemetry;

pub use canton_core::{
    Auth, Config, Error, ErrorCategory, ErrorInfo, ResourceInfo, Result, RetryConfig, TlsConfig,
    TokenSource,
};
