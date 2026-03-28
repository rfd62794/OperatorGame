# Design Precedent: Slime Garden (rpgCore) Dungeon Tracks

## Overview
*Slime Garden* in the **rpgCore** ecosystem utilized a node-based traversal model for procedural encounters. This system provided the foundational technical framework for "Dungeon Tracks"—a series of waypoints where slimes interact with hazards, enemies, or loot.

## Key Mechanics
1. **Procedural Waypoints**: Tracks aren't static; nodes are generated based on biome parameters and difficulty curves.
2. **Cumulative Hazard Checks**: Success is calculated per-node, with "partial rewards" granted at each stage, mirroring the gauntlet logic.
3. **Stat-Gated Progress**: Advancing through a track requires specific stat thresholds (STR, AGI, INT) to be met at each waypoint.
4. **Visual Progression**: A linear UI tracks the "slime squad" as they move through a series of discrete nodes.

## Lessons for OperatorGame
- **Node Data Model**: The `Vec<Target>` model implemented in G.5 is the correct data foundation for this system.
- **Phased Combat**: Splitting combat into phases (Preparation → Roll → Resolution) allows for better animation timing.
- **Retreat Logic**: Establishing "checkpoints" or safe zones along a track can prevent total failure while increasing tactical depth.
