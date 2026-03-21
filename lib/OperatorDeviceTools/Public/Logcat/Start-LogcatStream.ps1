function Start-LogcatStream {
    param(
        [Parameter(Mandatory=$true)]
        [Device]$Device,
        [string]$FilterPackage = "com.rfditservices.operatorgame",
        [ValidateSet("V","D","I","W","E","F","S")]
        [string]$Severity = "V",
        [switch]$AsJob
    )
    
    <#
    .SYNOPSIS
    Start a live streaming logcat filtered by package and severity.
    #>
    
    $adb = Resolve-AdbPath
    & $adb -s $Device.Serial logcat -c
    
    $filter = if ($FilterPackage) { $FilterPackage } else { ".*" }
    
    if ($AsJob) {
        $scriptBlock = {
            param($adbPath, $serial, $filterStr, $sev)
            & $adbPath -s $serial logcat "*:$sev" | Select-String -Pattern $filterStr
        }
        Write-Host "Starting background Logcat job (Severity $Severity+)..." -ForegroundColor Cyan
        return Start-Job -ScriptBlock $scriptBlock -ArgumentList $adb, $Device.Serial, $filter, $Severity
    } else {
        Write-Host "Monitoring logs (Severity $Severity+) for $FilterPackage on $($Device.Serial)... (Ctrl+C to stop)" -ForegroundColor Cyan
        & $adb -s $Device.Serial logcat "*:$Severity" | Select-String -Pattern $filter
    }
}
