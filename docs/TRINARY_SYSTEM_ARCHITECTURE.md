# Trinary System Architecture

> **Status:** SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-023, SPEC.md §3 (Genetic Tier Resolution)

## Overview

The Trinary System replaces the linear 6-point hex with a **Nested 9-Point Trinary System** that creates Rock-Paper-Scissors balance across three distinct layers. This architecture serves as the Counter-Strategy Matrix for the Command Deck's tactical decision-making.

## System Structure

### Layer Organization

```
INNER LOOP (Primary)    MIDDLE LOOP (Secondary)    OUTER LOOP (Tertiary)
┌─────────────────┐     ┌─────────────────┐        ┌─────────────────┐
│  RED (Ember)    │     │ ORANGE (New)    │        │ AMBER (New)     │
│       ↓         │     │       ↓         │        │       ↓         │
│ YELLOW (Gale)   │     │ GREEN (Marsh)   │        │  TEAL (New)     │
│       ↓         │     │       ↓         │        │       ↓         │
│  BLUE (Tide)    │     │ PURPLE (Crystal)│        │ FROST (Tundra) │
│       ↓         │     │       ↓         │        │       ↓         │
│  RED (Ember)    │     │ ORANGE (New)    │        │ AMBER (New)     │
└─────────────────┘     └─────────────────┘        └─────────────────┘
   ATK → SPD → CHM         MND → RES → HP            DUR → STB → END
```

### Culture Classification

| Culture | Layer | Type | Primary Stat | Position in Loop |
|---------|-------|------|--------------|------------------|
| Ember | Inner | Primary | ATK | Start |
| Gale | Inner | Primary | SPD | Middle |
| Tide | Inner | Primary | CHM | End |
| Orange | Middle | Secondary | MND | Start |
| Marsh | Middle | Secondary | RES | Middle |
| Crystal | Middle | Secondary | HP | End |
| Amber | Outer | Tertiary | DUR | Start |
| Teal | Outer | Tertiary | STB | Middle |
| Frost | Outer | Tertiary | END | End |
| Void | Exception | Universal | -- | Outside System |

## RPS Mechanics

### Within-Loop Relationships
Each loop follows standard RPS: A → B → C → A

**Example (Inner Loop):**
- Ember (RED) beats Gale (YELLOW)
- Gale (YELLOW) beats Tide (BLUE)  
- Tide (BLUE) beats Ember (RED)

### Cross-Layer Pressure

| Relationship | Effect | Rationale |
|--------------|--------|-----------|
| Primary → Secondary | +25% Pressure | Primal forces overwhelm Logic/Defense |
| Tertiary ← Primary | -25% Damage | Ancients resist Primal forces |
| Secondary ↔ Tertiary | Neutral | Balanced relationship |

### Void Exception
- **No Advantages:** Void cannot gain RPS bonuses
- **No Penalties:** Void ignores all RPS disadvantages  
- **Universal Constant:** Functions as strategic wildcard

## Strategic Integration

### "Shepherd's Dilemma" Mechanics

1. **Dispatch Choice**: When node is "Blue (Tide)," send "Yellow (Gale)" for +2 D20 advantage
2. **Breeding Choice**: Primary + Primary creates Secondary that counters parents
3. **Generational Ratchet**: Offspring surpass parents in specific tactical niches

### Systemic Balance

| Layer | Tactical Role | Map Function | Player Strategy |
|-------|---------------|--------------|-----------------|
| Primary | Combat/Speed | High-velocity scavenging | Territory takeover |
| Secondary | Logic/Defense | Fortified nodes | Ship repairs, holding |
| Tertiary | Survival/Endurance | Deep-planet expeditions | Atmospheric stress zones |

## Implementation Specification

### Data Structures

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Culture {
    // Inner Loop (Primary)
    Ember,    // RED - ATK
    Gale,     // YELLOW - SPD  
    Tide,     // BLUE - CHM
    
    // Middle Loop (Secondary)
    Orange,   // ORANGE - MND
    Marsh,    // GREEN - RES
    Crystal,  // PURPLE - HP
    
    // Outer Loop (Tertiary)
    Amber,    // AMBER - DUR
    Teal,     // TEAL - STB
    Frost,    // FROST - END
    
    Void,     // Exception
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrinaryLayer {
    Primary,    // Inner
    Secondary,  // Middle
    Tertiary,   // Outer
    Void,       // Exception
}
```

### Core Functions

```rust
impl Culture {
    pub fn layer(self) -> TrinaryLayer { /* ... */ }
    pub fn primary_stat(self) -> Stat { /* ... */ }
    pub fn beats(self) -> Culture { /* ... */ }
    pub fn loses_to(self) -> Culture { /* ... */ }
}

pub fn get_rps_modifier(attacker: Culture, defender: Culture) -> f32 {
    // Returns 1.25, 1.0, or 0.75 based on relationships
}
```

## UI Requirements

### Visual Indicators
- **Trinary Icon**: Triangle symbol on Slime Profile Cards
- **Color Wheel**: Breeding menu helper showing relationships
- **Layer Badges**: Inner/Middle/Outer indicators
- **Pressure Arrows**: Combat preview showing advantages

### Information Architecture
- **Culture Profile**: Show loop position and stat focus
- **Combat Preview**: Display RPS modifier before engagement
- **Breeding Forecast**: Predict offspring culture and layer

## Validation Criteria

- [ ] All 9 cultures properly implemented with correct layer assignment
- [ ] RPS relationships function correctly within each loop
- [ ] Cross-layer pressure modifiers apply correctly
- [ ] Void exception properly bypasses all RPS mechanics
- [ ] UI clearly communicates trinary relationships
- [ ] Map distribution creates strategic conflict zones
- [ ] Breeding follows "Generational Ratchet" principles

## Future Considerations

1. **Dynamic Culture Evolution**: Potential for cultures to shift between layers
2. **Environmental Modifiers**: Map nodes that amplify specific layers
3. **Combo Mechanics**: Multi-slime squads leveraging layer synergy
4. **Tournament Modes**: Structured PvP based on trinary balance
