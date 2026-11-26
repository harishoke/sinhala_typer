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
| **Installer (recommended)** | [sinhala_typer-setup-2.0.0.exe](https://github.com/harishoke/sinhala_typer/releases/download/v2.0.0/sinhala_typer-setup-2.0.0.exe) — per-user install, no admin needed |
| **Portable** | [sinhala_typer.exe](https://github.com/harishoke/sinhala_typer/releases/download/v2.0.0/sinhala_typer.exe) — single file, just run it |

**Setup (once):** install, open **Settings...** from the tray icon, click **Add the Windows keyboards** (one admin prompt), then add **Sinhala Typer (Unicode)** and/or **Sinhala Typer (FM Abhaya)** under Windows Settings → Time & language → Language → **Sinhala** → Keyboards. From then on, switch keyboards with **Win+Space** like any other language — typing works in every app, even with the companion app closed. The tray app only handles settings and updates.

## ✨ Features

- **Real Windows keyboards** — Sinhala Typer (Unicode) and Sinhala Typer (FM Abhaya) are genuine TSF keyboards under the Sinhala language. Conversion happens inside the text service, in the app you're typing in — no global hook, no companion process needed.
- **Real-time phonetic conversion** — type `ammaa` and watch අම්මා appear, letter by letter, as a proper IME composition you can backspace through.
- **The standard phonetic layout** — doubled vowels are long (`aa` = ආ), capitals give retroflex letters (`Ta` = ඨ, `Na` = ණ, `La` = ළ), `z` gives the naasikya letters (`zga` = ඟ), and the dental aspirates take a double h (`thha` = ථ, `dhha` = ධ). Full table: [data/keymap.md](data/keymap.md).
- **FM Abhaya done right** — the FM keyboard renders correct legacy glyphs in MS Office, including real punctuation, which the FM font normally turns into letters. (Set the document font to FM Abhaya.)
- **Smart about your shortcuts** — Ctrl / Alt / Win combos (copy, paste, save…) always pass through untouched.
- **In-app updates** — Settings → Updates checks for new versions and can download & install them in one click.
- **Featherweight companion** — the tray app uses a few MB of RAM, starts with Windows if you want it to, and is only needed for settings/updates.

## ⌨️ How to type

Just type the Singlish spelling — the engine handles conjuncts (ක්‍ර, ද්‍ය, ර්‍ම), anusvara and long vowels for you:

| You type | You get | | You type | You get |
| :--- | :--- | :--- | :--- | :--- |
| `ammaa` | අම්මා | | `sangha` | සංඝ |
| `lankaava` | ලංකාව | | `punchi` | පුංචි |
| `krama` | ක්‍රම | | `vidhyaava` | විද්‍යාව |
| `kohomadha` | කොහොමද | | `aayuboowan` | ආයුබෝවන් |

Handy keys: `ch` = ච · `chh` = ඡ · `kh` = ඛ · `gh` = ඝ · `tha` = ත · `thha` = ථ · `dha` = ද · `dhha` = ධ · `Ta` = ඨ · `Da` = ඪ · `sha` = ශ · `Sa`/`Sha` = ෂ · `R` = ඍ · `Ru` = ඎ · `kru` = කෘ · `kruu` = කෲ · `x` = ං · `H` = ඃ. Full table: [data/keymap.md](data/keymap.md).

## 🔧 Under the hood

- `engine/` — the shared, platform-independent Rust crate with the conversion engines: `phonetic.rs` (Singlish → Sinhala Unicode) and `legacy.rs` (Unicode → FM Abhaya). Fully unit-tested, including every row of the adopted phonetic spec.
- `tsf/` — the TSF text service (IME) DLL: advises a key-event sink, feeds keystrokes to the engine, and maintains a composition in the focused document. Registers the two keyboards under si-LK.
- `src/` — the companion tray app (settings window in a separate `--settings` process, autostart, notifications, TSF registration, in-app updates). Config lives in `%APPDATA%\sinhala_typer\settings.toml`.

## 📁 Layout

- `engine/` — shared typing engines (used by the app and the TSF service)
- `tsf/` — TSF text-service DLL (the two Windows keyboards)
- `src/win/` — autostart, notifications, tray message pump, TSF registration, in-app updates
- `src/ui/` — tray icon/menu, settings window
- `data/` — linguistic data (keymap spec, FM Abhaya table)
- `assets/` — app icon; the `.ico` is stored as base64 text (`sinhala_typer.ico.base64.a`/`.b`) so the repo stays text-only — `build.rs` decodes and embeds it

## 🛠️ Build

```sh
cargo test -p sinhala_engine   # engine unit tests (any OS)
cargo build --release          # companion app (Windows)
cargo build --release --manifest-path tsf/Cargo.toml   # TSF service (Windows)
```

Releases are built by GitHub Actions on a Windows runner (`.github/workflows/build.yml`): tests, both binaries, the NSIS installer (`installer/sinhala_typer.nsi`), and the GitHub release.

## 👤 Credits

Created by **Ashen Nethsara** —
[LinkedIn](https://www.linkedin.com/in/ashennethsara/) ·
[GitHub](https://github.com/AshenNethsara/)

FM Abhaya mapping data derives from the MIT-licensed [Pandukabhaya](https://github.com/akuruAI/Pandukabhaya) project. The FM Abhaya *font* itself is proprietary — install your own licensed copy to use legacy mode.
