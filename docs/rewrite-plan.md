# Sinhala Typer — Clean Rewrite Plan

> This document is a spec and work plan for an AI coding assistant (and for the human owner).
> Read it fully before writing any code. Follow the phases in order.

---

## 0. Rules for the AI assistant

1. **Do not guess the keymap.** The Singlish → Sinhala rules must follow the **Helakuru phonetic keyboard**. The only source of truth is `data/golden_pairs.csv` (see section 5). If that file is missing or incomplete, stop and ask the owner for it. Never invent mapping rules from memory.
2. **Work phase by phase.** Do not start a later phase until the current phase's exit criteria are met.
3. **Small, testable steps.** Every engine rule must be covered by a test generated from the golden table.
4. **One versioning scheme, one installer, no dead code.** If something is superseded, delete it.
5. When something in this document is marked **VERIFY**, check it against current Microsoft documentation before relying on it. Say so if you cannot verify.
6. Prefer boring, well-understood solutions over clever ones.

---

## 1. Goal

A typing tool for **Windows 10 and Windows 11** that:

- Adds **two keyboards under the Sinhala (`si-LK`) language** in Windows:
  1. **Sinhala Unicode (Phonetic)** — type Singlish, get Sinhala Unicode.
  2. **Sinhala Legacy (FM Abhaya)** — same typing, but outputs FM Abhaya legacy-encoded text (for MS Office documents that use the FM Abhaya font).
- Together with the normal English (`en-US`) keyboard, the user can press **Win + Space** to cycle: **English → Sinhala Unicode → Sinhala Legacy → English …**
- Works in every application, with **no companion app or background process required** for typing.
- Keymap follows **Helakuru phonetic** behaviour.

### Non-goals

- No voice typing, no word prediction, no theming.
- No SendKeys / simulated-typing / global keyboard hook approaches.
- No bundling of the FM Abhaya font (it is proprietary).
- No mobile or macOS/Linux support.

---

## 2. Background: why a rewrite

The existing repository (`harishoke/sinhala_typer`) went through four architectures in about four days (tray app with simulated typing → TSF DLL → hardened TSF → ...). Result:

- README links to a known-broken release; the "Latest" release is an old, different architecture.
- Two installer scripts (NSIS and a legacy Inno one), base64-split icon parts, stale PRs.
- Keyboards did not appear in Windows Settings in v2.0.0; the v3.0.0 fix has never been confirmed on a real machine.
- No repo-level license.
- Keymap was a custom "standard phonetic" layout, not Helakuru's.

The pure conversion engine (`engine/`) was the healthy part, but it must be **re-verified against the Helakuru golden table** before being reused.

---

## 3. Approach decision

| Approach | Pros | Cons |
|---|---|---|
| **A. TSF text service (own DLL)** | No dependency; real Windows keyboard | Complex: COM, x86/x64, registration, code signing, host-app compatibility |
| **B. Keyman keyboard (`.kmn`)** | Registration, Win+Space, Win10/11 handling done by Keyman; contextual rules are easy to express | User must install Keyman |
| **C. MSKLC / KbdEdit static layout** | Native and simple | Static mapping only; **cannot** do contextual Singlish (`kaa → කා`). Rejected. |

**Decision:**

- **Phase 2 builds a Keyman prototype first** to validate the keymap and the Win+Space flow quickly.
- **Phase 3 (TSF)** is only done if the owner wants a dependency-free product. If Keyman already satisfies the goal, Phase 3 is skipped.

---

## 4. Target repository layout

```
sinhala_typer/
├─ README.md
├─ LICENSE                      # MIT, repo level
├─ data/
│  ├─ golden_pairs.csv          # SOURCE OF TRUTH (from Helakuru)
│  └─ fm_abhaya_table.md        # Unicode -> FM Abhaya mapping (attribution required)
├─ engine/                      # pure Rust crate, no OS dependencies
│  ├─ src/
│  │  ├─ lib.rs
│  │  ├─ phonetic.rs            # Singlish -> Unicode
│  │  └─ legacy.rs              # Unicode -> FM Abhaya codes
│  └─ tests/golden.rs           # generated from data/golden_pairs.csv
├─ keyman/                      # Phase 2 prototype
│  ├─ sinhala_unicode.kmn
│  └─ sinhala_legacy.kmn
├─ tsf/                         # Phase 3 (optional): cdylib IME DLL
├─ installer/                   # ONE installer script only
└─ .github/workflows/ci.yml
```

---

