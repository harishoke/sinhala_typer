# Sinhala Phonetic Keymap (data/keymap.md)

**Purpose:** authoritative input for `src/engine/phonetic.rs` (Prompt 1). Implement exactly this; do not add or change entries.

## 0. Read this first: what “verified” means here

| Marker | Meaning |
| :--- | :--- |
| ✅ | **Structural fact.** Unicode code points and the rules for al-lakuna, ZWJ, rakaranshaya, yansaya (e.g. rakaranshaya = al-lakuna + ZWJ + ර; yansaya = al-lakuna + ZWJ + ය, as documented in the Wikipedia Sinhala font guide) or an entry that matches every Singlish scheme I compared. |
| 🟡 | **Scheme choice.** Singlish schemes disagree on these, and **no single official published key table for this scheme**. The option believed to be most common was chosen, compared against the UCSC “Singlish phonetic” converter (which disagrees on several of them, e.g. `sh`/`Sh`, `nnd`/`nndh`). Confirm each 🟡 row using the checklist in `verified_pairs.md` Part D, then edit this file if your reference differs. |

Two consequences you should know about:

1. Predictive phone keyboards also use word prediction and autocorrect, so they can show `අම්මා` after you type `amma` and pick a suggestion. A rule-based engine cannot do that. In this spec the rule-based input is `ammaa`. The original plan’s examples `amma`, `lankawa`, `vidyava` do not follow from any consistent rule, so they were replaced by `ammaa`, `lankaava`, `vidhyaava` (see `verified_pairs.md`).
2. Case matters. `t`/`T`, `d`/`D`, `n`/`N`, `l`/`L`, `s`/`S` are different keys. The hook must use the character actually produced (after Shift/Caps Lock), not the physical key.

## 1. Design decisions (the rules the engine implements)

| ID | Rule | Status |
| :--- | :--- | :--- |
| D1 | **Longest match wins**, case-sensitive, within the current word (letters since the last reset). | ✅ |
| D2 | A vowel at the start of the word, or right after another vowel, is an **independent vowel** (Section 2). After a consonant it is a **vowel sign** (pillam). | ✅ |
| D3 | Consonant + `a` = the bare consonant (inherent vowel). No `a` is output. | ✅ |
| D4 | A consonant followed by **no vowel** and then another consonant, or by the end of the word, gets **al-lakuna** (U+0DCA). Example: `mal` → මල්, `ammaa` → අම්මා. | ✅ |
| D5 | **Rakaranshaya:** consonant + `r` + vowel → C + U+0DCA + U+200D + ර + sign. Example: `krama` → ක්‍රම. | ✅ |
| D6 | **Yansaya:** consonant + `y` + vowel → C + U+0DCA + U+200D + ය + sign. Example: `vidhyaava` → විද්‍යාව. | ✅ |
| D7 | **Repaya:** `r` followed directly by a consonant (no vowel) → ර + U+0DCA + U+200D, then the consonant. Example: `Dharma` → ධර්‍ම. A `r` at the very end of a word or before a space is ර් with no ZWJ. | 🟡 |
| D8 | **Anusvara (ං) automatic:** a plain `n` directly before `k`, `g` or `h` (no vowel between) → ං. Examples: `lankaa` → ලංකා, `sinhala` → සිංහල, `gangaa` → ගංගා. Any other `n` without a vowel is න් (e.g. `pansala` → පන්සල). Explicit anusvara: `x`. Visarga ඃ: `H`. | 🟡 |
| D9 | **Prenasalised letters** use a doubled `n`: `nng` ඟ, `nnd` ඳ, `nnD` ඬ, `nnj` ඦ. `mb` → ඹ. These win by longest match over D8. | 🟡 |
| D10 | Digits, spaces, punctuation and all unmapped characters pass through unchanged and **reset** the word state. | ✅ |
| D11 | There is no escape key in the MVP. A sequence that is both a digraph and two letters (e.g. `sh` could be ස+හ) always becomes the digraph. | 🟡 |

## 2. Vowels

Roman keys, their independent form (start of word / after a vowel), and their sign form (after a consonant).

