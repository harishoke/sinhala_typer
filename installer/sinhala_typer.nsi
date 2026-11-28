; Sinhala Typer installer (NSIS). Used by the GitHub Actions pipeline
; (.github/workflows/build.yml) and for local builds.
; Build: makensis /DVERSION=3.0.0 /DEXE=<path to sinhala_typer.exe> /DTSF=<path to 64-bit sinhala_typer_tsf.dll> /DTSFX86=<path to 32-bit sinhala_typer_tsf.dll> /DICO=<path to sinhala_typer.ico> sinhala_typer.nsi

!ifndef VERSION
  !define VERSION "3.0.0"
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
; Admin: registering the TSF text service writes to HKLM.
RequestExecutionLevel admin
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
!ifdef TSF
  ; TSF text service for the Windows keyboard integration: the 64-bit DLL
  ; next to the exe, the 32-bit DLL in x86\ (32-bit apps cannot load a
  ; 64-bit IME DLL). Both are registered below; Settings can unregister
  ; and re-register them later.
  File /oname=sinhala_typer_tsf.dll "${TSF}"
!endif
!ifdef TSFX86
  SetOutPath "$INSTDIR\x86"
  File /oname=sinhala_typer_tsf.dll "${TSFX86}"
  SetOutPath "$INSTDIR"
!endif
!ifdef TSF
  ExecWait '"$SYSDIR\regsvr32.exe" /s "$INSTDIR\sinhala_typer_tsf.dll"'
!endif
!ifdef TSFX86
  ExecWait '"$WINDIR\SysWOW64\regsvr32.exe" /s "$INSTDIR\x86\sinhala_typer_tsf.dll"'
!endif
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
  ; Unregister the TSF service (both bitnesses) before removing the DLLs.
  ExecWait '"$SYSDIR\regsvr32.exe" /u /s "$INSTDIR\sinhala_typer_tsf.dll"'
  ExecWait '"$WINDIR\SysWOW64\regsvr32.exe" /u /s "$INSTDIR\x86\sinhala_typer_tsf.dll"'
  Delete "$INSTDIR\x86\sinhala_typer_tsf.dll"
  RMDir "$INSTDIR\x86"
  Delete "$INSTDIR\sinhala_typer_tsf.dll"
  Delete "$INSTDIR\sinhala_typer.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
SectionEnd
