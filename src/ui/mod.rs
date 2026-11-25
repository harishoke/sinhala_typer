use std::cell::RefCell;

use eframe::egui;
use muda::{CheckMenuItem, Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};

mod app_art;
mod tray_art;

use crate::engine::Mode;
use crate::state::{lock_read, lock_write, SharedState};

const ID_MODE_ENGLISH: &str = "mode_english";
const ID_MODE_UNICODE: &str = "mode_unicode";
const ID_MODE_LEGACY: &str = "mode_legacy";
const ID_ENABLED: &str = "enabled";
const ID_SETTINGS: &str = "settings";
const ID_EXIT: &str = "exit";

pub const ICON_SIZE: u32 = 32;

/// Designed tray artwork (see `assets/`): one tile per state, generated from
/// the app icon. Legacy mode carries an amber status dot, English a slate
/// dot, and the disabled state is greyed out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrayVisual {
    Unicode,
    Legacy,
    English,
    Off,
}

fn tray_visual(mode: Mode, enabled: bool) -> TrayVisual {
    if !enabled {
        TrayVisual::Off
    } else {
        match mode {
            Mode::Unicode => TrayVisual::Unicode,
            Mode::LegacyFm => TrayVisual::Legacy,
            Mode::English => TrayVisual::English,
        }
    }
}

fn tray_icon(visual: TrayVisual) -> Result<Icon, String> {
    let bytes: &[u8] = match visual {
        TrayVisual::Unicode => &tray_art::TRAY_UNICODE,
        TrayVisual::Legacy => &tray_art::TRAY_LEGACY,
        TrayVisual::English => &tray_art::TRAY_ENGLISH,
        TrayVisual::Off => &tray_art::TRAY_OFF,
    };
    Icon::from_rgba(bytes.to_vec(), ICON_SIZE, ICON_SIZE).map_err(|e| e.to_string())
}

/// Raw RGBA of the main app icon, for the settings window / taskbar.
pub fn app_icon() -> egui::IconData {
    egui::IconData {
        rgba: app_art::APP_ICON_48.to_vec(),
        width: 48,
        height: 48,
    }
}

struct TrayHandles {
    icon: TrayIcon,
    mode_english: CheckMenuItem,
    mode_unicode: CheckMenuItem,
    mode_legacy: CheckMenuItem,
    enabled: CheckMenuItem,
    visual: TrayVisual,
    tooltip: String,
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
        let st = lock_read(state);
        (st.config.mode, st.config.enabled)
    };
    let visual = tray_visual(mode, enabled);
    let menu = Menu::new();
    let mode_english = CheckMenuItem::with_id(
        ID_MODE_ENGLISH,
        "English",
        true,
        mode == Mode::English,
        None,
    );
    let mode_unicode = CheckMenuItem::with_id(
        ID_MODE_UNICODE,
        "Sinhala (Unicode)",
        true,
        mode == Mode::Unicode,
        None,
    );
    let mode_legacy = CheckMenuItem::with_id(
        ID_MODE_LEGACY,
        "Sinhala (FM Abhaya)",
        true,
        mode == Mode::LegacyFm,
        None,
    );
    let enabled = CheckMenuItem::with_id(ID_ENABLED, "Typing enabled", true, enabled, None);
    for item in [&mode_english, &mode_unicode, &mode_legacy, &enabled] {
        menu.append(item).map_err(|e| e.to_string())?;
    }
    let settings = MenuItem::with_id(ID_SETTINGS, "Settings...", true, None);
    let exit = MenuItem::with_id(ID_EXIT, "Exit", true, None);
    for item in [&settings, &exit] {
        menu.append(item).map_err(|e| e.to_string())?;
    }

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Sinhala Typer")
        .with_icon(tray_icon(visual)?)
        .build()
        .map_err(|e| e.to_string())?;
    TRAY.with(|t| {
        *t.borrow_mut() = Some(TrayHandles {
            icon: tray,
            mode_english,
            mode_unicode,
            mode_legacy,
            enabled,
            visual,
            tooltip: "Sinhala Typer".to_string(),
        });
    });
    Ok(())
}

fn apply_menu_id(state: &SharedState, id: &str) {
    let mut st = lock_write(state);
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
    while let Ok(ev) = TrayIconEvent::receiver().try_recv() {
        if matches!(ev, TrayIconEvent::DoubleClick { .. }) {
            if let Some(ctx) = ctx {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            }
        }
    }
    sync_tray(state);
}

