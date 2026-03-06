# ADR-025: The "Elder-Centric" Graph

**Status:** ACCEPTED | **Date:** 2026-03-04 | **Author:** Gemini (via PyPro SDD-Edition)

## Context

The current 15-node graph layout creates arbitrary tactical relationships without narrative cohesion. By placing the Sleeping Elder Void Slime at the center as Node 0, we create a "Living Totem" that anchors the entire 9-Point Chromatic Wheel. This transforms the world map from a tactical grid into a concentric ecosystem where distance from the center represents the Astronaut's "Intrusion" into the planetary ecosystem, with difficulty and culture tier naturally emerging from proximity to the Elder.

## Decision

Implement a radial graph layout centered on Node 0 (the Sleeping Elder), where distance from the center determines Difficulty Class (DC) and Culture Tier. The Elder acts as the "Living Interface" that provides contextual advantages and serves as the mentor figure for the Astronaut's journey.

## Architecture

### Radial Graph Structure

```rust
#[derive(Debug, Clone)]
pub struct RadialGraph {
    pub center_node: RadialNode,
    pub rings: Vec<RadialRing>,
    pub node_mapping: HashMap<Uuid, RadialNode>,
    pub culture_distribution: HashMap<Culture, Vec<RingPosition>>,
    pub difficulty_gradient: DifficultyGradient,
    pub elder_influence: ElderInfluence,
}

#[derive(Debug, Clone)]
pub struct RadialNode {
    pub id: Uuid,
    pub position: RadialPosition,
    pub node_type: NodeType,
    pub culture: Option<Culture>,
    pub difficulty_class: DifficultyClass,
    pub culture_tier: u8,
    pub distance_from_center: f32,
    pub angle: f32,
    pub connections: Vec<RadialConnection>,
    pub environmental_factors: EnvironmentalFactors,
    pub elder_blessing: ElderBlessing,
}

#[derive(Debug, Clone)]
pub struct RadialRing {
    pub ring_number: u8,
    pub radius: f32,
    pub dominant_culture: Option<Culture>,
    ring_type: RingType,
    pub base_difficulty: f32,
    pub environmental_modifiers: Vec<EnvironmentalModifier>,
    pub special_properties: Vec<SpecialProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Core,              // Node 0 - The Elder
    Primary,            // Rings 1-3 - Primary cultures
    Secondary,          // Rings 4-6 - Secondary cultures
    Tertiary,           // Rings 7-9 - Tertiary cultures
    VoidThreshold,      // Beyond ring 9 - Void threshold
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RingType {
    Heartland,           // Safe, low difficulty
    Frontier,           // Medium difficulty
    Forbidden,          // High difficulty
    Hazardous,           // Extreme difficulty
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DifficultyClass {
    Trivial,            // DC 5 or less
    Easy,               // DC 6-10
    Moderate,           // DC 11-15
    Hard,               // DC 16-20
    Extreme,           // DC 21+
    Impossible,         // DC 25+
}

#[derive(Debug, Clone)]
pub struct RadialPosition {
    pub distance: f32,    // Distance from center
    pub angle: f32,      // Angular position
    ring_number: u8,     // Ring number (0 for center)
    sector: Sector,        // Angular sector within ring
}
```

### Elder Integration

