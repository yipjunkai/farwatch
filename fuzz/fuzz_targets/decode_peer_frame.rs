#![no_main]
//! Fuzz the peer-to-peer frame decoder over arbitrary bytes.
//!
//! `decode_peer_frame` is the boundary a peer's transport feeds untrusted
//! bytes into before any key exchange completes — handshake frames arrive from
//! whoever connected. Beyond "no panic", asserts decode/encode are exact
//! inverses on the accepted set: an accepted frame must re-encode and decode
//! back unchanged.

use libfuzzer_sys::fuzz_target;
use protocol::{decode_peer_frame, encode_peer_frame};

fuzz_target!(|data: &[u8]| {
    if let Ok(frame) = decode_peer_frame(data) {
        let reencoded = encode_peer_frame(&frame).expect("a decoded PeerFrame must re-encode");
        let roundtripped =
            decode_peer_frame(&reencoded).expect("a re-encoded PeerFrame must decode");
        assert_eq!(
            frame, roundtripped,
            "peer-frame decode/encode is not a stable round-trip"
        );
    }
});
