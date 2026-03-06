# ADR-022: The 9-Point Chromatic Matrix

**Status:** ACCEPTED | **Date:** 2026-03-04 | **Author:** Gemini (via PyPro SDD-Edition)

## Context

The existing 6-point hex system provided basic cultural relationships but lacked the complexity needed for scaling the "Slime Planet" from a basic garden to a complex planetary ecosystem. The Genetic Tier system needed a more sophisticated chromatic framework where the "Void" (Tier 8/9) could serve as the center of a richer chromatic web.

## Decision

Expand the Culture enum from 6 to 9 types plus Void, implementing a Triangle-based Strength/Weakness system where Primary beats Secondary, Secondary beats Tertiary, and Tertiary beats Primary.

## Architecture

### The 9-Point Chromatic Wheel

| Layer | Color | Culture | Dominant Stat | Narrative Role |
|-------|-------|---------|---------------|----------------|
| Primary | Red | EMBER | ATK | Power / Heat / Combat |
| Primary | Blue | TIDE | CHM | Energy / Flow / Diplomacy |
| Primary | Yellow | GALE | SPD | Speed / Wind / Scouting |
| Secondary | Green | MARSH | RES | Balance / Toxic / Survival |
| Secondary | Orange | Orange (New) | MND | Logic / Construction / Engineering |
| Secondary | Purple | CRYSTAL | HP | Tank / Focus / Armor |
| Tertiary | Teal | Teal (New) | STB | Stability / Support |
| Tertiary | Amber | Amber (New) | DUR | Durability / Industry |
| Tertiary | Frost | TUNDRA | END | Preservation / Slow / Stasis |
| Exception | White | VOID | — | Universal Constant |

### Triangle-Based RPS System

```
Primary (Red/Blue/Yellow) 
    ↓ beats
Secondary (Green/Orange/Purple)
    ↓ beats  
Tertiary (Teal/Amber/Frost)
    ↓ beats
Primary (Red/Blue/Yellow)
```

## Implementation

### Culture Enum Expansion

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Culture {
    // Primary Layer (Inner Triangle)
    Ember,    // Red - ATK
    Tide,     // Blue - CHM
    Gale,     // Yellow - SPD
    
    // Secondary Layer (Middle Triangle)
    Marsh,    // Green - RES
    Orange,   // Orange - MND (NEW)
    Crystal,  // Purple - HP
    
    // Tertiary Layer (Outer Triangle)
    Teal,     // Teal - STB (NEW)
    Amber,    // Amber - DUR (NEW)
    Tundra,   // Frost - END
    
    // Exception
    Void,     // White Light - All 9 frequencies fused
}
```

### Layer Classification

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChromaticLayer {
    Primary,    // Red, Blue, Yellow
    Secondary,  // Green, Orange, Purple
    Tertiary,   // Teal, Amber, Frost
    Void,       // White Light
}
```

### Color Mixing Logic

```rust
impl Culture {
    pub fn mix_colors(a: Culture, b: Culture) -> Option<Culture> {
        match (a, b) {
            // Primary + Primary = Secondary
            (Culture::Ember, Culture::Gale) | (Culture::Gale, Culture::Ember) => Some(Culture::Orange),
            (Culture::Gale, Culture::Tide) | (Culture::Tide, Culture::Gale) => Some(Culture::Teal),
            (Culture::Tide, Culture::Ember) | (Culture::Ember, Culture::Tide) => Some(Culture::Purple),
            
            // Secondary + Secondary = Tertiary
            (Culture::Orange, Culture::Marsh) | (Culture::Marsh, Culture::Orange) => Some(Culture::Amber),
            (Culture::Marsh, Culture::Crystal) | (Culture::Crystal, Culture::Marsh) => Some(Culture::Teal),
            (Culture::Crystal, Culture::Orange) | (Culture::Orange, Culture::Crystal) => Some(Culture::Amber),
            
            // Tertiary + Tertiary = Primary (rare)
            (Culture::Teal, Culture::Amber) | (Culture::Amber, Culture::Teal) => Some(Culture::Gale),
            (Culture::Amber, Culture::Frost) | (Culture::Frost, Culture::Amber) => Some(Culture::Ember),
            (Culture::Frost, Culture::Teal) | (Culture::Teal, Culture::Frost) => Some(Culture::Tide),
            
            // Any + Void = Void
            (Culture::Void, _) | (_, Culture::Void) => Some(Culture::Void),
            
            // Same culture mixing = same culture
            (a, b) if a == b => Some(a),
            
            // Other combinations = None (failed synthesis)
            _ => None,
        }
    }
}
```

