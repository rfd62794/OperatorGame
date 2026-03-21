# Sub-Faction Generation Logic

> **Status:** HYBRID CULTURE SYSTEM v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-026, LIVING_ECONOMY_SUB_FACTIONS.md, PERSONALITY_CORES_SYSTEM.md

## Overview

The Sub-Faction Generation Logic creates hybrid cultural entities that emerge naturally from the overlap and interaction of primary cultures. These sub-factions represent the "Living Web" of the planet, where cultural mixing produces new entities with unique traits, economic specializations, and the ability to enable advanced cross-breeding that would be impossible through direct cultural interaction.

## Generation Algorithm

### Core Generation System

```rust
#[derive(Debug, Clone)]
pub struct SubFactionGenerator {
    pub generation_rules: GenerationRules,
    pub compatibility_matrix: CompatibilityMatrix,
    pub evolution_parameters: EvolutionParameters,
    pub environmental_factors: EnvironmentalFactors,
    pub shepherd_influence: ShepherdInfluence,
}

#[derive(Debug, Clone)]
pub struct GenerationRules {
    pub min_overlap_duration: Duration,
    pub min_cultural_compatibility: f32,
    pub max_generation_distance: u8,
    pub mutation_probability: f32,
    pub stability_threshold: f32,
    pub emergence_conditions: Vec<EmergenceCondition>,
}

#[derive(Debug, Clone)]
pub struct CompatibilityMatrix {
    pub cultural_compatibility: HashMap<(Culture, Culture), f32>,
    pub personality_compatibility: HashMap<(PersonalityCore, PersonalityCore), f32>,
    pub environmental_compatibility: HashMap<(BiomeType, BiomeType), f32>,
    pub temporal_compatibility: HashMap<(TimeWindow, TimeWindow), f32>,
}

#[derive(Debug, Clone)]
pub struct EvolutionParameters {
    pub mutation_rate: f32,
    pub adaptation_rate: f32,
    pub stability_decay: f32,
    pub growth_rate: f32,
    pub specialization_tendency: f32,
    pub diversification_pressure: f32,
}

impl SubFactionGenerator {
    pub fn new() -> Self {
        Self {
            generation_rules: GenerationRules::default(),
            compatibility_matrix: CompatibilityMatrix::new(),
            evolution_parameters: EvolutionParameters::default(),
            environmental_factors: EnvironmentalFactors::new(),
            shepherd_influence: ShepherdInfluence::new(),
        }
    }
    
    pub fn evaluate_subfaction_potential(
        &self,
        node_a: &MapNode,
        node_b: &MapNode,
        overlap_data: &OverlapData
    ) -> SubFactionPotential {
        // Calculate base compatibility
        let cultural_compatibility = self.calculate_cultural_compatibility(
            node_a.culture, 
            node_b.culture
        );
        
        // Check minimum requirements
        if cultural_compatibility < self.generation_rules.min_cultural_compatibility {
            return SubFactionPotential::Incompatible;
        }
        
        if overlap_data.duration < self.generation_rules.min_overlap_duration {
            return SubFactionPotential::InsufficientOverlap;
        }
        
        // Calculate generation probability
        let generation_score = self.calculate_generation_score(
            node_a, 
            node_b, 
            overlap_data, 
            cultural_compatibility
        );
        
        // Check emergence conditions
        let emergence_met = self.check_emergence_conditions(
            node_a, 
            node_b, 
            overlap_data, 
            &self.generation_rules.emergence_conditions
        );
        
        if !emergence_met {
            return SubFactionPotential::ConditionsNotMet;
        }
        
        SubFactionPotential::Viable {
            generation_probability: generation_score,
            estimated_traits: self.predict_hybrid_traits(node_a.culture, node_b.culture),
            economic_specialization: self.predict_economic_specialization(node_a.culture, node_b.culture),
            stability_prediction: self.predict_stability(node_a, node_b, overlap_data),
        }
    }
    
    pub fn generate_subfaction(
        &self,
        node_a: &MapNode,
        node_b: &MapNode,
        overlap_data: &OverlapData,
        shepherd_context: &ShepherdContext
    ) -> Result<SubFaction, GenerationError> {
        // Validate generation potential
        let potential = self.evaluate_subfaction_potential(node_a, node_b, overlap_data);
        
        let viable = match potential {
            SubFactionPotential::Viable { generation_probability, .. } => {
                // Apply shepherd influence
                let modified_probability = self.apply_shepherd_influence(
                    generation_probability, 
                    shepherd_context
                );
                
                // Random chance based on probability
                if rand::random::<f32>() > modified_probability {
                    return Err(GenerationError::GenerationFailed);
                }
                
                true
            },
            _ => return Err(GenerationError::NotViable),
        };
        
        // Generate hybrid culture
        let hybrid_culture = self.create_hybrid_culture(node_a.culture, node_b.culture);
        
        // Generate unique traits
        let unique_traits = self.generate_unique_traits(&hybrid_culture, node_a, node_b);
        
        // Determine economic specialization
        let trade_specialization = self.determine_trade_specialization(&hybrid_culture);
        
        // Calculate initial stability
        let initial_stability = self.calculate_initial_stability(
            node_a, 
            node_b, 
            &hybrid_culture, 
            overlap_data
        );
        
        // Generate subfaction
        let subfaction = SubFaction {
            id: Uuid::new_v4(),
            name: self.generate_subfaction_name(node_a.culture, node_b.culture),
            parent_cultures: (node_a.culture, node_b.culture),
            hybrid_culture,
            generation: 1,
            stability: initial_stability,
            influence_radius: self.calculate_influence_radius(node_a, node_b),
            unique_traits,
            trade_specialization,
            origin_location: self.calculate_origin_location(node_a, node_b),
            emergence_timestamp: chrono::Utc::now(),
            shepherd_involved: shepherd_context.is_present,
        };
        
        Ok(subfaction)
    }
    
    fn calculate_generation_score(
        &self,
        node_a: &MapNode,
        node_b: &MapNode,
        overlap_data: &OverlapData,
        cultural_compatibility: f32
    ) -> f32 {
        let mut score = cultural_compatibility;
        
        // Duration factor
        let duration_factor = (overlap_data.duration.as_secs_f32() / 
            self.generation_rules.min_overlap_duration.as_secs_f32()).min(2.0);
        score *= duration_factor;
        
        // Node strength factor
        let node_strength_factor = ((node_a.control_strength + node_b.control_strength) / 2.0).min(1.5);
        score *= node_strength_factor;
        
        // Environmental factor
        let environmental_factor = self.environmental_factors
            .calculate_compatibility(node_a.biome, node_b.biome);
        score *= environmental_factor;
        
        // Shepherd presence factor
        let shepherd_factor = if overlap_data.shepherd_present {
            1.2 // 20% bonus with shepherd present
        } else {
            1.0
        };
        score *= shepherd_factor;
        
        // Temporal factor
        let temporal_factor = self.calculate_temporal_compatibility(
            node_a.active_time_window,
            node_b.active_time_window
        );
        score *= temporal_factor;
        
        score.clamp(0.0, 1.0)
    }
}
```

