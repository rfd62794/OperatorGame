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
    try:
        with open(json_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    except FileNotFoundError:
        return None

def encode_image(image_path):
    with open(image_path, 'rb') as f:
        return base64.standard_b64encode(f.read()).decode('utf-8')

def analyze_screenshot(image_path, annotation_json=None, client=None):
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

ALWAYS return your analysis as structured JSON with these EXACT fields:
{
  "layout_issues": [{"location": "", "problem": "", "severity": "critical"}],
  "touch_targets": [{"element": "", "current_size_dp": 0, "recommended_size_dp": 0, "issue": ""}],
  "readability": [{"element": "", "issue": "", "fix": ""}],
  "egui_constraints": [{"widget": "", "current_constraint": "", "recommended_constraint": "", "reason": ""}],
  "priority_fixes": ["fix 1", "fix 2"]
}

Do NOT output anything except valid JSON. Do NOT wrap in markdown markdown blocks (no ```json). Be specific with coordinates, sizes, and actionable fixes."""

    user_message = f"""Analyze this OperatorGame UI screenshot.

Tab: {Path(image_path).stem}

Manual Annotation (if available):
{annotation_text}

Provide structured analysis in strict JSON format."""

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
    
    return response.content[0].text.strip()

def process_screenshot_folder(folder_path):
    """Process all screenshots in a folder."""
    client = Anthropic(api_key=os.environ.get("ANTHROPIC_API_KEY"))
    results = {}
    
    png_files = sorted(Path(folder_path).glob("*.png"))
    
    for png_file in png_files:
        json_file = png_file.with_suffix('.json')
        annotation = load_annotation(json_file)
        
        print(f"Analyzing {png_file.name}...")
        analysis = analyze_screenshot(str(png_file), annotation, client)
        
        # Save analysis
        analysis_file = png_file.with_name(png_file.stem + "_analysis.json")
        with open(analysis_file, 'w', encoding='utf-8') as f:
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
