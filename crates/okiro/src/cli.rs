//! clap-based CLI for the `okiro` binary.

use clap::{Parser, Subcommand};

const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
const AUTHOR: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const ABOUT: Option<&str> = option_env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug, Parser)]
#[command(
    name = NAME.unwrap_or("okiro"),
    author = AUTHOR.unwrap_or("Michał Czyż <mike@c2yz.com>"),
    version = VERSION.unwrap_or("unknown"),
    about = ABOUT.unwrap_or("okiro (起きろ) — remote computer wakeup / management / shutdown tool."),
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Send Wake-on-LAN to a configured host.
    Wake {
        /// Host name as defined in `okiro.toml`.
        target: String,
    },
    /// Ping a configured host (0.x stub).
    Ping {
        /// Host name.
        target: String,
    },
    /// Show status for configured hosts (0.x stub).
    Status {},
    /// Request shutdown of a host (0.x stub).
    PowerOff {
        /// Host name.
        target: String,
    },
    /// Open a host's web dashboard (0.x stub).
    Browse {
        /// Host name.
        target: String,
    },
    /// Open an SSH session to a configured host (0.x stub).
    Ssh {
        /// Host name.
        target: String,
        /// Apply flag (reserved for 1.0 — non-interactive confirmation).
        #[arg(long)]
        apply: bool,
    },
    /// List configured host names.
    List {},
}
