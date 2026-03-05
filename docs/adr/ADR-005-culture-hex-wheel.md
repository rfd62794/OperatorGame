# ADR-005 — Culture Hex-Wheel Genetics System

> **Status:** Accepted | **Date:** 2026-03-04 | **Milestone:** Sprint 1

---

## Context

Sprint 1 expanded OPERATOR beyond human mercenaries into a biological simulation layer. The core design question was: **how do you create a meaningful progression system for genetic entities without resorting to arbitrary level gates?**

The rpgCore systemic audit (see `docs/SLIME_BLUEPRINT.md`) extracted a hexagonal culture wheel from the Python codebase — a system where six elemental archetypes are arranged spatially, and the *relationship* between a slime's expressed cultures determines its genetic tier. This is mathematically elegant: tier is computable from a 6-float vector with no lookup tables.

---

## Decision

We adopted the **hexagonal culture wheel** as the primary genetic classification axis, implementated as a `CultureExpression([f32; 6])` newtype with the following invariant:

```
CultureExpression.0.iter().sum() ≈ 1.0  (enforced by normalise() on every write)
```

**The six cultures and their hex positions:**
```
        GALE
   EMBER    CRYSTAL
   MARSH    TIDE
        TUNDRA

Adjacency: Ember↔[Gale, Marsh] | Gale↔[Ember, Tundra] | Crystal↔[Gale, Tide]
           Marsh↔[Ember, Tide] | Tide↔[Crystal, Marsh]  | Tundra↔[Gale, Marsh]
Opposites: Ember↔Crystal | Gale↔Tundra | Marsh↔Tide
```

**GeneticTier** is derived purely from the count of "active" cultures (expression ≥ 0.05) and their wheel relationship — no magic constants, no lookup tables.

The breeding formula uses a **weighted blend + variance + renormalise** pattern:
```
blended[i]    = (parent_a[i] + parent_b[i]) / 2.0
variance      = uniform(−0.15, +0.15)
raw[i]        = max(0.0, blended[i] + variance × blended[i])
expression[i] = raw[i] / sum(raw)     // renormalise
```

Stat inheritance uses the **Ratchet Effect** (no regression):
```
HP  = ratchet(max(a.hp, b.hp),            cap)
ATK = ratchet((a.atk + b.atk) / 2.0,     cap)
SPD = ratchet(max(a.spd, b.spd) × 0.95,  cap)

ratchet(v, cap) = v + (cap − v) × 0.10   // 10% drift toward cap
                  then maybe mutate (5% default, 15% floor with Void parent)
```

---

## Consequences

**Positive:**
- Tier system provides 8 clear progression milestones without arbitrary unlock gates
- All computations are pure functions of `CultureExpression` — deterministic, testable
- `#[serde(default)]` on `GameState.slimes` ensures backward compatibility with old saves
- The Ratchet Effect means players always feel progress — stats never go backward
- Void-tier (all 6 cultures active) requires 7+ generational depth — natural endgame

**Negative / Trade-offs:**
- `f32` precision drift accumulates over many breed cycles; mitigated by `normalise()` + ε threshold
- "Dominant culture" derived from argmax is a lossy representation; accepted at current scale
- Void parentage mutation floor (15%) can occasionally produce negative mutations — accepted by design (called "Mutation Drift" in GDD)

---

## Alternatives Considered

| Alternative | Why Rejected |
|-------------|-------------|
| Simple level-gated tiers (1–10) | No compositional depth; player cannot engineer tier progression |
| Bitflag culture presence | Loses expression gradient; can't represent partial blending |
| Separate `Tier` field stored on `SlimeGenome` | Derived state always stalest; computed property is strictly better |

---

## References

- `src/genetics.rs` — full implementation
- `docs/SLIME_BLUEPRINT.md` — rpgCore audit that sourced these formulas
- `SPEC.md §3` — Genetic Tier Resolution spec
- `SPEC.md §5` — Breeding Resolution (4-step) spec
