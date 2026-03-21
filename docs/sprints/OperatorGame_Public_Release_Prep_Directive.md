# OperatorGame Public Release Preparation Directive

**Status:** Phase 6 Complete — Repository Hardening & Documentation  
**Target:** Production-ready GitHub public release  
**Scope:** Documentation, git history, repo structure, gitignore audit

---

## Part A: Create DEPLOYMENT.md (Repository Root)

**Location:** `C:\Github\OperatorGame\DEPLOYMENT.md`

**Content Requirements:**

1. **Header Section**
   - Clear title: "Android Deployment & Automation"
   - One-line: "Deploy OperatorGame to physical Android hardware using PowerShell automation"

2. **Quick Start (code block)**
   - List 4 critical commands:
     - `.\deploy_moto.ps1` (install + launch)
     - `.\check_moto.ps1` (health check + diagnostics)
     - `.\capture_screenshots.ps1` (automated UI capture)
     - `.\find_ui_coordinates.ps1` (interactive calibration)

3. **Prerequisites Section**
   - Windows PowerShell 5.1+ requirement
   - Android SDK path: `$env:LOCALAPPDATA\Android\Sdk`
   - ADB, apksigner, aapt availability checks
   - NDK 25.2.9519653+ pinned version note
   - Hardware: Moto G 2025 (API 35) tested baseline
   - USB debugging requirement

4. **Module Architecture Section**
   - Explain OperatorDeviceTools.psm1 purpose
   - List core function categories:
     - Device Management (Connect-Device, Is-AppRunning, Stop-OperatorApp)
     - App Lifecycle (Install-OperatorApp, Launch-OperatorApp, Detect-AppCrash)
     - Screen I/O (Capture-Screenshot, Invoke-DeviceTap, Invoke-DeviceInput)
     - Diagnostics (Get-DeviceLogcat, Audit-NdkConfig, Diagnose-ApkIssues)
   - One-line description per function (no full docs — link to module for details)

5. **Common Workflows Section**
   - Workflow 1: Deploy & Test (deploy_moto.ps1 → check_moto.ps1)
   - Workflow 2: Screenshot Sweep (capture_screenshots.ps1)
   - Workflow 3: Crash Diagnostics (Get-DeviceLogcat + Detect-AppCrash)
   - Workflow 4: APK Health Check (Diagnose-ApkIssues)

6. **Troubleshooting Section**
   - "ADB not found" → check PATH
   - "Device offline" → check USB debugging, restart adb daemon
   - "APK not discovered" → check repo root, target/ directory
   - "SIGKILL on launch" → run `Audit-NdkConfig` + `Diagnose-ApkIssues`
   - "Screenshot pull fails" → verify device storage space

7. **Reference Section**
   - Link to `/docs/sprints/` for phase directives
   - Link to `CONSTITUTION.md` for governance
   - Link to `SPEC.md` for game rules

**Tone:** Professional, concise, action-oriented. Target audience: external developers cloning the repo.

---

## Part B: Reorganize /docs Directory

**Current State:** 34 markdown files cluttering `/docs` root  
**Target State:** Logical categorization, clear navigation

**New Structure:**

```
/docs
├── README.md              (NEW: index + navigation)
├── adr/                   (KEEP: Architectural Decision Records)
├── roadmap/               (KEEP: Future state planning)
├── src/                   (KEEP: existing content)
├── design/                (NEW: design docs)
│   ├── DESIGN_BLUEPRINT.md
│   ├── VISUAL_IDENTITY.md
│   └── MATH_GENETICS.md
├── systems/               (NEW: system designs)
│   ├── TRINARY_SYSTEM.md
│   ├── COLOR_MIXING_LOGIC.md
│   ├── STAT_SYSTEM.md
│   └── LIFECYCLE_SDD.md
└── sprints/               (NEW: phase directives & planning)
    ├── phase_e_core_loop.md
    ├── phase_f_ui_polish.md
    ├── phase_f0_ui_polish_directive.md
    ├── android_tools_architecture.md
    ├── crash_diagnosis_directive.md
    ├── automation_formalization_directive.md
    └── (all other SDD directives go here)
```

