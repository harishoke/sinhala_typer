//! Golden-table tests, driven by `data/golden_pairs.csv` (the Helakuru
//! capture — the single source of truth for the keymap, see
//! `docs/rewrite-plan.md` section 5). Every CSV row becomes a test; all rows
//! must pass. If the table and any other document disagree, the table wins.

use sinhala_engine::legacy;
use sinhala_engine::phonetic::PhoneticEngine;
use sinhala_engine::{EditAction, ScreenSim};

struct Row {
    id: String,
    input: String,
    unicode_expected: String,
    legacy_expected: String,
}

/// Minimal RFC-4180-ish CSV parser (handles quoted fields and "" escapes).
fn parse_csv(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            row.push(std::mem::take(&mut field));
        } else if c == '\n' {
            row.push(std::mem::take(&mut field));
            rows.push(std::mem::take(&mut row));
        } else if c != '\r' {
            field.push(c);
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

fn golden_rows() -> Vec<Row> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/golden_pairs.csv");
    let text = std::fs::read_to_string(path).expect("read data/golden_pairs.csv");
    parse_csv(&text)
        .into_iter()
        .skip(1) // header
        .filter(|cols| cols.len() >= 5 && !cols[0].is_empty())
        .map(|cols| Row {
            id: cols[0].clone(),
            input: cols[2].clone(),
            unicode_expected: cols[3].clone(),
            legacy_expected: cols[4].clone(),
        })
        .collect()
}

/// Type `s` through the streaming engine, applying the returned edit actions
/// to a simulated screen; non-accepted characters pass through untouched.
fn screen_of(s: &str) -> String {
    let mut engine = PhoneticEngine::default();
    let mut sim = ScreenSim::new();
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
    sim.as_str().to_string()
}

#[test]
fn golden_pairs_unicode() {
    let rows = golden_rows();
    assert!(rows.len() >= 100, "golden table should have ~190 rows");
    let mut failures = Vec::new();
    for row in &rows {
        let got = screen_of(&row.input);
        if got != row.unicode_expected {
            failures.push(format!(
                "{}: input {:?} expected {:?} got {:?}",
                row.id, row.input, row.unicode_expected, got
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} golden row(s) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn golden_pairs_legacy() {
    // legacy_expected is filled from the FM Abhaya table; where present it
    // must match the legacy converter applied to the Unicode expectation.
    for row in golden_rows() {
        if !row.legacy_expected.is_empty() {
            let got = legacy::unicode_to_fm_abhaya(&row.unicode_expected);
            assert_eq!(
                got, row.legacy_expected,
                "{}: legacy mismatch for input {:?}",
                row.id, row.input
            );
        }
    }
}

#[test]
fn engine_never_panics_on_random_input() {
    // Deterministic LCG so failures are reproducible; covers ASCII letters,
    // digits, punctuation, spaces and some non-ASCII.
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .,!?;:'\"-_()[]{}<>/@#$%^&*+=|\\~`".chars().chain(['\u{0D9A}', '\u{200D}', '\u{1F600}', '\u{00E9}']).collect();
    let mut state: u64 = 0x5EED_5EED_5EED_5EED;
    let mut next = move || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (state >> 33) as usize
    };
    for _ in 0..2000 {
        let len = next() % 24;
        let s: String = (0..len).map(|_| charset[next() % charset.len()]).collect();
        let _ = PhoneticEngine::convert(&s);
        let mut engine = PhoneticEngine::default();
        let mut sim = ScreenSim::new();
        for c in s.chars() {
            let action = engine.push_char(c);
            sim.apply(&action);
            let _ = engine.backspace();
            engine.reset();
        }
    }
}
