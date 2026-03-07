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

$Keystore = "operatorgame-release.jks"
$Alias = "operatorgame"
$ApkUnsigned = "target/release/apk/operator.apk" # Adjust based on cargo-apk/xbuild output
$ApkAligned = "operatorgame-release-aligned.apk"
$ApkFinal = "operatorgame-release.apk"

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

Write-Host "🚀 Building release binary (aarch64-linux-android)..." -ForegroundColor Cyan
cargo build --release --target aarch64-linux-android

Write-Host "📦 Packaging APK..." -ForegroundColor Cyan
# Replace with xbuild if your project uses it instead of cargo-apk
cargo apk build --release 

if (-not (Test-Path $ApkUnsigned)) {
    Write-Host "Error: Could not find output APK at $ApkUnsigned. Adjust path in script." -ForegroundColor Red
    exit 1
}

Write-Host "🔐 Aligning APK..." -ForegroundColor Cyan
zipalign -v -p 4 $ApkUnsigned $ApkAligned

Write-Host "✍️ Signing APK via apksigner (or jarsigner fallback)..." -ForegroundColor Cyan
if (Get-Command apksigner -ErrorAction SilentlyContinue) {
    apksigner sign --ks $Keystore --out $ApkFinal $ApkAligned
} else {
    Write-Host "apksigner not found, falling back to jarsigner..." -ForegroundColor Yellow
    Copy-Item $ApkAligned $ApkFinal -Force
    jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 -keystore $Keystore $ApkFinal $Alias
}

Remove-Item $ApkAligned -ErrorAction SilentlyContinue

Write-Host "✅ Success! Final signed APK ready at: $ApkFinal" -ForegroundColor Green
