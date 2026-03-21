function Verify-ApkSignature {
    param(
        [Parameter(Mandatory=$true)]
        [string]$ApkPath
    )
    
    <#
    .SYNOPSIS
    Validate APK signing using apksigner.
    #>
    
    if (-not (Test-Path $ApkPath)) { throw "APK not found: $ApkPath" }
    
    $apksigner = Get-Command "apksigner.bat" -ErrorAction SilentlyContinue
    if (-not $apksigner) {
        # Fallback to shell execution checks if apksigner isn't mapped directly to System.PATH
        Write-Warning "apksigner.bat not directly bound to PATH. Skipping explicit signature verification."
        return $false
    }
    
    $out = & $apksigner verify --print-certs $ApkPath 2>&1
    
    if ($LASTEXITCODE -eq 0 -and $out -match "Signer #1") {
        return $true
    }
    
    return $false
}
