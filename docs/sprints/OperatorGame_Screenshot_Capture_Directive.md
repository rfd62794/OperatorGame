# OperatorGame — Mobile Screenshot Capture Directive

**Target:** Coding Agent (Gemini/Antigravity)  
**Goal:** Systematically capture screenshots of each UI tab on running Moto G  
**Scope:** Automated ADB screenshot capture, organized output, full tab coverage  
**Output:** Screenshots for each UI state (Roster Collection, Breeding, Missions Active, Quest Board, Map, Logs, etc.)  

---

## Context

**Current State:**
- App runs on Moto G but has UI overlay issues, alignment issues, and some systems aren't accessible
- Manual visual inspection on device is slow and hard to compare across tabs
- Need systematic visual audit before debugging specific issues

**Goal:**
Create a script that:
1. Ensures app is running on Moto G
2. Navigates to each UI tab programmatically (or via input simulation)
3. Captures a screenshot of each tab
4. Saves screenshots with clear naming (Roster_Collection.png, Missions_Active.png, etc.)
5. Organizes them in a timestamped folder for easy review

---

## Part A: Screenshot Capture Script

### Task A.1: Create `capture_screenshots.ps1`

**File:** `capture_screenshots.ps1` (repo root)

**Purpose:** Automate screenshot capture for all UI tabs on Moto G.

**Requirements:**

```powershell
param(
    [string]$Serial = $null,
    [string]$OutputDir = "screenshots_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
)

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  OperatorGame Mobile Screenshot Capture                   ║" -ForegroundColor Cyan
Write-Host "║  Automated UI tab screenshots from Moto G                 ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

if (-not (Test-Path $ADB)) {
    Write-Error "ADB not found. Run setup_local_forge.ps1 first."
    exit 1
}

# Auto-detect device if not specified
if (-not $Serial) {
    & $ADB start-server 2>&1 | Out-Null
    $devices = & $ADB devices | Select-Object -Skip 1 | Where-Object { $_.Trim() -and $_ -notmatch "List of" }
    
    if ($devices.Count -eq 0) {
        Write-Error "No devices connected."
        exit 1
    }
    
    $Serial = ($devices[0] -split '\s+')[0]
    Write-Host "✓ Auto-detected device: $Serial" -ForegroundColor Green
} else {
    Write-Host "✓ Using device: $Serial" -ForegroundColor Green
}

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

Write-Host "✓ Output directory: $OutputDir" -ForegroundColor Green

# Verify app is running
Write-Host "`nVerifying app is running..." -ForegroundColor Cyan
$pid = (& $ADB -s $Serial shell pidof com.rfditservices.operatorgame 2>$null).Trim()

if (-not $pid) {
    Write-Host "⚠️  App not running. Launching..." -ForegroundColor Yellow
    & $ADB -s $Serial shell am start -n "com.rfditservices.operatorgame/android.app.NativeActivity"
    Start-Sleep -Seconds 3
}

Write-Host "✓ App running (PID: $pid)" -ForegroundColor Green

# Screenshot capture function
function Capture-Tab {
    param(
        [string]$TabName,
        [string]$Description = "",
        [int]$DelaySeconds = 1
    )
    
    Write-Host "`n[$TabName]" -ForegroundColor Cyan
    if ($Description) {
        Write-Host "  $Description" -ForegroundColor Gray
    }
    
    # Wait for UI to settle
    Start-Sleep -Seconds $DelaySeconds
    
    # Capture screenshot
    $filename = "$OutputDir\$TabName.png"
    & $ADB -s $Serial exec-out screencap -p > $filename
    
    if (Test-Path $filename) {
        $size = (Get-Item $filename).Length / 1KB
        Write-Host "  ✓ Captured: $TabName.png ($([math]::Round($size, 1)) KB)" -ForegroundColor Green
    } else {
        Write-Host "  ❌ Failed to capture: $TabName" -ForegroundColor Red
    }
}

