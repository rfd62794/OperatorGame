# ADR-013 — Crashed Ship Tech Tree (Demo Unlock Progression)
**Status:** ACCEPTED | **Date:** 2026-03-04 | **Sprint 3**

---

## Context

The OPERATOR Command Deck is now a multi-layer system (Macro/Meso/Micro). The three dormant
simulation modules (`dungeon.rs`, `racing.rs`, `combat.rs` turn-order) need a progression gate so
they are introduced to the player organically, not dumped all at once.

The narrative frame — **a Xeno-Geneticist in a crashed ship** — provides the perfect rationale:
the ship's systems are damaged and must be repaired using **Biomass Scrap** mined from the planet.
Each repair unlocks a new expedition mode (demo).

## Decision

The ship's repair state is tracked as a `tech_tier: u8` field in `GameState` (0–8, matching `GeneticTier`).
Each tier unlocks one major feature. Tier is advanced by completing the corresponding demo expedition.

## Tech Tree

| Tier | Slime Requirement | Ship Component Repaired | Feature Unlocked |
|------|------------------|------------------------|------------------|
| 0 | — | Emergency Power | Command Deck online. Hatch + Slime Manifest. |
| 1 | Blooded (pure culture) | Life Support | Bio-Incubator: Genetic Synthesis. |
| 2 | Bordered (2 adj cultures) | Terrain Mapping Satellite | **Scouting Run** (Race demo) |
| 3 | Sundered (2 opp cultures) | Atmospheric Resonator | Culture-zone advantage live in D20 checks |
| 4 | Drifted (2 skip-one cultures) | Sub-Surface Scanner | **Excavation Run** (Dungeon demo) |
| 5 | Threaded (3 cultures) | Bio-Stress Analyzer | **Territory Dispute** (Sumo demo) |
| 6 | Convergent (4 cultures) | Propulsion Core #1 | **Crash Site Defence** (Tower Defense) |
| 7 | Liminal (5 cultures) | Jump Drive Capacitors | Faction Campaign (Slime Clan) |
| 8 | Void (all 6 cultures) | **THE SHIP** | Planetary Escape — Endgame / Roguelike Ascension |

## Implementation Notes

### `GameState.tech_tier: u8`
- Added with `#[serde(default)]` for backward compatibility (absent = 0).
- Increased by game logic when a repair milestone completes.

### Unlock Check (pseudo-Rust)
```rust
pub fn can_unlock_next_tier(state: &GameState) -> bool {
    let required_tier = GeneticTier::from_u8(state.tech_tier + 1);
    state.slimes.iter().any(|s| s.genetic_tier() as u8 >= required_tier as u8)
}
```

### Phase 1 (Sprint 3)
Only the data layer: `tech_tier` field in persistence, `can_unlock_next_tier()` helper.
The UI gate (greying out menu items, narrative log on repair) comes in a later sprint.

## Consequences

**Positive:**
- The player sees a clear goal (breed a Sundered slime → unlock dungeon)
- The 8-tier GeneticTier system doubles as the progression spine — no separate XP bar needed
- Players are naturally guided toward each demo via the breeding loop

**Negative:**
- High-tier demos (Tower Defense, Faction) are locked until late game — may frustrate returning users.
  *Mitigation: `--dev` flag bypasses tech_tier gate.*

## Related Decisions
- ADR-005: GeneticTier — the same enum drives both genetics and unlock gates
- ADR-010: Incubator Protocol — Tier 1 unlock (Life Support repair = incubator online)
- UNIFIED_SYSTEMS_MAP.md §2 — The original systems map that defined this tech tree
