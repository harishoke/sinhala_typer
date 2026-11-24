use crate::config::{self, Config};
use crate::engine::Mode;
use eframe::egui;

const CHEAT_SHEET: &[(&str, &str)] = &[
    ("a / aa / ae / aae", "independent vowels; after a consonant they become vowel signs"),
    ("i / ii, u / uu, e / ee, o / oo", "short/long vowel pairs"),
    ("ai / au / Ru", "diphthongs and Ru"),
    ("k kh g gh Ng", "velar consonants"),
    ("c ch j jh", "palatal consonants"),
    ("t T d D N", "retroflex consonants"),
    ("th Th dh Dh n", "dental consonants"),
    ("p ph b bh m", "labial consonants"),
    ("y r l v w s h L f", "remaining consonants (w = v)"),
    ("nng nnd nnD nnj mb", "prenasalised consonants"),
    ("sh Sh", "sibilants"),
    ("kr + vowel", "rakaranshaya (needs ZWJ)"),
    ("ky + vowel", "yansaya"),
    ("r + consonant", "repaya"),
    ("n before k/g/h", "automatic anusvara"),
    ("x / H", "explicit anusvara / visarga"),
];

const SHAPING_TEST: &str = "\u{0D9A}\u{0DD9} \u{0D9A}\u{0DDC} \u{0D9A}\u{0DDA} \u{0D9A}\u{0DCA}\u{200D}\u{0DBB} \u{0D9A}\u{0DCA}\u{200D}\u{0DBA} \u{0DB0}\u{0DBB}\u{0DCA}\u{200D}\u{0DB8}";

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([680.0, 520.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "sinhala_typer — Settings & Info",
        options,
        Box::new(|cc| {
            if let Some(path) = font_path() {
                if let Ok(bytes) = std::fs::read(&path) {
                    let mut fonts = egui::FontDefinitions::default();
                    fonts.font_data.insert(
                        "NotoSansSinhala".to_owned(),
                        egui::FontData::from_owned(bytes).into(),
                    );
                    fonts
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .or_default()
                        .push("NotoSansSinhala".to_owned());
                    cc.egui_ctx.set_fonts(fonts);
                }
            }
            Ok(Box::new(SettingsApp::new()))
        }),
    );
}

fn font_path() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    Some(exe.parent()?.join("assets").join("NotoSansSinhala.ttf"))
}

struct SettingsApp {
    cfg: Config,
    tab: usize,
    hotkey: String,
    new_exe: String,
    new_mode: Mode,
    cheat_filter: String,
    status: String,
}

impl SettingsApp {
    fn new() -> Self {
        let cfg = config::load();
        let hotkey = cfg.toggle_hotkey.clone();
        SettingsApp {
            cfg,
            tab: 0,
            hotkey,
            new_exe: String::new(),
            new_mode: Mode::Unicode,
            cheat_filter: String::new(),
            status: String::new(),
        }
    }

    fn save(&mut self) {
        self.cfg.toggle_hotkey = self.hotkey.clone();
        match config::save(&self.cfg) {
            Ok(()) => self.status = "Saved.".to_string(),
            Err(e) => self.status = format!("Save failed: {}", e),
        }
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (i, label) in ["General", "App Rules", "Cheat Sheet", "About"]
                    .iter()
                    .enumerate()
                {
                    ui.selectable_value(&mut self.tab, i, *label);
                }
            });
            ui.separator();
            match self.tab {
                0 => {
                    ui.label("Toggle hotkey:");
                    ui.text_edit_singleline(&mut self.hotkey);
                    ui.horizontal(|ui| {
                        ui.label("Default mode:");
                        egui::ComboBox::from_id_salt("default_mode")
                            .selected_text(format!("{:?}", self.cfg.default_mode))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.cfg.default_mode, Mode::Unicode, "Unicode");
                                ui.selectable_value(&mut self.cfg.default_mode, Mode::LegacyFm, "LegacyFm");
                                ui.selectable_value(&mut self.cfg.default_mode, Mode::English, "English");
                            });
                    });
                    ui.checkbox(&mut self.cfg.start_with_windows, "Start with Windows");
                    if ui.button("Save").clicked() {
                        self.save();
                    }
                    ui.label(&self.status);
                }
                1 => {
                    let mut remove: Option<String> = None;
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let keys: Vec<String> = self.cfg.app_rules.keys().cloned().collect();
                        for key in keys {
                            ui.horizontal(|ui| {
                                let mode = self.cfg.app_rules[&key];
                                ui.label(format!("{}  ->  {:?}", key, mode));
                                if ui.small_button("Remove").clicked() {
                                    remove = Some(key);
                                }
                            });
                        }
                    });
                    if let Some(key) = remove {
                        self.cfg.app_rules.remove(&key);
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.new_exe);
                        egui::ComboBox::from_id_salt("new_mode")
                            .selected_text(format!("{:?}", self.new_mode))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.new_mode, Mode::English, "English");
                                ui.selectable_value(&mut self.new_mode, Mode::Unicode, "Unicode");
                                ui.selectable_value(&mut self.new_mode, Mode::LegacyFm, "LegacyFm");
                            });
                        if ui.button("Add rule").clicked() && !self.new_exe.trim().is_empty() {
                            self.cfg.app_rules.insert(
                                self.new_exe.trim().to_uppercase(),
                                self.new_mode,
                            );
                            self.new_exe.clear();
                        }
                    });
                    if ui.button("Save").clicked() {
                        self.save();
                    }
                    ui.label(&self.status);
                }
                2 => {
                    ui.text_edit_singleline(&mut self.cheat_filter);
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("cheat").striped(true).show(ui, |ui| {
                            for (keys, desc) in CHEAT_SHEET {
                                if self.cheat_filter.is_empty()
                                    || keys.to_lowercase().contains(&self.cheat_filter.to_lowercase())
                                    || desc.to_lowercase().contains(&self.cheat_filter.to_lowercase())
                                {
                                    ui.label(*keys);
                                    ui.label(*desc);
                                    ui.end_row();
                                }
                            }
                        });
                    });
                }
                _ => {
                    ui.heading("sinhala_typer 0.1.0");
                    ui.label("Phases 0-2 engine core is running. Hook, tray and this window are Windows-only.");
                    ui.add_space(8.0);
                    ui.label("Sinhala shaping check (if glyphs look wrong, use the HTML cheat sheet in a browser):");
                    ui.label(SHAPING_TEST);
                    ui.add_space(8.0);
                    ui.label("Tips: Ctrl+Space toggles EN/Sinhala. Per-app modes are set in App Rules. Config lives in %APPDATA%\\sinhala_typer\\config.toml.");
                    ui.label(format!(
                        "Start-with-Windows registry entry present: {}",
                        crate::win::startup::is_start_with_windows()
                    ));
                }
            }
        });
    }
}
