# OperatorGame — Build/Deploy Automation Formalization Directive

**Target:** Coding Agent (Gemini/Antigravity)  
**Goal:** Create `deploy_moto.ps1` (full build→deploy→test cycle) + fix critical automation issues  
**Scope:** New script + fixes to existing scripts  
**Output:** Formalized, repeatable deploy workflow for Moto G testing  

---

## Context

**Current State:**
- 5 build/deploy scripts exist (setup_local_forge.ps1, build_android.ps1, build_aab.ps1, check_moto.ps1, build_android.sh)
- **Critical gap:** No automated deploy step. Testing Phase F.0 requires 6 manual commands across 3 scripts
- **Three systemic issues identified:**
  1. Three different hardcoded APK paths (fragility point)
  2. Cargo.toml signing config points to debug keystore (silent re-signing happens afterward)
  3. check_moto.ps1 ADB path hardcoded to one user's machine

**Goal:** Create a single `deploy_moto.ps1` that does the full build→install→launch→log cycle in one command.

---

## Part A: Create `deploy_moto.ps1` (Primary Deliverable)

**File:** `deploy_moto.ps1` (new file in repo root)

**Purpose:** Full build → deploy → test workflow for Moto G 2025. Chains all necessary steps so developer can test Phase F.0 UI polish with one command.

**Requirements:**

1. **Dot-source setup_local_forge.ps1** to preserve environment variables
   ```powershell
   . .\setup_local_forge.ps1
   ```

2. **Check Moto G is connected** via ADB before proceeding
   ```powershell
   $ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
   
   # Verify ADB path exists
   if (-not (Test-Path $ADB)) {
       Write-Error "ADB not found at $ADB. Run setup_local_forge.ps1 first."
       exit 1
   }
   
   # Check device is connected
   $devices = & $ADB devices | Select-Object -Skip 1 | Where-Object { $_.Trim() -and $_ -notmatch "List of" }
   if ($devices.Count -eq 0) {
       Write-Error "No ADB device connected. Connect Moto G and enable USB debugging."
       exit 1
   }
   Write-Host "✓ Device connected: $($devices[0])" -ForegroundColor Green
   ```

3. **Build + sign APK**
   ```powershell
   Write-Host "`n[Step 2/5] Building + signing APK..." -ForegroundColor Cyan
   .\build_android.ps1
   if (-not (Test-Path "operatorgame-release.apk")) {
       Write-Error "APK build failed. operatorgame-release.apk not found."
       exit 1
   }
   Write-Host "✓ APK built and signed" -ForegroundColor Green
   ```

4. **Install APK on phone** (with -r to replace existing)
   ```powershell
   Write-Host "`n[Step 3/5] Installing APK on Moto G..." -ForegroundColor Cyan
   & $ADB install -r operatorgame-release.apk
   if ($LASTEXITCODE -ne 0) {
       Write-Error "APK install failed. Check ADB connection."
       exit 1
   }
   Write-Host "✓ APK installed" -ForegroundColor Green
   ```

5. **Launch the app**
   ```powershell
   Write-Host "`n[Step 4/5] Launching OperatorGame..." -ForegroundColor Cyan
   & $ADB shell am start -n "com.rfditservices.operatorgame/android.app.NativeActivity"
   Write-Host "✓ App launched" -ForegroundColor Green
   ```

6. **Stream filtered logcat** (only OPERATOR package, Error level)
   ```powershell
   Write-Host "`n[Step 5/5] Streaming logs (Ctrl+C to stop)..." -ForegroundColor Cyan
   & $ADB logcat -c  # Clear buffer
   & $ADB logcat --pid=$(&$ADB shell pidof com.rfditservices.operatorgame)
   ```

**Full Script Structure:**
```powershell
param()

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  OperatorGame Deploy to Moto G 2025                       ║" -ForegroundColor Cyan
Write-Host "║  Full build → sign → install → launch → log cycle         ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

# [Step 1: Setup env]
# [Step 2: Build + sign]
# [Step 3: Install]
# [Step 4: Launch]
# [Step 5: Stream logs]

Write-Host "`n✅ Deploy cycle complete!" -ForegroundColor Green
```

**Output:**
- Formatted progress messages (Cyan headers, Green checkmarks)
- Real-time error handling (stops if any step fails)
- Logcat stream to terminal (Ctrl+C to stop)

**Status Code:**
- Exit 0 if successful
- Exit 1 if any step fails (ADB not found, device disconnected, APK build failed, install failed)

---

## Part B: Fix Critical Issues (Secondary)

### Issue 1: check_moto.ps1 Hardcoded ADB Path

**Current:**
```powershell
$adb = "C:\Users\cheat\AppData\Local\Android\Sdk\platform-tools\adb.exe"
```

**Fix:**
Replace with SDK-relative resolution (matching build_android.ps1 pattern):
```powershell
$ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"

if (-not (Test-Path $ADB)) {
    Write-Error "ADB not found. Run setup_local_forge.ps1 first."
    exit 1
}
```

**Also update logcat command to filter by package:**
```powershell
& $ADB logcat -c  # Clear buffer
& $ADB logcat --pid=$(& $ADB shell pidof com.rfditservices.operatorgame) *:E
# Or simpler: filter by tag
& $ADB logcat -s "OPERATOR"
```

---

### Issue 2: APK Path Discovery (Optional but Recommended)

**Current (fragile):**
```powershell
$ApkUnsigned = "target/release/apk/operator.apk"  # Hardcoded
```

**Better (discovers actual path):**
```powershell
# Find the APK cargo-apk actually produced
$ApkUnsigned = Get-ChildItem "target\release\apk\*.apk" -ErrorAction SilentlyContinue | 
    Where-Object { $_.Name -like "operator*.apk" } | 
    Select-Object -First 1 -ExpandProperty FullName

