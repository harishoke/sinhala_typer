use super::EditAction;

const ZWJ: char = '\u{200D}';
const AL_LAKUNA: char = '\u{0DCA}';
const RAKA: char = '\u{0DBB}';
const YANSA: char = '\u{0DBA}';

fn is_consonant(c: char) -> bool {
    ('\u{0D9A}'..='\u{0DC6}').contains(&c)
}

fn consonant_fm(c: char) -> Option<&'static str> {
    Some(match c {
        '\u{0D9A}' => "l",
        '\u{0D9B}' => "L",
        '\u{0D9C}' => ".",
        '\u{0D9D}' => ">",
        '\u{0D9E}' => "X",
        '\u{0D9F}' => "\u{00D5}",
        '\u{0DA0}' => "p",
        '\u{0DA1}' => "P",
        '\u{0DA2}' => "c",
        '\u{0DA3}' => "CO",
        '\u{0DA4}' => "[",
        '\u{0DA5}' => "{",
        '\u{0DA7}' => "g",
        '\u{0DA8}' => "G",
        '\u{0DA9}' => "v",
        '\u{0DAA}' => "V",
        '\u{0DAB}' => "K",
        '\u{0DAC}' => "~",
        '\u{0DAD}' => ";",
        '\u{0DAE}' => ":",
        '\u{0DAF}' => "o",
        '\u{0DB0}' => "O",
        '\u{0DB1}' => "k",
        '\u{0DB3}' => "\\|",
        '\u{0DB4}' => "m",
        '\u{0DB5}' => "M",
        '\u{0DB6}' => "n",
        '\u{0DB7}' => "N",
        '\u{0DB8}' => "u",
        '\u{0DB9}' => "U",
        '\u{0DBA}' => "h",
        '\u{0DBB}' => "r",
        '\u{0DBD}' => ",",
        '\u{0DC0}' => "j",
        '\u{0DC1}' => "Y",
        '\u{0DC2}' => "I",
        '\u{0DC3}' => "i",
        '\u{0DC4}' => "y",
        '\u{0DC5}' => "<",
        '\u{0DC6}' => "*",
        _ => return None,
    })
}

fn independent_fm(c: char) -> Option<&'static str> {
    Some(match c {
        '\u{0D85}' => "w",
        '\u{0D86}' => "wd",
        '\u{0D87}' => "we",
        '\u{0D88}' => "wE",
        '\u{0D89}' => "b",
        '\u{0D8A}' => "B",
        '\u{0D8B}' => "W",
        '\u{0D8C}' => "W!",
        '\u{0D8D}' => "R",
        '\u{0D8E}' => "RD",
        '\u{0D8F}' => "\u{00CC}",
        '\u{0D90}' => "\u{00CF}",
        '\u{0D91}' => "t",
        '\u{0D92}' => "ta",
        '\u{0D93}' => "ft",
        '\u{0D94}' => "T",
        '\u{0D95}' => "\u{00B4}",
        '\u{0D96}' => "T!",
        _ => return None,
    })
}

fn after_sign_fm(c: char) -> Option<&'static str> {
    Some(match c {
        '\u{0DCF}' => "d",
        '\u{0DD0}' => "e",
        '\u{0DD1}' => "E",
        '\u{0DD2}' => "s",
        '\u{0DD3}' => "S",
        '\u{0DD4}' => "q",
        '\u{0DD6}' => "Q",
        '\u{0DD8}' => "D",
        '\u{0DF2}' => "DD",
        '\u{0DDF}' => "!",
        '\u{0DCA}' => "a",
        '\u{0D82}' => "x",
        '\u{0D83}' => "#",
        _ => return None,
    })
}

fn is_after_sign(c: char) -> bool {
    after_sign_fm(c).is_some()
}

fn is_before_sign(c: char) -> bool {
    matches!(
        c,
        '\u{0DD9}' | '\u{0DDB}' | '\u{0DDA}' | '\u{0DDC}' | '\u{0DDD}' | '\u{0DDE}'
    )
}

