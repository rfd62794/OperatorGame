param(
    [int]$Cycles = 3,
    [switch]$SkipBuild = $false
)

for ($i = 1; $i -le $Cycles; $i++) {
    Write-Host "`n╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║  UI Iteration Cycle $i/$Cycles" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    
    if (-not $SkipBuild) {
        Write-Host "[1/5] Building APK..." -ForegroundColor Yellow
        & "$PSScriptRoot\..\build_android.ps1"
    }
    
    Write-Host "[2/5] Deploying to Moto G..." -ForegroundColor Yellow
    & "$PSScriptRoot\..\deploy_moto.ps1"
    
    Write-Host "[3/5] Capturing screenshots..." -ForegroundColor Yellow
    & "$PSScriptRoot\..\capture_screenshots.ps1"
    
    Write-Host "[4/5] Generating Annotations & Vision Parsing..." -ForegroundColor Yellow
    & "$PSScriptRoot\analyze_screenshots.ps1"
    
    $latestFolder = Get-ChildItem -Path $PSScriptRoot\.. -Filter "screenshots_*" -Directory | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    
    Write-Host "Running Claude Vision Inference on: $($latestFolder.Name)"
    & python.exe "$PSScriptRoot\analyze_screenshot_vision.py" $latestFolder.FullName
    
    Write-Host "[5/5] Synthesizing Markdown Report..." -ForegroundColor Yellow
    & "$PSScriptRoot\generate_ui_report.ps1"
    
    Write-Host "`nCycle $i complete. Review 'UI_FEEDBACK_REPORT.md'" -ForegroundColor Green
    if ($i -lt $Cycles) {
        Write-Host "Press Enter to continue to next cycle or Ctrl+C to stop..." -ForegroundColor Gray
        Read-Host
    }
}

Write-Host "`n✅ UI iteration complete. All screenshots analyzed." -ForegroundColor Green
