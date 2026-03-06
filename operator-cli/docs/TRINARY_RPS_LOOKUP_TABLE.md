# Trinary RPS Lookup Table Specification

> **Status:** TECHNICAL IMPLEMENTATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-023, SPEC.md §3, COUNTER_STRATEGY_MATRIX.md

## Overview

This document provides the complete technical specification for the Trinary RPS Lookup Table system. The lookup table replaces the linear hex adjacency system with a sophisticated 9-culture, 3-layer Rock-Paper-Scissors mechanism with cross-layer pressure mechanics.

## Core Data Structures

### Culture Enum Definition

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Culture {
    // Inner Loop (Primary) - ATK/SPD/CHM
    Ember,    // RED
    Gale,     // YELLOW  
    Tide,     // BLUE
    
    // Middle Loop (Secondary) - MND/RES/HP
    Orange,   // ORANGE
    Marsh,    // GREEN
    Crystal,  // PURPLE
    
    // Outer Loop (Tertiary) - DUR/STB/END
    Amber,    // AMBER
    Teal,     // TEAL
    Frost,    // FROST
    
    // Exception
    Void,     // Universal Constant
}
```

### Layer Classification

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrinaryLayer {
    Primary,    // Inner loop - Ember, Gale, Tide
    Secondary,  // Middle loop - Orange, Marsh, Crystal
    Tertiary,   // Outer loop - Amber, Teal, Frost
    Void,       // Exception - Void only
}
```

## RPS Relationship Mapping

### Within-Loop Relationships

```rust
impl Culture {
    /// Returns the culture that this one beats
    pub fn beats(self) -> Culture {
        match self {
            // Inner Loop: Ember → Gale → Tide → Ember
            Culture::Ember => Culture::Gale,
            Culture::Gale => Culture::Tide,
            Culture::Tide => Culture::Ember,
            
            // Middle Loop: Orange → Marsh → Crystal → Orange
            Culture::Orange => Culture::Marsh,
            Culture::Marsh => Culture::Crystal,
            Culture::Crystal => Culture::Orange,
            
            // Outer Loop: Amber → Teal → Frost → Amber
            Culture::Amber => Culture::Teal,
            Culture::Teal => Culture::Frost,
            Culture::Frost => Culture::Amber,
            
            // Void has no relationships
            Culture::Void => Culture::Void,
        }
    }
    
    /// Returns the culture that beats this one
    pub fn loses_to(self) -> Culture {
        match self {
            Culture::Ember => Culture::Tide,
            Culture::Gale => Culture::Ember,
            Culture::Tide => Culture::Gale,
            
            Culture::Orange => Culture::Crystal,
            Culture::Marsh => Culture::Orange,
            Culture::Crystal => Culture::Marsh,
            
            Culture::Amber => Culture::Frost,
            Culture::Teal => Culture::Amber,
            Culture::Frost => Culture::Teal,
            
            Culture::Void => Culture::Void,
        }
    }
    
    /// Returns the layer this culture belongs to
    pub fn layer(self) -> TrinaryLayer {
        match self {
            Culture::Ember | Culture::Gale | Culture::Tide => TrinaryLayer::Primary,
            Culture::Orange | Culture::Marsh | Culture::Crystal => TrinaryLayer::Secondary,
            Culture::Amber | Culture::Teal | Culture::Frost => TrinaryLayer::Tertiary,
            Culture::Void => TrinaryLayer::Void,
        }
    }
}
```

## Modifier Calculation System

### Core RPS Modifier Function

```rust
/// Calculates the base RPS modifier (ignoring cross-layer pressure)
pub fn get_base_rps_modifier(attacker: Culture, defender: Culture) -> f32 {
    if attacker == defender || attacker == Culture::Void || defender == Culture::Void {
        1.0 // Neutral or Void exception
    } else if attacker.beats() == defender {
        1.25 // +25% advantage
    } else {
        0.75 // -25% disadvantage
    }
}
```

### Cross-Layer Pressure System

