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
const CLSID_KEY: &str = "Software\\Classes\\CLSID\\{BC7202CF-0D8C-42E0-B2D9-4EA4ABDA9CCA}";

pub fn dll_path() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("sinhala_typer_tsf.dll")))
        .unwrap_or_else(|| std::path::PathBuf::from("sinhala_typer_tsf.dll"))
}

/// The 32-bit build of the DLL (in an `x86` subfolder). 32-bit apps cannot
/// load the 64-bit DLL, so both bitnesses are registered.
pub fn x86_dll_path() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| {
            p.parent()
                .map(|d| d.join("x86").join("sinhala_typer_tsf.dll"))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("x86\\sinhala_typer_tsf.dll"))
}

fn syswow64_regsvr32() -> String {
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
    format!("{windir}\\SysWOW64\\regsvr32.exe")
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

fn regsvr(file: &str, args: &str) -> bool {
    let operation: Vec<u16> = "runas\0".encode_utf16().collect();
    let file_w: Vec<u16> = file.encode_utf16().chain([0]).collect();
    let params: Vec<u16> = args.encode_utf16().chain([0]).collect();
    let r = unsafe {
        ShellExecuteW(
            None,
            PCWSTR(operation.as_ptr()),
            PCWSTR(file_w.as_ptr()),
            PCWSTR(params.as_ptr()),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };
    (r.0 as usize) > 32
}

/// Register the text service (elevated): the 64-bit DLL with the system
/// regsvr32, and the 32-bit DLL with the SysWOW64 regsvr32 when present.
/// Returns Err if the 64-bit DLL is missing or the UAC prompt was declined.
pub fn register() -> Result<(), String> {
    let dll = dll_path();
    if !dll.exists() {
        return Err("sinhala_typer_tsf.dll not found next to the app".to_string());
    }
    if !regsvr("regsvr32.exe", &format!("/s \"{}\"", dll.display())) {
        return Err("registration was cancelled".to_string());
    }
    let x86 = x86_dll_path();
    if x86.exists() {
        let _ = regsvr(&syswow64_regsvr32(), &format!("/s \"{}\"", x86.display()));
    }
    Ok(())
}

/// Unregister the text service (elevated), both bitnesses.
pub fn unregister() -> Result<(), String> {
    let dll = dll_path();
    if !regsvr("regsvr32.exe", &format!("/u /s \"{}\"", dll.display())) {
        return Err("unregistration was cancelled".to_string());
    }
    let x86 = x86_dll_path();
    if x86.exists() {
        let _ = regsvr(
            &syswow64_regsvr32(),
            &format!("/u /s \"{}\"", x86.display()),
        );
    }
    Ok(())
}
