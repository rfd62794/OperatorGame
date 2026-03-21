function Start-LogcatStream {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [string]$FilterPackage = "com.rfditservices.operatorgame"
    )
    
    <#
    .SYNOPSIS
    Start a live streaming logcat filtered by package.
    #>
    
    $adb = Resolve-AdbPath
    # Clean buffer first to ensure a strictly live tail
    & $adb -s $Device.Serial logcat -c
    
    Write-Host "Monitoring logs for $FilterPackage on $($Device.Serial)..." -ForegroundColor Cyan
    & $adb -s $Device.Serial logcat | Select-String -Pattern $FilterPackage
}
