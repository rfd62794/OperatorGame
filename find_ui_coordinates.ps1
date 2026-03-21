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

$adbPath = "adb.exe"
if (-not (Get-Command "adb.exe" -ErrorAction SilentlyContinue)) {
    $adbPath = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
}

foreach ($target in $targets) {
    Write-Host ">>> TARGET: $target" -ForegroundColor Cyan
    Write-Host "  Tap the screen (coordinates will stream here). Press ENTER when finished..." -ForegroundColor Gray
    
    $proc = New-Object System.Diagnostics.Process
    $proc.StartInfo.FileName = $adbPath
    $proc.StartInfo.Arguments = "-s $($dev.Serial) shell getevent -l"
    $proc.StartInfo.UseShellExecute = $false
    $proc.StartInfo.RedirectStandardOutput = $true
    $proc.StartInfo.CreateNoWindow = $true
    $proc.Start() | Out-Null
    
    $lastX = 0
    $lastY = 0
    
    $done = $false
    while (-not $done) {
        if ([Console]::KeyAvailable) {
            $key = [Console]::ReadKey($true)
            if ($key.Key -eq [ConsoleKey]::Enter) {
                $done = $true
            }
        }
        
        if (-not $done) {
            if ($proc.StandardOutput.EndOfStream) { 
                Start-Sleep -Milliseconds 10
                continue 
            }
            $line = $proc.StandardOutput.ReadLine()
            $updated = $false
            if ($line -match "ABS_MT_POSITION_X\s+([0-9a-fA-F]+)") { $lastX = [Convert]::ToInt32($matches[1], 16); $updated = $true }
            if ($line -match "ABS_MT_POSITION_Y\s+([0-9a-fA-F]+)") { $lastY = [Convert]::ToInt32($matches[1], 16); $updated = $true }
            
            if ($updated) {
                Write-Host "    -> Touch Detected: X:$lastX Y:$lastY" -ForegroundColor DarkGray
            }
        }
    }
    $proc.Kill()
    
    $coords[$target] = @{ X = $lastX; Y = $lastY }
    Write-Host "  [SAVED] $target -> ($lastX, $lastY)`n" -ForegroundColor Green
}

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "Final Coordinate Map:" -ForegroundColor Yellow
$coords.GetEnumerator() | Sort-Object Name | Format-Table @{Label="Target"; Expression={$_.Name}}, @{Label="X"; Expression={$_.Value.X}}, @{Label="Y"; Expression={$_.Value.Y}} -AutoSize

$outPath = Join-Path $PSScriptRoot "ui_coordinates.json"
$coords | ConvertTo-Json -Depth 3 | Set-Content $outPath
Write-Host "Mapping dumped to $outPath" -ForegroundColor Green
