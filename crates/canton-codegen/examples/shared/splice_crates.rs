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
    /// A path relative to the repository root, for the DAR committed here.
    /// Always available, so a crate generated from it is guarded in CI too.
    Repo(&'static str),
}

/// Which of a DAR's packages a crate emits.
#[derive(Clone, Copy)]
enum Emits {
    /// The DAR's whole closure, minus whatever is external. The ordinary case.
    Everything,
    /// Only these package-name families — for a crate that owns packages which
    /// ship no DAR of their own and can only be taken out of somebody else's.
    Families(&'static [&'static str]),
}

/// The standard library: `daml-stdlib`, `daml-prim`, `ghc-stdlib` and their
/// per-module packages. Every DAR carries a copy, so every crate would declare
/// its own `RelTime` and no two of them would be the same Rust type.
const STDLIB: &[&str] = &["daml-stdlib", "daml-prim", "ghc-stdlib"];
const STDLIB_CRATE: &str = "canton_daml_stdlib";

/// `(crate, DAR, packages referenced from crates of their own, what it emits)`
///
/// The externals list is a crate's Daml dependencies, transitively: naming only
/// the direct ones would leave the rest copied in, which is the defect this
/// whole arrangement removes. Order within a list does not matter.
const CRATES: &[(&str, DarSource, &[&str], Emits)] = &[
    // The standard library, taken out of the DAR committed to this repository.
    // It ships no DAR of its own — it arrives only as somebody else's
    // dependency — and every DAR in the corpus carries the identical 29
    // packages, so which one it is taken from does not matter. The committed
    // one is used because it needs no checkout, which puts this crate's drift
    // guard in CI along with the rest.
    (
        "canton-daml-stdlib",
        DarSource::Repo("testdata/splice-api-token-holding-v1-1.0.0.dar"),
        &[],
        Emits::Families(STDLIB),
    ),
    // Leaves — nothing else in the corpus is below them.
    (
        "canton-splice-api-token-metadata-v1",
        DarSource::SpliceDir("splice-api-token-metadata-v1-1.0.0.dar"),
        &[],
   
        Emits::Everything,
    ),
    (
        "canton-splice-api-featured-app-v1",
        DarSource::SpliceDir("splice-api-featured-app-v1-1.0.0.dar"),
        &[],
   
        Emits::Everything,
    ),
    (
        "canton-splice-api-token-holding-v1",
        DarSource::SpliceDir("splice-api-token-holding-v1-1.0.0.dar"),
        &[METADATA],
   
        Emits::Everything,
    ),
    (
        "canton-splice-api-token-allocation-v1",
        DarSource::SpliceDir("splice-api-token-allocation-v1-1.0.0.dar"),
        HOLDING,
   
        Emits::Everything,
    ),
    (
        "canton-splice-api-token-burn-mint-v1",
        DarSource::SpliceDir("splice-api-token-burn-mint-v1-1.0.0.dar"),
        HOLDING,
   
        Emits::Everything,
    ),
    (
        "canton-splice-api-token-transfer-instruction-v1",
        DarSource::SpliceDir("splice-api-token-transfer-instruction-v1-1.0.0.dar"),
        HOLDING,
   
        Emits::Everything,
    ),
    (
        "canton-splice-api-token-allocation-instruction-v1",
        DarSource::SpliceDir("splice-api-token-allocation-instruction-v1-1.0.0.dar"),
        ALLOCATION,
   
        Emits::Everything,
    ),
    (
        "canton-splice-api-token-allocation-request-v1",
        DarSource::SpliceDir("splice-api-token-allocation-request-v1-1.0.0.dar"),
        ALLOCATION,
   
        Emits::Everything,
    ),
    // The application DARs. `splice-util` has no DAR of its own, so it stays
    // generated inside amulet — and wallet reaches it through amulet.
    (
        "canton-splice-amulet",
        DarSource::SpliceDir("splice-amulet-0.1.14.dar"),
        API,
   
        Emits::Everything,
    ),
    (
        "canton-splice-wallet-payments",
        DarSource::SpliceDir("splice-wallet-payments-0.1.14.dar"),
        WITH_AMULET,
   
        Emits::Everything,
    ),
    (
        "canton-splice-wallet",
        DarSource::SpliceDir("splice-wallet-0.1.14.dar"),
        WITH_PAYMENTS,
   
        Emits::Everything,
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
   
        Emits::Everything,
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
///
/// Every crate references the standard library — unless it *is* the standard
/// library, which cannot reference itself.
fn externals_for(packages: &[&str], emits: Emits) -> canton_codegen::ExternalPackages {
    let mut map = canton_codegen::ExternalPackages::new();
    for package in packages {
        map = map.with(*package, crate_ident(package));
    }
    if matches!(emits, Emits::Everything) {
        for family in STDLIB {
            map = map.with_prefixed(*family, STDLIB_CRATE);
        }
    }
    map
}

/// What the lowering pass emits.
fn selection_for(emits: Emits) -> canton_codegen::Selection {
    match emits {
        Emits::Everything => canton_codegen::Selection::everything(),
        Emits::Families(families) => {
            canton_codegen::Selection::only(std::iter::empty::<String>())
                .and_prefixed(families.iter().copied())
        }
    }
}

/// Resolve a DAR to a path, or `None` when its source is not configured.
fn dar_path(source: DarSource) -> Option<String> {
    match source {
        DarSource::SpliceDir(name) => {
            std::env::var("SPLICE_DARS").ok().map(|dir| format!("{dir}/{name}"))
        }
        DarSource::Env(var) => std::env::var(var).ok(),
        DarSource::Repo(rel) => Some(format!(
            "{}/../../{rel}",
            env!("CARGO_MANIFEST_DIR")
        )),
    }
}
