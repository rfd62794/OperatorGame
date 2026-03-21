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
    
    Invoke-AdbCommand -Serial $Device.Serial -Command "shell input tap $X $Y" | Out-Null
    
    if ($DelayMs -gt 0) {
        Start-Sleep -Milliseconds $DelayMs
    }
}