fn before_sign_parts(c: char) -> (&'static str, Option<char>, &'static str) {
    match c {
        '\u{0DD9}' => ("f", None, ""),
        '\u{0DDB}' => ("ff", None, ""),
        '\u{0DDA}' => ("f", Some('\u{0DCA}'), "a"),
        '\u{0DDC}' => ("f", Some('\u{0DCF}'), "d"),
        '\u{0DDD}' => ("f", None, "da"),
        '\u{0DDE}' => ("f", Some('\u{0DDF}'), "!"),
        _ => ("", None, ""),
    }
}

fn precomposed(c: char, sign: char) -> Option<char> {
    Some(match (c, sign) {
        ('\u{0D9B}', '\u{0DD2}') => '\u{00C5}',
        ('\u{0D9B}', '\u{0DD3}') => '\u{00C7}',
        ('\u{0D9B}', '\u{0DCA}') => '\u{00C4}',
        ('\u{0D9E}', '\u{0DD3}') => '\u{00E3}',
        ('\u{0D9E}', '\u{0DCA}') => '\u{00D9}',
        ('\u{0DA0}', '\u{0DD2}') => '\u{00D1}',
        ('\u{0DA0}', '\u{0DD3}') => '\u{00D6}',
        ('\u{0DA0}', '\u{0DCA}') => '\u{00C9}',
        ('\u{0DA1}', '\u{0DD2}') => '\u{00FD}',
        ('\u{0DA1}', '\u{0DD3}') => '\u{00C2}',
        ('\u{0DA1}', '\u{0DCA}') => '\u{00FE}',
        ('\u{0DA2}', '\u{0DD2}') => '\u{00F0}',
        ('\u{0DA2}', '\u{0DD3}') => '\u{00D4}',
        ('\u{0DA2}', '\u{0DCA}') => '\u{00CA}',
        ('\u{0DA3}', '\u{0DD2}') => '\u{00AF}',
        ('\u{0DA3}', '\u{0DD3}') => '\u{00B0}',
        ('\u{0DA4}', '\u{0DD4}') => '\u{00FB}',
        ('\u{0DA4}', '\u{0DD6}') => '\u{00FC}',
        ('\u{0DA7}', '\u{0DD2}') => '\u{00E1}',
        ('\u{0DA7}', '\u{0DD3}') => '\u{00E0}',
        ('\u{0DA7}', '\u{0DCA}') => '\u{00DC}',
        ('\u{0DA8}', '\u{0DD2}') => '\u{00C0}',
        ('\u{0DA8}', '\u{0DD3}') => '\u{00C1}',
        ('\u{0DA9}', '\u{0DD2}') => '\u{00E4}',
        ('\u{0DA9}', '\u{0DCA}') => '\u{00E2}',
        ('\u{0DAA}', '\u{0DD2}') => '\u{00CE}',
        ('\u{0DAA}', '\u{0DD3}') => '\u{00D0}',
        ('\u{0DAB}', '\u{0DD2}') => '\u{201A}',
        ('\u{0DAB}', '\u{0DD3}') => '\u{0152}',
        ('\u{0DAC}', '\u{0DD2}') => '\u{00E7}',
        ('\u{0DAC}', '\u{0DD3}') => '\u{00E9}',
        ('\u{0DAC}', '\u{0DCA}') => '\u{00E5}',
        ('\u{0DAE}', '\u{0DD2}') => '\u{00D3}',
        ('\u{0DAE}', '\u{0DD3}') => '\u{00D2}',
        ('\u{0DAF}', '\u{0DD0}') => '\u{00B1}',
        ('\u{0DAF}', '\u{0DCF}') => '\u{00DE}',
        ('\u{0DAF}', '\u{0DD2}') => '\u{00C8}',
        ('\u{0DAF}', '\u{0DD3}') => '\u{00A7}',
        ('\u{0DAF}', '\u{0DD4}') => '\u{00FF}',
        ('\u{0DB0}', '\u{0DD2}') => '\u{00EA}',
        ('\u{0DB0}', '\u{0DD3}') => '\u{00EB}',
        ('\u{0DB0}', '\u{0DCA}') => '\u{00E8}',
        ('\u{0DB3}', '\u{0DD0}') => '\u{0192}',
        ('\u{0DB3}', '\u{0DD2}') => '\u{00A2}',
        ('\u{0DB3}', '\u{0DD3}') => '\u{00A3}',
        ('\u{0DB3}', '\u{0DD4}') => '\u{00F7}',
        ('\u{0DB3}', '\u{0DD6}') => '\u{00AA}',
        ('\u{0DB5}', '\u{0DD2}') => '\u{00DD}',
        ('\u{0DB5}', '\u{0DD3}') => '\u{00DA}',
        ('\u{0DB6}', '\u{0DD2}') => '\u{00EC}',
        ('\u{0DB6}', '\u{0DD3}') => '\u{00EE}',
        ('\u{0DB6}', '\u{0DCA}') => '\u{00ED}',
        ('\u{0DB8}', '\u{0DD2}') => '\u{00F1}',
        ('\u{0DB8}', '\u{0DD3}') => '\u{00F3}',
        ('\u{0DB8}', '\u{0DCA}') => '\u{00EF}',
        ('\u{0DB9}', '\u{0DD2}') => '\u{00F4}',
        ('\u{0DB9}', '\u{0DD3}') => '\u{00F6}',
        ('\u{0DB9}', '\u{0DCA}') => '\u{00F2}',
        ('\u{0DBB}', '\u{0DD0}') => '/',
        ('\u{0DBB}', '\u{0DD1}') => '?',
        ('\u{0DBB}', '\u{0DD2}') => '\u{00DF}',
        ('\u{0DBB}', '\u{0DD3}') => '\u{00CD}',
        ('\u{0DBB}', '\u{0DCA}') => '\u{00BE}',
        ('\u{0DBD}', '\u{0DD4}') => '\u{00A8}',
        ('\u{0DBD}', '\u{0DD6}') => '\u{00C6}',
        ('\u{0DC0}', '\u{0DD2}') => '\u{00FA}',
        ('\u{0DC0}', '\u{0DD3}') => '\u{00F9}',
        ('\u{0DC0}', '\u{0DCA}') => '\u{00F5}',
        ('\u{0DC5}', '\u{0DD4}') => '\u{00BF}',
        _ => return None,
    })
}

