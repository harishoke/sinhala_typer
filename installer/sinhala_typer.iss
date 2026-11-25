; Sinhala Typer installer script (Inno Setup 6)
; Built by .github/workflows/build.yml; MyAppVersion is passed with /D.

#ifndef MyAppVersion
#define MyAppVersion "1.1.0"
#endif

#define MyAppName "Sinhala Typer"
#define MyAppExeName "sinhala_typer.exe"
#define MyAppPublisher "Sinhala Typer"
#define MyAppURL "https://github.com/harishoke/sinhala_typer"

[Setup]
AppId={{8C2F86E4-3E4C-4B24-9E5B-5C8F1D9A7B21}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
VersionInfoVersion={#MyAppVersion}
VersionInfoProductName={#MyAppName}
DefaultDirName={localappdata}\{#MyAppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
OutputDir={#SourcePath}\..\dist
OutputBaseFilename=sinhala_typer-setup-{#MyAppVersion}
SetupIconFile={#SourcePath}\..\assets\sinhala_typer.ico
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\{#MyAppExeName}

[Messages]
WelcomeLabel2=This will install [name/ver] on your computer.%n%nIt is a lightweight Sinhala phonetic (Singlish) typing utility that runs from the system tray.

[Files]
Source: "..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Shortcuts:"
Name: "autostart"; Description: "Start {#MyAppName} automatically with Windows"; GroupDescription: "Autostart:"

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "sinhala_typer"; ValueData: """{app}\{#MyAppExeName}"""; Flags: uninsdeletevalue; Tasks: autostart; Check: ShouldRegisterRunKey

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch {#MyAppName}"; Flags: nowait postinstall skipifsilent

[Code]
function ShouldRegisterRunKey(): Boolean;
begin
  Result := True;
end;