fn sync_tray(state: &SharedState) {
    TRAY.with(|t| {
        let mut guard = t.borrow_mut();
        let Some(h) = guard.as_mut() else { return };
        let (mode, enabled) = {
            let st = lock_read(state);
            (st.config.mode, st.config.enabled)
        };
        let mode_label = match mode {
            Mode::English => "English",
            Mode::Unicode => "Sinhala (Unicode)",
            Mode::LegacyFm => "Sinhala (FM Abhaya)",
        };
        let label = format!(
            "Sinhala Typer - {}{}",
            mode_label,
            if enabled { "" } else { " (off)" }
        );
        // Repaints call this several times a second; touching the native
        // tray on every call flickers and wastes cycles, so update only
        // what changed.
        if label != h.tooltip {
            h.mode_english.set_checked(mode == Mode::English);
            h.mode_unicode.set_checked(mode == Mode::Unicode);
            h.mode_legacy.set_checked(mode == Mode::LegacyFm);
            h.enabled.set_checked(enabled);
            let _ = h.icon.set_tooltip(Some(&label));
            h.tooltip = label;
        }
        let visual = tray_visual(mode, enabled);
        if visual != h.visual {
            if let Ok(icon) = tray_icon(visual) {
                let _ = h.icon.set_icon(Some(icon));
                h.visual = visual;
            }
        }
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
            let st = lock_read(&state);
            (
                st.config.hotkey.clone(),
                st.config.legacy_processes.join("\n"),
            )
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

        // Work on a snapshot of the config and write it back at the end of
        // the frame if anything changed. Holding the write lock across the
        // whole frame used to stall the keyboard-hook thread (which reads
        // the config on every keystroke) whenever Settings was open.
        let mut cfg = lock_read(&self.state).config.clone();
        let before = format!("{cfg:?}");
        let mut save_clicked = false;
        let mut autostart_failed = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sinhala Typer - Settings");
            ui.add_space(8.0);
            ui.heading("Typing mode");
            ui.radio_value(&mut cfg.mode, Mode::English, "English (pass-through)");
            ui.radio_value(&mut cfg.mode, Mode::Unicode, "Sinhala (Unicode)");
            ui.radio_value(&mut cfg.mode, Mode::LegacyFm, "Sinhala (FM Abhaya, for MS Office)");
            ui.add_space(10.0);
            ui.checkbox(&mut cfg.enabled, "Typing enabled");
            ui.add_space(10.0);
            ui.label("Mode hotkey:");
            let response = ui.text_edit_singleline(&mut self.hotkey_text);
            if response.changed() {
                match crate::config::Hotkey::parse(&self.hotkey_text) {
                    Ok(_) => {
                        cfg.hotkey = self.hotkey_text.trim().to_string();
                        self.status.clear();
                    }
                    Err(err) => self.status = format!("Hotkey: {err}"),
                }
            }
            ui.small("e.g. ctrl+alt+s (cycles English / Unicode / FM Abhaya)");
            ui.add_space(10.0);
            ui.label("Apps that should use FM Abhaya mode (one .exe per line):");
            ui.add(
                egui::TextEdit::multiline(&mut self.legacy_text)
                    .desired_width(340.0)
                    .desired_rows(4),
            );
            if ui.button("Save").clicked() {
                save_clicked = true;
            }
            if ui.checkbox(&mut cfg.autostart, "Start with Windows").changed()
                && !crate::win::autostart::set_autostart(cfg.autostart)
            {
                autostart_failed = true;
            }
            if !self.status.is_empty() {
                ui.small(&self.status);
            }
            ui.add_space(10.0);
            ui.collapsing("About", |ui| {
                ui.small(format!(
                    "Sinhala Typer {} - lightweight Sinhala phonetic typing for Windows.",
                    env!("CARGO_PKG_VERSION")
                ));
                ui.small("Type Singlish and get Sinhala Unicode in real time, with a legacy FM Abhaya mode for MS Office.");
                ui.small("Ctrl / Alt / Win shortcuts (copy, paste, save...) always pass through unchanged.");
                ui.small(format!("Config: {}", crate::config::Config::config_dir().display()));
            });
        });

        if save_clicked {
            cfg.legacy_processes = self
                .legacy_text
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
        }
        if autostart_failed {
            cfg.autostart = crate::win::autostart::is_enabled();
            self.status = "Could not update autostart.".to_string();
        }
        if format!("{cfg:?}") != before || save_clicked {
            let mut st = lock_write(&self.state);
            st.config = cfg;
            match st.config.save() {
                Ok(()) if save_clicked => self.status = "Saved.".to_string(),
                Ok(()) => {}
                Err(err) => self.status = format!("Save failed: {err}"),
            }
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(300));
    }
}