## 5. The keymap: Helakuru golden table

No official Helakuru keymap document was found. Helakuru is a mobile keyboard (Android/iOS) with a smart phonetic layout and a Wijesekara layout. Therefore the mapping must be **captured by testing the real app**.

### 5.1 Capture process (owner does this)

1. On a phone with Helakuru (phonetic layout), type each test input and record the exact output.
2. Save to `data/golden_pairs.csv` with columns:

```csv
input,unicode_expected,legacy_expected,notes
ka,ක,,
kaa,කා,,long vowel
kra,ක්‍ර,,rakaransaya (ZWJ)
```

   `unicode_expected` is copied from Helakuru. `legacy_expected` is filled later from the FM Abhaya table.

3. Aim for about **300 pairs**, covering at least:
   - All vowels, short and long (`a aa i ii u uu`, ඍ ඎ, ඏ ඐ, ඓ ඖ)
   - Consonant families: `k kh g gh`, `ch j`, `t th d dh`, `T Th D Dh` (retroflex), `n N`, `p b m`, `y r l L v s S sh h f`
   - Aspirated / dental / retroflex distinctions (dental ත ථ ද ධ, retroflex ඨ ඪ ණ ළ, specials ඞ ඤ ඥ ෂ ශ)
   - Anusvara / visarga (ං ඃ) and any **automatic** anusvara rules
   - Rakaransaya, yansaya, repaya (`kra`, `kya`, `rka`) and other conjuncts (ZWJ usage)
   - Word-final hal consonants (`k`, `m`, `n` at word end)
   - Capital-letter behaviour
   - Real words and sentences (at least 50), e.g. `ammaa`, `sinhala`, `sangha`
   - Edge cases: digits, punctuation, spaces, repeated vowels, invalid sequences

### 5.2 Rules for using the table

- Every row becomes a test. **All rows must pass** before a phase is complete.
- If the table and any other document disagree, the **table wins**.
- Old rules from the previous repo that are not in the Helakuru table are **removed**.

---

## 6. Engine design (`engine/`, pure Rust)

### 6.1 Principles

- No Windows APIs, no I/O, no allocation surprises, **no panics** on any input (return a pass-through result instead).
- Deterministic and fully unit-testable on any OS.

### 6.2 Suggested API

```rust
pub enum Mode { Unicode, Legacy }

pub enum KeyResult {
    /// Not handled; the host should deliver the key normally.
    Pass,
    /// Replace the current in-progress syllable text with `composition`.
    Update { composition: String },
    /// Commit `text`, then continue with `new_composition` (may be empty).
    Commit { text: String, new_composition: String },
}

pub struct Engine { /* state */ }

impl Engine {
    pub fn new(mode: Mode) -> Self;
    pub fn key(&mut self, ch: char) -> KeyResult;   // printable key
    pub fn backspace(&mut self) -> KeyResult;
    pub fn reset(&mut self);                        // focus change, Esc, etc.
}
```

### 6.3 Algorithm

- Maintain a small **pending buffer** of Latin input for the current syllable.
- Use a **trie / longest-match** table for consonants, vowels, and conjunction rules.
- A consonant followed by a vowel yields consonant + vowel sign; a consonant followed by nothing yields consonant + hal (්) until a vowel arrives.
- Re-render the **whole current syllable** on each key (so the result is always consistent with the buffer).
- Space, punctuation, Enter, and focus loss **commit** the pending syllable.
- **Ctrl / Alt / Win** combinations always return `Pass`.

### 6.4 Composition instead of backspace emulation

In the IME, use a TSF **composition** (the underlined in-progress text). On each key, **replace the composition text**; on commit, end the composition. Do not emulate typing with fake backspaces: that breaks in Word and browsers.

---

## 7. Legacy (FM Abhaya) keyboard specifics

- **Same typing layout** as the Unicode keyboard (Helakuru phonetic). Only the **output encoding** differs.
- FM Abhaya is a **visual-order** encoding: pre-base vowel signs (e, ee, ai, o, oo, au) must be output **before** the consonant they belong to. The engine therefore always works on Unicode logically first, then converts the **whole syllable** to legacy codes (`legacy.rs`).
- Punctuation mapping differs in the FM font; follow the mapping table.
- The mapping table can be derived from the MIT-licensed Pandukabhaya project. **Keep attribution.**
- The font itself is proprietary: do not bundle it. Document that the user must set the font to **FM Abhaya** in Word.
- Add legacy expected values to the golden table and test them the same way.