```rust
#[derive(Debug, Clone)]
pub struct ElderSlime {
    pub id: Uuid,
    pub base_slime: SlimeGenome,
    pub current_state: ElderState,
    pub energy_reserves: f32,
    pub influence_radius: f32,
    pub blessing_active: bool,
    pub blessing_duration: Duration,
    pub current_mood: ElderMood,
    pub accumulated_affection: f32,
    pub wisdom_level: u8,
    pub planetary_resonance: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElderState {
    Sleeping,            // Dormant, massive
    Waking,            // Beginning to stir
    Active,            // Fully awake
    Meditating,         // Providing guidance
    Blessing,          // Providing bonuses
    Communicating,       // Sharing wisdom
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElderMood {
    Content,             // Peaceful and calm
    Concerned,           // Worried about state
    Pleased,            // Pleased with progress
    Urgent,             // Time-sensitive information
    Wise,              // Deep understanding
}

impl ElderSlime {
    pub fn new(base_slime: SlimeGenome) -> Self {
        Self {
            id: base_slime.id,
            base_slime,
            current_state: ElderState::Sleeping,
            energy_reserves: 1.0,
            influence_radius: 0.8,
            blessing_active: false,
            blessing_duration: Duration::from_secs(0),
            current_mood: ElderMood::Content,
            accumulated_affection: 0.0,
            wisdom_level: 8,
            planetary_resonance: 1.0,
        }
    }
    
    pub fn wake_up(&mut self) {
        self.current_state = ElderState::Waking;
        self.energy_reserves = 0.5; // Uses energy to wake up
        self.current_mood = ElderMood::Content;
        
        // Begin blessing cycle
        self.blessing_active = true;
        self.blessing_duration = Duration::from_secs(300); // 5 minutes
        
        // Generate wake-up effect
        self.generate_wake_up_effect();
    }
    
    pub fn activate(&mut self) {
        self.current_state = ElderState::Active;
        self.energy_reserves = 0.8;
        self.influence_radius = 1.0;
        self.current_mood = ElderMood::Wise;
        
        // Generate activation effect
        self.generate_activation_effect();
    }
    
    pub fn provide_blessing(&mut self, ring_number: u8, culture: Culture) -> BlessingEffect {
        if self.energy_reserves < 0.2 {
            return BlessingEffect::InsufficientEnergy;
        }
        
        let blessing_strength = self.calculate_blessing_strength(ring_number, culture);
        let blessing_duration = self.calculate_blessing_duration(ring_number, culture);
        
        self.energy_reserves -= 0.1;
        self.accumulated_affection += 0.05;
        
        BlessingEffect {
            blessing_type: BlessingType::CulturalBonus,
            target_culture: culture,
            ring_number,
            strength: blessing_strength,
            duration: blessing_duration,
            effect: self.calculate_blessing_effect(culture),
        }
    }
    
    fn calculate_blessing_strength(&self, ring_number: u8, culture: Culture) -> f32 {
        let base_strength = 0.2;
        let culture_affinity = self.calculate_culture_affinity(culture);
        let ring_modifier = match ring_number {
            1 => 1.2,    // Close to Elder
            2 => 1.0,    // Primary ring
            3 => 0.8,    // Secondary ring
            4 => 0.6,    // Tertiary ring
            _ => 0.4,    // Outer rings
        };
        
        let distance_modifier = (1.0 - (ring_number as f32 / 9.0)).max(0.2);
        
        base_strength * culture_affinity * ring_modifier * distance_modifier
    }
    
    fn calculate_culture_affinity(&self, culture: Culture) -> f32 {
        // Elder has affinity for all cultures (Void nature)
        match culture {
            Culture::Void => 1.0,
            _ => 0.8, // High affinity for all non-Void cultures
        }
    }
    
    fn generate_wake_up_effect(&self) {
        // Visual: Massive awakening pulse
        // Audio: Deep resonant hum
        // Environmental: Energy ripple through garden
        // Audio: Low-frequency resonance throughout ship
    }
    
    fn generate_activation_effect(&self) {
        // Visual: Elder opens eyes, begins breathing
        // Audio: Harmonic chorus of all 9 frequencies
        // Environmental: Garden glows with bio-luminescence
        // Audio: Ascension humming begins softly
    }
    
    fn calculate_blessing_duration(&self, ring_number: u8, culture: Culture) -> Duration {
        let base_duration = Duration::from_secs(1800); // 30 minutes base
        
        let culture_modifier = match culture {
            Culture::Void => Duration::from_secs(3600), // Void blessings last longest
            Culture::Crystal => Duration::from_secs(2700), // Crystal blessings are potent
            Culture::Ember => Duration::from_secs(2400), // Ember blessings are powerful
            Culture::Tide => Duration::from_secs(1800), // Tide blessings are efficient
            _ => Duration::from_secs(1200), // Standard blessings
        };
        
        let ring_modifier = match ring_number {
            1 => 1.5,    // Close to Elder
            2 => 1.2,    // Primary ring
            3 => 1.0,    // Secondary ring
            4 => 0.8,    // Tertiary ring
            _ => 0.6,    // Outer rings
        };
        
        Duration::from_secs((base_duration * ring_modifier as u64)
    }
    
    fn calculate_blessing_effect(&self, culture: Culture) -> BlessingEffect {
        let base_effect = match culture {
            Culture::Void => BlessingType::UniversalBonus,
            Culture::Crystal => BlessingType::StructuralIntegrity,
            Culture::Ember => BlessingType::CombatBonus,
            Culture::Tide => Blessing::DiplomaticBonus,
            Culture::Gale => Blessing::SpeedBonus,
            Culture::Orange => Blessing::EngineeringBonus,
            Culture::Marsh => Blessing::GrowthBonus,
            Culture::Teal => Blessing::StealthBonus,
            Culture::Amber => Blessing::DurabilityBonus,
            Culture::Tundra => Blessing::SurvivalBonus,
        };
        
        BlessingEffect {
            blessing_type: base_effect,
            target_culture: culture,
            ring_number: 0, // Will be set by caller
            strength: 0.0, // Will be calculated by caller
            duration: Duration::from_secs(0), // Will be set by caller
            effect: self.calculate_blessing_effect(culture),
        }
    }
    
    fn calculate_blessing_effect(&self, culture: Culture) -> Vec<String> {
        let mut effects = Vec::new();
        
        match culture {
            Culture::Void => {
                effects.push("Universal bonus to all rolls".to_string());
                effects.push("Ship systems efficiency +20%".to_string());
                effects.push("Void resonance field stabilization".to_string());
            },
            Culture::Crystal => {
                effects.push("Structural integrity +15%".to_string());
                effects.push("Shield generation +25%".to_string());
                effects.push("Research speed +30%".to_string());
            },
            Culture::Ember => {
                effects.push("Combat damage +25%".to_string());
                effects.push("Mining efficiency +20%".to_string());
                effects.push("Heat resistance +15%".to_string());
            },
            Culture::Tide => {
                effects.push("Diplomatic success +20%".to_string());
                effects.push("Trade bonus +15%".to_string());
                effects.push("Water purification +10%".to_string());
            },
            Culture::Gale => {
                effects.push("Speed +25%".to_string());
                effects.push("Evasion bonus +20%".to_string());
                effects.push("Scouting efficiency +15%".to_string());
            },
            Culture::Orange => {
                effects.push("Engineering speed +30%".to_string());
                effects.push("Construction speed +25%".to_string());
                effects.push("Repair efficiency +20%".to_string());
            },
            Culture::Marsh => {
                effects.push("Growth rate +40%".to_string());
                effects.push("Bio-processing +35%".to_string());
                effects.push("Healing speed +25%".to_string());
            },
            Culture::Teal => {
                effects.push("Stealth regeneration +30%".to_string());
                effects.push("Environmental resistance +20%".to_string());
                effects.push("Stealth +20%".to_string());
            },
            Culture::Amber => {
                effects.push("Durability +35%".to_string());
                effects.push("Mining yield +25%".to_string());
                effects.push("Construction speed +20%".to_string());
            },
            Culture::Tundra => {
                effects.push("Thermal resistance +40%".to_string());
                effects.push("Cold resistance +35%".to_string());
                effects.push("Stealth regeneration +30%".to_string());
            },
        }
        
        effects
    }
}
```

