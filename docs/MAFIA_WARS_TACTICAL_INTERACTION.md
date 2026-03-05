# Mafia Wars Tactical Interaction

> **Status:** STRATEGIC BLESSING SYSTEM v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-025, ELDER_LIVING_INTERFACE.md, PLANETARY_TOPOLOGY_RIPPLE_MAP.md

## Overview

The Mafia Wars Tactical Interaction system transforms the Elder from a passive background element into an active strategic asset that provides daily blessings and tactical advantages. The Elder's Blessing creates a meaningful daily interaction loop where the Astronaut can gain significant advantages for expeditions in specific cultural regions, creating strategic depth in resource allocation and expedition planning.

## Blessing Mechanics

### Daily Blessing System

```rust
#[derive(Debug, Clone)]
pub struct ElderBlessingSystem {
    pub daily_blessing_available: bool,
    pub blessing_cooldown: Duration,
    pub last_blessing_time: Option<chrono::DateTime<chrono::Utc>>,
    pub active_blessings: Vec<ActiveBlessing>,
    pub blessing_history: Vec<BlessingRecord>,
    pub blessing_requirements: BlessingRequirements,
    pub blessing_effects: BlessingEffects,
}

#[derive(Debug, Clone)]
pub struct BlessingRequirements {
    pub minimum_offering: ResourceCost,
    pub preferred_offering: Option<ResourceType>,
    pub offering_tiers: Vec<OfferingTier>,
    pub cultural_affinity_bonus: f32,
    pub timing_bonus: f32,
    pub relationship_bonus: f32,
}

#[derive(Debug, Clone)]
pub struct OfferingTier {
    pub tier_number: u8,
    pub resource_requirement: ResourceCost,
    pub blessing_strength: f32,
    pub blessing_duration: Duration,
    pub special_effects: Vec<SpecialEffect>,
}

#[derive(Debug, Clone)]
pub struct BlessingEffects {
    pub expedition_bonus: ExpeditionBonus,
    pub cultural_bonus: CulturalBonus,
    pub environmental_bonus: EnvironmentalBonus,
    pub resource_bonus: ResourceBonus,
    pub combat_bonus: CombatBonus,
}

#[derive(Debug, Clone)]
pub struct ExpeditionBonus {
    pub success_rate_modifier: f32,
    pub resource_yield_modifier: f32,
    pub encounter_difficulty_modifier: f32,
    pub travel_time_modifier: f32,
    pub risk_reduction: f32,
}

impl ElderBlessingSystem {
    pub fn new() -> Self {
        Self {
            daily_blessing_available: true,
            blessing_cooldown: Duration::from_secs(86400), // 24 hours
            last_blessing_time: None,
            active_blessings: Vec::new(),
            blessing_history: Vec::new(),
            blessing_requirements: BlessingRequirements::new(),
            blessing_effects: BlessingEffects::new(),
        }
    }
    
    pub fn can_request_blessing(&self) -> bool {
        self.daily_blessing_available
    }
    
    pub fn request_blessing(&mut self, offering: ResourceCost, target_culture: Culture) -> BlessingResult {
        if !self.can_request_blessing() {
            return BlessingResult::NotAvailable;
        }
        
        // Check if offering meets requirements
        if !self.meets_requirements(&offering) {
            return BlessingResult::InsufficientOffering;
        }
        
        // Calculate blessing strength
        let blessing_strength = self.calculate_blessing_strength(&offering, target_culture);
        
        // Generate blessing
        let blessing = self.generate_blessing(target_culture, blessing_strength);
        
        // Apply blessing
        self.active_blessings.push(blessing.clone());
        
        // Update cooldown
        self.daily_blessing_available = false;
        self.last_blessing_time = Some(chrono::Utc::now());
        
        // Record blessing
        self.blessing_history.push(BlessingRecord {
            timestamp: chrono::Utc::now(),
            offering,
            target_culture,
            blessing_strength,
            blessing_type: blessing.blessing_type,
            duration: blessing.duration,
        });
        
        BlessingResult::Success {
            blessing,
            remaining_cooldown: self.blessing_cooldown,
        }
    }
    
    fn meets_requirements(&self, offering: &ResourceCost) -> bool {
        let minimum = &self.blessing_requirements.minimum_offering;
        
        offering.biomass >= minimum.biomass &&
        offering.scrap >= minimum.scrap &&
        offering.energy >= minimum.energy &&
        offering.research_points >= minimum.research_points
    }
    
    fn calculate_blessing_strength(&self, offering: &ResourceCost, target_culture: Culture) -> f32 {
        let base_strength = 0.5;
        
        // Calculate offering value
        let offering_value = self.calculate_offering_value(offering);
        
        // Apply cultural affinity bonus
        let cultural_bonus = self.calculate_cultural_affinity_bonus(target_culture);
        
        // Apply timing bonus
        let timing_bonus = self.calculate_timing_bonus();
        
        // Apply relationship bonus
        let relationship_bonus = self.calculate_relationship_bonus();
        
        let total_strength = base_strength + offering_value + cultural_bonus + timing_bonus + relationship_bonus;
        
        total_strength.min(1.0).max(0.1)
    }
    
    fn calculate_offering_value(&self, offering: &ResourceCost) -> f32 {
        let minimum = &self.blessing_requirements.minimum_offering;
        
        let biomass_ratio = (offering.biomass as f32 / minimum.biomass as f32).min(2.0);
        let scrap_ratio = (offering.scrap as f32 / minimum.scrap as f32).min(2.0);
        let energy_ratio = (offering.energy as f32 / minimum.energy as f32).min(2.0);
        let research_ratio = (offering.research_points as f32 / minimum.research_points as f32).min(2.0);
        
        let total_ratio = (biomass_ratio + scrap_ratio + energy_ratio + research_ratio) / 4.0;
        
        (total_ratio - 1.0) * 0.3 // Convert to 0.0-0.3 range
    }
    
    fn calculate_cultural_affinity_bonus(&self, target_culture: Culture) -> f32 {
        // Elder has affinity for all cultures, but some more than others
        match target_culture {
            Culture::Void => 0.2, // Highest affinity
            Culture::Crystal => 0.15, // High affinity
            Culture::Ember => 0.1, // Good affinity
            Culture::Tide => 0.1, // Good affinity
            Culture::Gale => 0.05, // Moderate affinity
            Culture::Orange => 0.05, // Moderate affinity
            Culture::Marsh => 0.05, // Moderate affinity
            Culture::Teal => 0.05, // Moderate affinity
            Culture::Amber => 0.05, // Moderate affinity
            Culture::Tundra => 0.05, // Moderate affinity
        }
    }
    
    fn calculate_timing_bonus(&self) -> f32 {
        // Time of day affects blessing strength
        let current_hour = chrono::Utc::now().hour();
        
        match current_hour {
            6..=8 => 0.1, // Morning blessing
            12..=14 => 0.05, // Midday blessing
            18..=20 => 0.1, // Evening blessing
            0..=2 => 0.15, // Night blessing
            _ => 0.0, // Normal time
        }
    }
    
    fn calculate_relationship_bonus(&self) -> f32 {
        // Based on interaction history
        let recent_interactions = self.blessing_history.iter()
            .filter(|record| record.timestamp > chrono::Utc::now() - Duration::from_secs(86400 * 7)) // Last 7 days
            .count();
        
        match recent_interactions {
            0 => 0.0,
            1..=3 => 0.05,
            4..=7 => 0.1,
            8..=14 => 0.15,
            _ => 0.2,
        }
    }
    
    fn generate_blessing(&self, target_culture: Culture, strength: f32) -> ActiveBlessing {
        let blessing_type = self.determine_blessing_type(target_culture);
        let duration = self.calculate_blessing_duration(strength);
        let effects = self.calculate_blessing_effects(target_culture, strength);
        
        ActiveBlessing {
            id: Uuid::new_v4(),
            blessing_type,
            target_culture,
            strength,
            duration,
            start_time: chrono::Utc::now(),
            effects,
            visual_effect: self.generate_visual_effect(target_culture, strength),
            audio_effect: self.generate_audio_effect(target_culture, strength),
        }
    }
    
    fn determine_blessing_type(&self, target_culture: Culture) -> BlessingType {
        match target_culture {
            Culture::Void => BlessingType::UniversalBonus,
            Culture::Crystal => BlessingType::StructuralBonus,
            Culture::Ember => BlessingType::CombatBonus,
            Culture::Tide => BlessingType::DiplomaticBonus,
            Culture::Gale => BlessingType::SpeedBonus,
            Culture::Orange => BlessingType::EngineeringBonus,
            Culture::Marsh => BlessingType::GrowthBonus,
            Culture::Teal => BlessingType::StealthBonus,
            Culture::Amber => BlessingType::DurabilityBonus,
            Culture::Tundra => BlessingType::SurvivalBonus,
        }
    }
    
    fn calculate_blessing_duration(&self, strength: f32) -> Duration {
        let base_duration = Duration::from_secs(14400); // 4 hours base
        
        let duration_modifier = 1.0 + (strength * 2.0); // 1.0 to 3.0 multiplier
        
        Duration::from_secs((base_duration.as_secs() as f32 * duration_modifier) as u64)
    }
    
    fn calculate_blessing_effects(&self, target_culture: Culture, strength: f32) -> Vec<BlessingEffect> {
        let mut effects = Vec::new();
        
        // Base effects
        effects.push(BlessingEffect::SuccessRateBonus { modifier: strength * 0.2 });
        effects.push(BlessingEffect::ResourceYieldBonus { modifier: strength * 0.15 });
        effects.push(BlessingEffect::RiskReduction { modifier: strength * 0.1 });
        
        // Culture-specific effects
        match target_culture {
            Culture::Void => {
                effects.push(BlessingEffect::UniversalBonus { modifier: strength * 0.25 });
                effects.push(BlessingEffect::ShipSystemBonus { modifier: strength * 0.2 });
            },
            Culture::Crystal => {
                effects.push(BlessingEffect::StructuralIntegrityBonus { modifier: strength * 0.2 });
                effects.push(BlessingEffect::ShieldGenerationBonus { modifier: strength * 0.15 });
            },
            Culture::Ember => {
                effects.push(BlessingEffect::CombatDamageBonus { modifier: strength * 0.25 });
                effects.push(BlessingEffect::MiningEfficiencyBonus { modifier: strength * 0.15 });
            },
            Culture::Tide => {
                effects.push(BlessingEffect::DiplomaticSuccessBonus { modifier: strength * 0.2 });
                effects.push(BlessingEffect::TradeBonus { modifier: strength * 0.15 });
            },
            Culture::Gale => {
                effects.push(BlessingEffect::SpeedBonus { modifier: strength * 0.2 });
                effects.push(BlessingEffect::EvasionBonus { modifier: strength * 0.15 });
            },
            Culture::Orange => {
                effects.push(BlessingEffect::EngineeringSpeedBonus { modifier: strength * 0.2 });
                effects.push(BlessingEffect::ConstructionSpeedBonus { modifier: strength * 0.15 });
            },
            Culture::Marsh => {
                effects.push(BlessingEffect::GrowthRateBonus { modifier: strength * 0.25 });
                effects.push(BlessingEffect::BioProcessingBonus { modifier: strength * 0.15 });
            },
            Culture::Teal => {
                effects.push(BlessingEffect::StealthRegenerationBonus { modifier: strength * 0.2 });
                effects.push(BlessingEffect::EnvironmentalResistanceBonus { modifier: strength * 0.15 });
            },
            Culture::Amber => {
                effects.push(BlessingEffect::DurabilityBonus { modifier: strength * 0.2 });
                effects.push(BlessingEffect::MiningYieldBonus { modifier: strength * 0.15 });
            },
            Culture::Tundra => {
                effects.push(BlessingEffect::ThermalResistanceBonus { modifier: strength * 0.25 });
                effects.push(BlessingEffect::ColdResistanceBonus { modifier: strength * 0.15 });
            },
        }
        
        effects
    }
}
```