| Roman | Independent | Code point | Sign | Sign code point | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `a` | අ | U+0D85 | (none, inherent) | – | ✅ |
| `aa` | ආ | U+0D86 | ා | U+0DCF | ✅ |
| `ae` | ඇ | U+0D87 | ැ | U+0DD0 | 🟡 |
| `aae` | ඈ | U+0D88 | ෑ | U+0DD1 | 🟡 |
| `i` | ඉ | U+0D89 | ි | U+0DD2 | ✅ |
| `ii` | ඊ | U+0D8A | ී | U+0DD3 | ✅ |
| `u` | උ | U+0D8B | ු | U+0DD4 | ✅ |
| `uu` | ඌ | U+0D8C | ූ | U+0DD6 | ✅ |
| `Ru` | ඍ | U+0D8D | ෘ | U+0DD8 | 🟡 |
| `e` | එ | U+0D91 | ෙ | U+0DD9 | ✅ |
| `ee` | ඒ | U+0D92 | ේ | U+0DDA | ✅ |
| `ai` | ඓ | U+0D93 | ෛ | U+0DDB | 🟡 |
| `o` | ඔ | U+0D94 | ො | U+0DDC | ✅ |
| `oo` | ඕ | U+0D95 | ෝ | U+0DDD | ✅ |
| `au` | ඖ | U+0D96 | ෞ | U+0DDE | ✅ |

The two-part vowels ො (U+0DDC), ෝ (U+0DDD), ෞ (U+0DDE) must be emitted as **single precomposed code points**.

## 3. Consonants

| Roman | Sinhala | Code point | Status | Note |
| :--- | :--- | :--- | :--- | :--- |
| `k` | ක | U+0D9A | ✅ |  |
| `kh` | ඛ | U+0D9B | ✅ | aspirate |
| `g` | ග | U+0D9C | ✅ |  |
| `gh` | ඝ | U+0D9D | ✅ | aspirate |
| `Ng` | ඞ | U+0D9E | 🟡 | ඞ (rare) |
| `nng` | ඟ | U+0D9F | 🟡 | prenasalised |
| `c` | ච | U+0DA0 | ✅ |  |
| `ch` | ඡ | U+0DA1 | 🟡 | ඡ = `ch`; plain `c` = ච |
| `j` | ජ | U+0DA2 | ✅ |  |
| `jh` | ඣ | U+0DA3 | ✅ | aspirate |
| `nnj` | ඦ | U+0DA6 | 🟡 | prenasalised |
| `kn` | ඤ | U+0DA4 | 🟡 | ඤ |
| `gn` | ඥ | U+0DA5 | 🟡 | ඥ |
| `t` | ට | U+0DA7 | 🟡 | retroflex: ට |
| `T` | ඨ | U+0DA8 | 🟡 | retroflex aspirate: ඨ |
| `d` | ඩ | U+0DA9 | 🟡 | retroflex: ඩ |
| `D` | ඪ | U+0DAA | 🟡 | retroflex aspirate: ඪ |
| `N` | ණ | U+0DAB | ✅ | ණ |
| `nnD` | ඬ | U+0DAC | 🟡 | prenasalised (retroflex) |
| `th` | ත | U+0DAD | 🟡 | dental voiceless: ත |
| `Th` | ථ | U+0DAE | 🟡 | capital T + h |
| `dh` | ද | U+0DAF | 🟡 | dental voiced: ද |
| `Dh` | ධ | U+0DB0 | 🟡 | capital D + h |
| `n` | න | U+0DB1 | ✅ | න |
| `nnd` | ඳ | U+0DB3 | 🟡 | prenasalised (dental) |
| `p` | ප | U+0DB4 | ✅ |  |
| `ph` | ඵ | U+0DB5 | ✅ |  |
| `b` | බ | U+0DB6 | ✅ |  |
| `bh` | භ | U+0DB7 | ✅ |  |
| `m` | ම | U+0DB8 | ✅ |  |
| `mb` | ඹ | U+0DB9 | 🟡 | prenasalised |
| `y` | ය | U+0DBA | ✅ |  |
| `r` | ර | U+0DBB | ✅ |  |
| `l` | ල | U+0DBD | ✅ | ල |
| `v` | ව | U+0DC0 | ✅ | ව |
| `w` | ව | U+0DC0 | 🟡 | ව (same as `v`) |
| `sh` | ශ | U+0DC1 | 🟡 | ශ |
| `Sh` | ෂ | U+0DC2 | 🟡 | ෂ |
| `s` | ස | U+0DC3 | ✅ | ස |
| `h` | හ | U+0DC4 | ✅ |  |
| `L` | ළ | U+0DC5 | ✅ | ළ |
| `f` | ෆ | U+0DC6 | ✅ | ෆ |

