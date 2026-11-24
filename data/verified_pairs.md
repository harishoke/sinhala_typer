# Verified Pairs (data/verified_pairs.md)

Test data for Prompts 1 and 2. **Rule for Mistral: use only the expected outputs in this file; never invent new ones.**

## Status of this file

| Part | What it is | How it was checked | Still needs you |
| :--- | :--- | :--- | :--- |
| A | Roman → Sinhala Unicode | Expected Sinhala typed by hand from known spellings, then **reproduced exactly (30/30) by an independent reference implementation of `keymap_helakuru.md`**. This proves the keymap is internally consistent and gives correct standard spellings. | Confirm against the real Helakuru app (Part D) |
| B | Sinhala Unicode → FM Abhaya | Generated from the MIT Pandukabhaya mapping; **round-tripped 30/30** through its FM→Unicode converter. Two words also match examples published by other packages. | Visual check in Word (`fm_abhaya_table.md` Section 8) |
| C | Prefix expectations | Output of the reference implementation on every prefix | None (follows from Part A rules) |
| D | Helakuru phone checklist | – | You (10 minutes) |

## Part A: Roman → Sinhala Unicode

| ☐ | # | Roman (type this) | Sinhala (expected) | Tests |
| :--- | :--- | :--- | :--- | :--- |
| ☐ | A1 | `mama` | මම | basics |
| ☐ | A2 | `oya` | ඔය | independent vowel o |
| ☐ | A3 | `api` | අපි | a/i |
| ☐ | A4 | `ammaa` | අම්මා | al-lakuna, long aa |
| ☐ | A5 | `thaaththaa` | තාත්තා | th, doubled consonant |
| ☐ | A6 | `krama` | ක්‍රම | rakaranshaya |
| ☐ | A7 | `lankaa` | ලංකා | auto anusvara |
| ☐ | A8 | `lankaava` | ලංකාව | auto anusvara + v |
| ☐ | A9 | `vidhyaava` | විද්‍යාව | yansaya |
| ☐ | A10 | `sinhala` | සිංහල | auto anusvara before h |
| ☐ | A11 | `shrii lankaa` | ශ්‍රී ලංකා | rakaranshaya + ii + space |
| ☐ | A12 | `aayuboowan` | ආයුබෝවන් | oo two-part vowel, final al-lakuna |
| ☐ | A13 | `isthuthi` | ඉස්තුති | s + th, al-lakuna |
| ☐ | A14 | `kohomadha` | කොහොමද | o two-part vowel, dh |
| ☐ | A15 | `pansala` | පන්සල | n + s = න් |
| ☐ | A16 | `gamana` | ගමන | basics |
| ☐ | A17 | `paasala` | පාසල | aa |
| ☐ | A18 | `kaanthaawa` | කාන්තාව | n + th |
| ☐ | A19 | `shiShya` | ශිෂ්‍ය | Sh + yansaya |
| ☐ | A20 | `Dharma` | ධර්‍ම | repaya, Dh |
| ☐ | A21 | `karma` | කර්‍ම | repaya |
| ☐ | A22 | `mal` | මල් | final al-lakuna |
| ☐ | A23 | `rathu` | රතු | u sign |
| ☐ | A24 | `koLamba` | කොළඹ | L, mb |
| ☐ | A25 | `gangaa` | ගංගා | auto anusvara before g |
| ☐ | A26 | `suuriya` | සූරිය | uu sign |
| ☐ | A27 | `puujaa` | පූජා | uu, aa |
| ☐ | A28 | `RuShi` | ඍෂි | Ru independent |
| ☐ | A29 | `aanandha` | ආනන්ද | n + dh = න්ද |
| ☐ | A30 | `sundhara` | සුන්දර | u, n + dh |
| ☐ | A31 | `pela` | පෙල | kombuwa e |
| ☐ | A32 | `mee` | මේ | ee-pilla (ේ) |
| ☐ | A33 | `ekak` | එකක් | independent e, final al-lakuna |

