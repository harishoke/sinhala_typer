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
                        // Compare whole file names (or a path ending in the
                        // name), never a bare suffix: "xwinword.exe" must not
                        // match the configured "winword.exe".
                        let proc_name = proc.trim().to_ascii_lowercase().replace('\\', "/");
                        let wanted: Vec<String> = self
                            .config
                            .legacy_processes
                            .iter()
                            .map(|p| p.trim().to_ascii_lowercase().replace('\\', "/"))
                            .filter(|p| !p.is_empty())
                            .collect();
                        let is_legacy = wanted.iter().any(|p| {
                            let want_name = p.rsplit('/').next().unwrap_or(p.as_str());
                            let have_name =
                                proc_name.rsplit('/').next().unwrap_or(proc_name.as_str());
                            have_name == want_name
                        });
                        if is_legacy {
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

/// Read the shared state, recovering the value if a previous panic poisoned
/// the lock instead of crashing the hook / UI threads as well.
pub fn lock_read(state: &SharedState) -> std::sync::RwLockReadGuard<'_, AppState> {
    state
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Write counterpart of [`lock_read`].
pub fn lock_write(state: &SharedState) -> std::sync::RwLockWriteGuard<'_, AppState> {
    state
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub fn shared_state(config: Config) -> SharedState {
    Arc::new(RwLock::new(AppState::new(config)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn disabled_means_english_passthrough() {
        let cfg = Config {
            enabled: false,
            mode: Mode::Unicode,
            ..Config::default()
        };
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
    fn legacy_match_is_case_insensitive_for_the_process_name() {
        let st = AppState::new(Config::default());
        assert_eq!(st.effective_mode(Some("WINWORD.EXE")), Mode::LegacyFm);
        assert_eq!(st.effective_mode(Some("xwinword.exe")), Mode::Unicode);
        assert_eq!(st.effective_mode(Some("winword.exe.bak")), Mode::Unicode);
    }

    #[test]
    fn explicit_legacy_mode_matches_by_suffix_case_insensitive() {
        let cfg = Config {
            legacy_processes: vec!["POWERPNT.EXE".to_string()],
            ..Config::default()
        };
        let st = AppState::new(cfg);
        assert_eq!(
            st.effective_mode(Some("c:\\office\\powerpnt.exe")),
            Mode::LegacyFm
        );
        assert_eq!(
            st.effective_mode(Some("powerpnt.exe.malware")),
            Mode::Unicode
        );
    }
}