## Ring-Based Difficulty System

### Difficulty Gradient Calculation

```rust
impl RadialGraph {
    pub fn calculate_difficulty_gradient(&self) -> DifficultyGradient {
        let mut gradient = DifficultyGradient::new();
        
        for ring in &self.rings {
            let ring_difficulty = ring.base_difficulty;
            let ring_position = ring.ring_number as f32 / 9.0;
            
            // Apply environmental modifiers
            let environmental_modifier = ring.environmental_modifiers
                .iter()
                .sum(|modifier| modifier.modifier_value)
                .max(0.0)
                .min(0.5);
            
            let adjusted_difficulty = ring_difficulty * (1.0 + environmental_modifier);
            
            gradient.add_point(ring_position, adjusted_difficulty);
        }
        
        // Apply Elder influence
        let elder_modifier = self.elder_influence.influence_radius;
        gradient.apply_elder_influence(elder_modifier);
        
        gradient
    }
    
    pub fn get_difficulty_at_position(&self, position: RadialPosition) -> DifficultyClass {
        if position.distance > 3.0 {
            DifficultyClass::Impossible
        } else if position.distance > 2.5 {
            DifficultyClass::Extreme
        } else if position.distance > 2.0 {
            DifficultyClass::Hard
        } else if position.distance > 1.5 {
            DifficultyClass::Moderate
        } else if position.distance > 1.0 {
            DifficultyClass::Easy
        } else {
            DifficultyClass::Trivial
        }
    }
    
    pub fn get_culture_tier_at_position(&self, position: RadialPosition) -> u8 {
        if position.distance > 3.0 {
            0 // No culture beyond outer ring
        } else if position.distance > 2.5 {
            3 // Tertiary tier
        } else if position.distance > 2.0 {
            2 // Secondary tier
        } else if position.distance > 1.0 {
            1 // Primary tier
        } else {
            0 // Core zone (Elder)
        }
    }
}
```