if (-not $ApkUnsigned) {
    Write-Error "No APK found in target\release\apk\. cargo apk build may have failed."
    exit 1
}

Write-Host "Found unsigned APK: $ApkUnsigned"
```

**Apply this fix to:**
- build_android.ps1 (line ~20)
- build_aab.ps1 (if used for Play Store releases)

---

### Issue 3: Cargo.toml Signing Config (Informational Only)

**Current Risk:**
```toml
[package.metadata.android.signing.release]
path              = "C:/Users/cheat/.android/debug.keystore"  # ← points to debug keystore!
keystore_password = "android"
key_alias         = "androiddebugkey"
```

**Status:** The PS1 scripts re-sign the APK with the release key afterward, so the final APK is correct. However, this is a silent footgun.

**Recommendation (do not implement yet):**
Either update this to point to operatorgame-release.jks, or remove the block entirely so cargo-apk produces unsigned APK (and PS1 scripts sign it with release key). Defer to next phase (Phase F.1 or later) since current workflow is functioning.

---

## Part C: Documentation

Create a `DEPLOY.md` file in repo root:

```markdown
# OperatorGame Build & Deploy Guide

## Quick Start: Deploy to Moto G

### One-Command Deploy (Phase F.0 Testing)
```powershell
.\deploy_moto.ps1
```

This command:
1. Sets up the build environment (Rust, NDK, Android SDK)
2. Builds and signs the APK
3. Installs the APK on your connected Moto G
4. Launches the app
5. Streams logcat logs to your terminal

### Prerequisites
- Moto G 2025 connected via USB
- USB debugging enabled on phone
- Windows PowerShell (or Core 7+)

### Manual Workflow (if needed)

If `deploy_moto.ps1` encounters issues, run steps manually:

```powershell
# 1. Set up environment (one-time per session)
. .\setup_local_forge.ps1

# 2. Verify device is connected
adb devices

# 3. Build and sign APK
.\build_android.ps1

# 4. Install on phone
adb install -r operatorgame-release.apk

# 5. Launch app
adb shell am start -n "com.rfditservices.operatorgame/android.app.NativeActivity"

# 6. View logs
.\check_moto.ps1
```

### Build Artifacts
- `operatorgame-release.apk` — Signed, zipaligned APK ready for install
- `operatorgame-release.aab` — Play Store bundle (requires manual signing step)

### Troubleshooting
- **"ADB not found"**: Run `.\setup_local_forge.ps1` to set up environment
- **"No device connected"**: Connect Moto G via USB and enable USB debugging in Developer Options
- **"Install failed"**: APK may be incompatible with API 35; check Android Studio device manager

### Scripts Overview
| Script | Purpose | Status |
|--------|---------|--------|
| `setup_local_forge.ps1` | Environment setup (Rust, NDK, SDK) | ✅ Active |
| `build_android.ps1` | Build + sign APK | ✅ Active |
| `check_moto.ps1` | Stream device logs | ✅ Active (fixed) |
| `deploy_moto.ps1` | Full build → deploy → test | ✅ New |

```

---

## Acceptance Criteria

**Primary Deliverable (deploy_moto.ps1):**
✓ Chains all 5 steps (setup → build → install → launch → log)  
✓ Checks Moto G is connected before proceeding  
✓ Formatted output (progress messages, color coding, checkmarks)  
✓ Error handling (stops on failure, reports which step failed)  
✓ Exit codes (0 on success, 1 on failure)  
✓ Executable as-is (no manual steps required)  

**Secondary Fixes:**
✓ check_moto.ps1 ADB path fixed (no longer hardcoded to one user)  
✓ check_moto.ps1 logcat filtered by package (less noise)  
✓ Optional: APK path discovery applied to build_android.ps1  

**Documentation:**
✓ DEPLOY.md created (quick start, prerequisites, manual workflow, troubleshooting)  

---

## Success Looks Like

After this directive is complete:

**Developer workflow for Phase F.0 testing:**
```powershell
cd C:\Github\OperatorGame
.\deploy_moto.ps1
# (waits 2-3 minutes while building)
# APK installs on Moto G
# App launches
# Logcat streams to terminal
# Developer can test UI Polish immediately
```

**No manual ADB commands required.**

---

## Implementation Order

1. **First:** Create deploy_moto.ps1 (primary deliverable)
2. **Second:** Fix check_moto.ps1 ADB path + logcat filtering
3. **Third:** Create DEPLOY.md documentation
4. **Optional:** Apply APK path discovery to build_android.ps1

---

## Notes for Agent

- deploy_moto.ps1 should be executable immediately after creation (no manual tweaks)
- Error messages should be specific ("APK build failed" vs just "Error")
- Colors and formatting improve readability during long builds
- Assume developer has already run setup_local_forge.ps1 once per session
- Target audience: Robert testing Phase F.0 on Moto G (not a CI/CD pipeline)

---

## Deliverables

**New Files:**
- deploy_moto.ps1 (full build→deploy→test workflow)
- DEPLOY.md (user documentation)

**Modified Files:**
- check_moto.ps1 (fix hardcoded ADB path, add package filtering)
- build_android.ps1 (optional: add APK path discovery)

**Tests:**
- Manual: Run `.\deploy_moto.ps1` with Moto G connected, verify each step completes
- No unit tests needed (these are automation scripts, not library code)

---

## Post-Implementation

Once deploy_moto.ps1 is working:
1. Robert can test Phase F.0 UI Polish on Moto G with one command
2. Future phases (F.1, F.2, F.3, F.4) can be tested the same way
3. Scripts are now formally documented (DEPLOY.md)
4. Hardcoded paths are no longer a single point of failure

---
