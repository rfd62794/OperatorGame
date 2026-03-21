<#
.SYNOPSIS
Builds, packages, aligns, and signs the Android APK for Play Store deployment.

.EXAMPLE
.\build_android.ps1 -GenerateKeys
.\build_android.ps1
#>

param (
    [switch]$GenerateKeys
)

$ErrorActionPreference = "Stop"

$Keystore  = "operatorgame-release.jks"
$Alias     = "operatorgame"
$ApkAligned = "operatorgame-release-aligned.apk"
$ApkFinal   = "operatorgame-release.apk"

if ($GenerateKeys) {
    Write-Host "Generating release keystore..." -ForegroundColor Cyan
    keytool -genkey -v `
        -keystore $Keystore `
        -alias $Alias `
        -keyalg RSA -keysize 2048 `
        -validity 10000
    
    Write-Host "⚠️ IMPORTANT: Backup $Keystore securely in 2+ locations! ⚠️" -ForegroundColor Yellow
    exit 0
}

if (-not (Test-Path $Keystore)) {
    Write-Host "Error: Keystore not found at $Keystore" -ForegroundColor Red
    Write-Host "Run '.\build_android.ps1 -GenerateKeys' first to create one." -ForegroundColor Yellow
    exit 1
}

$AndroidHome = $env:LOCALAPPDATA + "\Android\Sdk"
if (-not (Test-Path $AndroidHome)) {
    Write-Host "Could not find Android SDK at $AndroidHome. Ensure ANDROID_HOME is set or SDK is installed." -ForegroundColor Red
    exit 1
}

$BuildToolsDir = Get-ChildItem -Path "$AndroidHome\build-tools" -Directory | Sort-Object Name -Descending | Select-Object -First 1
if ($null -eq $BuildToolsDir) {
    Write-Host "Could not find build-tools in Android SDK." -ForegroundColor Red
    exit 1
}

$Zipalign = "$($BuildToolsDir.FullName)\zipalign.exe"
$Apksigner = "$($BuildToolsDir.FullName)\apksigner.bat"

Write-Host "📦 Building and packaging APK with cargo apk..." -ForegroundColor Cyan

# Purge stale APK output so discovery below finds only the new build
Remove-Item -Recurse -Force "target\release\apk" -ErrorAction SilentlyContinue

cargo apk build --release

# Dynamic APK discovery — handles filename variations across cargo-apk versions
$ApkUnsigned = Get-ChildItem -Path "target\release\apk" -Filter "*.apk" -Recurse -ErrorAction SilentlyContinue |
    Select-Object -First 1 -ExpandProperty FullName

if (-not $ApkUnsigned) {
    Write-Host "Error: No APK found under target\release\apk\ after build." -ForegroundColor Red
    Write-Host "       Check the cargo-apk output above for compilation errors." -ForegroundColor Yellow
    exit 1
}

Write-Host "  Found unsigned APK: $ApkUnsigned" -ForegroundColor DarkGray

Write-Host "🔐 Aligning APK..." -ForegroundColor Cyan
& $Zipalign -v -p 4 $ApkUnsigned $ApkAligned

Write-Host "✍️ Signing APK via apksigner..." -ForegroundColor Cyan
if (Test-Path $Apksigner) {
    & $Apksigner sign --ks $Keystore --out $ApkFinal $ApkAligned
}
else {
    Write-Host "apksigner not found, falling back to jarsigner..." -ForegroundColor Yellow
    Copy-Item $ApkAligned $ApkFinal -Force
    jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 -keystore $Keystore $ApkFinal $Alias
}

Remove-Item $ApkAligned -ErrorAction SilentlyContinue

Write-Host "SUCCESS: Final signed APK ready at: $ApkFinal" -ForegroundColor Green
