use windows::core::PCWSTR;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
    HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_SET_VALUE, REG_SAM_FLAGS, REG_SZ,
};

const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const VALUE_NAME: &str = "sinhala_typer";

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn open_run_key(access: REG_SAM_FLAGS) -> Option<HKEY> {
    let mut hkey = HKEY::default();
    let rc = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(wide(RUN_KEY).as_ptr()),
            0,
            access,
            &mut hkey,
        )
    };
    if rc == WIN32_ERROR(0) {
        Some(hkey)
    } else {
        None
    }
}

pub fn set_autostart(enable: bool) -> bool {
    if enable {
        let Ok(exe) = std::env::current_exe() else {
            return false;
        };
        let quoted = format!("\"{}\"", exe.to_string_lossy());
        let units = wide(&quoted);
        let data: Vec<u8> = units.iter().flat_map(|u| u.to_le_bytes()).collect();
        let Some(hkey) = open_run_key(KEY_SET_VALUE) else {
            return false;
        };
        let rc = unsafe {
            RegSetValueExW(
                hkey,
                PCWSTR(wide(VALUE_NAME).as_ptr()),
                0,
                REG_SZ,
                Some(&data),
            )
        };
        let _ = unsafe { RegCloseKey(hkey) };
        rc == WIN32_ERROR(0)
    } else {
        let Some(hkey) = open_run_key(KEY_SET_VALUE) else {
            return false;
        };
        let rc = unsafe { RegDeleteValueW(hkey, PCWSTR(wide(VALUE_NAME).as_ptr())) };
        let _ = unsafe { RegCloseKey(hkey) };
        rc == WIN32_ERROR(0)
    }
}

pub fn is_enabled() -> bool {
    let Some(hkey) = open_run_key(KEY_QUERY_VALUE) else {
        return false;
    };
    let mut data = [0u8; 1024];
    let mut len = data.len() as u32;
    let rc = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(wide(VALUE_NAME).as_ptr()),
            None,
            None,
            Some(data.as_mut_ptr()),
            Some(&mut len),
        )
    };
    let _ = unsafe { RegCloseKey(hkey) };
    rc == WIN32_ERROR(0) && len > 2
}
