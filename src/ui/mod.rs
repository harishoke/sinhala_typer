use std::cell::RefCell;

use eframe::egui;
use muda::{CheckMenuItem, Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::engine::Mode;
use crate::state::SharedState;

const ID_MODE_ENGLISH: &str = "mode_english";
const ID_MODE_UNICODE: &str = "mode_unicode";
const ID_MODE_LEGACY: &str = "mode_legacy";
const ID_ENABLED: &str = "enabled";
const ID_SETTINGS: &str = "settings";
const ID_EXIT: &str = "exit";

const ICON_SIZE: u32 = 32;

/// Simple "S" tile so we ship without binary icon assets.
fn tray_icon_rgba() -> Vec<u8> {
    const S: [u8; 8] = [
        0b00111110,
        0b01111111,
        0b01110000,
        0b00111110,
        0b00110111,
        0b00000110,
        0b01110011,
        0b00111100,
    ];
    let mut out = Vec::with_capacity((ICON_SIZE * ICON_SIZE * 4) as usize);
    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let border = x == 0 || y == 0 || x == ICON_SIZE - 1 || y == ICON_SIZE - 1;
            let (r, g, b) = if border {
                (12u8, 42u8, 96u8)
            } else {
                let sx = x * 8 / ICON_SIZE;
                let sy = y * 8 / ICON_SIZE;
                if S[sy as usize] & (1 << (7 - sx)) != 0 {
                    (255u8, 255u8, 255u8)
                } else {
                    (24u8, 84u8, 176u8)
                }
            };
            out.extend_from_slice(&[r, g, b, 255]);
        }
    }
    out
}

struct TrayHandles {
    icon: TrayIcon,
    mode_english: CheckMenuItem,
    mode_unicode: CheckMenuItem,
    mode_legacy: CheckMenuItem,
    enabled: CheckMenuItem,
}

// Tray lives for the whole process on the main thread (muda/tray-icon
// handles are not Send/Sync), so it survives settings-window failures.
thread_local! {
    static TRAY: RefCell<Option<TrayHandles>> = const { RefCell::new(None) };
}

/// Create the tray icon + menu on the calling (main) thread. Must be called
/// before the event loop starts so the same thread pumps its messages.
pub fn init_tray(state: &SharedState) -> Result<(), String> {
    let (mode, enabled) = {
        let st = state.read().unwrap();
        (st.config.mode, st.config.enabled)
    };
    let menu = Menu::new();
    let mode_english = CheckMenuItem::with_id(ID_MODE_ENGLISH, "English", true, mode == Mode::English, None);
    let mode_unicode = CheckMenuItem::with_id(ID_MODE_UNICODE, "Sinhala (Unicode)", true, mode == Mode::Unicode, None);
    let mode_legacy = CheckMenuItem::with_id(ID_MODE_LEGACY, "Sinhala (FM Abhaya)", true, mode == Mode::LegacyFm, None);
    let enabled = CheckMenuItem::with_id(ID_ENABLED, "Typing enabled", true, enabled, None);
    for item in [&mode_english, &mode_unicode, &mode_legacy, &enabled] {
        menu.append(item).map_err(|e| e.to_string())?;
    }
    let settings = MenuItem::with_id(ID_SETTINGS, "Settings...", true, None);
    let exit = MenuItem::with_id(ID_EXIT, "Exit", true, None);
    for item in [&settings, &exit] {
        menu.append(item).map_err(|e| e.to_string())?;
    }

    let icon = Icon::from_rgba(tray_icon_rgba(), ICON_SIZE, ICON_SIZE).map_err(|e| e.to_string())?;
    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Sinhala Typer")
        .with_icon(icon)
        .build()
        .map_err(|e| e.to_string())?;
    TRAY.with(|t| {
        *t.borrow_mut() = Some(TrayHandles {
            icon: tray,
            mode_english,
            mode_unicode,
            mode_legacy,
            enabled,
        });
    });
    Ok(())
}

fn apply_menu_id(state: &SharedState, id: &str) {
    let mut st = state.write().unwrap();
    match id {
        ID_MODE_ENGLISH => st.config.mode = Mode::English,
        ID_MODE_UNICODE => st.config.mode = Mode::Unicode,
        ID_MODE_LEGACY => st.config.mode = Mode::LegacyFm,
        ID_ENABLED => st.config.enabled = !st.config.enabled,
        _ => {}
    }
    let _ = st.config.save();
}

