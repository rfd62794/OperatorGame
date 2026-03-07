<#
.SYNOPSIS
Builds, packages, aligns, and signs the Android App Bundle (AAB) for Play Store.

.EXAMPLE
.\build_aab.ps1 -GenerateKeys
.\build_aab.ps1
#>

param (
    [switch]$GenerateKeys
)

$ErrorActionPreference = "Stop"

$Keystore = "operatorgame-release.jks"
$Alias = "operatorgame"
$ApkUnsigned = "target/release/apk/operatorgame.apk"

if ($GenerateKeys) {
    Write-Host "Generating release keystore..." -ForegroundColor Cyan
    keytool -genkey -v `
        -keystore $Keystore `
        -alias $Alias `
        -keyalg RSA -keysize 2048 `
        -validity 10000

    Write-Host "IMPORTANT: Backup $Keystore securely in 2+ locations!" -ForegroundColor Yellow
    exit 0
}

if (-not (Test-Path $Keystore)) {
    Write-Host "Error: Keystore not found at $Keystore" -ForegroundColor Red
    Write-Host "Run '.\build_aab.ps1 -GenerateKeys' first to create one." -ForegroundColor Yellow
    exit 1
}

# 1. Download bundletool if needed
$BundleTool = "bundletool.jar"
if (-not (Test-Path $BundleTool)) {
    Write-Host "Downloading bundletool.jar..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri "https://github.com/google/bundletool/releases/download/1.17.1/bundletool-all-1.17.1.jar" -OutFile $BundleTool
}

# 2. Locate Android SDK tools
$AndroidHome = $env:LOCALAPPDATA + "\Android\Sdk"
if (-not (Test-Path $AndroidHome)) {
    Write-Host "Could not find Android SDK at $AndroidHome." -ForegroundColor Red
    exit 1
}

$BuildToolsDir = Get-ChildItem -Path "$AndroidHome\build-tools" -Directory | Sort-Object Name -Descending | Select-Object -First 1
$Aapt2 = "$($BuildToolsDir.FullName)\aapt2.exe"

$AndroidJar = Get-ChildItem -Path "$AndroidHome\platforms\android-35\android.jar" -ErrorAction SilentlyContinue
if (-not $AndroidJar) {
    Write-Host "Error: Could not find android.jar for API 35 at $AndroidJar" -ForegroundColor Red
    exit 1
}

# 3. Clean and Build APK
Write-Host "Cleaning cached cargo-apk metadata to force rebuild..." -ForegroundColor Cyan
Remove-Item -Recurse -Force "target\release\apk" -ErrorAction SilentlyContinue

Write-Host "Building Rust payload with cargo apk..." -ForegroundColor Cyan
cargo apk build --release

if (-not (Test-Path $ApkUnsigned)) {
    Write-Host "Error: Could not find output APK at $ApkUnsigned." -ForegroundColor Red
    exit 1
}

# 4. Extract Proto APK and Inject Fixed Manifest
# We use the root AndroidManifest.xml as the base but inject version info
$BaseManifest = "AndroidManifest.xml"
$TempManifest = "target/AndroidManifest.xml.tmp"

$CargoContent = Get-Content "Cargo.toml" -Raw
$VersionMatch = [regex]::Match($CargoContent, '(?m)^version\s*=\s*"([^"]+)"')
$VersionName = if ($VersionMatch.Success) { $VersionMatch.Groups[1].Value } else { "0.0.1" }

# Convert version "0.1.10" to integer 100110 (Major * 1000000 + Minor * 1000 + Patch)
# Or just use the patch count or simple increment. Let's do a simple parse:
$vParts = $VersionName.Split(".")
$vCode = 0
if ($vParts.Count -eq 3) {
    $vCode = [int]$vParts[0] * 1000000 + [int]$vParts[1] * 1000 + [int]$vParts[2]
}
else {
    $vCode = 1 # Fallback
}

Write-Host "Injecting VersionCode $vCode and VersionName $VersionName..." -ForegroundColor Cyan
$ManifestText = Get-Content $BaseManifest -Raw
$ManifestText = $ManifestText -replace '<manifest', "<manifest android:versionCode=""$vCode"" android:versionName=""$VersionName"""
Set-Content -Path $TempManifest -Value $ManifestText -Encoding UTF8

$ProtoApk = "target\proto.zip"
$BaseZip = "target\base.zip"
$AabBase = "target\aab_base"

Write-Host "Converting APK resources to protobuf format..." -ForegroundColor Cyan
& $Aapt2 convert --output-format proto -o $ProtoApk $ApkUnsigned

& $Aapt2 link -I $AndroidJar.FullName --manifest $TempManifest --proto-format -o "target\manifest_proto.zip"

Write-Host "Assembling AAB module structure..." -ForegroundColor Cyan
if (Test-Path $AabBase) { Remove-Item -Recurse -Force $AabBase }
if (Test-Path $BaseZip) { Remove-Item -Force $BaseZip }

Expand-Archive -Path $ProtoApk -DestinationPath $AabBase -Force
Remove-Item -Path "$AabBase\AndroidManifest.xml" -Force

Expand-Archive -Path "target\manifest_proto.zip" -DestinationPath "target\manifest_extract" -Force
New-Item -ItemType Directory -Path "$AabBase\manifest" -Force -ErrorAction SilentlyContinue | Out-Null
Move-Item -Path "target\manifest_extract\AndroidManifest.xml" -Destination "$AabBase\manifest\AndroidManifest.xml" -Force

Remove-Item -Recurse -Force "target\manifest_extract"
Remove-Item -Force "target\manifest_proto.zip"

if (Test-Path "$AabBase\META-INF") { Remove-Item -Recurse -Force "$AabBase\META-INF" }

Write-Host "Zipping module base.zip..." -ForegroundColor Cyan
Push-Location $AabBase
jar cMf "..\base.zip" *
Pop-Location

# 6. Build final AAB
$OutputDir = "target\googleplay"
if (-not (Test-Path $OutputDir)) { New-Item -ItemType Directory -Path $OutputDir | Out-Null }

# Use the $VersionName parsed earlier
$AabFinal = "$OutputDir\operatorgame-release-v$VersionName.aab"
if (Test-Path $AabFinal) { Remove-Item -Force $AabFinal }

Write-Host "Building Android App Bundle ($AabFinal)..." -ForegroundColor Cyan
java -jar $BundleTool build-bundle --modules=$BaseZip --output=$AabFinal

# Cleanup
Remove-Item $ProtoApk -ErrorAction SilentlyContinue
Remove-Item $BaseZip -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force $AabBase -ErrorAction SilentlyContinue

# 7. Provide explicit signing instructions
Write-Host "=============================================" -ForegroundColor Green
Write-Host "AAB built successfully! ($AabFinal)" -ForegroundColor Green
Write-Host "To sign it for Play Console, run this command manually:" -ForegroundColor Green
Write-Host "jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 -keystore operatorgame-release.jks $AabFinal operatorgame" -ForegroundColor Yellow
Write-Host "=============================================" -ForegroundColor Green