Longest-match order the engine should use: 3-letter keys (`nng nnd nnD nnj`), then 2-letter keys (`kh gh ch jh th Th dh Dh ph bh sh Sh gn kn Ng mb Ru aa ae ii uu ee ai oo au`), then 1-letter keys. `aae` is a 3-letter vowel key.

## 4. Special keys

| Roman | Output | Code point | Status |
| :--- | :--- | :--- | :--- |
| `x` | ං | U+0D82 | 🟡 |
| `H` | ඃ | U+0D83 | 🟡 |

## 5. Exact Unicode sequences the engine must produce

| Feature | Sequence | Example (Roman → code points) | Status |
| :--- | :--- | :--- | :--- |
| Al-lakuna | C + U+0DCA | `mal` → U+0DB8 U+0DBD U+0DCA | ✅ |
| Rakaranshaya | C + U+0DCA + **U+200D** + U+0DBB | `krama` → U+0D9A U+0DCA U+200D U+0DBB U+0DB8 | ✅ |
| Yansaya | C + U+0DCA + **U+200D** + U+0DBA | `vidhyaava` → U+0DC0 U+0DD2 U+0DAF U+0DCA U+200D U+0DBA U+0DCF U+0DC0 | ✅ |
| Repaya | U+0DBB + U+0DCA + **U+200D** + C | `Dharma` → U+0DB0 U+0DBB U+0DCA U+200D U+0DB8 | 🟡 |
| Anusvara | C + U+0D82 (after the sign if any) | `lankaa` → U+0DBD U+0D82 U+0D9A U+0DCF | 🟡 |

Rules: ZWJ (U+200D) appears **only** directly after al-lakuna in the three conjunct cases above. Never emit ZWNJ (U+200C). Never leave a lone ZWJ or a trailing al-lakuna + ZWJ after a Backspace.

## 6. Streaming and Backspace semantics

* After every key the on-screen text for the current word must equal `batch_convert(letters typed so far in this word)`, where `batch_convert` is the rule set above applied to the whole prefix. Intermediate states can look odd (for example `kr` shows ක්ර් before the next key); that is expected.
* Backspace undoes the **last Roman key**, i.e. the screen becomes `batch_convert(prefix minus its last letter)`.
* `reset()` on space, Enter, Tab, punctuation, digits, mouse click, focus change, mode toggle.
* `verified_pairs.md` Part C gives prefix-by-prefix expected screens for six words; use them as tests.

## 7. Known conflicts and open questions

| # | Item | Current choice | Alternatives seen in other schemes |
| :--- | :--- | :--- | :--- |
| 1 | `sh` / `Sh` | `sh` = ශ, `Sh` = ෂ | UCSC converter: `sh` = ෂ |
| 2 | `nnd` / `nndh` | `nnd` = ඳ, `nnD` = ඬ | UCSC converter: `nnd` = ඬ, `nndh` = ඳ |
| 3 | `t` / `th` / `d` / `dh` | `t` ට, `th` ත, `d` ඩ, `dh` ද | Not compared against another table; confirm with the checklist in `verified_pairs.md` Part D |
| 4 | `ch` | ඡ (plain `c` = ච) | UCSC converter also uses `ch` = ඡ (agrees) |
| 5 | Anusvara | automatic before k / g / h, else `x` | Some schemes: only explicit |
| 6 | `oo` | ඕ / ෝ | UCSC converter maps `oo` to ඌ / ූ (differs from my choice) |
| 7 | Repaya | automatic on `r` + consonant | UCSC converter: explicit `\r` key |

If the verification checklist shows a difference, change the row here **first**, then re-run the tests; never patch the Rust code to match one word.