# Navigation function (simulate tap on tab bar)
function Navigate-Tab {
    param(
        [ValidateSet("Roster", "Missions", "Map", "Logs")]
        [string]$Tab
    )
    
    # Moto G screen dimensions (approximate): 412 x 1900
    # Bottom tab bar is at bottom ~56dp
    # Tab positions (rough estimates for 412dp width / 4 tabs = ~103dp per tab):
    $tabPositions = @{
        "Roster"   = "50,1870"    # Left side
        "Missions" = "155,1870"   # Left-center
        "Map"      = "260,1870"   # Right-center
        "Logs"     = "365,1870"   # Right side
    }
    
    if ($tabPositions.ContainsKey($Tab)) {
        $pos = $tabPositions[$Tab]
        Write-Host "  Tapping $Tab at ($pos)..." -ForegroundColor Gray
        & $ADB -s $Serial shell input tap $pos.Split(',')[0] $pos.Split(',')[1]
        Start-Sleep -Seconds 1
    } else {
        Write-Host "  ❌ Unknown tab: $Tab" -ForegroundColor Red
    }
}

# Sub-tab navigation function
function Navigate-SubTab {
    param(
        [string]$SubTabName,
        [int]$TapX = 150,
        [int]$TapY = 300
    )
    
    Write-Host "  Tapping sub-tab: $SubTabName at ($TapX, $TapY)..." -ForegroundColor Gray
    & $ADB -s $Serial shell input tap $TapX $TapY
    Start-Sleep -Seconds 1
}

# ===== ROSTER TAB =====
Write-Host "`n[ROSTER TAB]" -ForegroundColor Yellow
Navigate-Tab "Roster"

# Sub-tab: Collection
Write-Host "`n  Sub-tab: Collection" -ForegroundColor Cyan
Navigate-SubTab "Collection" 150 300
Capture-Tab "01_Roster_Collection" "Slime cards, staging UI"

# Sub-tab: Breeding (if exists)
Write-Host "`n  Sub-tab: Breeding (if accessible)" -ForegroundColor Cyan
Navigate-SubTab "Breeding" 250 300
Capture-Tab "02_Roster_Breeding" "Incubator, breeding pair UI"

# ===== MISSIONS TAB =====
Write-Host "`n[MISSIONS TAB]" -ForegroundColor Yellow
Navigate-Tab "Missions"
Start-Sleep -Seconds 1

# Sub-tab: Active
Write-Host "`n  Sub-tab: Active" -ForegroundColor Cyan
Navigate-SubTab "Active" 150 300
Capture-Tab "03_Missions_Active" "In-progress deployments, AAR"

# Sub-tab: Quest Board
Write-Host "`n  Sub-tab: Quest Board (if accessible)" -ForegroundColor Cyan
Navigate-SubTab "QuestBoard" 250 300
Capture-Tab "04_Missions_QuestBoard" "Available quests, requirements"

# ===== MAP TAB =====
Write-Host "`n[MAP TAB]" -ForegroundColor Yellow
Navigate-Tab "Map"
Start-Sleep -Seconds 1

# Map default view (Zones)
Capture-Tab "05_Map_Zones" "Ring structure, zone cards"

# ===== LOGS TAB =====
Write-Host "`n[LOGS TAB]" -ForegroundColor Yellow
Navigate-Tab "Logs"
Start-Sleep -Seconds 1

# Sub-tab: Mission History
Write-Host "`n  Sub-tab: Mission History" -ForegroundColor Cyan
Navigate-SubTab "MissionHistory" 150 300
Capture-Tab "06_Logs_MissionHistory" "Past missions, AAR history"

# Sub-tab: Culture History (if accessible)
Write-Host "`n  Sub-tab: Culture History (if accessible)" -ForegroundColor Cyan
Navigate-SubTab "CultureHistory" 280 300
Capture-Tab "07_Logs_CultureHistory" "Culture timeline, discovery progression"

# ===== SUMMARY =====
Write-Host "`n" -ForegroundColor Green
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  Screenshot Capture Complete                              ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Green

