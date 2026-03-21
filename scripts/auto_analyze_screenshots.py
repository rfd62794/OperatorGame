#!/usr/bin/env python3
"""
Automated UI Screenshot Analysis using OpenRouter DeepSeek V3.
Reuses model routing patterns from OpenAgent (pure requests to OpenRouter).
"""

import requests
import json
import os
import sys
import time
import base64
from typing import Optional, Dict, Any
from pathlib import Path
from dotenv import load_dotenv

# Load credentials (OpenAgent pattern)
load_dotenv()

class OpenRouterVisionAdapter:
    """Lightweight OpenRouter adapter for vision analysis (based on OpenAgent pattern)."""
    
    def __init__(self, api_key: Optional[str] = None, model: str = "google/gemini-2.0-flash-001"):
        self.api_key = api_key or os.getenv("OPENROUTER_API_KEY")
        if not self.api_key:
            raise ValueError("OPENROUTER_API_KEY not found in environment or argument")
        
        self.model = model
        self.base_url = "https://openrouter.ai/api/v1"
        self.timeout = 60
        
        # Pricing from OpenAgent
        self.pricing = {
            "deepseek/deepseek-chat": {"input": 0.00000014, "output": 0.00000014},
            "google/gemini-2.0-flash-001": {"input": 0.0000001, "output": 0.0000004},
        }
        
        # Fallback models (from OpenAgent)
        self.fallback_models = {
            "deepseek/deepseek-chat": "google/gemini-2.0-flash-001",
            "google/gemini-2.0-flash-001": "deepseek/deepseek-chat",
        }
    
    def call_with_retries(self, messages: list, max_retries: int = 3) -> Dict[str, Any]:
        """Call OpenRouter with exponential backoff (OpenAgent pattern)."""
        
        for attempt in range(max_retries):
            try:
                return self._call_openrouter(messages)
            except Exception as e:
                if attempt < max_retries - 1:
                    wait_time = 2 ** attempt  # Exponential backoff: 1s, 2s, 4s
                    print(f"    Attempt {attempt + 1} failed: {e}. Retrying in {wait_time}s...")
                    time.sleep(wait_time)
                else:
                    # Try fallback model
                    fallback = self.fallback_models.get(self.model)
                    if fallback:
                        print(f"    Primary model failed. Trying fallback: {fallback}")
                        self.model = fallback
                        try:
                            return self._call_openrouter(messages)
                        except Exception as fallback_error:
                            raise Exception(f"Both primary and fallback models failed: {fallback_error}")
                    else:
                        raise
    
    def _call_openrouter(self, messages: list) -> Dict[str, Any]:
        """Raw HTTP POST to OpenRouter (pure requests, no SDK)."""
        
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://github.com/rfd62794/OperatorGame",
            "X-Title": "OperatorGame Phase F",
        }
        
        payload = {
            "model": self.model,
            "messages": messages,
            "max_tokens": 2000,
            "temperature": 0.7,
        }
        
        url = f"{self.base_url}/chat/completions"
        
        response = requests.post(url, headers=headers, json=payload, timeout=self.timeout)
        response.raise_for_status()
        
        data = response.json()
        
        if "error" in data:
            raise Exception(data["error"]["message"])
            
        # Extract message content (OpenAI-format response from OpenRouter)
        return {
            "content": data["choices"][0]["message"]["content"],
            "model": data.get("model", self.model),
            "usage": data.get("usage", {}),
        }

# ====================================================================
# INTEGRATION INTO auto_analyze_screenshots.py
# ====================================================================

