#![cfg_attr(not(target_os = "windows"), allow(dead_code))]

mod config;
mod engine;
mod state;

#[cfg(target_os = "windows")]
mod ui;
#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
fn main() -> Result<(), eframe::Error> {
    let cfg = config::Config::load();
    let state = state::shared_state(cfg);

    let mut autostart_cfg = state.read().unwrap().config.clone();
    if autostart_cfg.autostart != win::autostart::is_enabled() {
        let _ = win::autostart::set_autostart(autostart_cfg.autostart);
        autostart_cfg.autostart = win::autostart::is_enabled();
        state.write().unwrap().config.autostart = autostart_cfg.autostart;
    }
    if let Err(err) = state.read().unwrap().config.save() {
        eprintln!("sinhala_typer: could not save config: {err}");
    }

    let hook_state = state.clone();
    win::hook::spawn(hook_state);

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_visible(false)
            .with_inner_size([420.0, 520.0])
            .with_min_inner_size([380.0, 420.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sinhala Typer",
        options,
        Box::new(|_cc| Ok(Box::new(ui::TyperApp::new(state)))),
    )
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("sinhala_typer runs on Windows 10/11 only (global keyboard hook + tray UI).");
    println!("The typing engine core is platform-independent and covered by `cargo test`.");
}
