# Project State

updated: 2026-03-29
agent: human (Robert) + Claude

## Status

phase: G.5c Complete — UI Baseline Restored (Final egui State)
test_floor: 156 unit tests + 6 integration suites passing, 0 failing, 0 skipped
last_directive: G.5c UI Restoration & Flutter Transition Decision

## What Is Built

OperatorGame is a Rust tactical genetics game with a stable egui mobile UI. STAGE button functional via two-row header layout. SDD-038 implemented with egui constraints documented. ADR-039 (Header Layout) and ADR-040 (Flutter Migration Deferred) committed. Genetic persistence, combat logic, and world map are UI-independent and verified.

## What Is Next

Sprint F.H1: Flutter Migration Scaffold. Transitioning the UI from egui to Flutter using flutter_rust_bridge. Objective: Rebuild the Roster Collection view in Flutter while preserving the existing Rust game logic.

## Open Questions

- Single-row header (name + buttons same line) deferred per ADR-039 — revisit with Flutter migration.
- flutter_rust_bridge v2 vs v1.0 — which fits the current asynchronous model better?

## Open Questions

- Elder Passive Abilities: what does each culture's Elder passive actually do? Needs design before Level 10 is reachable.
- Mini-Boss design: what is it, narratively and mechanically? Needs SDD before G.10.
- First Boss design: same. Needs SDD.
- Pattern mechanical hooks: when Patterns gain gameplay meaning, what are they? Deferred.
- Elder Void Slime reveal: at what point does the player learn? What triggers it?
- Does the Elder know it is the template? Open narrative question — keep it open.
- JNI live insets: platform.rs safe area insets are hardcoded stubs. Real device values not read at runtime.
- Screenshot automation: OperatorDeviceTools coordinate map is stale after SidePanel refactor.
- SDD-035 radial.rs: confirmed false alarm — ring geometry math and mission DC are separate systems.
- Upkeep system: disabled with TODO in persistence.rs. Needs economy balance sprint before re-enabling.

## Recent Decisions

- ADR-030: SidePanel navigation replaces ui.horizontal for sidebar
- ADR-031: Static 14-mission tiered board
- ADR-032: Orphan protection — active deployments block mission pool removal
- ADR-033: DC formula derivation (round(danger * 20).clamp(4, 20))
- ADR-034: Resource economy mapping (Scrap/Biomass/Reagents)
- ADR-035: Hat inventory model (return to inventory on replace)
- ADR-036: Static node ID assignments (Ember=10, Gale=11, Tide=12, Marsh=13, Crystal=14, Tundra=15)
- ADR-037: 3-operator squad cap as hard architectural constraint
- ADR-038: Modular data layer (src/models/ directory structure)
- GAME_DESIGN.md v2.0: Canonical vision locked — Corporate-Absurdist tone, Void Slime endgame, 9-culture genetics
- ROADMAP.md v2.0: Sprint sequence locked — G.6 Leveling Feel first

## Key Architectural Notes (for new agent sessions)

- Language: Rust + egui/eframe
- Target: Android (Moto G 2025, API 35) + desktop
- Module structure: src/models/ directory (ADR-038), src/ui/ directory
- Save format: JSON via serde, SAVE_VERSION 11
- Squad cap: hard limit 3 operators (ADR-037)
- Node IDs: Center=0, Ember=10, Gale=11, Tide=12, Marsh=13, Crystal=14, Tundra=15 (ADR-036)
- Resource naming: Scrap (not MTL), Biomass, Reagents (ADR-034)
- Tone: astronaut thinks slimes are corporate employees. He is wrong. This is funny.
- Build pipeline: build_android.ps1 → deploy_moto.ps1
- Test command: cargo test
