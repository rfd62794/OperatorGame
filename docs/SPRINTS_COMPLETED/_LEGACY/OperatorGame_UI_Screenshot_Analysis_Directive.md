# OperatorGame UI Screenshot Analysis & Auto-Annotation Directive

**Status:** Phase F — UI Mapping & Layout Validation  
**Target:** Build automated system to translate raw Moto G screenshots into machine-readable UI element maps  
**Problem:** AI agents struggle with visual input; need structured text descriptions instead

---

## Part A: Screenshot Capture & Metadata Pipeline

**Agent Action:** Enhance `capture_screenshots.ps1` to include metadata extraction

Create new script: `C:\Github\OperatorGame\scripts\analyze_screenshots.ps1`

**Purpose:** For each captured PNG, generate companion `.json` file with:
1. Screenshot metadata (device, resolution, timestamp, tab name)
2. Placeholder for manual UI element mapping
3. Structure ready for Phase B (automated OCR/vision analysis)

**Script Logic:**

```powershell
param(
    [string]$ScreenshotFolder = "screenshots_*",
    [switch]$AutoAnalyze = $false
)

# Find latest screenshot folder
$folder = Get-ChildItem -Path $PSScriptRoot -Filter "screenshots_*" -Directory | Sort-Object LastWriteTime -Descending | Select-Object -First 1

if (-not $folder) {
    Write-Error "No screenshot folder found"
    exit 1
}

Write-Host "Analyzing screenshots in: $($folder.FullName)" -ForegroundColor Cyan

# For each PNG, create metadata JSON
Get-ChildItem -Path $folder.FullName -Filter "*.png" | ForEach-Object {
    $pngFile = $_
    $jsonPath = $pngFile.FullName -replace '\.png$', '.json'
    
    # Get image dimensions (requires ImageMagick or similar)
    # For now, assume Moto G 2025: 412×1900 dp
    
    $metadata = @{
        "screenshot" = @{
            "filename" = $pngFile.Name
            "tab" = $pngFile.BaseName
            "timestamp" = $pngFile.LastWriteTime.ToString("o")
            "device" = "Moto G 2025"
            "resolution_dp" = @{
                "width" = 412
                "height" = 1900
            }
            "file_size_kb" = [math]::Round($pngFile.Length / 1KB, 1)
        }
        "ui_elements" = @{
            "header" = @{
                "location" = "top"
                "height_dp" = 56
                "elements" = @()
            }
            "content" = @{
                "location" = "middle"
                "height_dp" = 1818
                "elements" = @()
            }
            "footer" = @{
                "location" = "bottom"
                "height_dp" = 56
                "elements" = @()
            }
            "subtabs" = @{
                "location" = "below_header"
                "height_dp" = 40
                "elements" = @()
            }
        }
        "notes" = @{
            "layout_issues" = @()
            "alignment_problems" = @()
            "touch_target_concerns" = @()
        }
    } | ConvertTo-Json -Depth 10
    
    $metadata | Set-Content $jsonPath
    Write-Host "  Generated: $($jsonPath | Split-Path -Leaf)" -ForegroundColor Green
}

Write-Host "`nScreenshot analysis metadata ready for Phase B (OCR/vision processing)." -ForegroundColor Cyan
```

---

## Part B: Manual UI Element Mapping (Structured Annotation)

**Agent Action:** Create interactive annotation template

Create script: `C:\Github\OperatorGame\scripts\annotate_ui_elements.ps1`

**Purpose:** Robert manually walks through each screenshot and documents:
- What UI elements are visible (buttons, text, cards, etc.)
- Where they are positioned (coordinates or grid-based)
- What they do (function/intent)
- What's wrong (alignment, spacing, visibility issues)

**Output Format:** Structured JSON that agents can **read without vision**.

**Example annotation (Roster tab):**

```json
{
  "tab": "01_Roster_Collection",
  "elements": [
    {
      "id": "slime_card_grid",
      "type": "grid",
      "location": "main_content_area",
      "position": { "x": 0, "y": 80, "width": 412, "height": 1680 },
      "description": "Horizontal-wrapped grid of slime operator cards",
      "items_visible": 6,
      "items_per_row": 2,
      "card_dimensions": { "width": 200, "height": 240 },
      "spacing": { "horizontal_gap": 6, "vertical_gap": 8 },
      "issues": [
        "Cards slightly overlap on left edge (margin calculation off by 2dp)",
        "Bottom padding insufficient — last row cuts off 8dp",
        "XP bar text is 9pt, unreadable on mobile"
      ]
    },
    {
      "id": "squad_staging_area",
      "type": "section",
      "location": "below_grid",
      "position": { "x": 0, "y": 1760, "width": 412, "height": 100 },
      "description": "Squad selection UI (max 3 slimes). Shows selected slime portraits.",
      "selected_count": 2,
      "max_capacity": 3,
      "issues": [
        "Spacing between portrait frames is inconsistent",
        "Selected indicator (checkmark) is hard to see on green background"
      ]
    },
    {
      "id": "header_bar",
      "type": "header",
      "location": "top_fixed",
      "position": { "x": 0, "y": 0, "width": 412, "height": 56 },
      "description": "Top status bar: 'Roster' title, inventory icon, settings icon",
      "elements": [
        { "name": "title", "text": "Roster", "alignment": "left", "font_size": 18 }
      ],
      "issues": []
    }
  ]
}
```

---

## Part C: Automated Vision Processing (Phase B Integration)

**Agent Action:** Create Python script for Claude vision API integration

Create script: `C:\Github\OperatorGame\scripts\analyze_screenshot_vision.py`

**Purpose:** Feed screenshots + manual annotations to Claude's vision API for structured feedback

**Python Script:**

```python
#!/usr/bin/env python3
"""
Analyze OperatorGame UI screenshots using Claude vision + structured annotations.
Generates improvement recommendations without requiring agents to parse visuals manually.
"""

