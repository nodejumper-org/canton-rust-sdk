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
    /// A directory of `.lfpayload` files, relative to the repository root.
    ///
    /// For packages that ship as no DAR at all — the V2 token standard is live
    /// on the network and published nowhere as a built artefact, so the only
    /// way to obtain it is to ask a participant. Each file name ends with the
    /// package id, which is the hash of its own bytes.
    Payloads(&'static str),
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
    // CIP-0112 — the V2 token standard. Each crate emits exactly its own
    // package and references the rest, as the V1 crates do. The dependency
    // graph below was not guessed: selecting one package at a time and reading
    // what the escape check reported is what produced it.
    (
        "canton-splice-api-token-holding-v2",
        V2,
        &[METADATA],
        Emits::Families(&["splice-api-token-holding-v2"]),
    ),
    (
        "canton-splice-api-token-transfer-instruction-v2",
        V2,
        V2_HOLDING,
        Emits::Families(&["splice-api-token-transfer-instruction-v2"]),
    ),
    (
        "canton-splice-api-token-transfer-events-v2",
        V2,
        V2_HOLDING,
        Emits::Families(&["splice-api-token-transfer-events-v2"]),
    ),
    (
        "canton-splice-api-token-allocation-v2",
        V2,
        V2_HOLDING,
        Emits::Families(&["splice-api-token-allocation-v2"]),
    ),
    (
        "canton-splice-api-token-allocation-instruction-v2",
        V2,
        V2_ALLOCATION,
        Emits::Families(&["splice-api-token-allocation-instruction-v2"]),
    ),
    (
        "canton-splice-api-token-allocation-request-v2",
        V2,
        V2_ALLOCATION,
        Emits::Families(&["splice-api-token-allocation-request-v2"]),
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

/// Where the V2 packages live — see `testdata/token-standard-v2/README.md`.
const V2: DarSource = DarSource::Payloads("testdata/token-standard-v2");
/// V2 reuses `metadata-v1`; there is no `metadata-v2`.
const V2_HOLDING: &[&str] = &[METADATA, "splice-api-token-holding-v2"];
const V2_ALLOCATION: &[&str] = &[
    METADATA,
    "splice-api-token-holding-v2",
    "splice-api-token-allocation-v2",
];
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
    // Every crate references the standard library — unless it *is* the standard
    // library. Keyed off what it emits rather than off its name: the stdlib
    // crate is the one whose selection is the stdlib families.
    let is_stdlib = matches!(emits, Emits::Families(f) if f == STDLIB);
    if !is_stdlib {
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

/// Resolve a source to a path, or `None` when it is not configured.
fn dar_path(source: DarSource) -> Option<String> {
    match source {
        DarSource::SpliceDir(name) => std::env::var("SPLICE_DARS")
            .ok()
            .map(|dir| format!("{dir}/{name}")),
        DarSource::Env(var) => std::env::var(var).ok(),
        DarSource::Repo(rel) | DarSource::Payloads(rel) => {
            Some(format!("{}/../../{rel}", env!("CARGO_MANIFEST_DIR")))
        }
    }
}

/// Load a source into decoded packages.
///
/// A DAR carries its whole closure; a directory of payloads carries whatever
/// was fetched from a participant. Both end up as the same `(id, package)`
/// pairs, which is what the lowering pass takes.
///
/// # Errors
/// If a file cannot be read or decoded, or a payload's name does not end with
/// its package id.
fn packages(
    source: DarSource,
    path: &str,
) -> Result<Vec<(String, canton_lf::pb::daml_lf_2::Package)>, Box<dyn std::error::Error>> {
    let DarSource::Payloads(_) = source else {
        return Ok(canton_lf::decode_all(&canton_lf::Dar::open(path)?)?);
    };
    let mut entries: Vec<_> = std::fs::read_dir(path)?
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.extension().is_some_and(|e| e == "lfpayload"))
        .collect();
    // Sorted, so the generated output does not depend on the order the
    // filesystem happens to hand them back in.
    entries.sort();

    let mut packages = Vec::new();
    for entry in entries {
        let name = entry
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("a payload with no name")?;
        // The id is the hash of the bytes, so the file name is the checksum.
        let id = name
            .rsplit('-')
            .next()
            .and_then(|s| s.strip_suffix(".lfpayload"))
            .ok_or_else(|| format!("`{name}` does not end with its package id"))?;
        packages.push((
            id.to_string(),
            canton_lf::decode_payload(&std::fs::read(&entry)?)?,
        ));
    }
    Ok(packages)
}
