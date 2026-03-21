function Diagnose-ApkIssues {
    param(
        [Parameter(Mandatory=$true)]
        [string]$ApkPath
    )
    
    <#
    .SYNOPSIS
    Comprehensive APK health check aggregating signature and structural content validations.
    #>
    
    $issues = @()
    $warnings = @()
    
    if (-not (Test-Path $ApkPath)) {
        $issues += "APK file completely missing from disk array bounds."
        return @{ Issues = $issues; Warnings = $warnings; Contents = $null }
    }
    
    $sig = Verify-ApkSignature -ApkPath $ApkPath
    if (-not $sig) {
        $issues += "APK Cryptographic Signature Verification internally Failed or pipeline dependency natively missing in Environment."
    }
    
    $contents = Get-ApkContents -ApkPath $ApkPath
    if ($contents) {
        if ($contents.NativeArchitectures -eq "None") {
            $warnings += "No native libraries (.so) identified in package. Verify C++ compile targets."
        }
        if ($contents.Package -eq "Unknown") {
            $issues += "Failed to parse literal package name from APK internal badging hierarchy."
        }
    } else {
        $warnings += "Could not analyze internal contents. Aapt pipeline degraded."
    }
    
    return @{
        Issues = $issues
        Warnings = $warnings
        Contents = $contents
    }
}
