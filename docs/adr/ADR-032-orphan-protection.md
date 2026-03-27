# ADR-032 — Mission Deployment Safety & Orphan Protection
> **Status:** Accepted | 2026-03-27

## Context
In a dynamic mission system, there is a risk that a `Mission` record is deleted while a `Deployment` is still referencing its `Uuid`. This "Dangling Reference" causes AAR resolution to fail, effectively soft-locking the squad.

## Decision
1. **Deletion Guard:** A mission with an active (unresolved) deployment is **never** removed from the `GameState` pool during a refresh or data migration.
2. **Orphan Reconstruction:** If a deployment is loaded that references a missing `Mission` ID, the loader must reconstruct a "Minimal Safe Mission" record with default values and mark it for `[ORPHANED]` display in the UI.
3. **API Level Safety:** Operations like `resolve_deployment` must handle missing mission data gracefully by falling back to the reconstructed orphan.

## Rationale
Player time is the most valuable currency. Losing a squad to a "missing ID" crash or deletion is an unacceptable UX failure. Orphan reconstruction ensures that even if the developer makes a breaking change to the mission pool, the player can still resolve their active contracts and get their operators back.

## Consequences
- **Positive:** Save files are inherently more robust against schema or content changes.
- **Positive:** Prevents soft-locks and "lost" squads.
- **Negative:** Reconstructed missions may have default (incorrect) rewards or DCs compared to their original state, though this is preferable to a crash.