Code points for the ZWJ cases (check your output byte-for-byte):

* `krama` → U+0D9A U+0DCA U+200D U+0DBB U+0DB8
* `vidhyaava` → U+0DC0 U+0DD2 U+0DAF U+0DCA U+200D U+0DBA U+0DCF U+0DC0
* `shrii lankaa` → U+0DC1 U+0DCA U+200D U+0DBB U+0DD3 U+0020 U+0DBD U+0D82 U+0D9A U+0DCF
* `shiShya` → U+0DC1 U+0DD2 U+0DC2 U+0DCA U+200D U+0DBA
* `Dharma` → U+0DB0 U+0DBB U+0DCA U+200D U+0DB8
* `karma` → U+0D9A U+0DBB U+0DCA U+200D U+0DB8

Pass-through checks: `mama 123` → `මම 123`; `mama, oya.` → `මම, ඔය.`; an empty word stays empty.

## Part B: Sinhala Unicode → FM Abhaya

The FM column is what must be **typed into Word** (as ordinary Unicode characters, injected with `KEYEVENTF_UNICODE`) so that the FM Abhaya font displays the Sinhala word. It contains non-ASCII Latin letters such as `ï`, `ß`, `Í`.

| ☐ | # | Sinhala (input) | FM Abhaya (expected) | Round-trip |
| :--- | :--- | :--- | :--- | :--- |
| ☐ | B1 | මම | `uu` | ✅ |
| ☐ | B2 | ඔය | `Th` | ✅ |
| ☐ | B3 | අපි | `wms` | ✅ |
| ☐ | B4 | අම්මා | `wïud` | ✅ |
| ☐ | B5 | තාත්තා | `;d;a;d` | ✅ |
| ☐ | B6 | ක්‍රම | `l%u` | ✅ |
| ☐ | B7 | ලංකා | `,xld` | ✅ |
| ☐ | B8 | ලංකාව | `,xldj` | ✅ |
| ☐ | B9 | විද්‍යාව | `úoHdj` | ✅ |
| ☐ | B10 | සිංහල | `isxy,` | ✅ |
| ☐ | B11 | ශ්‍රී ලංකා | `Y%S ,xld` | ✅ |
| ☐ | B12 | ආයුබෝවන් | `wdhqfndajka` | ✅ |
| ☐ | B13 | ඉස්තුති | `bia;q;s` | ✅ |
| ☐ | B14 | කොහොමද | `fldfyduo` | ✅ |
| ☐ | B15 | පන්සල | `mkai,` | ✅ |
| ☐ | B16 | ගමන | `.uk` | ✅ |
| ☐ | B17 | පාසල | `mdi,` | ✅ |
| ☐ | B18 | කාන්තාව | `ldka;dj` | ✅ |
| ☐ | B19 | ශිෂ්‍ය | `YsIH` | ✅ |
| ☐ | B20 | ධර්‍ම | `Ou_` | ✅ |
| ☐ | B21 | කර්‍ම | `lu_` | ✅ |
| ☐ | B22 | මල් | `u,a` | ✅ |
| ☐ | B23 | රතු | `r;q` | ✅ |
| ☐ | B24 | කොළඹ | `fld<U` | ✅ |
| ☐ | B25 | ගංගා | `.x.d` | ✅ |
| ☐ | B26 | සූරිය | `iQßh` | ✅ |
| ☐ | B27 | පූජා | `mQcd` | ✅ |
| ☐ | B28 | ඍෂි | `RIs` | ✅ |
| ☐ | B29 | ආනන්ද | `wdkkao` | ✅ |
| ☐ | B30 | සුන්දර | `iqkaor` | ✅ |
| ☐ | B31 | පෙල | `fm,` | ✅ |
| ☐ | B32 | මේ | `fï` | ✅ |
| ☐ | B33 | එකක් | `tlla` | ✅ |

Test notes for `legacy.rs`:

