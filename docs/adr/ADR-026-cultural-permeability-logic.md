# ADR-026: The "Cultural Permeability" Logic

**Status:** ACCEPTED | **Date:** 2026-03-04 | **Author:** Gemini (via PyPro SDD-Edition)

## Context

The existing node system treated all cultures as static entities with fixed relationships. To create a "Living Web" where cultures dynamically interact and evolve, we need to introduce cultural permeability - a system where each culture has varying levels of openness to foreign influence, affecting everything from breeding success to mission difficulty and trade relationships.

## Decision

Implement a Cultural Permeability system where each culture has an Openness float (0.0 to 1.0) that determines their behavior towards foreign slimes, trade partners, and diplomatic interactions. This creates dynamic relationships where the Shepherd must adapt their strategy based on current cultural moods.

## Architecture

### Openness Scale Definition

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalPermeability {
    pub openness: f32,              // 0.0 (Closed) to 1.0 (Open)
    pub personality_core: PersonalityCore,
    pub mood_modifier: f32,         // Temporary mood changes
    pub historical_bias: f32,       // Long-term cultural tendencies
    pub environmental_influence: f32, // Biome effects on openness
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalityCore {
    Dominant,      // Ember - Aggressive expansion
    Fluid,         // Tide - Social and adaptable
    Erratic,       // Gale - Unpredictable movement
    Analytical,    // Orange - Logical and methodical
    Nurturing,     // Marsh - Passive and welcoming
    Judgmental,    // Crystal - Rigid and exclusive
    Industrious,   // Amber - Focused and persistent
    Serene,        // Teal - Ethereal and mysterious
    Isolating,     // Frost - Cold and defensive
}
```

### Base Openness Values

```rust
impl Culture {
    pub fn base_openness(&self) -> f32 {
        match self {
            // High Openness Cultures (0.7-0.9)
            Culture::Tide => 0.9,     // Most open - fluid and social
            Culture::Marsh => 0.8,   // Very open - nurturing and passive
            
            // Medium Openness Cultures (0.4-0.6)
            Culture::Gale => 0.6,     // Moderately open - erratic but accepting
            Culture::Orange => 0.5,   // Neutral - analytical but not hostile
            Culture::Teal => 0.5,     // Neutral - serene but distant
            
            // Low Openness Cultures (0.1-0.3)
            Culture::Ember => 0.2,    // Very closed - dominant and aggressive
            Culture::Crystal => 0.1,  // Extremely closed - judgmental and rigid
            Culture::Amber => 0.3,    // Low openness - industrious but focused
            Culture::Tundra => 0.2,   // Very closed - isolating and cold
            
            // Exception
            Culture::Void => 0.5,     // Neutral - universal constant
        }
    }
    
    pub fn personality_core(&self) -> PersonalityCore {
        match self {
            Culture::Ember => PersonalityCore::Dominant,
            Culture::Tide => PersonalityCore::Fluid,
            Culture::Gale => PersonalityCore::Erratic,
            Culture::Orange => PersonalityCore::Analytical,
            Culture::Marsh => PersonalityCore::Nurturing,
            Culture::Crystal => PersonalityCore::Judgmental,
            Culture::Amber => PersonalityCore::Industrious,
            Culture::Teal => PersonalityCore::Serene,
            Culture::Tundra => PersonalityCore::Isolating,
            Culture::Void => PersonalityCore::Analytical, // Void is analytical
        }
    }
}
```

## Permeability Mechanics

### Dynamic Openness Calculation

```rust
impl CulturalPermeability {
    pub fn calculate_current_openness(&self, context: &PlanetaryContext) -> f32 {
        let base = self.base_openness();
        let mood = self.mood_modifier;
        let historical = self.historical_bias;
        let environmental = self.environmental_influence;
        let temporal = self.calculate_temporal_modifier(context);
        
        // Combine all factors with weights
        let openness = base * 0.4 + 
                       mood * 0.2 + 
                       historical * 0.2 + 
                       environmental * 0.1 + 
                       temporal * 0.1;
        
        openness.clamp(0.0, 1.0)
    }
    
    fn calculate_temporal_modifier(&self, context: &PlanetaryContext) -> f32 {
        let time_factor = context.current_time.hour() as f32 / 24.0;
        
        match self.personality_core {
            PersonalityCore::Dominant => {
                // More aggressive during "day" hours
                if time_factor > 0.3 && time_factor < 0.7 { 0.1 } else { -0.1 }
            },
            PersonalityCore::Fluid => {
                // More open during transitions (dawn/dusk)
                if (time_factor > 0.2 && time_factor < 0.3) || (time_factor > 0.7 && time_factor < 0.8) { 0.2 } else { 0.0 }
            },
            PersonalityCore::Erratic => {
                // Random fluctuations
                (context.current_time.minute() as f32 / 60.0 - 0.5) * 0.3
            },
            PersonalityCore::Nurturing => {
                // More open during "growth" periods
                if time_factor > 0.4 && time_factor < 0.6 { 0.15 } else { 0.0 }
            },
            _ => 0.0, // Others don't have strong time-based variations
        }
    }
    
    pub fn update_mood(&mut self, events: &[PlanetaryEvent]) {
        for event in events {
            match event.event_type {
                PlanetaryEventType::SuccessfulTrade { partner_culture, .. } => {
                    if self.is_friendly_towards(partner_culture) {
                        self.mood_modifier += 0.05;
                    }
                },
                PlanetaryEventType::BorderConflict { opponent_culture, .. } => {
                    if self.is_hostile_towards(opponent_culture) {
                        self.mood_modifier -= 0.1;
                    }
                },
                PlanetaryEventType::EnvironmentalStress => {
                    self.mood_modifier -= 0.05;
                },
                PlanetaryEventType::CulturalExchange => {
                    self.mood_modifier += 0.03;
                },
            }
        }
        
        // Mood naturally returns to baseline over time
        self.mood_modifier *= 0.95;
        self.mood_modifier = self.mood_modifier.clamp(-0.3, 0.3);
    }
}
```

### Breeding Permeability Effects

```rust
pub struct BreedingPermeability {
    pub base_cooldown: Duration,
    pub cross_culture_modifier: f32,
    pub success_rate_bonus: f32,
    pub mutation_chance_modifier: f32,
}

impl BreedingPermeability {
    pub fn calculate_breeding_effects(openness: f32, parent_cultures: (Culture, Culture)) -> BreedingPermeability {
        let is_cross_culture = parent_cultures.0 != parent_cultures.1;
        
        let base_cooldown = if is_cross_culture {
            Duration::from_secs((300.0 / openness) as u64) // Higher openness = shorter cooldown
        } else {
            Duration::from_secs(180) // Same culture breeding is always faster
        };
        
        let cross_culture_modifier = if is_cross_culture {
            openness // Openness directly affects success rate
        } else {
            1.0 // Same culture always normal success
        };
        
        let success_rate_bonus = if is_cross_culture {
            openness * 0.3 // Up to 30% bonus for high openness
        } else {
            0.1 // Small bonus for same culture
        };
        
        let mutation_chance_modifier = openness * 0.5; // Open cultures have more mutations
        
        BreedingPermeability {
            base_cooldown,
            cross_culture_modifier,
            success_rate_bonus,
            mutation_chance_modifier,
        }
    }
}
```

### Mission Permeability Effects

```rust
pub struct MissionPermeability {
    pub difficulty_modifier: f32,
    pub risk_adjustment: f32,
    pub reward_multiplier: f32,
    pub diplomatic_options: Vec<DiplomaticOption>,
}

impl MissionPermeability {
    pub fn calculate_mission_effects(
        node_openness: f32, 
        squad_cultures: &[Culture], 
        node_culture: Culture
    ) -> MissionPermeability {
        let foreign_penalty = self.calculate_foreign_penalty(squad_cultures, node_culture);
        let openness_bonus = node_openness;
        
        let difficulty_modifier = 1.0 + foreign_penalty - openness_bonus;
        let risk_adjustment = 1.0 + (foreign_penalty * 0.5) - (openness_bonus * 0.3);
        let reward_multiplier = 1.0 + (foreign_penalty * 0.2) + (openness_bonus * 0.1);
        
        let diplomatic_options = if openness_bonus > 0.7 {
            vec![
                DiplomaticOption::PeacefulResolution,
                DiplomaticOption::CulturalExchange,
                DiplomaticOption::TradeAgreement,
            ]
        } else if openness_bonus > 0.4 {
            vec![
                DiplomaticOption::NegotiatedSettlement,
                DiplomaticOption::LimitedCooperation,
            ]
        } else {
            vec![] // No diplomatic options for closed cultures
        };
        
        MissionPermeability {
            difficulty_modifier,
            risk_adjustment,
            reward_multiplier,
            diplomatic_options,
        }
    }
    
    fn calculate_foreign_penalty(&self, squad_cultures: &[Culture], node_culture: Culture) -> f32 {
        let foreign_count = squad_cultures.iter()
            .filter(|&&culture| culture != node_culture)
            .count();
        
        if foreign_count == 0 {
            0.0 // No penalty for same-culture squads
        } else {
            (foreign_count as f32 / squad_cultures.len() as f32) * 0.3 // Up to 30% penalty
        }
    }
}
```

## Trade Permeability Effects

### Barter Web Integration

```rust
pub struct TradePermeability {
    pub trade_willingness: f32,
    pub price_modifier: f32,
    pub trade_reputation: f32,
    pub available_contracts: Vec<TradeContract>,
}

impl TradePermeability {
    pub fn calculate_trade_effects(openness: f32, partner_culture: Culture) -> TradePermeability {
        let cultural_compatibility = self.calculate_cultural_compatibility(partner_culture);
        let trade_willingness = openness * cultural_compatibility;
        
        let price_modifier = if trade_willingness > 0.7 {
            0.8 // 20% discount for friendly trading
        } else if trade_willingness > 0.4 {
            1.0 // Normal prices
        } else {
            1.3 // 30% markup for reluctant trading
        };
        
        let available_contracts = self.generate_trade_contracts(trade_willingness, partner_culture);
        
        TradePermeability {
            trade_willingness,
            price_modifier,
            trade_reputation: cultural_compatibility,
            available_contracts,
        }
    }
    
    fn calculate_cultural_compatibility(&self, partner_culture: Culture) -> f32 {
        // Define cultural relationships
        match (self.personality_core, partner_culture.personality_core()) {
            (PersonalityCore::Fluid, PersonalityCore::Nurturing) => 1.2, // Tide + Marsh = excellent
            (PersonalityCore::Nurturing, PersonalityCore::Fluid) => 1.2,
            (PersonalityCore::Dominant, PersonalityCore::Judgmental) => 1.1, // Ember + Crystal = strong alliance
            (PersonalityCore::Judgmental, PersonalityCore::Dominant) => 1.1,
            (PersonalityCore::Analytical, PersonalityCore::Industrious) => 1.0, // Orange + Amber = good
            (PersonalityCore::Industrious, PersonalityCore::Analytical) => 1.0,
            (PersonalityCore::Dominant, PersonalityCore::Isolating) => 0.3, // Ember + Frost = poor
            (PersonalityCore::Isolating, PersonalityCore::Dominant) => 0.3,
            (PersonalityCore::Judgmental, PersonalityCore::Erratic) => 0.4, // Crystal + Gale = poor
            (PersonalityCore::Erratic, PersonalityCore::Judgmental) => 0.4,
            _ => 0.8, // Neutral relationships
        }
    }
    
    fn generate_trade_contracts(&self, willingness: f32, partner_culture: Culture) -> Vec<TradeContract> {
        let mut contracts = Vec::new();
        
        if willingness > 0.8 {
            // High willingness - many contracts
            contracts.push(TradeContract::ResourceExchange {
                offered: ResourceType::Biomass,
                requested: ResourceType::Scrap,
                amount: 100,
            });
            contracts.push(TradeContract::CulturalExchange {
                offered_slime_culture: self.culture,
                requested_slime_culture: partner_culture,
                duration: Duration::from_secs(3600),
            });
        } else if willingness > 0.5 {
            // Medium willingness - basic contracts
            contracts.push(TradeContract::ResourceExchange {
                offered: ResourceType::Scrap,
                requested: ResourceType::Biomass,
                amount: 50,
            });
        } else if willingness > 0.2 {
            // Low willingness - expensive contracts
            contracts.push(TradeContract::ResourceExchange {
                offered: ResourceType::Scrap,
                requested: ResourceType::Biomass,
                amount: 25,
            });
        }
        
        contracts
    }
}
```

## Elder Integration

### Market Report System

```rust
pub struct ElderConsultation {
    pub cultural_analysis: HashMap<Culture, CulturalAnalysis>,
    pub relationship_matrix: HashMap<(Culture, Culture), RelationshipStatus>,
    pub trade_opportunities: Vec<TradeOpportunity>,
    pub recommendations: Vec<ElderRecommendation>,
}

#[derive(Debug, Clone)]
pub struct CulturalAnalysis {
    pub culture: Culture,
    pub current_openness: f32,
    pub mood_trend: MoodTrend,
    pub recent_events: Vec<PlanetaryEvent>,
    pub forecast: CulturalForecast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoodTrend {
    Improving,
    Stable,
    Declining,
    Volatile,
}

#[derive(Debug, Clone)]
pub struct CulturalForecast {
    pub predicted_openness: f32,
    pub confidence: f32,
    pub key_factors: Vec<String>,
    pub time_horizon: Duration,
}

#[derive(Debug, Clone)]
pub struct ElderRecommendation {
    pub recommendation_type: RecommendationType,
    pub target_culture: Culture,
    pub reasoning: String,
    pub urgency: Urgency,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationType {
    AvoidContact,
    InitiateTrade,
    SendDiplomaticMission,
    PrepareForConflict,
    ExploitOpportunity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Urgency {
    Low,
    Medium,
    High,
    Critical,
}

impl ElderConsultation {
    pub fn generate_market_report(&self, planetary_context: &PlanetaryContext) -> String {
        let mut report = String::new();
        
        report.push_str("## Elder's Market Report\n\n");
        
        // Overall planetary mood
        let overall_mood = self.calculate_overall_mood();
        report.push_str(&format!("**Planetary Mood:** {:?}\n\n", overall_mood));
        
        // Cultural analysis
        report.push_str("### Cultural Analysis\n\n");
        for (culture, analysis) in &self.cultural_analysis {
            report.push_str(&format!("**{}:** {:.1}% openness ({:?})\n", 
                culture, analysis.current_openness * 100.0, analysis.mood_trend));
            
            if !analysis.recent_events.is_empty() {
                report.push_str("  Recent events: ");
                for event in &analysis.recent_events {
                    report.push_str(&format!("{}, ", event.event_type));
                }
                report.push_str("\n");
            }
            
            report.push_str(&format!("  Forecast: {:.1}% openness in {:?} (confidence: {:.1}%)\n\n",
                analysis.forecast.predicted_openness * 100.0,
                analysis.forecast.time_horizon,
                analysis.forecast.confidence * 100.0));
        }
        
        // Key recommendations
        report.push_str("### Recommendations\n\n");
        for recommendation in &self.recommendations {
            report.push_str(&format!("**{:?} - {:?}:** {}\n", 
                recommendation.urgency, recommendation.recommendation_type, recommendation.reasoning));
            report.push_str(&format!("  Expected outcome: {}\n\n", recommendation.expected_outcome));
        }
        
        // Trade opportunities
        if !self.trade_opportunities.is_empty() {
            report.push_str("### Trade Opportunities\n\n");
            for opportunity in &self.trade_opportunities {
                report.push_str(&format!("**{} ↔ {}:** {} for {} (profit margin: {:.1}%)\n",
                    opportunity.seller_culture, opportunity.buyer_culture,
                    opportunity.offered_resource, opportunity.requested_resource,
                    opportunity.profit_margin * 100.0));
            }
        }
        
        report
    }
    
    pub fn get_diplomatic_advice(&self, target_culture: Culture, action: DiplomaticAction) -> DiplomaticAdvice {
        let analysis = self.cultural_analysis.get(&target_culture)
            .expect("Culture analysis should exist");
        
        let success_probability = self.calculate_success_probability(target_culture, action);
        let risks = self.identify_risks(target_culture, action);
        let benefits = self.identify_benefits(target_culture, action);
        
        DiplomaticAdvice {
            recommended: success_probability > 0.6,
            success_probability,
            risks,
            benefits,
            alternative_actions: self.suggest_alternatives(target_culture, action),
        }
    }
}
```

## Implementation Tasks

### Core System Updates

1. **Update `src/world_map.rs`**: Add openness and personality_core fields to Node struct
2. **Implement Cultural Permeability**: Create permeability calculation system
3. **Add Dynamic Mood System**: Implement mood updates based on events
4. **Integrate Breeding Effects**: Apply permeability to breeding mechanics
5. **Update Mission System**: Add permeability effects to mission difficulty

### Elder Consultation UI

1. **Create Elder Window**: UI component for market reports and advice
2. **Implement Analysis System**: Generate cultural analysis and forecasts
3. **Add Recommendation Engine**: Provide actionable diplomatic advice
4. **Create Trade Opportunity Display**: Show profitable trade routes

### Balance and Testing

1. **Permeability Balance**: Ensure openness values create meaningful gameplay
2. **Elder Accuracy**: Test that elder advice is reliable and useful
3. **Trade Flow Validation**: Verify that barter web creates interesting decisions
4. **Cultural Dynamics**: Test that cultural relationships evolve naturally

## Consequences

### Positive
- **Dynamic World**: Cultures evolve and respond to player actions
- **Strategic Depth**: Shepherd must consider cultural moods in planning
- **Living Economy**: Trade relationships create meaningful decisions
- **Elder Value**: Elder becomes essential strategic advisor

### Negative
- **Increased Complexity**: More systems to understand and manage
- **Balance Challenges**: Dynamic relationships harder to balance
- **Information Overload**: More data for players to process

### Risks
- **Unpredictable Behavior**: Dynamic cultures may become too chaotic
- **Elder Dependency**: Players may rely too heavily on elder advice
- **Trade Imbalance**: Barter system may create economic exploits

## Validation Criteria

- [ ] All cultures have appropriate openness values and personality cores
- [ ] Cultural permeability affects breeding, missions, and trade meaningfully
- [ ] Elder provides accurate and useful advice
- [ ] Trade opportunities create interesting strategic decisions
- [ ] Cultural moods evolve based on player actions and events
- [ ] System remains balanced and predictable enough for strategic planning

The Cultural Permeability Logic transforms the static node system into a living, breathing planetary web where cultural relationships evolve dynamically, creating deep strategic opportunities for the Shepherd who can navigate these complex diplomatic waters.
