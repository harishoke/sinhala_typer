# Sinhala Typer 2.1.0

Fresh version of the clean-rewrite line (`docs/rewrite-plan.md`): the hardened
TSF keyboards from the 3.0.0 beta, a reset repo (MIT license, honest README,
CI on every push, one installer), and a keymap re-based onto the captured
Helakuru golden table (190 pairs, all covered by tests).

## What changed

- **Installer fixed:** upgrading over a running install no longer fails with
  "Error opening file for writing: sinhala_typer_tsf.dll". Locked files are
  renamed aside and the new build is written to the freed path; leftover
  `.old` files are removed on reboot.
- **Keymap = Helakuru golden table** (`data/golden_pairs.csv`, every row
  unit-tested): `ae`/`aae`, `RR`, `jh`, `gn`, `nj` added; `ru`/`ruu` are plain
  r + vowel; `R`/`RR` are the vocalic r; `dya` renders dha + yansaya.
- **Repo hygiene:** repo-level MIT LICENSE, CI (`fmt`/`clippy -D warnings`/
  tests) on every push, single NSIS installer, icon embedded from the
  committed `.ico` (no more base64 split parts).
- Keyboards (unchanged from 3.0.0 beta): **Sinhala Typer (Unicode)** and
  **Sinhala Typer (FM Abhaya)** as real TSF keyboards under Sinhala, x64 +
  x86 DLLs, panic-safe entry points, diagnostics in
  `%LOCALAPPDATA%\SinhalaTyper\tsf.log`.

## Download

- Installer: `sinhala_typer-setup-2.1.0.exe` (per-user; one admin prompt to
  register the keyboards)
- Portable: `sinhala_typer.exe`

If the keyboards do not appear in Windows Settings after install, send
`%LOCALAPPDATA%\SinhalaTyper\tsf.log`. Fallback stable: v1.2.0.

Made by Ashen Nethsara.
