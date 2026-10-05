use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use colorize::AnsiColor;

use okiro_core::commands;
use okiro_core::{Config, WolSender};

mod cli;
mod io;

/// Phase 1: hardcoded config path. Phase 3 will replace this with
/// `Config::default_path()` via the `dirs` crate.
fn config_path() -> PathBuf {
    if let Ok(p) = std::env::var("OKIRO_CONFIG") {
        return PathBuf::from(p);
    }
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".config").join("okiro").join("okiro.toml")
}

/// Inner entry point that the integration test drives directly.
pub fn run_with(
    cli: cli::Cli,
    cfg_path: &Path,
    wol: &dyn WolSender,
) -> Result<(), Box<dyn std::error::Error>> {
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
            let cfg = Config::load(cfg_path)?;
            commands::wake(&target, &cfg, wol)?;
        }
        Some(cli::Commands::PowerOff { target: _ }) => {
            println!("PowerOff");
        }
        Some(cli::Commands::List {}) => {
            let cfg = Config::load(cfg_path)?;
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

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    run_with(cli, &config_path(), &io::UdpWolSender)
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("{} {}", "✖".bold().red(), err);
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use std::sync::Arc;

    use clap::Parser;
    use okiro_core::io::{MockWolSender, WolSender};
    use parking_lot::Mutex;

    use super::*;

    /// Test double that shares state across `send` calls.
    #[derive(Clone, Default)]
    struct SharedWol {
        inner: Arc<Mutex<MockWolSender>>,
    }

    impl WolSender for SharedWol {
        fn send(&self, mac: okiro_core::host::MacAddr) -> Result<(), okiro_core::io::WolError> {
            self.inner.lock().send(mac)
        }
    }

    fn write_config(dir: &Path, body: &str) -> PathBuf {
        let path = dir.join("okiro.toml");
        std::fs::write(&path, body).expect("write config");
        path
    }

    fn unique_tmp(label: &str) -> PathBuf {
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("okiro-test-{label}-{pid}-{nanos}"))
    }

    #[test]
    fn wake_sends_to_resolved_mac() {
        let dir = unique_tmp("wake-sends");
        std::fs::create_dir_all(&dir).unwrap();
        let cfg_path = write_config(
            &dir,
            r#"
                [[hosts]]
                name = "laptop"
                mac  = "aa:bb:cc:dd:ee:ff"
            "#,
        );

        let wol = SharedWol::default();
        let cli = cli::Cli::parse_from(["okiro", "wake", "laptop"]);
        run_with(cli, &cfg_path, &wol).expect("wake ok");

        let recorded = wol.inner.lock().sent.lock().expect("wake recorded a MAC");
        assert_eq!(recorded.to_string(), "AA:BB:CC:DD:EE:FF");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn wake_unknown_host_errors() {
        let dir = unique_tmp("wake-unknown");
        std::fs::create_dir_all(&dir).unwrap();
        let cfg_path = write_config(
            &dir,
            r#"
                [[hosts]]
                name = "laptop"
                mac  = "aa:bb:cc:dd:ee:ff"
            "#,
        );

        let wol = SharedWol::default();
        let cli = cli::Cli::parse_from(["okiro", "wake", "no-such-host"]);
        let err = run_with(cli, &cfg_path, &wol).expect_err("should fail");
        assert!(err.to_string().contains("no-such-host"));

        assert!(
            wol.inner.lock().sent.lock().is_none(),
            "WOL must not be sent"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
