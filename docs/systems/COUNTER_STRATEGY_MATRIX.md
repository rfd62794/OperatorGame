# Counter-Strategy Matrix for Command Deck

> **Status:** TACTICAL SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-023, TRINARY_SYSTEM_ARCHITECTURE.md

## Overview

The Counter-Strategy Matrix transforms the Command Deck from simple stat-based decisions into a sophisticated tactical system where players must consider culture relationships, layer pressure, and strategic positioning. This matrix serves as the foundation for the "Shepherd's Dilemma" - the core strategic tension of dispatch and breeding choices.

## Matrix Structure

### Primary RPS Relationships

| Attacker → | Defender ↓ | Ember (RED) | Gale (YELLOW) | Tide (BLUE) | Result |
|------------|------------|-------------|---------------|-------------|--------|
| **Ember** | ATK | - | **+25%** | -25% | Advantage vs Gale |
| **Gale** | SPD | -25% | - | **+25%** | Advantage vs Tide |
| **Tide** | CHM | **+25%** | -25% | - | Advantage vs Ember |

### Secondary RPS Relationships

| Attacker → | Defender ↓ | Orange (MND) | Marsh (RES) | Crystal (HP) | Result |
|------------|------------|--------------|-------------|---------------|--------|
| **Orange** | MND | - | **+25%** | -25% | Advantage vs Marsh |
| **Marsh** | RES | -25% | - | **+25%** | Advantage vs Crystal |
| **Crystal** | HP | **+25%** | -25% | - | Advantage vs Orange |

### Tertiary RPS Relationships

| Attacker → | Defender ↓ | Amber (DUR) | Teal (STB) | Frost (END) | Result |
|------------|------------|-------------|------------|-------------|--------|
| **Amber** | DUR | - | **+25%** | -25% | Advantage vs Teal |
| **Teal** | STB | -25% | - | **+25%** | Advantage vs Frost |
| **Frost** | END | **+25%** | -25% | - | Advantage vs Amber |

## Cross-Layer Pressure System

### Pressure Mechanics

| Interaction | Modifier | Direction | Tactical Meaning |
|-------------|----------|-----------|------------------|
| Primary → Secondary | +25% | Outward Advantage | Primal forces overwhelm Logic/Defense |
| Primary ← Tertiary | -25% | Inward Resistance | Ancients resist Primal forces |
| Secondary ↔ Tertiary | 0% | Neutral | Balanced relationship |

### Combat Resolution Formula

```rust
fn calculate_combat_modifier(attacker: Culture, defender: Culture) -> f32 {
    let base_rps = get_rps_modifier(attacker, defender); // 1.25, 1.0, or 0.75
    let layer_pressure = get_layer_pressure_modifier(attacker.layer(), defender.layer());
    
    base_rps * layer_pressure // Final modifier applied to damage/success
}
```

## Strategic Decision Framework

### The Shepherd's Dilemma

#### 1. Dispatch Choice Matrix

| Node Culture | Optimal Response | Alternative | Risk Level |
|--------------|------------------|-------------|------------|
| Ember (RED) | Gale (YELLOW) | Tide (BLUE) | Medium |
| Gale (YELLOW) | Tide (BLUE) | Ember (RED) | Medium |
| Tide (BLUE) | Ember (RED) | Gale (YELLOW) | Medium |
| Orange (MND) | Marsh (RES) | Crystal (HP) | Medium |
| Marsh (RES) | Crystal (HP) | Orange (MND) | Medium |
| Crystal (HP) | Orange (MND) | Marsh (RES) | Medium |
| Amber (DUR) | Teal (STB) | Frost (END) | Medium |
| Teal (STB) | Frost (END) | Amber (DUR) | Medium |
| Frost (END) | Amber (DUR) | Teal (STB) | Medium |

#### 2. Breeding Strategy Matrix

