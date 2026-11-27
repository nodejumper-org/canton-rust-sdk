//! The check of an external party's onboarding transaction, on bytes a
//! participant chose. No input may panic it, and none should pass it: the
//! party and key are fixed, and a transaction registering that key for that
//! party is not something a fuzzer arrives at.
#![no_main]

use canton_admin::{ExpectedHosting, ExternalPartyTopology};
use canton_signer::{KeyFormat, KeySpec, PublicKey};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let key = PublicKey::new(KeyFormat::Raw, vec![0x11; 32], KeySpec::EcCurve25519);
    let Ok(fingerprint) = key.fingerprint() else {
        return;
    };
    let party = format!("fuzz::{fingerprint}");
    for expected in [
        ExpectedHosting::any_participant(),
        ExpectedHosting::participant("participant1::1220aa"),
    ] {
        let _ = ExternalPartyTopology::from_parts(&party, vec![data.to_vec()], &key, &expected);
    }
});
