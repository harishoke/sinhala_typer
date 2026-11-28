# Sinhala Typer 3.0.0 (beta)

⚠️ **Beta / pre-release** — this build will be promoted to Latest only after it is confirmed working on a real Windows machine.

v2.0.0 was published without ever running on a real machine, and it did not work. 3.0.0 rebuilds the TSF text service defensively and adds the diagnostics that were missing.

## What changed

- **Crash safety:** the IME DLL no longer aborts on a panic (which would have crashed the app you were typing in). Every entry point now catches panics and logs them.
- **Registration:** the service registers the full TSF capability categories (immersive / Store-app support, language-bar support, secure mode), not just the keyboard category.
- **32-bit + 64-bit:** both DLL builds are included — 32-bit apps can now load the IME too.
- **Installer registers the keyboards** (it runs elevated): after setup, just add the two keyboards under Windows Settings → Language → Sinhala → Keyboards and switch with Win+Space. Settings can still unregister/re-register.
- **Diagnostics:** the DLL logs what it does to `%LOCALAPPDATA%\SinhalaTyper\tsf.log`. If anything fails, send that file with your report.
- **Keymap completed:** `Lu` = ඏ, `Luu` = ඐ (signs: `kLu` = කෟ, `kLuu` = කෳ) — the last rows of the adopted phonetic spec.

The keymap is the standard phonetic (ශබ්දානුකූල) layout: `tha` = ත, `dha`/`qa` = ද, `thha` = ථ, `dhha` = ධ, capitals for retroflex letters, `z` prefixes for naasikya letters, `ch` = ච, automatic ං (`sangha` = සංඝ). Full spec: `data/keymap.md`.

Made by Ashen Nethsara.
