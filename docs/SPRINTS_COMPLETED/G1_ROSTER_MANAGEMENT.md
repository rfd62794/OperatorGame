# Sprint G.1: Roster Management (Retroactive SDD)

## Vision
Allow players to acquire, view, and manage a biological roster of slime "Operators" for tactical deployment.

## User Stories
- **As a Player**, I want to view all my slimes in a grid so I can see their stats (STR, AGI, INT) and current state (Idle, Deployed, Injured).
- **As a Player**, I want to see detailed profiles of each slime so I can plan squad composition for missions.
- **As a Designer**, I want all operator data to persist across sessions so that player progress is preserved.

## Acceptance Criteria
- [x] Operators display in a responsive card-based grid (Manifest tab).
- [x] Each card shows: Name, Stats (STR/AGI/INT), and Current State (Idle/Deployed/Injured).
- [x] Tapping a card opens a detailed profile view (Stats, Traits, Gear).
- [x] UI is mobile-first (400x800) but scales to desktop (1200x800).
- [x] Operators persist in `save.json` using the centralized `persistence.rs` layer.

## Data Model
```rust
pub struct Operator {
    pub id: uuid::Uuid,
    pub name: String,
    pub level: u32,
    pub xp: u32,
    pub hp: f32,
    pub max_hp: f32,
    pub stats: SlimeStats,      // STR, AGI, INT
    pub genetics: GeneticProfile, // Culture, traits, tier
    pub state: SlimeState,      // Idle, Deployed, Injured
    pub equipped_hat: Option<HatId>,
}

pub struct SlimeStats {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
}

pub enum SlimeState {
    Idle,
    Deployed,
    Injured(chrono::DateTime<chrono::Utc>),
}
```

## Implementation Phases
1. **Phase 1: Persistence**: Defined the `Operator` struct and wired it to the `GameState` in `persistence.rs`.
2. **Phase 2: Manifest UI**: Implemented the `render_manifest` card grid with 44dp tap targets.
3. **Phase 3: Detail View**: Added the side-panel (or popup) to show full operator stats and gear.

## Testing
- **Unit**: `test_operator_creation()` and `test_save_load_operator()`.
- **Integration**: Card renders correctly in the mobile emulator at 400x800.
- **Manual**: Create operator, save, reload app, verify operator still exists.
