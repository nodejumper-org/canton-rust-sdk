//! Daml values off the wire, in both encodings: the Ledger API's proto
//! `Value` decoded into the runtime's types, and the same types read from
//! LF-JSON.
#![no_main]

use canton_daml::{FromValue as _, Numeric, Party, Timestamp};
use canton_ledger::proto::Value;
use libfuzzer_sys::fuzz_target;
use prost::Message as _;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = Value::decode(data) {
        let _ = String::from_value(&value);
        let _ = i64::from_value(&value);
        let _ = bool::from_value(&value);
        let _ = Party::from_value(&value);
        let _ = Numeric::from_value(&value);
        let _ = Timestamp::from_value(&value);
        let _ = Vec::<String>::from_value(&value);
        let _ = Option::<i64>::from_value(&value);
    }
    let _ = serde_json::from_slice::<Timestamp>(data);
    let _ = serde_json::from_slice::<Numeric>(data);
    let _ = serde_json::from_slice::<Party>(data);
});
