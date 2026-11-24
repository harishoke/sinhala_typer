use std::sync::Mutex;
use std::time::Duration;
use muda::{CheckMenuItem, IsMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use crate::config::{self, Config};
use crate::engine::Mode;

static MODE_TX: Mutex<Option<std::sync::mpsc::Sender<Mode>>> = Mutex::new(None);
static SETTINGS_CHILD: Mutex<Option<std::process::Child>> = Mutex::new(None);

const SIZE: u32 = 32;

const GLYPH_E: [&str; 7] = ["11111", "10000", "10000", "11110", "10000", "10000", "11111"];
const GLYPH_S: [&str; 7] = ["01111", "10000", "10000", "01110", "00001", "00001", "11110"];
const GLYPH_L: [&str; 7] = ["10000", "10000", "10000", "10000", "10000", "10000", "11111"];

fn make_icon(bg: [u8; 3], glyph: &[&str; 7]) -> Icon {
    let mut rgba = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let i = ((y * SIZE + x) * 4) as usize;
            let corner = (x < 4 || x >= SIZE - 4) && (y < 4 || y >= SIZE - 4);
            let [r, g, b] = if corner { [240u8, 240u8, 240u8] } else { bg };
            rgba[i] = r;
            rgba[i + 1] = g;
            rgba[i + 2] = b;
            rgba[i + 3] = 255;
        }
    }
    let ox = (SIZE - 5) / 2;
    let oy = (SIZE - 7) / 2;
    for (gy, row) in glyph.iter().enumerate() {
        for (gx, c) in row.chars().enumerate() {
            if c == '1' {
                let i = (((oy + gy as u32) * SIZE + (ox + gx as u32)) * 4) as usize;
                rgba[i] = 255;
                rgba[i + 1] = 255;
                rgba[i + 2] = 255;
                rgba[i + 3] = 255;
            }
        }
    }
    Icon::from_rgba(rgba, SIZE, SIZE).expect("icon")
}

fn icon_for(mode: Mode) -> Icon {
    match mode {
        Mode::English => make_icon([43, 108, 176], &GLYPH_E),
        Mode::Unicode => make_icon([221, 107, 32], &GLYPH_S),
        Mode::LegacyFm => make_icon([47, 133, 90], &GLYPH_L),
    }
}

fn tooltip_for(mode: Mode) -> &'static str {
    match mode {
        Mode::English => "sinhala_typer — EN",
        Mode::Unicode => "sinhala_typer — Sinhala (Unicode)",
        Mode::LegacyFm => "sinhala_typer — Sinhala (Legacy FM)",
    }
}

pub fn notify_mode(mode: Mode) {
    if let Ok(guard) = MODE_TX.lock() {
        if let Some(tx) = guard.as_ref() {
            let _ = tx.send(mode);
        }
    }
}

fn build_menu(cfg: &Config) -> Menu {
    let menu = Menu::new();
    let toggle = MenuItem::with_id("toggle", "Toggle Sinhala / English (Ctrl+Space)", true, None);
    let switch = MenuItem::with_id("switch", "Switch Unicode / Legacy mode", true, None);
    let settings = MenuItem::with_id("settings", "Settings & Info", true, None);
    let startup = CheckMenuItem::with_id(
        "startup",
        "Start with Windows",
        true,
        cfg.start_with_windows,
        None,
    );
    let exit = MenuItem::with_id("exit", "Exit", true, None);
    let sep = PredefinedMenuItem::separator();
    menu.append_items(&[
        &toggle as &dyn IsMenuItem,
        &switch,
        &settings,
        &sep,
        &startup,
        &exit,
    ])
    .expect("menu");
    menu
}

fn open_settings_once() {
    let mut guard = SETTINGS_CHILD.lock().unwrap();
    if let Some(child) = guard.as_mut() {
        if child.try_wait().ok().flatten().is_none() {
            return;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Ok(child) = std::process::Command::new(exe).arg("--settings").spawn() {
            *guard = Some(child);
        }
    }
}

fn handle_menu(id: &str) {
    match id {
        "toggle" => {
            crate::win::hook::toggle_mode();
        }
        "switch" => {
            crate::win::hook::switch_unicode_legacy();
        }
        "settings" => open_settings_once(),
        "startup" => {
            let mut cfg = config::load();
            cfg.start_with_windows = !cfg.start_with_windows;
            let _ = crate::win::startup::set_start_with_windows(cfg.start_with_windows);
            let _ = config::save(&cfg);
        }
        "exit" => std::process::exit(0),
        _ => {}
    }
}

pub fn run(cfg: Config) -> ! {
    let (mode_tx, mode_rx) = std::sync::mpsc::channel::<Mode>();
    *MODE_TX.lock().unwrap() = Some(mode_tx);

    let menu = build_menu(&cfg);
    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip(tooltip_for(crate::win::hook::current_mode()))
        .with_icon(icon_for(crate::win::hook::current_mode()))
        .build()
        .expect("failed to create tray icon");

    let menu_events = MenuEvent::receiver();
    let tray_events = TrayIconEvent::receiver().clone();

    loop {
        while let Ok(event) = menu_events.try_recv() {
            handle_menu(&event.id.0);
        }
        while let Ok(mode) = mode_rx.try_recv() {
            let _ = tray.set_icon(Some(icon_for(mode)));
            let _ = tray.set_tooltip(Some(tooltip_for(mode)));
        }
        while let Ok(event) = tray_events.try_recv() {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                crate::win::hook::toggle_mode();
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
