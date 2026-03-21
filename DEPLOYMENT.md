# Android Deployment & Automation

Deploy OperatorGame to physical Android hardware using PowerShell automation.

## Quick Start

```powershell
.\deploy_moto.ps1             # install + launch
.\check_moto.ps1              # health check + diagnostics
.\capture_screenshots.ps1     # automated UI capture
.\find_ui_coordinates.ps1     # interactive calibration
```

## Prerequisites
- **Windows PowerShell 5.1+**
- **Android SDK path**: `$env:LOCALAPPDATA\Android\Sdk`
- **Tools**: ADB, apksigner, aapt available in PATH
- **NDK**: 25.2.9519653+ pinned version note
- **Hardware**: Moto G 2025 (API 35) tested baseline
- **Device**: USB debugging requirement enabled

## Module Architecture
The `OperatorDeviceTools.psm1` unifying module governs physical device testing.
- **Device Management**: `Connect-Device`, `Is-AppRunning`, `Stop-OperatorApp` (Object-oriented health tracking)
- **App Lifecycle**: `Install-OperatorApp`, `Launch-OperatorApp`, `Detect-AppCrash` (Sideloading and processes)
- **Screen I/O**: `Capture-Screenshot`, `Invoke-DeviceTap`, `Invoke-DeviceInput` (Binary-safe pull and inputs)
- **Diagnostics**: `Get-DeviceLogcat`, `Audit-NdkConfig`, `Diagnose-ApkIssues` (Offline buffer analysis)

## Common Workflows
- **Workflow 1 Deploy & Test**: `deploy_moto.ps1` → `check_moto.ps1`
- **Workflow 2 Screenshot Sweep**: `capture_screenshots.ps1`
- **Workflow 3 Crash Diagnostics**: `Get-DeviceLogcat` + `Detect-AppCrash`
- **Workflow 4 APK Health Check**: `Diagnose-ApkIssues`

## Troubleshooting
- **"ADB not found"** → check PATH
- **"Device offline"** → check USB debugging, restart adb daemon
- **"APK not discovered"** → check repo root, target/ directory
- **"SIGKILL on launch"** → run `Audit-NdkConfig` + `Diagnose-ApkIssues`
- **"Screenshot pull fails"** → verify device storage space

## Reference
- Core Governance: `CONSTITUTION.md`
- Game Rules: `SPEC.md`
- SDD Phase Sprints: `/docs/sprints/`
