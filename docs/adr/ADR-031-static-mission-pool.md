# ADR-031 — Static Tiered Mission Board
> **Status:** Accepted | 2026-03-27

## Context
The original mission system regenerated the pool periodically based on a time-seed. This created three problems:
1. **Progress Gaps:** A player might find themselves with only "Elite" missions they couldn't pass, or only "Starter" missions that offered no challenge.
2. **Mid-Flight Expiry:** Missions could disappear from the board while a squad was still en route, complicating resolution logic.
3. **Unpredictability:** Deterministic testing of the core loop was difficult with shifting targets.

## Decision
Adopt a **Static 14-Mission Board** keyed to standardized difficulty tiers:
- **Starter (4):** DC 5-7. Entry-level for L1 slimes.
- **Standard (4):** DC 10-12. Core loop for L3+ squads.
- **Advanced (4):** DC 15-18. High-risk ops.
- **Elite (2):** DC 20-25. Apex content with ~50% success for L5+ veterans.

## Rationale
A static board ensures that a "progression floor" (Starter) and a "progression ceiling" (Elite) are always available. This guarantees that players always have a viable path forward while seeing the rewards they *could* get by leveling up their crew.

## Consequences
- **Positive:** Guaranteed content availability at all player skill levels.
- **Positive:** Missions never cycle out; they are stable targets.
- **Positive:** Test anchors can pin specific missions (e.g., Node 0) for regression testing.
- **Neutral:** New content should be added to the pool rather than replacing existing missions to maintain deployment safety.
