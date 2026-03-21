# lib/device_registry.ps1
# Device detection and health checking module.
# Dot-source this file: . .\lib\device_registry.ps1

function Get-AdbPath {
    $adb = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
    if (-not (Test-Path $adb) -and $env:ANDROID_HOME) {
        $adb = "$env:ANDROID_HOME\platform-tools\adb.exe"
    }
    if (-not (Test-Path $adb)) {
        throw "ADB not found. Run '. .\setup_local_forge.ps1' first."
    }
    return $adb
}

function Get-ConnectedDevices {
    <#
    .SYNOPSIS
    Returns an array of connected ADB devices with serial, state, and model.
    #>
    $ADB = Get-AdbPath

    # Ensure daemon is running without triggering ErrorActionPreference = Stop
    $prev = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & $ADB start-server 2>&1 | Out-Null
    $ErrorActionPreference = $prev

    $lines = & $ADB devices | Select-Object -Skip 1 | Where-Object { $_.Trim() -ne "" }

    $devices = @()
    foreach ($line in $lines) {
        $parts = ($line -split "\s+")
        if ($parts.Count -ge 2 -and $parts[1] -eq "device") {
            $serial = $parts[0].Trim()
            $model  = (& $ADB -s $serial shell getprop ro.product.model 2>$null).Trim()
            $devices += [PSCustomObject]@{
                Serial = $serial
                State  = $parts[1]
                Model  = $model
            }
        }
    }

    return $devices
}

function Test-DeviceHealth {
    <#
    .SYNOPSIS
    Checks API level, debug state, and free storage for a given device serial.
    #>
    param([string]$Serial)

    $ADB = Get-AdbPath

    $health = [PSCustomObject]@{
        Serial       = $Serial
        Connected    = $false
        APILevel     = $null
        DebugEnabled = $false
        StorageFreeGB = $null
        Issues       = [System.Collections.Generic.List[string]]::new()
    }

    try {
        $state = (& $ADB -s $Serial get-state 2>$null).Trim()
        $health.Connected = ($state -eq "device")

        if (-not $health.Connected) {
            $health.Issues.Add("Device not in online state: $state")
            return $health
        }

        $health.APILevel    = (& $ADB -s $Serial shell getprop ro.build.version.sdk 2>$null).Trim()
        $debuggable         = (& $ADB -s $Serial shell getprop ro.debuggable 2>$null).Trim()
        $health.DebugEnabled = ($debuggable -eq "1")

        if (-not $health.DebugEnabled) {
            $health.Issues.Add("USB debugging not enabled on device")
        }

        if ([int]$health.APILevel -lt 26) {
            $health.Issues.Add("API level $($health.APILevel) < 26 (minSdkVersion)")
        }

        # Free storage (data partition, in KB)
        $dfOut = (& $ADB -s $Serial shell "df /data" 2>$null) | Select-Object -Last 1
        if ($dfOut -match "\s+(\d+)\s+\d+\s*$") {
            $freeKB = [int]$matches[1]
            $health.StorageFreeGB = [math]::Round($freeKB / 1024 / 1024, 2)
            if ($freeKB -lt 512000) {
                $health.Issues.Add("Low storage: $($health.StorageFreeGB) GB free")
            }
        }

    } catch {
        $health.Issues.Add("Health check exception: $_")
    }

    return $health
}
