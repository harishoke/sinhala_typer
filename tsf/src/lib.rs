//! Sinhala Typer TSF text service — a real Windows IME.
//!
//! Registers two language profiles under Sinhala (si-LK) —
//! "Sinhala Typer (Unicode)" and "Sinhala Typer (FM Abhaya)" — addable in
//! Windows Settings like any other keyboard. Unlike v1.x, conversion
//! happens *inside* the text service: the service advises a key-event
//! sink, feeds keystrokes to the shared phonetic engine
//! (`sinhala_engine`), and maintains a TSF composition in the focused
//! document whose text is the rendered Sinhala (or its FM Abhaya
//! rendering, for the legacy profile). No global hook, no companion
//! process involved in typing.

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

use sinhala_engine::legacy::{fm_punctuation, unicode_to_fm_abhaya};
use sinhala_engine::phonetic::PhoneticEngine;
use windows::core::{implement, IUnknown, IUnknownImpl, Interface, GUID, HRESULT, PCWSTR};
use windows::Win32::Foundation::{
    BOOL, E_FAIL, E_POINTER, HINSTANCE, LPARAM, S_FALSE, S_OK, WPARAM,
};
use windows::Win32::System::Com::{
    CoCreateInstance, IClassFactory, IClassFactory_Impl, CLSCTX_INPROC_SERVER,
};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegSetValueExW, HKEY, HKEY_LOCAL_MACHINE,
    KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ,
};
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState;
use windows::Win32::UI::TextServices::{
    CLSID_TF_CategoryMgr, CLSID_TF_InputProcessorProfiles, ITfCategoryMgr, ITfComposition,
    ITfCompositionSink, ITfCompositionSink_Impl, ITfContext, ITfContextComposition, ITfEditSession,
    ITfEditSession_Impl, ITfInputProcessorProfileMgr, ITfInputProcessorProfiles, ITfKeyEventSink,
    ITfKeyEventSink_Impl, ITfRange, ITfSource, ITfTextInputProcessor, ITfTextInputProcessorEx,
    ITfTextInputProcessorEx_Impl, ITfTextInputProcessor_Impl, ITfThreadMgr,
    GUID_TFCAT_TIPCAP_COMLESS, GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
    GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT, GUID_TFCAT_TIPCAP_SECUREMODE,
    GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT, GUID_TFCAT_TIPCAP_UIELEMENTENABLED, GUID_TFCAT_TIP_KEYBOARD,
    TF_AE_END, TF_ANCHOR_END, TF_ANCHOR_START, TF_CONTEXT_EDIT_CONTEXT_FLAGS, TF_DEFAULT_SELECTION,
    TF_ES_READWRITE, TF_ES_SYNC, TF_INPUTPROCESSORPROFILE, TF_SELECTION, TF_SELECTIONSTYLE,
};

/// CLSID of this text service.
const CLSID_TIP: GUID = GUID::from_u128(0xbc7202cf_0d8c_42e0_b2d9_4ea4abda9cca);
/// Profile: Sinhala Typer (Unicode).
const GUID_PROFILE_UNICODE: GUID = GUID::from_u128(0x0e326edd_2577_4a7c_a6bc_a60ac3754820);
/// Profile: Sinhala Typer (FM Abhaya).
const GUID_PROFILE_FM: GUID = GUID::from_u128(0xdbe8eee4_a7ca_4063_92db_3c370ee7447c);
/// LANGID for Sinhala (Sri Lanka).
const LANGID_SI_LK: u16 = 0x045B;

static MODULE: AtomicUsize = AtomicUsize::new(0);
static OBJ_COUNT: AtomicI32 = AtomicI32::new(0);

// ---------------------------------------------------------------------------
// Diagnostics + crash safety
//
// An IME runs inside other applications: no console, no easy debugger, and
// a panic that escapes a COM entry point aborts the *host* app. So:
//  - `panic = "unwind"` (tsf/Cargo.toml) and every entry point runs inside
//    `guard`, which catches panics and logs them.
//  - Key lifecycle events are appended to %LOCALAPPDATA%\SinhalaTyper\
//    tsf.log — the primary evidence when something fails on a real machine.
// ---------------------------------------------------------------------------

