//! Host types: the in-memory shape of `[[hosts]]` entries plus the
//! resolution errors that can arise from looking them up by name.

use std::fmt;
use std::str::FromStr;

use macaddr::MacAddr as RawMacAddr;

/// A single host entry from `okiro.toml`.
///
/// Only `name` and `mac` are required. The optional fields are parsed
/// eagerly so that future 1.0 commands can read them without a
/// configuration migration, but they are currently consumed only by
/// `okiro-core::commands::ssh`/`poweroff` (which themselves are
/// 0.x stubs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    /// Stable, user-chosen identifier (`okiro wake <name>`).
    pub name: String,
    /// MAC address used by `okiro wake`.
    pub mac: MacAddr,
    /// Optional hostname or IP. Used by `ping`, `browse`, `ssh`.
    pub host: Option<String>,
    /// Optional SSH login user.
    pub ssh_user: Option<String>,
    /// Optional SSH port. Defaults to 22.
    pub ssh_port: Option<u16>,
    /// Optional poweroff command; default is `sudo shutdown -h now`.
    pub poweroff_cmd: Option<String>,
}

/// Strongly-typed MAC address backed by the `macaddr` crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddr(RawMacAddr);

impl MacAddr {
    /// Raw 6-byte representation.
    #[must_use]
    pub fn as_bytes(&self) -> [u8; 6] {
        let bytes = self.0.as_bytes();
        let mut out = [0u8; 6];
        out.copy_from_slice(bytes);
        out
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MacAddr {
    type Err = macaddr::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<RawMacAddr>().map(MacAddr)
    }
}

/// Errors that can occur while resolving a `Host` by name.
#[derive(Debug, thiserror::Error)]
pub enum HostError {
    /// No host with that name was found in the loaded config.
    #[error("no host named '{name}' in config")]
    NotFound {
        /// Name that was looked up.
        name: String,
    },

    /// A MAC string failed to parse.
    #[error("invalid MAC address '{raw}': {source}")]
    ParseMac {
        /// Raw MAC input.
        raw: String,
        /// Underlying parser error.
        #[source]
        source: macaddr::ParseError,
    },
}
