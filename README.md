# okiro (起きろ)

A small command-line tool for remote computer wakeup, management, and shutdown.

起きろ (Japanese for "wake up") provides simple commands to wake machines via Wake-on-LAN, check reachability, open remote web interfaces, SSH into hosts, and perform remote shutdowns — all driven from a small, configurable command-line client.

## Features

- Wake machines with Wake-on-LAN (WOL).
- Check host reachability (ping).
- Open remote web dashboards in the default browser.
- Start SSH sessions to configured hosts.
- Request remote shutdown/poweroff.
- Query host status (last-seen, online/offline).
- Configurable hosts and settings via a user config file.

## Installation

Build from source (requires Rust/Cargo):

```bash
cargo build --release
```

Optional: create a system package or install via your distro's packaging tooling.

## Configuration

okiro reads a config file for named hosts (MAC, IP/hostname, optional SSH user/port). Default config path:

- ~/.config/okiro.toml

Example `okiro.toml`:

```toml
[[hosts]]
name = "laptop"
mac  = "aa:bb:cc:dd:ee:ff"
host = "laptop.example.local"
ssh_user = "mike"
ssh_port = 22
```

## Usage

Run `okiro --help` for full details. Example:

```txt
$ okiro --help
okiro - remote computer wakeup / management / shutdown tool

Usage: okiro [COMMAND]

Commands:
  wake       Send Wake-on-LAN to a configured host
  ping       Ping a host to check reachability
  ssh        Open an SSH session to a configured host
  browse     Open a host's web dashboard in the browser
  poweroff   Request shutdown of a host (via SSH or API)
  status     Show status for configured hosts
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Examples:

- Wake a host named "laptop":
  
  ```sh
  okiro wake laptop
  ```

- SSH into a host:
  
  ```sh
  okiro ssh laptop
  ```

- Open the host (web dashboard or file share):
  
  ```sh
  okiro browse server
  ```

- Ping a host to check if it's online:
  
  ```sh
  okiro ping server
  ```

- Power off a host (uses configured SSH if available):
  
  ```sh
  okiro poweroff desktop
  ```

- Show interactive status for all configured hosts:
  
  ```sh
  okiro status
  ```

## Contributing

Contributions welcome. Open issues or PRs for bug fixes, new features, or improvements to host configuration handling.

## License

MIT License. See LICENSE file for details.
