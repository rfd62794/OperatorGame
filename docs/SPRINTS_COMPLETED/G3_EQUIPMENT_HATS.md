# Sprint G.3: Equipment (Hats) (Retroactive SDD)

## Vision
Allow players to purchase and equip "Hats" to provide permanent stat bonuses to their biological operators.

## User Stories
- **As a Player**, I want to buy hats from the Quartermaster (Shop) using Scrap (MTL).
- **As a Player**, I want to equip a hat to a slime and see their stats increase immediately.
- **As a Designer**, I want equipment to be a persistent part of the operator's biological data.

## Acceptance Criteria
- [x] Quartermaster tab shows a list of available hats with prices and stat bonuses.
- [x] Hats provide flat bonuses to STR, AGI, or INT.
- [x] Only one hat can be equipped per operator at a time.
- [x] Equipped hats are saved in `Operator` data and persist across sessions.
- [x] UI updates to show a "Hat" icon on the operator card in the Manifest.

## Data Model
```rust
pub struct Operator {
    // ... existing fields ...
    pub equipped_hat: Option<HatId>,
}

pub struct Hat {
    pub id: HatId,
    pub name: String,
    pub cost: u32,
    pub bonus_str: u32,
    pub bonus_agi: u32,
    pub bonus_int: u32,
}

pub enum HatId {
    None,
    HardHat,      // +5 STR
    Beret,        // +5 INT
    Fedora,       // +5 AGI
    Crown,        // +10 STR, +10 AGI, +10 INT
}
```

## Implementation Phases
1. **Phase 1: Shop UI**: Implemented the `quartermaster.rs` tab with purchase logic and cost validation.
2. **Phase 2: Equipment Logic**: Added the `equipped_hat` field to `Operator` and logic to calculate composite stats.
3. **Phase 3: Visual Polish**: Updated the manifest card to show the hat icon and new stat totals.

## Testing
- **Unit**: `test_equip_hat()` and `test_stat_pumping_with_hats()`.
- **Integration**: Quartermaster correctly deducts MTL and adds hat to inventory/operator.
- **Manual**: Equip hat, verify stat Increase, save/reload, verify hat remains equipped.
