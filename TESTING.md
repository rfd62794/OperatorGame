# OperatorGame Android Testing Guide

## Quick Commands

```powershell
# Full pipeline: detect device, health check, build, deploy, launch, stream logs
.\run_android_tests.ps1

# Diagnostic only (no deploy): inspect device + APK contents
.\run_android_tests.ps1 -DiagnosticOnly

# Target a specific device by serial
.\run_android_tests.ps1 -Serial ZY2247XXXXX

# Single-command deploy
.\deploy_moto.ps1

# Inspect APK native libraries
.\diagnose_apk.ps1

# Audit NDK version alignment
.\check_ndk_config.ps1

# Stream filtered device logs
.\check_moto.ps1

# Diagnostic logcat (captures crash window)
.\check_moto.ps1 -Diagnostic
```

---

## Scripts Summary

| Script | Purpose |
|---|---|
| `run_android_tests.ps1` | Full test suite runner |
| `deploy_moto.ps1` | One-command deploy pipeline |
| `diagnose_apk.ps1` | Inspect APK native library contents |
| `check_ndk_config.ps1` | Verify NDK version alignment |
| `check_moto.ps1` | Stream device logcat |
| `setup_local_forge.ps1` | Validate + configure build environment |
| `build_android.ps1` | Build + sign APK |
| `lib/device_registry.ps1` | Module: device detection + health checks |
| `lib/logcat_monitor.ps1` | Module: logcat streaming + crash detection |

---

## Troubleshooting Crashes

### App crashes immediately (SIG: 9)

1. Check APK native library contents:
   ```powershell
   .\diagnose_apk.ps1
   ```
   Look for `MISSING: liboperator.so` or `CRITICAL: No .so files`.

2. Audit NDK:
   ```powershell
   .\check_ndk_config.ps1
   ```
   If NDK version mismatches `.cargo/config.toml`, update the linker path.

3. Rebuild cleanly:
   ```powershell
   cargo clean
   .\build_android.ps1
   .\diagnose_apk.ps1   # confirm .so files present before deploying
   ```

### Crash after startup (Rust panic / SIGSEGV)

```powershell
# Capture the crash window immediately after launch:
.\check_moto.ps1 -Diagnostic
```

Look for `linker`, `dlopen`, `SIGSEGV`, `SIGABRT`, or Rust `panicked at` lines.

### Device not detected

```powershell
.\run_android_tests.ps1 -DiagnosticOnly
```

Ensure USB Debugging is enabled: **Settings → Developer Options → USB Debugging**.  
Tap **Allow** on the USB authorisation dialog that appears on the phone.

---

## Build → Test Workflow (Per Phase)

```powershell
# 1. Set up environment (once per PowerShell session):
. .\setup_local_forge.ps1

# 2. Connect Moto G via USB

# 3. Deploy and test:
.\run_android_tests.ps1

# 4. If APK crashes — diagnose:
.\diagnose_apk.ps1
.\check_moto.ps1 -Diagnostic
```

---

## Library Modules

The `lib/` folder contains reusable PowerShell functions:

```powershell
# Device detection and health
. .\lib\device_registry.ps1
$devices = Get-ConnectedDevices
$health  = Test-DeviceHealth $devices[0].Serial

# Logcat control
. .\lib\logcat_monitor.ps1
Start-LogcatStream -Serial $devices[0].Serial
$crashDump = Get-CrashDump -Serial $devices[0].Serial
$crashes   = Detect-CrashInLines -Lines $crashDump
```
