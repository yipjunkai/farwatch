#![no_main]
//! Fuzz the secure-channel message decoder over arbitrary bytes.
//!
//! `decode_secure_message` runs on the plaintext an attacker controls *after*
//! it decrypts under a valid session key (a compromised or malicious peer), so
//! its parsing must be total and stable. It is deliberately infallible —
//! unrecognized input becomes `SecureMessage::Unknown` for forward
//! compatibility — so the invariant is: every recognized variant must
//! re-encode and decode back unchanged. `Unknown` wraps the raw bytes and
//! re-encodes differently by design, so it is excluded from the round-trip.

use libfuzzer_sys::fuzz_target;
use protocol::{SecureMessage, decode_secure_message, encode_secure_message};

fuzz_target!(|data: &[u8]| {
    let message = decode_secure_message(data).expect("decode_secure_message is infallible");
    if !matches!(message, SecureMessage::Unknown(_)) {
        let reencoded =
            encode_secure_message(&message).expect("a recognized SecureMessage must re-encode");
        let again = decode_secure_message(&reencoded).expect("decode_secure_message is infallible");
        assert_eq!(
            message, again,
            "secure-message decode/encode is not a stable round-trip"
        );
    }
});
