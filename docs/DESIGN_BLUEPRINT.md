DESIGN BLUEPRINT — rpgCore → OPERATOR
For the Designer Agent | Compiled 2026-03-04

This document is the primary reference for any agent working on OPERATOR (the Rust rewrite) who needs to understand the source of truth in rpgCore. It covers:

Dice Demo — exact animation contract
Culture Node Wars — exact faction simulation contract
Full Archive inventory — what lives where and why
PART 1 — DICE DEMO (
demos/dice_roller_v2.py
 + 
src/shared/ui/dice_visualizer.py
)
What it is
A standalone Pygame visualiser for D4/D6/D8/D10/D12/D20/D100 dice rolls. The 
dice_visualizer.py
 is the refactored component version — same math, extracted into a class that can be embedded in other scenes (e.g., the dungeon node combat UI).

Die Types and Colors
Sides	Shape	Body color (RGB)	Glow color (RGB)
D4	Triangle	(170, 55, 18)	(255, 140, 60)
D6	Square	(40, 120, 195)	(100, 180, 255)
D8	Octagon	(35, 150, 72)	(80, 220, 120)
D10	Star-10pt	(120, 45, 175)	(190, 100, 255)
D12	Pentagon	(155, 135, 18)	(235, 215, 60)
D20	Hexagon	(55, 55, 175)	(120, 120, 255)
D100	14-gon≈cir	(155, 28, 75)	(235, 80, 145)
3-Phase Roll Animation State Machine
roll() called
  │
  ▼
[FAST phase — 0.55s]
  • Face cycles randomly at 50% probability per frame (blur illusion)
  • Spin speed: 420–740°/s, decays × (1 − dt×1.2) per frame
  • Shake: ±7px intensity decreasing to 0
  • Squash: 1.0 + sin(t×22)×0.07×(1−progress)
  │
  ▼
[DECEL phase — 0.30s, 4 stutter frames at ~75ms each]
  • Stutter sequence: 3 decoy values + final result (never repeats, never early-reveals)
  • Slow residual spin decay × (1 − dt×4.0)
  • Gentle wobble: sin(t×10)×0.03
  │
  ▼
[LANDING phase — 0.28s]
  • t < 0.28 → squash 1.0→0.68, stretch 1.0→1.22  (impact compression)
  • t ≥ 0.28 → ease_out_bounce recovery (Squash 0.68→1.0)
  • Glow ramp: min(1.0, t×2.5)
  • y_offset drops 10px then recovers
  │
  ▼
[SETTLED — 1.4s glow decay]
  • Glow: 1.0 − (timer / 1.4)
  • Shows CRIT! (gold) if display == sides; FUMBLE (red) if display == 1
  │
  ▼
[IDLE]
Pip Layout System (D4 and D6 only)
Pips rendered as circles in [-1, 1] coordinate space, scaled to die radius:

D4 face 1: center (0,0)
D4/D6 face 6: 3×2 grid at ±0.38 in both axes
Key Design Decisions
6/9 disambiguation underline: D10/D100 draws a 1px underline under 6, 9, 16, 19, 60, 66, 90, 96, 99
Full-screen surface bug fix (dice_visualizer.py): uses bounding-box surface instead of WIDTH×HEIGHT surface to avoid VRAM pressure
Embedded version (
dice_visualizer.py
): uses 
tick()
 instead of 
update()
 to fit the component pattern; font lookup uses .get() with graceful skip, not dict access
Rust Transplant Notes
The animation maths are pure: squash, stretch, spin, glow, shake_x/y are all f32 state. A DieAnimState struct in Rust with these fields + phase: DiePhase + timer: f32 is the full contract. The polygon shapes are computed from angles — no assets needed.

PART 2 — CULTURE NODE WARS (
demos/culture_node_wars.py
)
What it is
A real-time Pygame visualiser for a 6-faction cellular automaton running on a procedurally generated Poisson-disk node graph. This is the direct source for 
src/world_map.rs
.

Graph Generation
python
NODE_COUNT     = 180    # nodes total
CONNECT_RADIUS = 110    # max pixel distance to form an edge
MAX_EDGES      = 6      # max connections per node
MIN_DIST       = 45     # minimum spacing between nodes (Poisson-disk approx)
Generation: random scatter → reject points < MIN_DIST apart → connect nearby nodes sorted by distance, stop at MAX_EDGES → BFS for supply chain.

