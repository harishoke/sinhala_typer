# sinhala_typer

Lightweight, low-resource Windows 10/11 Sinhala phonetic (Helakuru-style Singlish)
typing utility in Rust. Global keyboard hook converts Singlish to Sinhala Unicode in
real time, with an optional real-time legacy-font (FM Abhaya) mode for apps like
MS Office. Minimal system tray UI plus an on-demand Settings & Info window.

## Status

Phases 0-4 done. The pure-Rust engine core (`src/engine/`) is platform-independent
and fully tested anywhere: `phonetic.rs` (Singlish -> Sinhala Unicode, streaming with
smart backspace) and `legacy.rs` (Unicode -> FM Abhaya + `render_diff`).
Windows-only modules (`src/win/`, `src/ui/`) are compiled only on Windows
(`cfg(target_os = "windows")`) and type-checked here with
`cargo check --target x86_64-pc-windows-msvc`:
`win/hook.rs` (WH_KEYBOARD_LL + SendInput, toggle hotkey, per-app modes, legacy
re-render via `render_diff`), `win/app_detect.rs` (WinEvent-cached foreground exe),
`win/startup.rs` (single-instance mutex, Run key), `config.rs` (TOML at
%APPDATA%\\sinhala_typer\\config.toml), `ui/tray.rs` (tray icon + menu, spawns
`--settings` child), `ui/settings.rs` (egui window).

## Layout

- `src/engine/` — pure-Rust typing engines: `phonetic.rs` (Singlish -> Unicode),
  `legacy.rs` (Unicode -> FM Abhaya + `render_diff`), `mod.rs` (shared types:
  `Mode`, `EditAction`, test-only `ScreenSim`)
- `src/win/` — Windows-only: keyboard hook, foreground-app detection, startup (TODO)
- `src/ui/` — Windows-only: tray icon/menu, settings window (TODO)
- `data/` — owner-verified linguistic data (keymap, FM Abhaya table, test pairs).
  These files are owned by the product owner; no AI may invent their contents.
- `assets/` — tray icons and bundled font (TODO, added on Windows)

## Commands

- `cargo check` — fast compile check
- `cargo test` — engine unit tests (runs on any OS)
- `cargo run` — run the app (Windows)
- `cargo build --release` — optimized binary in `target/release/`

## Resource targets

Background tray process: ~10-15 MB RAM, ~0% idle CPU, tiny release binary
(`opt-level = "z"`, LTO, stripped).
