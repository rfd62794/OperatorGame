# OperatorGame — Android Crash Diagnosis + Testing Suite Extension Directive

**Target:** Coding Agent (Gemini/Antigravity)  
**Goal:** (1) Diagnose and fix Phase F.0 crash on Moto G, (2) Build formalized Android device testing suite  
**Scope:** APK analysis, logcat diagnostics, extended testing infrastructure  
**Output:** Working Phase F.0 on device + reusable Android testing framework  

---

## Part A: Phase F.0 Crash Diagnosis & Fix

### Context

**Current State:**
- APK builds, signs, and installs successfully ✅
- App launches on Moto G (PID: 19167) ✅
- App crashes immediately with SIG: 9 (SIGKILL) ❌
- No Rust panic—the OS is killing the process before Rust code runs

**Root Cause Hypothesis:**
Missing `libc++_shared.so` in APK (or NDK version mismatch). The native library can't find its C++ runtime dependency, causing linker failure → SIGKILL.

**Evidence:**
- `setup_local_forge.ps1` reports NDK 29 (fallback)
- `.cargo/config.toml` hardcodes aarch64-linux-android33-clang (NDK API 33)
- Version mismatch may prevent `libc++_shared.so` from being bundled correctly

---

### Task A.1: APK Contents Analysis

**Create a new diagnostic script:** `diagnose_apk.ps1`

**Purpose:** Inspect the APK file to verify all required .so files are present.

**Requirements:**

```powershell
param(
    [string]$ApkPath = "operatorgame-release.apk"
)

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  OperatorGame APK Diagnostic Tool                         ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

# Verify APK exists
if (-not (Test-Path $ApkPath)) {
    Write-Error "APK not found: $ApkPath"
    exit 1
}

Write-Host "`n[1/3] Inspecting APK contents..." -ForegroundColor Cyan

# List all .so files in APK
Add-Type -AssemblyName System.IO.Compression.FileSystem
$apk = [IO.Compression.ZipFile]::OpenRead((Resolve-Path $ApkPath).Path)

$soFiles = $apk.Entries | Where-Object { $_.Name -like "*.so" }

if ($soFiles.Count -eq 0) {
    Write-Host "❌ NO .SO FILES FOUND IN APK" -ForegroundColor Red
    Write-Host "   This is the crash cause. The native library was not bundled."
    $apk.Dispose()
    exit 1
}

Write-Host "Found $($soFiles.Count) native libraries:" -ForegroundColor Green
$soFiles | ForEach-Object {
    $size = $_.Length / 1KB
    Write-Host "  ✓ $($_.FullName) ($([math]::Round($size, 1)) KB)"
}

# Check for specific critical libraries
$critical = @("libc++_shared.so", "liboperatorgame.so")
Write-Host "`n[2/3] Checking for critical dependencies..." -ForegroundColor Cyan

foreach ($lib in $critical) {
    $found = $soFiles | Where-Object { $_.Name -eq $lib }
    if ($found) {
        Write-Host "  ✓ $lib — PRESENT" -ForegroundColor Green
    } else {
        Write-Host "  ❌ $lib — MISSING (crash cause?)" -ForegroundColor Red
    }
}

# List all files in lib/ directory
Write-Host "`n[3/3] Full lib/ directory contents:" -ForegroundColor Cyan
$libEntries = $apk.Entries | Where-Object { $_.FullName -like "lib/*" }
$libEntries | ForEach-Object {
    Write-Host "  $($_.FullName)"
}

$apk.Dispose()

Write-Host "`n✅ APK diagnostic complete." -ForegroundColor Green
```

**Output:**
- Lists all .so files in APK
- Flags if `libc++_shared.so` or `liboperatorgame.so` is missing
- Shows full lib/ directory structure

---

### Task A.2: Enhanced Logcat Capture

**Modify `check_moto.ps1`** to capture more context around crashes.

**Add a new mode:** `check_moto.ps1 -Diagnostic`

```powershell
param(
    [switch]$Diagnostic = $false
)

$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

if (-not (Test-Path $ADB)) {
    Write-Error "ADB not found. Run setup_local_forge.ps1 first."
    exit 1
}

