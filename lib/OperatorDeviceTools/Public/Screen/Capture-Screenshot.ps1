function Capture-Screenshot {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [Parameter(Mandatory=$true)]
        [string]$OutputPath,
        [string]$Label = "Screenshot"
    )
    
    <#
    .SYNOPSIS
    Capture screen from device and save to disk cleanly.
    
    .DESCRIPTION
    Uses adb shell screencap to save locally on the device's /tmp directory,
    then executes an adb pull to transfer the binary safely without UTF-16
    pipeline corruption, then MD5 checksums.
    
    .OUTPUTS
    [Screenshot] object with metadata (path, timestamp, size, hash)
    #>
    
    $dir = Split-Path -Parent $OutputPath
    if ($dir -and -not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir | Out-Null
    }
    
    $tmpDevicePath = "/data/local/tmp/screencap_$([guid]::NewGuid().ToString().Substring(0,8)).png"
    
    Invoke-AdbCommand -Serial $Device.Serial -Command "shell screencap -p $tmpDevicePath" | Out-Null
    Invoke-AdbCommand -Serial $Device.Serial -Command "pull $tmpDevicePath `"$OutputPath`"" | Out-Null
    Invoke-AdbCommand -Serial $Device.Serial -Command "shell rm $tmpDevicePath" -NoErrorCheck | Out-Null
    
    if (-not (Test-Path $OutputPath)) {
        throw "Failed to capture binary screenshot artifact to path: $OutputPath"
    }
    
    $fileInfo = Get-Item $OutputPath
    $hashStr = (Get-FileHash -Path $OutputPath -Algorithm MD5).Hash
    
    $shot = [Screenshot]::new()
    $shot.FilePath = $fileInfo.FullName
    $shot.Label = $Label
    $shot.Timestamp = $fileInfo.CreationTime
    $shot.SizeKb = [math]::Round($fileInfo.Length / 1KB, 1)
    $shot.Hash = $hashStr
    $shot.DeviceSerial = $Device.Serial
    
    return $shot
}
