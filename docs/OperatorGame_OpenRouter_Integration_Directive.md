# Phase F.3 Revision: OpenRouter Integration & OpenAgent Code Reuse

**Status:** Switch from Claude API to OpenRouter DeepSeek V3  
**Target:** Coding Agent to integrate OpenRouter, reuse patterns from OpenAgent  
**Scope:** Analyze OpenAgent codebase, extract model routing logic, apply to UI analysis  
**Output:** `auto_analyze_screenshots.py` using OpenRouter DeepSeek V3 with OpenAgent patterns

---

## Part A: Analyze OpenAgent Codebase

**Coding Agent Action:**

Inspect `C:\Github\OpenAgent` for:

1. **Model routing logic** — How does OpenAgent select/route between models?
   - Look for: `model_selection.py`, `router.py`, or similar
   - Find: How it handles API keys, model availability, fallback logic

2. **API client initialization** — How does OpenAgent create Anthropic/HTTP clients?
   - Look for: `__init__.py`, `client.py`, `api.py`
   - Find: BaseURL configuration, auth patterns, timeout handling

3. **Two-stage processing** — How does OpenAgent split work across models?
   - Look for: Inventory stage (DeepSeek), Directive stage (Claude)
   - Find: How it chains outputs from stage 1 → stage 2

4. **Error handling & fallbacks** — How does OpenAgent handle model failures?
   - Look for: Try/catch patterns, retry logic, graceful degradation
   - Find: What it does when a model is unavailable

5. **Configuration management** — How does OpenAgent manage API keys/settings?
   - Look for: `.env` usage, config files, environment variables
   - Find: How it loads credentials securely

**Report back with:**
- File paths and code snippets (5-10 lines) showing each pattern
- How OpenAgent initializes DeepSeek V3 specifically
- Any utility functions that could be reused

---

## Part B: Extract & Adapt OpenAgent Patterns

**Based on OpenAgent codebase, modify `auto_analyze_screenshots.py`:**

### **Pattern 1: OpenRouter Client Initialization**

If OpenAgent has DeepSeek initialization, use that exact pattern:

```python
# From OpenAgent (if it exists)
from openrouter_client import OpenRouterClient

client = OpenRouterClient(
    api_key=os.getenv("OPENROUTER_API_KEY"),
    model="deepseek/deepseek-chat",
    base_url="https://openrouter.ai/api/v1"
)
```

Or if using Anthropic SDK with OpenRouter:

```python
from anthropic import Anthropic

client = Anthropic(
    api_key=os.getenv("OPENROUTER_API_KEY"),
    base_url="https://openrouter.ai/api/v1"
)
```

### **Pattern 2: Model Routing (Two-Stage from OpenAgent)**

If OpenAgent uses two-stage routing, apply similar pattern:

```python
# Stage 1: DeepSeek V3 (inventory/analysis)
def analyze_screenshot_stage1(image_path):
    """Use DeepSeek V3 for fast visual analysis."""
    response = client.messages.create(
        model="deepseek/deepseek-chat",
        max_tokens=2000,
        messages=[...]
    )
    return response

# Stage 2: Claude (optional refinement if needed)
def refine_analysis_stage2(initial_analysis):
    """Optional: Use Claude for semantic refinement."""
    # Only if budget allows
    pass
```

### **Pattern 3: Error Handling from OpenAgent**

Reuse OpenAgent's error handling:

```python
# From OpenAgent pattern
def call_model_with_fallback(primary_model, fallback_model, prompt):
    try:
        return call_model(primary_model, prompt)
    except Exception as e:
        print(f"Primary model failed: {e}, trying fallback...")
        return call_model(fallback_model, prompt)

# Apply to screenshots:
try:
    analysis = analyze_screenshot_stage1(image_path)
except Exception as e:
    print(f"DeepSeek failed: {e}")
    # Fallback to Claude or skip
```

### **Pattern 4: Credentials from OpenAgent**

