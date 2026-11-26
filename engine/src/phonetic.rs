use super::EditAction;

const ZWJ: char = '\u{200D}';
const AL_LAKUNA: char = '\u{0DCA}';
const ANUSVARA: char = '\u{0D82}';
const RAKA: char = '\u{0DBB}';
const YANSA: char = '\u{0DBA}';

#[derive(Clone, Copy)]
enum Token {
    Vowel {
        independent: char,
        sign: Option<char>,
    },
    Cons(char),
    Special(char),
}

const KEYS: &[(&str, Token)] = &[
    // Key combinations follow the phonetic layout spec adopted by the
    // product owner on 2026-11-26 (see data/keymap.md): doubling a vowel
    // lengthens it, capitals give the retroflex/cerebral and special
    // letters, dental aspirates take a double h (thha, dhha), the
    // naasikya letters take a z prefix, and `ch` = ච per the owner's
    // explicit decision (the spec's `cha` works too — it is ch + a).
    ("chh", Token::Cons('\u{0DA1}')),
    ("thh", Token::Cons('\u{0DAE}')),
    ("dhh", Token::Cons('\u{0DB0}')),
    ("zdh", Token::Cons('\u{0DB3}')),
    (
        "ruu",
        Token::Vowel {
            independent: '\u{0D8E}',
            sign: Some('\u{0DF2}'),
        },
    ),
    ("kh", Token::Cons('\u{0D9B}')),
    ("gh", Token::Cons('\u{0D9D}')),
    ("zg", Token::Cons('\u{0D9F}')),
    ("zj", Token::Cons('\u{0DA6}')),
    ("zd", Token::Cons('\u{0DAC}')),
    ("zq", Token::Cons('\u{0DB3}')),
    ("zk", Token::Cons('\u{0DA4}')),
    ("zh", Token::Cons('\u{0DA5}')),
    ("ch", Token::Cons('\u{0DA0}')),
    ("th", Token::Cons('\u{0DAD}')),
    ("dh", Token::Cons('\u{0DAF}')),
    ("ph", Token::Cons('\u{0DB5}')),
    ("bh", Token::Cons('\u{0DB7}')),
    ("sh", Token::Cons('\u{0DC1}')),
    ("Sh", Token::Cons('\u{0DC2}')),
    ("zn", Token::Special(ANUSVARA)),
    (
        "aa",
        Token::Vowel {
            independent: '\u{0D86}',
            sign: Some('\u{0DCF}'),
        },
    ),
    (
        "Aa",
        Token::Vowel {
            independent: '\u{0D88}',
            sign: Some('\u{0DD1}'),
        },
    ),
    (
        "AA",
        Token::Vowel {
            independent: '\u{0D88}',
            sign: Some('\u{0DD1}'),
        },
    ),
    (
        "ai",
        Token::Vowel {
            independent: '\u{0D93}',
            sign: Some('\u{0DDB}'),
        },
    ),
    (
        "au",
        Token::Vowel {
            independent: '\u{0D96}',
            sign: Some('\u{0DDE}'),
        },
    ),
    (
        "ou",
        Token::Vowel {
            independent: '\u{0D96}',
            sign: Some('\u{0DDE}'),
        },
    ),
    (
        "ee",
        Token::Vowel {
            independent: '\u{0D92}',
            sign: Some('\u{0DDA}'),
        },
    ),
    (
        "ii",
        Token::Vowel {
            independent: '\u{0D8A}',
            sign: Some('\u{0DD3}'),
        },
    ),
    (
        "oo",
        Token::Vowel {
            independent: '\u{0D95}',
            sign: Some('\u{0DDD}'),
        },
    ),
    (
        "uu",
        Token::Vowel {
            independent: '\u{0D8C}',
            sign: Some('\u{0DD6}'),
        },
    ),
    (
        "Ru",
        Token::Vowel {
            independent: '\u{0D8E}',
            sign: Some('\u{0DF2}'),
        },
    ),
    (
        "ru",
        Token::Vowel {
            independent: '\u{0D8D}',
            sign: Some('\u{0DD8}'),
        },
    ),
    (
        "a",
        Token::Vowel {
            independent: '\u{0D85}',
            sign: None,
        },
    ),
    (
        "A",
        Token::Vowel {
            independent: '\u{0D87}',
            sign: Some('\u{0DD0}'),
        },
    ),
    (
        "e",
        Token::Vowel {
            independent: '\u{0D91}',
            sign: Some('\u{0DD9}'),
        },
    ),
    (
        "i",
        Token::Vowel {
            independent: '\u{0D89}',
            sign: Some('\u{0DD2}'),
        },
    ),
    (
        "o",
        Token::Vowel {
            independent: '\u{0D94}',
            sign: Some('\u{0DDC}'),
        },
    ),
    (
        "u",
        Token::Vowel {
            independent: '\u{0D8B}',
            sign: Some('\u{0DD4}'),
        },
    ),
    (
        "R",
        Token::Vowel {
            independent: '\u{0D8D}',
            sign: None,
        },
    ),
    ("k", Token::Cons('\u{0D9A}')),
    ("g", Token::Cons('\u{0D9C}')),
    ("c", Token::Cons('\u{0DA0}')),
    ("j", Token::Cons('\u{0DA2}')),
    ("t", Token::Cons('\u{0DA7}')),
    ("T", Token::Cons('\u{0DA8}')),
    ("d", Token::Cons('\u{0DA9}')),
    ("D", Token::Cons('\u{0DAA}')),
    ("N", Token::Cons('\u{0DAB}')),
    ("n", Token::Cons('\u{0DB1}')),
    ("p", Token::Cons('\u{0DB4}')),
    ("b", Token::Cons('\u{0DB6}')),
    ("B", Token::Cons('\u{0DB9}')),
    ("m", Token::Cons('\u{0DB8}')),
    ("y", Token::Cons('\u{0DBA}')),
    ("r", Token::Cons('\u{0DBB}')),
    ("l", Token::Cons('\u{0DBD}')),
    ("v", Token::Cons('\u{0DC0}')),
    ("w", Token::Cons('\u{0DC0}')),
    ("s", Token::Cons('\u{0DC3}')),
    ("S", Token::Cons('\u{0DC2}')),
    ("h", Token::Cons('\u{0DC4}')),
    ("L", Token::Cons('\u{0DC5}')),
    ("f", Token::Cons('\u{0DC6}')),
    ("q", Token::Cons('\u{0DAF}')),
    ("x", Token::Special(ANUSVARA)),
    ("X", Token::Special('\u{0D9E}')),
    ("H", Token::Special('\u{0D83}')),
];

