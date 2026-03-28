# run_mobile.ps1
# Launch OperatorGame in mobile emulation mode (Moto G 2025 profile)

$env:OPERATOR_MOBILE_EMU = "1"
Write-Host "🔧 Building OperatorGame in Mobile Emulation mode..." -ForegroundColor Cyan
cargo run --release
Write-Host "✓ Mobile emulation session closed." -ForegroundColor Green
