# The Ledger Client Standard, row by row

Digital Asset's *Ledger Client Standard* (the "Client Standard" capability
matrix, 2026-06) groups what a ledger client must do into domains, each tiered
Core (basic, extended) and Upcoming. Grant #407 targets the Core surface. This
file is the published map from that matrix to this SDK: of the standard's
70 rows, the 51 the grant delivers — the milestone that delivered each,
where it lives, and the name of the conformance test that claims it, one test
per row, named after the row, in
[`crates/canton-conformance/tests/conformance.rs`](../crates/canton-conformance/tests/conformance.rs) —
and, at the end, the 19 it does not claim.

[`capabilities.toml`](capabilities.toml) is the machine-readable half of this
file. The guards in `tests/completeness.rs` hold the two and the suite
together: every capability has a test named after it, no test claims a row the
registry does not list, capabilities plus declared gaps account for every
in-scope row (51), nothing claims a row recorded as a gap, and every gap gives
a reason. The standard's own wording for a row is given where this map
carries it.

## Codegen and bindings

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| Daml representation (codegen) | Code-gen tool Daml types → native; must support all serializable types, interfaces, contract keys, dependencies | M2 | `canton-codegen` | `codegen_and_bindings__daml_representation_codegen` |
| gRPC Bindings | wrappers bridging proto-generated classes ↔ Daml codegen | M2 | `canton-ledger` (gRPC) | `codegen_and_bindings__grpc_bindings` |
| gRPC Codec | serialize/deserialize generated objects to proto | M2 | generated + `canton-daml` | `codegen_and_bindings__grpc_codec` |
| JSON Bindings | wrappers bridging OpenAPI-generated classes ↔ Daml codegen | M2 | `canton-ledger` (JSON) | `codegen_and_bindings__json_bindings` |
| JSON Codec | serialize/deserialize generated objects to Ledger API JSON | M2 | generated + `canton-daml` | `codegen_and_bindings__json_codec` |
| PQS bindings | bridge PQS tables ↔ Daml codegen | M3 | `canton-pqs` | `codegen_and_bindings__pqs_bindings` |

## Basic infrastructure

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| Authorization | client-credentials OAuth, token refresh, JWT injection into call context | M1 | `canton-core` (`TokenProvider`) | `basic_infrastructure__authorization` |
| Error parsing & handling | classify retriable/non-retriable, JSON+gRPC errors, extract code/message/request-info/error-info/structured details | M1 | `canton-core` (`Error`, error parser) | `basic_infrastructure__error_parsing_and_handling` |
| Logging | standard lib, log req/resp, trace-ids, verbose LAPI errors, configurable level/destination/format | M1 | `canton-core` | `basic_infrastructure__logging` |
| Metrics | OTel metrics to OTLP: req/resp counts, success/error + per-endpoint breakdown | M1 | `canton-core` | `basic_infrastructure__metrics` |
| Node Health | monitor node health so apps respond to intermittent/permanent failure | M1 | `canton-core`/`canton-admin` health check | `basic_infrastructure__node_health` |
| Retry logic | retries on retriable only, timeouts, max retries | M1 | `canton-core` (tower layer) | `basic_infrastructure__retry_logic` |
| Signing | algorithms for externally-signed endpoints: KMS, file keys, all Canton algos | M3 | `canton-signer` (trait) | `basic_infrastructure__signing` |
| TLS | secure HTTP+gRPC, mutual + server-side | M1 | `canton-core` / `canton-ledger` | `basic_infrastructure__tls` |
| Tracing | OpenTelemetry: create+inject trace-ids into request headers, extract from response, JSON+gRPC, report spans to OTLP | M1 | `canton-core` | `basic_infrastructure__tracing` |

## Commands

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| Command recovery | recover pending state after participant/client crash, lost connection, timeout, via completion endpoint | M1 | `canton-ledger` | `commands__command_recovery` |
| Contract keys | exercise-by-key, key prefetch | M2 | `canton-codegen` + `canton-ledger` | `commands__contract_keys` |
| Deduplication | reliable submission via change-ID dedup, error handling, retries | M1 | `canton-ledger` + `canton-core` | `commands__deduplication` |
| Explicit disclosure | augment commands with disclosed contracts | M3 | `canton-ledger` / `canton-token` | `commands__explicit_disclosure` |
| External commands | `prepare`/`execute`/`executeAndWait` (externally signed) | M3 | `canton-ledger` + `canton-signer` | `commands__external_commands` |
| gRPC | construct + submit create/exercise over gRPC | M1 | `canton-ledger` | `commands__grpc` |
| Internal commands | `submit` + `submitAndWait` (all forms) | M1 | `canton-ledger` | `commands__internal_commands` |
| JSON | construct + submit create/exercise over JSON | M1 | `canton-ledger` | `commands__json` |

