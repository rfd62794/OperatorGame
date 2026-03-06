# ADR-004 — Success Formula: Per-Attribute Scoring with Incompetence Penalty
> **Status:** Accepted | 2026-03-04

## Context
The core game loop requires a success probability calculation that:
1. Rewards balanced squads over single-stat stacking.
2. Makes "wrong-job" missions feel genuinely dangerous, not just slightly harder.
3. Remains scrutable — a player must be able to predict outcomes intuitively.

Two approaches were evaluated:

**Approach A — Pooled Stats (Original Spec):**
```
success% = (Σ all squad stats / Σ all thresholds) × (1 - difficulty)
```

**Approach B — Per-Attribute Scoring (Accepted):**
```
str_score = min(squad_str / req_str, 1.0)
agi_score = min(squad_agi / req_agi, 1.0)
int_score = min(squad_int / req_int, 1.0)
success%  = ((str_score + agi_score + int_score) / 3) × (1 - difficulty)
```

## Decision
**Approach B — Per-Attribute Scoring** is implemented in `Mission::calculate_success_rate()`.

## Rationale
Under Approach A, a squad of three Strength-200 Breachers can compensate for zero Intelligence on a hacking mission by raw stat volume. This collapses the strategic depth of Job selection.

Under Approach B, the same squad scores `(1.0 + 1.0 + 0.0) / 3 = 0.67` *before* difficulty — and 0.0 on the INT axis regardless of how many Breachers you stack. This creates a hard incentive structure: **bring the right operator or accept the penalty**.

**Zero-threshold guard:** Missions may omit a stat requirement (e.g., a pure strength extraction with no hacking needed). Setting `req_intelligence = 0` would cause division by zero. The guard treats `req = 0` as trivially satisfied (`score = 1.0`), allowing flavor missions without defensive coding at call sites.

**Critical Failure floor:** A separate 5% roll (`roll >= 0.95`) triggers operator death regardless of success rate. This is intentionally disconnected from the formula to create the "XCOM moment" — a guaranteed element of irreducible risk that no amount of preparation eliminates.

## Consequences
- **Positive:** Squad composition is a genuine strategic decision. Analysts are mandatory for INT-heavy contracts.
- **Positive:** Formula is legible: three scores, averaged, penalty applied. Players can reverse-engineer it.
- **Negative:** Over-statted squads are capped at 1.0 per attribute — excess stats have no value. This is intentional (diminishing returns as a balance mechanic) but should be communicated to the player via UI.
- **Future:** Introduce `over_threshold_bonus` (+luck modifier) as a Tier 2 balance lever without touching the core formula.