### Hybrid Culture Creation

```rust
impl SubFactionGenerator {
    fn create_hybrid_culture(&self, primary: Culture, secondary: Culture) -> HybridCulture {
        let cultural_balance = self.calculate_cultural_balance(primary, secondary);
        let mixed_traits = self.generate_mixed_traits(primary, secondary, cultural_balance);
        let mutation_level = self.determine_mutation_level(primary, secondary);
        
        HybridCulture {
            primary_influence: primary,
            secondary_influence: secondary,
            mixed_traits,
            cultural_balance,
            mutation_level,
            emergent_properties: self.generate_emergent_properties(primary, secondary),
        }
    }
    
    fn calculate_cultural_balance(&self, primary: Culture, secondary: Culture) -> f32 {
        // Base balance influenced by cultural dominance
        let primary_dominance = primary.get_cultural_dominance();
        let secondary_dominance = secondary.get_cultural_dominance();
        
        let total_dominance = primary_dominance + secondary_dominance;
        let primary_ratio = primary_dominance / total_dominance;
        
        // Add some randomness for natural variation
        let random_factor = (rand::random::<f32>() - 0.5) * 0.2; // ±10%
        
        (primary_ratio + random_factor).clamp(0.1, 0.9)
    }
    
    fn generate_mixed_traits(
        &self,
        primary: Culture,
        secondary: Culture,
        cultural_balance: f32
    ) -> Vec<MixedTrait> {
        let mut traits = Vec::new();
        
        // Inherit traits from both parents
        traits.push(MixedTrait::InheritedFromPrimary(primary.get_core_trait()));
        traits.push(MixedTrait::InheritedFromSecondary(secondary.get_core_trait()));
        
        // Determine if traits conflict
        let primary_trait = primary.get_core_trait();
        let secondary_trait = secondary.get_core_trait();
        
        if self.traits_conflict(primary_trait, secondary_trait) {
            traits.push(MixedTrait::Conflicted(self.generate_conflicted_trait(primary_trait, secondary_trait)));
        } else {
            traits.push(MixedTrait::Harmonized(self.generate_harmonized_trait(primary_trait, secondary_trait)));
        }
        
        // Generate emergent traits based on combination
        let emergent_trait = self.generate_emergent_trait(primary, secondary, cultural_balance);
        traits.push(MixedTrait::Emergent(emergent_trait));
        
        // Add adaptive trait if balance is close to 0.5
        if (cultural_balance - 0.5).abs() < 0.2 {
            traits.push(MixedTrait::Emergent(EmergentTrait::Adaptive));
        }
        
        traits
    }
    
    fn generate_emergent_properties(&self, primary: Culture, secondary: Culture) -> Vec<EmergentProperty> {
        let mut properties = Vec::new();
        
        // Combine cultural properties
        let primary_properties = primary.get_cultural_properties();
        let secondary_properties = secondary.get_cultural_properties();
        
        // Find synergistic combinations
        for prop_a in &primary_properties {
            for prop_b in &secondary_properties {
                if let Some(synergy) = self.check_synergy(prop_a, prop_b) {
                    properties.push(synergy);
                }
            }
        }
        
        // Add unique hybrid properties
        properties.push(EmergentProperty::CrossCulturalCommunication);
        properties.push(EmergentProperty::AdaptiveEconomy);
        
        // Special properties for specific combinations
        match (primary, secondary) {
            (Culture::Tide, Culture::Marsh) => {
                properties.push(EmergentProperty::EnhancedGrowth);
                properties.push(EmergentProperty::Purification);
            },
            (Culture::Ember, Culture::Crystal) => {
                properties.push(EmergentProperty::EnergyAmplification);
                properties.push(EmergentProperty::StructuralIntegrity);
            },
            (Culture::Gale, Culture::Teal) => {
                properties.push(EmergentProperty::StealthMovement);
                properties.push(EmergentProperty::InformationGathering);
            },
            _ => {}
        }
        
        properties
    }
}
```

