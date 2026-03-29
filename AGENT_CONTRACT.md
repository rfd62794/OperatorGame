# AGENT_CONTRACT

version: 1.0
repo: OperatorGame
updated: 2026-03-28

## STRUCTURE
# Maps directory paths to purpose

src/                  : All Rust source code. Primary language: Rust + egui/eframe.
src/ui/               : All egui UI rendering modules. One file per major UI surface.
docs/adr/             : Architectural Decision Records. Locked after merge. Numbered sequentially ADR-001+.
docs/sdd/             : System Design Documents. One per major system. Authority over implementation.
docs/sprints/         : Sprint directives and audit reports. Archival — never edited after creation.
docs/state/           : current.md only. Always current. Updated every session.
tests/                : All Rust integration test files. cargo test convention.

## FILE_REGISTRY
# path | purpose | writer | frequency

docs/state/current.md          | Project state snapshot      | both  | every session
docs/GAME_DESIGN.md            | Authoritative design vision | human | on core change
docs/ROADMAP.md                | Canonical sprint sequence   | human | per sprint
docs/adr/ADR-NNN.md            | Decision record             | human | on decision
docs/sprints/*_Directive.md    | Sprint implementation plan  | human | per sprint
docs/sprints/AUDIT_REPORT_*.md | Pre-sprint audit findings   | agent | per sprint
AGENT_CONTRACT.md              | This file                   | human | on structural change

src/models/*            | Modular core data: operator.rs, mission.rs, item.rs, etc.       | agent | on model change
src/persistence.rs      | Save/load, GameState impl, SAVE_VERSION, migration             | agent | on state change
src/world_map.rs        | WorldMap, WorldNode, mission pool generation, ExpeditionTarget | agent | on map change
src/genetics.rs         | Genome generation, culture expression, breeding resolution      | agent | on genetics change
src/combat.rs           | D20 resolution, AarOutcome, calculate_success_chance           | agent | on combat change
src/log_engine.rs      | Log entry formatting and narrative generation                   | agent | rarely
src/platform.rs        | SafeArea, TAB_BAR_HEIGHT, ResponsiveLayout, Android insets     | agent | rarely
src/ui/mod.rs          | OperatorApp struct, update loop, tab routing, sub-tab enums    | agent | on nav change
src/ui/manifest.rs     | Roster tab: operator cards, slime detail, recruit panel        | agent | on roster change
src/ui/ops.rs          | Missions tab: active deployments, AAR panel, PROCESS AAR       | agent | on ops change
src/ui/contracts.rs    | Quest board: scout missions, available contracts, filtering     | agent | on mission UI change
src/ui/radar.rs        | Map tab: radial node map, grayscale locked nodes, pulse        | agent | on map UI change
src/ui/squad.rs        | Squad sub-tab: staged operator stats and totals                | agent | on squad UI change
src/ui/quartermaster.rs| Map sub-tab: hat shop, equip flow (PLANNED — Sprint G.3)       | agent | Sprint G.3

## STATE_SCHEMA
# Required fields in docs/state/current.md

required: [updated, agent, phase, test_floor, what_is_built, what_is_next, open_questions, recent_decisions]

## INVARIANTS

test_floor:   219 passing, 0 failing, 0 skipped — enforced before every directive
save_version: Current SAVE_VERSION = 10. Bump requires migration path. Never break existing saves.
scope:        Every directive lists explicit file scope. Unlisted files are read-only.
phases:       No phase begins without passing test floor from previous phase.
squad_cap:    Hard limit of 3 operators per deployment squad. Not configurable without ADR.
node_ids:     Ring 1 node IDs are fixed: Center=0, Ember=10, Gale=11, Tide=12, Marsh=13, Crystal=14, Tundra=15.
