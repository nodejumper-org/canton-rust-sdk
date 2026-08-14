//! Every capability this SDK claims has a test named after it.
//!
//! The proposal asks for "a conformance suite mapped to the Ledger Client
//! Standard", and the map is only worth something if it cannot quietly stop
//! being one. `conformance/capabilities.toml` is generated from the capability
//! matrix DA published; this asserts that the suite covers it exactly — no row
//! without a test, and no test claiming a row that does not exist.
//!
//! Checked against the *source* of the suite rather than a run of it, because a
//! test that exists and is ignored would otherwise count.
#![allow(clippy::unwrap_used, clippy::expect_used)]

const REGISTRY: &str = include_str!("../../../conformance/capabilities.toml");
const SUITE: &str = include_str!("conformance.rs");

/// How many rows of the Ledger Client Standard fall in this SDK's scope —
/// every M1, M2 and M3 row of the capability matrix DA published, excluding the
/// post-v1 ones the proposal defers.
///
/// Pinned as a number because the map it is counted from is a working document
/// outside this repository. That makes it the one thing here a human must keep
/// honest — and it is what closes the hole the first two guards could not see:
/// they compared the registry to the suite, so a row missing from *both* was
/// invisible to them. Ten were, including an M3 row. Now dropping a row makes
/// the sum short and this fails.
const IN_SCOPE_ROWS: usize = 49;

/// The `id = "…"` values under `[[capability]]` — what the SDK claims.
fn capabilities() -> Vec<String> {
    ids_under("[[capability]]")
}

/// The `id = "…"` values under `[[gap]]` — in-scope rows it does not implement.
fn gaps() -> Vec<String> {
    ids_under("[[gap]]")
}

/// Every `id` belonging to blocks of one kind.
fn ids_under(header: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let mut inside = false;
    for line in REGISTRY.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("[[") {
            inside = trimmed == header;
            continue;
        }
        if !inside {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("id")
            && let Some(value) = rest.trim_start().strip_prefix('=')
        {
            ids.push(value.trim().trim_matches('"').to_string());
        }
    }
    ids
}

/// The `fn <name>()` of every test in the suite.
fn suite_tests() -> Vec<String> {
    SUITE
        .lines()
        .filter_map(|line| {
            let rest = line
                .trim()
                .strip_prefix("async fn ")
                .or_else(|| line.trim().strip_prefix("fn "))?;
            Some(rest.split('(').next()?.to_string())
        })
        .collect()
}

#[test]
fn every_capability_has_a_test_named_after_it() {
    let capabilities = capabilities();
    let tests = suite_tests();
    let missing: Vec<_> = capabilities
        .iter()
        .filter(|id| !tests.contains(id))
        .collect();
    assert!(
        missing.is_empty(),
        "these capabilities are claimed with no test named for them: {missing:#?}"
    );
}

#[test]
fn no_test_claims_a_capability_that_is_not_registered() {
    let capabilities = capabilities();
    let stray: Vec<_> = suite_tests()
        .into_iter()
        // Helpers are named plainly; a conformance test carries the `__` that
        // separates an LCS section from the capability within it.
        .filter(|name| name.contains("__") && !capabilities.contains(name))
        .collect();
    assert!(
        stray.is_empty(),
        "these tests name a capability the registry does not list: {stray:#?}"
    );
}

/// Every in-scope row of the standard is accounted for — as a capability with a
/// test, or as a named gap.
///
/// This is the guard the first version did not have, and the reason it needed
/// one: the other two compare the registry to the suite, which cannot notice a
/// row absent from both. Ten were absent from both, among them **Explicit
/// disclosure** — an M3 row, implemented, and a named sub-item of the token
/// standard deliverable. The published matrix said the registry "lists every
/// capability of the standard this SDK claims", and CI derived its own
/// threshold from that same short file, so the gate grew *easier* as the
/// registry shrank.
#[test]
fn every_in_scope_row_of_the_standard_is_accounted_for() {
    let claimed = capabilities().len();
    let gaps = gaps().len();
    assert_eq!(
        claimed + gaps,
        IN_SCOPE_ROWS,
        "the registry accounts for {claimed} claimed + {gaps} gap(s) = {}, but the standard has \
         {IN_SCOPE_ROWS} in-scope rows. A row was dropped, or one was added to the standard and \
         nobody decided whether this SDK implements it.",
        claimed + gaps
    );
}

/// A gap is a statement that the SDK does *not* do something, so a test named
/// after one would be claiming the opposite.
#[test]
fn nothing_claims_a_capability_the_registry_records_as_a_gap() {
    let tests = suite_tests();
    let claimed: Vec<_> = gaps().into_iter().filter(|id| tests.contains(id)).collect();
    assert!(
        claimed.is_empty(),
        "these are recorded as gaps but have a test claiming them: {claimed:#?}"
    );
}

/// A gap must say why. "Not implemented" with no reason is what a reviewer
/// cannot weigh, and what quietly becomes permanent.
#[test]
fn every_gap_gives_a_reason() {
    let reasons = REGISTRY
        .lines()
        .filter(|line| line.trim_start().starts_with("reason"))
        .count();
    assert_eq!(
        reasons,
        gaps().len(),
        "every gap needs a `reason`; {} gap(s) have {reasons}",
        gaps().len()
    );
}
