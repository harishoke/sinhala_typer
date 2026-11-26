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
const ID_NOTIFY: &str = "notify";
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
    notify: CheckMenuItem,
    notify_checked: bool,
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
    let (mode, enabled, notifications) = {
        let st = lock_read(state);
        (
            st.config.mode,
            st.config.enabled,
            st.config.notifications,
        )
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
    let notify = CheckMenuItem::with_id(
        ID_NOTIFY,
        "Notify on mode change",
        true,
        notifications,
        None,
    );
    for item in [&mode_english, &mode_unicode, &mode_legacy, &enabled, &notify] {
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
            notify,
            notify_checked: notifications,
            visual,
            tooltip: "Sinhala Typer".to_string(),
        });
    });
    Ok(())
}

fn apply_menu_id(state: &SharedState, id: &str) {
    let mut notify_mode: Option<(Mode, bool)> = None;
    {
        let mut st = lock_write(state);
        match id {
            ID_MODE_ENGLISH => st.config.mode = Mode::English,
            ID_MODE_UNICODE => st.config.mode = Mode::Unicode,
            ID_MODE_LEGACY => st.config.mode = Mode::LegacyFm,
            ID_ENABLED => st.config.enabled = !st.config.enabled,
            ID_NOTIFY => st.config.notifications = !st.config.notifications,
            _ => {}
        }
        let _ = st.config.save();
        if matches!(
            id,
            ID_MODE_ENGLISH | ID_MODE_UNICODE | ID_MODE_LEGACY | ID_ENABLED
        ) && st.config.notifications
        {
            notify_mode = Some((st.config.mode, st.config.enabled));
        }
    }
    if let Some((mode, enabled)) = notify_mode {
        crate::win::notify::mode_changed(mode, enabled);
    }
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
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
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
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
        }
    }
    sync_tray(state);
}

fn sync_tray(state: &SharedState) {
    TRAY.with(|t| {
        let mut guard = t.borrow_mut();
        let Some(h) = guard.as_mut() else { return };
        let (mode, enabled, notifications) = {
            let st = lock_read(state);
            (
                st.config.mode,
                st.config.enabled,
                st.config.notifications,
            )
        };
        if notifications != h.notify_checked {
            h.notify.set_checked(notifications);
            h.notify_checked = notifications;
        }
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
    icon_tex: Option<egui::TextureHandle>,
    style_applied: bool,
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
            icon_tex: None,
            style_applied: false,
        }
    }

    fn apply_style(&mut self, ctx: &egui::Context) {
        if self.style_applied {
            return;
        }
        self.style_applied = true;
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.button_padding = egui::vec2(14.0, 7.0);
        style.spacing.window_margin = egui::Margin::same(18.0);
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(17, 20, 27);
        visuals.window_fill = egui::Color32::from_rgb(17, 20, 27);
        visuals.extreme_bg_color = egui::Color32::from_rgb(10, 12, 16);
        visuals.faint_bg_color = egui::Color32::from_rgb(24, 29, 38);
        visuals.selection.bg_fill = egui::Color32::from_rgb(37, 99, 235);
        visuals.hyperlink_color = egui::Color32::from_rgb(96, 165, 250);
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(10.0);
        visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        visuals.widgets.active.rounding = egui::Rounding::same(8.0);
        style.visuals = visuals;
        ctx.set_style(style);
    }
}

const ACCENT: egui::Color32 = egui::Color32::from_rgb(45, 140, 240);
const CARD_FILL: egui::Color32 = egui::Color32::from_rgb(24, 29, 38);

fn section(ui: &mut egui::Ui, title: &str, body: impl FnOnce(&mut egui::Ui)) {
    ui.add_space(14.0);
    ui.label(
        egui::RichText::new(title)
            .size(15.0)
            .strong()
            .color(egui::Color32::from_rgb(226, 232, 240)),
    );
    ui.add_space(6.0);
    egui::Frame::none()
        .fill(CARD_FILL)
        .rounding(egui::Rounding::same(12.0))
        .inner_margin(egui::Margin::same(14.0))
        .show(ui, body);
}

fn mode_row(
    ui: &mut egui::Ui,
    cfg_mode: &mut Mode,
    value: Mode,
    title: &str,
    desc: &str,
) {
    let selected = *cfg_mode == value;
    let frame = egui::Frame::none()
        .fill(if selected {
            egui::Color32::from_rgb(30, 58, 95)
        } else {
            egui::Color32::from_rgb(31, 38, 50)
        })
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::same(10.0));
    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(cfg_mode, value, "");
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(title).strong().size(14.0));
                ui.label(
                    egui::RichText::new(desc)
                        .size(12.0)
                        .color(egui::Color32::from_rgb(148, 163, 184)),
                );
            });
        });
    });
    ui.add_space(6.0);
}