import json
import base64
import os
from pathlib import Path
from anthropic import Anthropic

def load_annotation(json_path):
    """Load manual UI annotation."""
    try:
        with open(json_path, 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        return None

def encode_image(image_path):
    """Encode PNG to base64 for API."""
    with open(image_path, 'rb') as f:
        return base64.standard_b64encode(f.read()).decode('utf-8')

def analyze_screenshot(image_path, annotation_json=None, client=None):
    """
    Send screenshot + annotation to Claude for analysis.
    Returns structured improvement recommendations.
    """
    if not client:
        client = Anthropic()
    
    image_data = encode_image(image_path)
    annotation_text = json.dumps(annotation_json, indent=2) if annotation_json else "No annotation provided"
    
    system_prompt = """You are a mobile UI/UX expert analyzing OperatorGame screenshots.
    
Your job is to:
1. Identify UI layout issues (misalignment, overflow, spacing)
2. Flag touch target problems (buttons too small, hit targets insufficient)
3. Point out readability issues (text too small, contrast problems)
4. Suggest specific egui constraint fixes in Rust pseudo-code

ALWAYS return your analysis as structured JSON with these fields:
- layout_issues: [{location, problem, severity: "critical"|"major"|"minor"}]
- touch_targets: [{element, current_size_dp, recommended_size_dp, issue}]
- readability: [{element, issue, fix}]
- egui_constraints: [{widget, current_constraint, recommended_constraint, reason}]
- priority_fixes: [list of top 3 fixes to implement first]

Do NOT make vague observations. Be specific with coordinates, sizes, and actionable fixes."""

    user_message = f"""Analyze this OperatorGame UI screenshot.

Tab: {Path(image_path).stem}

Manual Annotation (if available):
{annotation_text}

Screenshot follows. Provide structured analysis in JSON format."""

    response = client.messages.create(
        model="claude-3-5-sonnet-20241022",
        max_tokens=2000,
        system=system_prompt,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": "image/png",
                            "data": image_data
                        }
                    },
                    {
                        "type": "text",
                        "text": user_message
                    }
                ]
            }
        ]
    )
    
    return response.content[0].text

