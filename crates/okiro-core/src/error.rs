//! Top-level error type for `okiro-core`.

use std::io;

use thiserror::Error;

use crate::host::HostError;
use crate::io::WolError;

/// Errors surfaced by `okiro-core` operations.
///
/// Each variant is intentionally narrow so callers (especially the
/// binary) can decide how to render or translate them.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Configuration file could not be read or parsed.
    #[error("config error: {0}")]
    Config(#[from] ConfigError),

    /// A host lookup failed (missing, malformed, or duplicated).
    #[error("host error: {0}")]
    Host(#[from] HostError),

    /// Wake-on-LAN send failure.
    #[error("wake-on-lan error: {0}")]
    Wol(#[from] WolError),

    /// Underlying I/O failure (read, write, spawn).
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    /// A command that is not yet implemented (0.x stubs).
    #[error("not implemented: {0}")]
    Unimplemented(&'static str),
}

/// Configuration-loading errors.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// The TOML file is missing or unreadable.
    #[error("could not read config file at {path}: {source}")]
    Read {
        /// Path that failed.
        path: String,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// The TOML file is present but malformed.
    #[error("could not parse config file at {path}: {source}")]
    Parse {
        /// Path that failed.
        path: String,
        /// Underlying parser error.
        #[source]
        source: toml::de::Error,
    },

    /// Two `[[hosts]]` entries shared the same `name`.
    #[error("duplicate host name '{name}' in config")]
    DuplicateHost {
        /// Duplicate name.
        name: String,
    },

    /// A `[[hosts]]` entry had a malformed MAC address.
    #[error("host '{name}' has invalid MAC '{mac}': {source}")]
    InvalidMac {
        /// Host name.
        name: String,
        /// Original (raw) MAC string.
        mac: String,
        /// Underlying parser error.
        #[source]
        source: macaddr::ParseError,
    },
}
