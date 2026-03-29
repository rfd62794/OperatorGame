function Invoke-DeviceTap {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [Parameter(Mandatory=$true)]
        [int]$X,
        [Parameter(Mandatory=$true)]
        [int]$Y,
        [int]$DelayMs = 500
    )
    
    <#
    .SYNOPSIS
    Simulate direct XY coordinate touch input on the target device screen.
    #>
    
    if (-not $Device.IsHealthy()) {
        throw "Cannot tap: Device $($Device.Serial) is not responsive or healthy."
    }
    
    if ($X -lt 0 -or $Y -lt 0 -or $X -gt 32768 -or $Y -gt 32768) {
        throw "Coordinates ($X, $Y) are outside reasonable boundary values (0-32768)."
    }
    
    Invoke-AdbCommand -Serial $Device.Serial -Command "shell input tap $X $Y" | Out-Null
    
    if ($DelayMs -gt 0) {
        Start-Sleep -Milliseconds $DelayMs
    }
}