Reuse credential loading pattern:

```python
# Load from .env (OpenAgent style)
from dotenv import load_dotenv
import os

load_dotenv()

OPENROUTER_API_KEY = os.getenv("OPENROUTER_API_KEY")
if not OPENROUTER_API_KEY:
    raise ValueError("OPENROUTER_API_KEY not found in .env")

client = Anthropic(
    api_key=OPENROUTER_API_KEY,
    base_url="https://openrouter.ai/api/v1"
)
```

---

## Part C: Complete Refactored Script

**Modify:** `C:\Github\OperatorGame\scripts\auto_analyze_screenshots.py`

**Template (agent fills in OpenAgent patterns):**

```python
#!/usr/bin/env python3
"""
Automated UI Screenshot Analysis using OpenRouter DeepSeek V3.
Reuses model routing patterns from OpenAgent.
"""

import json
import base64
import os
import sys
from pathlib import Path
from dotenv import load_dotenv
from anthropic import Anthropic

# Load credentials (OpenAgent pattern)
load_dotenv()
OPENROUTER_API_KEY = os.getenv("OPENROUTER_API_KEY")

if not OPENROUTER_API_KEY:
    print("ERROR: OPENROUTER_API_KEY not found in .env")
    print("Add to .env: OPENROUTER_API_KEY=sk-or-v1-xxxxx")
    sys.exit(1)

# Initialize OpenRouter client (reuse OpenAgent initialization)
client = Anthropic(
    api_key=OPENROUTER_API_KEY,
    base_url="https://openrouter.ai/api/v1"
)

def encode_image(image_path):
    """Encode PNG to base64 for API."""
    with open(image_path, 'rb') as f:
        return base64.standard_b64encode(f.read()).decode('utf-8')

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

    user_message = f"""Analyze this OperatorGame screenshot (Tab: {tab_name}).

Provide detailed structural and visual feedback in JSON format ONLY."""

    try:
        # Use DeepSeek V3 (35x cheaper than Claude, excellent for UI analysis)
        response = client.messages.create(
            model="deepseek/deepseek-chat",
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
    
    print(f"\n{'='*60}")
    print(f"  Automated UI Screenshot Analysis (OpenRouter DeepSeek V3)")
    print(f"  Folder: {folder.name}")
    print(f"  Screenshots: {len(png_files)}")
    print(f"{'='*60}\n")
    
    all_analyses = []
    
    for i, png_file in enumerate(png_files, 1):
        tab_name = png_file.stem
        print(f"[{i}/{len(png_files)}] Analyzing {png_file.name}...", end=" ", flush=True)
        
        analysis = analyze_screenshot(str(png_file), tab_name)
        all_analyses.append(analysis)
        
        # Save individual analysis
        analysis_file = png_file.with_stem(png_file.stem + "_analysis").with_suffix('.json')
        with open(analysis_file, 'w') as f:
            json.dump(analysis, f, indent=2)
        
        print(f"✅")
    
    # Generate composite report
    print(f"\n{'='*60}")
    print(f"  Generating Comprehensive Report")
    print(f"{'='*60}\n")
    
    report = generate_composite_report(all_analyses, folder)
    
    report_path = folder / "UI_FEEDBACK_REPORT.md"
    with open(report_path, 'w') as f:
        f.write(report)
    
    print(f"✅ Report generated: {report_path}\n")
    return report_path

def generate_composite_report(analyses, folder):
    """Generate markdown report combining all analyses."""
    
    report = f"""# OperatorGame UI Analysis Report

**Generated:** Phase F.3 OpenRouter Analysis (DeepSeek V3)  
**Folder:** {folder.name}  
**Screenshots Analyzed:** {len(analyses)}  
**Model:** DeepSeek V3 via OpenRouter  

---

## Executive Summary

"""
    
    # Count issues
    critical_count = sum(len(a.get('critical_issues', [])) for a in analyses)
    major_count = sum(len([i for i in a.get('spacing_issues', []) if i.get('severity') == 'major']) for a in analyses)
    minor_count = sum(len([i for i in a.get('spacing_issues', []) if i.get('severity') == 'minor']) for a in analyses)
    
    report += f"- **Critical Issues:** {critical_count}\n"
    report += f"- **Major Issues:** {major_count}\n"
    report += f"- **Minor Issues:** {minor_count}\n\n"
    
    # Per-tab analysis
    report += "## Tab-by-Tab Analysis\n\n"
    
    for i, analysis in enumerate(analyses, 1):
        if 'error' in analysis:
            report += f"### {i}. Tab {i} (Analysis Failed)\n\n"
            report += f"Error: {analysis.get('error', 'Unknown error')}\n\n"
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
        
        if 'egui_constraints' in analysis:
            report += "**Egui Constraint Changes:**\n```rust\n"
            for constraint in analysis['egui_constraints'][:5]:
                report += f"// {constraint.get('reason', '')}\n"
                report += f"// Current:  {constraint.get('current', '')}\n"
                report += f"// Change to: {constraint.get('recommended', '')}\n\n"
            report += "```\n\n"
    
    # Implementation roadmap
    report += "## Implementation Roadmap\n\n"
    report += "### Phase 1: Quick Wins (1-2 hours)\n"
    report += "- Standardize margins/padding across all tabs\n"
    report += "- Increase small font sizes for readability\n"
    report += "- Fix touch target sizes\n\n"
    
    report += "### Phase 2: Layout Refinement (3-4 hours)\n"
    report += "- Adjust grid spacing and alignment\n"
    report += "- Fix overflow/clipping issues\n\n"
    
    report += "### Phase 3: Polish (2-3 hours)\n"
    report += "- Color and contrast optimization\n\n"
    
    report += "---\n\n"
    report += f"**Analysis Tool:** OpenRouter DeepSeek V3 (Cost: ~$0.001 per 7 screenshots)\n"
    
    return report

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python auto_analyze_screenshots.py <screenshot_folder>")
        sys.exit(1)
    
    folder = sys.argv[1]
    process_screenshot_folder(folder)
