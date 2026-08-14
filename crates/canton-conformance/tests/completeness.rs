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

/// The `id = "…"` values, in registry order.
fn capabilities() -> Vec<String> {
    REGISTRY
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("id")?.trim_start();
            let value = rest.strip_prefix('=')?.trim();
            Some(value.trim_matches('"').to_string())
        })
        .collect()
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
    assert!(
        capabilities.len() >= 39,
        "the registry lost rows: {} left",
        capabilities.len()
    );
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