**Execution:**
1. Create `/docs/design/`, `/docs/systems/`, `/docs/sprints/` directories
2. Move design-related files to `/docs/design/`
3. Move system/mechanics files to `/docs/systems/`
4. Move all `*_Directive.md`, `*_Phase_*.md`, `*_SDD.md` files to `/docs/sprints/`
5. Create `/docs/README.md` with navigation index (see Part C below)
6. Leave only index in `/docs` root

**File Mapping Guide (for agent to execute):**

**→ /docs/design/:**
- DESIGN_BLUEPRINT.md
- OperatorGame_Vision.docx (convert to .md if possible, or link)

**→ /docs/systems/:**
- STAT_SYSTEM.md
- LIFECYCLE_SDD.md
- (Any color mixing, genetic algebra, or mechanics docs)

**→ /docs/sprints/:**
- OperatorGame_Unified_Android_Tools_Architecture_Directive.md
- OperatorGame_Phase_F_UI_Polish_Directive.md
- OperatorGame_Crash_Diagnosis_Testing_Suite_Directive.md
- OperatorGame_Automation_Formalization_Directive.md
- OperatorGame_Screenshot_Capture_Directive.md
- OperatorGame_Build_Deploy_Automation_Directive.md
- OperatorGame_UI_Layout_Audit_Directive.md
- OperatorGame_Android_Viewport_Phase_B.md
- OperatorGame_Android_Viewport_Directive.md
- OperatorGame_Sub_Tab_Scaffold_Phase_C.md
- OperatorGame_Unify_UI_Structure_Phase_E0b.md
- SlimeGarden_Core_Loop_Phase_E.md
- SlimeGarden_UI_Tab_Scaffold.md
- (All other phase/directive files)

---

## Part C: Create /docs/README.md (Navigation Index)

**Location:** `C:\Github\OperatorGame\docs\README.md`

**Content:**

```markdown
# OperatorGame Documentation

Welcome to the OperatorGame design, architecture, and development documentation.

## Quick Links

- **[Deployment & Automation](../DEPLOYMENT.md)** — How to build and deploy to Android
- **[Constitution](../CONSTITUTION.md)** — Governance, ADR discipline, project structure
- **[Specification](../SPEC.md)** — Game mechanics, rules, balance

## Documentation Structure

### [Design](./design/)
High-level visual identity, blueprint, and design philosophy.
- `DESIGN_BLUEPRINT.md` — Aesthetic direction, color system, UI patterns
- `VISUAL_IDENTITY.md` — Branding, asset guidelines

### [Systems](./systems/)
Core game mechanics: genetics, stats, lifecycle, combat.
- `STAT_SYSTEM.md` — Character stat derivation and growth
- `LIFECYCLE_SDD.md` — Breeding, incubation, lifecycle phases
- Genetics engine, color mixing, stat calculations

### [Sprints & Phases](./sprints/)
Development history: phase directives, SDD specs, architectural decisions.
- `phase_e_core_loop.md` — Core loop implementation
- `phase_f_ui_polish.md` — Mobile UI refinement
- `android_tools_architecture.md` — PowerShell automation framework
- `crash_diagnosis_directive.md` — Debugging and telemetry
- (All sprint planning and architectural evolution)

### [Architecture Decision Records](./adr/)
Critical technical decisions and their rationale.

### [Roadmap](./roadmap/)
Future features, Sprint 2+, long-term vision.

## For New Contributors

1. Read `../CONSTITUTION.md` for project governance
2. Read `../SPECIFICATION.md` for game rules
3. Skim `./design/` to understand aesthetic direction
4. Skim `./systems/` to understand mechanics
5. Review relevant `./sprints/` phase for context on current work

## For Build/Deploy

See `../DEPLOYMENT.md` for Android automation framework.

---

*Last updated: March 2026*
```