if ($Diagnostic) {
    Write-Host "Capturing diagnostic logcat..." -ForegroundColor Cyan
    
    # Clear buffer
    & $ADB logcat -c
    
    # Capture everything for 10 seconds, then filter
    Write-Host "Listening for 10 seconds..." -ForegroundColor Yellow
    Start-Sleep -Seconds 10
    
    # Dump buffer and filter for key messages
    & $ADB logcat -d | Select-String -Pattern "opera|linker|FATAL|dlopen|library|signal|error|exception" -CaseInsensitive | Select-Object -Last 50
    
    Write-Host "`nDone. Look for:" -ForegroundColor Cyan
    Write-Host "  - linker errors (undefined reference, cannot locate)"
    Write-Host "  - dlopen failures (cannot open .so)"
    Write-Host "  - signal messages (SIGKILL, SIGSEGV)"
} else {
    # Normal logcat stream (existing behavior)
    & $ADB logcat -c
    & $ADB logcat --pid=$(& $ADB shell pidof com.rfditservices.operatorgame)
}
```

**Usage:**
```powershell
# Normal stream:
.\check_moto.ps1

# Diagnostic mode (captures full logcat, looks for linker/crash messages):
.\check_moto.ps1 -Diagnostic
```

---

### Task A.3: NDK Version Investigation

**Create diagnostic summary:** `check_ndk_config.ps1`

```powershell
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  NDK Configuration Audit                                   ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

Write-Host "`n[1/3] Reading .cargo/config.toml..." -ForegroundColor Cyan
$cargoConfig = Get-Content ".cargo/config.toml" | Select-String "aarch64|android" -CaseInsensitive
Write-Host $cargoConfig

Write-Host "`n[2/3] Installed NDK versions:" -ForegroundColor Cyan
$sdkPath = "$env:LOCALAPPDATA\Android\Sdk\ndk"
if (Test-Path $sdkPath) {
    Get-ChildItem $sdkPath | ForEach-Object {
        $version = $_.Name
        Write-Host "  - NDK $version"
        if (Test-Path "$($_.FullName)\toolchains\llvm\prebuilt\windows\lib\libc++_shared.so") {
            Write-Host "    ✓ libc++_shared.so found at: toolchains/llvm/prebuilt/windows/lib/" -ForegroundColor Green
        } else {
            Write-Host "    ⚠️  libc++_shared.so not found at expected location" -ForegroundColor Yellow
        }
    }
} else {
    Write-Host "  ❌ NDK directory not found at $sdkPath" -ForegroundColor Red
}

Write-Host "`n[3/3] Cargo.toml [package.metadata.android] config:" -ForegroundColor Cyan
Select-String -Path "Cargo.toml" -Pattern "\[package.metadata.android\]" -Context 20 | Select-Object -First 30
```

---

### Task A.4: Fix Strategy

**Based on diagnostics, implement one of:**

**Option 1: Ensure NDK 33 is installed**
```powershell
# If setup_local_forge.ps1 is using NDK 29 as fallback, explicitly install NDK 33:
# (Add to setup script or run manually)
$ANDROID_SDK_ROOT = "$env:LOCALAPPDATA\Android\Sdk"
sdkmanager "ndk;25.2.9519653"  # Or the exact version .cargo/config.toml expects
```

**Option 2: Update .cargo/config.toml to use installed NDK**
```toml
# Replace hardcoded aarch64-linux-android33-clang with the actual installed NDK version
[target.aarch64-linux-android]
linker = "C:/Users/cheat/AppData/Local/Android/Sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/windows/bin/aarch64-linux-android33-clang"
```

**Option 3: Update Cargo.toml to NOT use C++ shared library**
```toml
[package.metadata.android]
# Change from:
# stl = "c++_shared"
# To:
stl = "c++_static"  # Static linking eliminates dependency on libc++_shared.so
```

**Recommendation:** Start with Option 3 (static linking) as it's the simplest and most reliable. Then verify the APK builds and runs.

---

### Task A.5: Rebuild & Test

Once fix is applied:

```powershell
# Clean previous build
cargo clean

# Rebuild with new config
.\build_android.ps1

# Verify APK contents
.\diagnose_apk.ps1

# Redeploy
.\deploy_moto.ps1
```

**Success criteria:**
- `diagnose_apk.ps1` shows all required .so files present
- App launches without SIG: 9
- Logcat shows normal app output (no linker errors)

---

## Part B: Extended Android Testing Suite

### Goal

Build a formalized, reusable Android device testing framework that handles:
1. Device detection & enumeration
2. Device health checking
3. App output capture (logcat with filtering)
4. Crash detection & reporting
5. Multiple device support (future)

---

### Task B.1: Device Registry Module

**Create:** `lib/device_registry.ps1`

```powershell
# device_registry.ps1 — Device detection and health checking

