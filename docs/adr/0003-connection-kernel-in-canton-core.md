# ADR-0003: Shared connection kernel lives in `canton-core`

**Status:** Accepted (M1) · **Date:** 2026-07

## Context

`canton-ledger` and `canton-admin` both need endpoint/TLS/auth/retry wiring.
Originally these were `pub(crate)` inside `canton-ledger`; `canton-admin`
would have had to duplicate them or depend on `canton-ledger`.

## Decision

Lift the connection kernel — `Config`, `Auth` + the object-safe `TokenSource`
trait, `TlsConfig`, channel building, and the retry pipeline — into
`canton-core`. Client crates depend on the kernel, never on each other:

```
canton-proto → canton-core → canton-auth → {canton-ledger, canton-admin}
```

`TokenSource` (in core) breaks the would-be cycle with `canton-auth`:
`TokenProvider` implements the trait, so `Config::with_oidc` accepts it
without core knowing about auth.

## Consequences

- One TLS/auth/retry implementation, identical behavior across clients (the
  JSON and WS transports build from the same `TlsConfig`).
- No client→client coupling; M2/M3 crates (`canton-codegen` runtime,
  `canton-token`, `canton-pqs`) get the same foundation.
- `canton-core` carries tonic's transport+TLS stack; auth-only consumers pay
  that compile cost (acceptable; revisit with a `connect` feature if demanded).