```rust
/// Calculates cross-layer pressure modifiers
pub fn get_layer_pressure_modifier(attacker_layer: TrinaryLayer, defender_layer: TrinaryLayer) -> f32 {
    match (attacker_layer, defender_layer) {
        // Primary → Secondary: Outward Advantage (+25%)
        (TrinaryLayer::Primary, TrinaryLayer::Secondary) => 1.25,
        
        // Primary ← Tertiary: Inward Resistance (-25% damage to Tertiary)
        // This means Tertiary takes 25% less damage from Primary
        (TrinaryLayer::Primary, TrinaryLayer::Tertiary) => 0.75,
        
        // All other cross-layer interactions: Neutral
        _ => 1.0,
    }
}
```

### Complete Combat Modifier Function

```rust
/// Main function to calculate complete combat modifier
pub fn get_rps_modifier(attacker: Culture, defender: Culture) -> f32 {
    // Void exception: ignores all RPS mechanics
    if attacker == Culture::Void || defender == Culture::Void {
        return 1.0;
    }
    
    let base_modifier = get_base_rps_modifier(attacker, defender);
    let layer_modifier = get_layer_pressure_modifier(attacker.layer(), defender.layer());
    
    base_modifier * layer_modifier
}
```

## Modifier Result Matrix

### Complete 9x9 Modifier Table

| Attacker → | Ember | Gale | Tide | Orange | Marsh | Crystal | Amber | Teal | Frost | Void |
|------------|-------|------|------|--------|-------|---------|-------|------|-------|------|
| **Ember** | 1.0 | **1.56** | 0.56 | **1.25** | **1.25** | **1.25** | 0.56 | 0.56 | 0.56 | 1.0 |
| **Gale** | 0.56 | 1.0 | **1.56** | **1.25** | **1.25** | **1.25** | 0.56 | 0.56 | 0.56 | 1.0 |
| **Tide** | **1.56** | 0.56 | 1.0 | **1.25** | **1.25** | **1.25** | 0.56 | 0.56 | 0.56 | 1.0 |
| **Orange** | 0.94 | 0.94 | 0.94 | 1.0 | **1.56** | 0.56 | **1.25** | **1.25** | **1.25** | 1.0 |
| **Marsh** | 0.94 | 0.94 | 0.94 | 0.56 | 1.0 | **1.56** | **1.25** | **1.25** | **1.25** | 1.0 |
| **Crystal** | 0.94 | 0.94 | 0.94 | **1.56** | 0.56 | 1.0 | **1.25** | **1.25** | **1.25** | 1.0 |
| **Amber** | **1.25** | **1.25** | **1.25** | 0.94 | 0.94 | 0.94 | 1.0 | **1.56** | 0.56 | 1.0 |
| **Teal** | **1.25** | **1.25** | **1.25** | 0.94 | 0.94 | 0.94 | 0.56 | 1.0 | **1.56** | 1.0 |
| **Frost** | **1.25** | **1.25** | **1.25** | 0.94 | 0.94 | 0.94 | **1.56** | 0.56 | 1.0 | 1.0 |
| **Void** | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |

**Legend:**
- **Bold**: Advantage scenarios (1.56 = max advantage)
- **0.56**: Maximum disadvantage (0.75 × 0.75)
- **0.94**: Cross-layer penalty (0.75 × 1.25)
- **1.25**: Cross-layer bonus (1.0 × 1.25)
- **1.0**: Neutral interactions

## Implementation Details

### Performance Optimization

```rust
// Pre-computed lookup table for maximum performance
static RPS_MODIFIER_TABLE: [[f32; 10]; 10] = [
    // [Attacker][Defender] - populated with values from matrix above
    [1.0, 1.56, 0.56, 1.25, 1.25, 1.25, 0.56, 0.56, 0.56, 1.0], // Ember
    [0.56, 1.0, 1.56, 1.25, 1.25, 1.25, 0.56, 0.56, 0.56, 1.0], // Gale
    // ... complete table
];

/// Fast lookup version using pre-computed table
pub fn get_rps_modifier_fast(attacker: Culture, defender: Culture) -> f32 {
    RPS_MODIFIER_TABLE[attacker as usize][defender as usize]
}
```

