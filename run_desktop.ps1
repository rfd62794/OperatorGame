# run_desktop.ps1
# Launch OperatorGame in standard desktop mode

$env:OPERATOR_MOBILE_EMU = ""
Write-Host "🖥️ Building OperatorGame in Desktop mode..." -ForegroundColor Cyan
cargo run --release
