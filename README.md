<div align="center">

<img src="assets/icon.png" width="120" alt="Sinhala Typer icon">

# Sinhala Typer

### Type Singlish. Get Sinhala. Anywhere on Windows.

**සිංග්ලිෂ් වලින් type කරන්න — සිංහල Unicode එවෙලේම.**

[![Latest release](https://img.shields.io/github/v/release/harishoke/sinhala_typer?style=for-the-badge)](https://github.com/harishoke/sinhala_typer/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/harishoke/sinhala_typer/total?style=for-the-badge)](https://github.com/harishoke/sinhala_typer/releases)
![Windows 10 | 11](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D6?style=for-the-badge&logo=windows)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-dea584?style=for-the-badge&logo=rust)

</div>

---

## ⬇️ Download

| | |
| :--- | :--- |
| **Installer (recommended)** | [sinhala_typer-setup-1.2.0.exe](https://github.com/harishoke/sinhala_typer/releases/download/v1.2.0/sinhala_typer-setup-1.2.0.exe) — per-user install, no admin needed |
| **Portable** | [sinhala_typer.exe](https://github.com/harishoke/sinhala_typer/releases/download/v1.2.0/sinhala_typer.exe) — single file, just run it |

The app starts **silently in the system tray**. Right-click the tray icon for modes and **Settings...**, or double-click it. Press **Ctrl+Alt+S** anywhere to cycle modes.

## ✨ Features

- **Real-time phonetic conversion** — type `ammaa` and watch අම්මා appear, letter by letter, in any app.
- **Three modes** — English (pass-through) · Sinhala (Unicode) · Sinhala (FM Abhaya) for legacy-font MS Office documents.
- **Smart about your shortcuts** — Ctrl / Alt / Win combos (copy, paste, save…) always pass through untouched.
- **Works *with* the Windows language bar** — if a window is on a Sinhala layout you picked in Windows (Win+Space), Sinhala Typer pauses there automatically instead of converting twice.
- **FM Abhaya done right** — correct legacy rendering in Word/Excel/PowerPoint (auto-detected), including real punctuation, which the FM font normally turns into letters.
- **Case-sensitive keys** — `t`/`T`, `d`/`D`, `sh`/`Sh`, `Ru`/`Ruu`, `H` are different letters, honoring Shift and Caps Lock.
- **Featherweight** — ~10–15 MB RAM, ~0% idle CPU, single instance, starts with Windows if you want it to.
- **Optional mode-change notifications** and a clean, dark Settings UI.

## ⌨️ How to type

Just type the Singlish spelling — the engine handles conjuncts (ක්‍ර, ද්‍ය, ර්‍ම), anusvara and long vowels for you:

| You type | You get | | You type | You get |
| :--- | :--- | :--- | :--- | :--- |
| `ammaa` | අම්මා | | `sangha` | සංඝ |
| `lankaava` | ලංකාව | | `punchi` | පුංචි |
| `krama` | ක්‍රම | | `vidhyaava` | විද්‍යාව |
| `kohomadha` | කොහොමද | | `aayuboowan` | ආයුබෝවන් |

Handy keys: `ch` = ච · `chh` = ඡ · `kh` = ඛ · `gh` = ඝ · `Th` = ථ · `Dh` = ධ · `sh` = ශ · `Sh` = ෂ · `Ru` = ඍ · `Ruu` = ඎ · `x` = ං · `H` = ඃ. Full table: [data/keymap.md](data/keymap.md).

## 🔧 Under the hood

The pure-Rust engine core (`src/engine/`) is platform-independent and fully tested (30 unit tests): `phonetic.rs` (Singlish → Sinhala Unicode, streaming with smart backspace) and `legacy.rs` (Unicode → FM Abhaya with `render_diff` screen tracking). The Windows-only modules (`src/win/`, `src/ui/`) provide:

- Global low-level keyboard hook (`WH_KEYBOARD_LL`) with an injection guard
- Real-time conversion via `SendInput`, foreground-app detection for Office, and Sinhala-layout detection for language-bar coexistence
- System tray menu + settings window (eframe/egui); config in `%APPDATA%\sinhala_typer\settings.toml` (atomic saves; a corrupt config is backed up, never silently wiped)
- Graceful degradation: no GPU/OpenGL (RDP, headless)? The app keeps running tray-only so typing still works; a wgpu fallback can be selected with `SINHALA_TYPER_RENDERER=wgpu`

## 📁 Layout

- `src/engine/` — typing engines and shared types (`Mode`, `EditAction`)
- `src/win/` — keyboard hook, synthesized input, foreground/layout detection, autostart, notifications
- `src/ui/` — tray icon/menu, settings window
- `src/config.rs`, `src/state.rs` — settings/hotkey parsing and per-process mode selection (tested anywhere)
- `data/` — verified linguistic data (keymap, FM Abhaya table, test pairs)
- `assets/` — app icon; the `.ico` is stored as base64 text (`sinhala_typer.ico.base64.a`/`.b`) so the repo stays text-only — `build.rs` decodes and embeds it, and tray/window artwork lives as generated RGBA tables in `src/ui/`

## 🛠️ Build

```sh
cargo test            # engine unit tests (any OS)
cargo build --release # optimized binary (Windows)
```

CI (`.github/workflows/build.yml`) runs the test suite, builds the Windows binary and produces the Inno Setup installer (`installer/sinhala_typer.iss`); tags `v*` create a draft release with both downloads attached. Release profile: `opt-level = "z"`, LTO, stripped.

## 👤 Credits

Created by **Ashen Nethsara** —
[LinkedIn](https://www.linkedin.com/in/ashennethsara/) ·
[GitHub](https://github.com/AshenNethsara/)

FM Abhaya mapping data derives from the MIT-licensed [Pandukabhaya](https://github.com/akuruAI/Pandukabhaya) project. The FM Abhaya *font* itself is proprietary — install your own licensed copy to use legacy mode.
