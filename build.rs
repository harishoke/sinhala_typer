fn main() {
    // Embed the application icon (and version info) into the Windows .exe,
    // so Explorer, the taskbar and shortcuts show the real artwork instead
    // of the default placeholder. Build scripts run on the host, so this
    // only takes effect for a native Windows build; cross-compilation
    // check builds skip it.
    //
    // The .ico itself is stored in the repo as base64 text
    // (`assets/sinhala_typer.ico.base64.a` + `.b`) so the whole tree stays
    // text-only; decode it to a scratch file for winres here.
    #[cfg(windows)]
    {
        match decode_icon() {
            Ok(icon_path) => {
                let mut res = winres::WindowsResource::new();
                res.set_icon(icon_path.to_str().unwrap_or_default());
                res.set("ProductName", "Sinhala Typer");
                res.set(
                    "FileDescription",
                    "Sinhala Typer - Sinhala phonetic typing for Windows",
                );
                res.set("CompanyName", "Sinhala Typer");
                if let Err(err) = res.compile() {
                    println!("cargo:warning=could not embed Windows resources: {err}");
                }
            }
            Err(err) => println!("cargo:warning=could not decode app icon: {err}"),
        }
    }
}

#[cfg(windows)]
fn decode_icon() -> Result<std::path::PathBuf, String> {
    let encoded =
        std::fs::read_to_string("assets/sinhala_typer.ico.base64").map_err(|e| e.to_string())?;
    let bytes = base64_decode(&encoded)?;
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| e.to_string())?)
        .join("sinhala_typer.ico");
    std::fs::write(&out, bytes).map_err(|e| e.to_string())?;
    Ok(out)
}

/// Minimal standard-alphabet base64 decoder (whitespace-tolerant); the icon
/// blob is small and trusted (it lives in this repository).
#[cfg(windows)]
fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    fn value(c: u8) -> Result<u32, String> {
        match c {
            b'A'..=b'Z' => Ok((c - b'A') as u32),
            b'a'..=b'z' => Ok((c - b'a' + 26) as u32),
            b'0'..=b'9' => Ok((c - b'0' + 52) as u32),
            b'+' => Ok(62),
            b'/' => Ok(63),
            _ => Err(format!("invalid base64 byte: {c}")),
        }
    }
    let clean: Vec<u8> = input.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        if chunk.len() < 2 {
            return Err("truncated base64".to_string());
        }
        let pad = chunk.iter().filter(|&&c| c == b'=').count();
        let mut n: u32 = 0;
        for (i, &c) in chunk.iter().enumerate() {
            n |= if c == b'=' {
                0
            } else {
                value(c)? << (18 - 6 * i)
            };
        }
        out.push((n >> 16) as u8);
        if pad < 2 {
            out.push((n >> 8) as u8);
        }
        if pad < 1 {
            out.push(n as u8);
        }
    }
    Ok(out)
}
