# Project State

updated: 2026-03-28
agent: Antigravity

## Status

phase: G.2 Complete — Map-Gated Progression
test_floor: 219 passing, 0 skipped, 0 failing
last_directive: OperatorGame_G2_MapGated_Progression_Directive.md

## What Is Built

OperatorGame is a Rust/egui mobile game running on Android (Moto G, API 35). The core loop is functional: recruit operators via genetics, stage a squad, dispatch on missions, collect AAR results with XP and resource rewards, level up. The world map has 19 radial nodes across 3 rings. Six scouting missions unlock Ring 1 nodes permanently, granting heritage resource yields and exposing culture-specific contracts. Locked nodes render gray; unlocked nodes render in culture color with a one-time pulse animation. The resource economy maps Cash→Recruitment, Biomass/Reagents→Breeding, Scrap→Equipment (planned).

## What Is Next

Sprint G.3: Equipment (Hat) System. Four hats (Scout Hood, Knight Helm, Mage Hood, Commander Cap) purchasable with Scrap from a Quartermaster sub-tab under the Map tab. Hats provide flat stat bonuses feeding into calculate_success_chance() via total_stats(). Unlock conditions gate hat availability behind node scouting. Directive at: OperatorGame_G3_Equipment_Directive.md.

## Open Questions

- SDD-035 radial.rs drift: audit found possible formula discrepancy between SDD and radial.rs implementation. Needs investigation before any difficulty tuning.
- SPEC.md and CONSTITUTION.md are significantly stale (still describe human mercenaries). Scheduled for rewrite after G.3.
- Upkeep system disabled (TODO in persistence.rs). Needs economy balance sprint before re-enabling.
- JNI live insets stub in platform.rs still unimplemented. Hardcoded safe area fallbacks in use.

## Recent Decisions

- ADR-030: SidePanel navigation replaces ui.horizontal for sidebar
- ADR-031: Static 14-mission tiered board (DC 5/10/15/20)
- ADR-032: Orphan protection — active deployments block mission pool removal
- ADR-033: DC formula derivation (round(danger * 20).clamp(4, 20))
- ADR-034: Resource economy mapping (Cash/Biomass/Scrap/Reagents)
- ADR-035: Hat inventory model (return to inventory on replace)
- ADR-036: Static node ID assignments (Ember=10, Gale=11, Tide=12, Marsh=13, Crystal=14, Tundra=15)
- ADR-037: 3-operator squad cap as hard architectural constraint
