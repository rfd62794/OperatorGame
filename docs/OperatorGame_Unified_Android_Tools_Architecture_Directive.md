# OperatorGame — Unified Android Tools Subsystem Architecture Directive

**Phase:** SDD (System Design) — Foundation & Architectural Mapping  
**Goal:** Design and implement a cohesive, modular Android automation framework that serves ALL scripts  
**Scope:** Module architecture, function contracts, error handling, extensibility  
**Output:** Reusable PowerShell module ecosystem for Android testing, deployment, diagnostics  

---

## Executive Summary

**Current State:**
- 8+ PowerShell scripts (deploy_moto.ps1, check_moto.ps1, build_android.ps1, capture_screenshots.ps1, find_ui_coordinates.ps1, check_ndk_config.ps1, diagnose_apk.ps1, run_android_tests.ps1)
- Heavy duplication: device detection, ADB initialization, error handling, path resolution
- No shared abstractions: each script re-implements connection logic, logcat parsing, input simulation
- Difficult to maintain: changes to ADB paths, error strategies require edits in multiple places
- Not extensible: new automation needs re-implement the wheel

**Problem:**
We've hit the complexity ceiling on monolithic scripts. The agent correctly identified this. But the solution isn't a single module—it's a **modular subsystem** that all scripts import and orchestrate.

**Solution:**
Build a PowerShell Module Library (`lib/OperatorDeviceTools/`) that centralizes:
- Device detection, connection, lifecycle management
- ADB interaction (install, launch, capture, tap, logcat)
- Build artifact handling (APK discovery, signing, verification)
- Error handling & auto-healing (missing APK → install, daemon restart, etc.)
- Diagnostics & health checking
- Logging & crash detection

Then refactor **all existing scripts** to import this module and focus on **orchestration**, not implementation.

---

## Part A: Module Architecture

### Directory Structure

```
C:\Github\OperatorGame\
├── lib/
│   └── OperatorDeviceTools/
│       ├── OperatorDeviceTools.psm1       (Main module manifest)
│       ├── Classes/
│       │   ├── Device.ps1                 (Device object, properties, methods)
│       │   ├── AdbSession.ps1             (ADB connection state)
│       │   └── Screenshot.ps1             (Screenshot metadata)
│       ├── Public/
│       │   ├── Device/
│       │   │   ├── Connect-Device.ps1
│       │   │   ├── Disconnect-Device.ps1
│       │   │   ├── Get-ConnectedDevices.ps1
│       │   │   ├── Test-DeviceHealth.ps1
│       │   │   └── Invoke-DeviceCommand.ps1
│       │   ├── App/
│       │   │   ├── Install-OperatorApp.ps1
│       │   │   ├── Launch-OperatorApp.ps1
│       │   │   ├── Is-AppRunning.ps1
│       │   │   └── Stop-OperatorApp.ps1
│       │   ├── Screen/
│       │   │   ├── Capture-Screenshot.ps1
│       │   │   ├── Invoke-DeviceTap.ps1
│       │   │   └── Invoke-DeviceInput.ps1
│       │   ├── Logcat/
│       │   │   ├── Get-DeviceLogcat.ps1
│       │   │   ├── Start-LogcatStream.ps1
│       │   │   ├── Detect-AppCrash.ps1
│       │   │   └── Parse-LogcatBuffer.ps1
│       │   ├── Build/
│       │   │   ├── Find-BuildArtifact.ps1
│       │   │   ├── Verify-ApkSignature.ps1
│       │   │   └── Get-ApkContents.ps1
│       │   └── Diagnostics/
│       │       ├── Test-AdbConnection.ps1
│       │       ├── Get-DeviceInfo.ps1
│       │       ├── Audit-NdkConfig.ps1
│       │       └── Diagnose-ApkIssues.ps1
│       └── Private/
│           ├── Initialize-AdbEnvironment.ps1
│           ├── Start-AdbDaemon.ps1
│           ├── Resolve-AdbPath.ps1
│           ├── Resolve-SdkPath.ps1
│           ├── Invoke-AdbCommand.ps1
│           ├── Wait-For-AppStartup.ps1
│           └── Convert-LogcatTimestamp.ps1
└── [existing scripts refactored to use module]
    ├── deploy_moto.ps1
    ├── check_moto.ps1
    ├── capture_screenshots.ps1
    ├── find_ui_coordinates.ps1
    ├── run_android_tests.ps1
    ├── check_ndk_config.ps1
    ├── diagnose_apk.ps1
    └── build_android.ps1
```

