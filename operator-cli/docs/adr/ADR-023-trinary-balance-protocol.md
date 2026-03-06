# ADR-023: The Trinary Balance Protocol

**Status:** ACCEPTED | **Date:** 2026-03-04 | **Author:** PyPro SDD-Edition

## Context

The existing 6-point hex system (Ember, Gale, Marsh, Crystal, Tundra, Tide) provided linear adjacency relationships but lacked strategic depth for the Command Deck's tactical decision-making. The system needed to evolve from simple color coding to a sophisticated Counter-Strategy Matrix that creates meaningful trade-offs in both dispatch and breeding choices.

## Decision

Replace the linear hex-beats-adjacent logic with a **Nested 9-Point Trinary System** organized into three distinct RPS loops, each with inward/outward pressure mechanics.

## Architecture

### 1. The Trinary RPS Architecture

| Layer | Type | The Loop (A → B → C) | Stat Focus |
|-------|------|----------------------|------------|
| Inner | Primary | RED (Ember) → YELLOW (Gale) → BLUE (Tide) → RED | ATK → SPD → CHM |
| Middle | Secondary | ORANGE (New) → GREEN (Marsh) → PURPLE (Crystal) → ORANGE | MND → RES → HP |
| Outer | Tertiary | AMBER (New) → TEAL (New) → FROST (Tundra) → AMBER | DUR → STB → END |

### 2. Inward/Outward Pressure Mechanics

- **Outward Advantage:** Primaries deal +25% "Pressure" to Secondaries
- **Inward Resistance:** Tertiaries take -25% "Damage" from Primaries (Ancients resisting Primal forces)

### 3. The Void Exception

The Void Slime (Tier 9) sits outside all loops:
- No RPS advantages
- Ignores all RPS penalties
- Functions as the "Universal Constant"

## Implementation

### Core Function Signature
```rust
fn get_rps_modifier(attacker: Culture, defender: Culture) -> f32
```

### Lookup Table Structure
```rust
enum TrinaryLayer {
    Primary,    // Inner loop
    Secondary,  // Middle loop  
    Tertiary,   // Outer loop
    Void,       // Exception case
}
```

### Pressure Modifiers
- **Advantage:** +1.25 (25% bonus)
- **Neutral:** 1.0 (no modifier)
- **Disadvantage:** 0.75 (25% penalty)
- **Void:** 1.0 (ignores all modifiers)

## Consequences

### Positive
- Creates meaningful tactical depth in Command Deck decisions
- Establishes "Shepherd's Dilemma" in breeding choices
- Provides natural balance through nested RPS relationships
- Enables strategic map control based on culture distribution

### Negative  
- Increases complexity from 6 to 9 cultures
- Requires refactoring existing combat logic
- Adds learning curve for players understanding trinary relationships

### Risks
- Void exception may create overpowered strategies
- Layer pressure mechanics need careful balancing
- UI complexity increases with trinary visualization

## Implementation Tasks

1. **Refactor `src/combat.rs`**: Implement TrinaryRPS lookup table
2. **Update `src/world_map.rs`**: Distribute 9 cultures across 15 nodes using trinary balance
3. **Enhance UI**: Add Trinary Icon (Triangle) to Slime Profile Cards
4. **Logic Updates**: Apply layer-based bonuses (Tertiary: Survival, Primary: Combat)

## Validation

- [ ] All 9 cultures properly assigned to trinary layers
- [ ] RPS modifiers return correct values in combat scenarios
- [ ] Void exception properly implemented and tested
- [ ] UI clearly indicates trinary relationships
- [ ] Map distribution creates strategic conflict zones

## Notes

This transformation creates a **Self-Balancing Ecosystem** where players manage a Chromatic Arms Race rather than simply leveling numbers. The trinary system ensures that no single culture dominates indefinitely, while the Void provides a strategic wildcard for advanced play.

The "Generational Ratchet" mechanic means offspring can eventually surpass their parents in specific tactical niches, creating long-term strategic depth.
