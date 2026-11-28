<div align="center">

# Sinhala Typer

### Type Singlish. Get Sinhala. Anywhere on Windows.

**Type in English letters — get perfect Unicode Sinhala (type `ammaa`, get the Sinhala word for "mother").**

[![Latest stable release](https://img.shields.io/github/v/release/harishoke/sinhala_typer?style=for-the-badge)](https://github.com/harishoke/sinhala_typer/releases/latest)
![Windows 10 | 11](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D6?style=for-the-badge&logo=windows)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-dea584?style=for-the-badge&logo=rust)
![License: MIT](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

</div>

---

## ⚠️ Current status (read this first)

- **v2.1.0 is the current release** (clean-rewrite line, see [docs/rewrite-plan.md](docs/rewrite-plan.md)): real TSF keyboards + Helakuru golden keymap + fixed installer.
- **v1.2.0** is the fallback stable of the old v1 architecture (tray app + keyboard hook).
- v2.0.0 was **known broken** (keyboards did not appear in Windows Settings) and is marked deprecated; the 3.0.0 beta was never confirmed on real hardware and is superseded by v2.1.0.

## ⬇️ Download

| | |
| :--- | :--- |
| **Installer (current, v2.1.0)** | [sinhala_typer-setup-2.1.0.exe](https://github.com/harishoke/sinhala_typer/releases/download/v2.1.0/sinhala_typer-setup-2.1.0.exe) — per-user install, one admin prompt to register the keyboards |
| **Portable (v2.1.0)** | [sinhala_typer.exe](https://github.com/harishoke/sinhala_typer/releases/download/v2.1.0/sinhala_typer.exe) — single file, just run it |
| **Fallback stable (v1.2.0, old architecture)** | [sinhala_typer-setup-1.2.0.exe](https://github.com/harishoke/sinhala_typer/releases/download/v1.2.0/sinhala_typer-setup-1.2.0.exe) |

## ✨ Features

- **Real Windows keyboards (v2+/v3)** — *Sinhala Typer (Unicode)* and *Sinhala Typer (FM Abhaya)* are genuine TSF keyboards under the Sinhala language: add them in Windows Settings → Time & language → Language → **Sinhala** → Keyboards, then switch with **Win+Space**. Typing works in every app, even with the companion app closed.
- **Real-time phonetic conversion** — type `ammaa` and watch the Sinhala word appear letter by letter as a proper IME composition you can backspace through.
- **FM Abhaya legacy mode** — same typing, output encoded for the FM Abhaya font (font not bundled; it is proprietary — set the document font to FM Abhaya in MS Office).
- **Smart about shortcuts** — Ctrl / Alt / Win combos always pass through untouched.
- **Tray companion app** — settings, keyboard registration, autostart, notifications, and in-app updates (checks GitHub Releases). Config lives in `%APPDATA%\sinhala_typer\settings.toml`.
- Keymap spec: [data/keymap.md](data/keymap.md) · verified typing pairs: [data/verified_pairs.md](data/verified_pairs.md) · Helakuru golden table (rewrite source of truth): [data/golden_pairs.csv](data/golden_pairs.csv)

Example (current keymap): `ammaa` → U+0D85 U+0DB8 U+0DCA U+0DB8 U+0DCF · `sangha` → U+0DC3 U+0D82 U+0D9D · `krama` → U+0D9A U+0DCA U+200D U+0DBB U+0DC3 · `kohomadha` → U+0D9A U+0DDC U+0DC4 U+0DDC U+0DB8 U+0DAF.

## 🗂 Layout

- `engine/` — shared, platform-independent Rust crate: `phonetic.rs` (Singlish → Sinhala Unicode), `legacy.rs` (Unicode → FM Abhaya). Unit-tested, incl. `tests/golden.rs` generated from the golden CSV.
- `tsf/` — TSF text-service DLL (the two Windows keyboards), used by v2+/v3.
- `src/` — companion tray app (v2+/v3): `src/win/` autostart, notifications, tray pump, TSF registration, in-app updates; `src/ui/` tray + settings window.
- `data/` — linguistic data (keymap spec, verified pairs, FM Abhaya table, Helakuru golden pairs).
- `installer/sinhala_typer.nsi` — the one installer script (NSIS, per-user, elevated only to register the TSF service).
- `assets/` — app icon (`.ico`, embedded into the exe by `build.rs`).
- `.github/workflows/` — `ci.yml` (fmt/clippy/test on every push) and `build.yml` (release builds on tags / manual dispatch, Windows runner).

## 🛠 Build

```sh
cargo test -p sinhala_engine                              # engine tests incl. golden table (any OS)
cargo test                                                # app tests (full suite on Windows; subset on other OSes)
cargo fmt --all --check && cargo clippy --all-targets -- -D warnings
cargo build --release                                     # companion app (Windows)
cargo build --release --manifest-path tsf/Cargo.toml      # TSF service DLL, x64 (Windows)
cargo build --release --manifest-path tsf/Cargo.toml --target i686-pc-windows-msvc   # 32-bit DLL
```

Releases are built by GitHub Actions on a Windows runner (`.github/workflows/build.yml`): tests, both DLLs (x64 + x86), the companion exe, the NSIS installer, and the GitHub release. CI lint/test runs on every push (`.github/workflows/ci.yml`).

## 🤝 Credits

Created by **Ashen Nethsara** —
[LinkedIn](https://www.linkedin.com/in/ashennethsara/) ·
[GitHub](https://github.com/AshenNethsara/)

FM Abhaya mapping data derives from the MIT-licensed [Pandukabhaya](https://github.com/akuruAI/Pandukabhaya) project. The FM Abhaya *font* itself is proprietary — install your own licensed copy to use legacy mode.

## 📜 License

[MIT](LICENSE) — © 2026 Ashen Nethsara.
