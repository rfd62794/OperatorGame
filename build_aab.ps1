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
$ApkUnsigned = "target/release/apk/operator.apk"

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

# 3. Build APK
Write-Host "Building Rust payload with cargo apk..." -ForegroundColor Cyan
cargo apk build --release

if (-not (Test-Path $ApkUnsigned)) {
    Write-Host "Error: Could not find output APK at $ApkUnsigned." -ForegroundColor Red
    exit 1
}

# 3.5 Patch AndroidManifest.xml for API 35
Write-Host "Patching AndroidManifest.xml API targets (cargo-apk hardcodes 30)..." -ForegroundColor Cyan
$ManifestPath = "target\release\apk\AndroidManifest.xml"
$ManifestContent = [System.IO.File]::ReadAllText($ManifestPath)
$ManifestContent = $ManifestContent -replace 'android:targetSdkVersion="\d+"', 'android:targetSdkVersion="35"'
$ManifestContent = $ManifestContent -replace 'android:minSdkVersion="\d+"', 'android:minSdkVersion="26"'
[System.IO.File]::WriteAllText($ManifestPath, $ManifestContent, [System.Text.Encoding]::UTF8)

# 4. Extract Proto APK and Inject Fixed Manifest
$ProtoApk = "target\proto.zip"
$BaseZip = "target\base.zip"
$AabBase = "target\aab_base"

Write-Host "Converting APK resources to protobuf format..." -ForegroundColor Cyan
& $Aapt2 convert --output-format proto -o $ProtoApk $ApkUnsigned

$CompiledManifestDir = "target\manifest_compiled"
if (Test-Path $CompiledManifestDir) { Remove-Item -Recurse -Force $CompiledManifestDir }
New-Item -ItemType Directory -Path $CompiledManifestDir | Out-Null

& $Aapt2 compile $ManifestPath -o $CompiledManifestDir
$FlatManifest = Get-ChildItem -Path $CompiledManifestDir -Filter "*.flat" | Select-Object -First 1

& $Aapt2 link -I $AndroidJar.FullName --manifest $ManifestPath -R $FlatManifest.FullName --proto-format -o "target\manifest_proto.zip"

Write-Host "Assembling AAB module structure..." -ForegroundColor Cyan
if (Test-Path $AabBase) { Remove-Item -Recurse -Force $AabBase }
if (Test-Path $BaseZip) { Remove-Item -Force $BaseZip }

Expand-Archive -Path $ProtoApk -DestinationPath $AabBase -Force

Expand-Archive -Path "target\manifest_proto.zip" -DestinationPath "target\manifest_extract" -Force
New-Item -ItemType Directory -Path "$AabBase\manifest" -Force -ErrorAction SilentlyContinue | Out-Null
Move-Item -Path "target\manifest_extract\AndroidManifest.xml" -Destination "$AabBase\manifest\AndroidManifest.xml" -Force

Remove-Item -Recurse -Force "target\manifest_extract"
Remove-Item -Force "target\manifest_proto.zip"
Remove-Item -Recurse -Force $CompiledManifestDir

if (Test-Path "$AabBase\META-INF") { Remove-Item -Recurse -Force "$AabBase\META-INF" }

Write-Host "Zipping module base.zip..." -ForegroundColor Cyan
Push-Location $AabBase
jar cMf "..\base.zip" *
Pop-Location

# 6. Build final AAB
$VersionLine = Select-String -Path "Cargo.toml" -Pattern '^version\s*=\s*"([^"]+)"' | Select-Object -First 1
$Version = if ($VersionLine) { $VersionLine.Matches.Groups[1].Value } else { "unknown" }

$OutputDir = "target\googleplay"
if (-not (Test-Path $OutputDir)) { New-Item -ItemType Directory -Path $OutputDir | Out-Null }

$AabFinal = "$OutputDir\operatorgame-release-v$Version.aab"
if (Test-Path $AabFinal) { Remove-Item -Force $AabFinal }

Write-Host "Building Android App Bundle ($AabFinal)..." -ForegroundColor Cyan
java -jar $BundleTool build-bundle --modules=$BaseZip --output=$AabFinal

# 7. Sign AAB
Write-Host "Signing AAB with jarsigner..." -ForegroundColor Cyan
jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 -keystore $Keystore $AabFinal $Alias

# Cleanup
Remove-Item $ProtoApk -ErrorAction SilentlyContinue
Remove-Item $BaseZip -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force $AabBase -ErrorAction SilentlyContinue

Write-Host "Success! Signed App Bundle ready for Play Store: $AabFinal" -ForegroundColor Green
