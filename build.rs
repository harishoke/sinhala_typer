fn main() {
    // Embed the application icon (and version info) into the Windows .exe,
    // so Explorer, the taskbar and shortcuts show the real artwork instead
    // of the default placeholder. Build scripts run on the host, so this
    // only takes effect for a native Windows build; cross-compilation
    // check builds skip it.
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/sinhala_typer.ico");
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
}