---

## Part B: Core Module Design

### Module Manifest: `OperatorDeviceTools.psm1`

```powershell
# OperatorDeviceTools.psm1 — Unified Android Tools Subsystem

#region Module Configuration
$ModulePath = Split-Path -Parent $MyInvocation.MyCommand.Path
$PublicFunctions = Get-ChildItem -Path "$ModulePath\Public\*\*.ps1" -Recurse
$PrivateFunctions = Get-ChildItem -Path "$ModulePath\Private\*.ps1" -Recurse
$Classes = Get-ChildItem -Path "$ModulePath\Classes\*.ps1" -Recurse

# Load classes first
foreach ($Class in $Classes) {
    . $Class.FullName
}

# Load private functions (not exported)
foreach ($Function in $PrivateFunctions) {
    . $Function.FullName
}

# Load public functions (exported)
foreach ($Function in $PublicFunctions) {
    . $Function.FullName
}

# Define module properties
$Script:OperatorDeviceTools = @{
    AdbPath         = $null
    SdkPath         = $null
    RepositoryRoot  = (Get-Item (Split-Path -Parent $ModulePath)).Parent.FullName
    LogLevel        = "Info"  # Debug, Info, Warn, Error
    DefaultSerial   = $null
    SessionTimeout  = 300     # seconds
}

# Export public functions
$PublicFunctionNames = $PublicFunctions | ForEach-Object { [System.IO.Path]::GetFileNameWithoutExtension($_.Name) }
Export-ModuleMember -Function $PublicFunctionNames

#endregion
```

---

### Classes: Device Object Model

**`Classes/Device.ps1`:**
```powershell
class Device {
    [string]$Serial
    [string]$State          # online, offline, recovery, etc.
    [string]$Model
    [int]$ApiLevel
    [bool]$DebugEnabled
    [double]$StorageFreeGb
    [string]$AndroidVersion
    [System.Diagnostics.Process]$AdbSession
    
    [void] Refresh() {
        # Reload device info from device
        $this.State = Invoke-AdbCommand -Serial $this.Serial -Command "get-state"
        $this.ApiLevel = [int](Invoke-AdbCommand -Serial $this.Serial -Command "shell getprop ro.build.version.sdk")
        $this.AndroidVersion = Invoke-AdbCommand -Serial $this.Serial -Command "shell getprop ro.build.version.release"
    }
    
    [bool] IsHealthy() {
        return $this.State -eq "device" -and $this.DebugEnabled -and $this.StorageFreeGb -gt 0.5
    }
    
    [string] ToString() {
        return "$($this.Serial) — $($this.Model) (API $($this.ApiLevel), $($this.StorageFreeGb) GB free)"
    }
}
```

---

### Public API: Function Contracts

#### Device Management Functions

**`Public/Device/Connect-Device.ps1`:**
```powershell
function Connect-Device {
    param(
        [string]$Serial = $null,
        [switch]$AutoLaunch = $true,
        [int]$TimeoutSeconds = 30
    )
    
    <#
    .SYNOPSIS
    Establish and validate connection to Android device.
    
    .DESCRIPTION
    - Auto-detects device if $Serial is not specified
    - Verifies device is online and responsive
    - Optionally launches app if $AutoLaunch is set
    - Returns Device object or throws on failure
    
    .OUTPUTS
    [Device] Connected device with validated state
    #>
    
    # Implementation...
}
```

