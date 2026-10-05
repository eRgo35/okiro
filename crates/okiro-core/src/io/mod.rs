//! I/O trait definitions used by command implementations.
//!
//! All four external interactions (`WolSender`, `Pinger`, `SshRunner`,
//! `BrowserOpener`) are abstracted behind small traits so tests can
//! inject deterministic fakes (notably `MockWolSender`).
//!
//! Default implementations are intentionally minimal for 0.x.2.3 —
//! the binary crate owns platform-specific defaults (`socket2` for
//! WOL, `webbrowser` for browse, `std::process::Command` for ssh)
//! and passes them into `okiro-core` command functions.

use std::time::Duration;

use crate::host::MacAddr;

/// Sends Wake-on-LAN magic packets.
pub trait WolSender {
    /// Send a single magic packet to the broadcast address
    /// `255.255.255.255:9`.
    ///
    /// # Errors
    ///
    /// Returns [`WolError::Send`] if the underlying socket fails.
    fn send(&self, mac: MacAddr) -> Result<(), WolError>;
}

/// Wake-on-LAN failure.
#[derive(Debug, thiserror::Error)]
pub enum WolError {
    /// Socket send failed.
    #[error("failed to send wake-on-lan packet: {0}")]
    Send(String),
}

/// Pings a host by name or IP.
pub trait Pinger {
    /// Send one ICMP echo and return the outcome.
    ///
    /// # Errors
    ///
    /// Returns [`PingError::Timeout`] if the host did not respond in
    /// `timeout`, or [`PingError::Other`] for transport-level errors.
    fn ping(&self, host: &str, timeout: Duration) -> Result<PingResult, PingError>;
}

/// Ping outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PingResult {
    /// Round-trip latency. `None` for the (yet-to-be-implemented)
    /// fast-fail variant.
    pub latency: Option<Duration>,
}

/// Ping failure.
#[derive(Debug, thiserror::Error)]
pub enum PingError {
    /// No reply within the timeout window.
    #[error("ping to {host} timed out after {timeout_secs}s")]
    Timeout {
        /// Host that was pinged.
        host: String,
        /// Timeout window.
        timeout_secs: u64,
    },
    /// Any other failure.
    #[error("ping to {host} failed: {message}")]
    Other {
        /// Host that was pinged.
        host: String,
        /// Failure description.
        message: String,
    },
}

/// Spawns an interactive SSH connection.
pub trait SshRunner {
    /// Run `ssh` (or an equivalent) connecting to `target`.
    ///
    /// # Errors
    ///
    /// Returns [`SshError::Spawn`] if the binary cannot be started
    /// or returns non-zero exit.
    fn run(&self, target: &SshTarget) -> Result<(), SshError>;
}

/// Parameters for an SSH invocation.
#[derive(Debug, Clone)]
pub struct SshTarget {
    /// Hostname or IP.
    pub host: String,
    /// Optional user.
    pub user: Option<String>,
    /// Optional port. Defaults to 22.
    pub port: Option<u16>,
}

impl SshTarget {
    /// Construct from the resolved `Host` entry.
    #[must_use]
    pub fn from_host(host: &crate::host::Host, fallback_host: Option<&str>) -> Self {
        Self {
            host: host
                .host
                .clone()
                .or_else(|| fallback_host.map(str::to_string))
                .unwrap_or_default(),
            user: host.ssh_user.clone(),
            port: host.ssh_port,
        }
    }
}

/// SSH runner failure.
#[derive(Debug, thiserror::Error)]
pub enum SshError {
    /// Spawning the SSH binary failed or it exited non-zero.
    #[error("ssh failed: {0}")]
    Spawn(String),
}

/// Opens a URL in the user's default browser.
pub trait BrowserOpener {
    /// # Errors
    ///
    /// Returns [`BrowserError::Open`] on platform-level failure.
    fn open(&self, url: &str) -> Result<(), BrowserError>;
}

/// Browser-opener failure.
#[derive(Debug, thiserror::Error)]
pub enum BrowserError {
    /// The browser could not be launched.
    #[error("could not open url {url}: {message}")]
    Open {
        /// URL that failed.
        url: String,
        /// Reason.
        message: String,
    },
}

// ---------------------------------------------------------------------
// Mock implementations used by tests and as a documentation example of
// how to inject fakes. They live here so the library has zero binary
// dependencies; the binary crate composes its own production impls.
// ---------------------------------------------------------------------

/// Test-only `WolSender` that records calls and never touches a
/// socket.
#[derive(Debug, Default)]
pub struct MockWolSender {
    /// `Some(call)` after the first `send`, `None` initially.
    pub sent: parking_lot::Mutex<Option<MacAddr>>,
}

impl WolSender for MockWolSender {
    fn send(&self, mac: MacAddr) -> Result<(), WolError> {
        *self.sent.lock() = Some(mac);
        Ok(())
    }
}
