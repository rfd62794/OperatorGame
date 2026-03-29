# capture_screenshots.ps1 — Full sweep: navigate to each tab/sub-tab, capture PNG + sidecar JSON
#
# Reads all coordinates from ui_coordinates.json (Fix 2).
# Creates a stub sidecar .json next to each PNG (Fix 4).
# Warns and skips gracefully if a coordinate is missing (never hits dead space silently).
#
# Requires: .\map_ui.ps1 to have been run after the current layout was deployed.

Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force -WarningAction SilentlyContinue

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  OperatorGame Mobile Screenshot Capture" -ForegroundColor Cyan
Write-Host "  Coordinate-driven — reads ui_coordinates.json" -ForegroundColor Cyan
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan

# --- Load coordinate map ---
$coordsPath = Join-Path $PSScriptRoot "ui_coordinates.json"
if (-not (Test-Path $coordsPath)) {
    Write-Error "ui_coordinates.json not found. Run .\map_ui.ps1 first to calibrate coordinates."
    exit 1
}
$coords = Get-Content $coordsPath -Raw | ConvertFrom-Json
Write-Host "[OK] Loaded coordinate map (calibrated: $($coords.calibrated))" -ForegroundColor Green

# --- Connect and launch ---
$dev = Connect-Device
Write-Host "[OK] Connected Target: $($dev.ToString())" -ForegroundColor Green

$pidNum = Launch-OperatorApp -Device $dev -TimeoutSeconds 8
Write-Host "[OK] Architecture online (PID: $pidNum)`n" -ForegroundColor Green

# --- Output directory ---
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$outDir    = Join-Path $PSScriptRoot "screenshots_$timestamp"
New-Item -ItemType Directory -Path $outDir -Force | Out-Null
Write-Host "[OK] Output: $outDir`n" -ForegroundColor Green

# --- Helper: safe tap from coord map ---
function Invoke-SafeTap {
    param($Device, $Coord, [string]$Label, [int]$DelayMs = 900)
    if ($null -eq $Coord) {
        Write-Warning "[$Label] No coordinate in map — skipping tap."
        return $false
    }
    $x = [int]$Coord[0]; $y = [int]$Coord[1]
    Invoke-DeviceTap -Device $Device -X $x -Y $y -DelayMs $DelayMs
    return $true
}

# --- Helper: capture PNG + sidecar JSON ---
function Invoke-Capture {
    param($Device, [string]$Name, [string]$MainTab, [string]$SubTab, [string]$OutDir)

    $pngPath  = Join-Path $OutDir "$Name.png"
    $jsonPath = Join-Path $OutDir "$Name.json"

    $shot = Capture-Screenshot -Device $Device -OutputPath $pngPath -Label $Name

    # Fix 4: Write stub sidecar JSON
    @{
        tab       = $MainTab
        sub_tab   = $SubTab
        name      = $Name
        timestamp = (Get-Date -Format "yyyy-MM-ddTHH:mm:ss")
        device    = $Device.ToString()
        size_kb   = $shot.SizeKb
        hash      = $shot.Hash
    } | ConvertTo-Json | Set-Content $jsonPath -Encoding UTF8

    Write-Host "  [OK] $Name.png ($($shot.SizeKb) KB)" -ForegroundColor Green
    return $shot
}

# ================================================================
# Capture plan — edit this list to add/remove captures
# ================================================================
$plan = @(
    @{ Name="01_Roster_Collection"; Main="Roster"; Sub="Collection" },
    @{ Name="02_Roster_Breeding";   Main="Roster"; Sub="Breeding"   },
    @{ Name="03_Roster_Recruit";    Main="Roster"; Sub="Recruit"    },
    @{ Name="04_Roster_Squad";      Main="Roster"; Sub="Squad"      },
    @{ Name="05_Missions_Active";   Main="Missions"; Sub="Active"   },
    @{ Name="06_Missions_Quests";   Main="Missions"; Sub="Quests"   },
    @{ Name="07_Map_Zones";         Main="Map";    Sub="Zones"      },
    @{ Name="08_Map_Shop";          Main="Map";    Sub="Shop"       },
    @{ Name="09_Logs_Missions";     Main="Logs";   Sub="Missions"   }
)

$currentMain = ""

foreach ($item in $plan) {
    Write-Host "[TAB] $($item.Name)" -ForegroundColor Cyan

    # Navigate to main tab only if it changed
    if ($item.Main -ne $currentMain) {
        $mainCoord = $coords.main_tabs.$($item.Main)
        if (-not (Invoke-SafeTap -Device $dev -Coord $mainCoord -Label "$($item.Main) tab" -DelayMs 1000)) {
            Write-Warning "  Skipping $($item.Name) — main tab coord missing."
            continue
        }
        $currentMain = $item.Main
    }

    # Navigate sub-tab
    $subCoord = $coords.sub_tabs.$($item.Main).$($item.Sub)
    $null = Invoke-SafeTap -Device $dev -Coord $subCoord -Label "$($item.Main).$($item.Sub)" -DelayMs 700

    # Capture
    Invoke-Capture -Device $dev -Name $item.Name -MainTab $item.Main -SubTab $item.Sub -OutDir $outDir | Out-Null
}

Write-Host "`n------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  Sweep complete. $($plan.Count) screens captured." -ForegroundColor Green
Write-Host "  Folder: $outDir" -ForegroundColor Green
Write-Host "  Next:   python scripts\analyze_screenshot_vision.py $outDir" -ForegroundColor Gray
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