**`Public/Device/Get-ConnectedDevices.ps1`:**
```powershell
function Get-ConnectedDevices {
    param([switch]$OnlineOnly = $true)
    
    <#
    .SYNOPSIS
    List all connected Android devices.
    
    .OUTPUTS
    [Device[]] Array of devices with populated properties
    #>
    
    # Implementation...
}
```

**`Public/Device/Test-DeviceHealth.ps1`:**
```powershell
function Test-DeviceHealth {
    param([Device]$Device)
    
    <#
    .SYNOPSIS
    Run comprehensive health check on device.
    
    .OUTPUTS
    [hashtable] with keys: IsHealthy, Issues[], Warnings[]
    #>
    
    # Implementation...
}
```

---

#### App Lifecycle Functions

**`Public/App/Install-OperatorApp.ps1`:**
```powershell
function Install-OperatorApp {
    param(
        [Device]$Device,
        [string]$ApkPath = $null,
        [switch]$Force = $false
    )
    
    <#
    .SYNOPSIS
    Install OperatorGame APK on device (with auto-healing).
    
    .DESCRIPTION
    - If $ApkPath not specified, searches repo for operatorgame-release.apk
    - Validates APK signature
    - Checks if app already installed (skips if not $Force)
    - If missing, auto-downloads or extracts from build artifacts
    - Installs via 'adb install -r'
    
    .OUTPUTS
    [bool] $true if install successful, throws on failure
    #>
    
    # Implementation...
}
```

**`Public/App/Launch-OperatorApp.ps1`:**
```powershell
function Launch-OperatorApp {
    param(
        [Device]$Device,
        [int]$WaitSeconds = 5,
        [switch]$KillIfRunning = $false
    )
    
    <#
    .SYNOPSIS
    Launch OperatorGame on device with async startup verification.
    
    .OUTPUTS
    [int] Process ID (PID) or throws on failure
    #>
    
    # Implementation...
}
```

**`Public/App/Is-AppRunning.ps1`:**
```powershell
function Is-AppRunning {
    param([Device]$Device)
    
    <#
    .SYNOPSIS
    Check if OperatorGame is currently running.
    
    .OUTPUTS
    [int] PID if running, $null if not
    #>
    
    # Implementation...
}
```

---

#### Screen Interaction Functions

**`Public/Screen/Capture-Screenshot.ps1`:**
```powershell
function Capture-Screenshot {
    param(
        [Device]$Device,
        [string]$OutputPath,
        [string]$Label = $null
    )
    
    <#
    .SYNOPSIS
    Capture screen from device and save to disk.
    
    .OUTPUTS
    [Screenshot] object with metadata (path, timestamp, size, hash)
    #>
    
    # Implementation...
}
```

**`Public/Screen/Invoke-DeviceTap.ps1`:**
```powershell
function Invoke-DeviceTap {
    param(
        [Device]$Device,
        [int]$X,
        [int]$Y,
        [int]$DelayMs = 500
    )
    
    <#
    .SYNOPSIS
    Simulate touch input on device screen.
    
    .OUTPUTS
    $null, throws on failure
    #>
    
    # Implementation...
}
```

---

#### Logcat Functions

**`Public/Logcat/Get-DeviceLogcat.ps1`:**
```powershell
function Get-DeviceLogcat {
    param(
        [Device]$Device,
        [ValidateSet("Stream", "Buffer", "Diagnostic")]
        [string]$Mode = "Buffer",
        [string]$FilterPackage = "com.rfditservices.operatorgame",
        [int]$Lines = 200
    )
    
    <#
    .SYNOPSIS
    Retrieve logcat from device with flexible filtering.
    
    .DESCRIPTION
    - Mode "Stream": Live stream to terminal (Ctrl+C to stop)
    - Mode "Buffer": Dump full buffer to string array
    - Mode "Diagnostic": Capture crash context around recent errors
    
    .OUTPUTS
    [string[]] logcat lines
    #>
    
    # Implementation...
}
```