### Tactical Application

```rust
impl ElderBlessingSystem {
    pub fn apply_blessing_to_expedition(&self, expedition: &mut Expedition, blessing: &ActiveBlessing) {
        // Apply blessing effects to expedition
        for effect in &blessing.effects {
            match effect {
                BlessingEffect::SuccessRateBonus { modifier } => {
                    expedition.success_rate_modifier += modifier;
                },
                BlessingEffect::ResourceYieldBonus { modifier } => {
                    expedition.resource_yield_modifier += modifier;
                },
                BlessingEffect::RiskReduction { modifier } => {
                    expedition.risk_reduction += modifier;
                },
                BlessingEffect::CombatDamageBonus { modifier } => {
                    expedition.combat_damage_modifier += modifier;
                },
                BlessingEffect::DiplomaticSuccessBonus { modifier } => {
                    expedition.diplomatic_success_modifier += modifier;
                },
                BlessingEffect::SpeedBonus { modifier } => {
                    expedition.travel_time_modifier *= (1.0 - modifier);
                },
                BlessingEffect::StealthBonus { modifier } => {
                    expedition.stealth_modifier += modifier;
                },
                BlessingEffect::EngineeringBonus { modifier } => {
                    expedition.engineering_success_modifier += modifier;
                },
                BlessingEffect::GrowthBonus { modifier } => {
                    expedition.biomass_yield_modifier += modifier;
                },
                BlessingEffect::ResearchBonus { modifier } => {
                    expedition.research_yield_modifier += modifier;
                },
                _ => {}
            }
        }
    }
    
    pub fn get_blessing_for_culture(&self, culture: Culture) -> Option<&ActiveBlessing> {
        self.active_blessings.iter()
            .find(|blessing| blessing.target_culture == culture || blessing.blessing_type == BlessingType::UniversalBonus)
    }
    
    pub fn get_active_blessings(&self) -> Vec<&ActiveBlessing> {
        self.active_blessings.iter().collect()
    }
    
    pub fn update_blessings(&mut self, delta_time: Duration) {
        let now = chrono::Utc::now();
        
        // Remove expired blessings
        self.active_blessings.retain(|blessing| {
            let elapsed = now.signed_duration_since(blessing.start_time);
            elapsed < blessing.duration
        });
        
        // Check if daily blessing is available
        if let Some(last_blessing) = self.last_blessing_time {
            let time_since_last = now.signed_duration_since(last_blessing);
            if time_since_last >= self.blessing_cooldown {
                self.daily_blessing_available = true;
            }
        }
    }
    
    pub fn get_blessing_recommendations(&self, current_expeditions: &[Expedition]) -> Vec<BlessingRecommendation> {
        let mut recommendations = Vec::new();
        
        for expedition in current_expeditions {
            let target_culture = expedition.target_culture;
            
            // Check if blessing is active
            if let Some(blessing) = self.get_blessing_for_culture(target_culture) {
                // Blessing is already active
                continue;
            }
            
            // Calculate blessing value
            let blessing_value = self.calculate_blessing_value_for_expedition(expedition);
            
            if blessing_value > 0.5 {
                recommendations.push(BlessingRecommendation {
                    target_culture,
                    recommended_offering: self.calculate_recommended_offering(target_culture),
                    expected_benefits: self.calculate_expected_benefits(target_culture),
                    urgency: self.calculate_blessing_urgency(expedition),
                    value_score: blessing_value,
                });
            }
        }
        
        // Sort by value score
        recommendations.sort_by(|a, b| b.value_score.partial_cmp(&a.value_score).unwrap());
        
        recommendations
    }
    
    fn calculate_blessing_value_for_expedition(&self, expedition: &Expedition) -> f32 {
        let mut value = 0.0;
        
        // Base value based on expedition difficulty
        value += expedition.difficulty_level * 0.2;
        
        // Value based on resource importance
        value += expedition.resource_importance * 0.3;
        
        // Value based on time sensitivity
        value += expedition.time_sensitivity * 0.2;
        
        // Value based on risk level
        value += expedition.risk_level * 0.3;
        
        value.min(1.0)
    }
    
    fn calculate_recommended_offering(&self, target_culture: Culture) -> ResourceCost {
        // Calculate optimal offering for target culture
        let base_offering = &self.blessing_requirements.minimum_offering;
        
        let cultural_modifier = match target_culture {
            Culture::Void => 2.0,
            Culture::Crystal => 1.5,
            Culture::Ember => 1.3,
            Culture::Tide => 1.2,
            Culture::Gale => 1.1,
            Culture::Orange => 1.0,
            Culture::Marsh => 1.0,
            Culture::Teal => 1.0,
            Culture::Amber => 1.0,
            Culture::Tundra => 1.0,
        };
        
        ResourceCost {
            biomass: (base_offering.biomass as f32 * cultural_modifier) as u64,
            scrap: (base_offering.scrap as f32 * cultural_modifier) as u64,
            energy: (base_offering.energy as f32 * cultural_modifier) as u64,
            research_points: (base_offering.research_points as f32 * cultural_modifier) as u64,
        }
    }
    
    fn calculate_expected_benefits(&self, target_culture: Culture) -> Vec<String> {
        let mut benefits = Vec::new();
        
        match target_culture {
            Culture::Void => {
                benefits.push("Universal bonus to all activities".to_string());
                benefits.push("Ship systems efficiency +20%".to_string());
                benefits.push("All expedition success rates +15%".to_string());
            },
            Culture::Crystal => {
                benefits.push("Structural integrity +20%".to_string());
                benefits.push("Shield generation +15%".to_string());
                benefits.push("Research speed +25%".to_string());
            },
            Culture::Ember => {
                benefits.push("Combat damage +25%".to_string());
                benefits.push("Mining efficiency +20%".to_string());
                benefits.push("Scrap yield +15%".to_string());
            },
            Culture::Tide => {
                benefits.push("Diplomatic success +20%".to_string());
                benefits.push("Trade bonus +15%".to_string());
                benefits.push("Biomass yield +10%".to_string());
            },
            Culture::Gale => {
                benefits.push("Speed +25%".to_string());
                benefits.push("Evasion bonus +20%".to_string());
                benefits.push("Travel time -15%".to_string());
            },
            Culture::Orange => {
                benefits.push("Engineering speed +30%".to_string());
                benefits.push("Construction speed +25%".to_string());
                benefits.push("Repair efficiency +20%".to_string());
            },
            Culture::Marsh => {
                benefits.push("Growth rate +40%".to_string());
                benefits.push("Bio-processing +35%".to_string());
                benefits.push("Healing speed +25%".to_string());
            },
            Culture::Teal => {
                benefits.push("Stealth regeneration +30%".to_string());
                benefits.push("Environmental resistance +20%".to_string());
                benefits.push("Stealth +20%".to_string());
            },
            Culture::Amber => {
                benefits.push("Durability +35%".to_string());
                benefits.push("Mining yield +25%".to_string());
                benefits.push("Construction speed +20%".to_string());
            },
            Culture::Tundra => {
                benefits.push("Thermal resistance +40%".to_string());
                benefits.push("Cold resistance +35%".to_string());
                benefits.push("Survival bonus +30%".to_string());
            },
        }
        
        benefits
    }
    
    fn calculate_blessing_urgency(&self, expedition: &Expedition) -> BlessingUrgency {
        if expedition.time_sensitivity > 0.8 {
            BlessingUrgency::Critical
        } else if expedition.risk_level > 0.7 {
            BlessingUrgency::High
        } else if expedition.resource_importance > 0.6 {
            BlessingUrgency::Medium
        } else {
            BlessingUrgency::Low
        }
    }
}
```

