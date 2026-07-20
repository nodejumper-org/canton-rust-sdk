# ADR-0004: Returned streams are detached from the client lifetime (`use<>`)

**Status:** Accepted (M1) · **Date:** 2026-07

## Context

In edition 2024, return-position `impl Trait` captures **all** in-scope
lifetimes by default. Every SDK method returning `impl Stream` from `&self`
therefore borrowed the client for the stream's whole life — even though the
bodies only clone cheap handles. `tokio::spawn`-ing a stream (the dominant
consumer pattern) failed to compile (E0505) without pre-cloning the client.

## Decision

Every stream-returning method declares precise capturing — `impl Stream +
Send + use<>` — and bodies clone what they need up front. A permanent
compile-test (`canton-ledger/tests/stream_capture_probe.rs`) drops the client
and spawns each stream; any signature regression breaks the build.

## Consequences

- `let s = client.updates(...).await?; drop(client); tokio::spawn(s)` works.
- Method bodies must not actually borrow from `self` past the first await —
  enforced by the compiler through the `use<>` bound itself.
