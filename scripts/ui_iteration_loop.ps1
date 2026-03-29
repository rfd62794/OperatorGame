# ui_iteration_loop.ps1 — Full UI capture + analysis pipeline
#
# Steps:
#   1. Build APK (skippable)
#   2. Deploy to Moto G
#   3. Capture screenshots (reads ui_coordinates.json)
#   4. Analyze with Claude vision (analyze_screenshot_vision.py)
#   5. Print report path
#
# Usage:
#   .\scripts\ui_iteration_loop.ps1              # full loop, 1 cycle
#   .\scripts\ui_iteration_loop.ps1 -Cycles 3   # 3 cycles with pause between each
#   .\scripts\ui_iteration_loop.ps1 -SkipBuild  # skip cargo apk, deploy existing APK

param(
    [int]$Cycles    = 1,
    [switch]$SkipBuild
)

# Check ui_coordinates.json is calibrated before starting
$coordsPath = Join-Path $PSScriptRoot ".." "ui_coordinates.json"
if (Test-Path $coordsPath) {
    $check = Get-Content $coordsPath -Raw | ConvertFrom-Json
    if ($check.calibrated -eq "UNCALIBRATED") {
        Write-Warning "ui_coordinates.json has not been calibrated."
        Write-Warning "Run .\map_ui.ps1 first, then re-run this loop."
        exit 1
    }
} else {
    Write-Warning "ui_coordinates.json missing. Run .\map_ui.ps1 first."
    exit 1
}

for ($i = 1; $i -le $Cycles; $i++) {
    Write-Host "`n╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║  UI Iteration Cycle $i/$Cycles" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

    if (-not $SkipBuild) {
        Write-Host "[1/4] Building APK..." -ForegroundColor Yellow
        & "$PSScriptRoot\..\build_android.ps1"
    } else {
        Write-Host "[1/4] Build skipped (-SkipBuild)" -ForegroundColor DarkGray
    }

    Write-Host "[2/4] Deploying to Moto G..." -ForegroundColor Yellow
    & "$PSScriptRoot\..\deploy_moto.ps1"

    Write-Host "[3/4] Capturing screenshots..." -ForegroundColor Yellow
    & "$PSScriptRoot\..\capture_screenshots.ps1"

    $latestFolder = Get-ChildItem -Path "$PSScriptRoot\.." -Filter "screenshots_[0-9]*" -Directory |
        Sort-Object LastWriteTime -Descending |
        Select-Object -First 1

    if (-not $latestFolder) {
        Write-Error "No screenshot folder found after capture. Aborting analysis."
        continue
    }

    Write-Host "[4/4] Running Claude vision analysis on: $($latestFolder.Name)" -ForegroundColor Yellow
    & python.exe "$PSScriptRoot\analyze_screenshot_vision.py" $latestFolder.FullName

    $reportPath = Join-Path $latestFolder.FullName "UI_FEEDBACK_REPORT.md"
    if (Test-Path $reportPath) {
        Write-Host "`n✅ Report ready: $reportPath" -ForegroundColor Green
    } else {
        Write-Warning "Report not found — check Python output above for errors."
    }

    Write-Host "`nCycle $i complete." -ForegroundColor Green
    if ($i -lt $Cycles) {
        Write-Host "Press Enter to continue to cycle $($i+1)/($Cycles), or Ctrl+C to stop..." -ForegroundColor Gray
        $null = Read-Host
    }
}

Write-Host "`n✅ UI iteration complete." -ForegroundColor Cyan
