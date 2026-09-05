# Token-standard OpenAPI documents

The registry HTTP API that `canton-token` speaks is specified by these seven
documents from the Splice repository, vendored here so the crate's paths and
payloads are pinned against a *specific* revision rather than whatever
`main` says on the day someone looks.

| File | Source (in `hyperledger-labs/splice`) | Document version |
|---|---|---|
| `token-metadata-v1.yaml` | `token-standard/splice-api-token-metadata-v1/openapi/` | 1.2.0 |
| `transfer-instruction-v1.yaml` | `token-standard/splice-api-token-transfer-instruction-v1/openapi/` | 1.1.0 |
| `allocation-v1.yaml` | `token-standard/splice-api-token-allocation-v1/openapi/` | 1.1.0 |
| `allocation-instruction-v1.yaml` | `token-standard/splice-api-token-allocation-instruction-v1/openapi/` | 1.0.0 |
| `transfer-instruction-v2.yaml` | `token-standard/splice-api-token-transfer-instruction-v2/openapi/` | 1.0.0 |
| `allocation-v2.yaml` | `token-standard/splice-api-token-allocation-v2/openapi/` | 1.0.0 |
| `allocation-instruction-v2.yaml` | `token-standard/splice-api-token-allocation-instruction-v2/openapi/` | 1.0.0 |

**Pinned revision:** git tag `0.6.11` — the Splice release the LocalNet these
crates are verified against runs (`splice-app:0.6.11`, `canton:0.6.11`). Fetched
2026-09-05 from
`https://raw.githubusercontent.com/hyperledger-labs/splice/0.6.11/token-standard/<package>/openapi/<file>`.

Why the pin matters: the copies of `token-metadata-v1` that ship inside
cn-quickstart are document version 1.0.0, and the crate's `Instrument` type was
first transcribed from them; 1.2.0 adds `paused`, `pauseInfo` and
`accountInputFieldsToShow`. Without a pinned document there is nothing to diff
against and that kind of drift is invisible.

To bump: change the tag in the URL above, refetch all seven, update the table,
and run `cargo test -p canton-token` — the in-process suite checks every path
the crate spells against these files.
