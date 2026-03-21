Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  UI Coordinate Finder (Interactive)" -ForegroundColor Cyan
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan

$dev = Connect-Device
Write-Host "[OK] Master Device locked: $($dev.Serial)" -ForegroundColor Green

Write-Host "`nApp Integrity Sequence..." -ForegroundColor Yellow
$isInstalled = Install-OperatorApp -Device $dev -Force:$false

$pidNum = Launch-OperatorApp -Device $dev
Write-Host "[OK] App locked and active (PID: $pidNum)`n" -ForegroundColor Green

Write-Host "Instructions: For each prompt, tap the target UI element on your device." -ForegroundColor Yellow
Write-Host "If you miss, simply tap again. Only your LAST tap before pressing Enter is saved.`n" -ForegroundColor Yellow

$targets = @("Roster", "Missions", "Map", "Logs", "Combat_Deploy", "Back_Button")
$coords = @{}

$adb = Resolve-AdbPath
foreach ($target in $targets) {
    Write-Host ">>> TARGET: $target" -ForegroundColor Cyan
    Write-Host "  Tap the screen, then press ENTER to confirm: " -NoNewline -ForegroundColor Gray
    
    # We execute raw getevent via job since this requires active stream parsing
    $job = Start-Job -ScriptBlock {
        param($adbPath, $serial)
        & $adbPath -s $serial shell getevent -l 2>&1
    } -ArgumentList $adb, $dev.Serial
    
    $null = Read-Host
    Stop-Job $job
    
    $events = Receive-Job $job
    Remove-Job $job
    
    $lastX = 0
    $lastY = 0
    foreach ($line in $events) {
        if ($line -match "ABS_MT_POSITION_X\s+([0-9a-fA-F]+)") { $lastX = [Convert]::ToInt32($matches[1], 16) }
        if ($line -match "ABS_MT_POSITION_Y\s+([0-9a-fA-F]+)") { $lastY = [Convert]::ToInt32($matches[1], 16) }
    }
    
    $coords[$target] = @{ X = $lastX; Y = $lastY }
    Write-Host "  [SAVED] $target -> ($lastX, $lastY)`n" -ForegroundColor Green
}

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "Final Coordinate Map:" -ForegroundColor Yellow
$coords.GetEnumerator() | Sort-Object Name | Format-Table @{Label="Target"; Expression={$_.Name}}, @{Label="X"; Expression={$_.Value.X}}, @{Label="Y"; Expression={$_.Value.Y}} -AutoSize

$outPath = Join-Path $PSScriptRoot "ui_coordinates.json"
$coords | ConvertTo-Json -Depth 3 | Set-Content $outPath
Write-Host "Mapping dumped to $outPath" -ForegroundColor Green