**`Public/Logcat/Detect-AppCrash.ps1`:**
```powershell
function Detect-AppCrash {
    param(
        [string[]]$LogLines,
        [ValidateSet("Strict", "Warn")]
        [string]$Sensitivity = "Strict"
    )
    
    <#
    .SYNOPSIS
    Analyze logcat for crash patterns (SIGKILL, SIGSEGV, FATAL, etc.).
    
    .OUTPUTS
    [hashtable] with Crashed=$bool, Type=$string, Details=$string[]
    #>
    
    # Implementation...
}
```

---

#### Build & Diagnostics Functions

**`Public/Build/Find-BuildArtifact.ps1`:**
```powershell
function Find-BuildArtifact {
    param(
        [ValidateSet("ApkRelease", "ApkDebug", "Aab", "Jks")]
        [string]$ArtifactType,
        [string]$SearchPath = $null
    )
    
    <#
    .SYNOPSIS
    Discover build artifacts in repo or build directory.
    
    .OUTPUTS
    [string] full path to artifact, or $null if not found
    #>
    
    # Implementation...
}
```

**`Public/Diagnostics/Diagnose-ApkIssues.ps1`:**
```powershell
function Diagnose-ApkIssues {
    param([string]$ApkPath)
    
    <#
    .SYNOPSIS
    Comprehensive APK analysis (contents, signatures, dependencies).
    
    .OUTPUTS
    [hashtable] with Issues[], Warnings[], SoFiles[]
    #>
    
    # Implementation...
}
```

**`Public/Diagnostics/Audit-NdkConfig.ps1`:**
```powershell
function Audit-NdkConfig {
    <#
    .SYNOPSIS
    Verify NDK installation and configuration consistency.
    
    .OUTPUTS
    [hashtable] with ConfiguredVersion, InstalledVersions[], Mismatches[]
    #>
    
    # Implementation...
}
```

---

#### Private Helper Functions

**`Private/Invoke-AdbCommand.ps1`:**
```powershell
function Invoke-AdbCommand {
    param(
        [string]$Serial,
        [string]$Command,
        [switch]$NoErrorCheck = $false,
        [int]$TimeoutSeconds = 30
    )
    
    <#
    .SYNOPSIS
    Execute ADB command with error handling and timeout.
    
    .DESCRIPTION
    - Ensures ADB daemon is running
    - Applies timeout to long-running commands
    - Parses stderr vs stdout intelligently
    - Throws on failure unless $NoErrorCheck
    #>
}
```

**`Private/Start-AdbDaemon.ps1`:**
```powershell
function Start-AdbDaemon {
    <#
    .SYNOPSIS
    Start ADB daemon (idempotent, safe if already running).
    #>
}
```

**`Private/Wait-For-AppStartup.ps1`:**
```powershell
function Wait-For-AppStartup {
    param(
        [Device]$Device,
        [int]$TimeoutSeconds = 30,
        [int]$PollIntervalMs = 500
    )
    
    <#
    .SYNOPSIS
    Poll for app process appearance (blocks until app launches or timeout).
    
    .OUTPUTS
    [int] PID
    #>
}
```

---

## Part C: Refactored Scripts (Examples)

### Before (Monolithic)
```powershell
# capture_screenshots.ps1 (before): 200+ lines, duplicated boilerplate
$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
& $ADB start-server 2>&1 | Out-Null
$devices = & $ADB devices | Select-Object -Skip 1 | Where-Object { $_.Trim() }
# ... 20 more lines of setup ...
```

### After (Clean Orchestration)
```powershell
# capture_screenshots.ps1 (after): 80 lines, pure orchestration
Import-Module "$PSScriptRoot\lib\OperatorDeviceTools" -Force

$Device = Connect-Device -AutoLaunch $true
$Screenshots = @()

foreach ($Tab in @("Roster", "Missions", "Map", "Logs")) {
    Invoke-DeviceTap -Device $Device -X $TabPositions[$Tab].X -Y $TabPositions[$Tab].Y
    Start-Sleep -Seconds 1
    
    $Screenshot = Capture-Screenshot -Device $Device -OutputPath "screenshots\$Tab.png" -Label $Tab
    $Screenshots += $Screenshot
}

Write-Host "✓ Captured $($Screenshots.Count) screenshots" -ForegroundColor Green
```