* Kombuwa-family **reordering** tests (the `f` comes before its consonant): B12 `ආයුබෝවන්` → `wdhqfndajka`, B14 `කොහොමද` → `fldfyduo`, B24 `කොළඹ` → `fld<U`, B31 `පෙල` → `fm,`, B32 `මේ` → `fï`.
* **Not covered by any pair yet:** ෛ (ai), ෞ (au) and ෘ. Add verified words for them before relying on those rules.
* B4 `අම්මා` → `wïud` uses the precomposed glyph `ï` (ම්), not `ua`.
* B11 `ශ්‍රී ලංකා` → `Y%S ,xld` keeps the space unchanged.

## Part C: Streaming prefix expectations (Backspace and `render_diff` tests)

Each row: after typing the Roman prefix, the screen must equal the “Unicode screen”. ‹ZWJ› marks an invisible U+200D. Backspace from row N must return to row N−1.

### `krama`

| Prefix | Unicode screen | Escaped | FM screen (for legacy mode) |
| :--- | :--- | :--- | :--- |
| `k` | ක් | `ක්` | `la` |
| `kr` | ක්ර් | `ක්ර්` | `la¾` |
| `kra` | ක්‍ර | `ක්‹ZWJ›ර` | `l%` |
| `kram` | ක්‍රම් | `ක්‹ZWJ›රම්` | `l%ï` |
| `krama` | ක්‍රම | `ක්‹ZWJ›රම` | `l%u` |

### `sinhala`

| Prefix | Unicode screen | Escaped | FM screen (for legacy mode) |
| :--- | :--- | :--- | :--- |
| `s` | ස් | `ස්` | `ia` |
| `si` | සි | `සි` | `is` |
| `sin` | සින් | `සින්` | `iska` |
| `sinh` | සිංහ් | `සිංහ්` | `isxya` |
| `sinha` | සිංහ | `සිංහ` | `isxy` |
| `sinhal` | සිංහල් | `සිංහල්` | `isxy,a` |
| `sinhala` | සිංහල | `සිංහල` | `isxy,` |

### `Dharma`

| Prefix | Unicode screen | Escaped | FM screen (for legacy mode) |
| :--- | :--- | :--- | :--- |
| `D` | ඪ් | `ඪ්` | `Va` |
| `Dh` | ධ් | `ධ්` | `è` |
| `Dha` | ධ | `ධ` | `O` |
| `Dhar` | ධර් | `ධර්` | `O¾` |
| `Dharm` | ධර්‍ම් | `ධර්‹ZWJ›ම්` | `Ou_්` |
| `Dharma` | ධර්‍ම | `ධර්‹ZWJ›ම` | `Ou_` |

### `shrii`

| Prefix | Unicode screen | Escaped | FM screen (for legacy mode) |
| :--- | :--- | :--- | :--- |
| `s` | ස් | `ස්` | `ia` |
| `sh` | ශ් | `ශ්` | `Ya` |
| `shr` | ශ්ර් | `ශ්ර්` | `Ya¾` |
| `shri` | ශ්‍රි | `ශ්‹ZWJ›රි` | `Y%s` |
| `shrii` | ශ්‍රී | `ශ්‹ZWJ›රී` | `Y%S` |

### `ammaa`

| Prefix | Unicode screen | Escaped | FM screen (for legacy mode) |
| :--- | :--- | :--- | :--- |
| `a` | අ | `අ` | `w` |
| `am` | අම් | `අම්` | `wï` |
| `amm` | අම්ම් | `අම්ම්` | `wïï` |
| `amma` | අම්ම | `අම්ම` | `wïu` |
| `ammaa` | අම්මා | `අම්මා` | `wïud` |

### `vidhyaava`