The Six Cultures + Void
ID	Name	Pressure mult	Supply sensitivity	Notes
1	Ember	1.3	1.0	Aggressive expander
2	Gale	1.6	0.6	Fastest spreader, supply-loose
3	Marsh	0.8	0.5	Slow, tenacious, supply-loose
4	Crystal	1.0	1.0	Balanced, needs 2 supply paths
5	Tundra	0.9	0.3	Near supply-immune
6	Tide	1.2	1.5	Supply-fragile, aggressive
7	Void	0.0	0.0	Static barrier, never expands
RPS_BEATS — The Culture Rock-Paper-Scissors Web
Ember   beats {Gale,    Marsh}
Gale    beats {Tundra,  Tide}
Marsh   beats {Crystal, Gale}
Crystal beats {Tide,    Ember}
Tundra  beats {Ember,   Crystal}
Tide    beats {Marsh,   Tundra}
Void    beats nothing
When Culture A attacks Culture B via a shared edge:

A beats B → pressure ×1.4
B beats A → pressure ×0.6
Neutral → pressure ×1.0
The Tick Loop (runs every 0.12 real-world seconds at default speed)
PRESSURE PHASE:
  For each owned node N:
    For each neighbor NB:
      if NB is EMPTY:  NB.pressure[N.culture] += PRESSURE_PER_TICK × pmult × N.strength
      if NB is ENEMY:  NB.pressure[N.culture] += PRESSURE_PER_TICK × pmult × rps_factor
CLAIM PHASE:
  For each EMPTY node N:
    if max(N.pressure) >= PRESSURE_THRESHOLD (60):
      find winner (RPS tiebreak if contested → random if still tied)
      claim_node(N, winner, strength=0.5)
CONTEST PHASE (owned nodes):
  if enemy_pressure > PRESSURE_THRESHOLD × 1.5:
    N.strength -= 0.08
    if strength < 0.15 and random() < 0.15: flip to enemy (strength=0.4)
  pressure decays × 0.85 per tick
SUPPLY PHASE:
  BFS from each culture's Capitol (most-connected node by same-culture neighbors × strength×3)
  supplied = BFS reachable set
  supplied nodes: strength += 0.02; age += 1
  unsupplied nodes: strength -= SUPPLY_DECAY × 0.01 × supply_sensitivity
  if strength <= 0: revert to EMPTY
Visual Contract
Interior node: blended between dim color and interior color by strength
Frontier node (has enemy neighbor): frontier color + bright ring
Capitol: gold color + double ring + white inner ring
Unsupplied: desaturated (40% original + 60% grey average)
Empty under pressure: tinted by leading culture color at ratio × 120 alpha
Channel edges: thick (3px) if both endpoints supplied; thin (1px) if not
Contested edges: split at midpoint, each half in its culture's frontier color
Flow dots: white 2px circles animated along supply channels every 3rd generation
Key Design Decisions
Pressure THRESHOLD = 60 with PRESSURE_PER_TICK = 4 means it takes ~15 ticks of uncontested pressure to claim an empty node (at default pmult=1.0, strength=1.0)
Void nodes are static — they block supply BFS and absorb no pressure
Collapse timer field exists in 
Node
 but is not used in v1 — reserved for v2
Differences from 
src/world_map.rs
Feature	Python Demo	Rust world_map.rs
Node count	180 (random scatter)	15 (named fixed graph)
Graph topology	Poisson-disk generated	Hardcoded 3-ring design
Tick interval	0.12s real-time	60s game-time
RPS system	Full 6-factor web	Simplified (is_opposite)
Supply BFS	Full per-tick	Planned (not yet implemented)
Player interaction	Paint/erase nodes	Send slime squads
The Rust version is intentionally simplified for v1 (15 fixed nodes, no full BFS supply). The Python demo is the design target for when the world_map matures.

PART 3 — FULL ARCHIVE INVENTORY
archive/ root (C:\Github\rpgCore\archive)
archive/
├── README.md                      — archive policy: why files were moved here
├── dead_tests_2026/               — 68 archived test files (see below)
├── legacy_docs_2026/              — 15 legacy markdown docs + 6 subdirs
├── legacy_refactor_2026/          — former Rust rewrite attempt + Python venv
├── legacy_root_2026/              — 57 files: build scripts, session summaries, godot project
├── rendering_donors/              — 3 rendering subsystem variants (godot/, pygame_shim/, terminal/)
├── roster.db.bak                  — SQLite backup of operator roster from v1
├── stories/                       — 3 narrative text files (first_extraction, seasoned_scout, veteran_pilot)
├── superseded_v1/                 — 7 variant subdirs (asset_ingestor, baker, body_legacy, misc, ppu, survival, voyager)
└── world_engine.py.legacy         — 13KB original world engine, replaced by shared/world/
archive/dead_tests_2026/ — 68 files
These are ALL tests that were passing at time of archival but became orphaned when the architecture pivoted. They are the gold mine for understanding what the system could do.

