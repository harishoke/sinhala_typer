use std::collections::HashMap;
use std::sync::Mutex;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, GetKeyboardLayout, GetKeyboardState, SendInput, ToUnicodeEx, INPUT, INPUT_0, INPUT_KEYBOARD,
    KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VK_BACK, VK_CONTROL,
    VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F24, VK_HOME, VK_LEFT, VK_LWIN, VK_MENU, VK_NEXT,
    VK_PRIOR, VK_RIGHT, VK_RWIN, VK_SPACE, VK_TAB, VK_UP,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, KBDLLHOOKSTRUCT, LLKHF_INJECTED, MSG,
    SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_SYSKEYDOWN,
};
use crate::config::Config;
use crate::engine::legacy::{render_diff, unicode_to_fm_abhaya};
use crate::engine::phonetic::PhoneticEngine;
use crate::engine::{EditAction, Mode};
use super::app_detect::cached_foreground_exe;

const MAGIC_EXTRA: usize = 0x5A4F_4E45;

struct HookState {
    engine: PhoneticEngine,
    mode: Mode,
    sinhala_mode: Mode,
    default_mode: Mode,
    app_rules: HashMap<String, Mode>,
    screen_legacy: String,
    last_exe: Option<String>,
}

impl HookState {
    fn new() -> Self {
        HookState {
            engine: PhoneticEngine::new(),
            mode: Mode::English,
            sinhala_mode: Mode::Unicode,
            default_mode: Mode::Unicode,
            app_rules: HashMap::new(),
            screen_legacy: String::new(),
            last_exe: None,
        }
    }

    fn mode_for(&self, exe: Option<&str>) -> Mode {
        match exe.and_then(|e| self.app_rules.get(e)) {
            Some(m) => *m,
            None => self.default_mode,
        }
    }

    fn reset_word(&mut self) {
        self.engine.reset();
        self.screen_legacy.clear();
    }
}

static STATE: Mutex<Option<HookState>> = Mutex::new(None);

pub fn init(config: &Config) {
    let mut guard = STATE.lock().unwrap();
    let mut state = HookState::new();
    state.default_mode = config.default_mode;
    state.mode = config.default_mode;
    state.sinhala_mode = config.default_mode;
    state.app_rules = config.app_rules.clone();
    *guard = Some(state);
}

pub fn toggle_mode() -> Mode {
    let mut guard = STATE.lock().unwrap();
    let state = guard.as_mut().expect("hook not initialized");
    if state.mode == Mode::English {
        state.mode = state.sinhala_mode;
    } else {
        state.sinhala_mode = state.mode;
        state.mode = Mode::English;
    }
    state.reset_word();
    let mode = state.mode;
    drop(guard);
    crate::ui::tray::notify_mode(mode);
    mode
}

pub fn switch_unicode_legacy() -> Mode {
    let mut guard = STATE.lock().unwrap();
    let state = guard.as_mut().expect("hook not initialized");
    state.sinhala_mode = match state.sinhala_mode {
        Mode::LegacyFm => Mode::Unicode,
        _ => Mode::LegacyFm,
    };
    if state.mode != Mode::English {
        state.mode = state.sinhala_mode;
    }
    state.reset_word();
    let mode = state.mode;
    drop(guard);
    crate::ui::tray::notify_mode(mode);
    mode
}

pub fn current_mode() -> Mode {
    STATE
        .lock()
        .unwrap()
        .as_ref()
        .map(|s| s.mode)
        .unwrap_or(Mode::English)
}

fn vk_to_char(vk: u16, scancode: u16) -> Option<char> {
    unsafe {
        let mut keystate = [0u8; 256];
        GetKeyboardState(&mut keystate).ok()?;
        let hkl = GetKeyboardLayout(0);
        let mut buf = [0u16; 8];
        let n = ToUnicodeEx(vk as u32, scancode as u32, &keystate, &mut buf, 0x4, hkl);
        if n == 1 {
            char::from_u32(buf[0] as u32)
        } else {
            None
        }
    }
}

fn held(vk: u16) -> bool {
    unsafe { (GetKeyState(vk as i32) as u16) & 0x8000 != 0 }
}