## Special Mechanics

### Trinity Bonus

```rust
pub fn has_trinity_bonus(squad: &[Culture]) -> bool {
    let has_primary = squad.iter().any(|c| matches!(c, Culture::Ember | Culture::Tide | Culture::Gale));
    let has_secondary = squad.iter().any(|c| matches!(c, Culture::Marsh | Culture::Orange | Culture::Crystal));
    let has_tertiary = squad.iter().any(|c| matches!(c, Culture::Teal | Culture::Amber | Culture::Tundra));
    
    has_primary && has_secondary && has_tertiary
}

pub fn apply_trinity_bonus(base_roll: f32) -> f32 {
    base_roll + 2.0 // +2 to all D20 rolls
}
```

### Void Core Evolution

```rust
impl Culture {
    pub fn is_void_core_candidate(cultures: &[Culture]) -> bool {
        // Void requires all 9 frequencies to be present in lineage
        let unique_cultures: HashSet<_> = cultures.iter().collect();
        unique_cultures.len() == 9 && !unique_cultures.contains(&Culture::Void)
    }
    
    pub fn create_void_core() -> Culture {
        Culture::Void // White Light - fusion of all 9 frequencies
    }
}
```

## Consequences

### Positive
- Enables complex planetary ecosystem scaling
- Creates intuitive color-mixing mechanics
- Provides clear progression path from Primary to Void
- Supports sophisticated "Shepherd's Strategy"
- Aligns with Genetic Tier system

### Negative
- Increases complexity from 6 to 9 cultures
- Requires significant refactoring of existing systems
- Adds learning curve for color mixing mechanics
- Potential for UI clutter with 9-faction display

### Risks
- Color mixing logic may be too complex for casual players
- Trinity bonus may over-power balanced squads
- Void Core requirements may be too difficult to achieve
- Balance between layers needs careful tuning

## Implementation Tasks

1. **Update `src/genetics.rs`**: Expand Culture enum to support 9 types plus Void
2. **Refactor Color Logic**: Update `draw_slime` function for 9-point spectrum rendering
3. **Update Dispatch Loop**: Modify `world_map.rs` for 9-culture planetary node distribution
4. **UI Enhancement**: Add Color Wheel reference to Command Deck
5. **Implement Trinity Bonus**: Add squad composition checking
6. **Update Void Mechanics**: Evolve Void from "all 6" to "all 9" fusion

## Validation

- [ ] All 9 cultures properly implemented with correct layer assignment
- [ ] Color mixing produces expected Secondary/Tertiary results
- [ ] Trinity bonus applies correctly to balanced squads
- [ ] Void Core creation requires all 9 frequencies
- [ ] UI clearly displays 9-point color wheel
- [ ] Planetary map distributes cultures strategically

## Migration Strategy

### Phase 1: Core System
- Update Culture enum and basic relationships
- Implement color mixing logic
- Update rendering system

### Phase 2: Mechanics Integration
- Add Trinity bonus system
- Update Void Core mechanics
- Integrate with breeding system

### Phase 3: UI/UX Updates
- Update Command Deck interface
- Add Color Wheel reference
- Update planetary map visualization

### Phase 4: Balance and Testing
- Tune layer relationships
- Test color mixing success rates
- Validate Void Core achievement difficulty

## Notes

The 9-Point Chromatic Matrix transforms the Slime Planet from a simple garden into a complex planetary ecosystem where players discover "Missing Frequencies" through strategic breeding. The system creates natural progression paths while maintaining the intuitive appeal of color mixing mechanics.

The Void evolves from a simple "all cultures" concept to the "White Light" achieved by fusing all 9 chromatic frequencies, making it the ultimate endgame goal that represents true mastery of the chromatic system.
