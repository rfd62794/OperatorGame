# Sprint 3 — Island Expedition
> **Status:** ⬜ PLANNED | Depends on Sprint 2 complete

---

## Goal
Transform the flat Mission → Deployment → AAR loop into a **multi-node roguelike expedition**.
A squad doesn't just "do a mission" — they move through a generated dungeon floor, hitting 3–5 nodes
where each node is its own D20 check against a zone-appropriate DC.

## The Problem Being Solved

A single D20 roll (even after Sprint 2) is a coin flip with modifiers.
The "Island Expedition" makes each mission a *journey*, creating narrative stakes that the combat log can render node-by-node:

```
[Bank Heist Recon] Node 1/4: Combat    — Ghost rolls 17 vs DC 10 → Clear
[Bank Heist Recon] Node 2/4: Trap      — Ghost rolls 4  vs DC 15 → INJURED
[Bank Heist Recon] Node 3/4: Treasure  — Ember zone bonus: +2 modifier
[Bank Heist Recon] Node 4/4: Extraction → SUCCESS — $500 collected
```

## Planned Deliverables

| Deliverable | File | Status |
|-------------|------|--------|
| `ExpeditionNode` enum (Combat/Treasure/Trap/Rest/Elite/Extraction) | `src/world_gen.rs` | ⬜ |
| `DungeonFloor` struct (depth, element, nodes: Vec<ExpeditionNode>) | `src/world_gen.rs` | ⬜ |
| `generate_floor(mission, depth, rng)` — procedural generation | `src/world_gen.rs` | ⬜ |
| Culture-zone bonus: matching dominant culture → Advantage on floor | `src/world_gen.rs` | ⬜ |
| `ExpeditionResult` (victory, xp_gained, gold, injured, killed) | `src/world_gen.rs` | ⬜ |
| `Deployment::resolve_expedition()` — multi-node resolution | `src/models.rs` | ⬜ |
| Node-by-node narrative in `log_engine` | `src/log_engine.rs` | ⬜ |
| War Room progress bar segmented by node count | `src/ui.rs` | ⬜ |
| ADR-007 — Expedition model | `docs/adr/` | ⬜ |

## Floor Generation Spec

```
depth 1 → 3 nodes:  [Combat, Treasure, Extraction]
depth 2 → 4 nodes:  [Combat, Trap OR Rest, Combat, Extraction]
depth 3 → 5 nodes:  [Combat, Elite, Trap, Treasure, Extraction]

zone_element = mission.dominant_stat → mapped to Culture:
    STR-dominant → Ember zone   (Breacher gets Advantage)
    AGI-dominant → Gale zone    (Infiltrator gets Advantage)
    INT-dominant → Crystal zone (Analyst gets Advantage)
    Balanced     → Tide zone    (no bonus)
```

## Culture → Zone Bonus (The Slime Integration Hook)

> This is where Layer A (Mercenaries) and Layer B (Slimes) merge.
> A high-Tier slime with matching culture dispatched alongside the squad
> provides a culture-zone bonus to *all* operator rolls on that floor.

```
if squad has slime with dominant_culture == floor.zone_element:
    all D20 checks on this floor → Advantage
```

## Success Criteria
- [ ] `cargo test` — all existing tests pass
- [ ] `generate_floor()` always ends with `Extraction` node
- [ ] Culture-zone bonus applied correctly when slime dispatched
- [ ] Node outcomes rendered in War Room combat log
- [ ] ADR-007 written and committed
