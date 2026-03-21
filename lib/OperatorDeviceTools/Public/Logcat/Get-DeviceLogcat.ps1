function Get-DeviceLogcat {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [ValidateSet("Stream", "Buffer", "Diagnostic")]
        [string]$Mode = "Buffer",
        [string]$FilterPackage = "com.rfditservices.operatorgame",
        [int]$Lines = 200,
        [string]$Since = $null
    )
    
    <#
    .SYNOPSIS
    Retrieve logcat from device with flexible filtering and timestamp ranges.
    #>
    
    if (-not $Device.IsHealthy()) {
        throw "Cannot retrieve logs: Device $($Device.Serial) is offline or unhealthy."
    }
    
    if ($Mode -eq "Buffer") {
        $cmd = "logcat -d"
        if ($Since) { $cmd += " -T `"$Since`"" } else { $cmd += " -t $Lines" }
        
        $logStr = Invoke-AdbCommand -Serial $Device.Serial -Command $cmd -NoErrorCheck
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
        $cmd = "logcat -d -b crash,main -t 500"
        if ($Since) { $cmd += " -T `"$Since`"" }
        $logStr = Invoke-AdbCommand -Serial $Device.Serial -Command $cmd -NoErrorCheck
        return @($logStr -split "`r`n")
        
    } else {
        Start-LogcatStream -Device $Device -FilterPackage $FilterPackage
    }
}
