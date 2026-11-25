use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::config::Hotkey;
use crate::engine::legacy::render_diff;
use crate::engine::phonetic::PhoneticEngine;
use crate::engine::{EditAction, Mode};
use crate::state::{lock_read, lock_write, SharedState};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CAPITAL};

use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG,
    WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_QUIT, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

const HC_ACTION: i32 = 0;
const LLKHF_INJECTED: u32 = 0x10;
const VK_BACK_CODE: u32 = 0x08;

static STOP: AtomicBool = AtomicBool::new(false);
static HOOK_THREAD_ID: OnceLock<u32> = OnceLock::new();

struct HookCtx {
    state: SharedState,
    engine: PhoneticEngine,
    word: String,
    legacy_screen: String,
    ctrl: bool,
    alt: bool,
    shift: bool,
    win: bool,
    fg_cache: Option<(Instant, Option<String>)>,
    last_fg: Option<String>,
}

thread_local! {
    static CTX: RefCell<Option<HookCtx>> = const { RefCell::new(None) };
}

impl HookCtx {
    fn new(state: SharedState) -> Self {
        Self {
            state,
            engine: PhoneticEngine::default(),
            word: String::new(),
            legacy_screen: String::new(),
            ctrl: false,
            alt: false,
            shift: false,
            win: false,
            fg_cache: None,
            last_fg: None,
        }
    }

    /// Full hotkey currently being held, parsed fresh from shared config.
    fn hotkey_held(&self, vk: u32) -> bool {
        let key_vk;
        let Hotkey {
            ctrl,
            alt,
            shift,
            win,
            ..
        } = {
            let st = lock_read(&self.state);
            let hk = st.config.hotkey();
            key_vk = char_to_vk(hk.key);
            hk
        };
        key_vk == vk
            && ctrl == self.ctrl
            && alt == self.alt
            && shift == self.shift
            && win == self.win
    }

    fn cycle_mode(&mut self) {
        let mut st = lock_write(&self.state);
        st.config.mode = match st.config.mode {
            Mode::English => Mode::Unicode,
            Mode::Unicode => Mode::LegacyFm,
            Mode::LegacyFm => Mode::English,
        };
        let _ = st.config.save();
    }

    fn fg_name(&mut self) -> Option<String> {
        let cached = self
            .fg_cache
            .as_ref()
            .filter(|(t, _)| t.elapsed() < Duration::from_millis(750))
            .map(|(_, name)| name.clone());
        match cached {
            Some(name) => name,
            None => {
                let name = super::fg::foreground_process_name();
                self.fg_cache = Some((Instant::now(), name.clone()));
                name
            }
        }
    }

    fn effective_mode(&mut self) -> Mode {
        let needs_fg = {
            let st = lock_read(&self.state);
            st.config.enabled
                && st.config.mode == Mode::Unicode
                && !st.config.legacy_processes.is_empty()
        };
        if !needs_fg {
            return lock_read(&self.state).effective_mode(None);
        }
        let fg = self.fg_name();
        // Switching apps must never carry a half-typed word (or its legacy
        // screen image) into the new window.
        if fg != self.last_fg {
            self.last_fg = fg.clone();
            self.reset_buffers();
        }
        lock_read(&self.state).effective_mode(fg.as_deref())
    }

    fn reset_buffers(&mut self) {
        self.engine.reset();
        self.word.clear();
        self.legacy_screen.clear();
    }

    fn emit_unicode(&mut self, action: &EditAction) {
        super::input::apply_edit(action);
    }

    fn emit_legacy(&mut self, action: &EditAction) {
        for _ in 0..action.backspaces {
            self.word.pop();
        }
        self.word.push_str(&action.insert);
        let diff = render_diff(&self.legacy_screen, &self.word);
        super::input::apply_edit(&diff);
        for _ in 0..diff.backspaces {
            self.legacy_screen.pop();
        }
        self.legacy_screen.push_str(&diff.insert);
    }

