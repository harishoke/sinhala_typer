use std::sync::atomic::{AtomicPtr, Ordering};
use windows::core::PWSTR;
use windows::Win32::Foundation::{CloseHandle, HMODULE, HWND};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Accessibility::{SetWinEventHook, HWINEVENTHOOK};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetForegroundWindow, GetMessageW, GetWindowThreadProcessId, TranslateMessage,
    EVENT_SYSTEM_FOREGROUND, MSG, WINEVENT_OUTOFCONTEXT,
};

static EXE_CACHE: AtomicPtr<u16> = AtomicPtr::new(std::ptr::null_mut());

pub(crate) fn exe_name_from_path(path: &str) -> String {
    path.rsplit(['\\', '/'])
        .next()
        .unwrap_or(path)
        .to_uppercase()
}

pub fn foreground_exe_name() -> Option<String> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return None;
        }
        let handle = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(h) => h,
            Err(_) => return None,
        };
        let mut buf = [0u16; 1024];
        let mut len = buf.len() as u32;
        let result = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(buf.as_mut_ptr()),
            &mut len,
        );
        let _ = CloseHandle(handle);
        if result.is_err() {
            return None;
        }
        Some(exe_name_from_path(&String::from_utf16_lossy(
            &buf[..len as usize],
        )))
    }
}

fn store_cached(name: &str) {
    let mut units: Vec<u16> = name.encode_utf16().collect();
    units.push(0);
    let ptr = Box::into_raw(units.into_boxed_slice()) as *mut u16;
    let old = EXE_CACHE.swap(ptr, Ordering::AcqRel);
    if !old.is_null() {
        unsafe {
            let mut len = 0usize;
            while *old.add(len) != 0 {
                len += 1;
            }
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(old, len + 1)));
        }
    }
}

pub fn cached_foreground_exe() -> Option<String> {
    let ptr = EXE_CACHE.load(Ordering::Acquire);
    if ptr.is_null() {
        return None;
    }
    unsafe {
        let mut len = 0usize;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        Some(String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len)))
    }
}

unsafe extern "system" fn win_event_proc(
    _hook: HWINEVENTHOOK,
    _event: u32,
    _hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_thread: u32,
    _time: u32,
) {
    if let Some(name) = foreground_exe_name() {
        store_cached(&name);
    }
}

pub fn start_foreground_watcher() {
    std::thread::spawn(|| unsafe {
        let _hook = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            HMODULE::default(),
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exe_name_extraction_uppercases_file_name() {
        assert_eq!(exe_name_from_path("C:\\Windows\\System32\\notepad.EXE"), "NOTEPAD.EXE");
        assert_eq!(exe_name_from_path("C:\\Program Files\\App\\winword.exe"), "WINWORD.EXE");
        assert_eq!(exe_name_from_path("plain.exe"), "PLAIN.EXE");
    }
}
