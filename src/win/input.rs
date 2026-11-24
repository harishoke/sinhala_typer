use crate::engine::EditAction;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
    INPUT_KEYBOARD, VIRTUAL_KEY, VK_BACK,
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
    let mut inputs = Vec::with_capacity(text.chars().count() * 2);
    for c in text.chars() {
        let scan = c as u32 as u16;
        inputs.push(key_input(
            VIRTUAL_KEY(0),
            scan,
            KEYEVENTF_UNICODE,
        ));
        inputs.push(key_input(
            VIRTUAL_KEY(0),
            scan,
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
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}
