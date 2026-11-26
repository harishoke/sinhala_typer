use std::cell::RefCell;

use eframe::egui;
use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};

mod app_art;
mod tray_art;

use crate::state::{lock_read, lock_write, SharedState};

const ICON_SIZE: u32 = 32;
const ID_SETTINGS: &str = "settings";
const ID_UPDATE: &str = "check_update";
const ID_EXIT: &str = "exit";

/// Tray artwork (see `assets/`): the plain app tile. v2 has no modes to
/// badge — the active Windows keyboard *is* the mode.
fn tray_icon() -> Result<Icon, String> {
    Icon::from_rgba(tray_art::TRAY_UNICODE.to_vec(), ICON_SIZE, ICON_SIZE)
        .map_err(|e| e.to_string())
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
    #[allow(dead_code)]
    icon: TrayIcon,
}

// The tray lives for the whole companion process on its main thread
// (muda/tray-icon handles are not Send/Sync).
thread_local! {
    static TRAY: RefCell<Option<TrayHandles>> = const { RefCell::new(None) };
}

/// Create the tray icon + menu on the calling (main) thread.
pub fn init_tray(_state: &SharedState) -> Result<(), String> {
    let menu = Menu::new();
    let settings = MenuItem::with_id(ID_SETTINGS, "Settings...", true, None);
    let update = MenuItem::with_id(ID_UPDATE, "Check for updates", true, None);
    let sep = PredefinedMenuItem::separator();
    let exit = MenuItem::with_id(ID_EXIT, "Exit", true, None);
    menu.append(&settings).map_err(|e| e.to_string())?;
    menu.append(&update).map_err(|e| e.to_string())?;
    menu.append(&sep).map_err(|e| e.to_string())?;
    menu.append(&exit).map_err(|e| e.to_string())?;

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Sinhala Typer — Win+Space switches keyboards")
        .with_icon(tray_icon()?)
        .build()
        .map_err(|e| e.to_string())?;
    TRAY.with(|t| {
        *t.borrow_mut() = Some(TrayHandles { icon: tray });
    });
    Ok(())
}

/// Open the Settings window: focus it if this process hosts it (the
/// `--settings` process), otherwise spawn that process.
fn open_settings(ctx: Option<&egui::Context>) {
    if let Some(ctx) = ctx {
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        return;
    }
    if let Ok(exe) = std::env::current_exe() {
        let _ = std::process::Command::new(exe).arg("--settings").spawn();
    }
}

/// Drain tray events. In the companion process (ctx None) Exit terminates
/// the process directly — there is no window whose close handler could
/// interfere, which is exactly why v2 is structured this way (the v1.x
/// exit bug cannot recur).
pub fn poll_events(_state: &SharedState, ctx: Option<&egui::Context>) {
    while let Ok(ev) = MenuEvent::receiver().try_recv() {
        match ev.id.0.as_str() {
            ID_SETTINGS | ID_UPDATE => open_settings(ctx),
            ID_EXIT => {
                if let Some(ctx) = ctx {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                } else {
                    std::process::exit(0);
                }
            }
            _ => {}
        }
    }
    while let Ok(ev) = TrayIconEvent::receiver().try_recv() {
        if matches!(ev, TrayIconEvent::DoubleClick { .. }) {
            open_settings(ctx);
        }
    }
}

pub struct TyperApp {
    state: SharedState,
    status: String,
    icon_tex: Option<egui::TextureHandle>,
    style_applied: bool,
    tsf_registered: Option<bool>,
}

