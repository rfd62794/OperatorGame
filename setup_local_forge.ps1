# setup_local_forge.ps1
# Automates the detection and validation of the local Android Build environment.

$NDK_VERSION = "29.0.14206865"
$NDK_DOWNLOAD_URL = "https://developer.android.com/ndk/downloads"

Write-Host "--- OPERATOR: LOCAL FORGE AUDIT ---" -ForegroundColor Cyan

# 1. Check Rust Targets
Write-Host "[1/4] Checking Rust Android Targets..." -ForegroundColor Gray
$targets = rustup target list --installed
if ($targets -like "*aarch64-linux-android*" -and $targets -like "*armv7-linux-androideabi*") {
    Write-Host "  OK: Android targets installed." -ForegroundColor Green
}
else {
    Write-Host "  MISSING: Running 'rustup target add aarch64-linux-android armv7-linux-androideabi'..." -ForegroundColor Yellow
    rustup target add aarch64-linux-android armv7-linux-androideabi
}

# 2. Check cargo-apk
Write-Host "[2/4] Checking cargo-apk..." -ForegroundColor Gray
if (Get-Command cargo-apk -ErrorAction SilentlyContinue) {
    Write-Host "  OK: cargo-apk is installed." -ForegroundColor Green
}
else {
    Write-Host "  MISSING: cargo-apk not found. Install it with 'cargo install cargo-apk'." -ForegroundColor Red
    return
}

# 3. Check ANDROID_HOME (SDK)
Write-Host "[3/4] Checking Android SDK..." -ForegroundColor Gray
$sdkPath = $env:ANDROID_HOME
if (-not $sdkPath) { $sdkPath = $env:ANDROID_SDK_ROOT }

# Auto-detect common paths if variables are missing
if (-not $sdkPath) {
    $commonSdkPaths = @(
        "$env:LOCALAPPDATA\Android\Sdk",
        "C:\Android\Sdk"
    )
    foreach ($p in $commonSdkPaths) {
        if (Test-Path $p) {
            $sdkPath = $p
            Write-Host "  AUTO-DETECTED SDK: $sdkPath" -ForegroundColor Yellow
            [System.Environment]::SetEnvironmentVariable('ANDROID_HOME', $sdkPath, 'User')
            $env:ANDROID_HOME = $sdkPath
            Write-Host "  ACTION: ANDROID_HOME environment variable set." -ForegroundColor Green
            break
        }
    }
}

if ($sdkPath -and (Test-Path $sdkPath)) {
    Write-Host "  OK: SDK found at $sdkPath" -ForegroundColor Green
}
else {
    Write-Host "  CRITICAL: ANDROID_HOME is not set and auto-detection failed." -ForegroundColor Red
    Write-Host "  Please install the Android SDK (via Android Studio) and set the environment variable." -ForegroundColor Yellow
}

# 4. Check ANDROID_NDK_HOME
Write-Host "[4/4] Checking Android NDK..." -ForegroundColor Gray
$ndkPath = $env:ANDROID_NDK_HOME

# Auto-detect common NDK paths if variables are missing
if (-not $ndkPath -and $sdkPath) {
    $ndkBase = Join-Path $sdkPath "ndk"
    if (Test-Path $ndkBase) {
        $versions = Get-ChildItem $ndkBase | Sort-Object Name -Descending
        if ($versions.Count -gt 0) {
            $ndkPath = $versions[0].FullName
            Write-Host "  AUTO-DETECTED NDK: $ndkPath" -ForegroundColor Yellow
            [System.Environment]::SetEnvironmentVariable('ANDROID_NDK_HOME', $ndkPath, 'User')
            $env:ANDROID_NDK_HOME = $ndkPath
            Write-Host "  ACTION: ANDROID_NDK_HOME environment variable set." -ForegroundColor Green
        }
    }
}

if ($ndkPath -and (Test-Path $ndkPath)) {
    Write-Host "  OK: NDK found at $ndkPath" -ForegroundColor Green
}
else {
    Write-Host "  CRITICAL: ANDROID_NDK_HOME is not set and auto-detection failed." -ForegroundColor Red
    Write-Host "  The 'Operator' build requires NDK $NDK_VERSION or higher." -ForegroundColor Yellow
    Write-Host "  Download it from: $NDK_DOWNLOAD_URL" -ForegroundColor Cyan
}

Write-Host "`nReady for 'cargo apk run'?" -ForegroundColor Cyan
if ($ndkPath -and $sdkPath -and (Get-Command cargo-apk -ErrorAction SilentlyContinue)) {
    Write-Host "  YES. Plug in your device and push the Vanguard." -ForegroundColor Green
    Write-Host "  (NOTE: You may need to RESTART your terminal for changes to fully apply.)" -ForegroundColor Yellow
}
else {
    Write-Host "  NO. Fix the missing variables above first." -ForegroundColor Red
}
