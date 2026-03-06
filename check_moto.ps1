# OPERATOR: MOTO G 2025 HARDWARE HANDSHAKE
$ADB = "C:\Users\cheat\AppData\Local\Android\Sdk\platform-tools\adb.exe"
Write-Host "--- Scanning Moto G (Power/5G) for Linker/Memory Errors ---" -ForegroundColor Cyan
& $ADB logcat -c  # Clear buffer
& $ADB logcat *:E # Stream only Errors to terminal
