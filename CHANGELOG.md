# Changelog

## 3.0.0 — 2026-11-28

v2.0.0 was published without ever running on a real machine, and field
reports said the IME did not work. 3.0.0 rebuilds the text service
defensively and adds the diagnostics that were missing. Released as a
pre-release (beta) until it is confirmed on a real Windows machine.

### Fixed / hardened — TSF text service
- **Crash safety:** the DLL no longer uses `panic = "abort"` (a panic
  would have killed the host application); every COM entry point and DLL
  export now catches panics and logs them instead.
- **Registration:** the service now registers the TSF capability
  categories (immersive/Store-app support, systray support, secure mode,
  UI-element, input-mode compartment, COM-less) in addition to the
  keyboard category.
- **32-bit + 64-bit:** both DLL builds ship; 32-bit apps can now load
  the IME. The installer (now elevated) registers both during setup;
  Settings can unregister/re-register both.
- **Diagnostics:** the DLL logs activation, registration and errors to
  `%LOCALAPPDATA%\SinhalaTyper\tsf.log`.

### Keymap
- Completed the adopted phonetic spec: `Lu` = ඏ and `Luu` = ඐ, with
  signs `kLu` = කෟ and `kLuu` = කෳ.

## 2.0.0 — 2026-11-26

A ground-up rework of how Sinhala Typer integrates with Windows, driven
by 1.3.0 field reports (the app flashed a window at launch and typing
behaved unreliably outside the tray app's own model).

### Changed — a real Windows IME
- Typing now happens **inside a TSF text service**, not a global
  keyboard hook. The two keyboards — **Sinhala Typer (Unicode)** and
  **Sinhala Typer (FM Abhaya)** — are genuine Windows keyboards under
  the Sinhala language: add them once in Windows Settings, switch with
  Win+Space, and type in any app. The companion exe no longer needs to
  be running to type.
- The companion app is now **tray-only** (Settings opens in its own
  process when asked). There is no hidden window to flash at startup,
  and tray Exit is a direct process exit — the v1.2.0/1.3.0 exit bug
  class is gone by construction.
- Modes, the mode hotkey, and per-app FM switching are gone: the
  Windows keyboard you pick *is* the mode.

### Changed — keymap
- The phonetic layout now follows the standard ශබ්දානුකූල (phonetic)
  scheme the owner adopted (see `data/keymap.md`): `tha` = ත,
  `dha`/`qa` = ද, `thha` = ථ, `dhha` = ධ, `Ta` = ඨ, `Da` = ඪ,
  capitals for retroflex letters (`Na` = ණ, `La` = ළ, `Sa`/`Sha` = ෂ),
  `z` prefixes for the naasikya letters (`zga` = ඟ, `zja` = ඦ,
  `zda` = ඬ, `zdha` = ඳ, `zka` = ඤ, `zha` = ඥ), `Ba` = ඹ, `A` = ඇ,
  `Aa` = ඈ, `R` = ඍ, `Ru` = ඎ, `kru` = කෘ, `kruu` = කෲ, `kax` = කං,
  `kaX` = කඞ. `ch` still types ච (owner decision). The convenient
  automatic ං before velar/palatal consonants is kept
  (`sangha` → සංඝ, `punchi` → පුංචි).

### Engineering
- The conversion engines moved into a shared crate (`engine/`) used by
  both the app and the TSF service, with the full spec table covered by
  unit tests.
- Builds and releases are produced by GitHub Actions on a Windows
  runner (`.github/workflows/build.yml`).


## 1.3.0 — 2026-11-26

### Fixed (reported against 1.2.0)
- **Exit works again.** The close-to-tray handler cancelled *every* window
  close, including the one from the tray menu's Exit item, so the app
  could never quit. Exit (and the updater's hand-over) now set an explicit
  flag and the process really terminates.
- **Silent start, enforced.** The Settings window could still surface at
  launch on some systems. The window now re-asserts its hidden state on
  every frame until the user explicitly opens Settings from the tray
  (menu item or double-click) — launch is always tray-only.

### New
- **Windows keyboard integration (TSF).** A companion text-service DLL
  (`sinhala_typer_tsf.dll`, source in `tsf/`) registers two keyboards
  under the Sinhala language — **Sinhala Typer (Unicode)** and
  **Sinhala Typer (FM Abhaya)** — addable in Windows Settings → Language
  → Sinhala → Keyboards. Switching with Win+Space / the language bar
  switches the app mode to match. Opt-in from Settings → "Windows
  keyboards" (one admin prompt for the one-time registration). First
  release of this component: not yet smoke-tested on a real machine.
- **In-app updates.** Settings → "Updates" checks GitHub Releases;
  "Download & install" fetches the new installer, launches it and exits
  cleanly so the exe can be replaced. A silent check also runs once per
  launch (toast only, and only if notifications are enabled).

## 1.2.0

### Fixed
- **`ch` now types ච** (it produced ඡ before, so words like `punchi` came
  out wrong). The aspirated ඡ moved to `chh`. Plain `c` still gives ච.
- **Aspirated (mahaprana) letters and anusvara placement.** A plain `n`
  before an aspirated or palatal consonant now becomes anusvara as it
  should: `sangha` → සංඝ, `sancaara` → සංචාර, `punchi` → පුංචි (previously
  only before `k`, `g`, `h`).
- **Long vocalic vowels (දීර්ඝ) can be typed.** `Ruu` now gives ඎ, and after
  a consonant the long sign ෲ (`kRuu` → කෲ). Previously `Ruu` produced the
  broken sequence ඍඋ.
- **Punctuation in FM Abhaya mode.** In the FM font the ASCII punctuation
  slots draw Sinhala glyphs, so `,` `.` `?` and friends appeared as
  letters. FM mode now substitutes the character whose FM Abhaya glyph is
  the wanted mark (mapping inverted from the Pandukabhaya FM table).

### Added
- **Silent start.** The app now starts hidden in the system tray; the
  Settings window no longer appears at launch. Closing Settings hides it
  to the tray instead of quitting the app; reopen it from the tray icon's
  right-click menu ("Settings...") or by double-clicking the icon.
- **Optional mode-change notifications** (off by default): a tray-menu or
  Settings toggle shows a toast whenever the typing mode changes.
- **Windows language bar coexistence.** When the foreground window uses a
  Sinhala layout selected in Windows (Win+Space / language bar), Sinhala
  Typer pauses there automatically so keystrokes are not converted twice;
  switch the layout back to English and Sinhala Typer resumes. The mode
  hotkey still cycles English / Sinhala Unicode / Sinhala FM Abhaya at any
  time.
- **Refreshed Settings UI** with a header card, mode cards and an About
  section crediting the creator, Ashen Nethsara.

## 1.1.0

### Fixed
- **Case-sensitive typing now works.** The keyboard hook used to lower-case
  every letter, so keys that only exist in upper case (`T`, `D`, `N`, `L`,
  `Sh`, `Th`, `Dh`, `Ru`, `H`) could never be typed. The hook now sends the
  character actually produced, honouring Shift and Caps Lock.
- **Shortcuts are no longer eaten.** Ctrl / Alt / Win + key combinations
  (copy, paste, save, undo, ...) used to be converted as if they were text
  whenever Sinhala mode was on. They now pass through untouched; only the
  configured mode hotkey is intercepted.
- **Settings window no longer stalls typing.** It held the global config
  write lock for a whole frame; it now edits a snapshot and writes back
  only on change.
- **App switching is clean.** A half-typed word (and its FM Abhaya screen
  image) no longer leaks into the next window, the foreground-app cache
  was shortened from 5 s to 750 ms, and legacy-mode matching now compares
  whole process names (`xwinword.exe` no longer triggers Office mode) and
  is case-insensitive on the process side too.
- **Unicode injection is correct beyond the BMP** (`SendInput` now feeds
  UTF-16 code units instead of truncating chars to `u16`) and partial
  injections are retried once and reported instead of silently dropped.
- **Tray behaviour.** Any tray mouse event (including hover) used to open
  the Settings window; now only a double-click does. Tray icon, checks and
  tooltip are updated only when they actually change.
- **Config safety.** Saves are atomic (temp file + rename), a corrupt
  config is backed up to `settings.toml.invalid.bak` instead of being
  silently overwritten, and duplicate hotkey keys/modifiers are rejected.
- **Robustness.** A poisoned state lock no longer cascades into panics in
  the hook/UI threads, the hook guards against a null event pointer,
  generic Shift/Ctrl/Alt virtual keys are tracked, and
  `PhoneticEngine::convert` no longer panics on non-ASCII input.
- The config file is no longer rewritten on every launch, and the
  autostart preference is reconciled with the registry once at startup.

### Added
- **Single-instance guard** - a second launch exits cleanly instead of
  installing a second hook and converting every keystroke twice.
- **New icon set** - a designed app icon (the Sinhala letter අ on a
  blue-to-teal tile) as a multi-size `.ico` (16-256 px), embedded into the
  `.exe` via `build.rs`/winres and used by the installer, plus per-mode
  tray artwork: amber status dot for FM Abhaya, slate dot for English,
  greyed tile when typing is off. The settings window shows the icon too.
  Assets are stored text-only (`.ico` as base64, tray RGBA as a generated
  Rust table) so the repository can be mirrored through text-only tools.
- CI now enforces `cargo fmt --check` and `cargo clippy -- -D warnings`;
  the codebase was formatted and the pre-existing clippy warnings fixed.
- Tests grew from 24 to 28 (hotkey normalisation/duplicates, process-name
  matching, non-ASCII conversion safety).

### Changed
- Version bumped to 1.1.0; the installer defaults to the same version
  (it previously fell back to 0.1.0) and carries a real publisher name
  and support URL.
- The phonetic keymap data file was renamed to `data/keymap.md`;
  the mapping tables themselves are unchanged.
- Release builds no longer open a console window (`windows_subsystem`).