| Prefix | Unicode screen | Escaped | FM screen (for legacy mode) |
| :--- | :--- | :--- | :--- |
| `v` | ව් | `ව්` | `õ` |
| `vi` | වි | `වි` | `ú` |
| `vid` | විඩ් | `විඩ්` | `úâ` |
| `vidh` | විද් | `විද්` | `úoa` |
| `vidhy` | විද්ය් | `විද්ය්` | `úoaha` |
| `vidhya` | විද්‍ය | `විද්‹ZWJ›ය` | `úoH` |
| `vidhyaa` | විද්‍යා | `විද්‹ZWJ›යා` | `úoHd` |
| `vidhyaav` | විද්‍යාව් | `විද්‹ZWJ›යාව්` | `úoHdõ` |
| `vidhyaava` | විද්‍යාව | `විද්‹ZWJ›යාව` | `úoHdj` |

For legacy mode the engine must call `render_diff(on_screen_fm, new_unicode_word)`. Example: in `krama`, typing `kr` changes ක්ර් to the rakaranshaya form on the next key, so backspaces are counted in **FM characters**, not Unicode ones.

## Part D: Helakuru phone checklist (you do this, ~10 minutes)

1. On your phone, open any text box with the **Helakuru keyboard** in **Phonetic** mode. **Turn off suggestions/prediction** if the app lets you; otherwise ignore the suggestion bar and keep typing.
2. Type each Roman string from Part A and compare with the Sinhala column.
3. For every mismatch, write what Helakuru produced in the last column and send me the list. I will update `keymap_helakuru.md` (and these pairs) so the spec matches the real app.

| ☐ | Roman | Expected | Helakuru produced (if different) |
| :--- | :--- | :--- | :--- |
| ☐ | `mama` | මම | |
| ☐ | `oya` | ඔය | |
| ☐ | `api` | අපි | |
| ☐ | `ammaa` | අම්මා | |
| ☐ | `thaaththaa` | තාත්තා | |
| ☐ | `krama` | ක්‍රම | |
| ☐ | `lankaa` | ලංකා | |
| ☐ | `lankaava` | ලංකාව | |
| ☐ | `vidhyaava` | විද්‍යාව | |
| ☐ | `sinhala` | සිංහල | |
| ☐ | `shrii lankaa` | ශ්‍රී ලංකා | |
| ☐ | `aayuboowan` | ආයුබෝවන් | |
| ☐ | `isthuthi` | ඉස්තුති | |
| ☐ | `kohomadha` | කොහොමද | |
| ☐ | `pansala` | පන්සල | |
| ☐ | `gamana` | ගමන | |
| ☐ | `paasala` | පාසල | |
| ☐ | `kaanthaawa` | කාන්තාව | |
| ☐ | `shiShya` | ශිෂ්‍ය | |
| ☐ | `Dharma` | ධර්‍ම | |
| ☐ | `karma` | කර්‍ම | |
| ☐ | `mal` | මල් | |
| ☐ | `rathu` | රතු | |
| ☐ | `koLamba` | කොළඹ | |
| ☐ | `gangaa` | ගංගා | |
| ☐ | `suuriya` | සූරිය | |
| ☐ | `puujaa` | පූජා | |
| ☐ | `RuShi` | ඍෂි | |
| ☐ | `aanandha` | ආනන්ද | |
| ☐ | `sundhara` | සුන්දර | |
| ☐ | `pela` | පෙල | |
| ☐ | `mee` | මේ | |
| ☐ | `ekak` | එකක් | |
| ☐ | `t` | ට | |
| ☐ | `th` | ත | |
| ☐ | `Th` | ථ | |
| ☐ | `d` | ඩ | |
| ☐ | `dh` | ද | |
| ☐ | `Dh` | ධ | |
| ☐ | `sh` | ශ | |
| ☐ | `Sh` | ෂ | |
| ☐ | `nnd` | ඳ | |
| ☐ | `nnD` | ඬ | |
| ☐ | `nng` | ඟ | |
| ☐ | `ch` | ඡ | |
| ☐ | `c` | ච | |
| ☐ | `x` | ං | |
| ☐ | `H` | ඃ | |
| ☐ | `Ru` | ඍ | |