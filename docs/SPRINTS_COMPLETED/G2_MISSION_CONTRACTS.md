# Sprint G.2: Mission Contracts (Retroactive SDD)

## Vision
Allow players to view available mission contracts and stage operators for deployment into these tactical challenges.

## User Stories
- **As a Player**, I want to see a list of available contracts with their requirements (Strength, Agility, Intelligence).
- **As a Player**, I want to stage operators (selecting 1-3) to see the probability of success before confirming launch.
- **As a Designer**, I want missions to be procedurally generated but balanced based on existing roster power.

## Acceptance Criteria
- [x] Contracts tab displays available missions with reward, difficulty, and duration.
- [x] Tapping a mission opens the "Briefing" panel with requirement breakdowns (radar chart or bars).
- [x] Staging UI allows selecting idle operators and shows an aggregate success probability.
- [x] "Confirm Dispatch" creates a `Deployment` and sets operator states to `Deployed`.
- [x] Missions persist across launches until accepted or expired.

## Data Model
```rust
pub struct Mission {
    pub id: uuid::Uuid,
    pub name: String,
    pub tier: MissionTier,
    pub targets: Vec<Target>, // G.5 Gauntlet Update
    pub reward: ResourceYield,
    pub duration_secs: u64,
    pub affinity: Option<Culture>,
    pub node_id: Option<usize>,
}

pub struct Deployment {
    pub id: uuid::Uuid,
    pub mission_id: uuid::Uuid,
    pub operator_ids: Vec<uuid::Uuid>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub resolved: bool,
}
```

## Implementation Phases
1. **Phase 1: Generation Engine**: Created `world_map.rs` and `mission::blueprint` for procedural contract creation.
2. **Phase 2: Briefing UI**: Implemented the `contracts.rs` tab with requirement visualizations.
3. **Phase 3: Staging Logic**: Added the selection of operators and probability calculations in `ops.rs`.

## Testing
- **Unit**: `test_mission_generation()` and `test_staging_success_probability()`.
- **Integration**: Briefing UI updates correctly when operators are selected.
- **Manual**: Select mission, select operator, launch, verify timer starts and operator is `Deployed`.
