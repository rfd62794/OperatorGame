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
    #>
    
    $pmList = Invoke-AdbCommand -Serial $Device.Serial -Command "shell pm list packages com.rfditservices.operatorgame" -NoErrorCheck
    $isInstalled = ($pmList -match "com.rfditservices.operatorgame")
    
    if ($isInstalled -and -not $Force) {
        return $true
    }
    
    if (-not $ApkPath) {
        $repoPath = $Script:OperatorDeviceTools.RepositoryRoot
        $apkFiles = Get-ChildItem -Path $repoPath -Filter "*operatorgame-release*.apk" -Recurse -ErrorAction SilentlyContinue
        if (-not $apkFiles) {
            throw "OperatorGame is not installed, and no matching .apk files were found in the workspace."
        }
        
        $ApkPath = ($apkFiles | Sort-Object LastWriteTime -Descending)[0].FullName
        Write-Host "Auto-discovered APK: $ApkPath" -ForegroundColor Cyan
    }
    
    if (-not (Test-Path $ApkPath)) {
        throw "APK not found at path: $ApkPath"
    }
    
    Write-Host "Installing APK onto $($Device.Serial)..." -ForegroundColor Yellow
    $installArgs = if ($Force) { "install -r" } else { "install" }
    
    $out = Invoke-AdbCommand -Serial $Device.Serial -Command "$installArgs `"$ApkPath`"" -TimeoutSeconds 120
    
    if ($out -match "Success") {
        Write-Host "Installation successful." -ForegroundColor Green
        return $true
    } else {
        throw "APK Installation failed:`n$out"
    }
}
