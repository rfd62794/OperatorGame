param()

$folder = Get-ChildItem -Path $PSScriptRoot\.. -Filter "screenshots_*" -Directory | Sort-Object LastWriteTime -Descending | Select-Object -First 1

if (-not $folder) {
    Write-Error "No screenshots folder found."
    exit 1
}

$reportPath = Join-Path $PSScriptRoot\.. "UI_FEEDBACK_REPORT.md"
$content = @("# UI Feedback Report — Phase F Analysis`n")

$analyses = Get-ChildItem -Path $folder.FullName -Filter "*_analysis.json"
if ($analyses.Count -eq 0) {
    Write-Warning "No analysis files found. Run analyze_screenshot_vision.py first."
    exit
}

$content += "## Executive Summary`n"
$content += "- Total tabs analyzed: $($analyses.Count)`n`n"
$content += "## Tab-by-Tab Breakdown`n"

foreach ($file in $analyses) {
    $tabName = $file.Name -replace "_analysis.json",""
    $content += "### $tabName`n"
    
    try {
        $rawText = Get-Content $file.FullName -Raw
        $json = $rawText | ConvertFrom-Json
        
        $content += "#### Layout Issues`n"
        if ($json.layout_issues) {
            foreach ($issue in $json.layout_issues) {
                $content += "1. **$($issue.problem)**`n"
                $content += "   - Location: $($issue.location)`n"
                $content += "   - Priority: $($issue.severity.ToUpper())`n"
            }
        } else { $content += "None reported.`n" }
        
        $content += "`n#### Touch Targets`n"
        if ($json.touch_targets) {
            foreach ($target in $json.touch_targets) {
                $content += "- $($target.element): $($target.issue) (Current: $($target.current_size_dp)dp, Rec: $($target.recommended_size_dp)dp)`n"
            }
        } else { $content += "None reported.`n" }
        
        $content += "`n#### Readability`n"
        if ($json.readability) {
            foreach ($read in $json.readability) {
                $content += "- $($read.element): $($read.issue) -> $($read.fix)`n"
            }
        } else { $content += "None reported.`n" }
        
        $content += "`n#### Recommended egui Changes`n"
        if ($json.egui_constraints) {
            $content += "```rust`n"
            foreach ($egui in $json.egui_constraints) {
                $content += "// Widget: $($egui.widget)`n"
                $content += "// Reason: $($egui.reason)`n"
                $content += "// Rec: $($egui.recommended_constraint)`n`n"
            }
            $content += "````n"
        }
        
        $content += "`n#### Priority Fixes (This Tab)`n"
        if ($json.priority_fixes) {
            foreach ($fix in $json.priority_fixes) {
                $content += "- $fix`n"
            }
        }
    } catch {
        $content += "Raw Analysis Dump (Failed to parse structured JSON):`n```json`n$rawText`n````n"
    }
    $content += "`n---`n"
}

$content += "`n## Next Action`nRobert: Review recommendations, prioritize critical fixes, trigger Phase F.1 implementation."

$content | Set-Content $reportPath -Encoding UTF8
Write-Host "Generated $reportPath" -ForegroundColor Green
