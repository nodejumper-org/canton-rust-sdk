//! Canton administrative client: party management, user self-inspect, and
//! topology read.
//!
//! Canton exposes these across **two** gRPC endpoints, so this crate provides
//! two clients:
//!
//! - [`AdminClient`] — party management ([`PartyManagementService`]) and user
//!   self-inspect ([`UserManagementService`]), served on the **Ledger API**
//!   port (e.g. `:3901`), the same channel/auth as the ledger client. Party
//!   admin RPCs require the `ParticipantAdmin` right; self-inspect
//!   ([`AdminClient::current_user`] / [`AdminClient::current_user_rights`])
//!   works with any authenticated token.
//! - [`TopologyClient`] — topology **read** (party→participant mappings,
//!   namespace delegations, vetted packages) via `TopologyManagerReadService`,
//!   served on the Canton **admin API** port (e.g. `:3902`).
//!
//! Both build on the shared [`canton_core::Config`] (endpoint, auth, TLS,
//! retry).
//!
//! [`PartyManagementService`]: https://docs.daml.com
//! [`UserManagementService`]: https://docs.daml.com
//!
//! ```no_run
//! # async fn run() -> canton_admin::Result<()> {
//! use canton_admin::{AdminClient, Config};
//!
//! let admin = AdminClient::connect_lazy(Config::new("http://localhost:3901"))?;
//! let me = admin.current_user().await?;
//! println!("acting as: {:?}", admin.acting_parties().await?);
//! # let _ = me;
//! # Ok(())
//! # }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod topology;

pub use canton_core::{Config, Error, Result, RetryConfig, TlsConfig};
pub use canton_proto::com::daml::ledger::api::v2::PackageStatus;
pub use client::AdminClient;
pub use topology::{Entry, Store, TopologyClient};

/// Generated Ledger API admin protobuf types (party & user management).
pub use canton_proto::com::daml::ledger::api::v2::admin as proto;
/// Generated Canton protocol types (topology mapping items).
pub use canton_proto::com::digitalasset::canton::protocol::v30 as protocol_proto;
/// Generated Canton admin-API topology types (query/result envelopes).
pub use canton_proto::com::digitalasset::canton::topology::admin::v30 as topology_proto;
