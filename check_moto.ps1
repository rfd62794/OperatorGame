<#
.SYNOPSIS
OPERATOR: Moto G 2025 - ADB logcat monitor.

.DESCRIPTION
Streams logs filtered to the OperatorGame process.
Use -Diagnostic to capture a crash window instead of streaming.

.EXAMPLE
.\check_moto.ps1
.\check_moto.ps1 -Diagnostic
#>
param(
    [switch]$Diagnostic = $false
)

$PackageName = "com.rfditservices.operatorgame"

$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
if (-not (Test-Path $ADB) -and $env:ANDROID_HOME) {
    $ADB = "$env:ANDROID_HOME\platform-tools\adb.exe"
}

if (-not (Test-Path $ADB)) {
    Write-Host "ERROR: ADB not found." -ForegroundColor Red
    Write-Host "  Run '. .\setup_local_forge.ps1' to configure the Android SDK." -ForegroundColor Yellow
    exit 1
}

Write-Host "--- OPERATOR: MOTO G 2025 LOGCAT MONITOR ---" -ForegroundColor Cyan
Write-Host "  Package : $PackageName" -ForegroundColor DarkGray
Write-Host "  ADB     : $ADB" -ForegroundColor DarkGray
Write-Host "  Mode    : $(if ($Diagnostic) { 'DIAGNOSTIC (crash capture)' } else { 'STREAM' })" -ForegroundColor DarkGray
Write-Host ""

# Verify device connected
$prev = $ErrorActionPreference
$ErrorActionPreference = "Continue"
& $ADB start-server 2>&1 | Out-Null
$ErrorActionPreference = $prev

$rawDevices = & $ADB devices
$deviceLines = $rawDevices | Where-Object {
    $_ -match "\S" -and $_ -notmatch "List of devices" -and $_ -match "\bdevice\b"
}

if ($deviceLines.Count -eq 0) {
    Write-Host "ERROR: No ADB device connected." -ForegroundColor Red
    Write-Host "  Connect Moto G via USB and enable USB Debugging." -ForegroundColor Yellow
    exit 1
}

$DeviceSerial = (& $ADB get-serialno 2>&1).Trim()
Write-Host "  Device  : $DeviceSerial" -ForegroundColor Green
Write-Host ""

if ($Diagnostic) {
    # -----------------------------------------------------------------------
    # Diagnostic mode: clear buffer, wait briefly, dump and filter for crashes
    # -----------------------------------------------------------------------
    Write-Host "Clearing logcat buffer..." -ForegroundColor Cyan
    & $ADB -s $DeviceSerial logcat -c

    Write-Host "Waiting 8 seconds to capture crash window..." -ForegroundColor Yellow
    Write-Host "(Launch the app now if it is not running)" -ForegroundColor DarkGray
    Start-Sleep -Seconds 8

    Write-Host ""
    Write-Host "Crash-relevant logcat lines:" -ForegroundColor Cyan
    Write-Host "--------------------------------------------" -ForegroundColor DarkGray

    $patterns = "opera|linker|FATAL|dlopen|library|signal|error|exception|crash|abort"
    $hits = & $ADB -s $DeviceSerial logcat -d 2>$null |
        Select-String -Pattern $patterns -CaseInsensitive |
        Select-Object -Last 50

    if ($hits) {
        $hits | ForEach-Object { Write-Host "$_" }
    } else {
        Write-Host "  (No crash-relevant lines found in buffer.)" -ForegroundColor DarkGray
    }

    Write-Host ""
    Write-Host "Hint: Look for 'linker', 'dlopen', 'SIGKILL', 'SIGSEGV', or 'cannot locate symbol'." -ForegroundColor Cyan
} else {
    # -----------------------------------------------------------------------
    # Normal stream mode: filtered by PID or fallback *:E
    # -----------------------------------------------------------------------
    $AppPid = (& $ADB -s $DeviceSerial shell pidof $PackageName 2>$null)
    if ($AppPid) { $AppPid = $AppPid.Trim() }

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
}
