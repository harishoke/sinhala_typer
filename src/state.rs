use std::sync::{Arc, RwLock};

use crate::config::Config;
use crate::engine::Mode;

#[derive(Debug)]
pub struct AppState {
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn effective_mode(&self, fg_process: Option<&str>) -> Mode {
        if !self.config.enabled {
            return Mode::English;
        }
        match self.config.mode {
            Mode::Unicode => {
                if let Some(proc) = fg_process {
                    if !self.config.legacy_processes.is_empty() {
                        let wanted: Vec<String> = self
                            .config
                            .legacy_processes
                            .iter()
                            .map(|p| p.trim().to_ascii_lowercase())
                            .filter(|p| !p.is_empty())
                            .collect();
                        if wanted.iter().any(|p| proc.ends_with(p.as_str())) {
                            return Mode::LegacyFm;
                        }
                    }
                }
                Mode::Unicode
            }
            other => other,
        }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

pub fn shared_state(config: Config) -> SharedState {
    Arc::new(RwLock::new(AppState::new(config)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn disabled_means_english_passthrough() {
        let mut cfg = Config::default();
        cfg.enabled = false;
        cfg.mode = Mode::Unicode;
        let st = AppState::new(cfg);
        assert_eq!(st.effective_mode(Some("winword.exe")), Mode::English);
        assert_eq!(st.effective_mode(None), Mode::English);
    }

    #[test]
    fn legacy_kick_in_for_office_processes() {
        let st = AppState::new(Config::default());
        assert_eq!(st.effective_mode(Some("C:\\winword.exe")), Mode::LegacyFm);
        assert_eq!(st.effective_mode(Some("notepad.exe")), Mode::Unicode);
        assert_eq!(st.effective_mode(None), Mode::Unicode);
    }

    #[test]
    fn explicit_legacy_mode_matches_by_suffix_case_insensitive() {
        let mut cfg = Config::default();
        cfg.legacy_processes = vec!("POWERPNT.EXE".to_string());
        let st = AppState::new(cfg);
        assert_eq!(st.effective_mode(Some("c:\\office\\powerpnt.exe")), Mode::LegacyFm);
        assert_eq!(st.effective_mode(Some("powerpnt.exe.malware")), Mode::Unicode);
    }
}
