#!/usr/bin/env python3
"""
Automated UI Screenshot Analysis using Claude Vision API.
Analyzes all screenshots in a folder and generates structured feedback.
"""

import json
import base64
import os
import sys
from pathlib import Path
from dotenv import load_dotenv
from anthropic import Anthropic

# Safely load local .env variables into os.environ
load_dotenv()

def encode_image(image_path):
    """Encode PNG to base64 for API."""
    with open(image_path, 'rb') as f:
        return base64.standard_b64encode(f.read()).decode('utf-8')

def analyze_screenshot(client, image_path, tab_name):
    """
    Analyze single screenshot using Claude vision.
    Returns structured JSON feedback.
    """
    
    image_data = encode_image(image_path)
    
    system_prompt = """You are a mobile UI/UX expert analyzing OperatorGame screenshots.

Your job is to examine the screenshot and provide STRUCTURED JSON feedback on:

1. **Visual Elements Identified:**
   - List every UI component you see (buttons, cards, text, icons, progress bars, etc.)
   - Describe their position and size

2. **Layout Analysis:**
   - Overall layout structure (grid, sidebar, panels)
   - Alignment issues (elements not aligned properly)
   - Spacing problems (gaps too large/small, inconsistent margins)
   - Visual hierarchy (is important content prominent?)

3. **Readability & Contrast:**
   - Font sizes (are they readable on mobile?)
   - Text contrast against backgrounds
   - Icon clarity

4. **Touch Targets:**
   - Estimated sizes of tappable elements
   - Are they large enough for mobile (minimum 44x44 dp recommended)?

5. **Issues Found:**
   - Specific layout problems
   - Color/contrast issues
   - Alignment discrepancies
   - Any elements that appear broken/clipped

6. **Egui Constraint Recommendations:**
   - Specific padding/margin changes
   - Font size adjustments
   - Layout restructuring suggestions
   - Width/height constraint fixes

ALWAYS return your response as VALID JSON with this exact structure:
{
  "tab": "01_roster_collection",
  "elements_identified": [
    {"name": "card_grid", "type": "grid", "position": "center", "elements_count": 6},
    {"name": "xp_bar", "type": "progress_bar", "position": "card_bottom", "width_estimate_dp": 180}
  ],
  "layout_structure": {
    "header": "fixed_top_56dp",
    "content": "scrollable_center",
    "footer": "bottom_tab_bar_56dp",
    "sidebar": "left_80dp"
  },
  "spacing_issues": [
    {"location": "card_grid_left", "problem": "2dp margin overflow", "severity": "minor"},
    {"location": "bottom_padding", "problem": "last row cuts off", "severity": "major"}
  ],
  "readability": {
    "font_sizes_ok": false,
    "issues": [{"element": "xp_text", "current_size_estimate": "9pt", "problem": "too small", "recommended": "11pt"}]
  },
  "touch_targets": [
    {"element": "card", "estimated_size_dp": "200x240", "status": "acceptable"},
    {"element": "xp_bar", "estimated_size_dp": "180x16", "status": "too_small_height"}
  ],
  "critical_issues": [
    "Card grid left margin overflow by ~2dp",
    "XP text unreadable at 9pt on 412dp width"
  ],
  "egui_constraints": [
    {"widget": "card_grid", "current": "spacing: 16dp", "recommended": "spacing: 14dp", "reason": "fix left overflow"},
    {"widget": "xp_text", "current": "font_size: 9pt", "recommended": "font_size: 11pt", "reason": "readability"},
    {"widget": "bottom_section", "current": "height: 1680dp", "recommended": "height: 1688dp", "reason": "prevent clipping"}
  ],
  "priority_fixes": [
    "Reduce card grid margin from 16dp to 14dp",
    "Increase XP font from 9pt to 11pt",
    "Increase bottom section height to 1688dp"
  ],
  "notes": "Overall layout is clean but has minor spacing inconsistencies."
}

Do NOT include any text outside the JSON block. Return ONLY valid JSON."""

    user_message = f"""Analyze this OperatorGame screenshot (Tab: {tab_name}).

Provide detailed structural and visual feedback in the JSON format specified."""

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
    
    # Extract JSON from response
    response_text = response.content[0].text
    try:
        analysis = json.loads(response_text)
    except json.JSONDecodeError:
        # If response isn't pure JSON, try to extract JSON block
        import re
        json_match = re.search(r'\{.*\}', response_text, re.DOTALL)
        if json_match:
            analysis = json.loads(json_match.group())
        else:
            analysis = {"error": "Failed to parse vision response", "raw": response_text}
    
    return analysis

