param(
    [string]$Serial = $null
)

Write-Host "UI Coordinate Finder - Touch to identify coordinates" -ForegroundColor Cyan
Write-Host "Instructions:" -ForegroundColor Yellow
Write-Host "  1. When prompted, touch the UI element on the Moto G screen"
Write-Host "  2. The script will capture the touch coordinates"
Write-Host "  3. Note the coordinates and update capture_screenshots.ps1"
Write-Host ""

$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

if (-not (Test-Path $ADB)) {
    Write-Error "ADB not found."
    exit 1
}

# Auto-detect device
if (-not $Serial) {
    & $ADB start-server 2>&1 | Out-Null
    $devices = & $ADB devices | Select-Object -Skip 1 | Where-Object { $_.Trim() -and ($_ -notmatch "List of") }
    
    if ($devices.Count -eq 0) {
        Write-Error "No devices connected."
        exit 1
    }

    $Serial = ($devices[0] -split '\s+')[0]
}

Write-Host "Device: $Serial" -ForegroundColor Green
Write-Host ""

# Start getevent on device and monitor for touch events
Write-Host "Ready. Touch the Moto G screen now..." -ForegroundColor Cyan
Write-Host "(Press Ctrl+C to stop)" -ForegroundColor Gray
Write-Host ""

& $ADB -s $Serial shell getevent /dev/input/event* | ForEach-Object {
    if ($_ -match "ABS_MT_POSITION_X|ABS_MT_POSITION_Y|BTN_TOUCH") {
        Write-Host $_
    }
}
