<#
.SYNOPSIS
Audits the NDK version in use against what .cargo/config.toml expects.

.EXAMPLE
.\check_ndk_config.ps1
#>

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  NDK Configuration Audit" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

# ---------------------------------------------------------------------------
# 1. What does .cargo/config.toml expect?
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[1/4] .cargo/config.toml linker configuration:" -ForegroundColor Cyan

if (Test-Path ".cargo\config.toml") {
    Get-Content ".cargo\config.toml" | ForEach-Object {
        if ($_ -match "linker|rustflags|aarch64|armv7|android") {
            Write-Host "  $_" -ForegroundColor DarkGray
        }
    }
} else {
    Write-Host "  WARNING: .cargo/config.toml not found." -ForegroundColor Yellow
}

# ---------------------------------------------------------------------------
# 2. What NDK versions are installed?
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[2/4] Installed NDK versions:" -ForegroundColor Cyan

$sdkPath = $env:ANDROID_HOME
if (-not $sdkPath) { $sdkPath = $env:ANDROID_SDK_ROOT }
if (-not $sdkPath) { $sdkPath = "$env:LOCALAPPDATA\Android\Sdk" }

$ndkBase = Join-Path $sdkPath "ndk"
if (Test-Path $ndkBase) {
    $ndkVersions = Get-ChildItem $ndkBase | Sort-Object Name -Descending
    foreach ($ndk in $ndkVersions) {
        Write-Host "  NDK $($ndk.Name)" -ForegroundColor White
        # Check for libc++_shared.so
        $libcppPath = Join-Path $ndk.FullName "toolchains\llvm\prebuilt\windows-x86_64\sysroot\usr\lib\aarch64-linux-android\libc++_shared.so"
        if (Test-Path $libcppPath) {
            Write-Host "    libc++_shared.so : FOUND (sysroot)" -ForegroundColor Green
        } else {
            # Try alternate location
            $altPaths = @(
                "$($ndk.FullName)\sources\cxx-stl\llvm-libc++\libs\arm64-v8a\libc++_shared.so",
                "$($ndk.FullName)\toolchains\llvm\prebuilt\windows-x86_64\lib\aarch64-linux-android\libc++_shared.so"
            )
            $found = $altPaths | Where-Object { Test-Path $_ }
            if ($found) {
                Write-Host "    libc++_shared.so : FOUND ($found)" -ForegroundColor Green
            } else {
                Write-Host "    libc++_shared.so : NOT FOUND at standard locations" -ForegroundColor Yellow
            }
        }

        # Check if api-33 clang is present (what config.toml hardcodes)
        $clangPath = Join-Path $ndk.FullName "toolchains\llvm\prebuilt\windows-x86_64\bin\aarch64-linux-android33-clang.cmd"
        if (Test-Path $clangPath) {
            Write-Host "    aarch64-android33-clang : FOUND" -ForegroundColor Green
        } else {
            $clangPath2 = Join-Path $ndk.FullName "toolchains\llvm\prebuilt\windows-x86_64\bin\aarch64-linux-android33-clang"
            if (Test-Path $clangPath2) {
                Write-Host "    aarch64-android33-clang : FOUND (no .cmd)" -ForegroundColor Green
            } else {
                Write-Host "    aarch64-android33-clang : NOT FOUND" -ForegroundColor Red
            }
        }
    }
} else {
    Write-Host "  ERROR: NDK directory not found at $ndkBase" -ForegroundColor Red
}

# ---------------------------------------------------------------------------
# 3. What does Cargo.toml say about STL?
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[3/4] Cargo.toml Android metadata:" -ForegroundColor Cyan

if (Test-Path "Cargo.toml") {
    $inAndroidSection = $false
    Get-Content "Cargo.toml" | ForEach-Object {
        if ($_ -match "\[package\.metadata\.android\]") { $inAndroidSection = $true }
        if ($inAndroidSection -and $_ -match "stl|build_targets|min_sdk|target_sdk|signing") {
            Write-Host "  $_" -ForegroundColor DarkGray
        }
        if ($inAndroidSection -and $_ -match "^\[" -and $_ -notmatch "metadata\.android") {
            $inAndroidSection = $false
        }
    }
} else {
    Write-Host "  WARNING: Cargo.toml not found." -ForegroundColor Yellow
}

# ---------------------------------------------------------------------------
# 4. Verdict
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[4/4] Verdict:" -ForegroundColor Cyan

$activeNdk = $ndkVersions | Select-Object -First 1
Write-Host "  Active NDK (most recent) : $($activeNdk.Name)"

$cargoLinker = (Get-Content ".cargo\config.toml" 2>$null) -match "linker\s*=" | Select-Object -First 1
Write-Host "  Linker from config.toml  : $cargoLinker"

$stlLine = (Get-Content "Cargo.toml" 2>$null) -match "stl\s*=" | Select-Object -First 1
Write-Host "  STL mode in Cargo.toml   : $stlLine"

Write-Host ""
Write-Host "  If 'stl = c++_shared' and libc++_shared.so is MISSING from APK:" -ForegroundColor Yellow
Write-Host "  -> Change to 'stl = `"c++_static`"' in Cargo.toml [package.metadata.android]" -ForegroundColor Yellow
Write-Host "  -> Run: cargo clean && .\build_android.ps1 && .\diagnose_apk.ps1" -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Cyan
