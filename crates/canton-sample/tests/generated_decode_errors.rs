//! What a decode failure tells a user, checked against **real generated
//! bindings** rather than a hand-written imitation of them.
//!
//! The emitter wraps every field decode in `.at(label)`, and `ValueError::at`
//! prepends, so a failure several records down should arrive naming the path it
//! came from. Nothing pinned that end to end: the runtime's own fixtures are
//! flat, so `at()` was only ever exercised one level deep, and the emitter's
//! drift guards compare generated code to regenerated code — they would happily
//! agree on output that had lost the wrapping altogether.
//!
//! This crate already depends on the committed `canton-quickstart-licensing`
//! bindings, which is why the assertion lives here rather than beside the
//! emitter.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_daml as rt;
use rt::{FromValue as _, ToValue as _};

use canton_quickstart_licensing::quickstart_licensing::Licensing_AppInstall::AppInstallRequest_Reject;

/// `AppInstallRequest_Reject { meta: Metadata { values: TextMap Text } }` — two
/// records deep, which is the shallowest shape that can tell a composed path
/// from a single label.
#[test]
fn a_failure_inside_a_nested_record_names_the_path_it_came_from() {
    let wrong_type_at_meta_values = rt::record(vec![(
        "meta",
        rt::record(vec![("values", "not a map".to_string().to_value())]),
    )]);

    let error = AppInstallRequest_Reject::from_value(&wrong_type_at_meta_values)
        .expect_err("a Text is not a TextMap");
    let message = error.to_string();

    assert!(
        message.contains("meta.values"),
        "should name the full path, not just the outer or inner field: {message}"
    );
    // Both halves matter: the path says *where*, the kinds say *what*. The kind
    // is named rather than the value printed, so a payload cannot ride along
    // into the caller's logs.
    assert!(message.contains("expected TextMap"), "{message}");
    assert!(message.contains("Text"), "{message}");
    assert!(
        !message.contains("not a map"),
        "the offending value must not appear: {message}"
    );
}

/// The outer field alone is not enough. A single `.at()` anywhere in the chain
/// would still produce *a* path, so the test above has to fail for a path that
/// is merely present — this pins that it is the composed one.
#[test]
fn the_path_is_the_whole_chain_not_the_outermost_field() {
    let wrong_type_at_meta = rt::record(vec![("meta", "not a record".to_string().to_value())]);

    let error = AppInstallRequest_Reject::from_value(&wrong_type_at_meta)
        .expect_err("a Text is not a Metadata");
    let message = error.to_string();

    // One level down: the path stops where the failure is.
    assert!(message.contains("`meta`"), "{message}");
    assert!(
        !message.contains("meta.values"),
        "a failure at `meta` must not claim to be inside it: {message}"
    );
}
