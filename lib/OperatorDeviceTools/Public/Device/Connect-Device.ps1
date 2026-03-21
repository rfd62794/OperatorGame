function Connect-Device {
    param(
        [string]$Serial = $null,
        [switch]$AutoLaunch = $true,
        [int]$TimeoutSeconds = 30
    )
    
    <#
    .SYNOPSIS
    Establish and validate connection to Android device.
    #>
    
    $adb = Resolve-AdbPath
    
    if (-not $Serial) {
        & $adb start-server 2>&1 | Out-Null
        $devices = @(& $adb devices | Select-Object -Skip 1 | Where-Object { $_ -match '\bdevice\b' })
        if ($devices.Count -eq 0) { throw "No devices currently connected to ADB daemon." }
        
        $Serial = ($devices[0] -split '\s+')[0]
    }
    
    $dev = [Device]::new()
    $dev.Serial = $Serial
    $dev.Refresh()
    
    if (-not $dev.IsHealthy()) {
        throw "Device $Serial is connected but failing health state (Offline or No Debug)."
    }
    
    return $dev
}