fn push_consonant(out: &mut String, c: char) {
    match consonant_fm(c) {
        Some(fm) => out.push_str(fm),
        None => out.push(c),
    }
}

fn emit_cons_with_sign(out: &mut String, c: char, sign: char) {
    if let Some(glyph) = precomposed(c, sign) {
        out.push(glyph);
    } else {
        push_consonant(out, c);
        match after_sign_fm(sign) {
            Some(fm) => out.push_str(fm),
            None => out.push(sign),
        }
    }
}

pub fn unicode_to_fm_abhaya(input: &str) -> String {
    let cs: Vec<char> = input.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c == RAKA
            && i + 3 < cs.len()
            && cs[i + 1] == AL_LAKUNA
            && cs[i + 2] == ZWJ
            && is_consonant(cs[i + 3])
        {
            push_consonant(&mut out, cs[i + 3]);
            out.push('_');
            i += 4;
            continue;
        }
        if is_consonant(c) {
            let j = i + 1;
            if j + 2 < cs.len()
                && cs[j] == AL_LAKUNA
                && cs[j + 1] == ZWJ
                && (cs[j + 2] == RAKA || cs[j + 2] == YANSA)
            {
                push_consonant(&mut out, c);
                out.push(if cs[j + 2] == RAKA { '%' } else { 'H' });
                i = j + 3;
                continue;
            }
            if j < cs.len() && is_before_sign(cs[j]) {
                let (before, after_sign, after_raw) = before_sign_parts(cs[j]);
                out.push_str(before);
                push_consonant(&mut out, c);
                match after_sign {
                    Some(sign) => {
                        if let Some(glyph) = precomposed(c, sign) {
                            let key_len = consonant_fm(c).map(|s| s.chars().count()).unwrap_or(1);
                            for _ in 0..key_len {
                                out.pop();
                            }
                            out.push(glyph);
                        } else {
                            out.push_str(after_raw);
                        }
                    }
                    None => out.push_str(after_raw),
                }
                i = j + 1;
                continue;
            }
            if j < cs.len() && is_after_sign(cs[j]) {
                emit_cons_with_sign(&mut out, c, cs[j]);
                i = j + 1;
                continue;
            }
            push_consonant(&mut out, c);
            i += 1;
            continue;
        }
        if let Some(fm) = independent_fm(c) {
            out.push_str(fm);
            i += 1;
            continue;
        }
        if is_before_sign(c) {
            let (before, _, after_raw) = before_sign_parts(c);
            out.push_str(before);
            out.push_str(after_raw);
            i += 1;
            continue;
        }
        if let Some(fm) = after_sign_fm(c) {
            out.push_str(fm);
            i += 1;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

/// In the FM Abhaya font most ASCII punctuation slots draw Sinhala glyphs
/// (`,` is ල, `.` is ග, `;` is ත ...), so a punctuation key pressed in FM
/// mode would show a letter, not the mark. This maps a punctuation character
/// the user wants to the character whose FM Abhaya glyph *is* that mark.
/// It is the inverse of the punctuation rows in the Pandukabhaya FM Abhaya
/// mapping (`mappings.singles`, FM -> Unicode). Characters with no entry
/// (digits, `-`, space, ...) already render as themselves in the font.
pub fn fm_punctuation(c: char) -> Option<char> {
    Some(match c {
        ',' => '"',
        '.' => '\'',
        '?' => '@',
        ':' => '(',
        ';' => '\u{00A6}',
        '!' => '\u{00E6}',
        '(' => '^',
        ')' => '&',
        '"' => '\u{02DC}',
        '/' => '$',
        '%' => ']',
        '=' => '}',
        '+' => '\u{00AC}',
        '{' => '\u{2122}',
        '}' => '\u{0161}',
        _ => return None,
    })
}

pub fn render_diff(on_screen_legacy: &str, new_unicode_word: &str) -> EditAction {
    let target = unicode_to_fm_abhaya(new_unicode_word);
    let old_chars: Vec<char> = on_screen_legacy.chars().collect();
    let new_chars: Vec<char> = target.chars().collect();
    let mut common = 0;
    while common < old_chars.len()
        && common < new_chars.len()
        && old_chars[common] == new_chars[common]
    {
        common += 1;
    }
    EditAction {
        backspaces: old_chars.len() - common,
        insert: new_chars[common..].iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::phonetic::PhoneticEngine;
    use crate::engine::ScreenSim;

    const PART_B: &[(&str, &str)] = &[
        ("\u{0DB8}\u{0DB8}", "uu"),
        ("\u{0D94}\u{0DBA}", "Th"),
        ("\u{0D85}\u{0DB4}\u{0DD2}", "wms"),
        ("\u{0D85}\u{0DB8}\u{0DCA}\u{0DB8}\u{0DCF}", "w\u{00EF}ud"),
        ("\u{0DAD}\u{0DCF}\u{0DAD}\u{0DCA}\u{0DAD}\u{0DCF}", ";d;a;d"),
        ("\u{0D9A}\u{0DCA}\u{200D}\u{0DBB}\u{0DB8}", "l%u"),
        ("\u{0DBD}\u{0D82}\u{0D9A}\u{0DCF}", ",xld"),
        ("\u{0DBD}\u{0D82}\u{0D9A}\u{0DCF}\u{0DC0}", ",xldj"),
        (
            "\u{0DC0}\u{0DD2}\u{0DAF}\u{0DCA}\u{200D}\u{0DBA}\u{0DCF}\u{0DC0}",
            "\u{00FA}oHdj",
        ),
        ("\u{0DC3}\u{0DD2}\u{0D82}\u{0DC4}\u{0DBD}", "isxy,"),
        (
            "\u{0DC1}\u{0DCA}\u{200D}\u{0DBB}\u{0DD3} \u{0DBD}\u{0D82}\u{0D9A}\u{0DCF}",
            "Y%S ,xld",
        ),
        (
            "\u{0D86}\u{0DBA}\u{0DD4}\u{0DB6}\u{0DDD}\u{0DC0}\u{0DB1}\u{0DCA}",
            "wdhqfndajka",
        ),
        (
            "\u{0D89}\u{0DC3}\u{0DCA}\u{0DAD}\u{0DD4}\u{0DAD}\u{0DD2}",
            "bia;q;s",
        ),
        (
            "\u{0D9A}\u{0DDC}\u{0DC4}\u{0DDC}\u{0DB8}\u{0DAF}",
            "fldfyduo",
        ),
        ("\u{0DB4}\u{0DB1}\u{0DCA}\u{0DC3}\u{0DBD}", "mkai,"),
        ("\u{0D9C}\u{0DB8}\u{0DB1}", ".uk"),
        ("\u{0DB4}\u{0DCF}\u{0DC3}\u{0DBD}", "mdi,"),
        (
            "\u{0D9A}\u{0DCF}\u{0DB1}\u{0DCA}\u{0DAD}\u{0DCF}\u{0DC0}",
            "ldka;dj",
        ),
        ("\u{0DC1}\u{0DD2}\u{0DC2}\u{0DCA}\u{200D}\u{0DBA}", "YsIH"),
        ("\u{0DB0}\u{0DBB}\u{0DCA}\u{200D}\u{0DB8}", "Ou_"),
        ("\u{0D9A}\u{0DBB}\u{0DCA}\u{200D}\u{0DB8}", "lu_"),
        ("\u{0DB8}\u{0DBD}\u{0DCA}", "u,a"),
        ("\u{0DBB}\u{0DAD}\u{0DD4}", "r;q"),
        ("\u{0D9A}\u{0DDC}\u{0DC5}\u{0DB9}", "fld<U"),
        ("\u{0D9C}\u{0D82}\u{0D9C}\u{0DCF}", ".x.d"),
        ("\u{0DC3}\u{0DD6}\u{0DBB}\u{0DD2}\u{0DBA}", "iQ\u{00DF}h"),
        ("\u{0DB4}\u{0DD6}\u{0DA2}\u{0DCF}", "mQcd"),
        ("\u{0D8D}\u{0DC2}\u{0DD2}", "RIs"),
        ("\u{0D86}\u{0DB1}\u{0DB1}\u{0DCA}\u{0DAF}", "wdkkao"),
        ("\u{0DC3}\u{0DD4}\u{0DB1}\u{0DCA}\u{0DAF}\u{0DBB}", "iqkaor"),
        ("\u{0DB4}\u{0DD9}\u{0DBD}", "fm,"),
        ("\u{0DB8}\u{0DDA}", "f\u{00EF}"),
        ("\u{0D91}\u{0D9A}\u{0D9A}\u{0DCA}", "tlla"),
    ];

    #[test]
    fn part_b_verified_pairs() {
        for (unicode, expected) in PART_B {
            assert_eq!(
                unicode_to_fm_abhaya(unicode),
                *expected,
                "input: {}",
                unicode
            );
        }
    }

    const PART_C_FM: &[(&str, &[(&str, &str)])] = &[
        (
            "krama",
            &[
                ("k", "la"),
                ("kr", "la\u{00BE}"),
                ("kra", "l%"),
                ("kram", "l%\u{00EF}"),
                ("krama", "l%u"),
            ],
        ),
        (
            "sinhala",
            &[
                ("s", "ia"),
                ("si", "is"),
                ("sin", "iska"),
                ("sinh", "isxya"),
                ("sinha", "isxy"),
                ("sinhal", "isxy,a"),
                ("sinhala", "isxy,"),
            ],
        ),
        (
            "shrii",
            &[
                ("s", "ia"),
                ("sh", "Ya"),
                ("shr", "Ya\u{00BE}"),
                ("shri", "Y%s"),
                ("shrii", "Y%S"),
            ],
        ),
        (
            "ammaa",
            &[
                ("a", "w"),
                ("am", "w\u{00EF}"),
                ("amm", "w\u{00EF}\u{00EF}"),
                ("amma", "w\u{00EF}u"),
                ("ammaa", "w\u{00EF}ud"),
            ],
        ),
        (
            "vidhyaava",
            &[
                ("v", "\u{00F5}"),
                ("vi", "\u{00FA}"),
                ("vid", "\u{00FA}\u{00E2}"),
                ("vidh", "\u{00FA}oa"),
                ("vidhy", "\u{00FA}oaha"),
                ("vidhya", "\u{00FA}oH"),
                ("vidhyaa", "\u{00FA}oHd"),
                ("vidhyaav", "\u{00FA}oHd\u{00F5}"),
                ("vidhyaava", "\u{00FA}oHdj"),
            ],
        ),
        (
            "Dharma",
            &[
                ("D", "Va"),
                ("Dh", "\u{00E8}"),
                ("Dha", "O"),
                ("Dhar", "O\u{00BE}"),
                ("Dharma", "Ou_"),
            ],
        ),
    ];

    #[test]
    fn render_diff_prefix_screens_match_part_c() {
        for (word, prefixes) in PART_C_FM {
            let mut sim = ScreenSim::new();
            for (prefix, expected_fm) in *prefixes {
                let unicode_word = PhoneticEngine::convert(prefix);
                let action = render_diff(sim.as_str(), &unicode_word);
                sim.apply(&action);
                assert_eq!(
                    sim.as_str(),
                    *expected_fm,
                    "FM prefix: {} of {}",
                    prefix,
                    word
                );
            }
        }
    }

    #[test]
    fn render_diff_property_all_prefixes() {
        let words = [
            "krama",
            "sinhala",
            "Dharma",
            "shrii",
            "ammaa",
            "vidhyaava",
            "lankaa",
            "kohomadha",
            "isthuthi",
            "aayuboowan",
        ];
        for word in words {
            let mut sim = ScreenSim::new();
            for k in 0..=word.len() {
                let prefix = &word[..k];
                let unicode_word = PhoneticEngine::convert(prefix);
                let action = render_diff(sim.as_str(), &unicode_word);
                sim.apply(&action);
                assert_eq!(
                    sim.as_str(),
                    unicode_to_fm_abhaya(&unicode_word),
                    "FM screen mismatch at prefix {:?} of {:?}",
                    prefix,
                    word
                );
            }
            assert_eq!(
                sim.as_str(),
                unicode_to_fm_abhaya(&PhoneticEngine::convert(word))
            );
        }
    }

    #[test]
    fn kombuwa_reorders_with_backspaces_in_fm_chars() {
        let action = render_diff("la", &PhoneticEngine::convert("ke"));
        assert_eq!(action.backspaces, 2);
        assert_eq!(action.insert, "fl");
    }

    #[test]
    fn unknown_chars_pass_through() {
        assert_eq!(unicode_to_fm_abhaya("a1! \u{0DB8}"), "a1! u");
    }

    #[test]
    fn fm_punctuation_maps_to_glyphs_that_draw_the_mark() {
        // Inverse of the Pandukabhaya singles rows: in the FM font the
        // right-hand character is the one drawn as the left-hand mark.
        let cases: &[(char, char)] = &[
            (',', '"'),
            ('.', '\''),
            ('?', '@'),
            (':', '('),
            (';', '\u{00A6}'),
            ('!', '\u{00E6}'),
            ('(', '^'),
            (')', '&'),
            ('"', '\u{02DC}'),
            ('/', '$'),
            ('%', ']'),
            ('=', '}'),
            ('+', '\u{00AC}'),
            ('{', '\u{2122}'),
            ('}', '\u{0161}'),
        ];
        for (want, fm) in cases {
            assert_eq!(fm_punctuation(*want), Some(*fm), "punct: {}", want);
        }
        // Digits, minus and letters need no substitution in FM mode.
        assert_eq!(fm_punctuation('5'), None);
        assert_eq!(fm_punctuation('-'), None);
        assert_eq!(fm_punctuation('a'), None);
    }
}
