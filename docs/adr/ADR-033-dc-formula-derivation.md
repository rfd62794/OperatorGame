# ADR-033 — Mission DC Formula Derivation
> **Status:** Accepted | 2026-03-27

## Context
Mission difficulty classes (DCs) were previously hand-tuned or assigned arbitrarily across different tiers. This made it difficult to predict the impact of stat growth (genetics) on player progression and led to "progression walls" where missions were either trivially easy or mathematically impossible. A deterministic mapping from heritage "danger" values to D20 difficulty classes was required.

## Decision
All mission DCs are now derived from a centralized formula:
`DC = round(danger * 20).clamp(4, 20)`

Where `danger` is the 0.0–1.0 scalar defined in the mission data.

## Rationale
By anchoring the DC to a linear scaling of the danger value, we ensure that the difficulty curve matches the player's expected stat growth. The clamp ensures a "floor" of DC 4 (fair for solo hatchlings) and a "ceiling" of DC 20 (reserved for elite veteran squads).

## Consequences
- **Positive:** Mission difficulty is mathematically predictable and easier to balance across tiers.
- **Positive:** Standardizes success chance expectations across different cultures and biomes.
- **Negative:** Hand-tuning individual missions for "flavor" difficulty is now discouraged in favor of the formula.
