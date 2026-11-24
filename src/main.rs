#![cfg_attr(not(target_os = "windows"), allow(dead_code))]

mod config;
mod engine;
mod state;

#[cfg(target_os = "windows")]
mod ui;
#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
fn main() {
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

    if let Err(err) = ui::init_tray(&state) {
        eprintln!("sinhala_typer: tray unavailable: {err}");
    }

    // Glow (OpenGL) is the default renderer; wgpu is a fallback for systems
    // with broken or missing GL drivers (select with SINHALA_TYPER_RENDERER=wgpu).
    let renderer = if std::env::var("SINHALA_TYPER_RENDERER").as_deref() == Ok("wgpu") {
        eframe::Renderer::Wgpu
    } else {
        eframe::Renderer::Glow
    };
    let options = eframe::NativeOptions {
        renderer,
        viewport: eframe::egui::ViewportBuilder::default()
            .with_visible(false)
            .with_inner_size([420.0, 520.0])
            .with_min_inner_size([380.0, 420.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        "Sinhala Typer",
        options,
        Box::new(|_cc| Ok(Box::new(ui::TyperApp::new(state.clone())))),
    );
    if let Err(err) = result {
        // No usable GPU/renderer: keep running tray-only so typing still works.
        eprintln!("sinhala_typer: settings window unavailable ({err}); running tray-only");
        let log = config::Config::config_dir().join("last_error.txt");
        if let Some(parent) = log.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&log, format!("{err}"));
        win::pump::pump_forever(state);
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("sinhala_typer runs on Windows 10/11 only (global keyboard hook + tray UI).");
    println!("The typing engine core is platform-independent and covered by `cargo test`.");
}
