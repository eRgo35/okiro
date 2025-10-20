use clap::Parser;
use colorize::AnsiColor;

mod cli;
mod config;

fn main() {
    let cli = cli::Cli::parse();

    let result: Result<(), Box<dyn std::error::Error>> = match cli.command {
        Some(cli::Commands::Status {}) => {
            println!("Status");
            Ok(())
        }
        Some(cli::Commands::Ping { target }) => {
            println!("Ping");
            Ok(())
        }
        Some(cli::Commands::Browse { target }) => {
            println!("Browse");
            Ok(())
        }
        Some(cli::Commands::Ssh { target, apply }) => todo!(),
        Some(cli::Commands::Wake { target }) => todo!(),
        Some(cli::Commands::PowerOff { target }) => todo!(),
        Some(cli::Commands::List {}) => todo!(),
        None => {
            println!("None");
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("{} {}", "✖".bold().red(), err.to_string());
        std::process::exit(1)
    }
}
