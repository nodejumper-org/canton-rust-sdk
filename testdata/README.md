# Test fixtures

`splice-api-token-holding-v1-1.0.0.dar` is the official Splice token-standard
Holding API package (Apache-2.0, from
[digital-asset/decentralized-canton-sync](https://github.com/hyperledger-labs/splice)),
vendored unmodified as a small real-world DAR so the decode → lower → emit
pipeline runs on every `cargo test` with no external setup. It bundles the
whole dependency closure (daml-prim, daml-stdlib, the metadata API) and carries
interfaces, interface choices, and a spread of data shapes.

The larger corpus (splice-amulet, splice-wallet, quickstart-licensing) stays
env-gated — see the testing section of the root README.
