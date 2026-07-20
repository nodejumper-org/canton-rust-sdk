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
//!
//! The shared foundation (`canton-core`: [`Config`], [`Error`], TLS, retry)
//! is re-exported at the crate root — the same types `canton-ledger` itself
//! re-exports, so both entry points name identical items.
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
pub use canton_ledger as ledger;

pub use canton_core::{
    Auth, Config, Error, ErrorInfo, Result, RetryConfig, TlsConfig, TokenSource,
};
