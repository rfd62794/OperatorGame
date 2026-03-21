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
Write-Host "ONE-TAP TRUST VALIDATION ENABLED. Script will advance the moment you touch." -ForegroundColor Yellow

$targets = @(
    @{ Id="Roster"; Desc="Bottom Navigation Bar: First Tab (Left)" },
    @{ Id="Missions"; Desc="Bottom Navigation Bar: Second Tab" },
    @{ Id="Map"; Desc="Bottom Navigation Bar: Third Tab" },
    @{ Id="Logs"; Desc="Bottom Navigation Bar: Fourth Tab (Right)" },
    @{ Id="Combat_Deploy"; Desc="Center Screen: Large Combat Deploy Button" },
    @{ Id="Back_Button"; Desc="Top Left: Back/Return Navigation Arrow" }
)
$coords = @{}

$adbPath = "adb.exe"
if (-not (Get-Command "adb.exe" -ErrorAction SilentlyContinue)) {
    $adbPath = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
}

foreach ($t in $targets) {
    Write-Host "`n>>> TARGET: $($t.Id)" -ForegroundColor Cyan
    Write-Host "  $($t.Desc)" -ForegroundColor Yellow
    Write-Host "  Waiting for 1 tap..." -ForegroundColor Gray
    
    $proc = New-Object System.Diagnostics.Process
    $proc.StartInfo.FileName = $adbPath
    $proc.StartInfo.Arguments = "-s $($dev.Serial) shell getevent -l"
    $proc.StartInfo.UseShellExecute = $false
    $proc.StartInfo.RedirectStandardOutput = $true
    $proc.StartInfo.CreateNoWindow = $true
    $proc.Start() | Out-Null
    
    $lastX = 0
    $lastY = 0
    $hasX = $false
    $hasY = $false
    
    while (-not ($hasX -and $hasY)) {
        if ($proc.StandardOutput.EndOfStream) { 
            Start-Sleep -Milliseconds 10
            continue 
        }
        $line = $proc.StandardOutput.ReadLine()
        
        if ($line -match "ABS_MT_POSITION_X\s+([0-9a-fA-F]+)") { 
            $lastX = [Convert]::ToInt32($matches[1], 16)
            $hasX = $true 
        }
        if ($line -match "ABS_MT_POSITION_Y\s+([0-9a-fA-F]+)") { 
            $lastY = [Convert]::ToInt32($matches[1], 16)
            $hasY = $true 
        }
    }
    
    $proc.Kill()
    
    $coords[$t.Id] = @{ X = $lastX; Y = $lastY }
    Write-Host "  [LOCKED] $($t.Id) -> X:$lastX Y:$lastY" -ForegroundColor Green
    
    # 1 second buffer so user doesn't accidentally double-tap the next target
    Start-Sleep -Seconds 1
}

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "Final Coordinate Map:" -ForegroundColor Yellow
$coords.GetEnumerator() | Sort-Object Name | Format-Table @{Label="Target"; Expression={$_.Name}}, @{Label="X"; Expression={$_.Value.X}}, @{Label="Y"; Expression={$_.Value.Y}} -AutoSize

$outPath = Join-Path $PSScriptRoot "ui_coordinates.json"
$coords | ConvertTo-Json -Depth 3 | Set-Content $outPath
Write-Host "Mapping dumped to $outPath" -ForegroundColor Green
