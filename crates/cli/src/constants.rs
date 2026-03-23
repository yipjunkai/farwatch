/// Production relay server URL. Users connect here by default.
pub const DEFAULT_RELAY_URL: &str = "wss://farwatch-relay.fly.dev/ws";

/// Production control API URL for registration and key management.
#[cfg(feature = "hosted")]
pub const DEFAULT_CONTROL_API_URL: &str = "https://farwatch-control.fly.dev";

/// Environment variable name to override the relay URL.
pub const RELAY_URL_ENV: &str = "FARWATCH_URL";

/// Environment variable name to override the control API URL.
#[cfg(feature = "hosted")]
pub const CONTROL_API_URL_ENV: &str = "FARWATCH_CONTROL_URL";

/// Environment variable name for the API key.
#[cfg(feature = "hosted")]
pub const API_KEY_ENV: &str = "FARWATCH_API_KEY";

/// Directory name for local state (under user's home directory).
pub const STATE_DIR_NAME: &str = ".farwatch";

/// Application name used in CLI help and metadata.
pub const APP_NAME: &str = "farwatch";

/// Client version sent during relay registration.
pub const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
