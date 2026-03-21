function Audit-NdkConfig {
    <#
    .SYNOPSIS
    Verify NDK installation and configuration consistency globally.
    #>
    
    $ndkBase = "$env:LOCALAPPDATA\Android\Sdk\ndk"
    $installed = @()
    if (Test-Path $ndkBase) {
        $installed = (Get-ChildItem $ndkBase -Directory).Name
    }
    
    $configured = $env:ANDROID_NDK_HOME
    $mismatches = @()
    if ($configured -and -not (Test-Path $configured)) {
        $mismatches += "ANDROID_NDK_HOME strictly points to null or non-existent path mapping: $configured"
    }
    
    return @{
        ConfiguredVersion = $configured
        InstalledVersions = $installed
        Mismatches = $mismatches
    }
}
