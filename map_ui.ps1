# map_ui.ps1 — Tool 2: Interactive UI Coordinate Mapper
# Run after any sprint that changes the layout.
# Walks you through tapping each element and records coordinates via ADB getevent.
# Writes ui_coordinates.json in the expanded schema.
#
# Usage:
#   .\map_ui.ps1              # full calibration (all elements)
#   .\map_ui.ps1 -Only MainTabs   # only remap main tabs
#   .\map_ui.ps1 -Only SubTabs    # only remap sub-tabs
#   .\map_ui.ps1 -Only Actions    # only remap action buttons

param(
    [ValidateSet("All","MainTabs","SubTabs","Actions")]
    [string]$Only = "All"
)

Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force -WarningAction SilentlyContinue

$adbPath = if (Get-Command "adb.exe" -ErrorAction SilentlyContinue) { "adb.exe" } else {
    "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  OPERATOR UI Coordinate Mapper — Tool 2" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  This tool records where each UI element lives on screen." -ForegroundColor Gray
Write-Host "  Run after any sprint that changes the layout." -ForegroundColor Gray
Write-Host "  Takes ~2-3 minutes. Writes ui_coordinates.json." -ForegroundColor Gray
Write-Host ""

$dev = Connect-Device
Write-Host "[OK] Device: $($dev.ToString())" -ForegroundColor Green

# Launch app and wait for it to be active
Write-Host "`nLaunching app..." -ForegroundColor Yellow
$null = Launch-OperatorApp -Device $dev -TimeoutSeconds 8
Write-Host "[OK] App active. Navigate to the starting screen if needed." -ForegroundColor Green
Write-Host ""

# --- Load existing coords to merge into ---
$coordsPath = Join-Path $PSScriptRoot "ui_coordinates.json"
$existing = @{
    main_tabs = @{}
    sub_tabs  = @{}
    actions   = @{}
}
if (Test-Path $coordsPath) {
    $loaded = Get-Content $coordsPath -Raw | ConvertFrom-Json
    # Convert PSCustomObject to hashtable safely
    if ($loaded.main_tabs) { $loaded.main_tabs.PSObject.Properties | ForEach-Object { $existing.main_tabs[$_.Name] = $_.Value } }
    if ($loaded.sub_tabs)  {
        $loaded.sub_tabs.PSObject.Properties | ForEach-Object {
            $tab = $_.Name
            $existing.sub_tabs[$tab] = @{}
            $_.Value.PSObject.Properties | ForEach-Object { $existing.sub_tabs[$tab][$_.Name] = $_.Value }
        }
    }
    if ($loaded.actions)   { $loaded.actions.PSObject.Properties | ForEach-Object { $existing.actions[$_.Name] = $_.Value } }
    Write-Host "[INFO] Loaded existing coords from ui_coordinates.json (merging)" -ForegroundColor DarkGray
}

# --- Helper: tap listener ---
function Read-TapCoordinate {
    param([string]$Serial, [string]$Prompt)

    Write-Host ""
    Write-Host ">>> $Prompt" -ForegroundColor Cyan
    Write-Host "    Navigate your phone so the element is visible." -ForegroundColor Gray
    Write-Host "    Press ENTER here when ready, then tap the element once." -ForegroundColor Gray
    $null = Read-Host

    Write-Host "    [READY] Tap now..." -ForegroundColor Green

    $proc = New-Object System.Diagnostics.Process
    $proc.StartInfo.FileName = $adbPath
    $proc.StartInfo.Arguments = "-s $Serial shell getevent -l"
    $proc.StartInfo.UseShellExecute = $false
    $proc.StartInfo.RedirectStandardOutput = $true
    $proc.StartInfo.CreateNoWindow = $true
    $proc.Start() | Out-Null

    $lastX = 0; $lastY = 0; $hasX = $false; $hasY = $false

    while (-not ($hasX -and $hasY)) {
        if ($proc.StandardOutput.EndOfStream) { Start-Sleep -Milliseconds 10; continue }
        $line = $proc.StandardOutput.ReadLine()
        if ($line -match "ABS_MT_POSITION_X\s+([0-9a-fA-F]+)") { $lastX = [Convert]::ToInt32($matches[1], 16); $hasX = $true }
        if ($line -match "ABS_MT_POSITION_Y\s+([0-9a-fA-F]+)") { $lastY = [Convert]::ToInt32($matches[1], 16); $hasY = $true }
    }
    $proc.Kill()

    Write-Host "    [LOCKED] X:$lastX Y:$lastY" -ForegroundColor Green
    Start-Sleep -Milliseconds 800  # buffer to prevent accidental double-taps
    return @($lastX, $lastY)
}

# ================================================================
# SECTION 1 — Main Tabs
# ================================================================
if ($Only -eq "All" -or $Only -eq "MainTabs") {
    Write-Host "`n--- MAIN TABS (bottom navigation bar) ---" -ForegroundColor Yellow
    Write-Host "Navigate your phone to show the bottom tab bar." -ForegroundColor Gray

    foreach ($tab in @("Roster","Missions","Map","Logs")) {
        $coord = Read-TapCoordinate -Serial $dev.Serial -Prompt "Tap the '$tab' tab in the bottom navigation bar"
        $existing.main_tabs[$tab] = $coord
    }
}

# ================================================================
# SECTION 2 — Sub-tabs (sidebar, changes per main tab)
# ================================================================
if ($Only -eq "All" -or $Only -eq "SubTabs") {
    Write-Host "`n--- SUB-TABS (left sidebar, changes per main tab) ---" -ForegroundColor Yellow

    $subTabDefs = @{
        "Roster"   = @("Collection","Breeding","Recruit","Squad")
        "Missions" = @("Active","Quests")
        "Map"      = @("Zones","Shop")
        "Logs"     = @("Missions","Culture")
    }

    foreach ($tab in $subTabDefs.Keys) {
        Write-Host "`n  [Tap the '$tab' main tab first to make its sidebar visible]" -ForegroundColor DarkGray
        if (-not $existing.sub_tabs[$tab]) { $existing.sub_tabs[$tab] = @{} }

        foreach ($sub in $subTabDefs[$tab]) {
            $coord = Read-TapCoordinate -Serial $dev.Serial -Prompt "[$tab] Tap the '$sub' sub-tab in the LEFT sidebar"
            $existing.sub_tabs[$tab][$sub] = $coord
        }
    }
}

# ================================================================
# SECTION 3 — Action buttons
# ================================================================
if ($Only -eq "All" -or $Only -eq "Actions") {
    Write-Host "`n--- ACTION ELEMENTS ---" -ForegroundColor Yellow

    $actionDefs = @{
        "process_aar"   = "The '⚡ PROCESS AAR' button (visible when a deployment completes)"
        "dismiss_aar"   = "The 'ACKNOWLEDGE & DISMISS' button in the AAR panel"
        "launch_mission"= "The '🚀 LAUNCH MISSION' button in the launch bar"
    }

    foreach ($action in $actionDefs.Keys) {
        $desc = $actionDefs[$action]
        Write-Host "`n  [Navigate so the button is visible: $desc]" -ForegroundColor DarkGray
        $coord = Read-TapCoordinate -Serial $dev.Serial -Prompt "Tap: $desc"
        $existing.actions[$action] = $coord
    }
}

# ================================================================
# Write output
# ================================================================
$outJson = @{
    main_tabs  = $existing.main_tabs
    sub_tabs   = $existing.sub_tabs
    actions    = $existing.actions
    calibrated = (Get-Date -Format "yyyy-MM-ddTHH:mm:ss")
    device     = $dev.ToString()
}

$outJson | ConvertTo-Json -Depth 5 | Set-Content $coordsPath -Encoding UTF8

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  Calibration complete." -ForegroundColor Green
Write-Host "  Written to: $coordsPath" -ForegroundColor Green
Write-Host "  Run .\screenshot.ps1 -Target 'Roster.Breeding' to verify." -ForegroundColor Gray
Write-Host "============================================================" -ForegroundColor Cyan
