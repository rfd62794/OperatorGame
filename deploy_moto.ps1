Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  Deploy OperatorGame to Moto architecture" -ForegroundColor Cyan
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan

try {
    $dev = Connect-Device
    Write-Host "[OK] Locked Target: $($dev.ToString())" -ForegroundColor Green

    Write-Host "Sideloading Artifact Payload..." -ForegroundColor Gray
    Install-OperatorApp -Device $dev -Force | Out-Null
    
    Write-Host "Triggering Engine bootstrap..." -ForegroundColor Gray
    $pidNum = Launch-OperatorApp -Device $dev -KillIfRunning
    
    Write-Host "[OK] Fully Deployed and Active. Native PID: $pidNum" -ForegroundColor Green
} catch {
    Write-Error "Deployment Pipeline Failure: $_"
}
