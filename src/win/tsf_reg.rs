//! Registration of the TSF text service (sinhala_typer_tsf.dll, built from
//! the tsf/ crate and shipped next to the exe). Registering a text service
//! writes to HKLM, so it runs through an elevated regsvr32 (one UAC prompt)
//! when the user asks for it in Settings — the app itself stays per-user.

use windows::core::PCWSTR;
use windows::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ,
};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

/// CLSID of the Sinhala Typer text service (matches tsf/src/lib.rs).
const CLSID_KEY: &str =
    "Software\\Classes\\CLSID\\{BC7202CF-0D8C-42E0-B2D9-4EA4ABDA9CCA}";

pub fn dll_path() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("sinhala_typer_tsf.dll")))
        .unwrap_or_else(|| std::path::PathBuf::from("sinhala_typer_tsf.dll"))
}

pub fn is_registered() -> bool {
    unsafe {
        let subkey: Vec<u16> = CLSID_KEY.encode_utf16().chain([0]).collect();
        let mut key = HKEY::default();
        let ok = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut key,
        )
        .is_ok();
        if ok {
            let _ = RegCloseKey(key);
        }
        ok
    }
}

fn regsvr(args: &str) -> bool {
    let operation: Vec<u16> = "runas\0".encode_utf16().collect();
    let file: Vec<u16> = "regsvr32.exe\0".encode_utf16().collect();
    let params: Vec<u16> = args.encode_utf16().chain([0]).collect();
    let r = unsafe {
        ShellExecuteW(
            None,
            PCWSTR(operation.as_ptr()),
            PCWSTR(file.as_ptr()),
            PCWSTR(params.as_ptr()),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };
    (r.0 as usize) > 32
}

/// Register the text service (elevated). Returns false if the DLL is
/// missing or the UAC prompt was declined.
pub fn register() -> Result<(), String> {
    let dll = dll_path();
    if !dll.exists() {
        return Err("sinhala_typer_tsf.dll not found next to the app".to_string());
    }
    if regsvr(&format!("/s \"{}\"", dll.display())) {
        Ok(())
    } else {
        Err("registration was cancelled".to_string())
    }
}

/// Unregister the text service (elevated).
pub fn unregister() -> Result<(), String> {
    let dll = dll_path();
    if regsvr(&format!("/u /s \"{}\"", dll.display())) {
        Ok(())
    } else {
        Err("unregistration was cancelled".to_string())
    }
}
