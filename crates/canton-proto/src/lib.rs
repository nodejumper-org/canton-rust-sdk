//! Generated Canton Ledger API v2 gRPC types and client stubs.
//!
//! This crate is internal SDK plumbing: the modules mirror the proto package
//! tree (`com.daml.ledger.api.v2` and friends, plus `google.rpc`). Generated
//! code is not held to the workspace lint set.
//!
//! # Stability policy
//!
//! The generated types are **exempt from the SDK's SemVer guarantees**: their
//! shape is owned by the wire protocol, not this SDK. They track the vendored
//! `.proto` files, which are **pinned to a Canton release** (currently 3.5.7)
//! and re-vendored when the supported Canton range moves — such re-vendoring
//! (and `prost`/`tonic` major bumps) may change these types in any release.
//! Until the M2 typed bindings land, client crates intentionally expose these
//! wire types on the dynamic path (re-exported as `canton_ledger::proto` and
//! friends); treat that surface as *protocol-stable* (stable while the pinned
//! Canton release line is stable) rather than *SemVer-stable*. The hand-written
//! SDK types (`Config`, `Error`, builders, clients) carry the SemVer promise.
#![allow(missing_docs, clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::large_enum_variant, clippy::doc_lazy_continuation)]
#![allow(rustdoc::all)]

/// `google.rpc` types imported by the Ledger API (e.g. `Status` in completions).
pub mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

/// The canonical gRPC health-checking service (`grpc.health.v1`), which Canton
/// serves on the Ledger API port for node health probes.
pub mod grpc {
    pub mod health {
        pub mod v1 {
            tonic::include_proto!("grpc.health.v1");
        }
    }
}

/// ScalaPB custom-options types imported by the Canton admin-api protos.
pub mod scalapb {
    tonic::include_proto!("scalapb");
}

/// The Canton / Daml proto package tree (`com.daml.*` Ledger API and
/// `com.digitalasset.canton.*` admin API).
pub mod com {
    pub mod daml {
        pub mod ledger {
            pub mod api {
                pub mod v2 {
                    tonic::include_proto!("com.daml.ledger.api.v2");

                    pub mod admin {
                        tonic::include_proto!("com.daml.ledger.api.v2.admin");
                    }

                    pub mod interactive {
                        tonic::include_proto!("com.daml.ledger.api.v2.interactive");

                        pub mod transaction {
                            pub mod v1 {
                                tonic::include_proto!(
                                    "com.daml.ledger.api.v2.interactive.transaction.v1"
                                );
                            }
                        }
                    }

                    pub mod testing {
                        tonic::include_proto!("com.daml.ledger.api.v2.testing");
                    }
                }
            }
        }
    }

    /// Canton admin-api proto tree (topology read service + its dependencies).
    pub mod digitalasset {
        pub mod canton {
            pub mod crypto {
                pub mod v30 {
                    tonic::include_proto!("com.digitalasset.canton.crypto.v30");
                }
            }
            pub mod protocol {
                pub mod v30 {
                    tonic::include_proto!("com.digitalasset.canton.protocol.v30");
                }
            }
            pub mod topology {
                pub mod admin {
                    pub mod v30 {
                        tonic::include_proto!("com.digitalasset.canton.topology.admin.v30");
                    }
                }
            }
        }
    }
}
