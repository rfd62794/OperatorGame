# setup_local_forge.ps1
# Automates the detection and validation of the local Android Build environment.
# Version: 2.1 (Smart Forge / Linker & Java Sovereignty)

$PREFERRED_NDK = "25.2.9519653" # r25c
Write-Host "--- OPERATOR: SMART FORGE AUDIT ---" -ForegroundColor Cyan

# 1. Check Rust Targets
Write-Host "[1/5] Checking Rust Android Targets..." -ForegroundColor Gray
$targets = rustup target list --installed
$reqTargets = @("aarch64-linux-android", "armv7-linux-androideabi", "i686-linux-android", "x86_64-linux-android")
foreach ($t in $reqTargets) {
    if ($targets -notlike "*$t*") {
        Write-Host "  MISSING: $t. Installing..." -ForegroundColor Yellow
        rustup target add $t
    }
}
Write-Host "  OK: Android architectures ready." -ForegroundColor Green

# 2. Check cargo-apk
Write-Host "[2/5] Checking cargo-apk..." -ForegroundColor Gray
if (-not (Get-Command cargo-apk -ErrorAction SilentlyContinue)) {
    Write-Host "  ACTION: Installing cargo-apk..." -ForegroundColor Yellow
    cargo install cargo-apk
}
Write-Host "  OK: Packaging tools ready." -ForegroundColor Green

# 3. Smart Scan: SDK & NDK
Write-Host "[3/5] Performing Smart Scan for SDK/NDK..." -ForegroundColor Gray
$sdkPath = $env:ANDROID_HOME
if (-not $sdkPath) { $sdkPath = $env:ANDROID_SDK_ROOT }
if (-not $sdkPath) {
    $commonSdkPaths = @("$env:LOCALAPPDATA\Android\Sdk", "C:\Android\Sdk")
    foreach ($p in $commonSdkPaths) { if (Test-Path $p) { $sdkPath = $p; break } }
}

if ($sdkPath) {
    Write-Host "  FOUND SDK: $sdkPath" -ForegroundColor Green
    $env:ANDROID_HOME = $sdkPath
    [System.Environment]::SetEnvironmentVariable('ANDROID_HOME', $sdkPath, 'User')
}
else {
    Write-Host "  CRITICAL: Android SDK not found. Install Android Studio." -ForegroundColor Red
    return
}

# NDK Logic: Detect and Prioritize
$ndkBase = Join-Path $sdkPath "ndk"
$ndkPath = ""
if (Test-Path $ndkBase) {
    $versions = Get-ChildItem $ndkBase | Sort-Object Name -Descending
    if ($versions.Count -gt 0) {
        # Check for preferred
        $prefMatch = $versions | Where-Object { $_.Name -eq $PREFERRED_NDK }
        if ($prefMatch) {
            $ndkPath = $prefMatch.FullName
            Write-Host "  MATCH: Preferred NDK found ($PREFERRED_NDK)." -ForegroundColor Green
        }
        else {
            $ndkPath = $versions[0].FullName
            Write-Host "  FALLBACK: Using most recent NDK found ($($versions[0].Name))." -ForegroundColor Yellow
        }
    }
}

if ($ndkPath) {
    Write-Host "  FOUND NDK: $ndkPath" -ForegroundColor Green
    $env:ANDROID_NDK_HOME = $ndkPath
    [System.Environment]::SetEnvironmentVariable('ANDROID_NDK_HOME', $ndkPath, 'User')
    
    # 4. Linker Sovereignty: Inject toolchain into PATH
    Write-Host "[4/5] Hardening Linker Sovereignty..." -ForegroundColor Gray
    $toolchainBin = Join-Path $ndkPath "toolchains\llvm\prebuilt\windows-x86_64\bin"
    if (Test-Path $toolchainBin) {
        if ($env:PATH -notlike "*$toolchainBin*") {
            $env:PATH = "$toolchainBin;$env:PATH"
            Write-Host "  ACTION: Injected NDK toolchain into session PATH." -ForegroundColor Green
        }
    }
    else {
        Write-Host "  WARNING: Could not find toolchain/llvm bin in NDK. Linker may fail." -ForegroundColor Red
    }
    
    # 4b. Java Sovereignty: Inject keytool into PATH
    $studioPath = "C:\Program Files\Android\Android Studio"
    if (Test-Path $studioPath) {
        $jbrBin = Join-Path $studioPath "jbr\bin"
        if (Test-Path $jbrBin) {
            if ($env:PATH -notlike "*$jbrBin*") {
                $env:PATH = "$jbrBin;$env:PATH"
                Write-Host "  ACTION: Injected Java JBR into session PATH (keytool)." -ForegroundColor Green
            }
        }
    }
}
else {
    Write-Host "  CRITICAL: NDK not found. Install NDK (Side-by-side) in Android Studio." -ForegroundColor Red
    return
}

# 5. The Purge (Optional Cleanup)
if ($args -contains "-Cleanup") {
    Write-Host "[5/5] Performing Resource Recovery (Cleanup)..." -ForegroundColor Cyan
    if ($versions.Count -gt 1) {
        foreach ($v in $versions[1..($versions.Count - 1)]) {
            Write-Host "  PRUNING: Redundant NDK version $($v.Name)..." -ForegroundColor Gray
            Remove-Item -Path $v.FullName -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
    Write-Host "  OK: Cleanup complete." -ForegroundColor Green
}
else {
    Write-Host "[5/5] Skipping Cleanup. (Use -Cleanup to prune redundant NDKs)." -ForegroundColor Gray
}

Write-Host "`n--- FORGE HARDENED: READY FOR VANGUARD ---" -ForegroundColor Cyan
Write-Host "Run: cargo apk run --release" -ForegroundColor Green
