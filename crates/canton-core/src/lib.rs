//! Core types shared across the Canton Rust SDK.
//!
//! The foundation the client crates build on: the [`Error`] type with its
//! retriable classification and the [`Result`] alias, the [`telemetry`] layer
//! (tracing spans + metrics), and the shared connection kernel — [`Config`],
//! [`Auth`]/[`TokenSource`], [`TlsConfig`], and the [`retry`] pipeline — that
//! both `canton-ledger` and `canton-admin` build their gRPC channels from.
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod auth;
pub mod config;
mod error;
pub mod retry;
pub mod telemetry;

pub use config::{Auth, Config, TlsConfig, TokenSource};
pub use error::{Error, ErrorInfo, Result};
pub use retry::RetryConfig;
