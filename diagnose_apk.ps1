<#
.SYNOPSIS
Inspects the OperatorGame APK for required native libraries and reports any missing dependencies.

.EXAMPLE
.\diagnose_apk.ps1
.\diagnose_apk.ps1 -ApkPath "C:\path\to\custom.apk"
#>
param(
    [string]$ApkPath = "operatorgame-release.apk"
)

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  OperatorGame APK Diagnostic Tool" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

if (-not (Test-Path $ApkPath)) {
    Write-Host "ERROR: APK not found: $ApkPath" -ForegroundColor Red
    exit 1
}

$resolvedPath = (Resolve-Path $ApkPath).Path
$sizeMB = [math]::Round((Get-Item $resolvedPath).Length / 1MB, 2)
Write-Host ""
Write-Host "APK: $resolvedPath ($sizeMB MB)" -ForegroundColor DarkGray

# ---------------------------------------------------------------------------
# 1. Enumerate all .so files
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[1/3] Inspecting native libraries (.so files)..." -ForegroundColor Cyan

Add-Type -AssemblyName System.IO.Compression.FileSystem
$apk = [IO.Compression.ZipFile]::OpenRead($resolvedPath)

$soFiles = $apk.Entries | Where-Object { $_.Name -like "*.so" }

if ($soFiles.Count -eq 0) {
    Write-Host "  CRITICAL: No .so files found in APK." -ForegroundColor Red
    Write-Host "  The native library was not bundled -- this is the crash cause." -ForegroundColor Red
    $apk.Dispose()
    exit 1
}

Write-Host "  Found $($soFiles.Count) native library file(s):" -ForegroundColor Green
$soFiles | ForEach-Object {
    $sizeKB = [math]::Round($_.Length / 1KB, 1)
    Write-Host "    $($_.FullName)   ($sizeKB KB)"
}

# ---------------------------------------------------------------------------
# 2. Check critical dependencies
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[2/3] Checking critical dependencies..." -ForegroundColor Cyan

$critical = @(
    "liboperator.so",        # Main Rust native library (name = "operator" in Cargo.toml)
    "libc++_shared.so"       # C++ shared runtime — required when stl = "c++_shared"
)

$anyCriticalMissing = $false
foreach ($lib in $critical) {
    $found = $soFiles | Where-Object { $_.Name -eq $lib }
    if ($found) {
        Write-Host "  PRESENT : $lib" -ForegroundColor Green
    } else {
        Write-Host "  MISSING : $lib  <-- likely crash cause" -ForegroundColor Red
        $anyCriticalMissing = $true
    }
}

# ---------------------------------------------------------------------------
# 3. Full lib/ directory tree
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[3/3] Full lib/ directory tree:" -ForegroundColor Cyan

$libEntries = $apk.Entries | Where-Object { $_.FullName -like "lib/*" } | Sort-Object FullName
if ($libEntries.Count -eq 0) {
    Write-Host "  WARNING: No lib/ directory found in APK." -ForegroundColor Yellow
} else {
    $libEntries | ForEach-Object {
        $sizeKB = [math]::Round($_.Length / 1KB, 1)
        Write-Host "  $($_.FullName)   ($sizeKB KB)"
    }
}

$apk.Dispose()

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
if ($anyCriticalMissing) {
    Write-Host "  RESULT: Critical libraries missing -- see above." -ForegroundColor Red
    Write-Host ""
    Write-Host "  RECOMMENDED FIX:" -ForegroundColor Yellow
    Write-Host "  If libc++_shared.so is missing, change Cargo.toml:" -ForegroundColor Yellow
    Write-Host "    stl = `"c++_shared`"  ->  stl = `"c++_static`"" -ForegroundColor Yellow
    Write-Host "  Then run: cargo clean && .\build_android.ps1" -ForegroundColor Yellow
} else {
    Write-Host "  RESULT: All critical libraries present." -ForegroundColor Green
    Write-Host "  If app still crashes, check logcat for linker/dlopen errors." -ForegroundColor DarkGray
}
Write-Host "============================================================" -ForegroundColor Cyan