### Evolution and Adaptation

```rust
impl SubFaction {
    pub fn evolve(&mut self, context: &EvolutionContext) {
        // Update stability based on environmental factors
        self.update_stability(context);
        
        // Check for mutations
        if self.should_mutate(context) {
            self.apply_mutation();
        }
        
        // Adapt to environment
        self.adapt_to_environment(context);
        
        // Update traits based on experience
        self.update_traits(context);
        
        // Potentially specialize or diversify
        self.consider_specialization(context);
    }
    
    fn update_stability(&mut self, context: &EvolutionContext) {
        let stability_change = context.environmental_stability;
        let shepherd_influence = context.shepherd_influence;
        let internal_pressure = self.calculate_internal_pressure();
        
        let net_change = stability_change + shepherd_influence - internal_pressure;
        self.stability = (self.stability + net_change).clamp(0.0, 1.0);
        
        // Low stability can cause trait changes
        if self.stability < 0.3 {
            self.handle_instability(context);
        }
    }
    
    fn should_mutate(&self, context: &EvolutionContext) -> bool {
        let base_mutation_chance = context.mutation_rate;
        let stability_factor = 1.0 - self.stability; // Less stable = more mutations
        let environmental_pressure = context.environmental_pressure;
        let shepherd_encouragement = context.shepherd_mutation_encouragement;
        
        let mutation_chance = base_mutation_chance * stability_factor * 
            (1.0 + environmental_pressure) * 
            (1.0 + shepherd_encouragement);
        
        rand::random::<f32>() < mutation_chance
    }
    
    fn apply_mutation(&mut self) {
        self.mutation_level += 1;
        
        // Randomly modify traits
        let trait_index = rand::random::<usize>() % self.hybrid_culture.mixed_traits.len();
        
        match rand::random::<u8>() % 4 {
            0 => {
                // Replace trait with emergent
                self.hybrid_culture.mixed_traits[trait_index] = 
                    MixedTrait::Emergent(EmergentTrait::Chaotic);
            },
            1 => {
                // Make trait conflicted
                if let MixedTrait::InheritedFromPrimary(_) | MixedTrait::InheritedFromSecondary(_) = 
                    &self.hybrid_culture.mixed_traits[trait_index] {
                    self.hybrid_culture.mixed_traits[trait_index] = 
                        MixedTrait::Conflicted(ConflictedTrait::Unstable);
                }
            },
            2 => {
                // Enhance trait
                self.enhance_trait(trait_index);
            },
            3 => {
                // Add new emergent trait
                self.hybrid_culture.mixed_traits.push(
                    MixedTrait::Emergent(self.generate_random_emergent_trait())
                );
            },
            _ => {}
        }
        
        // Update name to reflect mutation
        self.name = format!("Mutated {}", self.name);
    }
    
    fn adapt_to_environment(&mut self, context: &EvolutionContext) {
        // Adjust cultural balance based on environmental pressure
        let environmental_bias = context.get_environmental_bias();
        
        if environmental_bias.abs() > 0.3 {
            let adjustment = environmental_bias * 0.1; // Gradual adjustment
            self.hybrid_culture.cultural_balance = 
                (self.hybrid_culture.cultural_balance + adjustment).clamp(0.1, 0.9);
        }
        
        // Develop adaptive traits
        if context.environmental_pressure > 0.7 {
            if !self.has_adaptive_trait() {
                self.hybrid_culture.mixed_traits.push(
                    MixedTrait::Emergent(EmergentTrait::Adaptive)
                );
            }
        }
    }
    
    fn consider_specialization(&mut self, context: &EvolutionContext) {
        let specialization_pressure = context.specialization_pressure;
        let diversification_pressure = context.diversification_pressure;
        
        if specialization_pressure > diversification_pressure {
            // Move toward specialization
            self.specialize_further(context);
        } else if diversification_pressure > specialization_pressure {
            // Move toward diversification
            self.diversify_further(context);
        }
    }
    
    fn specialize_further(&mut self, context: &EvolutionContext) {
        // Enhance dominant traits
        let dominant_culture = if self.hybrid_culture.cultural_balance > 0.5 {
            self.hybrid_culture.primary_influence
        } else {
            self.hybrid_culture.secondary_influence
        };
        
        // Add specialization traits
        let specialization_traits = dominant_culture.get_specialization_traits();
        
        for trait_type in specialization_traits {
            if !self.has_trait_type(trait_type) {
                self.hybrid_culture.mixed_traits.push(
                    MixedTrait::InheritedFromPrimary(trait_type)
                );
                break; // Add one specialization at a time
            }
        }
    }
    
    fn diversify_further(&mut self, context: &EvolutionContext) {
        // Add more diverse emergent traits
        let new_emergent = self.generate_random_emergent_trait();
        
        if !self.has_emergent_trait(new_emergent) {
            self.hybrid_culture.mixed_traits.push(
                MixedTrait::Emergent(new_emergent)
            );
        }
    }
}
```

