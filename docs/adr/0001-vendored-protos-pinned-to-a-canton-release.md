# ADR-0001: Vendored protos, pinned to a Canton release

**Status:** Accepted (M1) · **Date:** 2026-07

## Context

The SDK's gRPC surface is generated from the Canton Ledger API v2 `.proto`
files (plus the Canton admin-api topology-read closure and `grpc.health.v1`).
The protos could be fetched at build time (git submodule, build-script
download) or vendored into the repository.

## Decision

Vendor the `.proto` files under `crates/canton-proto/proto/`, pinned to a
specific Canton release (currently **3.5.7**, extracted from the official
release artifacts). `build.rs` compiles everything under that directory with a
vendored `protoc`, so neither consumers nor docs.rs need any system tooling or
network access.

## Consequences

- Reproducible builds; `cargo build` works offline and on docs.rs.
- The supported Canton version is explicit and testable (the README
  compatibility table); moving it is a deliberate re-vendor commit.
- We carry ~a dozen third-party proto files in-tree and must refresh them per
  supported Canton release (M3's compatibility matrix automates the check).
