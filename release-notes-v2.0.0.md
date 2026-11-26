### Rebuilt as a real Windows keyboard (IME)

2.0.0 is a ground-up rework, driven by 1.3.0 field reports:

- **Typing now happens inside a TSF text service** — the two keyboards **Sinhala Typer (Unicode)** and **Sinhala Typer (FM Abhaya)** are genuine Windows keyboards under the Sinhala language. Register them once in Settings → Windows keyboards, add them under Windows Settings → Language → Sinhala → Keyboards, and switch with **Win+Space**. Typing works in every app, even with the companion app closed.
- **No more startup flash, no more stuck Exit**: the companion app is tray-only now (Settings opens in its own process when you ask for it), so there is no window to blink at launch and Exit simply exits.
- Modes, the mode hotkey and the global keyboard hook are gone — the Windows keyboard you pick *is* the mode.

### New keymap (standard phonetic layout)

The layout now follows the standard ශබ්දානුකූල phonetic scheme: `tha` = ත, `dha`/`qa` = ද, `thha` = ථ, `dhha` = ධ, `Ta` = ඨ, `Da` = ඪ, `Na` = ණ, `La` = ළ, `Sa`/`Sha` = ෂ, `zga` = ඟ, `zja` = ඦ, `zda` = ඬ, `zdha` = ඳ, `zka` = ඤ, `zha` = ඥ, `Ba` = ඹ, `A` = ඇ, `Aa` = ඈ, `R` = ඍ, `Ru` = ඎ, `kru` = කෘ, `kruu` = කෲ, `kax` = කං, `kaX` = කඞ. `ch` still types ච, and the automatic ං in words like `sangha` → සංඝ and `punchi` → පුංචි is kept. Full table in the repo: `data/keymap.md`.

Created by **Ashen Nethsara** — [LinkedIn](https://www.linkedin.com/in/ashennethsara/) · [GitHub](https://github.com/AshenNethsara/)

### Downloads

- `sinhala_typer-setup-2.0.0.exe` — installer (includes the TSF keyboard DLL)
- `sinhala_typer.exe` — portable companion app
- `sinhala_typer_tsf.dll` — the keyboard DLL on its own (portable users: keep it next to the exe)

Built and tested by GitHub Actions on a Windows runner. The TSF IME is new — please report how it behaves on your machine.