```

---

## Part D: Setup .env for OpenRouter

**Agent Action:**

1. Get OpenRouter API key from https://openrouter.ai/keys
2. Add to `C:\Github\OperatorGame\.env`:
   ```
   OPENROUTER_API_KEY=sk-or-v1-xxxxx
   ```
3. Verify `.gitignore` includes `.env` (should already be there)

---

## Part E: Execute Analysis

**Agent Action:**

```powershell
cd C:\Github\OperatorGame

# Install python-dotenv if not already installed
pip install python-dotenv

# Run analysis with OpenRouter
$latestFolder = Get-ChildItem "screenshots_uitree_*" -Directory | Sort-Object CreationTime -Descending | Select-Object -First 1

python.exe .\scripts\auto_analyze_screenshots.py $latestFolder.FullName
```

---

## Cost Comparison

| Model | Cost (7 screenshots) | Quality | Speed |
|-------|-------------------|---------|-------|
| Claude 3.5 Sonnet | ~$2.10 | Highest | Moderate |
| DeepSeek V3 | ~$0.06 | 95% of Claude | Fast |
| Qwen/QwQ | ~$0.03 | 90% of Claude | Moderate |

**Using DeepSeek V3 saves 35x cost vs Claude while maintaining excellent UI analysis quality.**

---

## Success Criteria

✅ OpenAgent code patterns identified and documented  
✅ `auto_analyze_screenshots.py` refactored to use OpenRouter  
✅ `.env` configured with OPENROUTER_API_KEY  
✅ Analysis runs successfully on all 7 screenshots  
✅ `UI_FEEDBACK_REPORT.md` generated with egui fixes  

---

**Agent: Proceed with Part A (analyze OpenAgent), then implement Parts B-E.**