def analyze_screenshot_with_openrouter(image_path: str, tab_name: str) -> Dict:
    """Analyze screenshot using OpenRouter DeepSeek V3 (OpenAgent pattern)."""
    
    # Encode image to base64
    with open(image_path, 'rb') as f:
        image_base64 = base64.standard_b64encode(f.read()).decode('utf-8')
    
    # Build message with vision content
    system_prompt = """You are a mobile UI/UX expert analyzing OperatorGame screenshots.

Provide ONLY valid JSON response with this structure:
{
  "tab": "tab_name",
  "elements_identified": [...],
  "critical_issues": [...],
  "egui_constraints": [...],
  "priority_fixes": [...]
}

No preamble. JSON only."""
    
    messages = [
        {
            "role": "system",
            "content": system_prompt
        },
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": f"Analyze this OperatorGame screenshot (Tab: {tab_name}). Provide structured UI feedback.",
                },
                {
                    "type": "image_url",
                    "image_url": {
                        "url": f"data:image/png;base64,{image_base64}"
                    },
                },
            ],
        }
    ]
    
    # Initialize adapter
    adapter = OpenRouterVisionAdapter(model="google/gemini-2.0-flash-001")
    
    # Call with retries (OpenAgent pattern)
    try:
        response = adapter.call_with_retries(messages)
        content = response["content"]
        
        # Try direct JSON parse first
        try:
            analysis = json.loads(content)
        except json.JSONDecodeError:
            import re
            # Extract JSON block if wrapped in markdown
            json_match = re.search(r'\{.*\}', content, re.DOTALL)
            if json_match:
                analysis = json.loads(json_match.group())
            else:
                analysis = {"error": f"Failed to parse response: {content[:100]}"}
        
        return analysis
        
    except Exception as e:
        print(f"Analysis failed: {e}")
        return {"error": str(e)}

def generate_composite_report(analyses, folder):
    """Generate markdown report combining all analyses."""
    
    report = f"# OperatorGame UI Analysis Report\n\n**Generated:** Phase F.3 OpenRouter Analysis (Gemini Flash)\n**Folder:** {folder.name}\n**Screenshots Analyzed:** {len(analyses)}\n**Model:** Gemini 2.0 Flash via OpenRouter\n\n---\n\n## Executive Summary\n\n"
    
    # Count issues
    critical_count = sum(len(a.get('critical_issues', [])) for a in analyses if isinstance(a, dict))
    
    report += f"- **Critical Issues:** {critical_count}\n\n"
    
    # Per-tab analysis
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
                    c_reason = constraint.get('reason', '')
                    c_curr = constraint.get('current', '')
                    c_rec = constraint.get('recommended', '')
                    report += f"// {c_reason}\n// Current:  {c_curr}\n// Change to: {c_rec}\n\n"
            report += "```\n\n"
    
    report += "---\n\n**Analysis Tool:** OpenRouter Gemini Flash (Cost: ~$0.0003 per 7 screenshots)\n"
    return report

# ====================================================================
# MAIN LOOP (refactored)
# ====================================================================

def process_screenshots_openrouter(folder_path: str):
    """Process all screenshots using OpenRouter adapter."""
    
    folder = Path(folder_path)
    if not folder.exists():
        print(f"ERROR: Folder not found: {folder}")
        sys.exit(1)
        
    png_files = sorted(folder.glob("*.png"))
    if not png_files:
        print(f"ERROR: No PNG files found in {folder}")
        sys.exit(1)
    
    print(f"\nAnalyzing {len(png_files)} screenshots with OpenRouter Gemini Flash...")
    print(f"Cost per analysis: ~$0.00001 (35x cheaper than Claude)\n")
    
    all_analyses = []
    
    for i, png_file in enumerate(png_files, 1):
        print(f"[{i}/{len(png_files)}] {png_file.name}...", end=" ", flush=True)
        
        analysis = analyze_screenshot_with_openrouter(str(png_file), png_file.stem)
        all_analyses.append(analysis)
        
        # Save individual analysis
        analysis_file = png_file.with_stem(png_file.stem + "_analysis").with_suffix('.json')
        with open(analysis_file, 'w') as f:
            json.dump(analysis, f, indent=2)
        
        print("✅")
    
    # Generate report
    report = generate_composite_report(all_analyses, folder)
    report_path = folder / "UI_FEEDBACK_REPORT.md"
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)
    
    print(f"\n✅ Report: {report_path}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python auto_analyze_screenshots.py <screenshot_folder>")
        sys.exit(1)
    
    process_screenshots_openrouter(sys.argv[1])