---

## 8. Phases

### Phase 0 — Reset (1–2 days)

- Create a clean repo or branch. Archive old releases (mark old pre-releases clearly as deprecated).
- Add `LICENSE` (MIT at repo level), a short honest `README.md`.
- Add CI: `cargo fmt --check`, `cargo clippy`, `cargo test`.

**Exit:** clean repo, CI green on an empty engine crate.

### Phase 1 — Golden table and engine (about 1 week)

- Owner supplies `data/golden_pairs.csv`.
- Implement `engine/` per section 6. Generate tests from the CSV.
- Implement `legacy.rs` and fill `legacy_expected`.

**Exit:** 100% of golden rows pass (Unicode and Legacy). No panics under fuzz / random input tests.

### Phase 2 — Keyman prototype (about 1 week)

- Write `sinhala_unicode.kmn` and `sinhala_legacy.kmn` with the same rules.
- Package as `.kmp`; register both under `si-LK`.
- Check Win+Space cycling: English → Sinhala Unicode → Sinhala Legacy.
- Test on Windows 10 22H2 and Windows 11 (VMs).

**Exit:** both keyboards appear in Windows Settings under Sinhala; Win+Space works; typing matches the golden table in Notepad and Word.

**Decision point:** ask the owner whether a dependency-free TSF product is required. If not, stop here and polish (docs, packaging).

### Phase 3 — TSF IME (optional)

Implement a Windows text service that uses `engine/`.

Requirements (items marked VERIFY must be checked against current Microsoft docs):

- **One CLSID, two profile GUIDs**, language `si-LK` (LANGID `0x045B`) — VERIFY.
- Register with `ITfInputProcessorProfileMgr::RegisterProfile`, and add keyboards to the user's language list with `InstallLayoutOrTip` (from `input.dll`) — VERIFY. This is the area that failed in the old v2.0.0.
- Registry (HKLM) registration requires admin; the installer must elevate.
- Ship **x64 and x86 DLLs** (32-bit apps must also work). ARM64 optional.
- **Never let a panic cross the FFI boundary.** Wrap every COM entry point in `std::panic::catch_unwind`, build with `panic = "unwind"`, and return a safe error code on failure. An IME loads inside other people's processes; it must never crash them.
- Keep the DLL minimal: no network access, no heavy dependencies, no blocking work. File logging only, and only behind a debug flag.
- Use the **composition** approach (section 6.4).
- Reference implementation if Rust COM is too painful: Microsoft's **SampleIME** (C++) — VERIFY availability.
- **Code signing:** an unsigned DLL injected into every process can be blocked by Defender, Windows 11 Smart App Control, and some protected processes. Plan for signing (for example SignPath Foundation for open source, or Azure Trusted Signing) — VERIFY options.
- Uninstall must remove all registry entries and profiles cleanly.

**Exit:** after installing on a clean Windows 10 VM and a clean Windows 11 VM, both keyboards appear in Settings, Win+Space cycles correctly, typing works in the test matrix below, uninstall leaves nothing behind.

### Phase 4 — Release

- One installer script. One version scheme (SemVer). A signed installer if possible.
- Update README (install steps, how to add the font, known issues, how to switch with Win+Space).
- Real-machine test on at least three machines (Win10, Win11, one with a 32-bit app).

---

## 9. Test matrix

| Area | Cases |
|---|---|
| Engine | All golden rows (Unicode + Legacy), random-input fuzz (no panics), backspace behaviour |
| Apps | Notepad (new Win11 version), Word, Excel, Chrome, Edge, VS Code, Windows Search box, Settings (UWP), a 32-bit app, an Admin window, RDP |
| Switching | Win+Space cycle order, per-app input method memory, switching mid-word (composition must commit or reset cleanly) |
| Install | Clean install, upgrade, uninstall, reinstall on Windows 10 22H2 and Windows 11 |
| Shortcuts | Ctrl+C / V / Z, Alt+Tab, Win key combos must pass through untouched |

---

## 10. Definition of done

- Windows 10 and 11: Settings shows **Sinhala Unicode (Phonetic)** and **Sinhala Legacy (FM Abhaya)** under Sinhala.
- **Win + Space** cycles English → Sinhala Unicode → Sinhala Legacy.
- Typing matches the Helakuru golden table exactly.
- Legacy output renders correctly with the FM Abhaya font in Word.
- No crashes in host apps; clean uninstall.
- Repo has a license, an accurate README, one installer, and green CI.
