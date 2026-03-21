function Launch-OperatorApp {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [int]$TimeoutSeconds = 15,
        [switch]$KillIfRunning = $false
    )
    
    <#
    .SYNOPSIS
    Launch OperatorGame on device with async startup verification.
    #>
    
    if ($KillIfRunning) {
        Stop-OperatorApp -Device $Device
        Start-Sleep -Seconds 1
    }
    
    $pidNum = Is-AppRunning -Device $Device
    if (-not $pidNum) {
        # Monkey is still the most resilient dynamic launcher (bypasses NativeActivity crashes)
        Invoke-AdbCommand -Serial $Device.Serial -Command "shell monkey -p com.rfditservices.operatorgame -c android.intent.category.LAUNCHER 1" -NoErrorCheck | Out-Null
        
        Write-Host "Polling for app bootstrap..." -ForegroundColor Gray
        $elapsed = 0
        while (-not $pidNum -and ($elapsed -lt $TimeoutSeconds)) {
            Start-Sleep -Milliseconds 500
            $elapsed += 0.5
            $pidNum = Is-AppRunning -Device $Device
        }
        
        if (-not $pidNum) {
            throw "Timeout: Failed to launch app or acquire PID after $TimeoutSeconds seconds."
        }
    }
    
    return $pidNum
}
