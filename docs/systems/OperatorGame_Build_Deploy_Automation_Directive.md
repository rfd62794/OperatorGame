# OperatorGame — Build/Deploy Automation Inventory & Analysis Directive

**Target:** Coding Agent (Gemini/Antigravity)  
**Goal:** Find, catalog, and analyze all build/deploy automation scripts in the OperatorGame repository  
**Output:** Structured inventory of all automation, build, and deployment infrastructure  
**Scope:** Scripts, configs, CI/CD, APK building, ADB deployment, custom automation  

---

## Task: Find & Inventory All Automation Scripts

### Search Criteria

Search the entire OperatorGame repository (root and all subdirectories) for files matching:

**Script Files:**
- `*.sh` (shell scripts)
- `*.bat`, `*.ps1` (Windows batch/PowerShell)
- `Makefile`, `makefile`
- `build.rs` (Rust build scripts)
- Any files in a `scripts/` directory
- `gradle.build`, `build.gradle` (Android Gradle)

**CI/CD Configuration:**
- `.github/workflows/*.yml` (GitHub Actions)
- `.gitlab-ci.yml` (GitLab CI)
- `.travis.yml` (Travis CI, if present)
- Any other CI/CD config files

**Build-Related Config:**
- `Cargo.toml` (check `[scripts]` section for custom commands)
- `build.rs` (Rust build script)
- `gradle.build` or Android build configs
- `.cargo/config.toml` (Cargo config)

**Android-Specific:**
- ADB scripts (anything with `adb` commands)
- APK signing configs
- Gradle build configs
- Emulator configs

**Custom Automation:**
- Any `.sh` or `.ps1` files in root directory
- Any executable files (Unix)
- Any batch files in the project

---

## For Each Script Found

Document the following in a structured format:

### Template (Repeat for each file)

```
## [SCRIPT NAME]

**File Path:** `path/to/file`

**File Type:** (shell script, PowerShell, Makefile, Gradle, etc.)

**Purpose:** 
[One sentence: what does this script do?]

**Dependencies:**
- [What does it need to run? (Node.js, Rust, ADB, etc.)]
- [Any environment variables required?]
- [Any external tools required?]

**Current Status:** 
- Working / Outdated / Unknown
- [Any notes on viability]

**Parameters/Inputs:**
- [What arguments does it accept?]
- [What environment setup does it expect?]
- [Any hardcoded paths or assumptions?]

**Output/Result:**
- [What does it produce? (APK, binary, logs, etc.)]
- [Where is the output placed?]

**Usage Example:**
```
[example command line]
```

**Notes:**
- [Any issues, gotchas, or maintenance notes]
- [Is this actively used or deprecated?]
```

---

## Analysis Questions to Answer

Once all scripts are found, answer these for each script:

1. **Is this script still in use?** (actively called, or abandoned?)
2. **Does it have any hardcoded paths or assumptions?** (what breaks if paths change?)
3. **What are the actual dependencies?** (does it require Node.js, Python, Rust, ADB, etc.?)
4. **Could this be improved or formalized?** (what's the systemic issue it's solving?)
5. **Does it conflict with other scripts?** (do multiple scripts do the same thing?)

---

## Categorization

Once all scripts are found, group them by category:

1. **Local Development** (scripts developers run locally during development)
2. **Android Build & Deploy** (APK building, signing, ADB deployment to phone)
3. **Testing & Verification** (automated testing, CI/CD)
4. **Maintenance & Cleanup** (cache clearing, file cleanup, etc.)
5. **Custom Automation** (project-specific automation that doesn't fit other categories)

---

## Deliverable Format

Return the results in this structure:

```markdown
# OperatorGame Build/Deploy Automation Inventory

## Summary
- **Total Scripts Found:** [N]
- **Categories:** [list]
- **Critical Scripts:** [scripts that must work for the dev workflow]
- **Deprecated/Unused:** [scripts that appear abandoned]

## Detailed Inventory

### Category: Android Build & Deploy
[List all scripts in this category with full details]

### Category: Local Development
[List all scripts in this category with full details]

### Category: Testing & Verification
[List all scripts in this category with full details]

### Category: Maintenance & Cleanup
[List all scripts in this category with full details]

### Category: Custom Automation
[List all scripts in this category with full details]

## Analysis & Recommendations

### Current State
[Summary of what automation exists, what's working, what's broken or missing]

### Systemic Issues Identified
[Any patterns, conflicts, or gaps in the automation?]

### Recommendations
[What could be formalized, improved, or consolidated?]

### Priority Actions
[What should be done first to enable testing on Moto G?]
```

---

## Success Criteria

✓ All automation scripts in the repo are found  
✓ Each script is documented with purpose, dependencies, parameters, usage  
✓ Scripts are categorized logically  
✓ Analysis answers the 5 questions above  
✓ Recommendations identify which scripts are critical for testing workflow  
✓ Output is structured and easy to parse  

---

## Notes for Agent

- **Scope is inventory only** — don't modify or refactor scripts yet
- **Be thorough** — check subdirectories, hidden files, etc.
- **Include context** — if a script references another script, note the dependency
- **Flag issues** — if a script appears broken or outdated, note it
- **Look for patterns** — if multiple scripts do similar things, flag consolidation opportunity

---

## Next Steps (After Inventory)

Once you have the inventory:
1. Identify which script(s) handle "build APK + deploy to Moto G"
2. Formalize that workflow (document it, make it repeatable)
3. Enable Robert to test Phase F.0 UI polish on the phone

---
