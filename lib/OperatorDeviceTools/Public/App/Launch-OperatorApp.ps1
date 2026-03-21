function Launch-OperatorApp {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [int]$WaitSeconds = 8,
        [switch]$KillIfRunning = $false
    )
    
    <#
    .SYNOPSIS
    Launch OperatorGame on device with async startup verification.
    
    .DESCRIPTION
    Uses Native Android monkey launcher to resolve intents dynamically,
    waiting structural milliseconds for the engine to boot.
    
    .OUTPUTS
    [int] Process ID (PID) or throws on failure
    #>
    
    if ($KillIfRunning) {
        Stop-OperatorApp -Device $Device
        Start-Sleep -Seconds 1
    }
    
    $pidNum = Is-AppRunning -Device $Device
    if (-not $pidNum) {
        Invoke-AdbCommand -Serial $Device.Serial -Command "shell monkey -p com.rfditservices.operatorgame -c android.intent.category.LAUNCHER 1" -NoErrorCheck | Out-Null
        
        Write-Host "Waiting $WaitSeconds seconds for app bootstrap..." -ForegroundColor Gray
        Start-Sleep -Seconds $WaitSeconds
        
        $pidNum = Is-AppRunning -Device $Device
        if (-not $pidNum) {
            throw "Failed to launch app or acquire PID string."
        }
    }
    
    return $pidNum
}
