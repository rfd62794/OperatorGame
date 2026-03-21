#!/usr/bin/env python3
"""
Automated UI Screenshot Analysis using OpenRouter DeepSeek V3.
Reuses model routing patterns from OpenAgent (pure requests to OpenRouter).
"""

import json
import base64
import os
import sys
import time
import requests
from pathlib import Path
from dotenv import load_dotenv

# Load credentials (OpenAgent pattern)
load_dotenv()
OPENROUTER_API_KEY = os.getenv("OPENROUTER_API_KEY")

if not OPENROUTER_API_KEY:
    print("ERROR: OPENROUTER_API_KEY not found in .env")
    print("Add to .env: OPENROUTER_API_KEY=sk-or-v1-xxxxx")
    sys.exit(1)

def encode_image(image_path):
    """Encode PNG to base64 for API."""
    with open(image_path, 'rb') as f:
        return base64.standard_b64encode(f.read()).decode('utf-8')

def call_openrouter_with_fallback(payload, max_retries=2):
    """OpenAgent Error Handling & Fallback logic (`call_with_retries`)"""
    headers = {
        "Authorization": f"Bearer {OPENROUTER_API_KEY}",
        "Content-Type": "application/json",
        "HTTP-Referer": "https://github.com/rfd62794/OperatorGame",
        "X-Title": "OperatorGame Vision",
    }
    url = "https://openrouter.ai/api/v1/chat/completions"
    
    last_err = None
    for attempt in range(max_retries):
        try:
            response = requests.post(url, headers=headers, json=payload, timeout=60)
            response.raise_for_status()
            
            # DeepSeek might return an error if it hits a vision wall. OpenRouter passes it.
            js = response.json()
            if "error" in js:
                raise Exception(js["error"]["message"])
                
            return js
        except Exception as e:
            last_err = e
            print(f"    [Retry {attempt+1}/{max_retries}] Primary model failed: {e}")
            time.sleep(1 ** attempt)
            
    # OpenAgent Protocol: Fallback execution path
    print("    Primary model completely failed. Routing to fallback (Claude 3.5)...")
    payload["model"] = "anthropic/claude-3.5-sonnet"
    response = requests.post(url, headers=headers, json=payload, timeout=60)
    response.raise_for_status()
    return response.json()

def analyze_screenshot(image_path, tab_name):
    """
    Analyze single screenshot using DeepSeek V3 via OpenRouter.
    Returns structured JSON feedback.
    """
    image_data = encode_image(image_path)
    
    system_prompt = """You are a mobile UI/UX expert analyzing OperatorGame screenshots.

Your job is to examine the screenshot and provide STRUCTURED JSON feedback on:

1. **Visual Elements Identified:** List every UI component (buttons, cards, text, icons, progress bars)
2. **Layout Analysis:** Grid structure, alignment issues, spacing problems, visual hierarchy
3. **Readability & Contrast:** Font sizes, text contrast, icon clarity
4. **Touch Targets:** Estimated sizes, mobile-friendliness (minimum 44x44 dp)
5. **Issues Found:** Specific layout problems, color/contrast issues, alignment discrepancies
6. **Egui Constraint Recommendations:** Padding/margin changes, font size adjustments, layout fixes

ALWAYS return VALID JSON with this structure (no preamble):
{
  "tab": "tab_name",
  "elements_identified": [...],
  "layout_structure": {...},
  "spacing_issues": [...],
  "readability": {...},
  "touch_targets": [...],
  "critical_issues": [...],
  "egui_constraints": [...],
  "priority_fixes": [...],
  "notes": "..."
}"""

    user_message = f"""Analyze this OperatorGame screenshot (Tab: {tab_name}). Provide detailed structural and visual feedback in JSON format ONLY."""

    payload = {
        "model": "deepseek/deepseek-chat",
        "messages": [
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": user_message
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": f"data:image/png;base64,{image_data}"
                        }
                    }
                ]
            }
        ]
    }

    try:
        response_data = call_openrouter_with_fallback(payload)
        
        if "choices" not in response_data:
            raise Exception("OpenRouter returned malformed JSON without choices array.")
            
        response_text = response_data["choices"][0]["message"]["content"]
        try:
            analysis = json.loads(response_text)
        except json.JSONDecodeError:
            import re
            json_match = re.search(r'\{.*\}', response_text, re.DOTALL)
            if json_match:
                analysis = json.loads(json_match.group())
            else:
                analysis = {"error": "Failed to parse response", "raw": response_text}
        
        return analysis
        
    except Exception as e:
        print(f"  Error: {e}")
        return {"error": str(e)}

