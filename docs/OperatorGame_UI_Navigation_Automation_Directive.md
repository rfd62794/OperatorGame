# Phase F.2: Automated UI Navigation & Comprehensive Screenshot Collection

**Status:** Pre-Analysis  
**Target:** Systematically navigate every UI tab and sub-tab, capture screenshots at each state  
**Scope:** Automation-driven exploration (no manual intervention after launch)  
**Output:** Complete screenshot library of all UI states

---

## Problem Statement

Current approach: Manual tab selection → capture 6 screenshots.

**Issue:** We're only capturing the **default state** of each tab. We miss:
- Sub-tab variations (Roster Collection vs Roster Breeding)
- Different screen states (empty lists, full lists, error states)
- Transitions and overlays
- Combat log expanded/collapsed
- Different squad compositions

**Solution:** Script that automatically navigates through **every UI branch** and captures at each node.

---

## Part A: Map the Full UI Navigation Tree

**Agent Action:** From `UI_COMPONENT_INVENTORY.md`, extract all states:

```
BottomTab: Roster
├── SubTab: Collection (default)
│   └── Capture: roster_collection_default.png
├── SubTab: Breeding
│   └── Capture: roster_breeding_default.png

BottomTab: Missions
├── SubTab: Active (default)
│   └── Capture: missions_active_default.png
├── SubTab: QuestBoard
│   └── Capture: missions_questboard_default.png

BottomTab: Map
├── SubTab: Zones (default)
│   └── Capture: map_zones_default.png

BottomTab: Logs
├── SubTab: MissionHistory (default)
│   └── Capture: logs_history_default.png
├── SubTab: CultureHistory
│   └── Capture: logs_culture_default.png

Global States:
├── CombatLog: Expanded
│   └── Capture: global_combatlog_expanded.png
├── CombatLog: Collapsed
│   └── Capture: global_combatlog_collapsed.png
```

**Total states to capture:** ~10-12 unique UI configurations

---

## Part B: Create Navigation Automation Script

**Script:** `C:\Github\OperatorGame\scripts\capture_ui_tree.ps1`

**Purpose:** Automatically navigate and capture every UI state

```powershell
param(
    [Device]$Device = $null,
    [string]$OutputFolder = $null
)

# Import module
Import-Module "$PSScriptRoot\..\lib\OperatorDeviceTools\OperatorDeviceTools.psm1"

if (-not $Device) {
    $Device = Connect-Device
}

if (-not $OutputFolder) {
    $OutputFolder = Join-Path $PSScriptRoot "..\screenshots_uitree_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
}

New-Item -ItemType Directory -Path $OutputFolder -Force | Out-Null

Write-Host "════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Automated UI Tree Navigation & Screenshot Collection" -ForegroundColor Cyan
Write-Host "════════════════════════════════════════════════════════════" -ForegroundColor Cyan

# Ensure app is running
Write-Host "`n[Setup] Launching OperatorGame..." -ForegroundColor Yellow
$pid = Launch-OperatorApp -Device $Device -KillIfRunning

# Load coordinates from ui_coordinates.json
$coordFile = Join-Path $PSScriptRoot "..\ui_coordinates.json"
if (-not (Test-Path $coordFile)) {
    Write-Error "ui_coordinates.json not found. Run .\find_ui_coordinates.ps1 first."
    exit 1
}

$coords = Get-Content $coordFile | ConvertFrom-Json

Write-Host "[Setup] Loaded coordinates from ui_coordinates.json" -ForegroundColor Green

# Define UI navigation tree
$uiTree = @(
    @{
        name = "Roster_Collection"
        description = "Roster → Collection (default)"
        navigation = @(
            @{ action = "tap"; target = "Roster"; delay = 800 }
        )
        filename = "01_roster_collection.png"
    },
    @{
        name = "Roster_Breeding"
        description = "Roster → Breeding"
        navigation = @(
            @{ action = "tap"; target = "Roster"; delay = 800 }
            @{ action = "tap"; target = "SubTab_Right"; x = 250; y = 300; delay = 800 }
        )
        filename = "02_roster_breeding.png"
    },
    @{
        name = "Missions_Active"
        description = "Missions → Active (default)"
        navigation = @(
            @{ action = "tap"; target = "Missions"; delay = 800 }
        )
        filename = "03_missions_active.png"
    },
    @{
        name = "Missions_QuestBoard"
        description = "Missions → Quest Board"
        navigation = @(
            @{ action = "tap"; target = "Missions"; delay = 800 }
            @{ action = "tap"; target = "SubTab_Right"; x = 250; y = 300; delay = 800 }
        )
        filename = "04_missions_questboard.png"
    },
    @{
        name = "Map_Zones"
        description = "Map → Zones (default)"
        navigation = @(
            @{ action = "tap"; target = "Map"; delay = 800 }
        )
        filename = "05_map_zones.png"
    },
    @{
        name = "Logs_History"
        description = "Logs → Mission History (default)"
        navigation = @(
            @{ action = "tap"; target = "Logs"; delay = 800 }
        )
        filename = "06_logs_history.png"
    },
    @{
        name = "Logs_Culture"
        description = "Logs → Culture History"
        navigation = @(
            @{ action = "tap"; target = "Logs"; delay = 800 }
            @{ action = "tap"; target = "SubTab_Right"; x = 250; y = 300; delay = 800 }
        )
        filename = "07_logs_culture.png"
    }
)

