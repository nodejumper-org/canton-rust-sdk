//! The two encoders a generated template carries must agree.
//!
//! The emitter writes the payload's field list twice: once in `ToValue`, and
//! once in `Template::to_record`. They are not the same code path at runtime
//! either — `create_command` submits `to_record`, and a contract read back
//! comes home through `from_value`. So a template whose two lists disagreed
//! would write one shape to the ledger and expect another when reading it, and
//! nothing in the workspace compared them: the round-trip tests all go
//! `to_value` → `from_value`, which is the half `create_command` does not use.
//!
//! The sample's live loop does catch it — submit, read back, compare — but only
//! against a running participant, which is not where a regression should first
//! be noticed.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_daml as rt;
use canton_quickstart_licensing::quickstart_licensing::Licensing_AppInstall::AppInstallRequest;
// `Metadata` comes from the crate that owns that Daml package, not from a
// copy inside the app's own bindings — so the value built here is the same
// Rust type every other crate in the ecosystem names.
use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata;
use rt::{Template as _, ToValue as _};

fn payload() -> AppInstallRequest {
    AppInstallRequest {
        provider: rt::Party::new("provider::1220ab"),
        user: rt::Party::new("user::1220cd"),
        meta: Metadata {
            values: rt::TextMap::new(),
        },
    }
}

/// What `create_command` puts on the ledger is what `from_value` reads back.
#[test]
fn the_submitted_record_decodes_as_the_payload_it_came_from() {
    let original = payload();

    // `to_record` is the submit path: `create_command` carries exactly this.
    let record = original.to_record();
    let decoded: AppInstallRequest = rt::from_record(&record).expect("the record it just wrote");

    assert_eq!(
        decoded, original,
        "a template's `to_record` and `FromValue` disagree, so a create would \
         write a shape the read path does not expect"
    );
}

/// And the two encoders produce the same thing, field for field.
///
/// Stronger than the round-trip above: a pair of encoders could both be wrong
/// in the same way and still round-trip. This says the submit path and the
/// codec path are the same bytes.
#[test]
fn to_record_and_to_value_encode_identically() {
    use canton_ledger::proto::value::Sum;

    let payload = payload();
    let submitted = rt::Value {
        sum: Some(Sum::Record(payload.to_record())),
    };

    assert_eq!(
        submitted,
        payload.to_value(),
        "`to_record` and `ToValue` are written from the same field list by the \
         emitter, and a template whose two copies drifted would submit one \
         shape and encode another"
    );
}
