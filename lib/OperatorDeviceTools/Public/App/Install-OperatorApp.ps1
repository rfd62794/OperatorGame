function Install-OperatorApp {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [string]$ApkPath = $null,
        [switch]$Force = $false
    )
    
    <#
    .SYNOPSIS
    Install OperatorGame APK on device (with auto-healing).
    
    .DESCRIPTION
    - Validates presence in `pm list`
    - Parses workplace for APK artifacts if none provided
    - Installs using standard hooks
    
    .OUTPUTS
    [bool] $true if install successful, throws on failure
    #>
    
    $pmList = Invoke-AdbCommand -Serial $Device.Serial -Command "shell pm list packages com.rfditservices.operatorgame" -NoErrorCheck
    $isInstalled = ($pmList -match "com.rfditservices.operatorgame")
    
    if ($isInstalled -and -not $Force) {
        return $true
    }
    
    if (-not $ApkPath) {
        $repoPath = $Script:OperatorDeviceTools.RepositoryRoot
        $apkFiles = Get-ChildItem -Path $repoPath -Filter "*.apk" -Recurse -ErrorAction SilentlyContinue
        if (-not $apkFiles) {
            throw "OperatorGame is not installed, and no .apk files were found in the workspace."
        }
        
        $ApkPath = ($apkFiles | Sort-Object LastWriteTime -Descending)[0].FullName
        Write-Host "Auto-discovered APK Artifact: $ApkPath" -ForegroundColor Cyan
    }
    
    if (-not (Test-Path $ApkPath)) {
        throw "APK artifact strictly not found at path: $ApkPath"
    }
    
    Write-Host "Re-deploying APK architecture to $($Device.Serial)..." -ForegroundColor Yellow
    $installArgs = if ($Force) { "install -r" } else { "install" }
    
    $out = Invoke-AdbCommand -Serial $Device.Serial -Command "$installArgs `"$ApkPath`"" -TimeoutSeconds 120
    
    if ($out -match "Success") {
        Write-Host "Deployment successful." -ForegroundColor Green
        return $true
    } else {
        throw "APK Build Installation failed:`n$out"
    }
}
