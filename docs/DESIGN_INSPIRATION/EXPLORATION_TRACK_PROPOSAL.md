# Design Proposal: Exploration Track System (G.7+)

## Vision
Visualize the sequential "Gauntlet" missions (G.5) as a spatial/temporal journey along a 1D dungeon track. This system transforms instant-resolution mission logs into an animated "march" where players see their squad encounter, engage, and progress through targets.

## Inspiration
- **Beastie Bay DX**: Walking character animations and percentage progress.
- **Slime Garden**: Procedural node-based traversal and stat-gated checkpoints.

## Proposed Mechanics

### 1. Spatial Progression
- Missions are no longer instantaneous; squads visually move from one "Target" to the next along a track.
- **Track Length**: Normalized to 100 virtual units. Waypoints are placed at `100 / (targets.len() + 1)` intervals.

### 2. Waypoint Interaction
- When a squad reaches a waypoint:
    1. **Encounter Pause**: Movement stops. The target's icon/name appears.
    2. **Combat Phase**: The check is performed (as implemented in G.5).
    3. **Resolution Phase**:
        - **Success**: Squad plays a victory pulse and continues to next target.
        - **Failure**: Squad plays a retreat animation (moving backward) and mission ends.

### 3. Partial Reward Visualization
- Players can see rewards accumulating as the squad passes each waypoint.
- "Loot" icons appear on the track at intervals, collected as the squad moves.

## Data Model (G.7 Scoping)
```rust
pub struct TrackProgress {
    pub mission_id: uuid::Uuid,
    pub current_node_idx: usize,
    pub virtual_pos: f32, // 0.0 to 1.0 represented as progress
    pub speed: f32,       // Units per real-second
    pub is_retreating: bool,
}
```

## UI & Layout (400x800)
- **Portrait Version**: A vertical track where the squad (at the bottom) moves upward toward "The End".
- **Visual Cues**: 44dp interaction points for pausing or manual retreat.
- **Progress Bar**: Persistent indicator of total gauntlet depth.

## Implementation Roadmap
- **Phase 1 (G.7)**: Static track rendering in the **Operations** tab.
- **Phase 2 (G.8)**: Smooth linear animation and state machine (Idle -> Move -> Combat -> Success/Retreat).
- **Phase 3 (G.9)**: Visual polish (particles, combat impact effects).
- **Phase 4 (G.10)**: Ambient track decorators per biome.
