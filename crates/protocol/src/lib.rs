pub mod crypto;
pub mod error;
pub mod pairing;
mod wire;

pub use error::{Error, Result};

// Re-export all public wire types and functions at the crate root.
pub use wire::{
    // Secure message types
    AgentCommand,
    AgentEvent,
    // Peer-to-peer frame types
    Handshake,
    HandshakeConfirm,
    // Constants
    PROTOCOL_VERSION,
    PROTOCOL_VERSION_MIN,
    PeerFrame,
    // Relay types
    PeerRole,
    PeerStatus,
    PushNotification,
    RegisterRequest,
    RegisterResponse,
    RelayError,
    RelayMessage,
    RelayRoute,
    SealedFrame,
    SecureMessage,
    TodoItem,
    VoiceAction,
    // Encode/decode functions
    decode_peer_frame,
    decode_relay,
    decode_secure_message,
    encode_peer_frame,
    encode_relay,
    encode_secure_message,
};