function Get-ConnectedDevices {
    param(
        [string]$AdbPath = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
    )
    
    if (-not (Test-Path $AdbPath)) {
        throw "ADB not found at $AdbPath"
    }
    
    # Start daemon silently
    & $AdbPath start-server 2>&1 | Out-Null
    
    # Get device list
    $rawDevices = & $AdbPath devices | Select-Object -Skip 1 | Where-Object { $_.Trim() }
    
    $devices = @()
    foreach ($line in $rawDevices) {
        $parts = $line -split '\s+'
        if ($parts.Count -ge 2) {
            $devices += [PSCustomObject]@{
                Serial = $parts[0]
                State  = $parts[1]
                Model  = (& $AdbPath -s $parts[0] shell getprop ro.product.model 2>$null).Trim()
            }
        }
    }
    
    return $devices
}

function Test-DeviceHealth {
    param(
        [string]$Serial,
        [string]$AdbPath = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
    )
    
    $health = [PSCustomObject]@{
        Serial       = $Serial
        Connected    = $false
        APILevel     = $null
        DebugEnabled = $false
        StorageFree  = $null
        Issues       = @()
    }
    
    try {
        # Check connection
        $state = & $AdbPath -s $Serial get-state
        $health.Connected = ($state -eq "device")
        
        if (-not $health.Connected) {
            $health.Issues += "Device not in online state: $state"
            return $health
        }
        
        # Get API level
        $health.APILevel = (& $AdbPath -s $Serial shell getprop ro.build.version.sdk).Trim()
        
        # Check debug enabled
        $debuggable = (& $AdbPath -s $Serial shell getprop ro.debuggable).Trim()
        $health.DebugEnabled = ($debuggable -eq "1")
        
        if (-not $health.DebugEnabled) {
            $health.Issues += "USB debugging not enabled"
        }
        
        # Check storage
        $storageKB = (& $AdbPath -s $Serial shell "df /data | tail -1" | awk '{print $4}').Trim()
        if ($storageKB -gt 0) {
            $health.StorageFree = [math]::Round($storageKB / 1024 / 1024, 1)  # Convert to GB
        }
        
        if ($health.StorageFree -lt 0.5) {
            $health.Issues += "Low storage: $($health.StorageFree) GB free"
        }
        
    } catch {
        $health.Issues += "Health check failed: $_"
    }
    
    return $health
}

export-modulemember -function Get-ConnectedDevices, Test-DeviceHealth
```

**Usage:**
```powershell
. .\lib\device_registry.ps1

$devices = Get-ConnectedDevices
$devices | ForEach-Object {
    Write-Host "Device: $($_.Serial) — $($_.Model) ($_State)"
    
    $health = Test-DeviceHealth $_.Serial
    if ($health.Issues.Count -gt 0) {
        Write-Host "  Issues: $($health.Issues -join ', ')"
    }
}
```

---

### Task B.2: Logcat Filtering & Crash Detection Module

**Create:** `lib/logcat_monitor.ps1`

```powershell
# logcat_monitor.ps1 — Logcat capture, filtering, and crash detection

function Start-LogcatCapture {
    param(
        [string]$Serial,
        [string]$PackageName = "com.rfditservices.operatorgame",
        [string]$AdbPath = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe",
        [int]$MaxLines = 200
    )
    
    Write-Host "Starting logcat capture for $PackageName..." -ForegroundColor Cyan
    
    # Clear buffer
    & $AdbPath -s $Serial logcat -c
    
    # Get PID of app
    $pid = (& $AdbPath -s $Serial shell pidof $PackageName 2>$null).Trim()
    
    if (-not $pid) {
        Write-Host "App not running. Starting..." -ForegroundColor Yellow
        return $null
    }
    
    # Stream logcat filtered to package PID
    & $AdbPath -s $Serial logcat --pid=$pid | Tee-Object -Variable logOutput | Select-Object -Last $MaxLines
    
    return $logOutput
}

function Detect-Crash {
    param(
        [string[]]$LogLines
    )
    
    $crashPatterns = @(
        "FATAL",
        "SIGKILL",
        "SIGSEGV",
        "SIGABRT",
        "java.lang.RuntimeException",
        "AndroidRuntime.*FATAL",
        "linker.*error",
        "dlopen.*failed"
    )
    
    $crashes = @()
    foreach ($line in $LogLines) {
        foreach ($pattern in $crashPatterns) {
            if ($line -match $pattern) {
                $crashes += $line
            }
        }
    }
    
    return $crashes
}

export-modulemember -function Start-LogcatCapture, Detect-Crash
```

---

### Task B.3: Testing Framework

**Create:** `run_android_tests.ps1`

```powershell
param(
    [string]$Serial = $null,
    [switch]$Verbose = $false,
    [switch]$DiagnosticOnly = $false
)

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  OperatorGame Android Testing Suite                       ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

# Load modules
. .\lib\device_registry.ps1
. .\lib\logcat_monitor.ps1

