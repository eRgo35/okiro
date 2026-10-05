# okiro MVP (0.2.0) Implementation Plan

**Date:** 2026-10-05
**Target version:** 0.2.0
**Path to 1.0:** `docs/superpowers/specs/2026-10-05-okiro-1.0-plan.md`

## Goal

Stand up the architectural skeleton (workspace split, trait-based I/O,
cross-platform config path, CI matrix) so the eventual 1.0 release is
mechanical implementation work, not refactoring. Only `wake` ships as
a working command; everything else is stubbed but routed through
traits.

## Non-goals (deferred to 1.0)

- Implementing `ping`, `browse`, `ssh`, `poweroff`, `status`, `list`
  end-to-end (only `wake` works in MVP).
- `--config <path>` global flag.
- `completions` subcommand, `--json` output.
- System packages (AUR, deb, Homebrew).
- `cargo-dist` GitHub releases, crates.io publish.
- Polished docs site; CHANGELOG discipline beyond a single entry.
- 80% coverage target; only smoke tests for `wake`.

## Acceptance gate for 0.x.x.0

- [ ] Workspace split in place; `okiro-core` compiles as a library.
- [ ] Trait-based I/O exists for all four external interactions
      (`WolSender`, `SshRunner`, `Pinger`, `BrowserOpener`) with default
      impls.
- [ ] `okiro wake <name>` works end-to-end on Linux, macOS, Windows
      against a real machine (or validated via packet capture).
- [ ] CI matrix green on `ubuntu-latest`, `macos-latest`,
      `windows-latest` for `fmt`, `clippy -D warnings`, `test`,
      `release build`.
- [ ] README honestly states MVP/pre-1.0 status.
- [ ] Config path resolved via `dirs` crate (cross-platform defaults).
- [ ] Version bumped to `0.2.0`.

---

## Phase 0 — Repo hygiene (½ day)

- Add `LICENSE-MIT` text file (MIT is the declared license; current
  `LICENSE` is already MIT but standardize the filename).
- Add `.editorconfig`, `rustfmt.toml`, tighten `.gitignore`
  (`target/`, `.DS_Store`, IDE files).
- Add `clippy.toml` with `disallowed-methods = [{ path = "std::option::Option::unwrap", reason = "no unwrap in lib code" }]`
  and same for `expect`. Binaries and tests are exempt.
- Update `README.md`:
  - Status section: "Pre-1.0 MVP. Only `okiro wake` is implemented;
    remaining commands are stubs."
  - Remove `browse`, `poweroff`, `ping`, `ssh`, `status` examples or
    mark them explicitly as planned.
- **Done gate:** `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`
  clean on current code at `main`.

## Phase 1 — Workspace split (1 day)

- Convert root `Cargo.toml` to a virtual workspace:
  ```toml
  [workspace]
  resolver = "2"
  members = ["crates/okiro-core", "crates/okiro"]
  ```
