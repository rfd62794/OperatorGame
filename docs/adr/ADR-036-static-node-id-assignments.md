# ADR-036 — Static Node ID Assignments
> **Status:** Accepted | 2026-03-27

## Context
As we moved to a map-gated progression system, we needed stable, non-volatile IDs for the map nodes in Ring 1. These IDs are used to identify which nodes are unlocked, which missions belong to which culture, and which hats are currently purchasable at the Quartermaster.

## Decision
The core node IDs for `Ring 0` and `Ring 1` are now fixed:
- **Center (Hidden Meadow):** 0
- **Ember Flats:** 10
- **Gale Ridge:** 11
- **Tide Basin:** 12
- **Marsh Delta:** 13
- **Crystal Spire:** 14
- **Tundra Shelf:** 15

## Rationale
Consistency and persistence. By having "hardcoded" IDs for the most critical early nodes, we ensure that a player's `unlocked_nodes` set in `save.json` remains valid across different versions of the map generator. This also allows for declarative mission definitions (e.g., "This mission belongs to Node 13").

## Consequences
- **Positive:** Robust save persistence across map updates.
- **Positive:** Easier development and debugging for culture-linked content.
- **Negative:** Any future change to these IDs will break existing save files (migration required).
