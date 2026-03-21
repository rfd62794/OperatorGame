<#
.SYNOPSIS
Integrated Android device testing suite for OperatorGame.

.EXAMPLE
.\run_android_tests.ps1
.\run_android_tests.ps1 -DiagnosticOnly
.\run_android_tests.ps1 -Serial ZY2247XXXXX
#>
param(
    [string]$Serial        = "",
    [switch]$DiagnosticOnly = $false,
    [switch]$Verbose        = $false
)

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  OperatorGame Android Testing Suite" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# Load modules
. .\lib\device_registry.ps1
. .\lib\logcat_monitor.ps1

# ---------------------------------------------------------------------------
# 1. Device detection
# ---------------------------------------------------------------------------
Write-Host "[1/4] Device detection..." -ForegroundColor Cyan

$devices = Get-ConnectedDevices

if ($devices.Count -eq 0) {
    Write-Host "  FAIL: No ADB devices connected." -ForegroundColor Red
    Write-Host "    Connect Moto G via USB and enable USB Debugging." -ForegroundColor Yellow
    exit 1
}

$target = if ($Serial) {
    $devices | Where-Object { $_.Serial -eq $Serial } | Select-Object -First 1
} else {
    $devices[0]
}

if (-not $target) {
    Write-Host "  FAIL: Serial '$Serial' not found. Connected devices:" -ForegroundColor Red
    $devices | ForEach-Object { Write-Host "    $($_.Serial) - $($_.Model)" }
    exit 1
}

Write-Host "  OK: $($target.Serial) - $($target.Model)" -ForegroundColor Green

# ---------------------------------------------------------------------------
# 2. Device health check
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[2/4] Device health check..." -ForegroundColor Cyan

$health = Test-DeviceHealth $target.Serial

if ($health.Issues.Count -eq 0) {
    Write-Host "  OK: Device healthy (API $($health.APILevel), $($health.StorageFreeGB) GB free)" -ForegroundColor Green
} else {
    Write-Host "  WARNING: $($health.Issues.Count) issue(s) found:" -ForegroundColor Yellow
    $health.Issues | ForEach-Object { Write-Host "    - $_" -ForegroundColor Yellow }
}

if ($DiagnosticOnly) {
    Write-Host ""
    Write-Host "  Diagnostic mode - skipping deploy." -ForegroundColor DarkGray
    Write-Host ""
    Write-Host "APK Contents:" -ForegroundColor Cyan
    & .\diagnose_apk.ps1
    Write-Host ""
    Write-Host "[DONE] Diagnostic complete." -ForegroundColor Green
    exit 0
}

# ---------------------------------------------------------------------------
# 3. Deploy APK
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[3/4] Deploying APK to $($target.Serial)..." -ForegroundColor Cyan

.\deploy_moto.ps1

# ---------------------------------------------------------------------------
# 4. Logcat monitoring (crash detection pass)
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[4/4] Post-launch logcat capture (5 seconds)..." -ForegroundColor Cyan
Start-Sleep -Seconds 5

$crashLines = Get-CrashDump -Serial $target.Serial
$crashes = Detect-CrashInLines -Lines ($crashLines | ForEach-Object { "$_" })

if ($crashes.Count -gt 0) {
    Write-Host "  CRASH DETECTED:" -ForegroundColor Red
    $crashes | ForEach-Object { Write-Host "    $_" -ForegroundColor Red }
} else {
    Write-Host "  OK: No crash signals detected in logcat." -ForegroundColor Green
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  Testing complete." -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Cyan
