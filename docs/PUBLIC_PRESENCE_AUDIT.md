# PUBLIC PRESENCE AUDIT: OperatorGame & RFD IT Services

> **Authority:** Architect Review Only (PyPro SDD-Edition)
> **Date:** 2026-03-07
> **Status:** PRE-FLIGHT AUDIT

---

## Phase A: Repository Inventory

### 1. Source Modules
| Filename | Purpose | Status |
|----------|---------|--------|
| `src/lib.rs` | Entry point & Android main loop | ✅ Complete |
| `src/models.rs` | Core domain entities (Operator, Mission, Slime) | ✅ Complete |
| `src/genetics.rs` | Genetic engine & Breeding resolution | ✅ Complete |
| `src/combat.rs` | D20 resolution & Culture-based RollModes | ✅ Complete |
| `src/dungeon.rs` | Procedural dungeon track engine | 🔄 Active (Dormant/UI-less) |
| `src/garden.rs` | Physics-based habitat simulation (Egui) | ✅ Active |
| `src/racing.rs` | Procedural race track physics | 🔄 Active (Dormant/UI-less) |
| `src/persistence.rs` | Atomic JSON save/load architecture | ✅ Complete |
| `src/log_engine.rs` | Narrative template generation | ✅ Complete |
| `src/cli.rs` | Command-tree implementation (Clap) | ✅ Complete |
| `src/ui/` | Egui view layers (War Room dashboard) | 🔄 Active |
| `src/render/` | Cymatics visual identity & Slime rendering | 🔄 Active |

### 2. Governance Documents
| Document | Version | Last Update | Purpose |
|----------|---------|-------------|---------|
| `CONSTITUTION.md` | v1.0 | 2026-03-04 | Non-negotiable principles & stack |
| `SPEC.md` | v2.0 | 2026-03-04 | Functional contracts & math |
| `docs/README.md` | - | 2026-03-07 | Document Index (Site map) |
| `VISION.md` | v1.0 | 2026-03-07 | Narrative vision & World-building |
| `docs/adr/` | - | 2026-03-04 | ADR-001 through ADR-052+ |

### 3. Demos & Features
- **Live (CLI):** Full game loop (Hire/Hatch, Deploy, AAR, Splice, Status).
- **Live (GUI):** War Room dashboard with real-time Garden simulation.
- **Stub/Concept:** Procedural mission generation, Multiplayer, Advanced Economy.

### 4. README Current State
- **Root README:** ❌ **MISSING**.
- **First-Time Visitor Experience:** Poor. A visitor to the GitHub root sees a file list but no project description, screenshots, or "Quick Start" guide. The 30-second comprehension test is currently failing.

### 5. Test Floor State
- **Total Tests:** 145 (confirmed via `cargo test`)
- **Modules Covered:** `models`, `persistence`, `log_engine`, `genetics`, `combat`, `dungeon`, `garden`, `racing`, `world_map`.
- **Status:** ✅ **145 PASSING / 0 FAILED**. (Substantial engineering floor; identifies drift from `SPEC.md` v2.0).

---

## Phase B: Audience Gap Analysis

### 1. Technical Recruiter / Hiring Manager
- **Current Sight:** High-quality Rust 2021 code, advanced `egui` implementation, and Android `cdylib` targeting.
- **Gap:** Evidence of the "front door" (README) and a clear architecture overview for the multi-crate workspace.
- **Competence Signal:** Strict ADR (Architectural Decision Record) governance, 145+ unit tests, and wall-clock async timer patterns for real-time simulation safety.

### 2. Indie Game Developer Community
- **Current Sight:** Custom D20 combat engine, procedural track generation, and physics-integrated genetics.
- **Gap:** No DevLog link at the root to explain the "Cymatics" visual identity or the math behind the genetic tiers.
- **Loss Point:** Requiring a deep dive into `docs/src/GDD/` to understand the vision.

### 3. Potential Player
- **Current Sight:** Code-heavy root.
- **Gap:** ❌ **ZERO VISUALS**. No screenshots, no GIFs of the Garden habitat, and no direct link to the WASM build.
- **Sentiment:** A "hardcore simulation" that is currently gated behind a technical barrier.

---

## Phase C: README Rewrite Recommendations

### Recommended Section Outline
1.  **Project Banner & Headline**: One-sentence hook + high-level vision. (New)
2.  **Visual Gallery**: Screenshots of the War Room and Garden. (New)
3.  **Project Status**: Current version, platform support (Android/WASM/CLI), and active Sprint. (New)
4.  **Core Features**: Bulleted list of what is currently playable. (Keep from SPEC)
5.  **Quick Start (CLI & GUI)**: Simple `uv run` commands to boot the game. (New)
6.  **The Governance Index**: Links to `CONSTITUTION`, `SPEC`, and `VISION`. (Keep from docs/README)
7.  **Tech Stack**: Brief list of core crates (Egui, Tokio, Serde). (New)
8.  **Public Presence**: Links to `rfditservices.com`, DevLog (Blog), and Play Store. (New)

---

## Phase D: Website Structure Plan (`rfditservices.com`)

### 1. Main Site (Static Portfolio)
- **Home**: Professional summary, featured projects carousel (including Operator).
- **The War Room (Operator)**: A dedicated project page featuring:
    - **WASM Embed**: Playable demo directly in the browser. (High Priority)
    - **Screenshots/Video**: Cinematic look at the Garden and Combat.
    - **Direct Links**: Play Store (Android) and GitHub (Source).
- **Services**: IT/Architectural consulting overview.
- **Contact**: Secure PII-handling form for business inquiries.

### 2. DevLog (`blog.rfditservices.com`)
- **Structure**: Chronological tags (e.g., `#Rust`, `#Operator`, `#Sprint9`).
- **Focus**: "The 'Why' behind the 'How'." Post-mortems on ADR decisions, genetic math deep-dives, and Android deployment hurdles.

### 3. Priority Order
1.  **P0 (Root README)**: Fix the repo entry point immediately.
2.  **P1 (WASM Build)**: Get the game running in-browser to lower the barrier to entry.
3.  **P2 (Operator Project Page)**: Create the static anchor for the public.
4.  **P3 (Continuous DevLog)**: Sync the current documentation (ROADMAP/ROADMAP.md) into public blog posts.
