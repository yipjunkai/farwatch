#![no_main]
//! Fuzz the pairing-URI parser over arbitrary text.
//!
//! `parse_pairing_uri` is fed whatever a user scans from a QR code or pastes —
//! fully attacker-controlled text. Beyond "no panic", asserts parse/build
//! agree on the accepted set: a URI that parses must rebuild via
//! `build_pairing_uri` and re-parse to the same fields. This catches
//! percent-encoding round-trip bugs (a value that survives parse but is
//! mangled on rebuild). `PairingUri` has no `PartialEq`, so fields are
//! compared explicitly.

use libfuzzer_sys::fuzz_target;
use protocol::pairing::{build_pairing_uri, parse_pairing_uri};

fuzz_target!(|data: &[u8]| {
    // Production only ever hands this parser a `&str`.
    let Ok(input) = std::str::from_utf8(data) else {
        return;
    };
    if let Ok(parsed) = parse_pairing_uri(input) {
        let rebuilt = build_pairing_uri(&parsed).expect("a parsed PairingUri must rebuild");
        let reparsed = parse_pairing_uri(&rebuilt).expect("a rebuilt pairing URI must re-parse");
        assert_eq!(parsed.relay_url, reparsed.relay_url, "relay_url not stable");
        assert_eq!(parsed.session_id, reparsed.session_id, "session_id not stable");
        assert_eq!(
            parsed.pairing_code, reparsed.pairing_code,
            "pairing_code not stable"
        );
        assert_eq!(
            parsed.expected_fingerprint, reparsed.expected_fingerprint,
            "expected_fingerprint not stable"
        );
        assert_eq!(parsed.api_key, reparsed.api_key, "api_key not stable");
    }
});
