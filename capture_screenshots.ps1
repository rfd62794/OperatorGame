Import-Module -Name "$PSScriptRoot\lib\OperatorDeviceTools\OperatorDeviceTools.psm1" -Force

Write-Host "------------------------------------------------------------" -ForegroundColor Cyan
Write-Host "  OperatorGame Mobile Screenshot Capture" -ForegroundColor Cyan
Write-Host "  Automated UI tab screenshots from Moto G" -ForegroundColor Cyan
Write-Host "------------------------------------------------------------" -ForegroundColor Cyan

$dev = Connect-Device
Write-Host "[OK] Connected Target: $($dev.ToString())" -ForegroundColor Green

$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$outDir = Join-Path $PSScriptRoot "screenshots_$timestamp"
New-Item -ItemType Directory -Path $outDir -Force | Out-Null
Write-Host "[OK] Output directory: $outDir`n" -ForegroundColor Green

Write-Host "Verifying app deployment and residency..." -ForegroundColor Yellow
$pidNum = Launch-OperatorApp -Device $dev -TimeoutSeconds 8
Write-Host "[OK] Architecture online (PID: $pidNum)`n" -ForegroundColor Green

$tabs = @(
    @{ Name = "01_Roster_Collection"; Desc = "Slime cards, staging UI"; X=50; Y=1870; SubX=150; SubY=300 },
    @{ Name = "02_Roster_Breeding"; Desc = "Incubator, breeding pair UI"; X=50; Y=1870; SubX=250; SubY=300 },
    @{ Name = "03_Missions_Active"; Desc = "Current deployment map"; X=250; Y=1870; SubX=150; SubY=300 },
    @{ Name = "04_Missions_QuestBoard"; Desc = "Available contracts"; X=250; Y=1870; SubX=250; SubY=300 },
    @{ Name = "05_Map_Overview"; Desc = "Strategic region view"; X=450; Y=1870; SubX=$null; SubY=$null },
    @{ Name = "06_Logs_Main"; Desc = "Text output, resource drops"; X=650; Y=1870; SubX=$null; SubY=$null }
)

foreach ($tab in $tabs) {
    Write-Host "[TAB] $($tab.Name)" -ForegroundColor Cyan
    
    Invoke-DeviceTap -Device $dev -X $tab.X -Y $tab.Y -DelayMs 1000
    if ($tab.SubX -and $tab.SubY) {
        Invoke-DeviceTap -Device $dev -X $tab.SubX -Y $tab.SubY -DelayMs 800
    }
    
    Write-Host "  $($tab.Desc)" -ForegroundColor Gray
    $outPath = Join-Path $outDir "$($tab.Name).png"
    
    $shot = Capture-Screenshot -Device $dev -OutputPath $outPath -Label $tab.Name
    Write-Host "  [OK] Captured Artifact: $($shot.FilePath) ($($shot.SizeKb) KB)`n" -ForegroundColor Green
}

Write-Host "Sweep complete. Artifacts stored in $outDir." -ForegroundColor Cyan
