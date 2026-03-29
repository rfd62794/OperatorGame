# screenshot.ps1 — Tool 1: Single Screenshot
# Captures whatever is currently on screen. No navigation, no coordinates needed.
#
# Usage:
#   .\screenshot.ps1                        # saves to screenshots/single_[timestamp].png
#   .\screenshot.ps1 -Label "roster_bug"   # adds a label to the filename
#   .\screenshot.ps1 -Target "Roster.Breeding"  # navigate then capture (requires Tool 2 coords)
#   .\screenshot.ps1 -Open                 # opens the PNG in the default viewer after capture

param(
    [string]$Label     = "",
    [string]$Target    = "",   # Optional: "Tab.SubTab" from ui_coordinates.json
    [switch]$Open              # Open in default viewer after capture
)

Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force -WarningAction SilentlyContinue

$dev = Connect-Device

# --- Optional: Navigate to a named target before capturing ---
if ($Target -ne "") {
    $coordsPath = Join-Path $PSScriptRoot "ui_coordinates.json"
    if (-not (Test-Path $coordsPath)) {
        Write-Warning "ui_coordinates.json not found. Run .\map_ui.ps1 first to calibrate coordinates."
        Write-Warning "Capturing current screen state instead."
    } else {
        $coords = Get-Content $coordsPath -Raw | ConvertFrom-Json

        # Parse "MainTab.SubTab" or just "MainTab"
        $parts = $Target.Split(".")
        $mainTab  = $parts[0]
        $subTab   = if ($parts.Length -gt 1) { $parts[1] } else { $null }

        # Tap main tab
        $mainCoord = $coords.main_tabs.$mainTab
        if ($mainCoord) {
            Write-Host "  [NAV] Tapping main tab: $mainTab" -ForegroundColor DarkGray
            Invoke-DeviceTap -Device $dev -X $mainCoord[0] -Y $mainCoord[1] -DelayMs 800
        } else {
            Write-Warning "Unknown main tab '$mainTab' in ui_coordinates.json"
        }

        # Tap sub-tab if specified
        if ($subTab) {
            $subCoord = $coords.sub_tabs.$mainTab.$subTab
            if ($subCoord) {
                Write-Host "  [NAV] Tapping sub-tab: $subTab" -ForegroundColor DarkGray
                Invoke-DeviceTap -Device $dev -X $subCoord[0] -Y $subCoord[1] -DelayMs 600
            } else {
                Write-Warning "Unknown sub-tab '$subTab' under '$mainTab' in ui_coordinates.json"
            }
        }

        # Small settle delay after navigation
        Start-Sleep -Milliseconds 400
    }
}

# --- Capture ---
$timestamp  = Get-Date -Format "yyyyMMdd_HHmmss"
$safeName   = if ($Label -ne "") { "_${Label}" } else { "" }
$targetSlug = if ($Target -ne "") { "_$($Target.Replace('.','_'))" } else { "" }
$filename   = "single${targetSlug}${safeName}_${timestamp}.png"
$outDir     = Join-Path $PSScriptRoot "screenshots"
$outPath    = Join-Path $outDir $filename

New-Item -ItemType Directory -Path $outDir -Force | Out-Null

$shot = Capture-Screenshot -Device $dev -OutputPath $outPath -Label ($Target + $Label)

Write-Host ""
Write-Host "✅ Captured: $($shot.FilePath)" -ForegroundColor Green
Write-Host "   Size:     $($shot.SizeKb) KB" -ForegroundColor Gray
Write-Host "   Hash:     $($shot.Hash)" -ForegroundColor Gray

if ($Open) {
    Start-Process $shot.FilePath
}

return $shot
