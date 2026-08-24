# Where these `.proto` files come from

Everything under this directory is vendored third-party schema, pinned to a
Canton release. `build.rs` compiles the whole tree with a vendored `protoc`, so
neither a consumer nor docs.rs needs network access or system tooling — see
[ADR-0001](../../../docs/adr/0001-vendored-protos-pinned-to-a-canton-release.md)
for why vendoring rather than a submodule or a build-time download.

## What is pinned

| Tree | Upstream | Pinned to |
| --- | --- | --- |
| `com/daml/ledger/api/v2/**` | Canton Ledger API v2 (`digital-asset/canton`, release line 3.5) | **3.5.7** |
| `com/digitalasset/canton/**` | Canton admin API — the topology-read closure only | **3.5.7** |
| `grpc/health/v1/health.proto` | gRPC health checking protocol | v1 (stable) |
| `google/**` | `google.rpc` status/error-details and well-known types | as shipped with the above |
| `scalapb/**` | ScalaPB options, imported by the Canton protos | as shipped with the above |

The supported Canton range is stated in
[`docs/compatibility-matrix.md`](../../../docs/compatibility-matrix.md); moving
it is a deliberate re-vendor commit, not a silent update.

## Integrity

`SHA256SUMS` records a SHA-256 for every `.proto` in this tree, and
`canton-proto`'s test suite verifies it on every run (`cargo test -p
canton-proto`). The point is narrow and worth stating plainly: it does not
prove the files came from upstream, it proves **nobody has hand-edited them
since they were vendored**. A local tweak to a third-party schema is the kind
of change that works locally and then diverges from the wire format the
participant actually speaks.

To re-generate after a deliberate re-vendor:

```sh
tools/vendor-protos.sh --rehash
```

## Verified against a running 3.5.7 participant

The official Canton image ships its own copy of these schemas at
`/app/protobuf`. Comparing this tree against the image that runs the LocalNet
these crates are tested on
(`ghcr.io/digital-asset/decentralized-canton-sync/docker/canton:0.6.11`,
bundling `canton-open-source-3.5.7.jar`), on 2026-08-19:

- **33 of 45 files are byte-identical.**
- **10 differ, and every difference is an addition on our side** — fields and
  documentation present here and absent there, such as
  `Completion.transaction_hash` (which its own comment dates to 3.5). Nothing
  in the image's copies is missing from ours: across all ten files the
  image-only lines are three, and none of them declares a field (a Google
  copyright year, `cc_enable_arenas`, one doc line). These copies are a later
  snapshot of the same 3.5 line, and a superset is wire-compatible in both
  directions — a field the participant never sets decodes as absent.
- **2 are not in that tree at all**: `grpc/health/v1/health.proto`, which is
  gRPC's own schema rather than Canton's, and
  `party_management_alpha_service.proto`.

Re-run that comparison with `tools/vendor-protos.sh --compare-image <container>`.
