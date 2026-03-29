# map_ui.ps1 — Tool 2: Interactive UI Coordinate Mapper
# Run after any sprint that changes the layout.
# Walks you through tapping each element and records coordinates via ADB getevent.
# Writes ui_coordinates.json in the expanded schema.

param(
    [ValidateSet("All","MainTabs","SubTabs","Actions")]
    [string]$Only = "All"
)

Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force -WarningAction SilentlyContinue

$adbPath = if (Get-Command "adb.exe" -ErrorAction SilentlyContinue) { "adb.exe" } else {
    "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
}

# --- Helper: tap listener defined BEFORE use ---
function Read-TapCoordinate {
    param([string]$Serial, [string]$Prompt)

    Write-Host ""
    Write-Host ("--- " + $Prompt) -ForegroundColor Cyan
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
        if ($line -match 'ABS_MT_POSITION_X\s+([0-9a-fA-F]+)') { $lastX = [Convert]::ToInt32($matches[1], 16); $hasX = $true }
        if ($line -match 'ABS_MT_POSITION_Y\s+([0-9a-fA-F]+)') { $lastY = [Convert]::ToInt32($matches[1], 16); $hasY = $true }
    }
    $proc.Kill()

    Write-Host "    [LOCKED] X:$lastX Y:$lastY" -ForegroundColor Green
    Start-Sleep -Milliseconds 800
    return @($lastX, $lastY)
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  OPERATOR UI Coordinate Mapper" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

$dev = Connect-Device
if ($null -eq $dev) {
    Write-Error "No device connected. Ensure your Moto G is plugged in and ADB is authorized."
    exit 1
}
Write-Host "[OK] Device: $($dev.ToString())" -ForegroundColor Green

# Launch app
Write-Host "`nLaunching app..." -ForegroundColor Yellow
$null = Launch-OperatorApp -Device $dev -TimeoutSeconds 8
Write-Host "[OK] App active." -ForegroundColor Green

# --- Load existing coords to merge into ---
$coordsPath = Join-Path $PSScriptRoot "ui_coordinates.json"
$existing = @{
    main_tabs = @{}
    sub_tabs  = @{}
    actions   = @{}
}
if (Test-Path $coordsPath) {
    try {
        $jsonStr = Get-Content $coordsPath -Raw
        if ($jsonStr) {
            $loaded = $jsonStr | ConvertFrom-Json
            if ($loaded.main_tabs) { $loaded.main_tabs.PSObject.Properties | ForEach-Object { $existing.main_tabs[$_.Name] = $_.Value } }
            if ($loaded.sub_tabs)  {
                $loaded.sub_tabs.PSObject.Properties | ForEach-Object {
                    $tab = $_.Name
                    $existing.sub_tabs[$tab] = @{}
                    $_.Value.PSObject.Properties | ForEach-Object { $existing.sub_tabs[$tab][$_.Name] = $_.Value }
                }
            }
            if ($loaded.actions)   { $loaded.actions.PSObject.Properties | ForEach-Object { $existing.actions[$_.Name] = $_.Value } }
            Write-Host "[INFO] Loaded existing coords (merging)" -ForegroundColor DarkGray
        }
    } catch {
        Write-Warning "Could not parse ui_coordinates.json. Starting fresh."
    }
}

# ================================================================
# SECTION 1 — Main Tabs
# ================================================================
if ($Only -eq "All" -or $Only -eq "MainTabs") {
    Write-Host "`n--- MAIN TABS ---" -ForegroundColor Yellow
    foreach ($tab in @("Roster","Missions","Map","Logs")) {
        $coord = Read-TapCoordinate -Serial $dev.Serial -Prompt "Tap the '$tab' tab"
        $existing.main_tabs[$tab] = $coord
    }
}

# ================================================================
# SECTION 2 — Sub-tabs
# ================================================================
if ($Only -eq "All" -or $Only -eq "SubTabs") {
    Write-Host "`n--- SUB-TABS ---" -ForegroundColor Yellow

    $subTabDefs = @{
        "Roster"   = @("Collection","Breeding","Recruit","Squad")
        "Missions" = @("Active","Quests")
        "Map"      = @("Zones","Shop")
        "Logs"     = @("Missions","Culture")
    }

    foreach ($tab in $subTabDefs.Keys) {
        Write-Host "`n[Navigate to $tab]" -ForegroundColor DarkGray
        if (-not $existing.sub_tabs[$tab]) { $existing.sub_tabs[$tab] = @{} }

        foreach ($sub in $subTabDefs[$tab]) {
            $coord = Read-TapCoordinate -Serial $dev.Serial -Prompt "[$tab] Tap '$sub'"
            $existing.sub_tabs[$tab][$sub] = $coord
        }
    }
}

# ================================================================
# SECTION 3 — Action buttons
# ================================================================
if ($Only -eq "All" -or $Only -eq "Actions") {
    Write-Host "`n--- ACTIONS ---" -ForegroundColor Yellow

    $actionDefs = @{
        "process_aar"   = "The 'PROCESS AAR' button"
        "dismiss_aar"   = "The 'DISMISS' button"
        "launch_mission"= "The 'LAUNCH MISSION' button"
    }

    foreach ($action in $actionDefs.Keys) {
        $coord = Read-TapCoordinate -Serial $dev.Serial -Prompt "Tap: $($actionDefs[$action])"
        $existing.actions[$action] = $coord
    }
}

# ================================================================
# Write output
# ================================================================
$outObj = @{
    main_tabs  = $existing.main_tabs
    sub_tabs   = $existing.sub_tabs
    actions    = $existing.actions
    calibrated = (Get-Date -Format "yyyy-MM-ddTHH:mm:ss")
    device     = $dev.ToString()
}

$outObj | ConvertTo-Json -Depth 5 | Set-Content $coordsPath -Encoding UTF8

Write-Host "`nCalibration complete." -ForegroundColor Green
Write-Host "Written to: $coordsPath" -ForegroundColor Green
