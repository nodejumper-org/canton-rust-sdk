# API reference

Every published crate is documented on docs.rs, built from the released
version. The links below always point at the latest release.

## Start here

| Crate | |
|---|---|
| [`canton`](https://docs.rs/canton/latest/canton/) | The Canton Network Rust SDK: one-stop entry point re-exporting the canton-* crate family. |

## Ledger API

| Crate | |
|---|---|
| [`canton-ledger`](https://docs.rs/canton-ledger/latest/canton_ledger/) | Async Canton Ledger API client (gRPC): commands, streaming, de-duplication, recovery. |
| [`canton-auth`](https://docs.rs/canton-auth/latest/canton_auth/) | JWT/OIDC authentication for the Canton Rust SDK (client-credentials, caching, refresh). |
| [`canton-core`](https://docs.rs/canton-core/latest/canton_core/) | Core types for the Canton Rust SDK: error model, telemetry, and the shared connection kernel (config, auth, TLS, retry). |
| [`canton-admin`](https://docs.rs/canton-admin/latest/canton_admin/) | Canton admin client: party management, user self-inspect, and topology read (gRPC). |
| [`canton-signer`](https://docs.rs/canton-signer/latest/canton_signer/) | Pluggable transaction signing for Canton interactive submission (HSM/KMS-compatible). |
| [`canton-pqs`](https://docs.rs/canton-pqs/latest/canton_pqs/) | Typed read client for the Daml Participant Query Store (PQS/Scribe): typed predicates compiled to parameterized JSONB queries. |

## Daml models and codegen

| Crate | |
|---|---|
| [`canton-daml`](https://docs.rs/canton-daml/latest/canton_daml/) | Runtime types and Value codecs for canton-codegen-generated Daml bindings. |
| [`canton-codegen`](https://docs.rs/canton-codegen/latest/canton_codegen/) | Type-safe Rust code generation from Daml packages (DAR to Rust). |
| [`canton-codegen-cli`](https://crates.io/crates/canton-codegen-cli) | dpm codegen-rust: generate a typed Rust crate of bindings from a DAR. |
| [`canton-lf`](https://docs.rs/canton-lf/latest/canton_lf/) | Daml-LF archive (DAR/DALF) reader and decoder for the Canton Rust SDK. |
| [`canton-daml-stdlib`](https://docs.rs/canton-daml-stdlib/latest/canton_daml_stdlib/) | Pre-built typed Rust bindings for the Daml standard library (daml-stdlib, daml-prim, ghc-stdlib). |

## Token standard

| Crate | |
|---|---|
| [`canton-token`](https://docs.rs/canton-token/latest/canton_token/) | Canton Network token-standard workflows (CIP-56 and CIP-0112): registry client, choice contexts, transfers and allocations. |
| [`canton-splice-api-featured-app-v1`](https://docs.rs/canton-splice-api-featured-app-v1/latest/canton_splice_api_featured_app_v1/) | Pre-built typed Rust bindings for the Splice `splice-api-featured-app-v1` DAR. |
| [`canton-splice-api-token-metadata-v1`](https://docs.rs/canton-splice-api-token-metadata-v1/latest/canton_splice_api_token_metadata_v1/) | Pre-built typed Rust bindings for the Splice `splice-api-token-metadata-v1` DAR. |
| [`canton-splice-api-token-holding-v1`](https://docs.rs/canton-splice-api-token-holding-v1/latest/canton_splice_api_token_holding_v1/) | Pre-built typed Rust bindings for the Splice `splice-api-token-holding-v1` DAR. |
| [`canton-splice-api-token-holding-v2`](https://docs.rs/canton-splice-api-token-holding-v2/latest/canton_splice_api_token_holding_v2/) | Pre-built typed Rust bindings for the Splice `splice-api-token-holding-v2` package (CIP-0112). |
| [`canton-splice-api-token-allocation-v1`](https://docs.rs/canton-splice-api-token-allocation-v1/latest/canton_splice_api_token_allocation_v1/) | Pre-built typed Rust bindings for the Splice `splice-api-token-allocation-v1` DAR. |
| [`canton-splice-api-token-allocation-v2`](https://docs.rs/canton-splice-api-token-allocation-v2/latest/canton_splice_api_token_allocation_v2/) | Pre-built typed Rust bindings for the Splice `splice-api-token-allocation-v2` package (CIP-0112). |
| [`canton-splice-api-token-burn-mint-v1`](https://docs.rs/canton-splice-api-token-burn-mint-v1/latest/canton_splice_api_token_burn_mint_v1/) | Pre-built typed Rust bindings for the Splice `splice-api-token-burn-mint-v1` DAR. |
| [`canton-splice-api-token-transfer-events-v2`](https://docs.rs/canton-splice-api-token-transfer-events-v2/latest/canton_splice_api_token_transfer_events_v2/) | Pre-built typed Rust bindings for the Splice `splice-api-token-transfer-events-v2` package (CIP-0112). |
| [`canton-splice-api-token-transfer-instruction-v1`](https://docs.rs/canton-splice-api-token-transfer-instruction-v1/latest/canton_splice_api_token_transfer_instruction_v1/) | Pre-built typed Rust bindings for the Splice `splice-api-token-transfer-instruction-v1` DAR. |
| [`canton-splice-api-token-transfer-instruction-v2`](https://docs.rs/canton-splice-api-token-transfer-instruction-v2/latest/canton_splice_api_token_transfer_instruction_v2/) | Pre-built typed Rust bindings for the Splice `splice-api-token-transfer-instruction-v2` package (CIP-0112). |
| [`canton-splice-api-token-allocation-instruction-v1`](https://docs.rs/canton-splice-api-token-allocation-instruction-v1/latest/canton_splice_api_token_allocation_instruction_v1/) | Pre-built typed Rust bindings for the Splice `splice-api-token-allocation-instruction-v1` DAR. |
| [`canton-splice-api-token-allocation-instruction-v2`](https://docs.rs/canton-splice-api-token-allocation-instruction-v2/latest/canton_splice_api_token_allocation_instruction_v2/) | Pre-built typed Rust bindings for the Splice `splice-api-token-allocation-instruction-v2` package (CIP-0112). |
| [`canton-splice-api-token-allocation-request-v1`](https://docs.rs/canton-splice-api-token-allocation-request-v1/latest/canton_splice_api_token_allocation_request_v1/) | Pre-built typed Rust bindings for the Splice `splice-api-token-allocation-request-v1` DAR. |
| [`canton-splice-api-token-allocation-request-v2`](https://docs.rs/canton-splice-api-token-allocation-request-v2/latest/canton_splice_api_token_allocation_request_v2/) | Pre-built typed Rust bindings for the Splice `splice-api-token-allocation-request-v2` package (CIP-0112). |

## Splice bindings

| Crate | |
|---|---|
| [`canton-splice-amulet`](https://docs.rs/canton-splice-amulet/latest/canton_splice_amulet/) | Pre-built typed Rust bindings for the Splice `splice-amulet` DAR. |
| [`canton-splice-wallet`](https://docs.rs/canton-splice-wallet/latest/canton_splice_wallet/) | Pre-built typed Rust bindings for the Splice `splice-wallet` DAR. |
| [`canton-splice-wallet-payments`](https://docs.rs/canton-splice-wallet-payments/latest/canton_splice_wallet_payments/) | Pre-built typed Rust bindings for the Splice `splice-wallet-payments` DAR. |

## Generated protocol types

| Crate | |
|---|---|
| [`canton-proto`](https://docs.rs/canton-proto/latest/canton_proto/) | Generated Canton Ledger API v2 gRPC types and client stubs (internal to the SDK). |

`canton-codegen-cli` is a binary (`dpm-codegen-rust`), so it has no API
documentation; its usage is in [Typed bindings from a DAR](guides/codegen.md).