### Cultural Distribution

```rust
impl RadialGraph {
    pub fn distribute_cultures(&mut self) {
        let culture_distribution = Self::calculate_culture_distribution();
        
        for (ring_number, ring) in self.rings.iter_mut() {
            if let Some(dominant_culture) = culture_distribution.get(&ring.ring_number) {
                ring.dominant_culture = Some(*dominant_culture);
            }
            
            // Distribute cultures around the ring
            let positions = Self::generate_ring_positions(ring_number, ring.ring_number);
            
            for (i, position) in positions.iter().enumerate() {
                let culture = if i < culture_distribution.len() {
                    culture_distribution[i]
                } else {
                    culture_distribution[0] // Default to first culture
                };
                
                let participant = FeedParticipant {
                    id: Uuid::new_v4(),
                    entity_type: EntityType::Environment,
                    visual_data: VisualData::new(),
                    current_position: position,
                    target_position: None,
                    movement_path: Vec::new(),
                    current_action: ParticipantAction::Idle,
                    status: ParticipantStatus::Active,
                    health: 1.0,
                    energy: 1.0,
                };
                
                ring.participants.push(participant);
            }
        }
    }
    
    fn calculate_culture_distribution() -> HashMap<u8, Culture> {
        let mut distribution = HashMap::new();
        
        // Ring 1 (Inner) - Primary cultures
        distribution.insert(1, Culture::Ember);
        distribution.insert(2, Culture::Tide);
        distribution.insert(3, Culture::Gale);
        
        // Ring 2 (Middle) - Secondary cultures
        distribution.insert(4, Culture::Orange);
        distribution.insert(5, Culture::Marsh);
        distribution.insert(6, Culture::Crystal);
        
        // Ring 3 (Outer) - Tertiary cultures
        distribution.insert(7, Culture::Amber);
        distribution.insert(8, Culture::Teal);
        distribution.insert(9, Culture::Tundra);
        
        distribution
    }
    
    fn generate_ring_positions(ring_number: u8, total_positions: usize) -> Vec<RadialPosition> {
        let mut positions = Vec::new();
        
        for i in 0..total_positions {
            let angle = (i as f32 / total_positions as f32) * std::f32::consts::TAU;
            let radius = ring_number as f32 / 9.0;
            
            let position = RadialPosition {
                distance: radius,
                angle,
                ring_number,
                sector: Sector::from_angle(angle),
            };
            
            positions.push(position);
        }
        
        positions
    }
}
```

## Implementation Tasks

### Core System Development

1. **Create Radial Graph Layout**: Implement radial graph generation
2. **Implement Elder Slime**: Create massive Void slime entity
3. **Build Ring System**: Create ring-based difficulty system
4. **Develop Cultural Distribution**: Balance culture placement
5. **Implement Elder Blessing System**: Create blessing mechanics

### Integration Points

1. **World Map Integration**: Replace 15-node graph with radial layout
2. **UI Integration**: Add Elder to garden background
3. **Mission System**: Connect difficulty to mission generation
4. **Audio System**: Add Elder audio and sound effects
5. **Visual Effects**: Create Elder visual effects and animations

### Visual Implementation

1. **Elder Rendering**: Massive, multi-layer visual rendering
2. **Ring Visualization**: Concentric ring visualization
3. **Blessing Effects**: Visual feedback for Elder interactions
4. **Environmental Effects**: Garden-wide environmental effects
5. **Animation System**: Breathing and movement animations

## Validation Criteria

- [ ] Radial graph provides logical difficulty progression
- [ Elder serves as central hub for planetary interaction
- [ Ring-based difficulty creates natural difficulty scaling
- Cultural distribution creates balanced gameplay
- Elder blessings provide meaningful strategic advantages
- Visual system maintains performance with massive entity rendering

## Future Enhancements

1. **Dynamic Ring Evolution**: Rings can expand or contract based on game state
2. **Elder Evolution**: Elder grows and changes with planetary alignment
3.Environmental Adaptation: Rings respond to planetary changes
4. **Advanced Blessing**: Complex blessing combinations
5. **Multi-Elder Support**: Multiple Elder entities in different regions

The Elder-Centric Graph creates a cohesive, narrative-driven world map where the Sleeping Elder serves as the central hub that anchors the entire planetary ecosystem, providing meaningful progression through distance-based difficulty and cultural balance.
