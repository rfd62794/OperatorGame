# Sprint G.5: Gauntlet Missions (Retroactive SDD)

## Vision
Expand the mission system from a single-check task to a sequential "Gauntlet" of encounters, while resolving critical deployment reliability bugs.

## User Stories
- **As a Player**, I want to face multiple targets in a single mission (1–3), where each target provides a new challenge.
- **As a Player**, I want to receive partial rewards even if I don't defeat all targets but manage to retreat.
- **As a Designer**, I want all mission resolution to be atomic and session-resilient, ensuring operators are correctly returned to `Idle` status.

## Acceptance Criteria
- [x] `Mission` model updated to use `Vec<Target>` for sequential encounters.
- [x] Combat resolution in `Deployment::resolve` iterates through targets, failing-fast on the first loss.
- [x] "Targets Defeated: X/Y" progress is tracked and displayed in the AAR summary and Combat Log.
- [x] Mission rewards (MTL, XP) scale based on the number of targets defeated (`defeated / total`).
- [x] Reliability Fix: Operator state reset for deployment is moved to the persistence layer for atomicity.

## Data Model
```rust
pub struct Mission {
    // ... existing fields ...
    pub targets: Vec<Target>,
}

pub struct Target {
    pub name: String,
    pub base_dc: u32,
    pub req_strength: u32,
    pub req_agility: u32,
    pub req_intelligence: u32,
}

pub enum AarOutcome {
    Victory { reward: ResourceYield, xp_gained: u32, targets_defeated: usize, total_targets: usize, .. },
    Failure { injured_ids: Vec<Uuid>, xp_gained: u32, targets_defeated: usize, total_targets: usize, .. },
    CriticalFailure { injured_ids: Vec<Uuid>, xp_gained: u32, targets_defeated: usize, total_targets: usize, .. },
}
```

## Implementation Phases
1. **Phase 1: Reliability Refactor**: Moved resolution logic to `GameState::resolve_deployment` in `persistence.rs` to fix the "Persistent Deployment" bug.
2. **Phase 2: Gauntlet Data Model**: Refactored `Mission` and `blueprint` to support `Vec<Target>`.
3. **Phase 3: Sequential Resolution**: Implemented the `resolve` loop with scaled rewards and XP in `mission.rs`.
4. **Phase 4: UI Updates**: Updated `contracts.rs` and `ops.rs` (AAR panel) to display gauntlet requirements and progress.

## Testing
- **Unit**: `test_gauntlet_partial_success()` and `test_operators_freed_after_resolve()`.
- **Integration**: AAR panel correctly shows "2 / 3 TARGETS ELIMINATED" on partial victory.
- **Manual**: Launch a 3-target mission, fail on the 2nd target, verify rewards for the 1st target are granted and operators return to `Idle`.