## Implementation Tasks

### Core System Development

1. **Create Blessing System**: Implement daily blessing mechanics
2. **Build Offering System**: Create resource offering logic
3. **Develop Effect System**: Implement blessing effects
4. **Create Recommendation System**: Build blessing recommendations
5. **Implement Tactical Application**: Apply blessings to expeditions

### UI Integration

1. **Blessing Interface**: Create blessing request interface
2. **Offering Interface**: Create resource offering interface
3. **Effect Display**: Show active blessing effects
4. **Recommendation Display**: Show blessing recommendations
5. **Cooldown Display**: Show blessing availability

### Integration Points

1. **Expedition System**: Apply blessings to expeditions
2. **Resource System**: Handle resource offerings
3. **Time System**: Manage blessing cooldowns
4. **Audio System**: Add blessing audio effects
5. **Visual System**: Add blessing visual effects

## Validation Criteria

- [ ] Daily blessing creates meaningful strategic choices
- [ Blessing effects provide tangible advantages
- [ Recommendation system provides useful guidance
- [ Tactical application enhances expedition success
- [ Resource offering creates meaningful economy
- [ System remains balanced throughout gameplay

The Mafia Wars Tactical Interaction system creates a strategic daily interaction loop where the Elder's Blessing provides significant advantages for expeditions, creating meaningful choices in resource allocation and expedition planning while maintaining the Elder's role as a central strategic asset.