Key tests by system area:

Test file	What it covered
test_genetic_breeding.py
Full genome splice, stat inheritance, mutation
test_faction_system.py
Culture faction pressure, supply, RPS resolution
test_combat_system.py
Turn-based D20 combat, stance transitions
test_territorial_grid.py
 (35KB!)	The original grid-based Cell War implementation
test_headless_derby.py
 (20KB)	Race engine running without display
test_multi_pass_rendering.py
Multi-layer pygame surface composition
test_raycasting_engine.py
First-person raycaster (Wolfenstein style)
test_sprint_e1_turbo_synthesis.py
Genetic synthesis with cooldown
test_sprint_e2_derby_engine.py
Race derby with terrain effects
test_sprint_e3_tycoon_orchestration.py
Resource/economy orchestration
test_adr_218_synthesis.py
Synthesis ADR compliance test
test_deep_time_simulation.py
Long-run faction simulation
test_ascii_doom.py
Terminal renderer for dungeon maps
test_cross_platform_loop.py
Platform-agnostic game loop contract
Designer note: 
test_territorial_grid.py
 (35.9KB) contains the v1 grid-based Cell War. The node-graph version (
culture_node_wars.py
) superseded it. Read both if designing the server-side simulation for the world map.

archive/legacy_docs_2026/ — 15 docs + 6 subdirs
Key documents:

File	Content
SYSTEM_MANUAL.md
 (16KB)	Full v1 system manual — architecture, contracts, CLI
VISION.md
 (7.9KB)	Original product vision document
TECHNICAL_VISIONARY_SUMMARY.md
 (7.7KB)	ADR rationale summary
TURBOSHELLS_AUDIT_REPORT.md
 (10KB)	Full audit of the TurboShells (racing) subsystem
DEPLOYMENT_LOCK_v1.0.md
 (9.5KB)	v1.0 deployment lockfile — what passed at ship
IMPLEMENTATION_DECISION_TREE.md
 (10KB)	Design decision flowchart
COLONY_SYSTEM.md
 (5.8KB)	Colony/base building system (not yet ported)
COMPONENT_ANALYSIS.md
 (4.7KB)	Component architecture analysis
REPO_AUDIT.md
 (7.9KB)	Full repository audit
VERSION.md
 (7.2KB)	Version history
MILESTONES.md
 (5KB)	Sprint milestone tracker
SESSION_2026_02_14_SUMMARY.md
 (4KB)	Feb 14 session recap
STATE.md
 (6KB)	System state snapshot
SCENE_MANAGER.md
 (2.7KB)	Scene manager design
ROADMAP.md
 (2.4KB)	Feature roadmap
Subdirs:

adr/ — Legacy ADR documents (pre-Rust pivot)
architecture/ — Architecture diagrams
benchmarks/ — Performance benchmark results
guides/ — Dev guides and onboarding docs
production/ — Production validation artifacts
summaries/ — Session summaries
archive/legacy_root_2026/ — 57 files
The old project root, archived wholesale. Contains:

14 build logs (
build_log_attempt3.txt
 through 
build_log_attempt14.txt
) — documents the 14 failed attempts to build the Godot project, averaging 45–77KB each. Rich failure archaeology.
SESSION_SUMMARY.md
 (16KB) — most comprehensive single-session summary in the archive
DELIVERABLES.md
 (13KB) — original deliverables spec for v1
py_inventory.md
 (38KB!) — complete Python file inventory from the v1 root, invaluable
src_tree.txt
 (142KB!) — full directory tree with all file sizes from v1
godot_project/ — the abandoned Godot port
legacy_logic_extraction/ — extracted game logic from Godot scripts
scripts/, tools/ — build automation and utility scripts
archive/rendering_donors/
Three rendering subsystem experiments, ready to be "donor transplanted":

godot/ — GDScript rendering components
pygame_shim/ — pygame compatibility shim for headless/test environments
terminal/ — ASCII/ANSI terminal renderer (feeds 
test_ascii_doom.py
)
archive/stories/
Three narrative text files defining the game world's lore voice:

first_extraction.md
 — narrative: first slime team exits the ship
