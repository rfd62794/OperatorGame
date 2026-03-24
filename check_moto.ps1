Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force

try {
    $dev = Connect-Device
} catch {
    Write-Warning "Device connection pipeline dead. $_"
    exit
}

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  Device Health Check: $($dev.Model)" -ForegroundColor Cyan
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan

Write-Host "Serial Target: $($dev.Serial)"
Write-Host "Daemon State:  $($dev.State)"
Write-Host "Target Object: $($dev.ToString())"

$native_pid = Is-AppRunning -Device $dev
if ($native_pid) {
    Write-Host "Game Client:   Online (Native PID: $native_pid)" -ForegroundColor Green
} else {
    Write-Host "Game Client:   Offline" -ForegroundColor Yellow
}

Write-Host "`nDiagnostics (App Crash Telemetry):" -ForegroundColor Cyan
$logs = Get-DeviceLogcat -Device $dev -Mode Diagnostic
$crash = Detect-AppCrash -LogLines $logs -Sensitivity Warn

if ($crash.Crashed) {
    Write-Host "FAIL: $($crash.Type)" -ForegroundColor Red
    Write-Host "Context StackTraces (Last 10 Lines):" -ForegroundColor Gray
    $crash.Details | Select-Object -Last 10 | ForEach-Object { Write-Host "  > $_" }
} else {
    Write-Host "PASS: No recent Native or JVM app crashes detected on log stream." -ForegroundColor Green
}
