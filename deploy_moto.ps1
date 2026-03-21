<#
.SYNOPSIS
Full build → sign → install → launch → log cycle for Moto G 2025.

.DESCRIPTION
One-command deploy pipeline for OperatorGame development testing.
Chains: env setup → cargo-apk build → APK sign → adb install → app launch → logcat stream.

.EXAMPLE
.\deploy_moto.ps1

.NOTES
Prerequisites:
  - Moto G 2025 connected via USB with USB Debugging enabled
  - Android SDK installed (setup_local_forge.ps1 will validate)
  - Rust Android toolchain installed (setup_local_forge.ps1 will validate)
#>

$ErrorActionPreference = "Stop"
$PackageName = "com.rfditservices.operatorgame"
$Activity    = "android.app.NativeActivity"
$ApkFinal    = "operatorgame-release.apk"

# ─────────────────────────────────────────────────────────────────────────────
# Header
# ─────────────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  OperatorGame — Deploy to Moto G 2025                     ║" -ForegroundColor Cyan
Write-Host "║  build → sign → install → launch → log                    ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# ─────────────────────────────────────────────────────────────────────────────
# Step 1/5 — Environment Setup
# ─────────────────────────────────────────────────────────────────────────────

Write-Host "[1/5] Hardening build environment..." -ForegroundColor Cyan

# Dot-source to preserve env var + PATH changes in the current session
. .\setup_local_forge.ps1

Write-Host "  ✓ Build environment ready." -ForegroundColor Green

# Resolve ADB after forge has set ANDROID_HOME
$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

if (-not (Test-Path $ADB)) {
    Write-Host ""
    Write-Host "  ✗ ADB not found at: $ADB" -ForegroundColor Red
    Write-Host "    Ensure Android SDK platform-tools are installed via Android Studio." -ForegroundColor Yellow
    exit 1
}

# ─────────────────────────────────────────────────────────────────────────────
# Step 2/5 — Verify Moto G is Connected
# ─────────────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "[2/5] Scanning for connected ADB devices..." -ForegroundColor Cyan

$rawDevices = & $ADB devices 2>&1
# Parse: skip the "List of devices attached" header, filter blank lines and "offline"
$devices = $rawDevices |
    Select-Object -Skip 1 |
    Where-Object { $_ -match "\S" -and $_ -match "\tdevice$" }

if ($devices.Count -eq 0) {
    Write-Host ""
    Write-Host "  ✗ No ADB device detected." -ForegroundColor Red
    Write-Host "    — Connect Moto G via USB cable" -ForegroundColor Yellow
    Write-Host "    — Enable: Settings > Developer Options > USB Debugging" -ForegroundColor Yellow
    Write-Host "    — Tap 'Allow' on the phone USB authorisation prompt" -ForegroundColor Yellow
    Write-Host "    — Run: adb devices    to verify" -ForegroundColor Yellow
    exit 1
}

$DeviceSerial = ($devices[0] -split "\t")[0].Trim()
Write-Host "  ✓ Device connected: $DeviceSerial" -ForegroundColor Green

# Bind ADB to this specific serial for the remainder of the script
$ADB_ARGS = @("-s", $DeviceSerial)

# ─────────────────────────────────────────────────────────────────────────────
# Step 3/5 — Build + Sign APK
# ─────────────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "[3/5] Building and signing APK (this takes 2-5 minutes)..." -ForegroundColor Cyan
Write-Host "      cargo apk build --release + zipalign + apksigner" -ForegroundColor DarkGray

$BuildStart = Get-Date
.\build_android.ps1
$BuildSecs  = [int]((Get-Date) - $BuildStart).TotalSeconds

if (-not (Test-Path $ApkFinal)) {
    Write-Host ""
    Write-Host "  ✗ Build failed: '$ApkFinal' not found after build_android.ps1." -ForegroundColor Red
    Write-Host "    Check the output above for cargo-apk or signing errors." -ForegroundColor Yellow
    exit 1
}

$ApkSizeKB = [int]((Get-Item $ApkFinal).Length / 1KB)
Write-Host "  ✓ APK built and signed in ${BuildSecs}s — ${ApkSizeKB}KB" -ForegroundColor Green

# ─────────────────────────────────────────────────────────────────────────────
# Step 4/5 — Install APK on phone
# ─────────────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "[4/5] Installing APK on device ($DeviceSerial)..." -ForegroundColor Cyan

& $ADB @ADB_ARGS install -r $ApkFinal
if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "  ✗ adb install failed (exit $LASTEXITCODE)." -ForegroundColor Red
    Write-Host "    Common causes:" -ForegroundColor Yellow
    Write-Host "      — Phone locked during install (unlock and retry)" -ForegroundColor Yellow
    Write-Host "      — minSdkVersion mismatch (phone needs Android 8.0+)" -ForegroundColor Yellow
    Write-Host "      — Previous install corrupt: try 'adb uninstall $PackageName'" -ForegroundColor Yellow
    exit 1
}

Write-Host "  ✓ APK installed successfully." -ForegroundColor Green

# ─────────────────────────────────────────────────────────────────────────────
# Step 5/5 — Launch app + Stream logcat
# ─────────────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "[5/5] Launching $PackageName..." -ForegroundColor Cyan

& $ADB @ADB_ARGS shell am start -n "$PackageName/$Activity" | Out-Null

Start-Sleep -Milliseconds 800   # Give the app a moment to start

# Resolve PID for filtered logcat. Fallback to unfiltered *:E if pidof fails.
$AppPid = & $ADB @ADB_ARGS shell pidof $PackageName 2>$null
$AppPid = $AppPid.Trim()

Write-Host "  ✓ App launched (PID: $(if ($AppPid) { $AppPid } else { 'resolving...' }))" -ForegroundColor Green
Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor DarkGray
Write-Host "  Streaming logcat  (Ctrl+C to stop)" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor DarkGray
Write-Host ""

# Clear stale buffer before streaming
& $ADB @ADB_ARGS logcat -c

if ($AppPid) {
    # Filtered: only log output from the OperatorGame process
    & $ADB @ADB_ARGS logcat --pid=$AppPid
} else {
    # Fallback: Error-level only to reduce noise
    Write-Host "  (PID not resolved — streaming *:E fallback)" -ForegroundColor DarkGray
    & $ADB @ADB_ARGS logcat *:E
}