def process_screenshot_folder(folder_path):
    """Process all screenshots in a folder and generate comprehensive report."""
    
    client = Anthropic()
    folder = Path(folder_path)
    
    if not folder.exists():
        print(f"ERROR: Folder not found: {folder}")
        sys.exit(1)
    
    png_files = sorted(folder.glob("*.png"))
    
    if not png_files:
        print(f"ERROR: No PNG files found in {folder}")
        sys.exit(1)
    
    print(f"\n{'='*60}")
    print(f"  Automated UI Screenshot Analysis")
    print(f"  Folder: {folder.name}")
    print(f"  Screenshots: {len(png_files)}")
    print(f"{'='*60}\n")
    
    all_analyses = []
    
    for png_file in png_files:
        tab_name = png_file.stem
        print(f"[{len(all_analyses)+1}/{len(png_files)}] Analyzing {png_file.name}...")
        
        try:
            analysis = analyze_screenshot(client, str(png_file), tab_name)
            all_analyses.append(analysis)
            
            # Save individual analysis
            analysis_file = png_file.with_stem(png_file.stem + "_analysis").with_suffix('.json')
            with open(analysis_file, 'w') as f:
                json.dump(analysis, f, indent=2)
            
            print(f"  ✅ Analyzed and saved to {analysis_file.name}\n")
            
        except Exception as e:
            print(f"  ❌ Error analyzing {png_file.name}: {e}\n")
    
    # Generate composite report
    print(f"\n{'='*60}")
    print(f"  Generating Comprehensive Report")
    print(f"{'='*60}\n")
    
    report = generate_composite_report(all_analyses, folder)
    
    report_path = folder / "UI_FEEDBACK_REPORT.md"
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)
    
    print(f"✅ Report generated: {report_path}")
    return report_path

def generate_composite_report(analyses, folder):
    """Generate markdown report combining all analyses."""
    
    report = f"""# OperatorGame UI Analysis Report

**Generated:** Phase F.3 Automated Vision Analysis  
**Folder:** {folder.name}  
**Screenshots Analyzed:** {len(analyses)}

---

## Executive Summary

"""
    
    # Count issues by severity
    critical_count = 0
    major_count = 0
    minor_count = 0
    
    for analysis in analyses:
        if 'critical_issues' in analysis:
            critical_count += len(analysis.get('critical_issues', []))
        if 'spacing_issues' in analysis:
            for issue in analysis.get('spacing_issues', []):
                if issue.get('severity') == 'major':
                    major_count += 1
                elif issue.get('severity') == 'minor':
                    minor_count += 1
    
    report += f"- **Critical Issues:** {critical_count}\n"
    report += f"- **Major Issues:** {major_count}\n"
    report += f"- **Minor Issues:** {minor_count}\n\n"
    
    # Per-tab analysis
    report += "## Tab-by-Tab Analysis\n\n"
    
    for i, analysis in enumerate(analyses, 1):
        tab = analysis.get('tab', f'Tab_{i}')
        report += f"### {i}. {tab}\n\n"
        
        if 'elements_identified' in analysis:
            report += f"**Elements:** {len(analysis['elements_identified'])} components identified\n\n"
        
        if 'critical_issues' in analysis and analysis['critical_issues']:
            report += "**Critical Issues:**\n"
            for issue in analysis['critical_issues']:
                report += f"- {issue}\n"
            report += "\n"
        
        if 'priority_fixes' in analysis and analysis['priority_fixes']:
            report += "**Priority Fixes:**\n"
            for fix in analysis['priority_fixes'][:3]:
                report += f"1. {fix}\n"
            report += "\n"
        
        if 'egui_constraints' in analysis:
            if isinstance(analysis['egui_constraints'], list):
                report += "**Egui Constraint Changes:**\n```rust\n"
                for constraint in analysis['egui_constraints'][:5]:
                    report += f"// {constraint.get('reason', '')}\n"
                    report += f"// Current:  {constraint.get('current', '')}\n"
                    report += f"// Change to: {constraint.get('recommended', '')}\n\n"
                report += "```\n\n"
    
    # Cross-tab patterns
    report += "## Cross-Tab Patterns\n\n"
    
    # Extract common issues
    all_issues = []
    for analysis in analyses:
        all_issues.extend(analysis.get('critical_issues', []))
    
    if all_issues:
        report += "**Recurring Issues:**\n"
        from collections import Counter
        # Only counting exact matches, so formatting issues might not cluster well
        # but it gives a rough idea
        issue_counts = Counter(str(iss) for iss in all_issues)
        for issue, count in issue_counts.most_common(5):
            if count > 1:
                report += f"- {issue} (found in {count} tabs)\n"
        report += "\n"
    
    # Implementation roadmap
    report += "## Implementation Roadmap\n\n"
    report += "### Phase 1: Quick Wins (1-2 hours)\n"
    report += "- Standardize margins/padding across all tabs\n"
    report += "- Increase small font sizes for readability\n"
    report += "- Fix touch target sizes\n\n"
    
    report += "### Phase 2: Layout Refinement (3-4 hours)\n"
    report += "- Adjust grid spacing and alignment\n"
    report += "- Fix overflow/clipping issues\n"
    report += "- Refine visual hierarchy\n\n"
    
    report += "### Phase 3: Polish (2-3 hours)\n"
    report += "- Color and contrast optimization\n"
    report += "- Animation/transition smoothing\n"
    report += "- Final screenshot validation\n\n"
    
    report += "---\n\n"
    report += "**Detailed analysis JSON files:**\n"
    report += f"- `*_analysis.json` files in this {folder.name} folder contain full structural breakdown per tab\n"
    
    return report

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python auto_analyze_screenshots.py <screenshot_folder>")
        sys.exit(1)
    
    folder = sys.argv[1]
    process_screenshot_folder(folder)
