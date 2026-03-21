param(
    [string]$Label = "build_v1.0"
)

# Grab the most recent F.3 UI Scrape Output
$latestFolder = Get-ChildItem "screenshots_uitree_*" -Directory | Sort-Object CreationTime -Descending | Select-Object -First 1

if (-not $latestFolder) {
    Write-Error "❌ No screenshots found in root directory. Run capture_ui_tree.ps1 first."
    exit 1
}

$baselineDir = "baselines\$Label"
if (Test-Path $baselineDir) { 
    Remove-Item -Recurse -Force $baselineDir 
}
New-Item -ItemType Directory -Path $baselineDir | Out-Null

# Copy PNGs and JSONs to the locked immutable baseline array
Copy-Item -Path "$($latestFolder.FullName)\*" -Destination $baselineDir -Recurse

# Generate an exact SHA256 cryptographic manifest to prevent pipeline drift
$manifest = @{
    version = $Label
    captured_at = (Get-Date).ToString("o")
    source_folder = $latestFolder.Name
    files = @()
}

Get-ChildItem -Path $baselineDir -File | ForEach-Object {
    $manifest.files += @{
        name = $_.Name
        size = $_.Length
        hash = (Get-FileHash $_.FullName -Algorithm SHA256).Hash
    }
}

$manifest | ConvertTo-Json -Depth 3 | Out-File -FilePath "$baselineDir\baseline_manifest.json" -Encoding UTF8
Write-Host "✅ Baseline locked: $Label -> $baselineDir\baseline_manifest.json" -ForegroundColor Green
