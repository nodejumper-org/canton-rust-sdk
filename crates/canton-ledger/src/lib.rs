//! Async Canton Ledger API client (gRPC + JSON).
//!
//! [`CantonClient`] speaks the gRPC Ledger API v2: command submission
//! (fire-and-forget [`CantonClient::submit`] and
//! [`CantonClient::submit_and_wait_for_transaction`]) with change-ID
//! de-duplication and completion-based recovery, ACS + update streaming
//! (paging, reverse-order, event query, and offset-**resumable** streams), a
//! node health check, and an opt-in retry pipeline. [`JsonClient`] mirrors the
//! core surface over the HTTP JSON Ledger API (submit + bounded reads, plus
//! WebSocket streaming behind the `ws` feature). Both share the
//! [`canton_core`] connection kernel (endpoint, [`Auth`](canton_core::Auth),
//! TLS, retry) and error model.
//!
//! ```no_run
//! # async fn run() -> canton_ledger::Result<()> {
//! use canton_ledger::{CantonClient, Config};
//!
//! let client = CantonClient::connect_lazy(Config::new("http://localhost:3901"))?;
//! println!("ledger api version: {}", client.version().await?);
//! # Ok(())
//! # }
//! ```

mod client;
mod command;
mod json;
#[cfg(feature = "ws")]
mod ws;

pub use canton_core::{Config, Error, Result, RetryConfig, TlsConfig};
pub use canton_proto::grpc::health::v1::health_check_response::ServingStatus;
pub use client::CantonClient;
pub use command::{Submit, create, exercise, identifier, record, value};
pub use json::{JsonClient, JsonCommands, JsonSubmitResponse, JsonTransaction};

/// The generated Ledger API v2 protobuf types, for the dynamic (untyped)
/// command path until typed bindings land in Milestone 2.
///
/// **Stability:** these wire types are *protocol-stable*, not SemVer-stable —
/// they track the vendored protos pinned to a Canton release and are exempt
/// from this crate's SemVer guarantees (see the `canton-proto` stability
/// policy). The hand-written SDK surface carries the SemVer promise.
pub use canton_proto::com::daml::ledger::api::v2 as proto;
