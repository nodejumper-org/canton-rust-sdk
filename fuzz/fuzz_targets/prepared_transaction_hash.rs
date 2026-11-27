//! The hash a prepared transaction is signed over, recomputed from bytes a
//! participant chose. Whatever decodes is hashed under both schemes; the
//! answer may be an error and must not be a panic or a hang.
#![no_main]

use canton_ledger::hashing::{
    hash_prepared_transaction, hash_prepared_transaction_v2, hash_prepared_transaction_v3,
};
use canton_ledger::proto::interactive::{HashingSchemeVersion, PreparedTransaction};
use libfuzzer_sys::fuzz_target;
use prost::Message as _;

fuzz_target!(|data: &[u8]| {
    let Ok(prepared) = PreparedTransaction::decode(data) else {
        return;
    };
    let _ = hash_prepared_transaction_v2(&prepared);
    let _ = hash_prepared_transaction_v3(&prepared);
    for scheme in [
        HashingSchemeVersion::Unspecified,
        HashingSchemeVersion::V2,
        HashingSchemeVersion::V3,
    ] {
        let _ = hash_prepared_transaction(&prepared, scheme);
    }
});
