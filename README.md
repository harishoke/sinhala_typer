# sinhala_typer

Lightweight, low-resource Windows 10/11 Sinhala phonetic (Singlish)
typing utility in Rust. Global keyboard hook converts Singlish to Sinhala Unicode in
real time, with an optional real-time legacy-font (FM Abhaya) mode for apps like
MS Office. Minimal system tray UI plus an on-demand Settings & Info window.


## Highlights

- **Case-sensitive phonetic engine** - `t`/`T`, `d`/`D`, `sh`/`Sh`, `Ru` and `H`
  are different keys, typed exactly as you press them (Shift and Caps Lock aware).
- **Shortcuts stay shortcuts** - Ctrl / Alt / Win combinations (copy, paste,
  save, undo...) always pass through untouched; only the configured mode
  hotkey is intercepted.
- **Single instance** - a second launch exits instead of double-converting
  every keystroke.
- **Forgiving internals** - poison-safe shared state, atomic config saves
  (a crash cannot truncate your settings), and a tray icon that shows the
  current mode at a glance (amber dot = FM Abhaya, slate dot = English,
  grey = off).

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
- Graceful degradation: if no GPU/OpenGL renderer is available (e.g. RDP or
  headless sessions), the app logs the error and keeps running tray-only so
  typing still works; a wgpu fallback renderer can be selected with
  `SINHALA_TYPER_RENDERER=wgpu`

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
- `assets/` — the app icon, stored as base64 text
  (`sinhala_typer.ico.base64.a`/`.b`) so the repository stays text-only: `build.rs`
  decodes it and embeds it into the .exe, and CI materialises the `.ico`
  for the installer. The tray/window artwork lives as generated RGBA
  tables in `src/ui/tray_art.rs` (amber dot = FM Abhaya, slate dot =
  English, grey = off).

## Build & install

Continuous integration (`.github/workflows/build.yml`) runs the engine test
suite, builds the optimized Windows binary, smoke-tests it, and produces a
signed-ready Inno Setup installer (`dist/sinhala_typer-setup-<version>.exe`):

- every push/PR: binaries + installer are uploaded as workflow artifacts
- every tag `v*`: a draft GitHub Release is created with the portable `.exe`
  and the setup installer attached

To build locally on Windows: `cargo build --release` produces the binary and
`iscc installer/sinhala_typer.iss` (Inno Setup 6) builds the installer.

- `cargo check` — fast compile check
- `cargo test` — engine unit tests (runs on any OS)
- `cargo run` — run the app (Windows)
- `cargo build --release` — optimized binary in `target/release/`

## Resource targets

Background tray process: ~10-15 MB RAM, ~0% idle CPU, tiny release binary
(`opt-level = "z"`, LTO, stripped).

## Credits

Created by **Ashen Nethsara** —
[LinkedIn](https://www.linkedin.com/in/ashennethsara/) ·
[GitHub](https://github.com/AshenNethsara/)
