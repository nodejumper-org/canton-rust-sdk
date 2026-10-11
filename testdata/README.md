# Test fixtures

`splice-api-token-holding-v1-1.0.0.dar` is the official Splice token-standard
Holding API package (Apache-2.0, from
[digital-asset/decentralized-canton-sync](https://github.com/hyperledger-labs/splice)),
vendored unmodified as a small real-world DAR so the decode → lower → emit
pipeline runs on every `cargo test` with no external setup. It bundles the
whole dependency closure (daml-prim, daml-stdlib, the metadata API) and carries
interfaces, interface choices, and a spread of data shapes.

`live/quickstart-licensing-0.0.1.dar` is the cn-quickstart licensing package
that `canton-quickstart-licensing` is generated from and that the live suites
upload to the participant before they submit commands. It was built with
`daml build` on Daml SDK 3.5.2 from
[cn-quickstart](https://github.com/digital-asset/cn-quickstart) commit
`41f2d75cd16eff28aedfaf2e9a2278a881b1c71a`, and is committed because
cn-quickstart builds it from source rather than tracking it in git.
`live/quickstart-licensing-0.0.1.dar.sha256` holds its checksum
(`68a50632961f0dece11a62f4f91f579fd6591c067fbc49adbf14321eefe9d7cf`), which
the `LocalNet` workflow verifies before uploading. Its main package id is
`edd5a8d857f6ece9b0b3b21b1096448fc5292e7614044b916746927cbefa919a`, the id
`canton-quickstart-licensing` pins in `PACKAGE_ID`.

The rest of the larger corpus (splice-amulet, splice-wallet) stays env-gated —
see the testing section of the root README.