fn is_passthrough_navigation(vk: u16) -> bool {
    vk == VK_LEFT.0
        || vk == VK_RIGHT.0
        || vk == VK_UP.0
        || vk == VK_DOWN.0
        || vk == VK_HOME.0
        || vk == VK_END.0
        || vk == VK_PRIOR.0
        || vk == VK_NEXT.0
        || vk == VK_TAB.0
        || vk == VK_ESCAPE.0
        || (vk >= VK_F1.0 && vk <= VK_F24.0)
}

fn backspace_input(down: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_BACK,
                wScan: 0,
                dwFlags: if down {
                    KEYBD_EVENT_FLAGS(0)
                } else {
                    KEYEVENTF_KEYUP
                },
                time: 0,
                dwExtraInfo: MAGIC_EXTRA,
            },
        },
    }
}

fn unicode_input(unit: u16, down: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                wScan: unit,
                dwFlags: if down {
                    KEYEVENTF_UNICODE
                } else {
                    KEYEVENTF_UNICODE | KEYEVENTF_KEYUP
                },
                time: 0,
                dwExtraInfo: MAGIC_EXTRA,
            },
        },
    }
}

fn inject(action: &EditAction) {
    let mut inputs: Vec<INPUT> = Vec::with_capacity(action.backspaces * 2 + action.insert.len() * 2);
    for _ in 0..action.backspaces {
        inputs.push(backspace_input(true));
        inputs.push(backspace_input(false));
    }
    for unit in action.insert.encode_utf16() {
        inputs.push(unicode_input(unit, true));
        inputs.push(unicode_input(unit, false));
    }
    if !inputs.is_empty() {
        unsafe {
            SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        }
    }
}

fn finish_action(state: &mut HookState, engine_action: EditAction) -> EditAction {
    match state.mode {
        Mode::LegacyFm => {
            let current = state.engine.current_unicode().to_string();
            let action = render_diff(&state.screen_legacy, &current);
            state.screen_legacy = unicode_to_fm_abhaya(&current);
            action
        }
        _ => engine_action,
    }
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    let kb = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };
    if kb.flags.contains(LLKHF_INJECTED) || kb.dwExtraInfo == MAGIC_EXTRA {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    let keydown = wparam.0 as u32 == WM_KEYDOWN || wparam.0 as u32 == WM_SYSKEYDOWN;
    if !keydown {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    let vk = kb.vkCode as u16;

    if vk == VK_SPACE.0 && held(VK_CONTROL.0) {
        toggle_mode();
        return LRESULT(1);
    }

    if is_passthrough_navigation(vk) {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    if held(VK_CONTROL.0) || held(VK_MENU.0) || held(VK_LWIN.0) || held(VK_RWIN.0) {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let mut guard = STATE.lock().unwrap();
    let state = guard.as_mut().expect("hook not initialized");
    let exe = cached_foreground_exe();
    if exe != state.last_exe {
        state.last_exe = exe.clone();
        state.reset_word();
    }
    let mode = state.mode_for(exe.as_deref());
    if state.mode != mode {
        state.mode = mode;
        state.reset_word();
    }

    if state.mode == Mode::English {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    if vk == VK_BACK.0 {
        let engine_action = state.engine.backspace();
        let had_input = !engine_action.insert.is_empty() || engine_action.backspaces > 0;
        if had_input {
            let action = finish_action(&mut *state, engine_action);
            inject(&action);
            return LRESULT(1);
        }
        return CallNextHookEx(None, code, wparam, lparam);
    }

    match vk_to_char(vk, kb.scanCode as u16) {
        Some(c) if PhoneticEngine::accepts(c) => {
            let engine_action = state.engine.push_char(c);
            let action = finish_action(&mut *state, engine_action);
            inject(&action);
            LRESULT(1)
        }
        _ => {
            state.reset_word();
            CallNextHookEx(None, code, wparam, lparam)
        }
    }
}

pub fn run() -> ! {
    unsafe {
        let hmod = GetModuleHandleW(None).expect("module handle");
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), HINSTANCE(hmod.0), 0)
            .expect("SetWindowsHookExW failed");
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        let _ = UnhookWindowsHookEx(hook);
    }
    std::process::exit(0);
}
