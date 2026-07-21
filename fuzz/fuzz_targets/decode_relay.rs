#![no_main]
//! Fuzz the relay wire decoder over arbitrary bytes.
//!
//! `decode_relay` is the relay server's untrusted-input boundary: it turns
//! bytes from any connected client into a `RelayMessage`. Beyond "no panic",
//! this asserts that decode/encode are exact inverses on the accepted set —
//! anything the decoder accepts must re-encode and decode back to an equal
//! value. A mismatch means a wire-format asymmetry (a message that survives a
//! round-trip changed shape), which is a correctness bug worth surfacing.

use libfuzzer_sys::fuzz_target;
use protocol::{decode_relay, encode_relay};

fuzz_target!(|data: &[u8]| {
    if let Ok(message) = decode_relay(data) {
        let reencoded = encode_relay(&message).expect("a decoded RelayMessage must re-encode");
        let roundtripped =
            decode_relay(&reencoded).expect("a re-encoded RelayMessage must decode");
        assert_eq!(
            message, roundtripped,
            "relay decode/encode is not a stable round-trip"
        );
    }
});
