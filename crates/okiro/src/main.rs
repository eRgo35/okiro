use std::process::ExitCode;

use clap::Parser;
use colorize::AnsiColor;

use okiro_core::Config;
use okiro_core::commands;

mod cli;
mod io;

/// Phase 1: hardcoded config path. Phase 3 will replace this with
/// `Config::default_path()` via the `dirs` crate.
fn config_path() -> std::path::PathBuf {
    let home = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    home.join(".config").join("okiro").join("okiro.toml")
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    match cli.command {
        Some(cli::Commands::Status {}) => {
            println!("Status");
        }
        Some(cli::Commands::Ping { target: _ }) => {
            println!("Ping");
        }
        Some(cli::Commands::Browse { target: _ }) => {
            println!("Browse");
        }
        Some(cli::Commands::Ssh {
            target: _,
            apply: _,
        }) => {
            println!("Ssh");
        }
        Some(cli::Commands::Wake { target }) => {
            let cfg = Config::load(config_path())?;
            commands::wake(&target, &cfg, &io::UdpWolSender)?;
        }
        Some(cli::Commands::PowerOff { target: _ }) => {
            println!("PowerOff");
        }
        Some(cli::Commands::List {}) => {
            let cfg = Config::load(config_path())?;
            for name in commands::list(&cfg)? {
                println!("{name}");
            }
        }
        None => {
            println!("None");
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("{} {}", "✖".bold().red(), err);
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
