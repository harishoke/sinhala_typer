use crate::engine::EditAction;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_BACK,
};

fn key_input(vk: VIRTUAL_KEY, scan: u16, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: scan,
                dwFlags: flags,
                dwExtraInfo: 0,
                time: 0,
            },
        },
    }
}

fn backspace_inputs(count: usize) -> Vec<INPUT> {
    let mut inputs = Vec::with_capacity(count * 2);
    for _ in 0..count {
        inputs.push(key_input(VK_BACK, 0, KEYBD_EVENT_FLAGS(0)));
        inputs.push(key_input(VK_BACK, 0, KEYEVENTF_KEYUP));
    }
    inputs
}

fn text_inputs(text: &str) -> Vec<INPUT> {
    // KEYEVENTF_UNICODE takes one UTF-16 code unit per event, so encode the
    // text as UTF-16 instead of truncating each char to a single u16 (which
    // corrupted every character outside the Basic Multilingual Plane).
    let units: Vec<u16> = text.encode_utf16().collect();
    let mut inputs = Vec::with_capacity(units.len() * 2);
    for unit in units {
        inputs.push(key_input(VIRTUAL_KEY(0), unit, KEYEVENTF_UNICODE));
        inputs.push(key_input(
            VIRTUAL_KEY(0),
            unit,
            KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
        ));
    }
    inputs
}

/// Synthesize the edits that transform what the user already typed on screen.
/// Backspaces are real VK_BACK presses, inserts are injected as unicode chars.
pub fn apply_edit(action: &EditAction) {
    let inputs: Vec<INPUT> = backspace_inputs(action.backspaces)
        .into_iter()
        .chain(text_inputs(&action.insert))
        .collect();
    if inputs.is_empty() {
        return;
    }
    unsafe {
        // SendInput reports how many events it actually injected. If the
        // system accepted only part of the batch (e.g. a UIPI boundary),
        // resend the remainder once and report a persistent shortfall
        // instead of silently leaving the screen half-edited.
        let mut sent = 0usize;
        for _ in 0..2 {
            if sent >= inputs.len() {
                break;
            }
            let n = SendInput(&inputs[sent..], std::mem::size_of::<INPUT>() as i32) as usize;
            if n == 0 {
                break;
            }
            sent += n;
        }
        if sent < inputs.len() {
            eprintln!(
                "sinhala_typer: SendInput injected {sent} of {} events",
                inputs.len()
            );
        }
    }
}