fn match_key(s: &str) -> Option<(&str, Token)> {
    for len in [3usize, 2, 1] {
        if s.len() >= len && s.is_char_boundary(len) {
            let cand = &s[..len];
            if let Some((_, token)) = KEYS.iter().find(|(k, _)| *k == cand) {
                return Some((cand, *token));
            }
        }
    }
    None
}

fn diff(old: &str, new: &str) -> EditAction {
    let old_chars: Vec<char> = old.chars().collect();
    let new_chars: Vec<char> = new.chars().collect();
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

#[derive(Default)]
pub struct PhoneticEngine {
    roman: String,
    emitted: String,
}

impl PhoneticEngine {
    pub fn accepts(c: char) -> bool {
        c.is_ascii_alphabetic() && KEYS.iter().any(|(k, _)| k.starts_with(c))
    }

    pub fn convert(roman: &str) -> String {
        let mut tokens: Vec<(&str, Token)> = Vec::new();
        let mut i = 0;
        while i < roman.len() {
            match match_key(&roman[i..]) {
                Some((key, token)) => {
                    tokens.push((key, token));
                    i += key.len();
                }
                None => {
                    // Skip a whole character: advancing one byte here used to
                    // panic (or slice mid-character) on non-ASCII input.
                    i += roman[i..].chars().next().map_or(1, |c| c.len_utf8());
                }
            }
        }

        let mut out = String::new();
        let mut pending: Option<(char, &str, bool)> = None;
        for (idx, (roman_tok, token)) in tokens.iter().enumerate() {
            match token {
                Token::Vowel { independent, sign } => {
                    match &pending {
                        Some((p, _, emitted)) => {
                            if !emitted {
                                out.push(*p);
                            }
                            if let Some(s) = sign {
                                out.push(*s);
                            }
                        }
                        None => out.push(*independent),
                    }
                    pending = None;
                }
                Token::Special(c) => {
                    if let Some((p, _, false)) = pending.take() {
                        out.push(p);
                        out.push(AL_LAKUNA);
                    }
                    out.push(*c);
                }
                Token::Cons(c) => {
                    if let Some((p, pending_tok, emitted)) = pending.take() {
                        let next_is_vowel =
                            matches!(tokens.get(idx + 1), Some((_, Token::Vowel { .. })));
                        if !emitted && *roman_tok == "r" && next_is_vowel {
                            out.push(p);
                            out.push(AL_LAKUNA);
                            out.push(ZWJ);
                            out.push(RAKA);
                            pending = Some((RAKA, "conj", true));
                        } else if !emitted && *roman_tok == "y" && next_is_vowel {
                            out.push(p);
                            out.push(AL_LAKUNA);
                            out.push(ZWJ);
                            out.push(YANSA);
                            pending = Some((YANSA, "conj", true));
                        } else if !emitted
                            && pending_tok == "n"
                            && matches!(
                                c,
                                '\u{0D9A}'
                                    | '\u{0D9B}'
                                    | '\u{0D9C}'
                                    | '\u{0D9D}'
                                    | '\u{0D9E}'
                                    | '\u{0D9F}'
                                    | '\u{0DA0}'
                                    | '\u{0DA1}'
                                    | '\u{0DA2}'
                                    | '\u{0DA3}'
                                    | '\u{0DA4}'
                                    | '\u{0DC4}'
                            )
                        {
                            out.push(ANUSVARA);
                            pending = Some((*c, roman_tok, false));
                        } else if !emitted && pending_tok == "r" {
                            out.push(RAKA);
                            out.push(AL_LAKUNA);
                            out.push(ZWJ);
                            pending = Some((*c, roman_tok, false));
                        } else {
                            if !emitted {
                                out.push(p);
                                out.push(AL_LAKUNA);
                            }
                            pending = Some((*c, roman_tok, false));
                        }
                    } else {
                        pending = Some((*c, roman_tok, false));
                    }
                }
            }
        }
        if let Some((p, _, emitted)) = pending {
            if !emitted {
                out.push(p);
            }
            out.push(AL_LAKUNA);
        }
        out
    }

    pub fn push_char(&mut self, c: char) -> EditAction {
        if !Self::accepts(c) {
            self.reset();
            return EditAction {
                backspaces: 0,
                insert: String::new(),
            };
        }
        self.roman.push(c);
        self.refresh()
    }

    pub fn backspace(&mut self) -> EditAction {
        if self.roman.is_empty() {
            return EditAction {
                backspaces: 0,
                insert: String::new(),
            };
        }
        self.roman.pop();
        self.refresh()
    }

    pub fn reset(&mut self) {
        self.roman.clear();
        self.emitted.clear();
    }

    pub fn is_active(&self) -> bool {
        !self.roman.is_empty()
    }

    fn refresh(&mut self) -> EditAction {
        let new = Self::convert(&self.roman);
        let action = diff(&self.emitted, &new);
        self.emitted = new;
        action
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ScreenSim;

    fn type_str(engine: &mut PhoneticEngine, sim: &mut ScreenSim, s: &str) {
        for c in s.chars() {
            let action = engine.push_char(c);
            sim.apply(&action);
            if !PhoneticEngine::accepts(c) {
                sim.apply(&EditAction {
                    backspaces: 0,
                    insert: c.to_string(),
                });
            }
        }
    }

    fn screen_of(s: &str) -> String {
        let mut engine = PhoneticEngine::default();
        let mut sim = ScreenSim::new();
        type_str(&mut engine, &mut sim, s);
        sim.as_str().to_string()
    }

    const PART_A: &[(&str, &str)] = &[
        ("mama", "\u{0DB8}\u{0DB8}"),
        ("oya", "\u{0D94}\u{0DBA}"),
        ("api", "\u{0D85}\u{0DB4}\u{0DD2}"),
        ("ammaa", "\u{0D85}\u{0DB8}\u{0DCA}\u{0DB8}\u{0DCF}"),
        (
            "thaaththaa",
            "\u{0DAD}\u{0DCF}\u{0DAD}\u{0DCA}\u{0DAD}\u{0DCF}",
        ),
        ("krama", "\u{0D9A}\u{0DCA}\u{200D}\u{0DBB}\u{0DB8}"),
        ("lankaa", "\u{0DBD}\u{0D82}\u{0D9A}\u{0DCF}"),
        ("lankaava", "\u{0DBD}\u{0D82}\u{0D9A}\u{0DCF}\u{0DC0}"),
        (
            "vidhyaava",
            "\u{0DC0}\u{0DD2}\u{0DAF}\u{0DCA}\u{200D}\u{0DBA}\u{0DCF}\u{0DC0}",
        ),
        ("sinhala", "\u{0DC3}\u{0DD2}\u{0D82}\u{0DC4}\u{0DBD}"),
        (
            "shrii lankaa",
            "\u{0DC1}\u{0DCA}\u{200D}\u{0DBB}\u{0DD3} \u{0DBD}\u{0D82}\u{0D9A}\u{0DCF}",
        ),
        (
            "aayuboowan",
            "\u{0D86}\u{0DBA}\u{0DD4}\u{0DB6}\u{0DDD}\u{0DC0}\u{0DB1}\u{0DCA}",
        ),
        (
            "isthuthi",
            "\u{0D89}\u{0DC3}\u{0DCA}\u{0DAD}\u{0DD4}\u{0DAD}\u{0DD2}",
        ),
        (
            "kohomadha",
            "\u{0D9A}\u{0DDC}\u{0DC4}\u{0DDC}\u{0DB8}\u{0DAF}",
        ),
        ("pansala", "\u{0DB4}\u{0DB1}\u{0DCA}\u{0DC3}\u{0DBD}"),
        ("gamana", "\u{0D9C}\u{0DB8}\u{0DB1}"),
        ("paasala", "\u{0DB4}\u{0DCF}\u{0DC3}\u{0DBD}"),
        (
            "kaanthaawa",
            "\u{0D9A}\u{0DCF}\u{0DB1}\u{0DCA}\u{0DAD}\u{0DCF}\u{0DC0}",
        ),
        (
            "shiShya",
            "\u{0DC1}\u{0DD2}\u{0DC2}\u{0DCA}\u{200D}\u{0DBA}",
        ),
        ("dhharma", "\u{0DB0}\u{0DBB}\u{0DCA}\u{200D}\u{0DB8}"),
        ("karma", "\u{0D9A}\u{0DBB}\u{0DCA}\u{200D}\u{0DB8}"),
        ("mal", "\u{0DB8}\u{0DBD}\u{0DCA}"),
        ("rathu", "\u{0DBB}\u{0DAD}\u{0DD4}"),
        (
            "koLamba",
            "\u{0D9A}\u{0DDC}\u{0DC5}\u{0DB8}\u{0DCA}\u{0DB6}",
        ),
        ("gangaa", "\u{0D9C}\u{0D82}\u{0D9C}\u{0DCF}"),
        ("suuriya", "\u{0DC3}\u{0DD6}\u{0DBB}\u{0DD2}\u{0DBA}"),
        ("puujaa", "\u{0DB4}\u{0DD6}\u{0DA2}\u{0DCF}"),
        ("Ru", "\u{0D8E}"),
        ("aanandha", "\u{0D86}\u{0DB1}\u{0DB1}\u{0DCA}\u{0DAF}"),
        (
            "sundhara",
            "\u{0DC3}\u{0DD4}\u{0DB1}\u{0DCA}\u{0DAF}\u{0DBB}",
        ),
        ("pela", "\u{0DB4}\u{0DD9}\u{0DBD}"),
        ("mee", "\u{0DB8}\u{0DDA}"),
        ("ekak", "\u{0D91}\u{0D9A}\u{0D9A}\u{0DCA}"),
    ];

    const PART_D_KEYS: &[(&str, &str)] = &[
        // Every row of the adopted phonetic spec (data/keymap.md).
        ("ta", "\u{0DA7}"),
        ("tha", "\u{0DAD}"),
        ("thha", "\u{0DAE}"),
        ("da", "\u{0DA9}"),
        ("dha", "\u{0DAF}"),
        ("qa", "\u{0DAF}"),
        ("dhha", "\u{0DB0}"),
        ("Ta", "\u{0DA8}"),
        ("Da", "\u{0DAA}"),
        ("Na", "\u{0DAB}"),
        ("La", "\u{0DC5}"),
        ("Lu", "\u{0DC5}\u{0DD4}"),
        ("sha", "\u{0DC1}"),
        ("Sha", "\u{0DC2}"),
        ("Sa", "\u{0DC2}"),
        ("va", "\u{0DC0}"),
        ("wa", "\u{0DC0}"),
        ("fa", "\u{0DC6}"),
        ("zga", "\u{0D9F}"),
        ("zja", "\u{0DA6}"),
        ("zda", "\u{0DAC}"),
        ("zdha", "\u{0DB3}"),
        ("zqa", "\u{0DB3}"),
        ("zka", "\u{0DA4}"),
        ("zha", "\u{0DA5}"),
        ("Ba", "\u{0DB9}"),
        ("cha", "\u{0DA0}"),
        ("ca", "\u{0DA0}"),
        ("x", "\u{0D82}"),
        ("H", "\u{0D83}"),
        ("R", "\u{0D8D}"),
        ("Ru", "\u{0D8E}"),
        ("A", "\u{0D87}"),
        ("Aa", "\u{0D88}"),
        ("AA", "\u{0D88}"),
        ("kA", "\u{0D9A}\u{0DD0}"),
        ("kAa", "\u{0D9A}\u{0DD1}"),
        ("kru", "\u{0D9A}\u{0DD8}"),
        ("kruu", "\u{0D9A}\u{0DF2}"),
        ("kai", "\u{0D9A}\u{0DDB}"),
        ("kau", "\u{0D9A}\u{0DDE}"),
        ("kou", "\u{0D9A}\u{0DDE}"),
        ("kax", "\u{0D9A}\u{0D82}"),
        ("kazn", "\u{0D9A}\u{0D82}"),
        ("kaX", "\u{0D9A}\u{0D9E}"),
        ("kaH", "\u{0D9A}\u{0D83}"),
        ("kya", "\u{0D9A}\u{0DCA}\u{200D}\u{0DBA}"),
    ];

    #[test]
    fn part_a_verified_pairs() {
        for (roman, expected) in PART_A {
            assert_eq!(screen_of(roman), *expected, "roman: {}", roman);
        }
    }

    #[test]
    fn part_d_single_keys() {
        for (roman, expected) in PART_D_KEYS {
            assert_eq!(screen_of(roman), *expected, "roman: {}", roman);
        }
    }

    #[test]
    fn aspirates_and_long_vowels_regressions() {
        // Every mahapraana (aspirated) consonant with a plain and a long vowel.
        let cases: &[(&str, &str)] = &[
            ("kha", "\u{0D9B}"),
            ("khaa", "\u{0D9B}\u{0DCF}"),
            ("khii", "\u{0D9B}\u{0DD3}"),
            ("gha", "\u{0D9D}"),
            ("ghoo", "\u{0D9D}\u{0DDD}"),
            ("cha", "\u{0DA0}"),
            ("chha", "\u{0DA1}"),
            ("thha", "\u{0DAE}"),
            ("dhha", "\u{0DB0}"),
            ("Ta", "\u{0DA8}"),
            ("Da", "\u{0DAA}"),
            ("pha", "\u{0DB5}"),
            ("bha", "\u{0DB7}"),
            ("bhaava", "\u{0DB7}\u{0DCF}\u{0DC0}"),
            // Long vocalic per spec: ඎ is "Ru", the sign ෲ is "ruu"
            // after a consonant (kruu), ඍ is "R", ෘ is "ru" (kru).
            ("Ru", "\u{0D8E}"),
            ("R", "\u{0D8D}"),
            ("kru", "\u{0D9A}\u{0DD8}"),
            ("kruu", "\u{0D9A}\u{0DF2}"),
            // Anusvara before aspirates and palatals, not only before k/g/h.
            ("sangha", "\u{0DC3}\u{0D82}\u{0D9D}"),
            ("sancaara", "\u{0DC3}\u{0D82}\u{0DA0}\u{0DCF}\u{0DBB}"),
            ("punchi", "\u{0DB4}\u{0DD4}\u{0D82}\u{0DA0}\u{0DD2}"),
        ];
        for (roman, expected) in cases {
            assert_eq!(screen_of(roman), *expected, "roman: {}", roman);
        }
    }

    #[test]
    fn convert_never_panics_on_non_ascii_input() {
        // Public convert() must tolerate arbitrary text, not only the ASCII
        // letters the streaming engine feeds it.
        let _ = PhoneticEngine::convert("mama සිංහල \u{1F600} test");
        assert_eq!(PhoneticEngine::convert(""), "");
    }

    #[test]
    fn passthrough_chars_reset_and_pass_through() {
        assert_eq!(screen_of("mama 123"), "\u{0DB8}\u{0DB8} 123");
        assert_eq!(
            screen_of("mama, oya."),
            "\u{0DB8}\u{0DB8}, \u{0D94}\u{0DBA}."
        );
        assert_eq!(screen_of(""), "");
    }

    const PART_C: &[(&str, &[(&str, &str)])] = &[
        (
            "krama",
            &[
                ("k", "\u{0D9A}\u{0DCA}"),
                ("kr", "\u{0D9A}\u{0DCA}\u{0DBB}\u{0DCA}"),
                ("kra", "\u{0D9A}\u{0DCA}\u{200D}\u{0DBB}"),
                ("kram", "\u{0D9A}\u{0DCA}\u{200D}\u{0DBB}\u{0DB8}\u{0DCA}"),
                ("krama", "\u{0D9A}\u{0DCA}\u{200D}\u{0DBB}\u{0DB8}"),
            ],
        ),
        (
            "sinhala",
            &[
                ("s", "\u{0DC3}\u{0DCA}"),
                ("si", "\u{0DC3}\u{0DD2}"),
                ("sin", "\u{0DC3}\u{0DD2}\u{0DB1}\u{0DCA}"),
                ("sinh", "\u{0DC3}\u{0DD2}\u{0D82}\u{0DC4}\u{0DCA}"),
                ("sinha", "\u{0DC3}\u{0DD2}\u{0D82}\u{0DC4}"),
                ("sinhal", "\u{0DC3}\u{0DD2}\u{0D82}\u{0DC4}\u{0DBD}\u{0DCA}"),
                ("sinhala", "\u{0DC3}\u{0DD2}\u{0D82}\u{0DC4}\u{0DBD}"),
            ],
        ),
        (
            "dhharma",
            &[
                ("d", "\u{0DA9}\u{0DCA}"),
                ("dh", "\u{0DAF}\u{0DCA}"),
                ("dhh", "\u{0DB0}\u{0DCA}"),
                ("dhha", "\u{0DB0}"),
                ("dhhar", "\u{0DB0}\u{0DBB}\u{0DCA}"),
                ("dhharm", "\u{0DB0}\u{0DBB}\u{0DCA}\u{200D}\u{0DB8}\u{0DCA}"),
                ("dhharma", "\u{0DB0}\u{0DBB}\u{0DCA}\u{200D}\u{0DB8}"),
            ],
        ),
        (
            "shrii",
            &[
                ("s", "\u{0DC3}\u{0DCA}"),
                ("sh", "\u{0DC1}\u{0DCA}"),
                ("shr", "\u{0DC1}\u{0DCA}\u{0DBB}\u{0DCA}"),
                ("shri", "\u{0DC1}\u{0DCA}\u{200D}\u{0DBB}\u{0DD2}"),
                ("shrii", "\u{0DC1}\u{0DCA}\u{200D}\u{0DBB}\u{0DD3}"),
            ],
        ),
        (
            "ammaa",
            &[
                ("a", "\u{0D85}"),
                ("am", "\u{0D85}\u{0DB8}\u{0DCA}"),
                ("amm", "\u{0D85}\u{0DB8}\u{0DCA}\u{0DB8}\u{0DCA}"),
                ("amma", "\u{0D85}\u{0DB8}\u{0DCA}\u{0DB8}"),
                ("ammaa", "\u{0D85}\u{0DB8}\u{0DCA}\u{0DB8}\u{0DCF}"),
            ],
        ),
        (
            "vidhyaava",
            &[
                ("v", "\u{0DC0}\u{0DCA}"),
                ("vi", "\u{0DC0}\u{0DD2}"),
                ("vid", "\u{0DC0}\u{0DD2}\u{0DA9}\u{0DCA}"),
                ("vidh", "\u{0DC0}\u{0DD2}\u{0DAF}\u{0DCA}"),
                ("vidhy", "\u{0DC0}\u{0DD2}\u{0DAF}\u{0DCA}\u{0DBA}\u{0DCA}"),
                ("vidhya", "\u{0DC0}\u{0DD2}\u{0DAF}\u{0DCA}\u{200D}\u{0DBA}"),
                (
                    "vidhyaa",
                    "\u{0DC0}\u{0DD2}\u{0DAF}\u{0DCA}\u{200D}\u{0DBA}\u{0DCF}",
                ),
                (
                    "vidhyaav",
                    "\u{0DC0}\u{0DD2}\u{0DAF}\u{0DCA}\u{200D}\u{0DBA}\u{0DCF}\u{0DC0}\u{0DCA}",
                ),
                (
                    "vidhyaava",
                    "\u{0DC0}\u{0DD2}\u{0DAF}\u{0DCA}\u{200D}\u{0DBA}\u{0DCF}\u{0DC0}",
                ),
            ],
        ),
    ];

    #[test]
    fn part_c_prefix_screens_streaming() {
        for (word, prefixes) in PART_C {
            let mut engine = PhoneticEngine::default();
            let mut sim = ScreenSim::new();
            let mut prev_len = 0;
            for (prefix, expected) in *prefixes {
                for c in prefix[prev_len..].chars() {
                    let action = engine.push_char(c);
                    sim.apply(&action);
                }
                prev_len = prefix.len();
                assert_eq!(sim.as_str(), *expected, "prefix: {} of {}", prefix, word);
            }
        }
    }

    #[test]
    fn backspace_undoes_last_roman_key() {
        for (word, prefixes) in PART_C {
            let mut engine = PhoneticEngine::default();
            let mut sim = ScreenSim::new();
            for c in word.chars() {
                let action = engine.push_char(c);
                sim.apply(&action);
            }
            let full = sim.as_str().to_string();
            assert_eq!(full, prefixes.last().unwrap().1);
            for (prefix, expected) in prefixes.iter().rev().skip(1) {
                let action = engine.backspace();
                sim.apply(&action);
                assert_eq!(sim.as_str(), *expected, "after backspace from {}", prefix);
            }
            let action = engine.backspace();
            sim.apply(&action);
            assert_eq!(sim.as_str(), "");
        }
    }

    #[test]
    fn no_stray_zwj_or_dangling_al_lakuna_after_backspace() {
        for (word, _) in PART_C {
            let mut engine = PhoneticEngine::default();
            let mut sim = ScreenSim::new();
            for c in word.chars() {
                let action = engine.push_char(c);
                sim.apply(&action);
            }
            for _ in 0..word.chars().count() {
                let action = engine.backspace();
                sim.apply(&action);
                let s = sim.as_str();
                assert!(!s.ends_with(ZWJ), "trailing ZWJ in {:?}", s);
                assert!(
                    !s.ends_with(&format!("{}{}", AL_LAKUNA, ZWJ)),
                    "trailing al-lakuna+ZWJ in {:?}",
                    s
                );
                assert!(!s.contains("\u{200D}\u{200D}"), "double ZWJ in {:?}", s);
            }
        }
    }
}
