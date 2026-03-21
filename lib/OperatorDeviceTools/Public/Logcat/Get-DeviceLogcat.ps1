function Get-DeviceLogcat {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [ValidateSet("Stream", "Buffer", "Diagnostic")]
        [string]$Mode = "Buffer",
        [string]$FilterPackage = "com.rfditservices.operatorgame",
        [int]$Lines = 200
    )
    
    <#
    .SYNOPSIS
    Retrieve logcat from device with flexible filtering.
    
    .DESCRIPTION
    - Mode "Stream": Live stream to terminal (Ctrl+C to stop)
    - Mode "Buffer": Dump full buffer to string array
    - Mode "Diagnostic": Capture crash context around recent errors
    
    .OUTPUTS
    [string[]] logcat lines
    #>
    
    if ($Mode -eq "Buffer") {
        $logStr = Invoke-AdbCommand -Serial $Device.Serial -Command "logcat -d -t $Lines" -NoErrorCheck
        $linesArr = $logStr -split "`r`n"
        if ($FilterPackage) {
            $pidRaw = Invoke-AdbCommand -Serial $Device.Serial -Command "shell pidof $FilterPackage" -NoErrorCheck
            $pidVal = $pidRaw.Trim()
            if ($pidVal -match '^\d+$') {
                return @($linesArr | Where-Object { $_ -match "\b$pidVal\b" -or $_ -match $FilterPackage })
            } else {
                return @($linesArr | Where-Object { $_ -match $FilterPackage })
            }
        }
        return @($linesArr)
    } elseif ($Mode -eq "Diagnostic") {
        $logStr = Invoke-AdbCommand -Serial $Device.Serial -Command "logcat -d -t 500" -NoErrorCheck
        return @($logStr -split "`r`n")
    } else {
        # Stream Mode
        Write-Host "Streaming logcat for $($Device.Serial)... (Press Ctrl+C to stop)" -ForegroundColor Cyan
        $adb = Resolve-AdbPath
        & $adb -s $Device.Serial logcat
    }
}
