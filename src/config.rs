use std::fmt;
use std::path::PathBuf;

use crate::engine::Mode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hotkey {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub win: bool,
    pub key: char,
}

impl Default for Hotkey {
    fn default() -> Self {
        Hotkey::parse("ctrl+alt+s").expect("default hotkey parses")
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts: Vec<&str> = Vec::new();
        if self.ctrl {
            parts.push("ctrl");
        }
        if self.alt {
            parts.push("alt");
        }
        if self.shift {
            parts.push("shift");
        }
        if self.win {
            parts.push("win");
        }
        let key = self.key.to_lowercase().to_string();
        parts.push(&key);
        write!(f, "{}", parts.join("+"))
    }
}

impl Hotkey {
    pub fn parse(s: &str) -> Result<Hotkey, String> {
        let mut hk = Hotkey {
            ctrl: false,
            alt: false,
            shift: false,
            win: false,
            key: '\0',
        };
        for part in s
            .trim()
            .split(|c: char| c == '+' || c.is_whitespace())
            .filter(|p| !p.is_empty())
        {
            match part.to_ascii_lowercase().as_str() {
                "ctrl" | "control" => hk.ctrl = true,
                "alt" => hk.alt = true,
                "shift" => hk.shift = true,
                "win" | "meta" | "super" => hk.win = true,
                _ => {
                    let mut chars = part.chars();
                    let c = chars.next();
                    let rest_empty = chars.next().is_none();
                    match c {
                        Some(c) if c.is_ascii_alphanumeric() && rest_empty => hk.key = c,
                        _ => return Err(format!("invalid hotkey part: {part}")),
                    }
                }
            }
        }
        if hk.key == '\0' {
            return Err("hotkey needs a main key".to_string());
        }
        if !hk.ctrl && !hk.alt && !hk.shift && !hk.win {
            return Err("hotkey needs at least one modifier".to_string());
        }
        Ok(hk)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub enabled: bool,
    pub mode: Mode,
    pub hotkey: String,
    pub legacy_processes: Vec<String>,
    pub autostart: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: Mode::Unicode,
            hotkey: "ctrl+alt+s".to_string(),
            legacy_processes: vec![
                "winword.exe".to_string(),
                "excel.exe".to_string(),
                "powerpnt.exe".to_string(),
            ],
            autostart: false,
        }
    }
}

impl Config {
    pub fn config_dir() -> PathBuf {
        #[cfg(target_os = "windows")]
        let base = std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        #[cfg(not(target_os = "windows"))]
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("sinhala_typer")
    }

    pub fn load() -> Config {
        let path = Self::config_dir().join("settings.toml");
        let Ok(text) = std::fs::read_to_string(&path) else {
            return Config::default();
        };
        match toml::from_str(&text) {
            Ok(cfg) => cfg,
            Err(err) => {
                eprintln!("sinhala_typer: ignoring invalid config {}: {err}", path.display());
                Config::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let dir = Self::config_dir();
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let text = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(dir.join("settings.toml"), text).map_err(|e| e.to_string())
    }

    pub fn hotkey(&self) -> Hotkey {
        Hotkey::parse(&self.hotkey).unwrap_or_default()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_hotkey() {
        let hk = Hotkey::parse("Ctrl+Alt+S").unwrap();
        assert!(hk.ctrl && hk.alt && !hk.shift && !hk.win);
        assert_eq!(hk.key, 'S');
        assert_eq!(hk.to_string(), "ctrl+alt+s");
    }

    #[test]
    fn parse_shift_f9_style_digit() {
        let hk = Hotkey::parse("shift+1").unwrap();
        assert!(hk.shift);
        assert_eq!(hk.key, '1');
    }

    #[test]
    fn reject_no_modifier() {
        assert!(Hotkey::parse("s").is_err());
    }

    #[test]
    fn reject_no_key() {
        assert!(Hotkey::parse("ctrl+alt").is_err());
    }

    #[test]
    fn reject_multi_char_key() {
        assert!(Hotkey::parse("ctrl+ab").is_err());
    }

    #[test]
    fn default_config_roundtrip() {
        let cfg = Config::default();
        let text = toml::to_string_pretty(&cfg).unwrap();
        let back: Config = toml::from_str(&text).unwrap();
        assert_eq!(back.hotkey, cfg.hotkey);
        assert_eq!(back.mode, Mode::Unicode);
        assert_eq!(back.hotkey().key, 's');
    }

    #[test]
    fn unknown_fields_tolerated_by_default() {
        let text = "enabled = false\nmode = \"English\"\nhotkey = \"shift+9\"\nlegacy_processes = []\nautostart = true\n";
        let cfg: Config = toml::from_str(text).unwrap();
        assert!(!cfg.enabled);
        assert_eq!(cfg.mode, Mode::English);
    }
}
