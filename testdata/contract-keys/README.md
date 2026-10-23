# Contract-keys fixture

`contractkeys.dar` is a small Daml package whose templates declare contract
keys, built for Daml-LF 2.3. It is the keyed DAR the conformance row, the
codegen compile test and the `lf-conformance` job run on, because no Splice
DAR declares a key.

| | |
|---|---|
| Source | [`peacefulstudio/daml-codegen-csharp`](https://github.com/peacefulstudio/daml-codegen-csharp), path `conformance/contractkeys/`, commit `3660768797374f19477587afda27ac664675d77d` (the repository is now `canton-dotnet-sdk`) |
| License | Apache-2.0, © 2026 Peaceful Studio OÜ |
| Built with | Daml SDK 3.5.2, `--target=2.3` |
| Package | `contractkeys-h61a17c769e62` 0.0.1 |
| Contents | 30 packages: 3 at Daml-LF 2.3, 27 at 2.1 (`daml-prim`, `daml-stdlib`) |
| Keyed templates | `Steward` (party key), `Account`, `Holiday` (record keys), `Schedule` (a key built by a function), `Membership`, `Enrollment` (tuple keys, one with an optional part) |
| SHA-256 | `67d90e0682bf71f1f3b3281b6190b7834e084b7851bbc712278e15bcad75f236` (`contractkeys.dar.sha256`, checked in CI before use) |

Vendored unmodified. The Daml sources are at the commit above.
