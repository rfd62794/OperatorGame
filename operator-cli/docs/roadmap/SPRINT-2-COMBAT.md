# Sprint 2 — D20 Combat Core
> **Status:** 🔄 NEXT | Target: after Sprint 1 stable

---

## Goal
Replace the flat `success_chance` float with a **proper D20 resolution engine**.
Every check — combat, lockpicking, genetic stability — runs through a single auditable resolver.

## The Problem Being Solved

Currently `Deployment::resolve()` rolls a single `f64` against `success_chance`.
This is statistically correct but narratively flat. It cannot express:
- **Advantage/Disadvantage** (squad synergy bonuses)
- **DC ladder** (mission difficulty as a named Difficulty Class, not an opaque scalar)
- **Per-operator rolls** (each soldier rolls independently — more tension, more narrative hooks)

## Planned Deliverables

| Deliverable | File | Status |
|-------------|------|--------|
| `D20Result` struct (roll, modifier, total, dc, success) | `src/combat.rs` | ⬜ |
| `DifficultyClass` enum (Trivial=5 … NearImpossible=30) | `src/combat.rs` | ⬜ |
| `RollMode` enum (Normal, Advantage, Disadvantage) | `src/combat.rs` | ⬜ |
| `d20_check(modifier, dc, mode, rng)` | `src/combat.rs` | ⬜ |
| Wire `Deployment::resolve()` to use D20 checks | `src/models.rs` | ⬜ |
| Map Mission `difficulty` (0.0–0.9) → `DifficultyClass` | `src/models.rs` | ⬜ |
| Squad STR/AGI/INT → per-stat D20 modifier | `src/models.rs` | ⬜ |
| Culture synergy → Advantage on matching-element mission | `src/combat.rs` | ⬜ |
| ADR-006 — D20 adoption | `docs/adr/` | ⬜ |

## Key Formula (replacing flat roll)

```
modifier   = (squad_effective_stat / threshold).clamp(0.0, 2.0) as i32 - 10
             [maps 200% coverage → +10 mod; 100% → 0; 50% → -5]

dc         = DifficultyClass::from_f64(mission.difficulty)
             [0.00–0.20 → Easy(10), 0.21–0.40 → Moderate(15), ...]

roll       = d20 + modifier
outcome    = if roll >= dc { Victory } else if roll < dc - 10 { CritFail } else { Failure }
```

## Success Criteria
- [ ] `cargo test` — all existing 24 tests still pass
- [ ] New `combat` module adds ≥ 5 tests (D20 boundary conditions)
- [ ] `operator deploy` output shows individual D20 rolls in the narrative log
- [ ] ADR-006 written and committed
