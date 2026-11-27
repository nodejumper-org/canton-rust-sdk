//! A DAR is a zip of Daml-LF archives, and it comes from wherever the caller
//! got it. Opening one and decoding every package in it may fail and must
//! not panic or allocate without bound.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(dar) = canton_lf::Dar::from_bytes(data) else {
        return;
    };
    let _ = dar.name();
    let _ = dar.main_package_bytes();
    for (_, bytes) in dar.package_entries() {
        let _ = canton_lf::decode_package(bytes);
    }
});