fn log_line(msg: &str) {
    use std::io::Write;
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let Ok(_guard) = LOCK.lock() else {
        return;
    };
    let Some(base) = std::env::var_os("LOCALAPPDATA") else {
        return;
    };
    let dir = std::path::Path::new(&base).join("SinhalaTyper");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("tsf.log"))
    else {
        return;
    };
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let _ = writeln!(f, "[{secs}] {msg}");
}

fn guard<T>(f: impl FnOnce() -> T) -> Option<T> {
    static HOOK: std::sync::Once = std::sync::Once::new();
    HOOK.call_once(|| {
        std::panic::set_hook(Box::new(|info| {
            log_line(&format!("panic: {info}"));
        }));
    });
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).ok()
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain([0]).collect()
}

fn guid_string(g: &GUID) -> String {
    format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        g.data1,
        g.data2,
        g.data3,
        g.data4[0],
        g.data4[1],
        g.data4[2],
        g.data4[3],
        g.data4[4],
        g.data4[5],
        g.data4[6],
        g.data4[7]
    )
}

// ---------------------------------------------------------------------------
// Service state (single apartment thread — Rc is safe here)
// ---------------------------------------------------------------------------

#[derive(Default)]
struct State {
    thread_mgr: Option<ITfThreadMgr>,
    client_id: u32,
    key_sink_cookie: u32,
    comp_sink: Option<ITfCompositionSink>,
    fm: bool,
    word: String,
    composition: Option<ITfComposition>,
}

type Shared = Rc<RefCell<State>>;

fn render(word: &str, fm: bool) -> String {
    let unicode = PhoneticEngine::convert(word);
    if fm {
        unicode_to_fm_abhaya(&unicode)
    } else {
        unicode
    }
}

