# Sinhala Typer — phonetic keymap (owner-adopted spec, 2026-11-26)

The product owner adopted a well-known phonetic (ශබ්දානුකූල) layout as the
one and only keymap on 2026-11-26, replacing the v1.x in-house map. Rules:

- Doubling a vowel letter lengthens it (`a` → `aa`).
- Capitals give the retroflex (මුර්ධජ) and special letters.
- Aspirates (මහප්‍රාණ) add `h` — except the dentals, which take a double
  `h` (`thha`, `dhha`), and the retroflex aspirates, which are capitals
  (`Ta`, `Da`).
- The naasikya (සඤ්ඤක) letters take a `z` prefix; ඹ is capital `B`.
- Owner decision (2026-11-26): `ch` types ච (the spec's `cha` is ch + a,
  so both work); plain `c` also gives ච.
- Convenience kept from v1: a pending `n` before a velar/palatal
  consonant or `h` renders as ං (e.g. `sangha` → සංඝ, `punchi` → පුංචි).

## Independent vowels

| Key | Letter | | Key | Letter |
| :--- | :--- | :--- | :--- | :--- |
| a | අ U+0D85 | | e | එ U+0D91 |
| aa | ආ U+0D86 | | ee | ඒ U+0D92 |
| A | ඇ U+0D87 | | ai | ඓ U+0D93 |
| Aa / AA | ඈ U+0D88 | | o | ඔ U+0D94 |
| i | ඉ U+0D89 | | oo | ඕ U+0D95 |
| ii | ඊ U+0D8A | | au / ou | ඖ U+0D96 |
| u | උ U+0D8B | | R | ඍ U+0D8D |
| uu | ඌ U+0D8C | | Ru | ඎ U+0D8E |

## Consonants (stem + vowel; bare stem = consonant + ්)

| Key | Letter | | Key | Letter |
| :--- | :--- | :--- | :--- | :--- |
| ka | ක U+0D9A | | na | න U+0DB1 |
| kha | ඛ U+0D9B | | Na | ණ U+0DAB |
| ga | ග U+0D9C | | pa | ප U+0DB4 |
| gha | ඝ U+0D9D | | pha | ඵ U+0DB5 |
| cha (ch) | ච U+0DA0 | | ba | බ U+0DB6 |
| chha | ඡ U+0DA1 | | bha | භ U+0DB7 |
| ja | ජ U+0DA2 | | Ba | ඹ U+0DB9 |
| ta | ට U+0DA7 | | ma | ම U+0DB8 |
| Ta | ඨ U+0DA8 | | ya | ය U+0DBA |
| da | ඩ U+0DA9 | | ra | ර U+0DBB |
| Da | ඪ U+0DAA | | la | ල U+0DBD |
| tha | ත U+0DAD | | La | ළ U+0DC5 |
| thha | ථ U+0DAE | | wa / va | ව U+0DC0 |
| dha / qa | ද U+0DAF | | sha | ශ U+0DC1 |
| dhha | ධ U+0DB0 | | Sa / Sha | ෂ U+0DC2 |
| zga | ඟ U+0D9F | | sa | ස U+0DC3 |
| zja | ඦ U+0DA6 | | ha | හ U+0DC4 |
| zda | ඬ U+0DAC | | fa | ෆ U+0DC6 |
| zdha / zqa | ඳ U+0DB3 | | zka | ඤ U+0DA4 |
| zha | ඥ U+0DA5 | | | |

## Vowel signs (shown with ක)

| Key | Form | | Key | Form |
| :--- | :--- | :--- | :--- | :--- |
| k | ක් (් U+0DCA) | | ke | කෙ (ෙ U+0DD9) |
| ka | ක | | kee | කේ (ේ U+0DDA) |
| kaa | කා (ා U+0DCF) | | kai | කෛ (ෛ U+0DDB) |
| kA | කැ (ැ U+0DD0) | | ko | කො (ො U+0DDC) |
| kAa / kAA | කෑ (ෑ U+0DD1) | | koo | කෝ (ෝ U+0DDD) |
| ki | කි (ි U+0DD2) | | kau / kou | කෞ (ෞ U+0DDE) |
| kii | කී (ී U+0DD3) | | kaH | කඃ (ඃ U+0D83) |
| ku | කු (ු U+0DD4) | | kax / kazn | කං (ං U+0D82) |
| kuu | කූ (ූ U+0DD6) | | kaX | කඞ (ඞ U+0D9E) |
| kru | කෘ (ෘ U+0DD8) | | kya | ක්‍ය (් + ZWJ + ය) |
| kruu | කෲ (ෲ U+0DF2) | | kra | ක්‍ර (් + ZWJ + ර) |

Conjuncts: a consonant + `r` + vowel gives the රකාරාංශය (ක්‍ර), a bare
`r` before another consonant gives රේඵය (ර්‍), and consonant + `y` +
vowel gives the යංශය (ක්‍ය) — all automatic.