impl TyperApp {
    pub fn new(state: SharedState) -> Self {
        Self {
            state,
            status: String::new(),
            icon_tex: None,
            style_applied: false,
            tsf_registered: None,
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

impl eframe::App for TyperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_style(ctx);
        poll_events(&self.state, Some(ctx));

        if self.icon_tex.is_none() {
            let image = egui::ColorImage::from_rgba_unmultiplied([48, 48], &app_art::APP_ICON_48);
            self.icon_tex = Some(ctx.load_texture("app-icon", image, egui::TextureOptions::LINEAR));
        }

        // Work on a snapshot of the config and write it back at the end of
        // the frame if anything changed.
        let mut cfg = lock_read(&self.state).config.clone();
        let before = format!("{cfg:?}");
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

                section(ui, "Windows keyboards", |ui| {
                    ui.label("Sinhala Typer types through two real Windows keyboards — Sinhala Typer (Unicode) and Sinhala Typer (FM Abhaya). Register them here once, then add them under Windows Settings → Time & language → Language → Sinhala → Keyboards and switch with Win+Space. Typing then works in every app, even if this companion app is closed.");
                    if self.tsf_registered.is_none() {
                        self.tsf_registered = Some(crate::win::tsf_reg::is_registered());
                    }
                    let registered = self.tsf_registered.unwrap_or(false);
                    ui.add_space(4.0);
                    if registered {
                        ui.label(
                            egui::RichText::new("Status: registered ✓")
                                .color(egui::Color32::from_rgb(74, 222, 128)),
                        );
                        if ui.button("Remove the Windows keyboards").clicked() {
                            match crate::win::tsf_reg::unregister() {
                                Ok(()) => {
                                    self.tsf_registered =
                                        Some(crate::win::tsf_reg::is_registered());
                                    self.status = "Keyboards removed.".to_string();
                                }
                                Err(e) => self.status = e,
                            }
                        }
                    } else {
                        ui.label(
                            egui::RichText::new("Status: not registered")
                                .color(egui::Color32::from_rgb(148, 163, 184)),
                        );
                        if ui.button("Add the Windows keyboards (one admin prompt)").clicked() {
                            match crate::win::tsf_reg::register() {
                                Ok(()) => {
                                    self.tsf_registered =
                                        Some(crate::win::tsf_reg::is_registered());
                                    self.status =
                                        "Keyboards registered. Now add them under the Sinhala language in Windows Settings.".to_string();
                                }
                                Err(e) => self.status = e,
                            }
                        }
                    }
                });

                section(ui, "General", |ui| {
                    ui.checkbox(&mut cfg.notifications, "Notify me when an update is available");
                    if ui
                        .checkbox(&mut cfg.autostart, "Start with Windows")
                        .changed()
                        && !crate::win::autostart::set_autostart(cfg.autostart)
                    {
                        autostart_failed = true;
                    }
                });

                section(ui, "Updates", |ui| {
                    use crate::win::update::UpdateState;
                    match crate::win::update::state() {
                        UpdateState::Idle | UpdateState::UpToDate => {
                            ui.label(format!(
                                "You have version {}.",
                                env!("CARGO_PKG_VERSION")
                            ));
                            if ui.button("Check for updates").clicked() {
                                crate::win::update::start_check();
                            }
                            if crate::win::update::state() == UpdateState::UpToDate {
                                ui.label(
                                    egui::RichText::new("This is the latest version ✓")
                                        .color(egui::Color32::from_rgb(74, 222, 128)),
                                );
                            }
                        }
                        UpdateState::Checking => {
                            ui.label("Checking for updates…");
                        }
                        UpdateState::Available(v) => {
                            ui.label(
                                egui::RichText::new(format!("Version {v} is available!"))
                                    .strong()
                                    .color(egui::Color32::from_rgb(74, 222, 128)),
                            );
                            if ui.button(format!("Download & install v{v}")).clicked() {
                                crate::win::update::start_install(v);
                            }
                        }
                        UpdateState::Downloading => {
                            ui.label("Downloading the update… the installer will open when it is ready.");
                        }
                        UpdateState::Error(e) => {
                            ui.label(
                                egui::RichText::new(format!("Update check failed: {e}"))
                                    .color(egui::Color32::from_rgb(252, 211, 77)),
                            );
                            if ui.button("Try again").clicked() {
                                crate::win::update::start_check();
                            }
                        }
                    }
                });

                section(ui, "About", |ui| {
                    ui.label("Sinhala phonetic typing for Windows 10/11 as two real Windows keyboards: type Singlish and get Sinhala Unicode, or pick the FM Abhaya keyboard for legacy-font MS Office documents.");
                    ui.label("The phonetic layout is the standard ශබ්දානුකූල (phonetic) scheme: doubled vowels are long, capitals give the retroflex letters, and z gives the naasikya letters (zga = ඟ).");
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

        if autostart_failed {
            cfg.autostart = crate::win::autostart::is_enabled();
            self.status = "Could not update autostart.".to_string();
        }
        if format!("{cfg:?}") != before {
            let mut st = lock_write(&self.state);
            st.config = cfg;
            if let Err(err) = st.config.save() {
                self.status = format!("Save failed: {err}");
            }
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(300));
    }
}
