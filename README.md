# sinhala_typer

Lightweight, low-resource Windows 10/11 Sinhala phonetic (Helakuru-style Singlish)
typing utility in Rust. Global keyboard hook converts Singlish to Sinhala Unicode in
real time, with an optional real-time legacy-font (FM Abhaya) mode for apps like
MS Office. Minimal system tray UI plus an on-demand Settings & Info window.

## Status

The app is feature-complete for a first usable release. The pure-Rust engine core
(`src/engine/`) is platform-independent and fully tested anywhere: `phonetic.rs`
(Singlish -> Sinhala Unicode, streaming with smart backspace) and `legacy.rs`
(Unicode -> FM Abhaya + `render_diff`). The Windows-only modules (`src/win/`,
`src/ui/`) are compiled only on Windows (`cfg(target_os = "windows")`) and provide:

- Global low-level keyboard hook (`WH_KEYBOARD_LL`) with injection guard
- Real-time Singlish -> Sinhala conversion via `SendInput` (unicode + legacy
  FM Abhaya rendering using `render_diff` screen tracking)
- Configurable mode hotkey (default `ctrl+alt+s`, cycles English/Unicode/FM Abhaya)
- Foreground-app detection to auto-switch to FM Abhaya in MS Office apps
- System tray menu + settings window (eframe/egui), config persisted to TOML
- Optional autostart via the `Run` registry key

Settings live in `%APPDATA%\sinhala_typer\settings.toml`.

## Layout

- `src/engine/` — pure-Rust typing engines: `phonetic.rs` (Singlish -> Unicode),
  `legacy.rs` (Unicode -> FM Abhaya + `render_diff`), `mod.rs` (shared types:
  `Mode`, `EditAction`, test-only `ScreenSim`)
- `src/win/` — Windows-only: keyboard hook, synthesized input, foreground-app
  detection, autostart registry
- `src/ui/` — Windows-only: tray icon/menu, settings window
- `src/config.rs` — cross-platform settings/hotkey parsing (tested anywhere)
- `src/state.rs` — shared app state and per-process mode selection (tested anywhere)
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
