# OPERATOR: MOTO G 2025 HARDWARE HANDSHAKE
Write-Host "--- Scanning Moto G (Power/5G) for Linker/Memory Errors ---" -ForegroundColor Cyan
adb logcat -c  # Clear buffer
adb logcat *:E # Stream only Errors to terminal
