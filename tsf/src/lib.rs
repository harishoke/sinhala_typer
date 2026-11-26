//! Sinhala Typer TSF text service.
//!
//! Registers two language profiles under Sinhala (si-LK) —
//! "Sinhala Typer (Unicode)" and "Sinhala Typer (FM Abhaya)" — so the user
//! can add them in Windows Settings like any other keyboard and switch
//! with Win+Space / the language bar. The DLL does not convert keystrokes
//! itself: on activation it records the chosen profile in
//! HKCU\Software\SinhalaTyper\ActiveProfile, and the Sinhala Typer tray
//! app (which owns the keyboard hook and the conversion engines) follows
//! it. On deactivation it records "none" if one of our profiles was last.

use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

use windows::core::{implement, Interface, GUID, HRESULT, PCWSTR};
use windows::Win32::Foundation::{BOOL, E_POINTER, HINSTANCE, S_FALSE, S_OK};
use windows::Win32::System::Com::{
    CoCreateInstance, IClassFactory, IClassFactory_Impl, CLSCTX_INPROC_SERVER,
};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegSetValueExW, HKEY, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, KEY_SET_VALUE, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ,
};
use windows::Win32::UI::TextServices::{
    CLSID_TF_CategoryMgr, CLSID_TF_InputProcessorProfiles, GUID_TFCAT_TIP_KEYBOARD,
    ITfCategoryMgr, ITfInputProcessorProfileMgr, ITfInputProcessorProfiles,
    ITfTextInputProcessor, ITfTextInputProcessorEx, ITfTextInputProcessorEx_Impl,
    ITfTextInputProcessor_Impl, ITfThreadMgr, TF_INPUTPROCESSORPROFILE,
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

fn profile_name(g: &GUID) -> Option<&'static str> {
    if *g == GUID_PROFILE_UNICODE {
        Some("unicode")
    } else if *g == GUID_PROFILE_FM {
        Some("fm")
    } else {
        None
    }
}

/// Record the active profile for the tray app (HKCU\Software\SinhalaTyper).
fn write_active_profile(value: &str) {
    unsafe {
        let subkey = wide("Software\\SinhalaTyper");
        let mut key = HKEY::default();
        if RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            None,
            &mut key,
            None,
        )
        .is_err()
        {
            return;
        }
        let name = wide("ActiveProfile");
        let val = wide(value);
        let bytes: &[u8] =
            std::slice::from_raw_parts(val.as_ptr() as *const u8, val.len() * 2);
        let _ = RegSetValueExW(key, PCWSTR(name.as_ptr()), 0, REG_SZ, Some(bytes));
        let _ = RegCloseKey(key);
    }
}

// ---------------------------------------------------------------------------
// Text service object
// ---------------------------------------------------------------------------

#[implement(ITfTextInputProcessor, ITfTextInputProcessorEx)]
struct TextService;

fn activate_with(guid_profile: &GUID) {
    if let Some(name) = profile_name(guid_profile) {
        write_active_profile(name);
    }
}

/// Ask TSF which language profile is current and record it if it is one
/// of ours. (This binding's ActivateEx carries flags, not the profile
/// GUID, so the current-profile query is the reliable route.)
fn record_current_profile() {
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
                    activate_with(&profile.guidProfile);
                }
            }
        }
    }
}

impl ITfTextInputProcessor_Impl for TextService_Impl {
    fn Activate(
        &self,
        _ptim: Option<&ITfThreadMgr>,
        _tid: u32,
    ) -> windows::core::Result<()> {
        record_current_profile();
        Ok(())
    }

    fn Deactivate(&self) -> windows::core::Result<()> {
        write_active_profile("none");
        Ok(())
    }
}

impl ITfTextInputProcessorEx_Impl for TextService_Impl {
    fn ActivateEx(
        &self,
        _ptim: Option<&ITfThreadMgr>,
        _tid: u32,
        _dwflags: u32,
    ) -> windows::core::Result<()> {
        record_current_profile();
        Ok(())
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
        _outer: Option<&windows::core::IUnknown>,
        iid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> windows::core::Result<()> {
        if ppv.is_null() {
            return Err(E_POINTER.into());
        }
        unsafe {
            *ppv = std::ptr::null_mut();
        }
        let service: ITfTextInputProcessor = TextService.into();
        OBJ_COUNT.fetch_add(1, Ordering::SeqCst);
        unsafe {
            if iid.is_null() {
                OBJ_COUNT.fetch_sub(1, Ordering::SeqCst);
                return Err(E_POINTER.into());
            }
            let wanted = *iid;
            if wanted == ITfTextInputProcessor::IID
                || wanted == windows::core::IUnknown::IID
            {
                *ppv = service.into_raw();
            } else if wanted == ITfTextInputProcessorEx::IID {
                match service.cast::<ITfTextInputProcessorEx>() {
                    Ok(ex) => *ppv = ex.into_raw(),
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
    }

    fn LockServer(&self, _lock: BOOL) -> windows::core::Result<()> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// DLL entry points
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
    unsafe {
        let wanted = *riid;
        if wanted == IClassFactory::IID || wanted == windows::core::IUnknown::IID {
            *ppv = factory.into_raw();
            S_OK
        } else {
            windows::Win32::Foundation::E_NOINTERFACE
        }
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
    let result = (|| -> windows::core::Result<()> {
        register_com_server()?;
        unsafe {
            let profiles: ITfInputProcessorProfiles = CoCreateInstance(
                &CLSID_TF_InputProcessorProfiles,
                None,
                CLSCTX_INPROC_SERVER,
            )?;
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
            let _ = profiles.EnableLanguageProfile(
                &CLSID_TIP,
                LANGID_SI_LK,
                &GUID_PROFILE_FM,
                true,
            );
            if let Ok(catmgr) = CoCreateInstance::<_, ITfCategoryMgr>(
                &CLSID_TF_CategoryMgr,
                None,
                CLSCTX_INPROC_SERVER,
            ) {
                let _ = catmgr.RegisterCategory(
                    &CLSID_TIP,
                    &GUID_TFCAT_TIP_KEYBOARD,
                    &CLSID_TIP,
                );
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
    unsafe {
        if let Ok(profiles) = CoCreateInstance::<_, ITfInputProcessorProfiles>(
            &CLSID_TF_InputProcessorProfiles,
            None,
            CLSCTX_INPROC_SERVER,
        ) {
            let _ = profiles.RemoveLanguageProfile(
                &CLSID_TIP,
                LANGID_SI_LK,
                &GUID_PROFILE_UNICODE,
            );
            let _ = profiles.RemoveLanguageProfile(&CLSID_TIP, LANGID_SI_LK, &GUID_PROFILE_FM);
            let _ = profiles.Unregister(&CLSID_TIP);
        }
        if let Ok(catmgr) = CoCreateInstance::<_, ITfCategoryMgr>(
            &CLSID_TF_CategoryMgr,
            None,
            CLSCTX_INPROC_SERVER,
        ) {
            let _ = catmgr.UnregisterCategory(&CLSID_TIP, &GUID_TFCAT_TIP_KEYBOARD, &CLSID_TIP);
        }
    }
    unregister_com_server();
    S_OK
}
