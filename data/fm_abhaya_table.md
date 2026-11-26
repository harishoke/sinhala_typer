# FM Abhaya Table: Unicode → FM Abhaya (data/fm_abhaya_table.md)

**Purpose:** authoritative input for `src/engine/legacy.rs` (Prompt 2). Build a static table from this file; do not recall FM codes from memory.

## 0. Provenance and verification status

| Item | Detail |
| :--- | :--- |
| Source | **Pandukabhaya** FM Abhaya mapping (`fm_abhaya.json`, v1.0, author *paarandika*, **MIT license**, https://github.com/akuruAI/Pandukabhaya), which is itself derived from the UCSC mapping. The tables below are generated directly from that file (no hand-typed codes). |
| Direction | The source maps FM → Unicode. Tables here are the **inverse** (Unicode → FM), plus encoding rules. |
| Cross-checks done | (1) `අම්මා` → `wïud` matches the example published by another independent package (`sinhala_unicode_converter`). (2) `ශ්‍රී ලංකා` → `Y%S ,xld` matches the example published by a third package (`sinhala-unicode-coverter`, DL-Manel/FM share this word). (3) All 30 words in `verified_pairs.md` Part B were produced by an encoder built from these rules and **round-tripped back to the original Unicode through the Pandukabhaya converter with 0 mismatches**. |
| **NOT yet verified** | **Visual rendering in Word with the real FM Abhaya font.** A round-trip proves the codes are consistent with a community mapping, not that every glyph looks right. Run the visual check sheet in Section 8 (10 minutes) before trusting a row marked ⚠. |
| Font licence | The FM Abhaya *font* is proprietary (Pushpananda Ekanayaka). Install your own licensed copy for testing; do not bundle it. |

Legend: ✅ = in the source mapping and exercised by round-trip tests; ⚠ = in the source mapping but **rendering must be checked visually**; 🚧 = not covered yet (see Section 9).

## 1. Independent vowels

| Unicode | Code point | FM keys | Status |
| :--- | :--- | :--- | :--- |
| අ | U+0D85 | `w` | ✅ |
| ආ | U+0D86 | `wd` | ✅ |
| ඇ | U+0D87 | `we` | ✅ |
| ඈ | U+0D88 | `wE` | ✅ |
| ඉ | U+0D89 | `b` | ✅ |
| ඊ | U+0D8A | `B` | ✅ |
| උ | U+0D8B | `W` | ✅ |
| ඌ | U+0D8C | `W!` | ✅ |
| ඍ | U+0D8D | `R` | ✅ |
| ඎ | U+0D8E | `RD` | ✅ |
| ඏ | U+0D8F | `Ì` | ⚠ |
| ඐ | U+0D90 | `Ï` | ⚠ |
| එ | U+0D91 | `t` | ✅ |
| ඒ | U+0D92 | `ta` | ✅ |
| ඓ | U+0D93 | `ft` | ✅ |
| ඔ | U+0D94 | `T` | ✅ |
| ඕ | U+0D95 | `´` | ⚠ |
| ඖ | U+0D96 | `T!` | ⚠ |

Note: `ඏ` (`Ì`) and `ඐ` (`Ï`) are extremely rare; `ඖ` and `ඕ` use extended glyphs, so check them visually.

## 2. Consonants (base letters, with inherent “a”)

| Unicode | Code point | FM key(s) | Alternates seen in source | Status |
| :--- | :--- | :--- | :--- | :--- |
| ක | U+0D9A | `l` | – | ✅ |
| ඛ | U+0D9B | `L` | – | ✅ |
| ග | U+0D9C | `.` | – | ✅ |
| ඝ | U+0D9D | `>` | – | ✅ |
| ඞ | U+0D9E | `X` | – | ⚠ |
| ඟ | U+0D9F | `Õ` | – | ⚠ |
| ච | U+0DA0 | `p` | – | ✅ |
| ඡ | U+0DA1 | `P` | – | ✅ |
| ජ | U+0DA2 | `c` | – | ✅ |
| ඣ | U+0DA3 | `CO` | `®` | ⚠ |
| ඤ | U+0DA4 | `[` | `Ø` | ⚠ |
| ඥ | U+0DA5 | `{` | `×` | ⚠ |
| ට | U+0DA7 | `g` | – | ✅ |
| ඨ | U+0DA8 | `G` | – | ✅ |
| ඩ | U+0DA9 | `v` | – | ✅ |
| ඪ | U+0DAA | `V` | – | ✅ |
| ණ | U+0DAB | `K` | – | ✅ |
| ඬ | U+0DAC | `~` | – | ⚠ |
| ත | U+0DAD | `;` | – | ✅ |
| ථ | U+0DAE | `:` | – | ✅ |
| ද | U+0DAF | `o` | – | ✅ |
| ධ | U+0DB0 | `O` | – | ✅ |
| න | U+0DB1 | `k` | – | ✅ |
| ඳ | U+0DB3 | `\|` | – | ⚠ |
| ප | U+0DB4 | `m` | – | ✅ |
| ඵ | U+0DB5 | `M` | – | ✅ |
| බ | U+0DB6 | `n` | – | ✅ |
| භ | U+0DB7 | `N` | – | ✅ |
| ම | U+0DB8 | `u` | – | ✅ |
| ඹ | U+0DB9 | `U` | – | ✅ |
| ය | U+0DBA | `h` | – | ✅ |
| ර | U+0DBB | `r` | – | ✅ |
| ල | U+0DBD | `,` | – | ✅ |
| ව | U+0DC0 | `j` | – | ✅ |
| ශ | U+0DC1 | `Y` | – | ✅ |
| ෂ | U+0DC2 | `I` | – | ✅ |
| ස | U+0DC3 | `i` | – | ✅ |
| හ | U+0DC4 | `y` | – | ✅ |
| ළ | U+0DC5 | `<` | – | ✅ |
| ෆ | U+0DC6 | `*` | – | ✅ |

Rule: **always encode with the primary key**. Alternates exist because FM has duplicate glyphs; the converter accepts both, but the primary one is what the FM keyboard layout produces.

Special note for prenasalised letters `ඟ ඬ ඳ ඦ`: the source also lists a back-tick form (`` `. `` `` `v `` `` `o `` `` `P ``). Do **not** use those; use the single-glyph keys above.

## 3. Vowel signs, al-lakuna, anusvara, visarga

| Unicode | Code point | FM keys | Position | Status |
| :--- | :--- | :--- | :--- | :--- |
| ා | U+0DCF | `d` | after the consonant | ✅ |
| ැ | U+0DD0 | `e` | after the consonant | ✅ |
| ෑ | U+0DD1 | `E` | after the consonant | ✅ |
| ි | U+0DD2 | `s` | after the consonant | ✅ |
| ී | U+0DD3 | `S` | after the consonant | ✅ |
| ු | U+0DD4 | `q` | after the consonant | ✅ |
| ූ | U+0DD6 | `Q` | after the consonant | ✅ |
| ෘ | U+0DD8 | `D` | after the consonant | ✅ |
| ෲ | U+0DF2 | `DD` | after the consonant | ✅ |
| ෟ | U+0DDF | `!` | after the consonant | ✅ |
| ් | U+0DCA | `a` | after the consonant | ✅ |
| ං | U+0D82 | `x` | after the consonant | ✅ |
| ඃ | U+0D83 | `#` | after the consonant | ✅ |
| ෙ | U+0DD9 | `f` | **before** the consonant | ✅ |
| ෛ | U+0DDB | `ff` | **before** the consonant | ✅ |
| ේ | U+0DDA | `f` before + `a` after | before + after (see Section 4) | ✅ |
| ො | U+0DDC | `f` before + `d` after | before + after | ✅ |
| ෝ | U+0DDD | `f` before + `da` after | before + after | ✅ |
| ෞ | U+0DDE | `f` before + `!` after | before + after | ✅ |

Input normalisation: the phonetic engine must emit the **precomposed** two-part vowels (U+0DDC, U+0DDD, U+0DDE). If text arrives decomposed (U+0DD9 + U+0DCF etc.), normalise it first.

## 4. Conjunct marks and composition rules

Let `C` be the FM key(s) of the base consonant (Section 2, or a precomposed glyph from Section 5).

| Unicode sequence | Meaning | FM encoding | Example (Unicode → FM) | Status |
| :--- | :--- | :--- | :--- | :--- |
| C + ් + ZWJ + ර | rakaranshaya | `C` + `%` | ක්‍ර → `l%` | ✅ |
| C + ් + ZWJ + ය | yansaya | `C` + `H` | ද්‍ය → `oH` | ✅ |
| ර + ් + ZWJ + C | repaya (on following C) | `C` + `_` | ර්‍ම → `u_`; කර්‍ම → `lu_` | ✅ |
| C + vowel sign | normal pillam | `C` + sign key; use a **precomposed glyph** instead when Section 5 has one | ක + ා → `ld` | ✅ |
| C + ් | al-lakuna | `C` + `a`; use precomposed glyph when Section 5 has one | ම් → `ï` (not `ua`) | ✅ |
| C + ් + ZWJ + ර + sign | rakaranshaya + pillam | `C` + `%` + sign key | ශ්‍රී → `Y%S` | ✅ |
| ෙ + C | kombuwa | `f` + `C` | කෙ → `fl` | ✅ |
| ෛ + C | ai-pilla | `ff` + `C` | ඩෛ → `ffv` | ✅ |
| ේ | ee-pilla | `f` + `C` (+ al-lakuna form) | ඥේ → `f{a`; ඛේ → `fÄ` | ✅ |
| ො | o-pilla | `f` + `C` + `d` | කො → `fld` | ✅ |
| ෝ | oo-pilla | `f` + `C` + `da` | ධෝ → `fOda` | ✅ |
| ෞ | au-pilla | `f` + `C` + `!` | බෞ → `fn!` | ✅ |
| Any of the above with conjunct marks | e.g. ද්‍රෙ | `f` + `C` + mark | ද්‍රෙ → `fo%` | ✅ |

**Reordering consequence (important for the hook):** the kombuwa family is typed *before* the consonant. When the user has typed a consonant and then types `e`/`o`/`ee`/`oo`/`ai`/`au`, the characters already on screen change. This is why `render_diff` is required (see plan Section 2.1).

**Order inside one syllable (FM keys):** `[f|ff]` + `C` + `[% | H]` + `[_]` + `[sign]`. Repaya `_` comes after the base and conjunct marks (source examples: `vH_` = ර්‍ඩ්‍ය).

## 5. Precomposed consonant + sign / al-lakuna glyphs (prefer these)

The FM font has single glyphs for many consonant + ි ී ු ූ ැ ෑ ා ් combinations. The FM keyboard produces them (example `අම්මා` → `wïud`, with `ï` = ම්). Use the glyph below whenever the pair exists; fall back to Section 2 + Section 3 otherwise.

| Unicode | FM glyph | Code point of FM char | Other FM chars for same text |
| :--- | :--- | :--- | :--- |
| ඛි | `Å` | U+00C5 | – |
| ඛී | `Ç` | U+00C7 | – |
| ඛ් | `Ä` | U+00C4 | – |
| ඞී | `ã` | U+00E3 | – |
| ඞ් | `Ù` | U+00D9 | – |
| චි | `Ñ` | U+00D1 | – |
| චී | `Ö` | U+00D6 | – |
| ච් | `É` | U+00C9 | – |
| ඡි | `ý` | U+00FD | – |
| ඡී | `Â` | U+00C2 | – |
| ඡ් | `þ` | U+00FE | – |
| ජි | `ð` | U+00F0 | – |
| ජී | `Ô` | U+00D4 | – |
| ජ් | `Ê` | U+00CA | – |
| ඣි | `¯` | U+00AF | `ˉ` |
| ඣී | `°` | U+00B0 | – |
| ඤු | `û` | U+00FB | – |
| ඤූ | `ü` | U+00FC | – |
| ටි | `á` | U+00E1 | – |
| ටී | `à` | U+00E0 | – |
| ට් | `Ü` | U+00DC | – |
| ඨි | `À` | U+00C0 | – |
| ඨී | `Á` | U+00C1 | – |
| ඩි | `ä` | U+00E4 | – |
| ඩ් | `â` | U+00E2 | – |
| ඪි | `Î` | U+00CE | – |
| ඪී | `Ð` | U+00D0 | – |
| ණි | `‚` | U+201A | – |
| ණී | `Œ` | U+0152 | – |
| ඬි | `ç` | U+00E7 | – |
| ඬී | `é` | U+00E9 | – |
| ඬ් | `å` | U+00E5 | – |
| ථි | `Ó` | U+00D3 | – |
| ථී | `Ò` | U+00D2 | – |
| දැ | `±` | U+00B1 | – |
| දා | `Þ` | U+00DE | – |
| දි | `È` | U+00C8 | – |
| දී | `§` | U+00A7 | – |
| දු | `ÿ` | U+00FF | – |
| ධි | `ê` | U+00EA | – |
| ධී | `ë` | U+00EB | – |
| ධ් | `è` | U+00E8 | – |
| ඳැ | `ƒ` | U+0192 | – |
| ඳි | `¢` | U+00A2 | – |
| ඳී | `£` | U+00A3 | – |
| ඳු | `÷` | U+00F7 | – |
| ඳූ | `ª` | U+00AA | – |
| ඵි | `Ý` | U+00DD | – |
| ඵී | `Ú` | U+00DA | – |
| බි | `ì` | U+00EC | – |
| බී | `î` | U+00EE | – |
| බ් | `í` | U+00ED | – |
| මි | `ñ` | U+00F1 | – |
| මී | `ó` | U+00F3 | – |
| ම් | `ï` | U+00EF | – |
| ඹි | `ô` | U+00F4 | – |
| ඹී | `ö` | U+00F6 | – |
| ඹ් | `ò` | U+00F2 | – |
| රැ | `/` | U+002F | – |
| රෑ | `?` | U+003F | – |
| රි | `ß` | U+00DF | – |
| රී | `Í` | U+00CD | – |
| ර් | `¾` | U+00BE | – |
| ලු | `¨` | U+00A8 | – |
| ලූ | `Æ` | U+00C6 | – |
| වි | `ú` | U+00FA | – |
| වී | `ù` | U+00F9 | – |
| ව් | `õ` | U+00F5 | – |
| ළු | `¿` | U+00BF | – |

(69 entries. Where “Other FM chars” is not empty, always emit the one in the FM glyph column.)

## 6. Ligature glyphs for common conjuncts (optional optimisation)

The encoder in `verified_pairs.md` uses the generic rules of Section 4 (e.g. `ද්‍ර` → `o%`). These single-glyph ligatures also exist in the source mapping. Whether the ligature or the generic form looks better in Word is a **visual check (⚠)**; prefer the ligature once confirmed.

| Unicode | FM ligature | Generic form (Section 4) |
| :--- | :--- | :--- |
| ක්‍ෂ | `Ë` | `C` + `I` |
| ත්‍ථ | `†` | `F` + `:` |
| ත්‍ව | `…` | `F` + `j` |
| න්‍ද | `‡` | `J` + `o` |
| ද්‍ව | `„` | – |
| ද්‍ධ | `Š` | – |
| ද්‍ර | `ø` | `o%` |
| ශ්‍රී | `›` | `Y%S` |
| ර්‍ණ | `“` | `K_` |
| ද්‍ධි | `‹` | – |
| ද්‍වි | `‰` | – |
| න්‍දා | `ˆ` | – |

Half-form keys: `C` = ක්‍ + ZWJ, `F` = ත්‍ + ZWJ, `J` = න්‍ + ZWJ (these precede the next consonant, e.g. `CI` = ක්‍ෂ).

## 7. Special single-glyph cases

| Unicode | FM | Note |
| :--- | :--- | :--- |
| රැ | `/` | ✅ in source |
| රෑ | `?` | ✅ in source |
| රි | `ß` | ✅ in source (precomposed) |
| රී | `Í` | ✅ in source |
| ර් | `¾` | ✅ in source |
| ළු | `¿` | ✅ in source |
| රු / රූ | 🚧 | The source normalises `=` to `q` and `+` to `Q`, which suggests FM uses alternate glyphs (`=`, `+`) for ු/ූ in some contexts (likely after ර). **Unverified: check visually** whether `rq` or `r=` gives the correct රු. |

## 8. Visual check sheet (do this once, ~10 minutes)

1. Install the **FM Abhaya** font. Open Word, set the font to FM Abhaya.
2. Paste each **FM text** below. The glyphs should read as the **Sinhala** column (view that column in a Unicode font like Iskoola Pota / Noto Sans Sinhala).
3. Tick the row or write the problem. Any ✗ row goes back to me/Mistral as a bug in the table, not in your Rust code.

| ☐ | Sinhala (Unicode) | FM text to paste |
| :--- | :--- | :--- |
| ☐ | මම | `uu` |
| ☐ | ඔය | `Th` |
| ☐ | අපි | `wms` |
| ☐ | අම්මා | `wïud` |
| ☐ | තාත්තා | `;d;a;d` |
| ☐ | ක්‍රම | `l%u` |
| ☐ | ලංකා | `,xld` |
| ☐ | ලංකාව | `,xldj` |
| ☐ | විද්‍යාව | `úoHdj` |
| ☐ | සිංහල | `isxy,` |
| ☐ | ශ්‍රී ලංකා | `Y%S ,xld` |
| ☐ | ආයුබෝවන් | `wdhqfndajka` |
| ☐ | ඉස්තුති | `bia;q;s` |
| ☐ | කොහොමද | `fldfyduo` |
| ☐ | පන්සල | `mkai,` |
| ☐ | ගමන | `.uk` |
| ☐ | පාසල | `mdi,` |
| ☐ | කාන්තාව | `ldka;dj` |
| ☐ | ශිෂ්‍ය | `YsIH` |
| ☐ | ධර්‍ම | `Ou_` |
| ☐ | කර්‍ම | `lu_` |
| ☐ | මල් | `u,a` |
| ☐ | රතු | `r;q` |
| ☐ | කොළඹ | `fld<U` |
| ☐ | ගංගා | `.x.d` |
| ☐ | සූරිය | `iQßh` |
| ☐ | පූජා | `mQcd` |
| ☐ | ඍෂි | `RIs` |
| ☐ | ආනන්ද | `wdkkao` |
| ☐ | සුන්දර | `iqkaor` |
| ☐ | පෙල | `fm,` |
| ☐ | මේ | `fï` |
| ☐ | එකක් | `tlla` |

## 9. Known gaps (🚧): not covered by this table yet

* Rare conjuncts not in Section 6 (e.g. `ස්ථ`, `ස්ව`, `ක්ෂ්‍ය`): use generic al-lakuna + ZWJ form until a visual check confirms a ligature.
* `ර + ු/ූ` (Section 7).
* DL-Araliya: a different code table; not included. Add it later as a second file.
* Digits and `-` pass through unchanged. Other punctuation is substituted by the hook in FM mode using the inverse of the Pandukabhaya `singles` punctuation rows (see `fm_punctuation` in `src/engine/legacy.rs`): in the FM font `,` draws ල and `.` draws ග, so the marks are emitted as the characters whose glyphs draw the mark (`,` → `"`, `.` → `'`, `?` → `@`, `:` → `(`, `;` → `¦`, `!` → `æ`, `(` → `^`, `)` → `&`, `"` → `˜`, `/` → `$`, `%` → `]`, `=` → `}`, `+` → `¬`, `{` → `™`, `}` → `š`).