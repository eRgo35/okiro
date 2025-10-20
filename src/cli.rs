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
    about = ABOUT.unwrap_or("A command-line tool for managing LaTeX projects"),
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Wake {
        target: String,
    },
    Ping {
        target: String,
    },
    Status {},
    PowerOff {
        target: String,
    },
    Browse {
        target: String,
    },
    Ssh {
        target: String,

        #[arg(long)]
        apply: bool,
    },
    List {},
}