### Validation and Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rps_relationships() {
        assert_eq!(Culture::Ember.beats(), Culture::Gale);
        assert_eq!(Culture::Gale.loses_to(), Culture::Ember);
        assert_eq!(Culture::Void.beats(), Culture::Void);
    }
    
    #[test]
    fn test_layer_classification() {
        assert_eq!(Culture::Ember.layer(), TrinaryLayer::Primary);
        assert_eq!(Culture::Orange.layer(), TrinaryLayer::Secondary);
        assert_eq!(Culture::Amber.layer(), TrinaryLayer::Tertiary);
        assert_eq!(Culture::Void.layer(), TrinaryLayer::Void);
    }
    
    #[test]
    fn test_modifier_calculations() {
        // Primary advantage
        assert_eq!(get_rps_modifier(Culture::Ember, Culture::Gale), 1.56);
        
        // Cross-layer pressure
        assert_eq!(get_rps_modifier(Culture::Ember, Culture::Orange), 1.25);
        
        // Void exception
        assert_eq!(get_rps_modifier(Culture::Void, Culture::Ember), 1.0);
        assert_eq!(get_rps_modifier(Culture::Ember, Culture::Void), 1.0);
    }
    
    #[test]
    fn test_symmetry() {
        // Test that advantage/disadvantage is symmetric
        for attacker in ALL_CULTURES {
            for defender in ALL_CULTURES {
                let forward = get_rps_modifier(attacker, defender);
                let backward = get_rps_modifier(defender, attacker);
                
                if attacker != defender && attacker != Culture::Void && defender != Culture::Void {
                    assert!((forward * backward - 1.0).abs() < 0.01);
                }
            }
        }
    }
}
```

## Integration Points

### Combat System Integration

```rust
impl CombatResolver {
    pub fn calculate_damage(base_damage: f32, attacker: Culture, defender: Culture) -> f32 {
        let modifier = get_rps_modifier(attacker, defender);
        base_damage * modifier
    }
    
    pub fn calculate_success_chance(base_chance: f32, attacker: Culture, defender: Culture) -> f32 {
        let modifier = get_rps_modifier(attacker, defender);
        (base_chance * modifier).min(0.95).max(0.05) // Apply bounds
    }
}
```

### UI Integration

```rust
impl Culture {
    pub fn get_advantage_indicator(self) -> &'static str {
        match self.layer() {
            TrinaryLayer::Primary => "⚔️",  // Sword for combat
            TrinaryLayer::Secondary => "🛡️", // Shield for defense
            TrinaryLayer::Tertiary => "🏔️",  // Mountain for endurance
            TrinaryLayer::Void => "⚫",       // Black circle for void
        }
    }
    
    pub fn get_color_hex(self) -> &'static str {
        match self {
            Culture::Ember => "#FF4444",
            Culture::Gale => "#FFFF44",
            Culture::Tide => "#4444FF",
            Culture::Orange => "#FF8844",
            Culture::Marsh => "#44FF44",
            Culture::Crystal => "#FF44FF",
            Culture::Amber => "#FFAA44",
            Culture::Teal => "#44FFAA",
            Culture::Frost => "#AAFFFF",
            Culture::Void => "#333333",
        }
    }
}
```

## Migration Strategy

### From Hex System

1. **Data Migration**: Convert existing 6-culture saves to 9-culture system
2. **Backward Compatibility**: Handle missing cultures gracefully
3. **UI Updates**: Replace hex visualization with trinary triangles
4. **Tutorial Updates**: Teach new RPS relationships

### Validation Checklist

- [ ] All 9 cultures implemented with correct relationships
- [ ] Cross-layer pressure applies correctly
- [ ] Void exception bypasses all modifiers
- [ ] Performance optimized with lookup table
- [ ] Comprehensive test coverage
- [ ] UI clearly communicates relationships
- [ ] Migration from old system handles edge cases

This specification provides the complete technical foundation for implementing the Trinary RPS system that will drive the strategic depth of the Command Deck and create the "Shepherd's Dilemma" gameplay experience.