- Bump root package metadata or remove the `[package]` table entirely
  (virtual workspaces don't carry one).
- Create `crates/okiro-core/`:
  - `Cargo.toml` with `name = "okiro-core"`, edition `2024`, deps:
    `thiserror`, `dirs`, `socket2`, `webbrowser`.
  - Modules:
    - `config.rs` — `Host`, `Config` types; `Config::load(path)`,
      `Config::default_path()` using `dirs`.
    - `host.rs` — `resolve(&str) -> Result<&Host, ResolveError>`
      (errors: missing name, duplicate names detected at load).
    - `error.rs` — `pub enum CoreError { Config(ConfigError),
      Io(io::Error), Host(HostError), Wol(WolError) }`.
    - `io/` — submodule defining traits + default impls:
      - `pub trait WolSender { fn send(&self, mac: MacAddr) -> Result<(), WolError>; }`
      - `pub trait SshRunner { fn run(&self, target: &SshTarget) -> Result<(), SshError>; }`
      - `pub trait Pinger { fn ping(&self, host: &str, timeout: Duration) -> Result<PingResult, PingError>; }`
      - `pub trait BrowserOpener { fn open(&self, url: &str) -> Result<(), BrowserError>; }`
    - Default impls: `UdpWolSender`, `CommandSshRunner`,
      `SystemPinger` (shells to `ping` for MVP — replace with
      `surge-ping` at 1.0), `WebBrowserOpener` (`webbrowser` crate).
- Create `crates/okiro/` (binary):
  - Move `src/cli.rs` → `crates/okiro/src/cli.rs`.
  - Move `src/main.rs` → `crates/okiro/src/main.rs`; update module
    paths.
  - Move `src/commands/*.rs` → `crates/okiro-core/src/<cmd>.rs` as
    public functions:
    - `pub fn wake(target: &str, cfg: &Config, wol: &impl WolSender) -> Result<(), CoreError>`
    - same shape for `ping`, `browse`, `ssh`, `poweroff`, `status`,
      `list`.
- **Done gate:** `cargo build -p okiro` succeeds; binary behavior
  matches the pre-split crate (commands may still `todo!()` but
  compile and route through `okiro-core`).

## Phase 2 — `wake` end-to-end (1–2 days)

- `Config::load` parses `[[hosts]]` entries with required `name`, `mac`
  and optional `host`, `ssh_user`, `ssh_port`, `poweroff_cmd`. Reject
  duplicate names and malformed MACs (`aa:bb:cc:dd:ee:ff` format).
- `MacAddr` newtype with `FromStr` impl.
- `UdpWolSender::send(mac)`:
  - Build 6× magic packet (0xFF followed by 16 repetitions of the 6-byte
    MAC).
  - Send via `socket2::Socket::new(DGRAM)` with `SO_BROADCAST` set, to
    `255.255.255.255:9`.
  - Return `WolError::Broadcast` if `setsockopt` fails (Windows can be
    picky here — fallback to `std::net::UdpSocket` if needed).
- CLI: replace `todo!()` arm for `Commands::Wake { target }` with:
  ```rust
  let cfg = Config::load(Config::default_path()?)?;
  okiro_core::wake(&target, &cfg, &UdpWolSender)?;
  ```
- Tests:
  - **Unit:** MAC parsing (valid, invalid length, invalid chars),
    `resolve` (hit/miss/duplicate detection), magic-packet byte
    sequence.
  - **CLI integration:** `assert_cmd` test with `MockWolSender` injected
    through a test-only constructor on `Config::load_from_str`. Assert
    success exit code and that `MockWolSender` recorded the expected
    MAC.
- **Done gate:** manual smoke against a real WOL-capable machine on
  at least one platform; or packet capture (`tcpdump`/`Wireshark`)
  proving the magic packet leaves the interface on Linux.

## Phase 3 — CI + cross-platform config (½–1 day)

- `.github/workflows/ci.yml`:
  - Trigger on `push` and `pull_request` to `main`.
  - Matrix: `ubuntu-latest`, `macos-latest`, `windows-latest`.
  - Steps: `actions/checkout`, `dtolnay/rust-toolchain@stable`,
    `Swatinem/rust-cache@v2`, then:
      - `cargo fmt --all -- --check`
      - `cargo clippy --all-targets -- -D warnings`
      - `cargo test --all`
      - `cargo build --release`
- `Config::default_path()` via `dirs`:
  - Linux: `~/.config/okiro/okiro.toml`
  - macOS: `~/Library/Application Support/okiro/okiro.toml`
  - Windows: `%APPDATA%\okiro\okiro.toml`
- Document the per-platform paths in `README.md` under "Configuration."
- **Done gate:** CI badge green on all three platforms.

## Phase 4 — MVP cut (½ day)

- Bump `okiro-core` and `okiro` versions to `0.2.0`.
- Update `Cargo.toml` `description` / `keywords` to reflect MVP scope
  (drop features that aren't implemented yet from `keywords` if it
  feels dishonest — keep `wake-on-lan`, `cli`).
- Add a one-line entry to `CHANGELOG.md`:
  > ## 0.2.0 (2026-10-05)
  > Initial MVP. Only `okiro wake` is implemented; remaining commands
  > are stubs. Workspace split into `okiro` (binary) and `okiro-core`
  > (library). Cross-platform config path resolution.
- Tag `v0.2.0`. No GitHub release artifacts, no crates.io publish.
- Verify `cargo install --path crates/okiro` works in a fresh dir.

---

## Risks & open questions

- **WOL on Windows via `socket2`:** if `SO_BROADCAST` fails, we
  fall back to `UdpSocket` — but `UdpSocket::send_to` to a broadcast
  address may still need `socket2` for SO_REUSEADDR on some
  configurations. Mitigation: Phase 2 includes a real-machine
  smoke; if Windows blocks, document it and defer Windows support
  to 1.0.
- **Config file path on macOS** (`~/Library/Application Support`) vs
  the README's current `~/.config` claim. Phase 3 fixes the doc to
  match code.
- **`SystemPinger` shells out to `ping`** — this is intentionally
  MVP-grade (cross-platform flags, parsing output). 1.0 swaps to
  `surge-ping` for purity. Tests for `ping` are deferred to 1.0.
- **CI time on Windows** is the slowest matrix leg; expect 4–6
  min total for the matrix.

## Verification checklist before tagging v0.2.0

- [ ] `cargo fmt --check` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo test --all` green
- [ ] CI green on all three OSes
- [ ] `okiro wake <name>` succeeds against a real machine on at
      least one platform
- [ ] README states MVP status honestly
- [ ] `LICENSE-MIT` file present
- [ ] `CHANGELOG.md` has 0.2.0 entry