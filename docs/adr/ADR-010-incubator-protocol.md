# ADR-010 — Incubator Protocol (Genetic Synthesis)
**Status:** ACCEPTED | **Date:** 2026-03-04 | **Sprint 2**

---

## Context

The "Splice" command produced offspring instantly. In the narrative frame of a **Xeno-Geneticist
operating via a crashed ship bio-lab**, this made no sense:

- Sampling genetic material from live donors should temporarily exhaust them ("Cellular Exhaustion")
- Growing a new organism in a shipboard incubator takes real time (the ship must divert power)
- Instantly popping a Tier-8 Void slime from two Tier-1 donors felt like cheating

The goal is to make the cost of breeding *tangible* without being punishing.

## Decision

**Rename:** "Breeding" / "Splice" → **"Genetic Synthesis"** or **"Fusion Protocol"** in all UI labels.

### Mechanism

#### 1. Cellular Exhaustion (Parent Cooldown)
After `operator splice` runs, each donor enters **Cellular Exhaustion** for **10 minutes** (`600s`).

```
SlimeGenome.synthesis_cooldown_until: Option<DateTime<Utc>>

Before breeding: check that Utc::now() >= synthesis_cooldown_until (if Some)
After  breeding: set synthesis_cooldown_until = Some(Utc::now() + 600s) on both parents
```

**Rationale:** The Ratchet Effect (stats never regress) remains intact — the cooldown is a *time cost*, not a stat cost. This prevents spamming the incubator while keeping every synthesis meaningful.

#### 2. Incubation Slot (IncubatingGenome)
The offspring is not immediately available. It enters a **Synthesizing** state persisted in `GameState`:

```rust
pub struct IncubatingGenome {
    pub genome:        SlimeGenome,
    pub completes_at:  DateTime<Utc>,
}
```

`GameState.incubating: Vec<IncubatingGenome>` — written atomically (same `.tmp` → rename pattern).

**Incubation duration** is **15 minutes** (`900s`) by default. A higher-tier offspring (Sundered+) incubates 5 minutes longer per tier above Bordered.

```
duration = 900 + max(0, tier - 2) * 300   [seconds]
Blooded:    900s (15 min)
Bordered:   900s (15 min)
Sundered:  1200s (20 min)
Drifted:   1200s (20 min)
Threaded:  1500s (25 min)
Convergent:1800s (30 min)
Liminal:   2100s (35 min)
Void:      2400s (40 min)
```

**Collect:** `operator incubate` — lists active syntheses and moves any completed ones to the stable.

#### 3. The Void Glitch (1% Easter Egg)
If two **Sundered** (Tier-3, opposite cultures) donors are synthesized, there is a **1% chance** the ship's incubator produces a **Tier-8 Void** genome directly — skipping the normal culture blending.

```rust
let sundered_glitch = a.genetic_tier() == GeneticTier::Sundered
    && b.genetic_tier() == GeneticTier::Sundered
    && rng.gen::<f32>() < 0.01;

if sundered_glitch {
    offspring.culture_expr = CultureExpression::void();
    // Log: "⚡ INCUBATOR ANOMALY — VOID GENESIS EVENT"
}
```

This is communicated to the player as a rare log event, never shown as a percentage.

## Consequences

**Positive:**
- Breeding has a narrative cost (time) without destroying the Ratchet Effect
- The incubation progress bar is a War Room "ambient animation" that keeps the UI alive between missions
- The Void Glitch provides a memorable discovery moment without being exploitable

**Negative:**
- 10-minute parent cooldown can feel slow in testing. *Mitigation: `--dev` flag sets cooldown to 5 seconds.*
- Incubation state requires a new persistence field — mitigated by `#[serde(default)]` on `GameState.incubating`

## Alternatives Considered

- **Biomass cost (resource fee)**: DEFERRED to Sprint 3 — requires resource system first
- **Instant offspring, no cooldown**: REJECTED — removes narrative weight
- **Permanent stat drain on parents**: REJECTED — violates the Ratchet Effect design principle

## Related Decisions
- ADR-005: GeneticTier calculation used for Void Glitch condition check
- ADR-006: D20 used for future Genetic Stability Check roll during incubation
- SPRINT-2-COMBAT.md — acceptance criteria
