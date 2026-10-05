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
        Some(cli::Commands::Ping { target: _ }) => {
            println!("Ping");
            Ok(())
        }
        Some(cli::Commands::Browse { target: _ }) => {
            println!("Browse");
            Ok(())
        }
        Some(cli::Commands::Ssh {
            target: _,
            apply: _,
        }) => todo!(),
        Some(cli::Commands::Wake { target: _ }) => todo!(),
        Some(cli::Commands::PowerOff { target: _ }) => todo!(),
        Some(cli::Commands::List {}) => todo!(),
        None => {
            println!("None");
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("{} {}", "✖".bold().red(), err);
        std::process::exit(1)
    }
}
