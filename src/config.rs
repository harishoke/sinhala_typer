use crate::engine::Mode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub toggle_hotkey: String,
    pub default_mode: Mode,
    pub start_with_windows: bool,
    pub app_rules: HashMap<String, Mode>,
}

impl Default for Config {
    fn default() -> Self {
        let mut app_rules = HashMap::new();
        for exe in ["WINWORD.EXE", "EXCEL.EXE", "POWERPNT.EXE", "PHOTOSHOP.EXE"] {
            app_rules.insert(exe.to_string(), Mode::LegacyFm);
        }
        app_rules.insert("CODE.EXE".to_string(), Mode::English);
        Config {
            toggle_hotkey: "Ctrl+Space".to_string(),
            default_mode: Mode::Unicode,
            start_with_windows: false,
            app_rules,
        }
    }
}

pub fn config_path() -> PathBuf {
    let base = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("sinhala_typer").join("config.toml")
}

pub fn load() -> Config {
    load_from(&config_path())
}

pub fn load_from(path: &Path) -> Config {
    match std::fs::read_to_string(path) {
        Ok(text) => toml::from_str(&text).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

pub fn save(config: &Config) -> Result<(), String> {
    save_to(config, &config_path())
}

pub fn save_to(config: &Config, path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = toml::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(path, text).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_falls_back_to_defaults() {
        let dir = std::env::temp_dir().join(format!("st_test_{}", std::process::id()));
        let path = dir.join("nope.toml");
        let cfg = load_from(&path);
        assert_eq!(cfg.default_mode, Mode::Unicode);
        assert_eq!(cfg.toggle_hotkey, "Ctrl+Space");
        assert_eq!(cfg.app_rules.get("WINWORD.EXE"), Some(&Mode::LegacyFm));
    }

    #[test]
    fn corrupt_file_falls_back_to_defaults() {
        let dir = std::env::temp_dir().join(format!("st_test_corrupt_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");
        std::fs::write(&path, "this is [ not = valid toml {{{").unwrap();
        let cfg = load_from(&path);
        assert_eq!(cfg.default_mode, Mode::Unicode);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn roundtrip_save_load() {
        let dir = std::env::temp_dir().join(format!("st_test_rt_{}", std::process::id()));
        let path = dir.join("nested").join("config.toml");
        let mut cfg = Config::default();
        cfg.start_with_windows = true;
        cfg.app_rules.insert("NOTEPAD.EXE".to_string(), Mode::English);
        save_to(&cfg, &path).unwrap();
        let loaded = load_from(&path);
        assert_eq!(loaded.start_with_windows, true);
        assert_eq!(loaded.app_rules.get("NOTEPAD.EXE"), Some(&Mode::English));
        assert_eq!(loaded.app_rules.get("WINWORD.EXE"), Some(&Mode::LegacyFm));
        assert_eq!(loaded.toggle_hotkey, "Ctrl+Space");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
