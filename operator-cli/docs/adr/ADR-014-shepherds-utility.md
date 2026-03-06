# ADR-014 — Shepherd's Utility (Pikmin-Style Node Requirements)
**Status:** ACCEPTED | **Date:** 2026-03-04 | **Sprint 4**

---

## Context

Closing the campaign loop requires a reason to build a **diverse roster** of slimes rather than
one maxed-out "Uber-Slime." Without specialisation pressure, the genetics system has no feedback
into the player's decisions — you'd just breed the highest-stat slime and send it everywhere.

The "Pikmin Moment": in Pikmin, Yellow Pikmin throw bombs. Red Pikmin fight. Blue Pikmin swim.
You can't beat a water obstacle with pure Reds no matter how many you have.

We need the same mechanic for slimes, without the complexity of elemental weakness charts.

## Decision

Every `WorldNode` has a **ShepherdRequirement** that checks squad composition, not just stat total.

### Requirements

| Requirement | Gate Check | Narrative |
|-------------|-----------|-----------|
| **HeavyLift** | Cumulative body size score ≥ threshold | Mining scrap requires mass |
| **FastScout** | Aggregate SPD ≥ threshold | Terrain mapping needs agile runners |
| **CombatReady** | Aggregate ATK ≥ threshold | Territory fights need fighters |
| **Curious** | Aggregate MND ≥ threshold | Bio-surveys need intelligent observers |
| **Charismatic** | Aggregate CHM ≥ threshold | Trade needs persuasive diplomats |

### Formulas

```
HeavyLift score  = Σ (slime.body_size.scalar() × slime.base_atk)  for all squad members
FastScout score  = Σ slime.base_spd
CombatReady score= Σ slime.base_atk
Curious score    = Σ (slime.curiosity × 10 + slime.energy × 5)    [personality-derived MND proxy]
Charismatic score= Σ (slime.affection × 10 + (1-slime.shyness)×5) [personality-derived CHM proxy]

Threshold = node.influence × base_threshold
base_threshold per type:
  HeavyLift:    50.0
  FastScout:    40.0
  CombatReady:  35.0
  Curious:      30.0
  Charismatic:  30.0
```

This means a **contested node** (low influence) is *easier* to enter — the factions haven't fortified it.
A **controlled node** (high influence) has a harder gate — you need the right squad.

### Profile Card Integration
Each slime's Profile Card displays a utility score row:
```
⚙ LIFT: 47   ⚡ SCOUT: 32   ⚔ FIGHT: 28   🔬 MND: 18   💬 CHM: 22
```
This lets the player "cast" the right squad without a detailed stat sheet.

## Consequences

**Positive:**
- Creates natural specialisation pressure: you need at least one "tank" (Massive/Ember), one
  "scout" (Gale/SPD), one "diplomat" (Tide/CHM), etc.
- Node influence directly modulates difficulty — frontline nodes are accessible even with a weak squad
- No new stat axes needed: CHM and MND are derived from existing personality floats

**Negative:**
- "Curious" and "Charismatic" are personality-derived, so a player can't fully optimise them
  without understanding the personality system. *Mitigation: Profile Card shows the score directly.*
- Threshold scaling (× influence) can feel arbitrary until the player understands the Cell War system.
  *Mitigation: Profile Card shows node requirement next to squad score ("Need 50, have 47").*

## Alternatives Considered

- **Elemental weakness matrix**: REJECTED — too complex, breaks KISS
- **Single stat gate**: REJECTED — removes specialisation incentive
- **Unlock flags (binary gate)**: REJECTED — not granular enough for the influence system

## Related Decisions
- ADR-005: Culture hex-wheel → dominant culture → culture_accent for Profile Card colour
- ADR-006: D20 roll mode derived from squad culture vs node owner culture
- ADR-013: Tech Tree — gates which NodeZoneTypes are even visible at a given tech_tier
- UNIFIED_SYSTEMS_MAP.md §2: Maturity → Resource pipeline (stage gates per zone type)
