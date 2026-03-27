# ADR-006 — D20 Combat System Adoption
**Status:** ACCEPTED | **Date:** 2026-03-04 | **Sprint 2**

---

## Context

OPERATOR's original mission resolution used a flat `f64` success_chance multiplied by a squad power ratio.
This was statistically correct but had two critical gaps:

1. **Narrative flatness** — "0.73 success" tells no story. A D20 roll with a modifier gives a visible number the player can reason about.
2. **No advantage mechanism** — The Culture hex-wheel had no gameplay leverage. A pure-Ember slime dispatched into an Ember zone was indistinguishable from a mismatched one.

## Decision

Replace `success_chance` with a **D20-based resolution engine** (`src/combat.rs`):

```
total = d20 + modifier
success if total >= dc (and not nat-1)
```

All mission checks, future D20 combat node checks (Sprint 3), and the genetic stability check (future) share this single module.

## Key Formulas

### Stat Coverage → Modifier
```
coverage       = squad_effective_stat / mission_requirement   [0.0–2.0 clamped]
modifier       = round((coverage - 1.0) × 10)
                 coverage 2.0 → +10   |   1.0 → 0   |   0.5 → -5   |   0.0 → -10
```

### Difficulty Scalar → DC
| < 0.15 | 5 | Starter |
| 0.15–0.45 | 10 | Standard |
| 0.45–0.75 | 15 | Advanced |
| 0.75–0.95 | 20 | Elite |

### Success Chance HUD Labels (Sprint G.1)
The UI displays a qualitative label based on the calculated success probability (clamped 0-1):

| Probability | Label |
|-------------|-------|
| `None` | `UNSTAFFED` |
| < 25% | `DESPERATE` |
| 25% – 50% | `DANGEROUS` |
| 50% – 75% | `RISKY` |
| 75% – 100% | `GOOD ODDS` |
| 100% | `GUARANTEED` |

### Culture-Zone Roll Mode
```
slime.dominant_culture == zone_element  → Advantage    (roll 2d20, take max)
slime.dominant_culture.is_opposite(zone) → Disadvantage (roll 2d20, take min)
adjacency or Void                       → Normal        (roll 1d20)
```

### Critical Boundaries (preserved from original design)
```
Nat 20 → always success, regardless of modifier
Nat  1 → always failure  (5% crit-fail floor, per original spec)
```

## Consequences

**Positive:**
- Hex-wheel culture now has mechanical weight in every expedition
- Combat narrative log can render `[Roll: 14 + 3 = 17 vs DC 15 → SUCCESS ✅]`
- Single source of truth for all probabilistic resolution

**Negative:**
- D20 is more variance-heavy than a smooth curve at low roll counts. Players may feel "robbed" by a nat-1 on a 95% mission. *Mitigation: never show the raw probability; show the roll.*

## Alternatives Considered

- **Keep flat probability**: REJECTED — loses narrative value and hex-wheel synergy
- **2d6 bell curve**: REJECTED — harder to explain, breaks natural 20/1 convention
- **Pure percentile (d100)**: REJECTED — too fine-grained, same narrative flatness problem

## Related Decisions
- ADR-005: Culture Hex-Wheel → defines adjacency and opposition used by `culture_zone_mode()`
- ADR-010: Incubator Protocol → uses D20 Genetic Stability Check (future)
- SPRINT-2-COMBAT.md — acceptance criteria
