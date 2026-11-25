use windows::core::PCWSTR;
use windows::Win32::Foundation::{GetLastError, HANDLE, WIN32_ERROR};
use windows::Win32::System::Threading::CreateMutexW;

const ERROR_ALREADY_EXISTS: u32 = 183;

/// Guard for the single-instance mutex. Keep it alive for the whole process;
/// dropping it closes the handle and lets another copy start.
pub struct SingleInstance {
    _handle: HANDLE,
}

// The handle is only used as a lifetime token.
unsafe impl Send for SingleInstance {}

/// Returns `Some(guard)` for the first running copy, `None` when another
/// copy already owns the mutex (or the mutex cannot be created, in which
/// case we fail open so typing still works).
pub fn acquire() -> Option<SingleInstance> {
    let name: Vec<u16> = "Local\\SinhalaTyperSingleInstance"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    unsafe {
        match CreateMutexW(None, true, PCWSTR(name.as_ptr())) {
            Ok(handle) => {
                if GetLastError() == WIN32_ERROR(ERROR_ALREADY_EXISTS) {
                    None
                } else {
                    Some(SingleInstance { _handle: handle })
                }
            }
            Err(_) => Some(SingleInstance {
                _handle: HANDLE::default(),
            }),
        }
    }
}
