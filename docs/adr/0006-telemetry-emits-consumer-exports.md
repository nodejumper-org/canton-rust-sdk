# ADR-0006: Telemetry — the SDK emits, the application exports

**Status:** Accepted (M1) · **Date:** 2026-07

## Context

An SDK that installs a global tracing subscriber, metrics recorder, or OTLP
exporter fights the host application for process-global state and drags heavy
exporter dependencies into every build.

## Decision

The SDK only **emits** through the ecosystem facades: a `canton.rpc`
`tracing` span per RPC (method + transport fields) and `metrics` counters
(`canton_client_requests_total` / `canton_client_errors_total`, labelled by
method, transport, retriability). The application installs whatever
subscriber/recorder it wants. The optional `otel` feature adds the two pieces
that genuinely belong SDK-side: an OTLP span-exporter constructor
(`telemetry::otel::otlp_tracer`) and automatic W3C trace-context injection
into outgoing gRPC metadata and JSON headers.

## Consequences

- Zero process-global state claimed by the SDK; no exporter deps in default
  builds.
- Per-endpoint success/error breakdowns come from the labels; "success" =
  requests − errors.
- Log level/format/destination are consumer-owned (standard Rust model), which
  is how the proposal's "configurable logging" is satisfied.
