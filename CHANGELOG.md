# Changelog

All notable changes to okiro are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/) and the project adheres
to [Semantic Versioning](https://semver.org/) — pre-1.0 versions may
break compatibility freely.

## [0.2.0] - 2026-10-05

Initial MVP. The architecture is in place so the 0.x → 1.0 work is
implementation, not refactoring. Only `okiro wake` is implemented; the
remaining commands are stubs that lock the public API surface.

### Added

- Workspace split into `okiro-core` (library) and `okiro` (binary).
- `okiro_core::Config::load(path)` / `from_toml_str` parsing
  `[[hosts]]` tables with required `name` and `mac` plus optional
  `host`, `ssh_user`, `ssh_port`, `poweroff_cmd`.
- `okiro_core::default_path()` for cross-platform config resolution
  via the `dirs` crate (Linux XDG, macOS, Windows AppData).
- Trait-based I/O: `WolSender`, `Pinger`, `SshRunner`, `BrowserOpener`.
- `MockWolSender` test fake.
- `okiro wake <name>` implemented end-to-end: sends a 102-byte magic
  packet (6×0xFF + 16×MAC) to `255.255.255.255:9` via a `socket2`
  UDP socket with `SO_BROADCAST`.
- Stub commands (`ping`, `browse`, `ssh`, `poweroff`, `status`,
  `list`) return `CoreError::Unimplemented` with locked signatures.
- `OKIRO_CONFIG` environment-variable override for the config path.
- GitHub Actions CI matrix on `ubuntu-latest`, `macos-latest`,
  `windows-latest` running `cargo fmt`, `cargo clippy -D warnings`,
  `cargo test`, `cargo build --release`.
- Repo hygiene: `.editorconfig`, `rustfmt.toml`, `clippy.toml`,
  `.gitignore`, `LICENSE-MIT`.
- 8 unit tests + integration tests (magic-packet layout, MAC parsing,
  duplicate detection, resolve, default_path, CLI smoke with mock).

### Changed

- Workspace now uses `[workspace.package]` and `[workspace.dependencies]`
  to keep crate metadata in sync.

### Deferred to 1.0

- `okiro ping`, `browse`, `ssh`, `poweroff`, `status` implementations.
- `--config <path>` global flag, shell completions subcommand,
  `--json` output.
- System packages (AUR, deb, Homebrew tap).
- `cargo-dist` GitHub releases; crates.io publish.

## [0.1.0] - 2025-10-04

Initial skeleton: clap CLI scaffolding, empty `config.rs`,
`todo!()` arms for wake/ssh/poweroff/list, MIT license, README.

[0.2.0]: https://github.com/eRgo35/okiro/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eRgo35/okiro/releases/tag/v0.1.0