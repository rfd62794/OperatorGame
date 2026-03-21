# Color Mixing Logic for Incubator

> **Status:** BREEDING SYSTEM SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-022, CHROMATIC_FRAMEWORK.md, SPEC.md §5

## Overview

The Color Mixing Logic transforms the Bio-Incubator from a simple breeding system into an intuitive "color-mixing lab" where players discover new cultures through strategic combinations. This system creates the "Generational Ratchet" where offspring can surpass their parents in specific tactical niches.

## Mixing Mathematics

### Core Mixing Rules

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixingResult {
    Success(Culture),
    Failure(MixingFailureReason),
    Void(Culture), // Void dominance case
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixingFailureReason {
    IncompatibleColors,
    SameColorNoEffect,
    InsufficientGeneration,
    EnvironmentalMismatch,
    RandomChance,
}

impl Culture {
    /// Primary mixing function - determines offspring culture
    pub fn mix_colors(parent_a: Culture, parent_b: Culture, context: &MixingContext) -> MixingResult {
        // Void dominance rule
        if parent_a == Culture::Void || parent_b == Culture::Void {
            return MixingResult::Void(Culture::Void);
        }
        
        // Same color mixing
        if parent_a == parent_b {
            return MixingResult::Success(parent_a);
        }
        
        // Check generation requirements
        if context.min_generation > 0 {
            let avg_generation = (context.parent_a_generation + context.parent_b_generation) / 2;
            if avg_generation < context.min_generation {
                return MixingResult::Failure(MixingFailureReason::InsufficientGeneration);
            }
        }
        
        // Calculate mixing probability
        let base_probability = self.calculate_mixing_probability(parent_a, parent_b);
        let environmental_modifier = context.get_environmental_modifier();
        let final_probability = base_probability * environmental_modifier;
        
        // Apply random chance
        if context.rng.gen::<f32>() > final_probability {
            return MixingResult::Failure(MixingFailureReason::RandomChance);
        }
        
        // Determine offspring based on color combination
        let offspring = self.determine_offspring(parent_a, parent_b);
        MixingResult::Success(offspring)
    }
    
    /// Calculate angular distance and base probability
    fn calculate_mixing_probability(self, other: Culture) -> f32 {
        let wheel = CHROMATIC_WHEEL.get();
        let self_pos = wheel.get_position(self);
        let other_pos = wheel.get_position(other);
        
        let angular_distance = (self_pos.angle - other_pos.angle).abs().min(360.0 - (self_pos.angle - other_pos.angle).abs());
        
        match angular_distance {
            0.0 => 0.5,    // Same color
            60.0 => 0.8,   // Adjacent (optimal)
            120.0 => 0.4,  // Skip-one
            180.0 => 0.1,  // Opposite (rare)
            _ => 0.0,      // Invalid
        }
    }
}
```

### Mixing Matrix Definition

```rust
pub struct MixingMatrix {
    pub primary_to_secondary: HashMap<(Culture, Culture), Culture>,
    pub secondary_to_tertiary: HashMap<(Culture, Culture), Culture>,
    pub tertiary_to_primary: HashMap<(Culture, Culture), Culture>,
    pub special_combinations: HashMap<(Culture, Culture), Culture>,
}

impl MixingMatrix {
    pub fn new() -> Self {
        let mut primary_to_secondary = HashMap::new();
        let mut secondary_to_tertiary = HashMap::new();
        let mut tertiary_to_primary = HashMap::new();
        let mut special_combinations = HashMap::new();
        
        // Primary + Primary = Secondary
        primary_to_secondary.insert((Culture::Ember, Culture::Gale), Culture::Orange);
        primary_to_secondary.insert((Culture::Gale, Culture::Ember), Culture::Orange);
        primary_to_secondary.insert((Culture::Gale, Culture::Tide), Culture::Teal);
        primary_to_secondary.insert((Culture::Tide, Culture::Gale), Culture::Teal);
        primary_to_secondary.insert((Culture::Tide, Culture::Ember), Culture::Purple);
        primary_to_secondary.insert((Culture::Ember, Culture::Tide), Culture::Purple);
        
        // Secondary + Secondary = Tertiary
        secondary_to_tertiary.insert((Culture::Orange, Culture::Marsh), Culture::Amber);
        secondary_to_tertiary.insert((Culture::Marsh, Culture::Orange), Culture::Amber);
        secondary_to_tertiary.insert((Culture::Marsh, Culture::Crystal), Culture::Teal);
        secondary_to_tertiary.insert((Culture::Crystal, Culture::Marsh), Culture::Teal);
        secondary_to_tertiary.insert((Culture::Crystal, Culture::Orange), Culture::Amber);
        secondary_to_tertiary.insert((Culture::Orange, Culture::Crystal), Culture::Amber);
        
        // Tertiary + Tertiary = Primary (rare, high generation requirement)
        tertiary_to_primary.insert((Culture::Teal, Culture::Amber), Culture::Gale);
        tertiary_to_primary.insert((Culture::Amber, Culture::Teal), Culture::Gale);
        tertiary_to_primary.insert((Culture::Amber, Culture::Frost), Culture::Ember);
        tertiary_to_primary.insert((Culture::Frost, Culture::Amber), Culture::Ember);
        tertiary_to_primary.insert((Culture::Frost, Culture::Teal), Culture::Tide);
        tertiary_to_primary.insert((Culture::Teal, Culture::Frost), Culture::Tide);
        
        // Special cross-layer combinations
        special_combinations.insert((Culture::Ember, Culture::Marsh), Culture::Orange);
        special_combinations.insert((Culture::Gale, Culture::Crystal), Culture::Teal);
        special_combinations.insert((Culture::Tide, Culture::Orange), Culture::Purple);
        
        Self {
            primary_to_secondary,
            secondary_to_tertiary,
            tertiary_to_primary,
            special_combinations,
        }
    }
    
    pub fn get_offspring(&self, parent_a: Culture, parent_b: Culture) -> Option<Culture> {
        // Check ordered combinations
        if let Some(&offspring) = self.primary_to_secondary.get(&(parent_a, parent_b)) {
            return Some(offspring);
        }
        if let Some(&offspring) = self.secondary_to_tertiary.get(&(parent_a, parent_b)) {
            return Some(offspring);
        }
        if let Some(&offspring) = self.tertiary_to_primary.get(&(parent_a, parent_b)) {
            return Some(offspring);
        }
        if let Some(&offspring) = self.special_combinations.get(&(parent_a, parent_b)) {
            return Some(offspring);
        }
        
        None
    }
}
```

## Contextual Factors

### Mixing Context Structure

```rust
pub struct MixingContext {
    pub parent_a_generation: u32,
    pub parent_b_generation: u32,
    pub min_generation: u32,
    pub environmental_modifier: f32,
    pub facility_level: u8,
    pub rng: ThreadRng,
    pub special_conditions: Vec<SpecialCondition>,
}

#[derive(Debug, Clone)]
pub enum SpecialCondition {
    FullMoon,           // +20% success rate
    SolarFlare,         // +15% mutation chance
    ChromaticStorm,     // Enables rare combinations
    VoidResonance,      // Higher Void chance
    SeasonalBloom,      // Layer-specific bonuses
}

impl MixingContext {
    pub fn get_environmental_modifier(&self) -> f32 {
        let mut modifier = 1.0;
        
        // Facility level bonus
        modifier += self.facility_level as f32 * 0.05;
        
        // Special conditions
        for condition in &self.special_conditions {
            modifier += match condition {
                SpecialCondition::FullMoon => 0.2,
                SpecialCondition::SolarFlare => 0.0, // Affects mutation, not success
                SpecialCondition::ChromaticStorm => 0.3,
                SpecialCondition::VoidResonance => 0.1,
                SpecialCondition::SeasonalBloom => 0.15,
            };
        }
        
        modifier.min(2.0) // Cap at 100% bonus
    }
}
```

### Environmental Alignment

```rust
pub struct BiomeMixingBonus {
    pub biome: BiomeType,
    pub favored_combinations: Vec<(Culture, Culture)>,
    pub bonus_multiplier: f32,
}

impl BiomeMixingBonus {
    pub fn new() -> HashMap<BiomeType, BiomeMixingBonus> {
        let mut bonuses = HashMap::new();
        
        bonuses.insert(BiomeType::Volcanic, BiomeMixingBonus {
            biome: BiomeType::Volcanic,
            favored_combinations: vec![
                (Culture::Ember, Culture::Orange),
                (Culture::Ember, Culture::Amber),
            ],
            bonus_multiplier: 1.5,
        });
        
        bonuses.insert(BiomeType::Aquatic, BiomeMixingBonus {
            biome: BiomeType::Aquatic,
            favored_combinations: vec![
                (Culture::Tide, Culture::Teal),
                (Culture::Tide, Culture::Frost),
            ],
            bonus_multiplier: 1.5,
        });
        
        bonuses.insert(BiomeType::Arid, BiomeMixingBonus {
            biome: BiomeType::Arid,
            favored_combinations: vec![
                (Culture::Gale, Culture::Amber),
                (Culture::Gale, Culture::Orange),
            ],
            bonus_multiplier: 1.5,
        });
        
        bonuses.insert(BiomeType::Toxic, BiomeMixingBonus {
            biome: BiomeType::Toxic,
            favored_combinations: vec![
                (Culture::Marsh, Culture::Crystal),
                (Culture::Marsh, Culture::Teal),
            ],
            bonus_multiplier: 1.5,
        });
        
        bonuses.insert(BiomeType::Frozen, BiomeMixingBonus {
            biome: BiomeType::Frozen,
            favored_combinations: vec![
                (Culture::Tundra, Culture::Frost),
                (Culture::Tundra, Culture::Amber),
            ],
            bonus_multiplier: 1.5,
        });
        
        bonuses
    }
}
```

## Advanced Mechanics

### Mutation System

```rust
pub struct MutationSystem {
    pub base_mutation_chance: f32,
    pub void_mutation_chance: f32,
    pub special_mutation_conditions: HashMap<SpecialCondition, f32>,
}

impl MutationSystem {
    pub fn calculate_mutation_chance(
        &self,
        parent_a: &SlimeGenome,
        parent_b: &SlimeGenome,
        context: &MixingContext,
    ) -> f32 {
        let mut chance = self.base_mutation_chance;
        
        // Void parent increases mutation chance
        if parent_a.culture == Culture::Void || parent_b.culture == Culture::Void {
            chance = chance.max(self.void_mutation_chance);
        }
        
        // High generation parents increase mutation chance
        let avg_generation = (parent_a.generation + parent_b.generation) as f32 / 2.0;
        chance += avg_generation * 0.01;
        
        // Special conditions
        for condition in &context.special_conditions {
            if let Some(&bonus) = self.special_mutation_conditions.get(condition) {
                chance += bonus;
            }
        }
        
        chance.min(0.5) // Cap at 50%
    }
    
    pub fn apply_mutation(&self, offspring: &mut SlimeGenome, context: &MixingContext) {
        let mutation_roll = context.rng.gen::<f32>();
        
        if mutation_roll < self.calculate_mutation_chance(&offspring, &offspring, context) {
            // Apply random mutation
            match context.rng.gen_range(0..4) {
                0 => self.mutate_stats(offspring),
                1 => self.mutate_visuals(offspring),
                2 => self.mutate_personality(offspring),
                3 => self.mutate_culture_expression(offspring),
                _ => {}
            }
        }
    }
    
    fn mutate_stats(&self, slime: &mut SlimeGenome) {
        let mutation_type = match rand::random::<f32>() {
            x if x < 0.7 => 1.25, // Positive mutation
            _ => 0.85,            // Negative mutation
        };
        
        slime.base_hp *= mutation_type;
        slime.base_atk *= mutation_type;
        slime.base_spd *= mutation_type;
    }
    
    fn mutate_visuals(&self, slime: &mut SlimeGenome) {
        // Random visual mutation
        if rand::random::<f32>() < 0.1 {
            slime.accessory = match rand::random::<u8>() % 6 {
                0 => Accessory::Crown,
                1 => Accessory::Scar,
                2 => Accessory::Glow,
                3 => Accessory::Shell,
                4 => Accessory::Crystals,
                _ => Accessory::None,
            };
        }
    }
}
```

### Generational Ratchet

```rust
pub struct GenerationalRatchet {
    pub ratchet_rate: f32,
    pub cap_multiplier: f32,
    pub decay_resistance: f32,
}

impl GenerationalRatchet {
    pub fn apply_ratchet(&self, current_stat: f32, cap: f32, is_void: bool) -> f32 {
        let ratchet_strength = if is_void { self.ratchet_rate * 1.5 } else { self.ratchet_rate };
        
        // Move 10% toward the cap
        let ratcheted = current_stat + (cap - current_stat) * ratchet_strength;
        
        // Apply decay resistance
        let decay_resisted = ratcheted * (1.0 + self.decay_resistance);
        
        decay_resisted.min(cap)
    }
    
    pub fn calculate_stat_cap(&self, base_stat: f32, culture_modifier: f32, generation: u32) -> f32 {
        let generation_bonus = (generation as f32 - 1.0) * 0.1;
        base_stat * culture_modifier * self.cap_multiplier * (1.0 + generation_bonus)
    }
}
```

## UI Integration

### Incubator Interface

```rust
pub struct IncubatorWidget {
    pub selected_parents: [Option<SlimeGenome>; 2],
    pub mixing_context: MixingContext,
    pub preview_result: Option<MixingResult>,
    pub mixing_history: Vec<MixingRecord>,
}

impl IncubatorWidget {
    pub fn render_mixing_preview(&self, ui: &mut egui::Ui) {
        if let (Some(parent_a), Some(parent_b)) = (&self.selected_parents[0], &self.selected_parents[1]) {
            ui.horizontal(|ui| {
                ui.label("Mixing Preview:");
                
                // Parent colors
                ui.colored_color(
                    egui::Color32::from_rgb(parent_a.culture.get_rgb_color()),
                    format!("{} + ", parent_a.name)
                );
                ui.colored_color(
                    egui::Color32::from_rgb(parent_b.culture.get_rgb_color()),
                    parent_b.name
                );
                
                ui.label(" = ");
                
                // Preview result
                if let Some(result) = &self.preview_result {
                    match result {
                        MixingResult::Success(offspring) => {
                            ui.colored_color(
                                egui::Color32::from_rgb(offspring.get_rgb_color()),
                                format!("{}", offspring)
                            );
                        },
                        MixingResult::Failure(reason) => {
                            ui.colored_color(egui::Color32::RED, format!("Failed: {:?}", reason));
                        },
                        MixingResult::Void(_) => {
                            ui.colored_color(egui::Color32::BLACK, "Void");
                        },
                    }
                }
            });
            
            // Success probability
            if let (Some(parent_a), Some(parent_b)) = (&self.selected_parents[0], &self.selected_parents[1]) {
                let probability = Culture::calculate_mixing_probability(parent_a.culture, parent_b.culture);
                let environmental_modifier = self.mixing_context.get_environmental_modifier();
                let final_probability = probability * environmental_modifier;
                
                ui.horizontal(|ui| {
                    ui.label("Success Chance:");
                    ui.colored_color(
                        egui::Color32::LIGHT_BLUE,
                        format!("{:.1}%", final_probability * 100.0)
                    );
                });
            }
        }
    }
    
    pub fn render_color_wheel_guide(&self, ui: &mut egui::Ui) {
        ui.heading("Color Mixing Guide");
        
        ui.columns(2, |columns| {
            columns[0].label("Primary + Primary:");
            columns[0].label("  Ember + Gale = Orange");
            columns[0].label("  Gale + Tide = Teal");
            columns[0].label("  Tide + Ember = Purple");
            
            columns[1].label("Secondary + Secondary:");
            columns[1].label("  Orange + Marsh = Amber");
            columns[1].label("  Marsh + Crystal = Teal");
            columns[1].label("  Crystal + Orange = Amber");
        });
        
        ui.separator();
        
        ui.label("Tertiary + Tertiary (Rare, High Gen):");
        ui.label("  Teal + Amber = Gale");
        ui.label("  Amber + Frost = Ember");
        ui.label("  Frost + Teal = Tide");
    }
}
```

## Implementation Tasks

### Core System Updates

1. **Update `src/genetics.rs`**: Expand Culture enum and mixing logic
2. **Implement Mixing Matrix**: Create comprehensive combination rules
3. **Add Context System**: Environmental and facility modifiers
4. **Integrate Mutation System**: Stat and visual mutations
5. **Update UI Components**: Incubator interface with color wheel

### Testing Requirements

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_primary_mixing() {
        let result = Culture::mix_colors(
            Culture::Ember, 
            Culture::Gale, 
            &MixingContext::default()
        );
        assert!(matches!(result, MixingResult::Success(Culture::Orange)));
    }
    
    #[test]
    fn test_void_dominance() {
        let result = Culture::mix_colors(
            Culture::Void,
            Culture::Ember,
            &MixingContext::default()
        );
        assert!(matches!(result, MixingResult::Void(_)));
    }
    
    #[test]
    fn test_mixing_probabilities() {
        let adjacent_prob = Culture::calculate_mixing_probability(Culture::Ember, Culture::Gale);
        assert!((adjacent_prob - 0.8).abs() < 0.01);
        
        let opposite_prob = Culture::calculate_mixing_probability(Culture::Ember, Culture::Tide);
        assert!((opposite_prob - 0.1).abs() < 0.01);
    }
}
```

## Validation Criteria

- [ ] All mixing combinations produce expected offspring
- [ ] Probability calculations reflect angular relationships
- [ ] Environmental modifiers apply correctly
- [ ] Void dominance rule functions properly
- [ ] Mutation system applies balanced enhancements
- [ ] Generational ratchet prevents stat decay
- [ ] UI provides clear mixing guidance

## Future Enhancements

1. **Advanced Mixing**: Three-parent combinations
2. **Chromatic Storm Events**: Temporary rare combination unlocks
3. **Cultural Evolution**: Dynamic culture discovery
4. **Seasonal Mixing**: Time-based combination bonuses
5. **Community Recipes**: Player-discovered special combinations

The Color Mixing Logic transforms breeding from a simple mechanical process into an intuitive, visually-driven system that rewards experimentation and strategic thinking while maintaining mathematical rigor and balance.
