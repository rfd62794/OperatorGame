<#
.SYNOPSIS
OPERATOR: Moto G 2025 - ADB logcat monitor.
Streams logs filtered to the OperatorGame process.

.DESCRIPTION
Resolves ADB from the Android SDK (no hardcoded paths), clears the logcat buffer,
then streams process-filtered output. Use this for targeted debug monitoring
when the app is already running on the device.

.EXAMPLE
.\check_moto.ps1
#>

$PackageName = "com.rfditservices.operatorgame"

# Resolve ADB from Android SDK (consistent with build_android.ps1 pattern)
$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

# Fallback: try ANDROID_HOME env var if LOCALAPPDATA path does not exist
if (-not (Test-Path $ADB) -and $env:ANDROID_HOME) {
    $ADB = "$env:ANDROID_HOME\platform-tools\adb.exe"
}

if (-not (Test-Path $ADB)) {
    Write-Host "ERROR: ADB not found." -ForegroundColor Red
    Write-Host "  Expected: $ADB" -ForegroundColor Yellow
    Write-Host "  Run '. .\setup_local_forge.ps1' to validate and configure your Android SDK." -ForegroundColor Yellow
    exit 1
}

Write-Host "--- OPERATOR: MOTO G 2025 LOGCAT MONITOR ---" -ForegroundColor Cyan
Write-Host "  Package : $PackageName" -ForegroundColor DarkGray
Write-Host "  ADB     : $ADB" -ForegroundColor DarkGray
Write-Host ""

# Verify a device is connected
$rawDevices = & $ADB devices 2>&1
$devices = $rawDevices |
    Select-Object -Skip 1 |
    Where-Object { $_ -match "\S" -and $_ -match "`tdevice$" }

if ($devices.Count -eq 0) {
    Write-Host "ERROR: No ADB device connected." -ForegroundColor Red
    Write-Host "  Connect Moto G via USB and enable USB Debugging." -ForegroundColor Yellow
    exit 1
}

$DeviceSerial = ($devices[0] -split "`t")[0].Trim()
Write-Host "  Device  : $DeviceSerial" -ForegroundColor Green
Write-Host ""

# Resolve PID of the running OperatorGame process
$AppPid = (& $ADB -s $DeviceSerial shell pidof $PackageName 2>$null)
if ($AppPid) { $AppPid = $AppPid.Trim() }

# Clear stale logcat buffer
& $ADB -s $DeviceSerial logcat -c

Write-Host "Streaming logs (Ctrl+C to stop)..." -ForegroundColor Cyan
Write-Host "----------------------------------------" -ForegroundColor DarkGray

if ($AppPid) {
    Write-Host "  Filtered to PID $AppPid ($PackageName)" -ForegroundColor Green
    & $ADB -s $DeviceSerial logcat --pid=$AppPid
} else {
    Write-Host "  App not running - streaming all errors as fallback" -ForegroundColor Yellow
    Write-Host "  Launch the app first, then re-run for filtered output." -ForegroundColor DarkGray
    & $ADB -s $DeviceSerial logcat *:E
}
