use std::time::Duration;

use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_QUIT,
};

use crate::state::SharedState;
use crate::ui;

/// Fallback message pump used when the settings window cannot be created
/// (e.g. no usable GPU/OpenGL). Keeps the tray icon and keyboard hook alive.
pub fn pump_forever(state: SharedState) {
    unsafe {
        let mut msg = MSG::default();
        loop {
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    return;
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            ui::poll_events(&state, None);
            std::thread::sleep(Duration::from_millis(200));
        }
    }
}