## Streams

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| ACS paging | paged ACS retrieval, gRPC + JSON | M1 | `canton-ledger` | `streams__acs_paging` |
| ACS streaming | streaming (gRPC) / WebSocket (JSON) ACS retrieval | M1 | `canton-ledger` | `streams__acs_streaming` |
| gRPC | query + subscribe ACS/Update streams over gRPC | M1 | `canton-ledger` | `streams__grpc` |
| Interfaces | subscribe to + process interface views | M2 | `canton-codegen` + `canton-ledger` | `streams__interfaces` |
| JSON | query + subscribe ACS/Update streams over JSON (WebSocket) | M1 | `canton-ledger` | `streams__json` |
| Resilient streams | recover from last offset / record-time vector / continuation token | M1 | `canton-ledger` | `streams__resilient_streams` |
| Reverse order | serve data in reverse order | M1 | `canton-ledger` | `streams__reverse_order` |
| Updates paging | paged updates | M1 | `canton-ledger` | `streams__updates_paging` |
| Updates streaming | streaming updates | M1 | `canton-ledger` | `streams__updates_streaming` |

## Parties

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| External parties creation |  | M3 | `AdminClient::generate_external_party_topology` + `allocate_external_party`: the participant describes the onboarding for a public key, the key… | `parties__external_parties_creation` |
| gRPC party mgmt |  | M1 | `PartyManagementService` | `parties__grpc_party_mgmt` |
| JSON party mgmt |  | M1 | `JsonClient::list_known_parties_page` / `list_known_parties` / `get_parties` / `allocate_party` / `allocate_party_with` / `update_party_details` /… | `parties__json_party_mgmt` |
| List parties |  | M1 | with filtering | `parties__list_parties` |
| Local parties creation |  | M1 | self-administration mode | `parties__local_parties_creation` |

## Packages

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| gRPC package mgmt |  | M1 | `canton-admin` (read) | `packages__grpc_package_mgmt` |
| JSON package mgmt |  | M1 | `JsonClient::list_packages` / `package_status` — the reads, as `canton-admin` has them over gRPC. Upload stays out: `POST /v2/dars` is an operator… | `packages__json_package_mgmt` |
| Listing |  | M1 | list package ids | `packages__listing` |
| Vetting |  | M1 | list vetted packages (topology read) | `packages__vetting` |

## Topology

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| Generic mappings |  | M1 | `canton-admin` (read) | `topology__generic_mappings` |
| List mappings |  | M1 | party→participant mappings (`TopologyManagerReadService`) | `topology__list_mappings` |
| Namespace delegations |  | M1 | read only | `topology__namespace_delegations` |

## User

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| Self-inspect (pull own record/permissions) |  | M1 | `canton-admin` via `UserManagementService` | `user__self_inspect` |

## Multi-synchronizer

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| Listing connected synchronizers |  | M3 | `CantonClient::connected_synchronizers` (`GetConnectedSynchronizers`), so a party's synchronizer is asked of the participant rather than… | `multi_synchronizer__listing_connected_synchronizers` |

## Token Standard

| Row | The standard asks for | Milestone | Where | Conformance test |
|---|---|---|---|---|
| Allocate |  | M3 | two-step allocate path | `token_standard__allocate` |
| Instrument inspection |  | M3 | registry/instrument metadata | `token_standard__instrument_inspection` |
| One-step transfers |  | M3 | sender-initiated, receiver pre-approved | `token_standard__one_step_transfers` |
| Pre-approvals |  | M3 | `TransferPreapproval` | `token_standard__pre_approvals` |
| Transfer |  | M3 | `TransferFactory_Transfer` | `token_standard__transfer` |

## Rows the grant does not claim

The rows of the standard the proposal lists under "out of scope for v1", with
the tier they belong to. None has a conformance test, by design: a test that
claims a row the SDK does not implement is what the guards exist to refuse.

| Domain | Row | The standard asks for | Status |
|---|---|---|---|
| Commands | Batching | batched LAPI commands | roadmap (post-v1) |
| Commands | Pending set | track consuming/non-consuming exercises + creates in-flight | roadmap (post-v1) |
| Commands | Package selection | select package-version preference for submission | roadmap (post-v1) |
| Commands | HA (dedup) | dedup across multiple participants hosting same party | roadmap (post-v1) |
| Streams | Topology events | subscribe to + process topology events | roadmap (post-v1) |
| Streams | In-memory ACS | in-process ACS copy, param by parties/templates, filters, kept current off update stream, interface views | roadmap (post-v1) |
| Streams | HA (streams) | recover after switch-over to secondary participant | roadmap (post-v1) |
| Parties | Local parties modification |  | roadmap (post-v1) |
| Parties | External parties modification |  | roadmap (post-v1) |
| Parties | Decentralized party creation |  | roadmap (post-v1) |
| Packages | Upload |  | roadmap (post-v1) |
| Packages | Package preference |  | roadmap (post-v1) |
| Packages | Description |  | roadmap (post-v1) |
| Topology | Decentralized namespaces |  | roadmap (post-v1) |
| User | Self-admin (manage parties attached to user) |  | roadmap (post-v1) |
| Multi-synchronizer | Parties across synchronizers |  | roadmap (post-v1) |
| Multi-synchronizer | Packages (per-synchronizer vetting) |  | roadmap (post-v1) |
| Multi-synchronizer | Commands (target-synchronizer selection + retry tracking) |  | roadmap (post-v1) |
| Multi-synchronizer | Streams |  | roadmap (post-v1) |