### Shepherd Influence System

```rust
#[derive(Debug, Clone)]
pub struct ShepherdInfluence {
    pub presence_bonus: f32,
    pub diplomatic_bonus: f32,
    pub economic_bonus: f32,
    pub stability_bonus: f32,
    pub mutation_encouragement: f32,
    pub specialization_guidance: HashMap<Culture, SpecializationGuidance>,
}

#[derive(Debug, Clone)]
pub struct SpecializationGuidance {
    pub target_specialization: EconomicSpecialization,
    pub encouragement_level: f32,
    pub resource_support: HashMap<ResourceType, u64>,
    pub time_limit: Option<Duration>,
}

impl ShepherdInfluence {
    pub fn new() -> Self {
        Self {
            presence_bonus: 0.2,
            diplomatic_bonus: 0.15,
            economic_bonus: 0.1,
            stability_bonus: 0.25,
            mutation_encouragement: 0.05,
            specialization_guidance: HashMap::new(),
        }
    }
    
    pub fn apply_influence(&self, subfaction: &mut SubFaction, context: &ShepherdContext) {
        // Apply presence bonus
        if context.is_present {
            subfaction.stability = (subfaction.stability + self.presence_bonus).min(1.0);
        }
        
        // Apply diplomatic bonus
        if context.diplomatic_actions > 0 {
            subfaction.improve_relationships(self.diplomatic_bonus);
        }
        
        // Apply economic bonus
        if context.trade_support {
            subfaction.boost_economy(self.economic_bonus);
        }
        
        // Apply stability bonus
        if context.stability_support {
            subfaction.stability = (subfaction.stability + self.stability_bonus).min(1.0);
        }
        
        // Apply mutation encouragement
        if context.mutation_encouragement {
            subfaction.mutation_encouragement = self.mutation_encouragement;
        }
        
        // Apply specialization guidance
        if let Some(guidance) = self.specialization_guidance.get(&subfaction.parent_cultures.0) {
            subfaction.apply_specialization_guidance(guidance);
        }
    }
    
    pub fn calculate_influence_probability(&self, base_probability: f32, context: &ShepherdContext) -> f32 {
        let mut modified_probability = base_probability;
        
        if context.is_present {
            modified_probability += self.presence_bonus;
        }
        
        if context.diplomatic_actions > 0 {
            modified_probability += self.diplomatic_bonus * context.diplomatic_actions as f32 * 0.1;
        }
        
        if context.trade_support {
            modified_probability += self.economic_bonus;
        }
        
        modified_probability.clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct ShepherdContext {
    pub is_present: bool,
    pub diplomatic_actions: u32,
    pub trade_support: bool,
    pub stability_support: bool,
    pub mutation_encouragement: bool,
    pub resource_contributions: HashMap<ResourceType, u64>,
    pub relationship_level: f32,
    pub recent_actions: Vec<ShepherdAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShepherdAction {
    DiplomaticMission { target_culture: Culture },
    TradeFacilitation { partner_culture: Culture },
    ResourceDonation { resource: ResourceType, amount: u64 },
    StabilitySupport { duration: Duration },
    MutationEncouragement,
    SpecializationGuidance { target: Culture, specialization: EconomicSpecialization },
}
```