$screenshots = Get-ChildItem $OutputDir -Filter "*.png" | Measure-Object
Write-Host "`n✓ Captured $($screenshots.Count) screenshots" -ForegroundColor Green
Write-Host "✓ Output directory: $OutputDir" -ForegroundColor Green
Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  1. Review screenshots for UI issues (overlay, alignment, accessibility)"
Write-Host "  2. Document issues found (which tabs, which elements)"
Write-Host "  3. Create directive to fix identified issues"
```

**Key Points:**
- Auto-detects device (or accepts `-Serial` parameter)
- Creates timestamped output directory
- Verifies app is running (launches if needed)
- Navigates to each tab systematically
- Simulates taps on UI elements to switch tabs/sub-tabs
- Captures screenshot after each navigation
- Numbered/named for easy review (01_Roster_Collection.png, etc.)

---

## Part B: Coordinate-Finding Helper

### Task B.1: Create `find_ui_coordinates.ps1`

**Purpose:** Help identify exact tap coordinates on Moto G screen for accurate sub-tab navigation.

```powershell
param(
    [string]$Serial = $null
)

Write-Host "UI Coordinate Finder — Touch to identify coordinates" -ForegroundColor Cyan
Write-Host "Instructions:" -ForegroundColor Yellow
Write-Host "  1. When prompted, touch the UI element on the Moto G screen"
Write-Host "  2. The script will capture the touch coordinates"
Write-Host "  3. Note the coordinates and update capture_screenshots.ps1"
Write-Host ""

$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

if (-not (Test-Path $ADB)) {
    Write-Error "ADB not found."
    exit 1
}

# Auto-detect device
if (-not $Serial) {
    & $ADB start-server 2>&1 | Out-Null
    $devices = & $ADB devices | Select-Object -Skip 1 | Where-Object { $_.Trim() -and $_ -notmatch "List of" }
    $Serial = ($devices[0] -split '\s+')[0]
}

Write-Host "Device: $Serial" -ForegroundColor Green
Write-Host ""

# Start getevent on device and monitor for touch events
Write-Host "Ready. Touch the Moto G screen now..." -ForegroundColor Cyan
Write-Host "(Press Ctrl+C to stop)" -ForegroundColor Gray
Write-Host ""

& $ADB -s $Serial shell getevent /dev/input/event* | ForEach-Object {
    if ($_ -match "ABS_MT_POSITION_X|ABS_MT_POSITION_Y|BTN_TOUCH") {
        Write-Host $_
    }
}
```

**Usage:**
```powershell
.\find_ui_coordinates.ps1
# Touch elements on screen to see their coordinates
# (Ctrl+C to exit)
```

---

## Part C: Manual Coordinate Reference

**Estimated tap positions for Moto G 2025 (412dp × 1900dp, portrait):**

```
Bottom Tab Bar (y ≈ 1850-1900):
  Roster:   x ≈ 50    (left)
  Missions: x ≈ 155   (left-center)
  Map:      x ≈ 260   (right-center)
  Logs:     x ≈ 365   (right)

Sub-tabs (typical positions, y ≈ 300):
  Left sub-tab:   x ≈ 150
  Right sub-tab:  x ≈ 250
  Third sub-tab:  x ≈ 280

Content Area: y ≈ 400-1800 (scrollable)
```

---

## Acceptance Criteria

✓ Script captures screenshots of all UI tabs (Roster, Missions, Map, Logs)  
✓ Script navigates to sub-tabs where they exist  
✓ Screenshots are numbered and clearly named (01_Roster_Collection.png, etc.)  
✓ Output organized in timestamped directory  
✓ Script handles device auto-detection  
✓ Script verifies app is running (launches if needed)  
✓ Coordinate-finding helper available for fine-tuning tap positions  
✓ All scripts parse cleanly, execute without errors  

---

## Success Looks Like

After running:
```powershell
.\capture_screenshots.ps1
```

You get a folder like `screenshots_20260321_145230/` containing:
```
01_Roster_Collection.png
02_Roster_Breeding.png
03_Missions_Active.png
04_Missions_QuestBoard.png
05_Map_Zones.png
06_Logs_MissionHistory.png
07_Logs_CultureHistory.png
```

You can then:
1. Review each screenshot for UI issues (overlay, alignment, accessibility)
2. Document which elements are broken or inaccessible
3. Create a second directive to fix identified issues

---

## Notes for Agent

- Tap coordinates are approximations — use find_ui_coordinates.ps1 to refine them
- Some sub-tabs may not exist yet (Breeding, QuestBoard, CultureHistory) — script should skip with warning if tap fails
- Screencap output should be in PNG format (binary, not text)
- Add comments in script showing estimated screen layout for future reference
- Consider adding a `-Interactive` mode that waits for user input between captures (slower, but more reliable)

---
