param(
    [string]$Serial = ''
)

Write-Host '------------------------------------------------------------' -ForegroundColor Cyan
Write-Host '  UI Coordinate Finder (Interactive)' -ForegroundColor Cyan
Write-Host '------------------------------------------------------------' -ForegroundColor Cyan

$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

if (-not (Test-Path $ADB)) {
    Write-Error 'ADB not found.'
    exit 1
}

if ($Serial -eq '') {
    & $ADB start-server 2>&1 | Out-Null
    $devices = @(& $ADB devices | Select-Object -Skip 1 | Where-Object { $_ -match 'device' -and $_ -notmatch 'List of' })
    if ($devices.Count -eq 0) {
        Write-Error 'No devices connected.'
        exit 1
    }
    $Serial = ($devices[0] -split '\s+')[0]
}

Write-Host ('[OK] Device locked: ' + $Serial) -ForegroundColor Green

Write-Host "`nVerifying App Installation and Runtime State..." -ForegroundColor Cyan
$pmList = & $ADB -s $Serial shell pm list packages com.rfditservices.operatorgame
if ($pmList -notmatch 'com.rfditservices.operatorgame') {
    Write-Error 'Package com.rfditservices.operatorgame is NOT installed on this device.'
    exit 1
}

$pidRaw = (& $ADB -s $Serial shell pidof com.rfditservices.operatorgame 2>$null)
if ($pidRaw -ne $null) {
    if ($pidRaw.GetType().Name -eq 'Object[]') { $pidRaw = $pidRaw -join ' ' }
    $pidValue = $pidRaw.Trim()
} else {
    $pidValue = ''
}

if ($pidValue -eq '') {
    Write-Host '[WARN] App not running. Launching...' -ForegroundColor Yellow
    & $ADB -s $Serial shell am start -n "com.rfditservices.operatorgame/android.app.NativeActivity"
    Write-Host '  Waiting 8 seconds for game logic to load...' -ForegroundColor Gray
    Start-Sleep -Seconds 8
} else {
    Write-Host ('[OK] App running (PID: ' + $pidValue + ')') -ForegroundColor Green
}

Write-Host "`nInstructions: For each prompt, tap the target UI element on your Moto G." -ForegroundColor Yellow
Write-Host 'If you miss, simply tap again. Only your LAST tap before pressing Enter is saved.' -ForegroundColor Yellow
Write-Host ''

$targets = @(
    'Roster',
    'Missions',
    'Map',
    'Logs',
    'Collection',
    'Breeding',
    'Active',
    'QuestBoard',
    'MissionHistory',
    'CultureHistory'
)

$results = @{}
$tempFile = "$env:TEMP\getevent.log"

foreach ($target in $targets) {
    Write-Host ('>>> TARGET: ' + $target) -ForegroundColor Cyan
    
    if (Test-Path $tempFile) { 
        Remove-Item $tempFile -Force -ErrorAction SilentlyContinue 
    }
    
    # Start capturing raw hex coordinates strictly into a file via background job
    $job = Start-Job -ScriptBlock {
        param($adb, $serial, $tmp)
        cmd.exe /c "$adb -s $serial shell getevent -l > ""$tmp"""
    } -ArgumentList $ADB, $Serial, $tempFile
    
    # Give getevent 500ms to bind to device inputs
    Start-Sleep -Milliseconds 500
    
    Read-Host '  Tap the screen, then press ENTER to confirm'
    
    Stop-Job $job | Out-Null
    Remove-Job $job | Out-Null
    
    # Wait a fraction for file locks from cmd.exe to clear
    Start-Sleep -Milliseconds 200
    
    $lastX = ''
    $lastY = ''
    
    if (Test-Path $tempFile) {
        $lines = Get-Content $tempFile -ErrorAction SilentlyContinue
        if ($lines -ne $null) {
            foreach ($line in $lines) {
                # Moto G and most Androids track multi-touch hex position
                # Absolute positioning check X
                if ($line -match 'ABS_MT_POSITION_X\s+([0-9a-fA-F]+)') {
                    $lastX = [convert]::ToInt32($matches[1], 16).ToString()
                } elseif ($line -match 'ABS_X\s+([0-9a-fA-F]+)') {
                    $lastX = [convert]::ToInt32($matches[1], 16).ToString()
                }
                
                # Absolute positioning check Y
                if ($line -match 'ABS_MT_POSITION_Y\s+([0-9a-fA-F]+)') {
                    $lastY = [convert]::ToInt32($matches[1], 16).ToString()
                } elseif ($line -match 'ABS_Y\s+([0-9a-fA-F]+)') {
                    $lastY = [convert]::ToInt32($matches[1], 16).ToString()
                }
            }
        }
    }
    
    if (($lastX -ne '') -and ($lastY -ne '')) {
        Write-Host ('  [CAPTURED] ' + $target + ' -> X: ' + $lastX + ', Y: ' + $lastY) -ForegroundColor Green
        $results[$target] = ($lastX + ',' + $lastY)
    } else {
        Write-Host '  [WARN] No tap detected. Standard coordinates will be skipped.' -ForegroundColor Yellow
    }
    Write-Host ''
}

Write-Host '------------------------------------------------------------' -ForegroundColor Cyan
Write-Host '  FINAL COORDINATE MAPPING' -ForegroundColor Cyan
Write-Host '------------------------------------------------------------' -ForegroundColor Cyan

Write-Host 'Copy and paste these exact strings into capture_screenshots.ps1:' -ForegroundColor Yellow
Write-Host ''
foreach ($key in $targets) {
    if ($results.ContainsKey($key)) {
        Write-Host ("'" + $key + "' = '" + $results[$key] + "'") -ForegroundColor White
    }
}
Write-Host ''
