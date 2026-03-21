function Get-ApkContents {
    param(
        [Parameter(Mandatory=$true)]
        [string]$ApkPath
    )
    
    <#
    .SYNOPSIS
    Inspect .so files and resources within an APK structurally mapping package identities.
    #>
    
    if (-not (Test-Path $ApkPath)) { throw "APK not found: $ApkPath" }
    
    $aapt = Get-Command "aapt.exe" -ErrorAction SilentlyContinue
    if (-not $aapt) {
        Write-Warning "aapt.exe not found in PATH. Cannot inspect internal binary contents."
        return $null
    }
    
    $out = & $aapt dump badging $ApkPath 2>&1
    
    $package = if ($out -match "package: name='([^']+)'") { $matches[1] } else { "Unknown" }
    $versionCode = if ($out -match "versionCode='([^']+)'") { $matches[1] } else { "Unknown" }
    $versionName = if ($out -match "versionName='([^']+)'") { $matches[1] } else { "Unknown" }
    $nativeCode = if ($out -match "native-code: '([^']+)'") { $matches[1] } else { "None" }
    
    return [PSCustomObject]@{
        Package = $package
        VersionCode = $versionCode
        VersionName = $versionName
        NativeArchitectures = $nativeCode
    }
}
