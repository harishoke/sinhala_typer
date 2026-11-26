use crate::engine::Mode;

/// Optional toast when the typing mode changes (Settings -> "Notify me when
/// the mode changes", off by default). Shown from a helper thread so neither
/// the keyboard hook nor the UI ever blocks on the notification service.
pub fn mode_changed(mode: Mode, enabled: bool) {
    let body = if !enabled {
        "Typing is off".to_string()
    } else {
        match mode {
            Mode::English => "English (pass-through)",
            Mode::Unicode => "Sinhala (Unicode)",
            Mode::LegacyFm => "Sinhala (FM Abhaya)",
        }
        .to_string()
    };
    std::thread::spawn(move || {
        let _ = notify_rust::Notification::new()
            .appname("Sinhala Typer")
            .summary("Sinhala Typer")
            .body(&body)
            .show();
    });
}
