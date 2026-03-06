# Sprint 1 — Genetics Engine
> **Status:** ✅ SHIPPED | 2026-03-04 | 24 tests passing

---

## Goal
Implement the biological simulation layer extracted from the rpgCore systemic audit.
A player must be able to `hatch`, `level`, and `splice` slimes, with results persisting to `save.json`.

## Deliverables

| Deliverable | File | Status |
|-------------|------|--------|
| Culture enum + hex-wheel adjacency | `src/genetics.rs` | ✅ |
| CultureExpression (6-float normalised vector) | `src/genetics.rs` | ✅ |
| GeneticTier (8 tiers from hexagon) | `src/genetics.rs` | ✅ |
| LifeStage gates (Hatchling → Elder) | `src/genetics.rs` | ✅ |
| SlimeGenome (fully `Serialize + Deserialize`) | `src/genetics.rs` | ✅ |
| BreedingResolver (3-rule stat inheritance + ratchet + mutation) | `src/genetics.rs` | ✅ |
| `generate_random()` seeded from culture | `src/genetics.rs` | ✅ |
| `slimes: Vec<SlimeGenome>` in `GameState` | `src/persistence.rs` | ✅ |
| `operator hatch <name> <culture>` | `src/cli.rs + main.rs` | ✅ |
| `operator splice <id_a> <id_b> <name>` | `src/cli.rs + main.rs` | ✅ |
| `operator slimes` | `src/cli.rs + main.rs` | ✅ |
| ADR-005 | `docs/adr/ADR-005-culture-hex-wheel.md` | ✅ |

## Test Results

```
test genetics::tests::test_culture_expression_normalises  ... ok
test genetics::tests::test_genetic_tier_blooded           ... ok
test genetics::tests::test_genetic_tier_bordered          ... ok
test genetics::tests::test_genetic_tier_sundered          ... ok
test genetics::tests::test_genetic_tier_void              ... ok
test genetics::tests::test_stat_ratchet_never_exceeds_cap ... ok
test genetics::tests::test_breed_basic                    ... ok
test genetics::tests::test_life_stage_gates               ... ok
test genetics::tests::test_xp_curve                       ... ok
test genetics::tests::test_race_stats_massive_vs_tiny     ... ok

Total: 24/24 passing
```

## Quick-Start (after `cargo build`)

```bash
# Seed two pure-breed slimes
operator hatch Ember-1 ember
operator hatch Crystal-1 crystal

# List your stable — copy the first 8 chars of each ID
operator slimes

# Splice (parents must be Young+ / level ≥ 4)
operator splice <ember_id> <crystal_id> Sundered-Child
# → Tier 3 Sundered (Crystal × Ember are hex-opposites)
```