---

## Part D: Benefits & Success Metrics

### Benefits

✅ **DRY Principle:** No more copy-paste initialization logic  
✅ **Extensibility:** New scripts simply import module, focus on orchestration  
✅ **Maintainability:** Changes to ADB interaction happen in one place  
✅ **Error Handling:** Centralized, consistent error strategy (including auto-healing)  
✅ **Testability:** Module functions can be unit-tested in isolation  
✅ **Documentation:** Each function has inline help (Get-Help works)  
✅ **Reusability:** Same module serves build, test, diagnostics, UI automation  

### Success Metrics

- All 8+ scripts reduced by 40-60% lines of code
- Zero duplication of device detection, ADB initialization, error handling
- New scripts can be written in <50 lines (pure orchestration)
- All scripts consistent in error reporting, logging, exit codes
- Module documentation complete (Get-Help for each function)
- All scripts still pass their original test scenarios

---

## Part E: Implementation Plan

### Phase 1: Module Foundation (Parallel)
1. Create module directory structure and manifest
2. Implement `Classes/Device.ps1` (object model)
3. Implement private helper functions (ADB, daemon, timeout)

### Phase 2: Core Public API (Sequential)
1. Implement Device functions (Connect, Get, Test)
2. Implement App functions (Install, Launch, Is-Running)
3. Implement Screen functions (Capture, Tap)
4. Implement Logcat functions (Get, Detect-Crash)

### Phase 3: Build & Diagnostics (Sequential)
1. Implement Build functions (Find-Artifact, Verify)
2. Implement Diagnostics functions (Diagnose-Apk, Audit-Ndk)

### Phase 4: Refactor Existing Scripts (Sequential)
1. Refactor `deploy_moto.ps1`
2. Refactor `check_moto.ps1`
3. Refactor `capture_screenshots.ps1`
4. Refactor `find_ui_coordinates.ps1`
5. Refactor `run_android_tests.ps1`
6. Refactor build/check scripts

### Phase 5: Documentation & Testing
1. Write comprehensive help (Get-Help)
2. Create usage examples
3. Test all refactored scripts

---

## Acceptance Criteria

✓ Module structure created as specified  
✓ All core functions implemented with contracts  
✓ All existing scripts refactored to import module  
✓ All scripts produce identical functionality to originals  
✓ No duplication of initialization/error handling logic  
✓ All 149 Rust tests still pass  
✓ Module exports proper help documentation (Get-Help works)  
✓ New scripts can be written in <50 lines  

---

## Timeline Estimate

- **Phase 1-2 (Foundation + Core API):** 4-6 hours agent work
- **Phase 3 (Build & Diagnostics):** 2-3 hours
- **Phase 4 (Refactor Existing):** 3-4 hours
- **Phase 5 (Documentation & Testing):** 2-3 hours
- **Total:** ~12-16 hours of focused agent work

---

## Notes for Agent

- This is a **systematic refactor**, not an incremental patch
- The module should be **production-grade** — comprehensive error handling, timeouts, logging
- Each function should be **independently testable**
- Documentation should target **developers using the module** (not end users)
- Once Phase 4 is complete, all scripts should be **semantically identical** to their original behavior
- Consider adding a **registry/config** for device preferences, artifact paths, timeouts (Phase 6 enhancement)

---

## Future Extensions (Post-MVP)

Once the module is solid, it enables:
- CI/CD integration (GitHub Actions → module functions)
- Multi-device test farms (loop over device array)
- Automated regression testing (screenshot diffs, crash detection)
- Telemetry collection (parse logcat → metrics)
- Test report generation (HTML, JSON exports)

---
