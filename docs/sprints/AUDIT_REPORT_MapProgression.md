# Audit Report: Map Progression System

Inventory of existing world map architecture and mission-to-node relationships.

## §A — World Map Structure

The world map is a radial graph of **19 nodes** distributed across 3 rings.

- **Nodes**: Defined as `WorldNode` in `src/world_map.rs` (L118).
- **Center Node (ID 0)**: "Hidden Meadow". Ring 0. Always present and neutral (`Culture::Void`).
- **Rings**: 
  - **Ring 1**: 6 nodes (Radius 120). Assigned primary cultures (Ember, Gale, Tide).
  - **Ring 2**: 6 nodes (Radius 220). Assigned secondary cultures (Marsh, Crystal, Tundra).
  - **Ring 3**: 6 nodes (Radius 320). Assigned primary cultures (Ember, Gale, Tide).
- **Node Data**: 
  - Tracks `id`, `ring`, `position` (x, y), `owner` (Culture), and `influence`.
  - Also tracks `difficulty_dc` (individual node difficulty) and `occupied` (expedition active).
  - **CRITICAL**: There is **no** `unlocked` or `discovered` flag on individual nodes or the `WorldMap` struct.

## §B — Mission-to-Node Relationship

Current missions and map nodes exist as **disconnected systems**.

- **Mission Pool**: Generated as a flat pool of 14 items via `generate_static_missions` in `world_map.rs` (L587).
- **Linkage**: 
  - The `Mission` struct has **no** `node_id` or `zone_id` field.
  - The `WorldNode` struct has **no** reference to available missions.
- **Node-based Content**: There is a legacy `ExpeditionTarget` system (L672) with 9 sites, but it is not integrated with the `WorldMap` node graph.
- **Progression**: Completing a mission does not currently trigger any state changes on map nodes.

## §C — Map Rendering (`radar.rs`)

The map view is a canvas-based radial visualization.

- **Node Color**: Pulled directly from `culture_accent(node.owner)`. 
- **Icons/States**: 
  - Nodes pulse if `influence < 0.3` (contested).
  - Nodes have a white stroke (L65).
  - **CRITICAL**: All nodes currently render in full color regardless of "unlocked" status. There is no grayscale or "undiscovered" visual state implemented.
- **Scaling**: Coordinates are scaled based on safe area width to ensure mobile fit (L19).

## §D — GameState Persistence

- **WorldMap**: Serialized as a field on `GameState` (L118).
- **Node States**: `owner`, `influence`, and `occupied` are preserved across sessions.
- **Gating Content**: There is no `zones_unlocked` list in `GameState`. The `missions` pool is re-seeded daily from a date-based RNG, ignoring map state.

## §E — Existing Nomenclature & Mechanics

- **"Zone"**: Primarily refers to `NodeZoneType` (the category of mission, e.g., "Excavation") or elemental affinity in combat.
- **"Territory"**: Used only in the mission name "Territory Dispute".
- **Unlock Mechanics**: The only existing unlock flag is `lens_unlocked` (genetics). 
- **Blockers for Phase G.2**:
  - `WorldNode` requires an `unlocked: bool` field (or `GameState` needs `unlocked_nodes: HashSet<usize>`).
  - `generate_static_missions` needs to be replaced or modified to filter by/link to unlocked nodes.
## §F — Legacy Heritage (`ExpeditionTarget`)

The `seed_expedition_targets` function (L693) specifies 9 canonical sites that map 1:1 to the game's non-void cultures.

| Name | Culture | Danger | Yield (B/S/R) | Ring Alignment |
|------|---------|--------|---------------|----------------|
| **Ember Flats** | Ember | 0.20 | 15 / 5 / 2 | Ring 1 / 3 |
| **Gale Ridge** | Gale | 0.35 | 8 / 3 / 8 | Ring 1 / 3 |
| **Marsh Delta** | Marsh | 0.15 | 25 / 2 / 3 | Ring 2 |
| **Crystal Spire** | Crystal | 0.50 | 5 / 8 / 15 | Ring 2 |
| **Tundra Shelf** | Tundra | 0.55 | 10 / 20 / 5 | Ring 2 |
| **Tide Basin** | Tide | 0.30 | 12 / 10 / 6 | Ring 1 / 3 |
| **Orange Reach** | Orange | 0.35 | 12 / 0 / 0 | Ring 2 |
| **Teal Shelf** | Teal | 0.45 | 0 / 0 / 4 | Ring 1 / 3 |
| **Frost Basin** | Frost | 0.60 | 0 / 18 / 0 | Ring 2 |

**Findings**:
- **Naming Overlap**: Names like `Gale Ridge` and `Tide Basin` are already hardcoded into the Ring 1 node generation logic.
- **Content Potential**: These 9 sites provide a coherent set of resource yields and danger levels that can be converted into "Node-Link Missions."
- **Nomenclature**: The term "Basin", "Ridge", and "Shelf" provides consistent geographic flavor across the map.
- **Status**: Currently, these are entirely unconnected to the `WorldNode` graph. They exist as a legacy side-system for "Expeditions" (Sprint 3) rather than "Missions" (G.1).
