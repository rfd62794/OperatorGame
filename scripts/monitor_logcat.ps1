param(
    [string]$OutputFile = "logs\crash_report.json"
)

if (-not (Test-Path "logs")) { 
    New-Item -ItemType Directory -Path "logs" | Out-Null 
}

Write-Host "📡 Starting live ADB Logcat monitor..." -ForegroundColor Cyan
Write-Host "Listening for RUST_PANIC, FATAL EXCEPTION, and SIGSEGV... (Press Ctrl+C to stop)" -ForegroundColor DarkGray

# Clear the old buffer so we don't trip on stale crashes
adb logcat -c

# Panic signatures for Rust, Java, and NDK
$patterns = @("RUST_PANIC", "FATAL EXCEPTION", "SIGSEGV")

# Tail the stream
adb logcat | ForEach-Object {
    $line = $_
    foreach ($pattern in $patterns) {
        if ($line -match $pattern) {
            Write-Host "❌ FATAL CRASH DETECTED: $pattern" -ForegroundColor Red
            Write-Host $line -ForegroundColor Yellow
            
            $report = @{
                timestamp = (Get-Date).ToString("o")
                type = $pattern
                details = $line
            }
            $report | ConvertTo-Json | Out-File -FilePath $OutputFile -Encoding UTF8
            
            # Instantly sever the pipeline if a panic is caught
            Write-Host "Halting Pipeline..." -ForegroundColor Red
            exit 1
        }
    }
}