# Detect devices
Write-Host "`n[1/4] Device Detection..." -ForegroundColor Cyan
$devices = Get-ConnectedDevices

if ($devices.Count -eq 0) {
    Write-Host "❌ No devices connected" -ForegroundColor Red
    exit 1
}

$targetDevice = if ($Serial) {
    $devices | Where-Object { $_.Serial -eq $Serial }
} else {
    $devices[0]
}

if (-not $targetDevice) {
    Write-Host "❌ Device not found: $Serial" -ForegroundColor Red
    exit 1
}

Write-Host "  ✓ Using device: $($targetDevice.Serial) — $($targetDevice.Model)" -ForegroundColor Green

# Health check
Write-Host "`n[2/4] Device Health Check..." -ForegroundColor Cyan
$health = Test-DeviceHealth $targetDevice.Serial

if ($health.Issues.Count -gt 0) {
    Write-Host "  ⚠️  Issues found:" -ForegroundColor Yellow
    $health.Issues | ForEach-Object { Write-Host "    - $_" }
} else {
    Write-Host "  ✓ Device healthy" -ForegroundColor Green
    Write-Host "    API Level: $($health.APILevel)"
    Write-Host "    Storage Free: $($health.StorageFree) GB"
}

if ($DiagnosticOnly) {
    Write-Host "`n✅ Diagnostic complete. Exiting." -ForegroundColor Green
    exit 0
}

# Deploy APK
Write-Host "`n[3/4] Deploying APK..." -ForegroundColor Cyan
.\deploy_moto.ps1
```

---

### Task B.4: Documentation

**Create:** `TESTING.md`

```markdown
# OperatorGame Android Testing Suite

## Quick Start

### Run Full Test Suite
```powershell
.\run_android_tests.ps1
```

### Diagnostic Only (no deploy)
```powershell
.\run_android_tests.ps1 -DiagnosticOnly
```

### Analyze APK
```powershell
.\diagnose_apk.ps1
```

### Enhanced Logcat
```powershell
# Stream logs
.\check_moto.ps1

# Diagnostic mode (captures crash info)
.\check_moto.ps1 -Diagnostic
```

## Troubleshooting

### App Crashes on Launch
1. Run `.\diagnose_apk.ps1` to check if all .so files are present
2. Run `.\check_ndk_config.ps1` to verify NDK version alignment
3. Run `.\check_moto.ps1 -Diagnostic` to capture crash context

### Device Not Detected
1. Run `.\run_android_tests.ps1 -DiagnosticOnly` to see all connected devices
2. Ensure USB debugging is enabled
3. Check `adb devices` manually

### Low Storage
Device may need space for APK installation. Clear app cache or uninstall unused apps.
```

---

## Acceptance Criteria

**Part A (Crash Diagnosis & Fix):**
✓ APK diagnostic script created and identifies missing .so files  
✓ Enhanced logcat capture with diagnostic mode  
✓ NDK version audit script  
✓ Root cause identified (NDK version mismatch or missing C++ library)  
✓ Fix applied (static linking, NDK version alignment, or other)  
✓ Phase F.0 app launches on Moto G without crash  
✓ All 149 Rust tests still pass  

**Part B (Testing Suite Extension):**
✓ Device registry module (device detection, health checking)  
✓ Logcat monitor module (filtering, crash detection)  
✓ Integrated test runner script  
✓ TESTING.md documentation  
✓ All scripts parse cleanly  
✓ Reusable for future phases (F.1, F.2, F.3, F.4)  

---

## Implementation Order

1. **First:** Create diagnostic scripts (diagnose_apk.ps1, check_ndk_config.ps1)
2. **Second:** Run diagnostics to identify crash cause
3. **Third:** Implement fix based on findings
4. **Fourth:** Verify fix works (deploy, test on Moto G)
5. **Fifth:** Build testing suite modules (device_registry.ps1, logcat_monitor.ps1)
6. **Sixth:** Create run_android_tests.ps1 and TESTING.md

---

## Success Looks Like

After completion:

```powershell
# One command: diagnose + deploy + test
.\run_android_tests.ps1

# Output:
# ✓ Device detected
# ✓ Device healthy
# ✓ APK deployed
# ✓ App launched (no crash)
# ✓ Logcat streaming
```

And Phase F.0 UI is visible on Moto G with no errors.

---

## Notes for Agent

- Diagnostic scripts should be standalone (no dependencies beyond PowerShell + ADB)
- Testing suite should be extensible (easy to add new device types, filtering rules)
- All scripts should follow the same error handling pattern (exit 1 on failure)
- Documentation should target developers (not end users)

---