# Execute navigation tree
$capturedCount = 0
foreach ($state in $uiTree) {
    Write-Host "`n[State] $($state.description)" -ForegroundColor Cyan
    Write-Host "  Navigation steps: $($state.navigation.Count)" -ForegroundColor Gray
    
    # Execute navigation steps
    foreach ($step in $state.navigation) {
        switch ($step.action) {
            "tap" {
                if ($step.target -and $coords.$($step.target)) {
                    $tapCoord = $coords.$($step.target)
                    Write-Host "    → Tap: $($step.target) at ($($tapCoord.X), $($tapCoord.Y))" -ForegroundColor Gray
                    Invoke-DeviceTap -Device $Device -X $tapCoord.X -Y $tapCoord.Y -DelayMs 300
                } elseif ($step.x -and $step.y) {
                    Write-Host "    → Tap: Custom coords ($($step.x), $($step.y))" -ForegroundColor Gray
                    Invoke-DeviceTap -Device $Device -X $step.x -Y $step.y -DelayMs 300
                }
                
                # Wait for navigation
                if ($step.delay) {
                    Start-Sleep -Milliseconds $step.delay
                }
            }
        }
    }
    
    # Capture screenshot
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
Write-Host "Output folder: $OutputFolder" -ForegroundColor Cyan

# Generate manifest
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
$manifest | ConvertTo-Json -Depth 10 | Set-Content $manifestPath
Write-Host "Manifest saved: $manifestPath" -ForegroundColor Green
```

---

## Part C: Execution Flow

**Agent Action:**

```powershell
cd C:\Github\OperatorGame

# Ensure latest build
Write-Host "Building latest APK..." -ForegroundColor Yellow
& .\build_android.ps1

# Deploy to device
Write-Host "Deploying to Moto G..." -ForegroundColor Yellow
& .\deploy_moto.ps1

# Run automated UI tree capture
Write-Host "Starting automated UI exploration..." -ForegroundColor Yellow
& .\scripts\capture_ui_tree.ps1
```

**What happens:**
1. Builds APK (if needed)
2. Deploys to Moto G
3. Launches app
4. Navigates: Roster → Collection (capture) → Breeding (capture)
5. Navigates: Missions → Active (capture) → QuestBoard (capture)
6. Navigates: Map → Zones (capture)
7. Navigates: Logs → History (capture) → Culture (capture)
8. **Total:** ~7-10 captures of different UI states
9. Generates `manifest.json` linking all screenshots

**Output folder:** `screenshots_uitree_YYYYMMDD_HHMMSS/` with:
```
manifest.json
01_roster_collection.png
02_roster_breeding.png
03_missions_active.png
04_missions_questboard.png
05_map_zones.png
06_logs_history.png
07_logs_culture.png
```

---

## Part D: Extend Navigation Tree (Advanced)

Once basic tree works, add additional states:

```powershell
# Optional: Scroll states (empty list, full list, partial)
@{
    name = "Roster_ScrollDown"
    description = "Roster → Collection → Scroll down"
    navigation = @(
        @{ action = "tap"; target = "Roster"; delay = 800 }
        @{ action = "scroll"; direction = "down"; amount = 5; delay = 500 }
    )
    filename = "01_roster_collection_scrolled.png"
}

# Optional: Overlay states (combat log expanded)
@{
    name = "Global_CombatLogExpanded"
    description = "Combat Log panel expanded"
    navigation = @(
        @{ action = "tap"; target = "CombatLog_Expand"; x = 200; y = 1600; delay = 800 }
    )
    filename = "00_combatlog_expanded.png"
}
```

**Add these to `$uiTree` array as you discover new states to test.**

---

## Success Criteria

✅ Script launches app automatically  
✅ Script navigates to each UI state using calibrated coordinates  
✅ Script captures screenshot at each state  
✅ All captures succeed (0 failures)  
✅ `manifest.json` lists all states and filenames  
✅ Screenshot folder contains complete UI tree  

---

## Next Phase (After Successful Capture)

Once all screenshots are collected:

1. **Generate metadata JSON** for each screenshot (current empty template)
2. **Robert annotates visually** (what you see in each screenshot)
3. **Pixel analysis pipeline** extracts text coordinates, color regions
4. **Vision API** (optional) provides semantic feedback
5. **Generate UI_FEEDBACK_REPORT.md** with cross-state analysis

---

## Ready to Execute?

**Agent:** Run `capture_ui_tree.ps1` to collect complete UI screenshot library.

**Robert:** Confirm once all screenshots are captured and manifest is valid.

Then we proceed to **Part D: Screenshot Analysis & Pixel Checking**.
