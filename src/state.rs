use std::sync::{Arc, RwLock};

use crate::config::Config;

#[derive(Debug)]
pub struct AppState {
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self { config }
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
