#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod config;
mod engine;
#[cfg(target_os = "windows")]
mod ui;
#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--settings") {
        ui::settings::run();
        return;
    }
    if !win::startup::acquire_single_instance() {
        return;
    }
    let cfg = config::load();
    if cfg.start_with_windows {
        let _ = win::startup::set_start_with_windows(true);
    }
    win::app_detect::start_foreground_watcher();
    win::hook::init(&cfg);
    let _hook_thread = std::thread::spawn(win::hook::run);
    ui::tray::run(cfg);
}

#[cfg(not(target_os = "windows"))]
fn main() {
    use engine::phonetic::PhoneticEngine;
    let modes = [
        engine::Mode::English,
        engine::Mode::Unicode,
        engine::Mode::LegacyFm,
    ];
    println!("sinhala_typer 0.1.0 — modes: {:?}", modes);
    println!("(Windows-only hook/tray/settings run on Windows; engine tests: cargo test)");
    let word = PhoneticEngine::convert("krama");
    println!("krama unicode: {}", word);
    println!("krama FM Abhaya: {}", engine::legacy::unicode_to_fm_abhaya(&word));
}