seasoned_scout.md
 — narrative: experienced slime explorer voice
veteran_pilot.md
 — narrative: the Astronaut's internal monologue
Designer note: These are the canonical voice samples. Any in-game log entry should match the register of 
veteran_pilot.md
.

archive/superseded_v1/
7 versioned system variants, superseded by the current architecture:

asset_ingestor_variants/ — asset loading system evolution
baker_variants/ — sprite baking pipeline
body_legacy/ — early SlimeBody rendering (pre-CultureExpression)
misc/ — miscellaneous experiments
ppu_variants/ — PPU (pixel processing unit) rendering variants
survival_variants/ — early survival system (pre-expedition)
voyager_variants/ — world navigation prototypes
docs/agents/ARCHIVE/
The Agent System Archive — prompts, memory, and session logs from rpgCore's AI development pipeline:

docs/agents/ARCHIVE/
├── ARCHIVE_INDEX.md             — master index of this archive
├── AUTONOMOUS_SWARM_GUIDE.md    — multi-agent coordination guide (11KB)
├── ECOSYSTEM_ANALYSIS.md        — analysis of the full agent ecosystem (7.5KB)
├── SELF_AWARE_SWARM_GUIDE.md    — advanced swarm patterns (10KB)
├── examples/
│   └── swarm_integration_examples.md — concrete swarm usage examples
├── memory/
│   └── agent_memory.md          — persistent agent state format
├── prompts/                     — 22 agent system prompt files:
│   ├── analyzer_system.md       — Code analyzer agent
│   ├── archivist_system.md      — Archive management agent (v1 + v2 + fewshot)
│   ├── coder_system.md          — Implementation agent
│   ├── coordinator_system.md    — Multi-agent coordinator
│   ├── docstring_system.md      — Docstring writer
│   ├── executor_system.md       — Command execution agent
│   ├── generic_system.md        — General-purpose agent
│   ├── herald_system.md         — Communication/notification agent
│   ├── planner_system.md        — Planning agent
│   ├── quality_log.md           — Quality gate log
│   ├── reviewer_system.md       — Code review agent
│   ├── scribe_system.md         — Documentation writer
│   ├── strategist_system.md     — High-level strategy agent
│   └── tester_system.md         — Test writing agent
└── session_logs/                — Historical session transcripts
PART 4 — TRANSPLANT PRIORITY MATRIX
What the OPERATOR Rust rewrite still needs from rpgCore, by priority:

Priority	What	Source file	Target
🔴 HIGH	Die animation state machine	
dice_roller_v2.py
src/dice.rs (future)
🔴 HIGH	Full RPS_BEATS pressure system	
culture_node_wars.py
src/world_map.rs
 (v2)
🔴 HIGH	BFS supply chain	culture_node_wars.py::compute_supply()	
src/world_map.rs
🟡 MED	Capitol election logic	culture_node_wars.py::find_capitols()	
src/world_map.rs
🟡 MED	Node narrative voice	
archive/stories/veteran_pilot.md
src/log_engine.rs
🟡 MED	Colony/base building	
legacy_docs_2026/COLONY_SYSTEM.md
src/base.rs (Sprint 5+)
🟢 LOW	Terminal renderer	archive/rendering_donors/terminal/	src/terminal_ui.rs (debug)
🟢 LOW	Sumo/arena combat	(in apps/ — not yet inventoried)	src/arena.rs
PART 5 — KEY MATH CONSTANTS (for Rust implementation)
Culture Node Wars constants to carry over exactly:
rust
const PRESSURE_THRESHOLD: f32  = 60.0;
const PRESSURE_PER_TICK:  f32  = 4.0;
const SUPPLY_DECAY:       f32  = 2.0;   // × supply_sensitivity × 0.01 per tick
const PRESSURE_DECAY:     f32  = 0.85;  // multiplicative per tick
const FLIP_CHANCE:        f32  = 0.15;  // random() on strength < 0.15
const FLIP_THRESHOLD:     f32  = 1.5;   // enemy_pressure > THRESHOLD × this to contest
Dice animation timings:
rust
const FAST_DURATION:  f32 = 0.55;
const DECEL_DURATION: f32 = 0.30;
const LAND_DURATION:  f32 = 0.28;
const SETTLE_GLOW:    f32 = 1.4;
const STUTTER_FRAMES: u32 = 4;
const STUTTER_HOLD:   f32 = DECEL_DURATION / STUTTER_FRAMES as f32; // 0.075s