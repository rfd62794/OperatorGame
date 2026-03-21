# lib/logcat_monitor.ps1
# Logcat capture, filtering, and crash detection module.
# Dot-source this file: . .\lib\logcat_monitor.ps1

function Start-LogcatStream {
    <#
    .SYNOPSIS
    Clears the logcat buffer and streams filtered output for the given package.
    If the app is running, filters by PID. Otherwise streams *:E as fallback.
    #>
    param(
        [string]$Serial,
        [string]$PackageName = "com.rfditservices.operatorgame"
    )

    $ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
    if (-not (Test-Path $ADB) -and $env:ANDROID_HOME) {
        $ADB = "$env:ANDROID_HOME\platform-tools\adb.exe"
    }

    # Clear stale buffer
    & $ADB -s $Serial logcat -c

    $appPid = (& $ADB -s $Serial shell pidof $PackageName 2>$null).Trim()

    if ($appPid) {
        Write-Host "  Streaming PID $appPid ($PackageName)" -ForegroundColor Green
        & $ADB -s $Serial logcat --pid=$appPid
    } else {
        Write-Host "  App not running -- streaming *:E fallback" -ForegroundColor Yellow
        & $ADB -s $Serial logcat *:E
    }
}

function Get-CrashDump {
    <#
    .SYNOPSIS
    Dumps the logcat buffer and returns lines matching crash/linker error patterns.
    Run this after an app crash (within a few seconds).
    #>
    param(
        [string]$Serial,
        [string]$PackageName = "com.rfditservices.operatorgame",
        [int]$MaxLines = 60
    )

    $ADB = "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe"
    if (-not (Test-Path $ADB) -and $env:ANDROID_HOME) {
        $ADB = "$env:ANDROID_HOME\platform-tools\adb.exe"
    }

    $patterns = @(
        "FATAL", "SIGKILL", "SIGSEGV", "SIGABRT",
        "linker", "dlopen", "cannot locate", "cannot open",
        "java\.lang\.", "AndroidRuntime",
        $PackageName
    )

    $combined = $patterns -join "|"

    Write-Host "Dumping logcat crash context..." -ForegroundColor Cyan
    $lines = & $ADB -s $Serial logcat -d 2>$null
    $hits  = $lines | Select-String -Pattern $combined -CaseSensitive:$false | Select-Object -Last $MaxLines

    return $hits
}

function Detect-CrashInLines {
    <#
    .SYNOPSIS
    Given an array of log lines, returns any that match crash signal patterns.
    #>
    param([string[]]$Lines)

    $crashPatterns = "FATAL|SIGKILL|SIGSEGV|SIGABRT|dlopen.*failed|cannot locate symbol|linker.*error"
    return $Lines | Where-Object { $_ -match $crashPatterns }
}
