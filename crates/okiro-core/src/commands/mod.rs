//! Command implementations for `okiro`.
//!
//! Each public function takes:
//!   * a `&Config` (for host lookup),
//!   * one or more `&impl Trait` I/O handles,
//!   * the `target` string passed on the CLI.
//!
//! 0.x MVP: only `wake` is implemented end-to-end. The others return
//! [`CoreError::Unimplemented`] and exist purely to lock the public
//! API surface. See
//! `docs/superpowers/specs/2026-10-05-okiro-mvp-plan.md` for the
//! roadmap to 1.0.

use crate::config::Config;
use crate::error::CoreError;
use crate::host::HostError;
use crate::io::{BrowserOpener, PingResult, Pinger, SshTarget, WolSender};

/// Send a Wake-on-LAN magic packet to the named host.
///
/// # Errors
///
/// Returns [`CoreError::Host`] if the name does not match a configured
/// host, or any error returned by [`WolSender::send`].
pub fn wake(target: &str, cfg: &Config, wol: &dyn WolSender) -> Result<(), CoreError> {
    let host = cfg.resolve(target).map_err(|err| match err {
        CoreError::Host(HostError::NotFound { name }) => {
            CoreError::Host(HostError::NotFound { name })
        }
        other => other,
    })?;
    wol.send(host.mac)?;
    Ok(())
}

/// Ping a configured host.
///
/// 0.x stub — returns [`CoreError::Unimplemented`]. The signature is
/// locked so the binary can call it today.
pub fn ping(target: &str, cfg: &Config, pinger: &impl Pinger) -> Result<PingResult, CoreError> {
    let _host = cfg.resolve(target)?;
    let _ = pinger;
    Err(CoreError::Unimplemented("ping"))
}

/// Open the host's web interface.
///
/// 0.x stub — see [`wake`] for the pattern.
pub fn browse(target: &str, cfg: &Config, browser: &impl BrowserOpener) -> Result<(), CoreError> {
    let _host = cfg.resolve(target)?;
    let _ = browser;
    Err(CoreError::Unimplemented("browse"))
}

/// Open an interactive SSH session.
///
/// 0.x stub — see [`wake`] for the pattern.
pub fn ssh(target: &str, cfg: &Config, apply: bool) -> Result<SshTarget, CoreError> {
    let _host = cfg.resolve(target)?;
    let _ = apply;
    Err(CoreError::Unimplemented("ssh"))
}

/// Request remote poweroff.
///
/// 0.x stub — see [`wake`] for the pattern.
pub fn poweroff(target: &str, cfg: &Config) -> Result<(), CoreError> {
    let _host = cfg.resolve(target)?;
    Err(CoreError::Unimplemented("poweroff"))
}

/// Show status for all configured hosts.
///
/// 0.x stub — see [`wake`] for the pattern.
pub fn status(_cfg: &Config) -> Result<(), CoreError> {
    Err(CoreError::Unimplemented("status"))
}

/// List configured host names.
///
/// 0.x stub — see [`wake`] for the pattern. The CLI uses this to
/// drive `okiro list`.
pub fn list(cfg: &Config) -> Result<Vec<String>, CoreError> {
    Ok(cfg.hosts().iter().map(|h| h.name.clone()).collect())
}
