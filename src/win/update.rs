//! In-app updates: check the GitHub Releases feed, and on request download
//! the new installer and hand over to it (the installer replaces the exe;
//! the app exits so the file is not locked).

use std::sync::Mutex;

const LATEST_API: &str =
    "https://api.github.com/repos/harishoke/sinhala_typer/releases/latest";

#[derive(Clone, Debug, PartialEq)]
pub enum UpdateState {
    Idle,
    Checking,
    UpToDate,
    Available(String),
    Downloading,
    Error(String),
}

static STATE: Mutex<UpdateState> = Mutex::new(UpdateState::Idle);

pub fn state() -> UpdateState {
    STATE.lock().map(|s| s.clone()).unwrap_or(UpdateState::Idle)
}

fn set(state: UpdateState) {
    if let Ok(mut s) = STATE.lock() {
        *s = state;
    }
}

/// Parse "1.3.0" / "v1.3.0" into a comparable tuple.
fn parse_ver(s: &str) -> Option<(u32, u32, u32)> {
    let s = s.trim().trim_start_matches('v');
    let mut parts = s.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    Some((major, minor, patch))
}

/// Blocking check; returns the newer version string if one exists.
pub fn check_latest() -> Result<Option<String>, String> {
    let resp = ureq::get(LATEST_API)
        .set("User-Agent", "SinhalaTyper")
        .call()
        .map_err(|e| format!("network error: {e}"))?;
    let body: serde_json::Value = resp
        .into_json()
        .map_err(|e| format!("bad response: {e}"))?;
    let tag = body
        .get("tag_name")
        .and_then(|t| t.as_str())
        .ok_or("release has no tag")?;
    let latest = parse_ver(tag).ok_or("could not parse latest version")?;
    let current = parse_ver(env!("CARGO_PKG_VERSION")).unwrap_or((0, 0, 0));
    if latest > current {
        Ok(Some(tag.trim_start_matches('v').to_string()))
    } else {
        Ok(None)
    }
}

/// Run a check on a background thread, publishing progress to `state()`.
pub fn start_check() {
    if state() == UpdateState::Checking {
        return;
    }
    set(UpdateState::Checking);
    std::thread::spawn(|| match check_latest() {
        Ok(Some(v)) => set(UpdateState::Available(v)),
        Ok(None) => set(UpdateState::UpToDate),
        Err(e) => set(UpdateState::Error(e)),
    });
}

/// Download the installer for `version` and launch it, then exit the app
/// so the installer can replace the running exe.
pub fn start_install(version: String) {
    set(UpdateState::Downloading);
    std::thread::spawn(move || {
        let url = format!(
            "https://github.com/harishoke/sinhala_typer/releases/download/v{version}/sinhala_typer-setup-{version}.exe"
        );
        let result = (|| -> Result<std::path::PathBuf, String> {
            let resp = ureq::get(&url)
                .set("User-Agent", "SinhalaTyper")
                .call()
                .map_err(|e| format!("download failed: {e}"))?;
            let dest = std::env::temp_dir().join(format!("sinhala_typer-setup-{version}.exe"));
            let mut file =
                std::fs::File::create(&dest).map_err(|e| format!("cannot write file: {e}"))?;
            std::io::copy(&mut resp.into_reader(), &mut file)
                .map_err(|e| format!("download failed: {e}"))?;
            Ok(dest)
        })();
        match result {
            Ok(path) => {
                let _ = std::process::Command::new(&path).spawn();
                crate::ui::request_exit();
                std::process::exit(0);
            }
            Err(e) => set(UpdateState::Error(e)),
        }
    });
}