def generate_composite_report(analyses, folder):
    """Generate markdown report combining all analyses."""
    report = f"""# OperatorGame UI Analysis Report\n\n**Generated:** Phase F.3 OpenRouter Analysis\n**Folder:** {folder.name}\n**Screenshots Analyzed:** {len(analyses)}\n**Model Sequence:** DeepSeek V3 -> Claude 3.5 Fallback (via OpenRouter)\n\n---\n\n## Executive Summary\n\n"""
    
    critical_count = sum(len(a.get('critical_issues', [])) for a in analyses if isinstance(a, dict))
    major_count = sum(len([i for i in a.get('spacing_issues', []) if isinstance(i, dict) and i.get('severity') == 'major']) for a in analyses if isinstance(a, dict))
    minor_count = sum(len([i for i in a.get('spacing_issues', []) if isinstance(i, dict) and i.get('severity') == 'minor']) for a in analyses if isinstance(a, dict))
    
    report += f"- **Critical Issues:** {critical_count}\n"
    report += f"- **Major Issues:** {major_count}\n"
    report += f"- **Minor Issues:** {minor_count}\n\n"
    
    report += "## Tab-by-Tab Analysis\n\n"
    for i, analysis in enumerate(analyses, 1):
        if not isinstance(analysis, dict) or 'error' in analysis:
            report += f"### {i}. Tab {i} (Analysis Failed)\n\nError: {analysis.get('error', 'Unknown error') if isinstance(analysis, dict) else 'Bad Type'}\n\n"
            continue
        
        tab = analysis.get('tab', f'Tab_{i}')
        report += f"### {i}. {tab}\n\n"
        
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
        
        if 'egui_constraints' in analysis and isinstance(analysis['egui_constraints'], list):
            report += "**Egui Constraint Changes:**\n```rust\n"
            for constraint in analysis['egui_constraints'][:5]:
                if isinstance(constraint, dict):
                    report += f"// {constraint.get('reason', '')}\n"
                    report += f"// Current:  {constraint.get('current', '')}\n"
                    report += f"// Change to: {constraint.get('recommended', '')}\n\n"
            report += "```\n\n"
    
    report += "## Implementation Roadmap\n\n### Phase 1: Quick Wins (1-2 hours)\n- Standardize margins/padding across all tabs\n- Increase small font sizes for readability\n- Fix touch target sizes\n\n### Phase 2: Layout Refinement (3-4 hours)\n- Adjust grid spacing and alignment\n- Fix overflow/clipping issues\n\n### Phase 3: Polish (2-3 hours)\n- Color and contrast optimization\n\n---\n\n**Analysis Tool:** OpenRouter DeepSeek V3 (Cost: ~$0.001 per 7 screenshots)\n"
    return report

def process_screenshot_folder(folder_path):
    """Process all screenshots in a folder using OpenRouter."""
    folder = Path(folder_path)
    if not folder.exists():
        print(f"ERROR: Folder not found: {folder}")
        sys.exit(1)
    
    png_files = sorted(folder.glob("*.png"))
    if not png_files:
        print(f"ERROR: No PNG files found in {folder}")
        sys.exit(1)
    
    print(f"\n{'='*60}\n  Automated UI Screenshot Analysis (OpenRouter DeepSeek V3)\n  Folder: {folder.name}\n  Screenshots: {len(png_files)}\n{'='*60}\n")
    all_analyses = []
    
    for i, png_file in enumerate(png_files, 1):
        tab_name = png_file.stem
        print(f"[{i}/{len(png_files)}] Analyzing {png_file.name}...", end=" ", flush=True)
        analysis = analyze_screenshot(str(png_file), tab_name)
        all_analyses.append(analysis)
        
        analysis_file = png_file.with_stem(png_file.stem + "_analysis").with_suffix('.json')
        with open(analysis_file, 'w') as f:
            json.dump(analysis, f, indent=2)
        print(f"✅")
    
    print(f"\n{'='*60}\n  Generating Comprehensive Report\n{'='*60}\n")
    report = generate_composite_report(all_analyses, folder)
    report_path = folder / "UI_FEEDBACK_REPORT.md"
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)
    print(f"✅ Report generated: {report_path}\n")
    return report_path

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python auto_analyze_screenshots.py <screenshot_folder>")
        sys.exit(1)
    process_screenshot_folder(sys.argv[1])
