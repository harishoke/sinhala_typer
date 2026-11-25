# Changelog

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