impl eframe::App for TyperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_style(ctx);
        poll_events(&self.state, Some(ctx));

        // Closing Settings must never quit the app: the tray icon is the
        // home of this program. Hide the window instead; it reopens from
        // the tray menu ("Settings...") or a double-click on the icon.
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }

        if self.icon_tex.is_none() {
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [48, 48],
                &app_art::APP_ICON_48,
            );
            self.icon_tex =
                Some(ctx.load_texture("app-icon", image, egui::TextureOptions::LINEAR));
        }

        // Work on a snapshot of the config and write it back at the end of
        // the frame if anything changed. Holding the write lock across the
        // whole frame used to stall the keyboard-hook thread (which reads
        // the config on every keystroke) whenever Settings was open.
        let mut cfg = lock_read(&self.state).config.clone();
        let before = format!("{cfg:?}");
        let mut save_clicked = false;
        let mut autostart_failed = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Header: icon, name, version, creator.
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(21, 34, 54))
                    .rounding(egui::Rounding::same(14.0))
                    .inner_margin(egui::Margin::same(14.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if let Some(tex) = &self.icon_tex {
                                ui.image((tex.id(), egui::vec2(48.0, 48.0)));
                            }
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Sinhala Typer")
                                        .size(22.0)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Version {}  ·  by Ashen Nethsara",
                                        env!("CARGO_PKG_VERSION")
                                    ))
                                    .size(12.5)
                                    .color(egui::Color32::from_rgb(148, 197, 255)),
                                );
                            });
                        });
                    });

                section(ui, "Typing mode", |ui| {
                    mode_row(
                        ui,
                        &mut cfg.mode,
                        Mode::Unicode,
                        "Sinhala (Unicode)",
                        "Type Singlish, get Sinhala Unicode everywhere.",
                    );
                    mode_row(
                        ui,
                        &mut cfg.mode,
                        Mode::LegacyFm,
                        "Sinhala (FM Abhaya)",
                        "Legacy font mode for MS Office documents.",
                    );
                    mode_row(
                        ui,
                        &mut cfg.mode,
                        Mode::English,
                        "English",
                        "Pass-through: your keyboard works as normal.",
                    );
                    ui.checkbox(&mut cfg.enabled, "Typing enabled");
                    ui.label(
                        egui::RichText::new(format!(
                            "Tip: press {} anywhere to cycle modes. If you switch a window to a Sinhala layout in the Windows language bar, Sinhala Typer pauses there automatically.",
                            cfg.hotkey
                        ))
                        .size(12.0)
                        .color(egui::Color32::from_rgb(148, 163, 184)),
                    );
                });

                section(ui, "General", |ui| {
                    ui.checkbox(&mut cfg.notifications, "Notify me when the mode changes");
                    if ui
                        .checkbox(&mut cfg.autostart, "Start with Windows")
                        .changed()
                        && !crate::win::autostart::set_autostart(cfg.autostart)
                    {
                        autostart_failed = true;
                    }
                    ui.add_space(4.0);
                    ui.label("Mode hotkey (cycles the three modes):");
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
                    ui.label(
                        egui::RichText::new("e.g. ctrl+alt+s")
                            .size(12.0)
                            .color(egui::Color32::from_rgb(148, 163, 184)),
                    );
                });

                section(ui, "FM Abhaya apps", |ui| {
                    ui.label(
                        "These apps automatically use FM Abhaya mode while the mode is Sinhala (Unicode). One .exe per line:",
                    );
                    ui.add(
                        egui::TextEdit::multiline(&mut self.legacy_text)
                            .desired_width(f32::INFINITY)
                            .desired_rows(4),
                    );
                    if ui
                        .button(egui::RichText::new("Save app list").color(egui::Color32::WHITE))
                        .clicked()
                    {
                        save_clicked = true;
                    }
                });

                section(ui, "About", |ui| {
                    ui.label("Lightweight Sinhala phonetic typing for Windows 10/11. Type Singlish and get Sinhala Unicode in real time, with a legacy FM Abhaya mode for MS Office.");
                    ui.label("Ctrl / Alt / Win shortcuts (copy, paste, save...) always pass through unchanged.");
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Created by");
                        ui.label(egui::RichText::new("Ashen Nethsara").strong().color(ACCENT));
                    });
                    ui.horizontal(|ui| {
                        ui.hyperlink_to("LinkedIn", "https://www.linkedin.com/in/ashennethsara/");
                        ui.label("·");
                        ui.hyperlink_to("GitHub", "https://github.com/AshenNethsara/");
                    });
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "Config: {}",
                            crate::config::Config::config_dir().display()
                        ))
                        .size(11.5)
                        .color(egui::Color32::from_rgb(148, 163, 184)),
                    );
                });

                if !self.status.is_empty() {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new(&self.status)
                            .size(12.5)
                            .color(egui::Color32::from_rgb(252, 211, 77)),
                    );
                }
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
