param([switch]$Package)
$ErrorActionPreference = "Stop"

cargo build --release
$exe = "target\release\sinhala_typer.exe"
if (-not (Test-Path $exe)) { throw "build failed: $exe not found" }
Write-Host "Built $exe (portable: run it directly, config is created in %APPDATA%\sinhala_typer)"

if ($Package) {
    $iscc = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
    if (-not (Test-Path $iscc)) { throw "Inno Setup 6 not found at $iscc — install from https://jrsoftware.org/isdl.php" }
    & $iscc "installer\sinhala_typer.iss"
    Write-Host "Installer built: installer\output\sinhala_typer_setup.exe"
}
