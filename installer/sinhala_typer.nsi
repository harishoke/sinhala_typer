; Sinhala Typer installer (NSIS) - Linux-buildable equivalent of the
; Inno Setup script (sinhala_typer.iss). The GitHub Actions pipeline uses
; the Inno script; this one produces the same install layout and is used
; for local / cross-built releases.
; Build: makensis /DVERSION=1.1.0 /DEXE=<path to sinhala_typer.exe> sinhala_typer.nsi

!ifndef VERSION
  !define VERSION "1.1.0"
!endif
!ifndef EXE
  !define EXE "..\target\x86_64-pc-windows-gnu\release\sinhala_typer.exe"
!endif
!ifndef ICO
  !define ICO "sinhala_typer.ico"
!endif

!include "MUI2.nsh"

Name "Sinhala Typer"
OutFile "sinhala_typer-setup-${VERSION}.exe"
InstallDir "$LOCALAPPDATA\Sinhala Typer"
InstallDirRegKey HKCU "Software\Sinhala Typer" "InstallDir"
RequestExecutionLevel user
SetCompressor /SOLID lzma

VIProductVersion "${VERSION}.0"
VIAddVersionKey "ProductName" "Sinhala Typer"
VIAddVersionKey "FileDescription" "Sinhala Typer Setup"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "CompanyName" "Sinhala Typer"

!define MUI_ICON "${ICO}"
!define MUI_UNICON "${ICO}"
!define MUI_ABORTWARNING

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Section "Sinhala Typer" SecMain
  SectionIn RO
  SetOutPath "$INSTDIR"
  File /oname=sinhala_typer.exe "${EXE}"
  WriteRegStr HKCU "Software\Sinhala Typer" "InstallDir" "$INSTDIR"
  WriteUninstaller "$INSTDIR\uninstall.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sinhala Typer" "DisplayName" "Sinhala Typer"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sinhala Typer" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sinhala Typer" "Publisher" "Sinhala Typer"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sinhala Typer" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sinhala Typer" "DisplayIcon" "$INSTDIR\sinhala_typer.exe"
SectionEnd

Section "Start Menu shortcut" SecStartMenu
  CreateDirectory "$SMPROGRAMS\Sinhala Typer"
  CreateShortcut "$SMPROGRAMS\Sinhala Typer\Sinhala Typer.lnk" "$INSTDIR\sinhala_typer.exe"
  CreateShortcut "$SMPROGRAMS\Sinhala Typer\Uninstall.lnk" "$INSTDIR\uninstall.exe"
SectionEnd

Section "Desktop shortcut" SecDesktop
  CreateShortcut "$DESKTOP\Sinhala Typer.lnk" "$INSTDIR\sinhala_typer.exe"
SectionEnd

Section "Start with Windows" SecAutostart
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "sinhala_typer" '"$INSTDIR\sinhala_typer.exe"'
SectionEnd

Section "Uninstall"
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "sinhala_typer"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Sinhala Typer"
  DeleteRegKey HKCU "Software\Sinhala Typer"
  Delete "$DESKTOP\Sinhala Typer.lnk"
  Delete "$SMPROGRAMS\Sinhala Typer\Sinhala Typer.lnk"
  Delete "$SMPROGRAMS\Sinhala Typer\Uninstall.lnk"
  RMDir "$SMPROGRAMS\Sinhala Typer"
  Delete "$INSTDIR\sinhala_typer.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
SectionEnd
