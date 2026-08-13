// The one description of how this repository's bindings crates are generated.
//
// `include!`d by `examples/regenerate_splice.rs`, which writes them, and by
// `tests/up_to_date.rs`, which checks the committed files still match. Those
// two must agree exactly — the external-package map is part of the output, so
// a table copied into both would let them drift apart and make the guard pass
// against the wrong expectation. It lives under `examples/shared/`, which
// Cargo does not treat as a target (no `main.rs`), so it compiles only where
// it is included.

/// Where a DAR comes from.
#[derive(Clone, Copy)]
enum DarSource {
    /// A file in `$SPLICE_DARS` — everything cn-quickstart *ships* as a DAR.
    SpliceDir(&'static str),
    /// A path in a variable of its own, for a DAR a checkout *builds*.
    Env(&'static str),
}

/// `(crate, DAR, packages referenced from crates of their own)`
///
/// The externals list is a crate's Daml dependencies, transitively: naming only
/// the direct ones would leave the rest copied in, which is the defect this
/// whole arrangement removes. Order within a list does not matter.
const CRATES: &[(&str, DarSource, &[&str])] = &[
    // Leaves — nothing else in the corpus is below them.
    (
        "canton-splice-api-token-metadata-v1",
        DarSource::SpliceDir("splice-api-token-metadata-v1-1.0.0.dar"),
        &[],
    ),
    (
        "canton-splice-api-featured-app-v1",
        DarSource::SpliceDir("splice-api-featured-app-v1-1.0.0.dar"),
        &[],
    ),
    (
        "canton-splice-api-token-holding-v1",
        DarSource::SpliceDir("splice-api-token-holding-v1-1.0.0.dar"),
        &[METADATA],
    ),
    (
        "canton-splice-api-token-allocation-v1",
        DarSource::SpliceDir("splice-api-token-allocation-v1-1.0.0.dar"),
        HOLDING,
    ),
    (
        "canton-splice-api-token-burn-mint-v1",
        DarSource::SpliceDir("splice-api-token-burn-mint-v1-1.0.0.dar"),
        HOLDING,
    ),
    (
        "canton-splice-api-token-transfer-instruction-v1",
        DarSource::SpliceDir("splice-api-token-transfer-instruction-v1-1.0.0.dar"),
        HOLDING,
    ),
    (
        "canton-splice-api-token-allocation-instruction-v1",
        DarSource::SpliceDir("splice-api-token-allocation-instruction-v1-1.0.0.dar"),
        ALLOCATION,
    ),
    (
        "canton-splice-api-token-allocation-request-v1",
        DarSource::SpliceDir("splice-api-token-allocation-request-v1-1.0.0.dar"),
        ALLOCATION,
    ),
    // The application DARs. `splice-util` has no DAR of its own, so it stays
    // generated inside amulet — and wallet reaches it through amulet.
    (
        "canton-splice-amulet",
        DarSource::SpliceDir("splice-amulet-0.1.14.dar"),
        API,
    ),
    (
        "canton-splice-wallet-payments",
        DarSource::SpliceDir("splice-wallet-payments-0.1.14.dar"),
        WITH_AMULET,
    ),
    (
        "canton-splice-wallet",
        DarSource::SpliceDir("splice-wallet-0.1.14.dar"),
        WITH_PAYMENTS,
    ),
    // The reference app. Its DAR is built by a cn-quickstart checkout rather
    // than shipped, so it has a variable of its own and is skipped when unset.
    // It is not published, but it is the crate an application developer reads,
    // and it duplicated four token-API packages — which is exactly what stops
    // it from being usable next to `canton-splice-amulet`. Its copies carry the
    // same package ids as the standalone crates, so referencing them is not an
    // upgrade of anything: it is the same Daml package, named once.
    (
        "canton-quickstart-licensing",
        DarSource::Env("CANTON_LICENSING_DAR"),
        ALLOCATION_REQUEST,
    ),
];

const METADATA: &str = "splice-api-token-metadata-v1";
const HOLDING: &[&str] = &[METADATA, "splice-api-token-holding-v1"];
const ALLOCATION: &[&str] = &[
    METADATA,
    "splice-api-token-holding-v1",
    "splice-api-token-allocation-v1",
];
const ALLOCATION_REQUEST: &[&str] = &[
    METADATA,
    "splice-api-token-holding-v1",
    "splice-api-token-allocation-v1",
    "splice-api-token-allocation-request-v1",
];
const API: &[&str] = &[
    METADATA,
    "splice-api-token-holding-v1",
    "splice-api-token-allocation-v1",
    "splice-api-token-allocation-instruction-v1",
    "splice-api-token-transfer-instruction-v1",
    "splice-api-featured-app-v1",
];
const WITH_AMULET: &[&str] = &[
    METADATA,
    "splice-api-token-holding-v1",
    "splice-api-token-allocation-v1",
    "splice-api-token-allocation-instruction-v1",
    "splice-api-token-transfer-instruction-v1",
    "splice-api-featured-app-v1",
    "splice-amulet",
];
const WITH_PAYMENTS: &[&str] = &[
    METADATA,
    "splice-api-token-holding-v1",
    "splice-api-token-allocation-v1",
    "splice-api-token-allocation-instruction-v1",
    "splice-api-token-transfer-instruction-v1",
    "splice-api-featured-app-v1",
    "splice-amulet",
    "splice-wallet-payments",
];

/// `splice-api-token-holding-v1` → `canton_splice_api_token_holding_v1`
fn crate_ident(package: &str) -> String {
    format!("canton-{package}").replace('-', "_")
}

/// The map handed to the lowering pass: package name → the crate that owns it.
fn externals_for(packages: &[&str]) -> canton_codegen::ExternalPackages {
    let mut map = canton_codegen::ExternalPackages::new();
    for package in packages {
        map = map.with(*package, crate_ident(package));
    }
    map
}

/// Resolve a DAR to a path, or `None` when its source is not configured.
fn dar_path(source: DarSource) -> Option<String> {
    match source {
        DarSource::SpliceDir(name) => {
            std::env::var("SPLICE_DARS").ok().map(|dir| format!("{dir}/{name}"))
        }
        DarSource::Env(var) => std::env::var(var).ok(),
    }
}