/// Drain tray events. `ctx` is None in the GUI-less fallback mode, where the
/// Settings item is unavailable and Exit terminates the process.
pub fn poll_events(state: &SharedState, ctx: Option<&egui::Context>) {
    while let Ok(ev) = MenuEvent::receiver().try_recv() {
        let id = ev.id.0.clone();
        match id.as_str() {
            ID_SETTINGS => {
                if let Some(ctx) = ctx {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                }
            }
            ID_EXIT => {
                crate::win::hook::stop();
                if let Some(ctx) = ctx {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                } else {
                    std::process::exit(0);
                }
            }
            _ => apply_menu_id(state, &id),
        }
    }
    while let Ok(_ev) = TrayIconEvent::receiver().try_recv() {
        if let Some(ctx) = ctx {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        }
    }
    sync_tray(state);
}

fn sync_tray(state: &SharedState) {
    TRAY.with(|t| {
        let guard = t.borrow();
        let Some(h) = guard.as_ref() else { return };
        let st = state.read().unwrap();
        let _ = h.mode_english.set_checked(st.config.mode == Mode::English);
        let _ = h.mode_unicode.set_checked(st.config.mode == Mode::Unicode);
        let _ = h.mode_legacy.set_checked(st.config.mode == Mode::LegacyFm);
        let _ = h.enabled.set_checked(st.config.enabled);
        let mode_label = match st.config.mode {
            Mode::English => "English",
            Mode::Unicode => "Sinhala (Unicode)",
            Mode::LegacyFm => "Sinhala (FM Abhaya)",
        };
        let label = format!(
            "Sinhala Typer — {}{}",
            mode_label,
            if st.config.enabled { "" } else { " (off)" }
        );
        let _ = h.icon.set_tooltip(Some(&label));
    });
}

pub struct TyperApp {
    state: SharedState,
    hotkey_text: String,
    legacy_text: String,
    status: String,
}

impl TyperApp {
    pub fn new(state: SharedState) -> Self {
        let (hotkey_text, legacy_text) = {
            let st = state.read().unwrap();
            (st.config.hotkey.clone(), st.config.legacy_processes.join("\n"))
        };
        Self {
            state,
            hotkey_text,
            legacy_text,
            status: String::new(),
        }
    }
}

impl eframe::App for TyperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        poll_events(&self.state, Some(ctx));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sinhala Typer — Settings");
            ui.add_space(8.0);
            let mut st = self.state.write().unwrap();
            ui.heading("Typing mode");
            ui.radio_value(&mut st.config.mode, Mode::English, "English (pass-through)");
            ui.radio_value(&mut st.config.mode, Mode::Unicode, "Sinhala (Unicode)");
            ui.radio_value(&mut st.config.mode, Mode::LegacyFm, "Sinhala (FM Abhaya, for MS Office)");
            ui.add_space(10.0);
            ui.checkbox(&mut st.config.enabled, "Typing enabled");
            ui.add_space(10.0);
            ui.label("Mode hotkey:");
            let response = ui.text_edit_singleline(&mut self.hotkey_text);
            if response.changed() {
                if crate::config::Hotkey::parse(&self.hotkey_text).is_ok() {
                    st.config.hotkey = self.hotkey_text.trim().to_string();
                }
            }
            ui.small("e.g. ctrl+alt+s (toggles English / Unicode / FM Abhaya)");
            ui.add_space(10.0);
            ui.label("Apps that should use FM Abhaya mode (one .exe per line):");
            ui.add(
                egui::TextEdit::multiline(&mut self.legacy_text)
                    .desired_width(340.0)
                    .desired_rows(4),
            );
            if ui.button("Save").clicked() {
                st.config.legacy_processes = self
                    .legacy_text
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty())
                    .collect();
                match st.config.save() {
                    Ok(()) => self.status = "Saved.".to_string(),
                    Err(err) => self.status = format!("Save failed: {err}"),
                }
            }
            let mut autostart = st.config.autostart;
            if ui.checkbox(&mut autostart, "Start with Windows").changed() {
                if crate::win::autostart::set_autostart(autostart) {
                    st.config.autostart = autostart;
                } else {
                    self.status = "Could not update autostart.".to_string();
                }
            }
            if !self.status.is_empty() {
                ui.small(&self.status);
            }
            ui.add_space(10.0);
            ui.collapsing("About", |ui| {
                ui.small("Lightweight Sinhala phonetic typing utility (Helakuru-style).");
                ui.small("Engine: Singlish -> Sinhala Unicode in real time, legacy FM Abhaya for Office.");
                ui.small(format!("Config: {}", crate::config::Config::config_dir().display()));
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(300));
    }
}
