#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]
#![cfg_attr(not(target_os = "windows"), allow(dead_code))]

mod config;
mod state;
#[cfg(target_os = "windows")]
mod ui;
#[cfg(target_os = "windows")]
mod win;

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("Sinhala Typer is a Windows application.");
}

/// v2 architecture: typing lives entirely in the TSF text service
/// (`sinhala_typer_tsf.dll`), which Windows loads into every app when one
/// of our keyboards is active. This executable is only a companion:
///
/// - default: a tray-only process (no window at all — it cannot flash on
///   screen the way the v1.x settings window could, and Exit is a plain
///   `process::exit`, which cannot be blocked by a close handler);
/// - `--settings`: the Settings window in its own process, where closing
///   the window simply ends that process.
#[cfg(target_os = "windows")]
fn main() {
    if std::env::args().any(|a| a == "--settings") {
        run_settings();
    } else {
        run_companion();
    }
}

#[cfg(target_os = "windows")]
fn run_companion() {
    let Some(_instance) = win::single_instance::acquire() else {
        return; // already running
    };
    let config = config::Config::load();
    let state = state::shared_state(config);
    {
        let cfg = state::lock_read(&state).config.clone();
        if cfg.autostart {
            let _ = win::autostart::set_autostart(true);
        }
    }
    if let Err(e) = ui::init_tray(&state) {
        eprintln!("tray init failed: {e}");
    }
    // One silent update check shortly after launch; toast only when an
    // update exists and notifications are on.
    {
        let st = state.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(8));
            win::update::start_check();
            for _ in 0..40 {
                std::thread::sleep(std::time::Duration::from_millis(500));
                match win::update::state() {
                    win::update::UpdateState::Available(v) => {
                        if state::lock_read(&st).config.notifications {
                            win::notify::show(
                                "Sinhala Typer update",
                                &format!(
                                    "Version {v} is available — tray menu → Settings → Updates to install it."
                                ),
                            );
                        }
                        break;
                    }
                    win::update::UpdateState::Checking => {}
                    _ => break,
                }
            }
        });
    }
    win::pump::pump_forever(state);
}

#[cfg(target_os = "windows")]
fn run_settings() {
    let config = config::Config::load();
    let state = state::shared_state(config);
    for renderer in [eframe::Renderer::Wgpu, eframe::Renderer::Glow] {
        let options = eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([560.0, 660.0])
                .with_min_inner_size([480.0, 520.0])
                .with_title("Sinhala Typer — Settings")
                .with_icon(ui::app_icon()),
            renderer,
            ..Default::default()
        };
        let st = state.clone();
        match eframe::run_native(
            "Sinhala Typer",
            options,
            Box::new(move |_cc| Ok(Box::new(ui::TyperApp::new(st)))),
        ) {
            Ok(()) => return,
            Err(e) => eprintln!("settings window ({renderer:?}) failed: {e}"),
        }
    }
}
