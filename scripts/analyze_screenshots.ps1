param(
    [string]$ScreenshotFolder = "screenshots_*",
    [switch]$AutoAnalyze = $false
)

$folder = Get-ChildItem -Path $PSScriptRoot\.. -Filter "screenshots_*" -Directory | Sort-Object LastWriteTime -Descending | Select-Object -First 1

if (-not $folder) {
    Write-Error "No screenshot folder found in repository root."
    exit 1
}

Write-Host "Analyzing screenshots in: $($folder.FullName)" -ForegroundColor Cyan

Get-ChildItem -Path $folder.FullName -Filter "*.png" | ForEach-Object {
    $pngFile = $_
    $jsonPath = $pngFile.FullName -replace '\.png$', '.json'
    
    $metadata = @{
        "screenshot" = @{
            "filename" = $pngFile.Name
            "tab" = $pngFile.BaseName
            "timestamp" = $pngFile.LastWriteTime.ToString("o")
            "device" = "Moto G 2025"
            "resolution_dp" = @{
                "width" = 412
                "height" = 1900
            }
            "file_size_kb" = [math]::Round($pngFile.Length / 1KB, 1)
        }
        "ui_elements" = @{
            "header" = @{ "location" = "top"; "height_dp" = 56; "elements" = @() }
            "content" = @{ "location" = "middle"; "height_dp" = 1818; "elements" = @() }
            "footer" = @{ "location" = "bottom"; "height_dp" = 56; "elements" = @() }
            "subtabs" = @{ "location" = "below_header"; "height_dp" = 40; "elements" = @() }
        }
        "notes" = @{
            "layout_issues" = @()
            "alignment_problems" = @()
            "touch_target_concerns" = @()
        }
    } | ConvertTo-Json -Depth 10
    
    $metadata | Set-Content $jsonPath -Encoding UTF8
    Write-Host "  Generated: $($jsonPath | Split-Path -Leaf)" -ForegroundColor Green
}

Write-Host "`nScreenshot analysis metadata ready for Phase B (OCR/vision processing)." -ForegroundColor Cyan