## Implementation Tasks

### Core Algorithm Development

1. **Implement Generation Rules**: Define sub-faction creation criteria
2. **Build Compatibility Matrix**: Cultural and environmental compatibility
3. **Create Evolution System**: Mutation and adaptation mechanics
4. **Develop Shepherd Influence**: Player impact on sub-faction development
5. **Implement Trait System**: Mixed trait generation and evolution

### Integration Points

1. **World Map Integration**: Sub-faction node placement and visualization
2. **Economic System**: Trade specialization and market integration
3. **Breeding System**: Cross-culture breeding capabilities
4. **UI Integration**: Sub-faction information and interaction

### Balance and Testing

1. **Generation Balance**: Ensure sub-factions don't overwhelm primary cultures
2. **Evolution Stability**: Prevent runaway mutations or instability
3. **Shepherd Impact**: Balance player influence with natural evolution
4. **Economic Integration**: Ensure sub-factions enhance rather than disrupt trade

## Validation Criteria

- [ ] Sub-factions generate logically from compatible culture overlaps
- [ ] Evolution system creates meaningful diversity without chaos
- [ ] Shepherd influence provides strategic depth without overpowered control
- [ ] Economic specialization creates interesting trade opportunities
- [ ] Trait system produces balanced and useful hybrid abilities
- [ ] System remains stable and predictable enough for strategic planning

The Sub-Faction Generation Logic creates a living, evolving web of hybrid cultures that emerge naturally from planetary interactions, providing rich strategic opportunities while maintaining the balance and coherence of the cultural ecosystem.