| Parent A | Parent B | Offspring Culture | Tactical Value |
|----------|----------|-------------------|----------------|
| Primary + Primary | Same Loop | Secondary | Counters parents |
| Secondary + Secondary | Same Loop | Primary | Vulnerable to parents |
| Tertiary + Tertiary | Same Loop | Primary | High risk, high reward |
| Cross-Layer | Any | Random | Unpredictable outcome |
| Void + Any | Any | Void | Universal constant |

### Tactical Scenarios

#### Scenario 1: Territory Control
- **Situation:** Enemy controls "Blue (Tide)" node
- **Optimal:** Deploy "Yellow (Gale)" slime for +25% advantage
- **Risk:** Enemy may have "Red (Ember)" reinforcements
- **Counter:** Keep "Blue (Tide)" reserves in reserve

#### Scenario 2: Breeding for Defense
- **Situation:** Enemy favors "Primary" layer attacks
- **Optimal:** Breed "Secondary" slimes for resistance
- **Trade-off:** Lose offensive pressure against "Tertiary"
- **Strategy:** Maintain balanced stable across all layers

#### Scenario 3: Void Deployment
- **Situation:** Uncertain enemy composition
- **Optimal:** Deploy Void slime for consistent performance
- **Limitation:** No advantage potential
- **Best Use:** Key defensive positions where reliability matters

## Implementation Details

### Decision Tree Logic

```rust
pub struct TacticalDecision {
    pub recommended_slime: Option<Uuid>,
    pub success_rate: f64,
    pub risk_assessment: RiskLevel,
    pub alternative_options: Vec<Alternative>,
}

impl CommandDeck {
    pub fn analyze_node(&self, node: &MapNode) -> TacticalDecision {
        // 1. Identify node culture
        // 2. Check available slimes with advantage
        // 3. Calculate success rates with modifiers
        // 4. Assess risks and alternatives
        // 5. Return recommendation
    }
}
```

### AI Behavior Patterns

```rust
pub enum AITacticalPersonality {
    Aggressive,    // Always seek advantage
    Defensive,     // Prefer resistance bonuses  
    Balanced,      // Mix of strategies
    Unpredictable, // Random choices
    VoidPreferred, // Favor Void slimes
}
```

## Player Interface Requirements

### Tactical Display Elements

1. **Node Culture Indicator**: Clear color coding and symbol
2. **Advantage Preview**: Show potential modifiers before deployment
3. **Risk Assessment**: Visual indicators for success probability
4. **Alternative Options**: Display backup choices with trade-offs
5. **Breeding Forecast**: Show potential offspring tactical value

### Information Hierarchy

```
Primary Information:
- Node culture and current controller
- Recommended slime with advantage
- Success rate with modifiers

Secondary Information:
- Alternative options and trade-offs
- Risk factors and enemy response patterns
- Long-term strategic implications

Tertiary Information:
- Historical performance data
- Breeding recommendations
- Resource cost analysis
```

## Balancing Considerations

### Modifier Tuning
- **Current**: ±25% for standard RPS
- **Void**: 0% modifiers (universal constant)
- **Layer Pressure**: Additional ±25% for cross-layer
- **Maximum Potential**: +56.25% (advantage + outward pressure)
- **Minimum Potential**: -43.75% (disadvantage + inward resistance)

### Economic Factors
- **Deployment Cost**: Varies by slime tier and culture
- **Repair Costs**: Higher for disadvantaged engagements
- **Experience Gain**: Bonus for successful advantage plays
- **Breeding Investment**: Higher cost for strategic cross-breeding

## Validation Metrics

- [ ] All RPS relationships produce correct modifiers
- [ ] Cross-layer pressure applies appropriately
- [ ] AI personalities demonstrate distinct tactical patterns
- [ ] Player success rates correlate with strategic choices
- [ ] Breeding outcomes follow predictable patterns
- [ ] Void mechanics provide balanced alternative

## Future Expansions

1. **Environmental Modifiers**: Map-specific bonuses/penalties
2. **Combo System**: Multi-slime squad synergies
3. **Temporal Mechanics**: Time-based culture evolution
4. **Faction System**: Cultural allegiance bonuses
5. **Tournament Mode**: Structured competitive play
