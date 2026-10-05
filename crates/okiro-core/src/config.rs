//! Configuration loader.
//!
//! The shape of `okiro.toml` is intentionally minimal: a top-level
//! array of `[[hosts]]` tables, each with a required `name` and `mac`
//! plus optional `host`, `ssh_user`, `ssh_port`, and `poweroff_cmd`.
//!
//! Cross-platform default path resolution lives in Phase 3
//! (`Config::default_path` via the `dirs` crate). For Phase 1 we
//! expose `Config::from_toml_str` and `Config::load(path)` so the
//! binary can wire it up.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{ConfigError, CoreError};
use crate::host::{Host, HostError, MacAddr};
use std::str::FromStr;

/// Validated, in-memory configuration.
#[derive(Debug, Default, Clone)]
pub struct Config {
    hosts: Vec<Host>,
}

/// Wire format of `okiro.toml` (private — callers go through
/// [`Config::from_toml_str`] / [`Config::load`]).
#[derive(Debug, Default, Deserialize)]
struct RawConfigFile {
    #[serde(default)]
    hosts: Vec<RawHost>,
}

#[derive(Debug, Deserialize)]
struct RawHost {
    name: String,
    mac: String,
    #[serde(default)]
    host: Option<String>,
    #[serde(default)]
    ssh_user: Option<String>,
    #[serde(default)]
    ssh_port: Option<u16>,
    #[serde(default)]
    poweroff_cmd: Option<String>,
}

impl Config {
    /// Load and validate a config from a TOML string.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] if the TOML is malformed, or
    /// [`ConfigError::DuplicateHost`] / [`ConfigError::InvalidMac`]
    /// if the entries are inconsistent.
    pub fn from_toml_str(s: &str) -> Result<Self, CoreError> {
        let raw: RawConfigFile = toml::from_str(s).map_err(|source| {
            CoreError::Config(ConfigError::Parse {
                path: "<inline>".to_owned(),
                source,
            })
        })?;
        Self::from_raw(raw)
    }

    /// Load and validate a config from an arbitrary file path.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Read`] if the file cannot be read, or
    /// any error from [`Self::from_toml_str`].
    pub fn load(path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let path = path.as_ref();
        let s = fs::read_to_string(path).map_err(|source| {
            CoreError::Config(ConfigError::Read {
                path: path.display().to_string(),
                source,
            })
        })?;
        Self::from_toml_str(&s).map_err(|err| match err {
            CoreError::Config(ConfigError::Parse { source, .. }) => {
                CoreError::Config(ConfigError::Parse {
                    path: path.display().to_string(),
                    source,
                })
            }
            other => other,
        })
    }

    fn from_raw(raw: RawConfigFile) -> Result<Self, CoreError> {
        let mut hosts = Vec::with_capacity(raw.hosts.len());
        let mut seen: HashMap<String, ()> = HashMap::with_capacity(raw.hosts.len());

        for raw_host in raw.hosts {
            if seen.insert(raw_host.name.clone(), ()).is_some() {
                return Err(CoreError::Config(ConfigError::DuplicateHost {
                    name: raw_host.name,
                }));
            }
            let mac = MacAddr::from_str(&raw_host.mac).map_err(|source| {
                CoreError::Config(ConfigError::InvalidMac {
                    name: raw_host.name.clone(),
                    mac: raw_host.mac.clone(),
                    source,
                })
            })?;
            hosts.push(Host {
                name: raw_host.name,
                mac,
                host: raw_host.host,
                ssh_user: raw_host.ssh_user,
                ssh_port: raw_host.ssh_port,
                poweroff_cmd: raw_host.poweroff_cmd,
            });
        }

        Ok(Self { hosts })
    }

    /// All configured hosts in load order.
    #[must_use]
    pub fn hosts(&self) -> &[Host] {
        &self.hosts
    }

    /// Look up a host by name.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::Host`] (variant [`HostError::NotFound`])
    /// if no entry matches.
    pub fn resolve(&self, name: &str) -> Result<&Host, CoreError> {
        self.hosts.iter().find(|h| h.name == name).ok_or_else(|| {
            CoreError::Host(HostError::NotFound {
                name: name.to_owned(),
            })
        })
    }
}

/// Cross-platform default config path.
///
/// Stubbed in Phase 1 — Phase 3 will replace this with the `dirs`
/// crate to produce the per-OS path table documented in the README.
#[deprecated(note = "default_path() is implemented in Phase 3")]
#[must_use]
pub fn default_path() -> Option<PathBuf> {
    None
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_config() {
        let toml = r#"
                [[hosts]]
                name = "laptop"
                mac  = "aa:bb:cc:dd:ee:ff"
            "#;
        let cfg = Config::from_toml_str(toml).expect("valid minimal config");
        assert_eq!(cfg.hosts().len(), 1);
        let h = cfg.resolve("laptop").expect("laptop present");
        assert_eq!(h.mac.to_string(), "AA:BB:CC:DD:EE:FF");
        assert!(h.ssh_port.is_none());
    }

    #[test]
    fn rejects_duplicate_host_names() {
        let toml = r#"
                [[hosts]]
                name = "laptop"
                mac  = "aa:bb:cc:dd:ee:ff"
                [[hosts]]
                name = "laptop"
                mac  = "11:22:33:44:55:66"
            "#;
        let err = Config::from_toml_str(toml).expect_err("duplicates");
        assert!(matches!(
            err,
            CoreError::Config(ConfigError::DuplicateHost { .. })
        ));
    }

    #[test]
    fn rejects_invalid_mac() {
        let toml = r#"
                [[hosts]]
                name = "laptop"
                mac  = "not-a-mac"
            "#;
        let err = Config::from_toml_str(toml).expect_err("bad mac");
        assert!(matches!(
            err,
            CoreError::Config(ConfigError::InvalidMac { .. })
        ));
    }

    #[test]
    fn resolve_returns_not_found() {
        let cfg = Config::from_toml_str(
            r#"
                [[hosts]]
                name = "laptop"
                mac  = "aa:bb:cc:dd:ee:ff"
            "#,
        )
        .unwrap();
        let err = cfg.resolve("desktop").expect_err("missing host");
        assert!(matches!(err, CoreError::Host(HostError::NotFound { .. })));
    }
}
