# ADR-034 — Resource Economy Taxonomy
> **Status:** Accepted | 2026-03-27

## Context
OperatorGame features four core resources ($Cash, Biomass, Scrap, and Reagents) that were previously blurred in their consumption paths. For instance, should equipment be bought with Cash or Scrap? This ambiguity caused friction in player agency and "soft" economies where the player would only focus on the single most efficient yield.

## Decision
The resource economy is now strictly partitioned:
- **Cash ($):** Recruitment of new operators from the Union only.
- **Biomass & Reagents:** Biological synthesis and genetic refinement (Breeding) only.
- **Scrap:** Acquisition and maintenance of professional gear (Equipment) only.

Cross-system spending (e.g., buying a hat with Cash) is forbidden without a specific design document.

## Rationale
Partitioning the economy forces players to engage with all mission types. If you want better operators, you must breed (Biomass). If you want better gear, you must scout industrial wreckage (Scrap). This ensures that no single mission tier or biome becomes the "only" path to progression.

## Consequences
- **Positive:** Encourages diverse gameplay loops across the world map.
- **Positive:** Clearer UI mapping for resource-specific sub-tabs.
- **Negative:** Players cannot "brute force" one system by over-earning in another (e.g., you cannot buy a high-tier helm with excess Cash).
