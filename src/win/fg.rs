use windows::core::PWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

/// LANG_SINHALESE primary language id (si-LK and si-IN layouts).
const PRIMARY_LANG_SINHALESE: u32 = 0x5B;

/// True when the foreground window's active Windows keyboard layout is a
/// Sinhala one (chosen through the Windows language bar / Win+Space). In
/// that case Windows itself is converting keystrokes, so Sinhala Typer must
/// pass keys through untouched instead of converting a second time.
pub fn foreground_is_sinhala_layout() -> bool {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0.is_null() {
            return false;
        }
        let mut pid: u32 = 0;
        let tid =
            windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if tid == 0 {
            return false;
        }
        let hkl = GetKeyboardLayout(tid);
        (hkl.0 as u32 & 0x3FF) == PRIMARY_LANG_SINHALESE
    }
}

/// The Sinhala Typer TSF profile currently active in Windows, as written by
/// the text-service DLL (tsf/) to HKCU\Software\SinhalaTyper\ActiveProfile:
/// "unicode", "fm", "none", or None when the integration was never used.
/// The hook follows *changes* of this value so picking a keyboard in the
/// Windows language bar switches the app mode.
pub fn tsf_profile() -> Option<String> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_CURRENT_USER, KEY_READ,
        REG_VALUE_TYPE,
    };
    unsafe {
        let subkey: Vec<u16> = "Software\\SinhalaTyper\0".encode_utf16().collect();
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut key,
        )
        .is_err()
        {
            return None;
        }
        let name: Vec<u16> = "ActiveProfile\0".encode_utf16().collect();
        let mut ty = REG_VALUE_TYPE::default();
        let mut buf = [0u8; 128];
        let mut len = buf.len() as u32;
        let res = RegQueryValueExW(
            key,
            PCWSTR(name.as_ptr()),
            None,
            Some(&mut ty),
            Some(buf.as_mut_ptr()),
            Some(&mut len),
        );
        let _ = RegCloseKey(key);
        if res.is_err() || (len as usize) > buf.len() {
            return None;
        }
        let units: Vec<u16> = buf[..len as usize]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| u16::from_le_bytes(*c))
            .collect();
        let s = String::from_utf16_lossy(&units)
            .trim_end_matches('\0')
            .to_string();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}

/// Lower-cased image name (e.g. `winword.exe`) of the foreground process.
pub fn foreground_process_name() -> Option<String> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }
        let mut pid: u32 = 0;
        windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return None;
        }
        let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
            return None;
        };
        let name = image_name(handle);
        let _ = CloseHandle(handle);
        name
    }
}

unsafe fn image_name(handle: HANDLE) -> Option<String> {
    let mut buf = [0u16; 1024];
    let mut len = buf.len() as u32;
    let ok = QueryFullProcessImageNameW(
        handle,
        PROCESS_NAME_WIN32,
        PWSTR(buf.as_mut_ptr()),
        &mut len,
    );
    if ok.is_err() || (len as usize) > buf.len() {
        return None;
    }
    let full = String::from_utf16_lossy(&buf[..len as usize]);
    Some(
        full.rsplit(['\\', '/'])
            .next()
            .unwrap_or(&full)
            .to_ascii_lowercase(),
    )
}