fn current_profile_is_fm() -> bool {
    unsafe {
        if let Ok(profiles) = CoCreateInstance::<_, ITfInputProcessorProfiles>(
            &CLSID_TF_InputProcessorProfiles,
            None,
            CLSCTX_INPROC_SERVER,
        ) {
            if let Ok(mgr) = profiles.cast::<ITfInputProcessorProfileMgr>() {
                let mut profile = TF_INPUTPROCESSORPROFILE::default();
                if mgr
                    .GetActiveProfile(&GUID_TFCAT_TIP_KEYBOARD, &mut profile)
                    .is_ok()
                {
                    return profile.guidProfile == GUID_PROFILE_FM;
                }
            }
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Edit sessions
// ---------------------------------------------------------------------------

enum Action {
    /// Set the composition text (starting the composition if needed).
    Update(String),
    /// End the composition, keeping its text.
    End,
    /// Insert committed text at the selection (no composition).
    Insert(String),
}

#[implement(ITfEditSession)]
struct EditSession {
    state: Shared,
    context: ITfContext,
    sink: ITfCompositionSink,
    action: Action,
}

impl EditSession {
    unsafe fn selection_range(&self, ec: u32) -> windows::core::Result<ITfRange> {
        let mut sel = TF_SELECTION {
            range: std::mem::ManuallyDrop::new(None),
            style: TF_SELECTIONSTYLE {
                ase: TF_AE_END,
                fInterimChar: BOOL(0),
            },
        };
        let mut fetched = 0u32;
        self.context.GetSelection(
            ec,
            TF_DEFAULT_SELECTION,
            std::slice::from_mut(&mut sel),
            &mut fetched,
        )?;
        sel.range
            .as_ref()
            .cloned()
            .ok_or_else(|| windows::core::Error::from_hresult(windows::Win32::Foundation::E_FAIL))
    }

    unsafe fn move_caret_to(&self, ec: u32, range: &ITfRange) -> windows::core::Result<()> {
        let collapsed = range.clone();
        collapsed.Collapse(ec, TF_ANCHOR_END)?;
        let sel = TF_SELECTION {
            range: std::mem::ManuallyDrop::new(Some(collapsed)),
            style: TF_SELECTIONSTYLE {
                ase: TF_AE_END,
                fInterimChar: BOOL(0),
            },
        };
        self.context.SetSelection(ec, &[sel])
    }
}

impl ITfEditSession_Impl for EditSession_Impl {
    fn DoEditSession(&self, ec: u32) -> windows::core::Result<()> {
        guard(|| -> windows::core::Result<()> {
            unsafe {
                match &self.action {
                    Action::Update(text) => {
                        let existing = self.state.borrow().composition.clone();
                        let text_w: Vec<u16> = text.encode_utf16().collect();
                        match existing {
                            Some(comp) => {
                                let range = comp.GetRange()?;
                                range.SetText(ec, 0, &text_w)?;
                                self.move_caret_to(ec, &range)?;
                            }
                            None => {
                                let range = self.selection_range(ec)?;
                                range.Collapse(ec, TF_ANCHOR_START)?;
                                let ctx_comp = self.context.cast::<ITfContextComposition>()?;
                                let comp = ctx_comp.StartComposition(ec, &range, &self.sink)?;
                                let crange = comp.GetRange()?;
                                crange.SetText(ec, 0, &text_w)?;
                                self.state.borrow_mut().composition = Some(comp);
                                self.move_caret_to(ec, &crange)?;
                            }
                        }
                    }
                    Action::End => {
                        let existing = self.state.borrow_mut().composition.take();
                        if let Some(comp) = existing {
                            comp.EndComposition(ec)?;
                        }
                    }
                    Action::Insert(text) => {
                        let range = self.selection_range(ec)?;
                        let text_w: Vec<u16> = text.encode_utf16().collect();
                        range.SetText(ec, 0, &text_w)?;
                        self.move_caret_to(ec, &range)?;
                    }
                }
            }
            Ok(())
        })
        .unwrap_or_else(|| {
            log_line("PANIC in DoEditSession");
            Err(E_FAIL.into())
        })
    }
}

// ---------------------------------------------------------------------------
// Text service
// ---------------------------------------------------------------------------

#[implement(
    ITfTextInputProcessor,
    ITfTextInputProcessorEx,
    ITfKeyEventSink,
    ITfCompositionSink
)]
struct TextService {
    state: Shared,
}

const VK_BACK: u32 = 0x08;
const VK_SHIFT: i32 = 0x10;
const VK_CONTROL: i32 = 0x11;
const VK_MENU: i32 = 0x12;
const VK_CAPITAL: i32 = 0x14;

fn key_down(vk: i32) -> bool {
    unsafe { (GetKeyState(vk) as u16 & 0x8000) != 0 }
}

/// The letter a letter-key VK represents right now, honouring Shift and
/// Caps Lock. Returns None for non-letter keys.
fn letter_of(vk: u32) -> Option<char> {
    if !(0x41..=0x5A).contains(&vk) {
        return None;
    }
    let base = (b'a' + (vk as u8 - 0x41)) as char;
    let shift = key_down(VK_SHIFT);
    let caps = unsafe { (GetKeyState(VK_CAPITAL) as u16 & 0x0001) != 0 };
    Some(if shift ^ caps {
        base.to_ascii_uppercase()
    } else {
        base
    })
}

/// US-layout character for digit/punctuation keys (base, shifted), used
/// only by the FM profile to find punctuation substitutions.
fn punct_char(vk: u32) -> Option<char> {
    let shift = key_down(VK_SHIFT);
    let pair = match vk {
        0x30 => ('0', ')'),
        0x31 => ('1', '!'),
        0x32 => ('2', '@'),
        0x33 => ('3', '#'),
        0x34 => ('4', '$'),
        0x35 => ('5', '%'),
        0x36 => ('6', '^'),
        0x37 => ('7', '&'),
        0x38 => ('8', '*'),
        0x39 => ('9', '('),
        0xBA => (';', ':'),
        0xBB => ('=', '+'),
        0xBC => (',', '<'),
        0xBD => ('-', '_'),
        0xBE => ('.', '>'),
        0xBF => ('/', '?'),
        0xDB => ('[', '{'),
        0xDD => (']', '}'),
        0xDE => ('\'', '"'),
        _ => return None,
    };
    Some(if shift { pair.1 } else { pair.0 })
}

impl TextService {
    fn request(&self, ctx: &ITfContext, action: Action) {
        let (tid, sink) = {
            let st = self.state.borrow();
            (st.client_id, st.comp_sink.clone())
        };
        let Some(sink) = sink else {
            return;
        };
        let session: ITfEditSession = EditSession {
            state: self.state.clone(),
            context: ctx.clone(),
            sink,
            action,
        }
        .into();
        let flags = TF_CONTEXT_EDIT_CONTEXT_FLAGS(TF_ES_SYNC.0 | TF_ES_READWRITE.0);
        if let Err(e) = unsafe { ctx.RequestEditSession(tid, &session, flags) } {
            log_line(&format!("RequestEditSession failed: {e:?}"));
        }
    }

    fn end_composition(&self, ctx: &ITfContext) {
        if self.state.borrow().composition.is_some() || !self.state.borrow().word.is_empty() {
            self.state.borrow_mut().word.clear();
            self.request(ctx, Action::End);
        }
    }

    /// Decide whether a key-down is ours, without mutating state.
    fn would_eat(&self, vk: u32) -> bool {
        guard(|| self.would_eat_inner(vk)).unwrap_or(false)
    }

    fn would_eat_inner(&self, vk: u32) -> bool {
        if key_down(VK_CONTROL) || key_down(VK_MENU) {
            return false;
        }
        let st = self.state.borrow();
        if vk == VK_BACK {
            return !st.word.is_empty();
        }
        if let Some(c) = letter_of(vk) {
            return PhoneticEngine::accepts(c);
        }
        if st.fm {
            if let Some(ch) = punct_char(vk) {
                return fm_punctuation(ch).is_some();
            }
        }
        false
    }
}

impl ITfTextInputProcessor_Impl for TextService_Impl {
    fn Activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> windows::core::Result<()> {
        guard(|| -> windows::core::Result<()> {
            log_line("Activate");
            let Some(tm) = ptim else {
                return Ok(());
            };
            {
                let mut st = self.state.borrow_mut();
                st.thread_mgr = Some(tm.clone());
                st.client_id = tid;
                st.fm = current_profile_is_fm();
                st.comp_sink = Some(self.to_interface::<ITfCompositionSink>());
            }
            let fm = self.state.borrow().fm;
            if let Ok(source) = tm.cast::<ITfSource>() {
                let sink: IUnknown = self.to_interface::<ITfKeyEventSink>().into();
                match unsafe { source.AdviseSink(&ITfKeyEventSink::IID, &sink) } {
                    Ok(cookie) => {
                        self.state.borrow_mut().key_sink_cookie = cookie;
                        log_line(&format!("Activate ok: key sink advised (fm={fm})"));
                    }
                    Err(e) => log_line(&format!("Activate: AdviseSink failed: {e:?}")),
                }
            } else {
                log_line("Activate: ITfSource cast failed");
            }
            Ok(())
        })
        .unwrap_or_else(|| {
            log_line("PANIC in Activate");
            Err(E_FAIL.into())
        })
    }

    fn Deactivate(&self) -> windows::core::Result<()> {
        guard(|| -> windows::core::Result<()> {
            log_line("Deactivate");
            let (tm, cookie) = {
                let st = self.state.borrow();
                (st.thread_mgr.clone(), st.key_sink_cookie)
            };
            if let Some(tm) = tm {
                if let Ok(source) = tm.cast::<ITfSource>() {
                    let _ = unsafe { source.UnadviseSink(cookie) };
                }
            }
            let mut st = self.state.borrow_mut();
            st.thread_mgr = None;
            st.comp_sink = None;
            st.key_sink_cookie = 0;
            st.word.clear();
            st.composition = None;
            Ok(())
        })
        .unwrap_or_else(|| {
            log_line("PANIC in Deactivate");
            Err(E_FAIL.into())
        })
    }
}

impl ITfTextInputProcessorEx_Impl for TextService_Impl {
    fn ActivateEx(
        &self,
        ptim: Option<&ITfThreadMgr>,
        tid: u32,
        _dwflags: u32,
    ) -> windows::core::Result<()> {
        ITfTextInputProcessor_Impl::Activate(self, ptim, tid)
    }
}

impl ITfCompositionSink_Impl for TextService_Impl {
    fn OnCompositionTerminated(
        &self,
        _ecwrite: u32,
        _pcomposition: Option<&ITfComposition>,
    ) -> windows::core::Result<()> {
        guard(|| -> windows::core::Result<()> {
            let mut st = self.state.borrow_mut();
            st.composition = None;
            st.word.clear();
            Ok(())
        })
        .unwrap_or_else(|| {
            log_line("PANIC in OnCompositionTerminated");
            Err(E_FAIL.into())
        })
    }
}

impl ITfKeyEventSink_Impl for TextService_Impl {
    fn OnSetFocus(&self, fforeground: BOOL) -> windows::core::Result<()> {
        guard(|| -> windows::core::Result<()> {
            if !fforeground.as_bool() {
                let mut st = self.state.borrow_mut();
                st.word.clear();
                st.composition = None;
            }
            Ok(())
        })
        .unwrap_or_else(|| {
            log_line("PANIC in OnSetFocus");
            Err(E_FAIL.into())
        })
    }

    fn OnTestKeyDown(
        &self,
        _pic: Option<&ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> windows::core::Result<BOOL> {
        Ok(BOOL(self.would_eat(wparam.0 as u32) as i32))
    }

    fn OnTestKeyUp(
        &self,
        _pic: Option<&ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> windows::core::Result<BOOL> {
        Ok(BOOL(self.would_eat(wparam.0 as u32) as i32))
    }

    fn OnKeyDown(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> windows::core::Result<BOOL> {
        guard(|| -> windows::core::Result<BOOL> {
            let Some(ctx) = pic else {
                return Ok(BOOL(0));
            };
            let vk = wparam.0 as u32;
            static KEY_EVENTS: AtomicUsize = AtomicUsize::new(0);
            let n = KEY_EVENTS.fetch_add(1, Ordering::SeqCst);
            if n < 10 {
                log_line(&format!("OnKeyDown #{n}: vk={vk:#04x}"));
            }
            if key_down(VK_CONTROL) || key_down(VK_MENU) {
                self.end_composition(ctx);
                return Ok(BOOL(0));
            }
            if vk == VK_BACK {
                let has_word = !self.state.borrow().word.is_empty();
                if !has_word {
                    return Ok(BOOL(0));
                }
                let (text, empty) = {
                    let mut st = self.state.borrow_mut();
                    st.word.pop();
                    (render(&st.word, st.fm), st.word.is_empty())
                };
                if empty {
                    self.request(ctx, Action::Update(String::new()));
                    self.request(ctx, Action::End);
                    self.state.borrow_mut().composition = None;
                } else {
                    self.request(ctx, Action::Update(text));
                }
                return Ok(BOOL(1));
            }
            if let Some(c) = letter_of(vk) {
                if !PhoneticEngine::accepts(c) {
                    self.end_composition(ctx);
                    return Ok(BOOL(0));
                }
                let text = {
                    let mut st = self.state.borrow_mut();
                    st.word.push(c);
                    render(&st.word, st.fm)
                };
                self.request(ctx, Action::Update(text));
                return Ok(BOOL(1));
            }
            // FM profile: punctuation that the FM font would draw as letters
            // is committed as the substitute glyph instead.
            let is_fm = self.state.borrow().fm;
            if is_fm {
                if let Some(ch) = punct_char(vk) {
                    if let Some(sub) = fm_punctuation(ch) {
                        self.end_composition(ctx);
                        self.request(ctx, Action::Insert(sub.to_string()));
                        return Ok(BOOL(1));
                    }
                }
            }
            // Anything else (space, enter, arrows...) commits the word.
            self.end_composition(ctx);
            Ok(BOOL(0))
        })
        .unwrap_or_else(|| {
            log_line("PANIC in OnKeyDown");
            Ok(BOOL(0))
        })
    }

    fn OnKeyUp(
        &self,
        _pic: Option<&ITfContext>,
        wparam: WPARAM,
        _lparam: LPARAM,
    ) -> windows::core::Result<BOOL> {
        Ok(BOOL(self.would_eat(wparam.0 as u32) as i32))
    }

    fn OnPreservedKey(
        &self,
        _pic: Option<&ITfContext>,
        _rguid: *const GUID,
    ) -> windows::core::Result<BOOL> {
        Ok(BOOL(0))
    }
}

impl Drop for TextService {
    fn drop(&mut self) {
        OBJ_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
}

// ---------------------------------------------------------------------------
// Class factory
// ---------------------------------------------------------------------------

#[implement(IClassFactory)]
struct ClassFactory;

impl IClassFactory_Impl for ClassFactory_Impl {
    fn CreateInstance(
        &self,
        _outer: Option<&IUnknown>,
        iid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> windows::core::Result<()> {
        guard(|| -> windows::core::Result<()> {
            if ppv.is_null() || iid.is_null() {
                return Err(E_POINTER.into());
            }
            unsafe {
                *ppv = std::ptr::null_mut();
            }
            let service: ITfTextInputProcessor = TextService {
                state: Rc::new(RefCell::new(State::default())),
            }
            .into();
            OBJ_COUNT.fetch_add(1, Ordering::SeqCst);
            unsafe {
                let wanted = *iid;
                if wanted == ITfTextInputProcessor::IID || wanted == IUnknown::IID {
                    *ppv = service.into_raw();
                } else if wanted == ITfTextInputProcessorEx::IID {
                    match service.cast::<ITfTextInputProcessorEx>() {
                        Ok(ex) => *ppv = ex.into_raw(),
                        Err(e) => {
                            OBJ_COUNT.fetch_sub(1, Ordering::SeqCst);
                            return Err(e);
                        }
                    }
                } else if wanted == ITfKeyEventSink::IID {
                    match service.cast::<ITfKeyEventSink>() {
                        Ok(s) => *ppv = s.into_raw(),
                        Err(e) => {
                            OBJ_COUNT.fetch_sub(1, Ordering::SeqCst);
                            return Err(e);
                        }
                    }
                } else if wanted == ITfCompositionSink::IID {
                    match service.cast::<ITfCompositionSink>() {
                        Ok(s) => *ppv = s.into_raw(),
                        Err(e) => {
                            OBJ_COUNT.fetch_sub(1, Ordering::SeqCst);
                            return Err(e);
                        }
                    }
                } else {
                    OBJ_COUNT.fetch_sub(1, Ordering::SeqCst);
                    return Err(windows::core::Error::from_hresult(
                        windows::Win32::Foundation::E_NOINTERFACE,
                    ));
                }
            }
            Ok(())
        })
        .unwrap_or_else(|| {
            log_line("PANIC in CreateInstance");
            Err(E_FAIL.into())
        })
    }

    fn LockServer(&self, _lock: BOOL) -> windows::core::Result<()> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// DLL entry points and registration
// ---------------------------------------------------------------------------

#[no_mangle]
extern "system" fn DllMain(hinst: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    const DLL_PROCESS_ATTACH: u32 = 1;
    if reason == DLL_PROCESS_ATTACH {
        MODULE.store(hinst.0 as usize, Ordering::SeqCst);
    }
    BOOL(1)
}

#[no_mangle]
unsafe extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    match guard(|| unsafe { get_class_object_inner(rclsid, riid, ppv) }) {
        Some(hr) => hr,
        None => {
            log_line("PANIC in DllGetClassObject");
            E_FAIL
        }
    }
}

unsafe fn get_class_object_inner(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    *ppv = std::ptr::null_mut();
    if rclsid.is_null() || *rclsid != CLSID_TIP {
        return windows::Win32::Foundation::CLASS_E_CLASSNOTAVAILABLE;
    }
    let factory: IClassFactory = ClassFactory.into();
    if riid.is_null() {
        return E_POINTER;
    }
    let wanted = *riid;
    if wanted == IClassFactory::IID || wanted == IUnknown::IID {
        *ppv = factory.into_raw();
        S_OK
    } else {
        windows::Win32::Foundation::E_NOINTERFACE
    }
}

#[no_mangle]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    if OBJ_COUNT.load(Ordering::SeqCst) == 0 {
        S_OK
    } else {
        S_FALSE
    }
}

fn module_path() -> Vec<u16> {
    let h = HINSTANCE(MODULE.load(Ordering::SeqCst) as *mut c_void);
    let mut buf = vec![0u16; 1024];
    let len = unsafe { GetModuleFileNameW(h, &mut buf) } as usize;
    buf.truncate(len + 1);
    buf
}

fn set_reg_sz(key: HKEY, name: Option<&str>, value: &[u16]) {
    let name_w = name.map(wide);
    let name_ptr = name_w
        .as_ref()
        .map(|v| PCWSTR(v.as_ptr()))
        .unwrap_or_else(PCWSTR::null);
    let bytes: &[u8] =
        unsafe { std::slice::from_raw_parts(value.as_ptr() as *const u8, value.len() * 2) };
    unsafe {
        let _ = RegSetValueExW(key, name_ptr, 0, REG_SZ, Some(bytes));
    }
}

fn register_com_server() -> windows::core::Result<()> {
    unsafe {
        let path = format!(
            "Software\\Classes\\CLSID\\{}\\InprocServer32",
            guid_string(&CLSID_TIP)
        );
        let path_w = wide(&path);
        let mut key = HKEY::default();
        RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(path_w.as_ptr()),
            0,
            PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut key,
            None,
        )
        .ok()?;
        let dll = module_path();
        set_reg_sz(key, None, &dll);
        set_reg_sz(key, Some("ThreadingModel"), &wide("Apartment"));
        let _ = RegCloseKey(key);
        Ok(())
    }
}

fn unregister_com_server() {
    unsafe {
        let path = format!("Software\\Classes\\CLSID\\{}", guid_string(&CLSID_TIP));
        let path_w = wide(&path);
        let _ = RegDeleteTreeW(HKEY_LOCAL_MACHINE, PCWSTR(path_w.as_ptr()));
    }
}

#[no_mangle]
extern "system" fn DllRegisterServer() -> HRESULT {
    match guard(register_server_inner) {
        Some(hr) => {
            log_line(&format!("DllRegisterServer -> {hr:?}"));
            hr
        }
        None => {
            log_line("PANIC in DllRegisterServer");
            E_FAIL
        }
    }
}

fn register_server_inner() -> HRESULT {
    let result = (|| -> windows::core::Result<()> {
        register_com_server()?;
        unsafe {
            let profiles: ITfInputProcessorProfiles =
                CoCreateInstance(&CLSID_TF_InputProcessorProfiles, None, CLSCTX_INPROC_SERVER)?;
            profiles.Register(&CLSID_TIP)?;
            let desc_u = wide("Sinhala Typer (Unicode)");
            let desc_f = wide("Sinhala Typer (FM Abhaya)");
            let empty: &[u16] = &[];
            profiles.AddLanguageProfile(
                &CLSID_TIP,
                LANGID_SI_LK,
                &GUID_PROFILE_UNICODE,
                &desc_u[..desc_u.len() - 1],
                empty,
                0,
            )?;
            profiles.AddLanguageProfile(
                &CLSID_TIP,
                LANGID_SI_LK,
                &GUID_PROFILE_FM,
                &desc_f[..desc_f.len() - 1],
                empty,
                0,
            )?;
            let _ = profiles.EnableLanguageProfile(
                &CLSID_TIP,
                LANGID_SI_LK,
                &GUID_PROFILE_UNICODE,
                true,
            );
            let _ =
                profiles.EnableLanguageProfile(&CLSID_TIP, LANGID_SI_LK, &GUID_PROFILE_FM, true);
            // Also register through the profile-manager API — the modern
            // path the Windows Settings keyboard list is built from.
            // Best effort: AddLanguageProfile above may already suffice,
            // but doing both removes a whole class of "registered yet not
            // listed" failures. Results are logged either way.
            if let Ok(mgr) = profiles.cast::<ITfInputProcessorProfileMgr>() {
                let hkl = windows::Win32::UI::Input::KeyboardAndMouse::HKL::default();
                let r1 = mgr.RegisterProfile(
                    &CLSID_TIP,
                    LANGID_SI_LK,
                    &GUID_PROFILE_UNICODE,
                    &desc_u[..desc_u.len() - 1],
                    empty,
                    0,
                    hkl,
                    0,
                    BOOL(1),
                    0,
                );
                log_line(&format!("RegisterProfile(Unicode) -> {r1:?}"));
                let r2 = mgr.RegisterProfile(
                    &CLSID_TIP,
                    LANGID_SI_LK,
                    &GUID_PROFILE_FM,
                    &desc_f[..desc_f.len() - 1],
                    empty,
                    0,
                    hkl,
                    0,
                    BOOL(1),
                    0,
                );
                log_line(&format!("RegisterProfile(FM) -> {r2:?}"));
            } else {
                log_line("ProfileMgr cast failed; AddLanguageProfile only");
            }
            if let Ok(catmgr) = CoCreateInstance::<_, ITfCategoryMgr>(
                &CLSID_TF_CategoryMgr,
                None,
                CLSCTX_INPROC_SERVER,
            ) {
                let _ = catmgr.RegisterCategory(&CLSID_TIP, &GUID_TFCAT_TIP_KEYBOARD, &CLSID_TIP);
                // Capability categories: without IMMERSIVESUPPORT the
                // keyboard is unavailable in Store/UWP apps, without
                // SYSTRAYSUPPORT it may not appear in the tray switcher.
                for cap in [
                    GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
                    GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT,
                    GUID_TFCAT_TIPCAP_SECUREMODE,
                    GUID_TFCAT_TIPCAP_UIELEMENTENABLED,
                    GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT,
                    GUID_TFCAT_TIPCAP_COMLESS,
                ] {
                    let _ = catmgr.RegisterCategory(&CLSID_TIP, &cap, &CLSID_TIP);
                }
            }
        }
        Ok(())
    })();
    match result {
        Ok(()) => S_OK,
        Err(e) => e.code(),
    }
}

#[no_mangle]
extern "system" fn DllUnregisterServer() -> HRESULT {
    match guard(unregister_server_inner) {
        Some(hr) => {
            log_line(&format!("DllUnregisterServer -> {hr:?}"));
            hr
        }
        None => {
            log_line("PANIC in DllUnregisterServer");
            E_FAIL
        }
    }
}

fn unregister_server_inner() -> HRESULT {
    unsafe {
        if let Ok(profiles) = CoCreateInstance::<_, ITfInputProcessorProfiles>(
            &CLSID_TF_InputProcessorProfiles,
            None,
            CLSCTX_INPROC_SERVER,
        ) {
            let _ = profiles.RemoveLanguageProfile(&CLSID_TIP, LANGID_SI_LK, &GUID_PROFILE_UNICODE);
            let _ = profiles.RemoveLanguageProfile(&CLSID_TIP, LANGID_SI_LK, &GUID_PROFILE_FM);
            let _ = profiles.Unregister(&CLSID_TIP);
        }
        if let Ok(catmgr) =
            CoCreateInstance::<_, ITfCategoryMgr>(&CLSID_TF_CategoryMgr, None, CLSCTX_INPROC_SERVER)
        {
            let _ = catmgr.UnregisterCategory(&CLSID_TIP, &GUID_TFCAT_TIP_KEYBOARD, &CLSID_TIP);
            for cap in [
                GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
                GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT,
                GUID_TFCAT_TIPCAP_SECUREMODE,
                GUID_TFCAT_TIPCAP_UIELEMENTENABLED,
                GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT,
                GUID_TFCAT_TIPCAP_COMLESS,
            ] {
                let _ = catmgr.UnregisterCategory(&CLSID_TIP, &cap, &CLSID_TIP);
            }
        }
    }
    unregister_com_server();
    S_OK
}
