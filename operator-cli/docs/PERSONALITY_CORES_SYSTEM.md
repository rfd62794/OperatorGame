# 9-Point Personality Cores System

> **Status:** CULTURAL BEHAVIOR SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-026, LIVING_ECONOMY_SUB_FACTIONS.md, PLANETARY_MAP_LAYOUT.md

## Overview

The 9-Point Personality Cores System transforms static cultures into living entities with distinct behaviors, economic patterns, and diplomatic tendencies. Each culture's Personality Core determines how it expands, trades, fights, and interacts with the Shepherd's slimes, creating a rich web of relationships that evolves over time.

## Personality Core Definitions

### Primary Layer Cores

#### Ember (Red) - Dominant/Aggressive

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DominantCore {
    pub expansion_rate: f32,        // 0.8 - High expansion
    pub territorial_behavior: TerritorialBehavior,
    pub aggression_threshold: f32,  // 0.3 - Low threshold for conflict
    pub resource_focus: ResourceFocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerritorialBehavior {
    Closed,                         // Highly defensive borders
    Expansionist,                   // Actively expands territory
    Fortified,                      // Builds defensive structures
}

impl DominantCore {
    pub fn new() -> Self {
        Self {
            expansion_rate: 0.8,
            territorial_behavior: TerritorialBehavior::Closed,
            aggression_threshold: 0.3,
            resource_focus: ResourceFocus::Scrap,
        }
    }
    
    pub fn calculate_border_behavior(&self, neighbor_culture: Culture) -> BorderAction {
        match neighbor_culture.personality_core() {
            PersonalityCore::Dominant => BorderAction::Conflict,
            PersonalityCore::Judgmental => BorderAction::Alliance,
            PersonalityCore::Nurturing => BorderAction::Exploit,
            _ => BorderAction::Ignore,
        }
    }
    
    pub fn get_economic_profile(&self) -> EconomicProfile {
        EconomicProfile {
            primary_export: ResourceType::Scrap,
            primary_import: ResourceType::Biomass,
            trade_style: TradeStyle::Aggressive,
            price_tendency: PriceTendency::High,
        }
    }
}
```

#### Tide (Blue) - Fluid/Social

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluidCore {
    pub social_openness: f32,        // 0.9 - Very open
    pub adaptation_rate: f32,        // 0.7 - Adapts quickly
    pub cross_breeding_bonus: f32,   // 0.5 - Bonus for mixed breeding
    pub migration_tendency: f32,     // 0.6 - Moves nodes frequently
}

impl FluidCore {
    pub fn new() -> Self {
        Self {
            social_openness: 0.9,
            adaptation_rate: 0.7,
            cross_breeding_bonus: 0.5,
            migration_tendency: 0.6,
        }
    }
    
    pub fn calculate_migration_decision(&self, current_pressure: f32) -> MigrationAction {
        if current_pressure > 0.7 {
            MigrationAction::Relocate
        } else if current_pressure > 0.4 {
            MigrationAction::Expand
        } else {
            MigrationAction::Stabilize
        }
    }
    
    pub fn get_diplomatic_approach(&self, other_culture: Culture) -> DiplomaticApproach {
        match other_culture.personality_core() {
            PersonalityCore::Nurturing => DiplomaticApproach::Friendly,
            PersonalityCore::Erratic => DiplomaticApproach::Cautious,
            PersonalityCore::Dominant => DiplomaticApproach::Appeasing,
            _ => DiplomaticApproach::Neutral,
        }
    }
}
```

#### Gale (Yellow) - Erratic/Absent-minded

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErraticCore {
    pub unpredictability: f32,       // 0.8 - Highly unpredictable
    pub attention_span: f32,         // 0.3 - Short attention
    pub innovation_rate: f32,        // 0.7 - High innovation
    pub memory_retention: f32,       // 0.4 - Poor memory
}

impl ErraticCore {
    pub fn new() -> Self {
        Self {
            unpredictability: 0.8,
            attention_span: 0.3,
            innovation_rate: 0.7,
            memory_retention: 0.4,
        }
    }
    
    pub fn calculate_next_action(&self, context: &NodeContext) -> NodeAction {
        let random_factor = rand::random::<f32>();
        
        if random_factor < self.unpredictability {
            // Random action
            match rand::random::<u8>() % 4 {
                0 => NodeAction::MoveNode,
                1 => NodeAction::ChangeAlliance,
                2 => NodeAction::RandomTrade,
                3 => NodeAction::Innovate,
                _ => NodeAction::NoAction,
            }
        } else {
            // Logical action based on context
            self.calculate_logical_action(context)
        }
    }
    
    fn calculate_logical_action(&self, context: &NodeContext) -> NodeAction {
        if context.resource_shortage {
            NodeAction::SeekResources
        } else if context.threatened {
            NodeAction::Flee
        } else if rand::random::<f32>() > self.attention_span {
            NodeAction::ForgetBorders
        } else {
            NodeAction::MaintainStatus
        }
    }
}
```

### Secondary Layer Cores

#### Orange - Analytical/Stoic

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyticalCore {
    pub logic_priority: f32,         // 0.9 - Very logical
    pub efficiency_focus: f32,       // 0.8 - Efficiency focused
    pub fortification_tendency: f32, // 0.7 - Builds fortifications
    pub research_output: f32,       // 0.6 - High research
}

impl AnalyticalCore {
    pub fn new() -> Self {
        Self {
            logic_priority: 0.9,
            efficiency_focus: 0.8,
            fortification_tendency: 0.7,
            research_output: 0.6,
        }
    }
    
    pub fn calculate_optimal_strategy(&self, situation: &StrategicSituation) -> OptimalStrategy {
        let efficiency_score = self.calculate_efficiency_score(situation);
        let risk_assessment = self.assess_risk(situation);
        
        if efficiency_score > 0.8 && risk_assessment < 0.3 {
            OptimalStrategy::FortifyAndExpand
        } else if risk_assessment > 0.7 {
            OptimalStrategy::DefensivePosture
        } else {
            OptimalStrategy::EfficientProduction
        }
    }
    
    pub fn get_building_preferences(&self) -> Vec<BuildingType> {
        vec![
            BuildingType::ResearchLab,
            BuildingType::Fortification,
            BuildingType::ResourceProcessor,
            BuildingType::StorageFacility,
        ]
    }
}
```

#### Marsh (Green) - Nurturing/Passive

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NurturingCore {
    pub hospitality_level: f32,     // 0.9 - Very welcoming
    pub growth_focus: f32,         // 0.8 - Focus on growth
    pub defense_priority: f32,      // 0.2 - Low defense priority
    pub sharing_tendency: f32,     // 0.8 - Shares resources
}

impl NurturingCore {
    pub fn new() -> Self {
        Self {
            hospitality_level: 0.9,
            growth_focus: 0.8,
            defense_priority: 0.2,
            sharing_tendency: 0.8,
        }
    }
    
    pub fn calculate_trade_generosity(&self, partner_culture: Culture) -> TradeGenerosity {
        let partner_need = self.assess_partner_need(partner_culture);
        let own_surplus = self.calculate_surplus();
        
        if partner_need > 0.7 && own_surplus > 0.5 {
            TradeGenerosity::VeryGenerous
        } else if partner_need > 0.4 {
            TradeGenerosity::Fair
        } else {
            TradeGenerosity::Cautious
        }
    }
    
    pub fn get_breeding_policy(&self) -> BreedingPolicy {
        BreedingPolicy {
            cross_culture_acceptance: 0.9,
            foreign_slime_welcome: true,
            breeding_bonuses: vec![
                BreedingBonus::GrowthRate,
                BreedingBonus::SurvivalRate,
                BreedingBonus::Fertility,
            ],
        }
    }
}
```

#### Crystal (Purple) - Judgmental/Rigid

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JudgmentalCore {
    pub purity_standard: f32,       // 0.9 - High purity standards
    pub exclusion_tendency: f32,    // 0.8 - Excludes foreigners
    pub ritual_importance: f32,    // 0.7 - Values tradition
    pub cultural_pride: f32,       // 0.8 - Very proud
}

impl JudgmentalCore {
    pub fn new() -> Self {
        Self {
            purity_standard: 0.9,
            exclusion_tendency: 0.8,
            ritual_importance: 0.7,
            cultural_pride: 0.8,
        }
    }
    
    pub fn calculate_acceptance(&self, foreign_culture: Culture) -> AcceptanceLevel {
        let cultural_similarity = self.calculate_similarity(foreign_culture);
        let purity_score = cultural_similarity * self.purity_standard;
        
        if purity_score > 0.8 {
            AcceptanceLevel::Accepted
        } else if purity_score > 0.5 {
            AcceptanceLevel::Tolerated
        } else {
            AcceptanceLevel::Rejected
        }
    }
    
    pub fn get_diplomatic_requirements(&self) -> Vec<DiplomaticRequirement> {
        vec![
            DiplomaticRequirement::CulturalPurity,
            DiplomaticRequirement::RitualRespect,
            DiplomaticRequirement::FormalProtocol,
            DiplomaticRequirement::GiftExchange,
        ]
    }
}
```

### Tertiary Layer Cores

#### Amber - Industrious/Dull

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndustriousCore {
    pub work_ethic: f32,            // 0.9 - Very hard working
    pub specialization_level: f32,   // 0.8 - Highly specialized
    pub innovation_resistance: f32, // 0.6 - Resists change
    pub production_focus: f32,     // 0.9 - Production focused
}

impl IndustriousCore {
    pub fn new() -> Self {
        Self {
            work_ethic: 0.9,
            specialization_level: 0.8,
            innovation_resistance: 0.6,
            production_focus: 0.9,
        }
    }
    
    pub fn calculate_production_efficiency(&self, resource_type: ResourceType) -> f32 {
        match resource_type {
            ResourceType::Scrap => 1.5, // Very efficient at scrap production
            ResourceType::Biomass => 0.7, // Less efficient at biomass
            ResourceType::Research => 0.5, // Poor at research
            ResourceType::Energy => 1.2, // Good at energy
        }
    }
    
    pub fn get_specialization_preference(&self) -> SpecializationType {
        SpecializationType::HeavyIndustry
    }
}
```

#### Teal - Serene/Ethereal

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SereneCore {
    pub tranquility_level: f32,     // 0.9 - Very tranquil
    pub mystery_level: f32,        // 0.8 - Mysterious
    pub isolation_preference: f32, // 0.7 - Prefers isolation
    pub spiritual_focus: f32,      // 0.8 - Spiritually focused
}

impl SereneCore {
    pub fn new() -> Self {
        Self {
            tranquility_level: 0.9,
            mystery_level: 0.8,
            isolation_preference: 0.7,
            spiritual_focus: 0.8,
        }
    }
    
    pub fn calculate_visibility(&self) -> VisibilityLevel {
        VisibilityLevel {
            detection_chance: 0.3,  // Hard to detect
            tracking_difficulty: 0.8, // Hard to track
            communication_range: 0.4, // Limited communication
        }
    }
    
    pub fn get_interaction_preferences(&self) -> InteractionPreferences {
        InteractionPreferences {
            preferred_contact_time: TimeWindow::Night,
            communication_style: CommunicationStyle::Meditative,
            trade_frequency: TradeFrequency::Rare,
            diplomatic_approach: DiplomaticApproach::Mysterious,
        }
    }
}
```

#### Frost (Tundra) - Isolating/Cold

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolatingCore {
    pub coldness_level: f32,        // 0.9 - Very cold
    pub isolation_intensity: f32,   // 0.8 - Strongly isolating
    pub self_sufficiency: f32,      // 0.9 - Self-sufficient
    pub territorial_defense: f32,   // 0.8 - Strong defense
}

impl IsolatingCore {
    pub fn new() -> Self {
        Self {
            coldness_level: 0.9,
            isolation_intensity: 0.8,
            self_sufficiency: 0.9,
            territorial_defense: 0.8,
        }
    }
    
    pub fn calculate_terrain_effect(&self, adjacent_nodes: &[Culture]) -> TerrainEffect {
        let slow_effect = self.isolation_intensity * 0.5; // Slows adjacent nodes
        
        TerrainEffect {
            movement_modifier: 1.0 - slow_effect,
            production_modifier: 0.9, // Slightly reduces production
            diplomatic_modifier: 0.5, // Halves diplomatic effectiveness
            defense_modifier: 1.3, // Increases defense
        }
    }
    
    pub fn get_border_policy(&self) -> BorderPolicy {
        BorderPolicy {
            access_level: AccessLevel::Restricted,
            visa_requirements: vec![
                VisaRequirement::CulturalCompatibility,
                VisaRequirement::PurposeDeclaration,
                VisaRequirement::DurationLimit,
            ],
            enforcement_strength: EnforcementStrength::Maximum,
        }
    }
}
```

## Behavioral Integration

### Cultural Behavior Engine

```rust
pub struct CulturalBehaviorEngine {
    pub personality_cores: HashMap<Culture, PersonalityCore>,
    pub behavior_history: HashMap<Culture, Vec<BehaviorEvent>>,
    pub current_moods: HashMap<Culture, CulturalMood>,
}

impl CulturalBehaviorEngine {
    pub fn new() -> Self {
        let mut personality_cores = HashMap::new();
        
        // Initialize all personality cores
        personality_cores.insert(Culture::Ember, PersonalityCore::Dominant(DominantCore::new()));
        personality_cores.insert(Culture::Tide, PersonalityCore::Fluid(FluidCore::new()));
        personality_cores.insert(Culture::Gale, PersonalityCore::Erratic(ErraticCore::new()));
        personality_cores.insert(Culture::Orange, PersonalityCore::Analytical(AnalyticalCore::new()));
        personality_cores.insert(Culture::Marsh, PersonalityCore::Nurturing(NurturingCore::new()));
        personality_cores.insert(Culture::Crystal, PersonalityCore::Judgmental(JudgmentalCore::new()));
        personality_cores.insert(Culture::Amber, PersonalityCore::Industrious(IndustriousCore::new()));
        personality_cores.insert(Culture::Teal, PersonalityCore::Serene(SereneCore::new()));
        personality_cores.insert(Culture::Tundra, PersonalityCore::Isolating(IsolatingCore::new()));
        
        Self {
            personality_cores,
            behavior_history: HashMap::new(),
            current_moods: HashMap::new(),
        }
    }
    
    pub fn update_cultural_behavior(&mut self, context: &PlanetaryContext) {
        for (culture, core) in &self.personality_cores {
            let behavior = self.calculate_behavior(culture, core, context);
            self.record_behavior_event(*culture, behavior);
            self.update_cultural_mood(*culture, behavior);
        }
    }
    
    fn calculate_behavior(&self, culture: &Culture, core: &PersonalityCore, context: &PlanetaryContext) -> CulturalBehavior {
        match core {
            PersonalityCore::Dominant(dominant) => {
                let border_actions = self.calculate_dominant_borders(dominant, context);
                let economic_actions = self.calculate_dominant_economy(dominant, context);
                
                CulturalBehavior {
                    border_actions,
                    economic_actions,
                    diplomatic_actions: vec![DiplomaticAction::AssertDominance],
                    internal_policies: vec![InternalPolicy::Militarization],
                }
            },
            PersonalityCore::Fluid(fluid) => {
                let migration_actions = self.calculate_fluid_migration(fluid, context);
                let trade_actions = self.calculate_fluid_trade(fluid, context);
                
                CulturalBehavior {
                    border_actions: vec![BorderAction::OpenBorders],
                    economic_actions: trade_actions,
                    diplomatic_actions: vec![DiplomaticAction::FormAlliances],
                    internal_policies: vec![InternalPolicy::CulturalExchange],
                }
            },
            PersonalityCore::Erratic(erratic) => {
                let random_actions = self.calculate_erratic_behavior(erratic, context);
                
                CulturalBehavior {
                    border_actions: random_actions.border_actions,
                    economic_actions: random_actions.economic_actions,
                    diplomatic_actions: random_actions.diplomatic_actions,
                    internal_policies: random_actions.internal_policies,
                }
            },
            // ... other personality cores
            _ => CulturalBehavior::default(),
        }
    }
    
    fn update_cultural_mood(&mut self, culture: Culture, behavior: CulturalBehavior) {
        let current_mood = self.current_moods.entry(culture).or_insert(CulturalMood::Neutral);
        
        // Update mood based on behavior outcomes
        *current_mood = self.calculate_new_mood(*current_mood, behavior);
    }
}
```

## Economic Profiles

### Culture-Specific Economies

```rust
#[derive(Debug, Clone)]
pub struct EconomicProfile {
    pub primary_export: ResourceType,
    pub primary_import: ResourceType,
    pub trade_style: TradeStyle,
    pub price_tendency: PriceTendency,
    pub production_efficiency: HashMap<ResourceType, f32>,
    pub consumption_patterns: HashMap<ResourceType, f32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeStyle {
    Aggressive,      // High prices, expansionist
    Cooperative,     // Fair prices, collaborative
    Isolationist,    // Limited trade, self-sufficient
    Exploitative,    // Takes advantage of partners
    Generous,        // Gives favorable terms
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriceTendency {
    High,            // Charges premium prices
    Low,             // Undercuts competition
    Fair,            // Market-based pricing
    Variable,        // Prices fluctuate
    Fixed,           // Stable prices
}

impl Culture {
    pub fn get_economic_profile(&self) -> EconomicProfile {
        match self {
            Culture::Ember => EconomicProfile {
                primary_export: ResourceType::Scrap,
                primary_import: ResourceType::Biomass,
                trade_style: TradeStyle::Aggressive,
                price_tendency: PriceTendency::High,
                production_efficiency: hashmap! {
                    ResourceType::Scrap => 1.5,
                    ResourceType::Biomass => 0.5,
                    ResourceType::Energy => 0.8,
                },
                consumption_patterns: hashmap! {
                    ResourceType::Biomass => 1.2,
                    ResourceType::Scrap => 0.8,
                },
            },
            Culture::Tide => EconomicProfile {
                primary_export: ResourceType::Biomass,
                primary_import: ResourceType::Scrap,
                trade_style: TradeStyle::Cooperative,
                price_tendency: PriceTendency::Fair,
                production_efficiency: hashmap! {
                    ResourceType::Biomass => 1.4,
                    ResourceType::Scrap => 0.9,
                    ResourceType::Energy => 1.0,
                },
                consumption_patterns: hashmap! {
                    ResourceType::Scrap => 1.1,
                    ResourceType::Biomass => 0.9,
                },
            },
            Culture::Gale => EconomicProfile {
                primary_export: ResourceType::Research,
                primary_import: ResourceType::Energy,
                trade_style: TradeStyle::Variable,
                price_tendency: PriceTendency::Variable,
                production_efficiency: hashmap! {
                    ResourceType::Research => 1.3,
                    ResourceType::Energy => 0.7,
                    ResourceType::Scrap => 0.8,
                },
                consumption_patterns: hashmap! {
                    ResourceType::Energy => 1.3,
                    ResourceType::Research => 0.7,
                },
            },
            // ... other cultures
            _ => EconomicProfile::default(),
        }
    }
}
```

## Validation Criteria

### Behavioral Consistency

```rust
pub struct PersonalityValidation {
    pub behavior_consistency: f32,
    pub economic_alignment: f32,
    pub diplomatic_coherence: f32,
    pub cultural_distinctiveness: f32,
}

impl PersonalityValidation {
    pub fn validate_personality_system(&self) -> ValidationReport {
        ValidationReport {
            behaviors_consistent: self.behavior_consistency > 0.8,
            economics_aligned: self.economic_alignment > 0.7,
            diplomacy_coherent: self.diplomatic_coherence > 0.7,
            cultures_distinct: self.cultural_distinctiveness > 0.8,
            overall_valid: self.behavior_consistency > 0.8 &&
                           self.economic_alignment > 0.7 &&
                           self.diplomatic_coherence > 0.7 &&
                           self.cultural_distinctiveness > 0.8,
        }
    }
}
```

The 9-Point Personality Cores System creates a living, breathing planetary web where each culture behaves according to its core nature, creating complex relationships, economic patterns, and diplomatic challenges that evolve dynamically based on the Shepherd's actions and planetary events.