def process_screenshot_folder(folder_path):
    """Process all screenshots in a folder."""
    client = Anthropic()
    results = {}
    
    png_files = sorted(Path(folder_path).glob("*.png"))
    
    for png_file in png_files:
        json_file = png_file.with_suffix('.json')
        annotation = load_annotation(json_file)
        
        print(f"Analyzing {png_file.name}...")
        analysis = analyze_screenshot(str(png_file), annotation, client)
        
        # Save analysis
        analysis_file = png_file.with_stem(png_file.stem + "_analysis").with_suffix('.json')
        with open(analysis_file, 'w') as f:
            f.write(analysis)
        
        results[png_file.name] = analysis
        print(f"  ✅ Saved to {analysis_file.name}")
    
    return results

if __name__ == "__main__":
    import sys
    
    if len(sys.argv) < 2:
        print("Usage: python analyze_screenshot_vision.py <screenshot_folder>")
        sys.exit(1)
    
    folder = sys.argv[1]
    process_screenshot_folder(folder)
```

---

## Part D: Aggregated UI Feedback Report

**Agent Action:** Create synthesis script to combine all analyses

Create script: `C:\Github\OperatorGame\scripts\generate_ui_report.ps1`

**Purpose:** Aggregate all vision analyses + annotations into a single actionable report

**Output:** `UI_FEEDBACK_REPORT.md`

**Format:**

```markdown
# UI Feedback Report — Phase F Analysis

## Executive Summary
- Total tabs analyzed: 6
- Critical issues: 3
- Major issues: 8
- Minor issues: 12

## Tab-by-Tab Breakdown

### 01_Roster_Collection
**Status:** [CRITICAL ISSUES]

#### Layout Issues
1. **Card grid overflow**
   - Problem: Cards on left edge are cut off by 2dp
   - Location: Grid X coordinate
   - Fix: Adjust egui spacing constraint from `16dp` to `14dp`
   - Priority: CRITICAL

2. **Bottom padding insufficient**
   - Problem: Last row of cards truncated
   - Current height: 1680dp
   - Fix: Increase to 1688dp or wrap grid differently
   - Priority: MAJOR

#### Touch Targets
- XP progress bar: Currently 18pt height, recommend 24pt minimum
- Card tap zones: Currently 200×240dp, acceptable but verify 24pt padding

#### Readability
- Font sizes: 9pt XP text is unreadable on 412dp width
- Contrast: Green text on dark background acceptable
- Recommendation: Bump XP font to 10-11pt, add subtle shadow

#### Recommended egui Changes
```rust
// Current
let card = egui::Frame::none()
    .inner_margin(16.0)
    .show(ui, |ui| { ... });

// Recommended
let card = egui::Frame::none()
    .inner_margin(14.0)  // Reduce left/right margin
    .show(ui, |ui| {
        ui.vertical(|ui| {
            ui.set_min_height(1688.0);  // Explicit height
            // ... grid content
        });
    });
```

#### Priority Fixes (This Tab)
1. Fix card grid X overflow (2dp)
2. Increase XP text to 11pt
3. Verify last row padding

---

### 02_Roster_Breeding
[Similar structure...]

---

## Cross-Tab Patterns

### Consistent Issues Found
- Bottom tab bar has 2px misalignment across all tabs
- Header spacing inconsistent: sometimes 8dp, sometimes 10dp
- Sub-tab styling lacks visual hierarchy

### Quick Wins
1. Standardize all margins to 12dp (currently mixed 8/10/14/16)
2. Fix bottom tab bar Y offset (apply to all tabs at once)
3. Increase all small font sizes by 1pt

---

## Implementation Roadmap

### Phase F.0 (Quick Wins — 2 hours)
- [ ] Standardize margins
- [ ] Fix tab bar alignment
- [ ] Bump small fonts

### Phase F.1 (Layout Refinement — 4 hours)
- [ ] Fix card grid overflow
- [ ] Adjust padding on all tabs
- [ ] Verify touch targets

### Phase F.2 (Polish — 2 hours)
- [ ] Color refinement
- [ ] Animation smoothing
- [ ] Final screenshot sweep

---

## Next Action
Robert: Review recommendations, prioritize critical fixes, trigger Phase F.1 implementation.
```

---

## Part E: Iteration Loop Setup

**Agent Action:** Create automated iteration script

**Purpose:** Compile → Deploy → Capture → Analyze → Report → Repeat

```powershell
# C:\Github\OperatorGame\scripts\ui_iteration_loop.ps1

