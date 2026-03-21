Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force

try {
    $dev = Connect-Device
} catch {
    Write-Warning "Device connection pipeline dead. $_"
    exit
}

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  Device Profile Telemetry: $($dev.Model)" -ForegroundColor Cyan
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan

Write-Host "Serial Target: $($dev.Serial)"
Write-Host "Daemon State:  $($dev.State)"
Write-Host "OS Android:    $($dev.AndroidVersion) (API $($dev.ApiLevel))"
Write-Host "Disk Free:     $($dev.StorageFreeGb) GB"

$pid = Is-AppRunning -Device $dev
if ($pid) {
    Write-Host "Game Client:   Online (Native PID: $pid)" -ForegroundColor Green
} else {
    Write-Host "Game Client:   Offline" -ForegroundColor Yellow
}
