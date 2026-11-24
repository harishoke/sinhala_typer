mod engine;

use engine::phonetic::PhoneticEngine;

fn main() {
    let modes = [
        engine::Mode::English,
        engine::Mode::Unicode,
        engine::Mode::LegacyFm,
    ];
    println!("sinhala_typer 0.1.0 — modes: {:?}", modes);
    let word = PhoneticEngine::convert("krama");
    println!("krama unicode: {}", word);
    println!("krama FM Abhaya: {}", engine::legacy::unicode_to_fm_abhaya(&word));
}
