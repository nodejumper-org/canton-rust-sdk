//! The fixture in `src/lib.rs` says it is written "exactly as the generator
//! emits". Nothing checked that, and it drifted twice in one working session:
//! once when generated `FromValue` bodies gained `.at(label)` on every field
//! decode, and once when it turned out the fixture never had the serde derives
//! every emitted payload carries. Both times the drift hid an untested path —
//! the composed error path, and the whole JSON half of the codec — and both
//! times the tests stayed green, because a fixture cannot notice that it has
//! stopped resembling anything.
//!
//! So the claim is checked here instead of asserted in a comment. This runs the
//! real emitter over the DAR committed to this repository and looks for the
//! constructs the fixture mirrors. It is deliberately about *shape* rather than
//! text: the fixture is hand-written and will never be byte-identical to
//! generated output, and a comparison that demanded that would be rewritten
//! into uselessness the first time it failed.
//!
//! There is no dependency cycle. `canton-codegen` depends on `canton-lf` and
//! never on this crate, and a dev-dependency reaches no consumer.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_codegen::{generate_crate, lower_dar};
use canton_lf::Dar;

/// Everything the fixture claims to mirror, as a pattern the emitter's real
/// output must contain, and the reason each one is load-bearing.
const REQUIRED: &[(&str, &str)] = &[
    (
        "rt::serde::Serialize",
        "generated payloads derive Serialize; a fixture without it leaves the \
         JSON encoder untested against the shape it mirrors",
    ),
    (
        "rt::serde::Deserialize",
        "the same for the decoder — and `Template::from_json_created_event` \
         needs it, so a fixture without it cannot exercise the typed JSON read",
    ),
    (
        "rt::required_field(value",
        "fields are located by index *and* label, because non-verbose records \
         arrive without labels",
    ),
    (
        "rt::optional_field(value",
        "an absent trailing Optional decodes as None; record normalisation \
         under Smart Contract Upgrade drops them",
    ),
    (
        ".map_err(|e| e.at(",
        "a decode failure names the field it came from; without this the \
         fixture cannot show what a nested failure reports",
    ),
    (
        "#[serde(rename",
        "the Daml field label is preserved on the wire while the Rust field is \
         snake-cased",
    ),
];

#[test]
fn the_fixture_mirrors_what_the_generator_actually_emits() {
    let dar = Dar::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testdata/splice-api-token-holding-v1-1.0.0.dar"
    ))
    .expect("the fixture DAR is committed to this repository");
    let (krate, _skipped) = lower_dar(&dar).expect("lower the DAR");
    let generated = generate_crate(&krate).expect("generate");

    let missing: Vec<&str> = REQUIRED
        .iter()
        .filter(|(pattern, _)| !generated.contains(pattern))
        .map(|(pattern, _)| *pattern)
        .collect();
    assert!(
        missing.is_empty(),
        "the emitter no longer produces {missing:?}. Either it changed and the \
         `AppInstall` fixture in src/lib.rs must change with it, or this list \
         is stale — but do not delete the entry without checking which."
    );

    // The other half: what the emitter produces, the fixture must actually
    // contain. Reading the fixture out of the source keeps the two in one
    // place; comparing behaviour is what the tests beside it already do.
    let fixture = include_str!("../src/lib.rs");
    let fixture = fixture
        .split_once("mod tests {")
        .expect("the fixture lives in the test module")
        .1;
    for (pattern, why) in REQUIRED {
        // The fixture writes these without the `rt::` prefix, since inside this
        // crate the runtime is `self`.
        let bare = pattern.trim_start_matches("rt::");
        assert!(
            fixture.contains(bare),
            "the fixture has lost `{bare}`, which the generator emits — {why}"
        );
    }

    // Presence is not enough, and neither is a count over the whole module:
    // other impls in the fixture carry `.at(` too, so their slack hides a field
    // that lost one. Count inside the decoder itself, where the number is
    // exact — the generator attaches `.at(label)` to every field it looks up,
    // so one missing means one field whose failures report no path, and that
    // field is precisely the one the fixture then cannot demonstrate.
    let decoder = block(fixture, "impl FromValue for AppInstall {");
    let lookups = decoder.matches("required_field(value").count()
        + decoder.matches("optional_field(value").count();
    let attributed = decoder.matches(".at(").count();
    assert_eq!(
        attributed, lookups,
        "the fixture's decoder looks up {lookups} fields and attributes \
         {attributed} failures to one. The generator does both for every field."
    );

    // The same, exactly, for the renames on the payload itself.
    let payload = block(fixture, "struct AppInstall {");
    assert_eq!(
        payload.matches("#[serde(rename").count(),
        lookups,
        "the fixture renames a different number of fields than it decodes. The \
         generator emits a rename for every field, and a fixture whose labels \
         all happen to equal their Rust names never exercises one at all."
    );

    // Counting renames is still not enough: a rename to the field's own name is
    // a rename that renames nothing, and a fixture made entirely of those never
    // moves a single byte between the wire label and the Rust name. Daml writes
    // camelCase and the generator snake-cases it, so at least one has to
    // differ, or the mapping this exists to demonstrate is never demonstrated.
    let differing = renamed_fields(payload)
        .filter(|(label, field)| label != field)
        .count();
    assert!(
        differing > 0,
        "every field of the fixture is renamed to its own Rust name, so nothing \
         here shows a Daml label surviving the snake-casing. Give one a \
         camelCase label."
    );
}

/// `(wire label, Rust field name)` for each renamed field in a struct body.
///
/// The generator puts the attribute immediately above the declaration, and the
/// fixture follows it, so the field is the next line carrying `name: Type`.
fn renamed_fields(payload: &str) -> impl Iterator<Item = (String, String)> + '_ {
    let lines: Vec<&str> = payload.lines().map(str::trim).collect();
    (0..lines.len()).filter_map(move |index| {
        let label = lines[index]
            .strip_prefix("#[serde(rename = \"")?
            .strip_suffix("\")]")?;
        let field = lines[index + 1..]
            .iter()
            .find_map(|line| line.split_once(": ").map(|(name, _)| name))?;
        Some((label.to_string(), field.to_string()))
    })
}

/// The source between `header` and the line that closes it, by brace depth.
///
/// Crude on purpose: it reads one hand-written fixture in one known file, and a
/// real parser here would be a second thing to keep working.
fn block<'a>(source: &'a str, header: &str) -> &'a str {
    let start = source
        .find(header)
        .unwrap_or_else(|| panic!("the fixture no longer contains `{header}`"));
    let rest = &source[start + header.len()..];
    let mut depth = 1_usize;
    for (offset, c) in rest.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &rest[..offset];
                }
            }
            _ => {}
        }
    }
    panic!("`{header}` is not closed");
}
