param(
    [string]$Serial = $null,
    [string]$OutputDir = "screenshots_$(Get-Date -Format 'yyyyMMdd_HHmmss')",
    [switch]$Interactive
)

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  OperatorGame Mobile Screenshot Capture                    " -ForegroundColor Cyan
Write-Host "  Automated UI tab screenshots from Moto G                  " -ForegroundColor Cyan
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan

$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

if (-not (Test-Path $ADB)) {
    Write-Error "ADB not found. Run setup_local_forge.ps1 first."
    exit 1
}

# Auto-detect device if not specified
if (-not $Serial) {
    & $ADB start-server 2>&1 | Out-Null
    $devices = & $ADB devices | Select-Object -Skip 1 | Where-Object { $_.Trim() -and ($_ -notmatch "List of") }
    
    if ($devices.Count -eq 0) {
        Write-Error "No devices connected."
        exit 1
    }
    
    $Serial = ($devices[0] -split '\s+')[0]
    Write-Host "[OK] Auto-detected device: $Serial" -ForegroundColor Green
} else {
    Write-Host "[OK] Using device: $Serial" -ForegroundColor Green
}

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

Write-Host "[OK] Output directory: $OutputDir" -ForegroundColor Green

# Verify app is running
Write-Host "`nVerifying app is running..." -ForegroundColor Cyan
$pidValue = (& $ADB -s $Serial shell pidof com.rfditservices.operatorgame 2>$null).Trim()

if (-not $pidValue) {
    Write-Host "[WARN] App not running. Launching..." -ForegroundColor Yellow
    & $ADB -s $Serial shell am start -n "com.rfditservices.operatorgame/android.app.NativeActivity"
    Start-Sleep -Seconds 3
    $pidValue = (& $ADB -s $Serial shell pidof com.rfditservices.operatorgame 2>$null).Trim()
}

Write-Host "[OK] App running (PID: $pidValue)" -ForegroundColor Green

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
    if ($Interactive) {
        Read-Host "  [Interactive] Press Enter to capture when UI is ready..."
    } else {
        Start-Sleep -Seconds $DelaySeconds
    }
    
    # Capture screenshot
    $filename = "$OutputDir\$TabName.png"
    & $ADB -s $Serial exec-out screencap -p > $filename
    
    if (Test-Path $filename) {
        $size = (Get-Item $filename).Length / 1KB
        $sizeStr = [math]::Round($size, 1)
        Write-Host "  [OK] Captured: $TabName.png ($sizeStr KB)" -ForegroundColor Green
    } else {
        Write-Host "  [ERROR] Failed to capture: $TabName" -ForegroundColor Red
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
        $parts = $pos -split ','
        & $ADB -s $Serial shell input tap $parts[0] $parts[1]
        Start-Sleep -Seconds 1
    } else {
        Write-Host "  [ERROR] Unknown tab: $Tab" -ForegroundColor Red
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
Write-Host "------------------------------------------------------------" -ForegroundColor Green
Write-Host "  Screenshot Capture Complete                               " -ForegroundColor Green
Write-Host "------------------------------------------------------------" -ForegroundColor Green

$screenshots = Get-ChildItem $OutputDir -Filter "*.png" | Measure-Object
Write-Host "`n[OK] Captured $($screenshots.Count) screenshots" -ForegroundColor Green
Write-Host "[OK] Output directory: $OutputDir" -ForegroundColor Green
Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  1. Review screenshots for UI issues (overlay, alignment, accessibility)"
Write-Host "  2. Document issues found (which tabs, which elements)"
Write-Host "  3. Create directive to fix identified issues"
