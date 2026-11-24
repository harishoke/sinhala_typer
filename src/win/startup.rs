use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, ERROR_FILE_NOT_FOUND};
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
    HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_SZ,
};
use windows::Win32::System::Threading::CreateMutexW;

const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const APP_VALUE: &str = "sinhala_typer";
const MUTEX_NAME: &str = "Global\\sinhala_typer_single_instance";

pub fn acquire_single_instance() -> bool {
    unsafe {
        let name = HSTRING::from(MUTEX_NAME);
        match CreateMutexW(None, true, PCWSTR(name.as_ptr())) {
            Ok(handle) => {
                let already = GetLastError() == ERROR_ALREADY_EXISTS;
                if already {
                    let _ = CloseHandle(handle);
                }
                !already
            }
            Err(_) => true,
        }
    }
}

fn exe_command_line() -> String {
    match std::env::current_exe() {
        Ok(path) => format!("\"{}\"", path.display()),
        Err(_) => String::new(),
    }
}

pub fn set_start_with_windows(enabled: bool) -> Result<(), String> {
    unsafe {
        let key = HSTRING::from(RUN_KEY);
        let mut hkey = HKEY::default();
        let status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(key.as_ptr()),
            0,
            KEY_READ | KEY_WRITE,
            &mut hkey,
        );
        if status.0 != 0 {
            return Err(format!("RegOpenKeyExW failed: {}", status.0));
        }
        let name = HSTRING::from(APP_VALUE);
        let result = if enabled {
            let value = exe_command_line();
            let mut bytes: Vec<u8> = value
                .encode_utf16()
                .flat_map(|u| u.to_le_bytes())
                .collect();
            bytes.extend_from_slice(&[0, 0]);
            RegSetValueExW(hkey, PCWSTR(name.as_ptr()), 0, REG_SZ, Some(&bytes))
        } else {
            RegDeleteValueW(hkey, PCWSTR(name.as_ptr()))
        };
        let _ = RegCloseKey(hkey);
        if result.0 != 0 && result != ERROR_FILE_NOT_FOUND {
            return Err(format!("registry write failed: {}", result.0));
        }
        Ok(())
    }
}

pub fn is_start_with_windows() -> bool {
    unsafe {
        let key = HSTRING::from(RUN_KEY);
        let mut hkey = HKEY::default();
        if RegOpenKeyExW(HKEY_CURRENT_USER, PCWSTR(key.as_ptr()), 0, KEY_READ, &mut hkey).0 != 0 {
            return false;
        }
        let name = HSTRING::from(APP_VALUE);
        let mut buf = [0u8; 512];
        let mut len = buf.len() as u32;
        let mut value_type = windows::Win32::System::Registry::REG_VALUE_TYPE(0);
        let status = RegQueryValueExW(
            hkey,
            PCWSTR(name.as_ptr()),
            None,
            Some(&mut value_type),
            Some(buf.as_mut_ptr()),
            Some(&mut len),
        );
        let _ = RegCloseKey(hkey);
        status.0 == 0
    }
}