---

## Part D: Update/Audit .gitignore

**Location:** `C:\Github\OperatorGame\.gitignore`

**Add/Verify these entries:**

```gitignore
# PowerShell Automation Artifacts
screenshots_*/
ui_coordinates.json

# Android SDK / Local Config
local.properties
.gradle/

# Android Keystores (CRITICAL: Never leak signing keys)
*.jks
*.keystore
*.p12
*.pfx

# Rust Build Artifacts (if not already present)
/target/
/Cargo.lock

# Generated Files
*.apk
*.aab

# OS/IDE
.DS_Store
.vscode/
*.swp
*.swo
*~
.idea/

# Python
__pycache__/
*.pyc
*.pyo
.venv/
venv/

# Temporary
.tmp/
temp/
```

**Verification Steps (for agent):**
1. Open `.gitignore`
2. Verify `target/`, `*.jks`, `local.properties` are already present
3. Add missing entries above
4. Run: `git status` to verify no sensitive files are tracked
5. If any `.jks` or screenshots are tracked, execute: `git rm -r --cached <filename>`

---

## Part E: Update Main README.md

**Location:** `C:\Github\OperatorGame\README.md`

**Action:** Add one line under the "Quick Start" or "Getting Started" section:

```markdown
### For Android Hardware Deployment

See [DEPLOYMENT.md](./DEPLOYMENT.md) for automated build & test pipeline.
```

**Do NOT rewrite the entire README** — it's already excellent. Just link to DEPLOYMENT.md.

---

## Part F: Commit History Cleanup (Conditional)

**Action:** Review the micro-commit history from this SDD sprint.

Run:
```powershell
git log --oneline | head -50
```

**Decision Point:**

**If commit count < 100 and history is clean:**
- Leave as-is. The micro-commits show iterative discipline.

**If commit count > 100 and includes many `fix: typo`, `refactor: minor` commits:**
- Execute squash: `git rebase -i HEAD~[Count]` to combine into:
  - `feat: Introduce OperatorDeviceTools unified PowerShell automation framework`
  - Keep this as a single cohesive commit showing the full SDD sprint

**Agent Decision:** Inspect history first, then decide. If history is noisy, squash. If history is clean, leave it.

---

## Part G: Final Validation Checklist

**Before marking "Public Ready," verify:**

- [ ] DEPLOYMENT.md exists in repo root
- [ ] /docs/ reorganization complete (34 files moved to subdirs)
- [ ] /docs/README.md created with navigation
- [ ] .gitignore updated with PowerShell + Android + Keystore entries
- [ ] Main README.md links to DEPLOYMENT.md
- [ ] `git status` shows clean working tree
- [ ] No `.jks` files tracked in git
- [ ] No `screenshots_*/` folders tracked in git
- [ ] `git log --oneline | head -20` shows reasonable history (not 200 micro-commits)

---

## Execution Order (Agent)

1. **Create DEPLOYMENT.md** (Part A)
2. **Reorganize /docs/** (Part B)
3. **Create /docs/README.md** (Part C)
4. **Update .gitignore** (Part D)
5. **Update main README.md** (Part E)
6. **Review + optionally squash commit history** (Part F)
7. **Run final validation checklist** (Part G)
8. **Report completion status** to Robert

---

## Success Criteria

✅ Repository is **documentation-complete**  
✅ /docs is **logically organized** and navigation-clear  
✅ DEPLOYMENT.md is **actionable** for external devs  
✅ .gitignore **prevents credential leakage**  
✅ Commit history is **clean and professional**  
✅ Main README links to deployment guide  
✅ No uncommitted changes or tracked secrets  

**Status: Ready for `git push origin main` to public GitHub** ✅
