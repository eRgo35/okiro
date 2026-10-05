# okiro (起きろ)

A small command-line tool for remote computer wakeup, management, and shutdown.

起きろ (Japanese for "wake up") provides simple commands to wake machines via Wake-on-LAN, check reachability, open remote web interfaces, SSH into hosts, and perform remote shutdowns — all driven from a small, configurable command-line client.

## Status: Pre-1.0 MVP (0.2.0)

This release is the **architectural MVP**. Only `okiro wake` is fully
implemented. The remaining commands (`ping`, `browse`, `ssh`,
`poweroff`, `status`, `list`) are stubs that exist to lock the public
library API surface (`okiro-core`). They will be filled in during
the 0.x → 1.0 workstream — see
`docs/superpowers/specs/2026-10-05-okiro-mvp-plan.md` for the roadmap.

What's intentionally **not** implemented yet:

- `okiro ping <name>`
- `okiro browse <name>`
- `okiro ssh <name>`
- `okiro poweroff <name>`
- `okiro status`
- `okiro list`
- `--config <path>` global flag
- Shell completions subcommand
- `--json` output flag
- System packages (AUR, deb, Homebrew tap)
- crates.io / GitHub release artifacts

## Features (in 0.2.0)

- Wake machines via Wake-on-LAN (`okiro wake <name>`).
- Configurable hosts via `okiro.toml` (name + MAC + optional host
  fields).

## Installation

Build from source (Rust 1.85+, edition 2024):

```bash
cargo build --release
```

The binary lands at `target/release/okiro`.

## Configuration

Default config path is resolved via the `dirs` crate:

| Platform | Path                                                              |
|----------|-------------------------------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME/okiro/okiro.toml` (default `~/.config/okiro/okiro.toml`) |
| macOS    | `~/Library/Application Support/okiro/okiro.toml`                  |
| Windows  | `%APPDATA%\okiro\okiro.toml`                                      |

Override the location for one invocation by setting the `OKIRO_CONFIG`
environment variable to an explicit file path. This is the same
mechanism the test suite uses to point at a tmpdir config.

Example `okiro.toml`:

```toml
[[hosts]]
name = "laptop"
mac  = "aa:bb:cc:dd:ee:ff"
host = "laptop.example.local"
ssh_user = "mike"
ssh_port = 22
poweroff_cmd = "sudo shutdown -h now"
```

Only `name` and `mac` are required. The remaining fields are parsed
but currently unused by `okiro wake`; they exist so that the future
1.0 commands can read them without a config migration.

## Commands

```txt
$ okiro --help
okiro (起きろ) — remote computer wakeup / management / shutdown tool

Usage: okiro <COMMAND>

Commands:
  wake       Send Wake-on-LAN to a configured host (implemented)
  ping       Ping a host (stub)
  browse     Open remote web dashboard (stub)
  ssh        Open an SSH session (stub)
  poweroff   Request remote shutdown (stub)
  status     Show host status (stub)
  list       List configured hosts (stub)
```

Working example:

```sh
okiro wake laptop
```

## Project layout

This is a Cargo workspace:

- `crates/okiro-core/` — library: config parsing, host resolution,
  trait-based I/O (`WolSender`, `Pinger`, `SshRunner`,
  `BrowserOpener`).
- `crates/okiro/` — binary: clap CLI, calls into `okiro-core`.

## License

MIT. See [`LICENSE-MIT`](LICENSE-MIT).