$folder = Get-ChildItem -Path $PSScriptRoot\.. -Filter "screenshots_*" -Directory | Sort-Object LastWriteTime -Descending | Select-Object -First 1

if (-not $folder) {
    Write-Error "No screenshots folder found."
    exit 1
}

$jsons = Get-ChildItem -Path $folder.FullName -Filter "*.json" | Where-Object { $_.Name -notmatch "_analysis\.json" -and $_.Name -ne "manifest.json" }

if (-not $jsons) {
    Write-Warning "No metadata JSON files found. Run analyze_screenshots.ps1 first."
    exit
}

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  Manual UI Annotation Pipeline" -ForegroundColor Cyan
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan

foreach ($j in $jsons) {
    Write-Host "`n>>> Opening $($j.Name) for manual annotation..." -ForegroundColor Yellow
    
    # Attempt to open in VSCode, fallback to notepad
    if (Get-Command "code" -ErrorAction SilentlyContinue) {
        code $j.FullName
    } else {
        notepad $j.FullName
    }
    
    Write-Host "Add specific egui element maps, layout bounds, and structural notes." -ForegroundColor Gray
    Read-Host "Press [Enter] when finished with this tab to proceed to the next"
}

Write-Host "`nAll tabs annotated. Ready for AI Vision inference!" -ForegroundColor Green
