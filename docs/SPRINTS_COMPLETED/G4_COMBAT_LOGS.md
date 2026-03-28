# Sprint G.4: Combat Logs (Retroactive SDD)

## Vision
Provide players with a historical record of all mission outcomes, including tactical narratives and result breakdowns.

## User Stories
- **As a Player**, I want to see a log of all past missions so I can track my squad's performance.
- **As a Player**, I want to read flavorful descriptions of combat encounters (e.g., "Splortch walloped the enemy").
- **As a Designer**, I want logs to be color-coded and structured for easy readability.

## Acceptance Criteria
- [x] Combat Log tab (Radar) shows the last 50 mission results.
- [x] Each entry shows: Mission Name, Timestamp, Narrative, and Outcome (Victory/Failure).
- [x] Outcome labels are color-coded: Green (Victory), Red (Failure), Yellow (Partial).
- [x] Narratives are procedurally generated based on the mission's primary stat and difficulty.
- [x] Logs persist in `save.json` using the centralized `persistence.rs` layer.

## Data Model
```rust
pub struct GameState {
    // ... existing fields ...
    pub combat_log: Vec<LogEntry>,
}

pub struct LogEntry {
    pub timestamp: u64,
    pub message: String,
    pub outcome: OutcomeType,
}

pub enum OutcomeType {
    Victory,
    Failure,
    Partial,
    Critical,
}
```

## Implementation Phases
1. **Phase 1: Log Engine**: Implemented `log_engine.rs` to generate flavorful mission narratives based on mission data.
2. **Phase 2: Persistence**: Added the `combat_log` collection to `GameState` and wired it for saving.
3. **Phase 3: Log UI**: Created the `radar.rs` tab (Combat Log) with a scrollable list of historical entries.

## Testing
- **Unit**: `test_narrative_generation()` and `test_log_persistence()`.
- **Integration**: Combat Log renders correctly in mobile emulation at 400x800.
- **Manual**: Launch mission, resolve it, verify new entry appears in the log with correct color and narrative.