    /// Handle a non-modifier key-down. Returns true when the event is consumed.
    fn on_key_down(&mut self, vk: u32) -> bool {
        if self.hotkey_held(vk) {
            self.cycle_mode();
            self.reset_buffers();
            return true;
        }
        if !lock_read(&self.state).config.enabled {
            self.reset_buffers();
            return false;
        }
        // Ctrl / Alt / Win + a key is a shortcut (copy, paste, save, ...),
        // never text. The configured mode hotkey was handled above; every
        // other modified keystroke passes through untouched.
        if self.ctrl || self.alt || self.win {
            self.reset_buffers();
            return false;
        }
        let mode = self.effective_mode();
        if mode == Mode::English {
            self.reset_buffers();
            return false;
        }
        if (b'A' as u32..=b'Z' as u32).contains(&vk) {
            // The engine is case-sensitive (t/T, d/D, sh/Sh, Ru, H ...), so
            // type the character the user actually produced: Shift and the
            // Caps Lock toggle decide upper vs lower case.
            let caps_lock = unsafe { (GetAsyncKeyState(VK_CAPITAL.0 as i32) & 1) != 0 };
            let upper = self.shift ^ caps_lock;
            let c = if upper {
                (vk as u8) as char
            } else {
                (vk as u8 + b'a' - b'A') as char
            };
            let c = if PhoneticEngine::accepts(c) {
                c
            } else {
                c.to_ascii_lowercase()
            };
            if PhoneticEngine::accepts(c) {
                let action = self.engine.push_char(c);
                match mode {
                    Mode::LegacyFm => self.emit_legacy(&action),
                    _ => self.emit_unicode(&action),
                }
                return true;
            }
            self.reset_buffers();
            return false;
        }
        if vk == VK_BACK_CODE {
            if self.engine.is_active() {
                let action = self.engine.backspace();
                match mode {
                    Mode::LegacyFm => self.emit_legacy(&action),
                    _ => self.emit_unicode(&action),
                }
                return true;
            }
            return false;
        }
        self.reset_buffers();
        false
    }
}

fn char_to_vk(c: char) -> u32 {
    if c.is_ascii_digit() {
        c as u32
    } else if c.is_ascii_alphabetic() {
        c.to_ascii_uppercase() as u32
    } else {
        0
    }
}

fn vk_modifier(vk: u32) -> Option<fn(&mut HookCtx, bool)> {
    match vk {
        0x10 | 0xA0 | 0xA1 => Some(|ctx, down| ctx.shift = down),
        0x11 | 0xA2 | 0xA3 => Some(|ctx, down| ctx.ctrl = down),
        0x12 | 0xA4 | 0xA5 => Some(|ctx, down| ctx.alt = down),
        0x5B | 0x5C => Some(|ctx, down| ctx.win = down),
        _ => None,
    }
}

unsafe extern "system" fn hook_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if ncode != HC_ACTION {
        return CallNextHookEx(None, ncode, wparam, lparam);
    }
    if lparam.0 == 0 {
        return CallNextHookEx(None, ncode, wparam, lparam);
    }
    let kb = lparam.0 as *const KBDLLHOOKSTRUCT;
    let kb = &*kb;
    if kb.flags.0 & LLKHF_INJECTED != 0 {
        return CallNextHookEx(None, ncode, wparam, lparam);
    }
    let msg = wparam.0 as u32;
    let down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
    let up = msg == WM_KEYUP || msg == WM_SYSKEYUP;

    let mut handled = false;
    CTX.with(|cell| {
        let mut ctx = cell.borrow_mut();
        let Some(ctx) = ctx.as_mut() else { return };
        let _ = up;
        if let Some(set_mod) = vk_modifier(kb.vkCode) {
            set_mod(ctx, down);
            return;
        }
        if down {
            handled = ctx.on_key_down(kb.vkCode);
        }
    });
    if handled {
        LRESULT(1)
    } else {
        CallNextHookEx(None, ncode, wparam, lparam)
    }
}

pub fn spawn(state: SharedState) -> JoinHandle<()> {
    STOP.store(false, Ordering::SeqCst);
    thread::Builder::new()
        .name("kbd-hook".into())
        .spawn(move || run_hook_thread(state))
        .expect("spawn keyboard hook thread")
}

fn run_hook_thread(state: SharedState) {
    unsafe {
        let _ = HOOK_THREAD_ID.set(GetCurrentThreadId());
        CTX.with(|cell| {
            *cell.borrow_mut() = Some(HookCtx::new(state));
        });
        let module = GetModuleHandleW(None).unwrap_or_default();
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), module, 0);
        let Ok(hook) = hook else {
            eprintln!("sinhala_typer: SetWindowsHookExW failed");
            return;
        };
        let mut msg = MSG::default();
        while !STOP.load(Ordering::SeqCst) {
            if GetMessageW(&mut msg, None, 0, 0).0 == 0 {
                break;
            }
            if msg.message == WM_QUIT {
                break;
            }
        }
        let _ = UnhookWindowsHookEx(hook);
    }
}

/// Tells the hook thread to unhook and exit.
pub fn stop() {
    STOP.store(true, Ordering::SeqCst);
    if let Some(&tid) = HOOK_THREAD_ID.get() {
        unsafe {
            let _ = windows::Win32::UI::WindowsAndMessaging::PostThreadMessageW(
                tid,
                WM_QUIT,
                WPARAM(0),
                LPARAM(0),
            );
        }
    }
}
