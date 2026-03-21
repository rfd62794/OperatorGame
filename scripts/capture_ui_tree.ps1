param(
    [string]$OutputFolder = $null
)

Import-Module "$PSScriptRoot\..\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force

try {
    $Device = Connect-Device
} catch {
    Write-Warning "Device connection pipeline dead. $_"
    exit
}

if (-not $OutputFolder) {
    $OutputFolder = Join-Path $PSScriptRoot "..\screenshots_uitree_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
}

New-Item -ItemType Directory -Path $OutputFolder -Force | Out-Null

Write-Host "════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Automated UI Tree Navigation & Screenshot Collection" -ForegroundColor Cyan
Write-Host "════════════════════════════════════════════════════════════" -ForegroundColor Cyan

Write-Host "`n[Setup] Launching OperatorGame..." -ForegroundColor Yellow
$pidNum = Launch-OperatorApp -Device $Device -KillIfRunning
Start-Sleep -Seconds 3 # Let app bootstrap

$coordFile = Join-Path $PSScriptRoot "..\ui_coordinates.json"
if (-not (Test-Path $coordFile)) {
    Write-Error "ui_coordinates.json not found. Run .\find_ui_coordinates.ps1 first."
    exit 1
}

$coords = Get-Content $coordFile | ConvertFrom-Json
Write-Host "[Setup] Loaded coordinates from ui_coordinates.json" -ForegroundColor Green

# Note: Sub-tabs on Moto G via Android display (Screen coords: 1080x2400 roughly).
# getevent output was X: ~450, Y: ~5800 which is digitizer scale.
# If Invoke-DeviceTap limits to 5000 we scale, but let's trust ADB input tap.
# Custom X/Y for subtabs are rough screen ratios since subtabs are on the left.
# Moto G physical density is 2.0. SubTab 1 is ~120dp (240px). SubTab 2 is ~165dp (330px).
$subX = 80
$subTab1Y = 240 
$subTab2Y = 330 

$uiTree = @(
    @{
        name = "Roster_Collection"
        description = "Roster → Collection (default)"
        navigation = @(
            @{ action = "tap"; target = "Roster"; delay = 1000 }
            @{ action = "tap"; x = $subX; y = $subTab1Y; delay = 600 }
        )
        filename = "01_roster_collection.png"
    },
    @{
        name = "Roster_Breeding"
        description = "Roster → Breeding"
        navigation = @(
            @{ action = "tap"; target = "Roster"; delay = 1000 }
            @{ action = "tap"; x = $subX; y = $subTab2Y; delay = 600 }
        )
        filename = "02_roster_breeding.png"
    },
    @{
        name = "Missions_Active"
        description = "Missions → Active (default)"
        navigation = @(
            @{ action = "tap"; target = "Missions"; delay = 1000 }
            @{ action = "tap"; x = $subX; y = $subTab1Y; delay = 600 }
        )
        filename = "03_missions_active.png"
    },
    @{
        name = "Missions_QuestBoard"
        description = "Missions → Quest Board"
        navigation = @(
            @{ action = "tap"; target = "Missions"; delay = 1000 }
            @{ action = "tap"; x = $subX; y = $subTab2Y; delay = 600 }
        )
        filename = "04_missions_questboard.png"
    },
    @{
        name = "Map_Zones"
        description = "Map → Zones (default)"
        navigation = @(
            @{ action = "tap"; target = "Map"; delay = 1000 }
            @{ action = "tap"; x = $subX; y = $subTab1Y; delay = 600 }
        )
        filename = "05_map_zones.png"
    },
    @{
        name = "Logs_History"
        description = "Logs → Mission History (default)"
        navigation = @(
            @{ action = "tap"; target = "Logs"; delay = 1000 }
            @{ action = "tap"; x = $subX; y = $subTab1Y; delay = 600 }
        )
        filename = "06_logs_history.png"
    },
    @{
        name = "Logs_Culture"
        description = "Logs → Culture History"
        navigation = @(
            @{ action = "tap"; target = "Logs"; delay = 1000 }
            @{ action = "tap"; x = $subX; y = $subTab2Y; delay = 600 }
        )
        filename = "07_logs_culture.png"
    }
)

$capturedCount = 0
foreach ($state in $uiTree) {
    Write-Host "`n[State] $($state.description)" -ForegroundColor Cyan
    Write-Host "  Navigation steps: $($state.navigation.Count)" -ForegroundColor Gray
    
    foreach ($step in $state.navigation) {
        if ($step.action -eq "tap") {
            if ($step.target) {
                # Statically mapped to Moto G 720x1604 physical bounds:
                # Android OS Nav Bar takes bottom ~100px. Egui bottom bar sits safely above it (Y=1450).
                # 4 tabs evenly spaced across 720px width:
                switch ($step.target) {
                    "Roster"   { $tapX = 90;  $tapY = 1450 }
                    "Missions" { $tapX = 270; $tapY = 1450 }
                    "Map"      { $tapX = 450; $tapY = 1450 }
                    "Logs"     { $tapX = 630; $tapY = 1450 }
                    default    { $tapX = 360; $tapY = 800  } # Center fallback
                }

                Write-Host "    → Tap: $($step.target) natively locked at Screen($tapX, $tapY)" -ForegroundColor Gray
                try {
                    Invoke-DeviceTap -Device $Device -X $tapX -Y $tapY -DelayMs 300
                } catch {
                     Write-Warning "Failed tap: $_."
                }
            } elseif ($step.x -and $step.y) {
                Write-Host "    → Tap: Custom coords ($($step.x), $($step.y))" -ForegroundColor Gray
                Invoke-DeviceTap -Device $Device -X $step.x -Y $step.y -DelayMs 300
            }
            if ($step.delay) { Start-Sleep -Milliseconds $step.delay }
        }
    }
    
    Write-Host "  Capturing screenshot..." -ForegroundColor Yellow
    $outPath = Join-Path $OutputFolder $state.filename
    try {
        $shot = Capture-Screenshot -Device $Device -OutputPath $outPath -Label $state.name
        Write-Host "  ✅ Captured: $($shot.FilePath) ($($shot.SizeKb) KB)" -ForegroundColor Green
        $capturedCount++
    } catch {
        Write-Host "  ❌ Capture failed: $_" -ForegroundColor Red
    }
}

Write-Host "`n════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "  UI Tree Exploration Complete" -ForegroundColor Green
Write-Host "════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "Total screenshots captured: $capturedCount / $($uiTree.Count)" -ForegroundColor Cyan

$manifest = @{
    timestamp = Get-Date -Format "o"
    device = $Device.Serial
    total_states = $uiTree.Count
    captured = $capturedCount
    output_folder = $OutputFolder
    states = @()
}

foreach ($state in $uiTree) {
    $manifest.states += @{
        name = $state.name
        description = $state.description
        filename = $state.filename
        filepath = Join-Path $OutputFolder $state.filename
    }
}

$manifestPath = Join-Path $OutputFolder "manifest.json"
$manifest | ConvertTo-Json -Depth 10 | Set-Content $manifestPath -Encoding UTF8
Write-Host "Manifest saved: $manifestPath" -ForegroundColor Green
