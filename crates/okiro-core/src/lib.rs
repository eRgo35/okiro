//! `okiro-core` — core library: configuration, host resolution, and
//! trait-based I/O for okiro. The binary crate `okiro` is a thin
//! clap-based wrapper around the public items re-exported here.
//!
//! In the 0.x series, most command modules (`ping`, `browse`, `ssh`,
//! `poweroff`, `status`, `list`) are still stubs that exist to lock
//! the public API surface. Only `wake` ships as a complete
//! implementation. See
//! `docs/superpowers/specs/2026-10-05-okiro-mvp-plan.md` for the
//! roadmap.

#![deny(missing_docs)]
#![deny(rust_2018_idioms)]

pub mod commands;
pub mod config;
pub mod error;
pub mod host;
pub mod io;

pub use config::{Config, default_path};
pub use error::CoreError;
pub use host::{Host, HostError};
pub use io::{
    BrowserOpener, MockWolSender, PingError, PingResult, Pinger, SshError, SshRunner, WolError,
    WolSender,
};