param(
    [int]$Cycles = 3,
    [switch]$SkipBuild = $false
)

for ($i = 1; $i -le $Cycles; $i++) {
    Write-Host "`n╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║  UI Iteration Cycle $i/$Cycles" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    
    if (-not $SkipBuild) {
        Write-Host "[1/4] Building APK..." -ForegroundColor Yellow
        & "$PSScriptRoot\..\build_android.ps1"
    }
    
    Write-Host "[2/4] Deploying to Moto G..." -ForegroundColor Yellow
    & "$PSScriptRoot\..\deploy_moto.ps1"
    
    Write-Host "[3/4] Capturing screenshots..." -ForegroundColor Yellow
    & "$PSScriptRoot\..\capture_screenshots.ps1"
    
    Write-Host "[4/4] Analyzing with vision..." -ForegroundColor Yellow
    $latestFolder = Get-ChildItem -Path $PSScriptRoot -Filter "screenshots_*" -Directory | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    & python.exe "$PSScriptRoot\analyze_screenshot_vision.py" $latestFolder.FullName
    
    Write-Host "`nCycle $i complete. Review 'UI_FEEDBACK_REPORT.md'" -ForegroundColor Green
    Write-Host "Press Enter to continue or Ctrl+C to stop..." -ForegroundColor Gray
    Read-Host
}

Write-Host "`n✅ UI iteration complete. All screenshots analyzed." -ForegroundColor Green
```

---

## Implementation Checklist

**Phase B.1: Screenshot Metadata (2 hours)**
- [ ] Enhance `capture_screenshots.ps1` with JSON metadata generation
- [ ] Create `analyze_screenshots.ps1` for automation
- [ ] Verify metadata structure on first run

**Phase B.2: Manual Annotation Template (4 hours)**
- [ ] Create `annotate_ui_elements.ps1` interactive guide
- [ ] Robert manually annotates each tab's screenshot
- [ ] Output: 6 `.json` files (one per tab) with UI element maps

**Phase B.3: Vision API Integration (4 hours)**
- [ ] Create `analyze_screenshot_vision.py` with Claude vision API calls
- [ ] Implement structured JSON output from vision analysis
- [ ] Test on single screenshot, validate output format

**Phase B.4: Report Generation (2 hours)**
- [ ] Create `generate_ui_report.ps1` aggregation script
- [ ] Produce `UI_FEEDBACK_REPORT.md` with cross-tab analysis
- [ ] Format with actionable egui constraint recommendations

**Phase B.5: Iteration Loop (2 hours)**
- [ ] Create `ui_iteration_loop.ps1` for automated cycles
- [ ] Test full cycle: build → deploy → capture → analyze → report
- [ ] Ready for 3-5 iteration cycles

**Total estimated time:** ~14 hours (can parallelize with agent + manual annotation)

---

## Success Criteria

✅ Agents can read UI analysis **without looking at images**  
✅ All recommendations are **specific and actionable** (not vague)  
✅ Report includes **egui pseudo-code** for fixes  
✅ Iteration loop is **fully automated** (build → feedback in ~15 min)  
✅ Each cycle produces **new screenshots + new analysis**  
✅ Issue prioritization is **data-driven** (severity ratings)  

---

## Output Artifacts

After each iteration:
- `screenshots_YYYYMMDD_HHMMSS/` — Raw PNGs
- `screenshots_YYYYMMDD_HHMMSS/*.json` — Manual annotations
- `screenshots_YYYYMMDD_HHMMSS/*_analysis.json` — Vision analysis
- `UI_FEEDBACK_REPORT.md` — Aggregated actionable report
- `ui_iteration_log.txt` — History of all cycles

**This becomes your UI design spec.**

---

## Next Steps

If approved, agent can:
1. Build Parts A-B immediately (metadata + annotation template)
2. Robert manually annotates 6 tabs (~30 min per tab = 3 hours)
3. Agent builds Parts C-D (vision API + report generation)
4. Execute Part E (iteration loop) for 3-5 cycles
5. Lock UI spec and move to implementation

**Proceed with this system?**
