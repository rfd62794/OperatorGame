# Planetary Topology: The "Ripple" Map

> **Status:** WORLD STRUCTURE SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-025, ELDER_CONSULTATION_UI.md, SPEC.md §8

## Overview

The Ripple Map transforms the planetary topology into concentric rings expanding outward from the Hidden Meadow (the crash site), where the Sleeping Elder Void Slime serves as the central anchor. This creates a natural difficulty gradient and narrative progression where the Astronaut's "Intrusion" into the ecosystem is measured by distance from the center.

## Ring Architecture

### Concentric Ring System

```rust
#[derive(Debug, Clone)]
pub struct RippleMap {
    pub center: PlanetaryCenter,
    pub rings: Vec<PlanetaryRing>,
    pub ring_transitions: Vec<RingTransition>,
    pub environmental_gradient: EnvironmentalGradient,
    pub cultural_distribution: CulturalDistribution,
    pub difficulty_progression: DifficultyProgression,
}

#[derive(Debug, Clone)]
pub struct PlanetaryCenter {
    pub position: PlanetaryCoordinates,
    pub elder_slime: ElderSlime,
    pub crash_site: CrashSite,
    pub hidden_meadow: HiddenMeadow,
    pub safe_zone: SafeZone,
    pub resonance_field: ResonanceField,
}

#[derive(Debug, Clone)]
pub struct PlanetaryRing {
    pub ring_number: u8,              // 0-4
    pub ring_name: String,
    pub radius: f32,                 // Distance from center
    pub dominant_culture: Option<Culture>,
    pub ring_type: RingType,
    pub environmental_conditions: EnvironmentalConditions,
    pub difficulty_class: DifficultyClass,
    pub access_requirements: Vec<AccessRequirement>,
    pub special_properties: Vec<SpecialProperty>,
    pub resource_distribution: ResourceDistribution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RingType {
    Meadow,              // Ring 0 - Safe zone
    Heartland,           // Ring 1 - Primary cultures
    Wilds,              // Ring 2 - Secondary cultures
    Forbidden,          // Ring 3 - Tertiary cultures
    Horizon,           // Ring 4 - Void threshold
}

#[derive(Debug, Clone)]
pub struct EnvironmentalConditions {
    pub temperature: TemperatureRange,
    pub humidity: f32,
    pub atmospheric_pressure: f32,
    pub radiation_level: f32,
    pub bio_hazard_level: f32,
    pub weather_patterns: Vec<WeatherPattern>,
    pub seasonal_variations: Vec<SeasonalVariation>,
}

impl RippleMap {
    pub fn new() -> Self {
        let center = PlanetaryCenter::new();
        let rings = Self::generate_rings(&center);
        
        Self {
            center,
            rings,
            ring_transitions: Vec::new(),
            environmental_gradient: EnvironmentalGradient::new(),
            cultural_distribution: CulturalDistribution::new(),
            difficulty_progression: DifficultyProgression::new(),
        }
    }
    
    fn generate_rings(center: &PlanetaryCenter) -> Vec<PlanetaryRing> {
        vec![
            // Ring 0: The Meadow
            PlanetaryRing {
                ring_number: 0,
                ring_name: "The Hidden Meadow".to_string(),
                radius: 0.0,
                dominant_culture: None,
                ring_type: RingType::Meadow,
                environmental_conditions: EnvironmentalConditions::new_meadow(),
                difficulty_class: DifficultyClass::Trivial,
                access_requirements: vec![],
                special_properties: vec![
                    SpecialProperty::SafeZone,
                    SpecialProperty::BioSanctuary,
                    SpecialProperty::ElderProtection,
                ],
                resource_distribution: ResourceDistribution::new_meadow(),
            },
            
            // Ring 1: The Inner Rim (Primary Heartlands)
            PlanetaryRing {
                ring_number: 1,
                ring_name: "The Inner Rim".to_string(),
                radius: 1.0,
                dominant_culture: Some(Culture::Ember),
                ring_type: RingType::Heartland,
                environmental_conditions: EnvironmentalConditions::new_heartland(),
                difficulty_class: DifficultyClass::Easy,
                access_requirements: vec![
                    AccessRequirement::BasicSurvival,
                ],
                special_properties: vec![
                    SpecialProperty::HighStability,
                    SpecialProperty::LowRisk,
                    SpecialProperty::AbundantResources,
                ],
                resource_distribution: ResourceDistribution::new_primary(),
            },
            
            // Ring 2: The Wilds (Secondary Zones)
            PlanetaryRing {
                ring_number: 2,
                ring_name: "The Wilds".to_string(),
                radius: 2.0,
                dominant_culture: Some(Culture::Orange),
                ring_type: RingType::Wilds,
                environmental_conditions: EnvironmentalConditions::new_wilds(),
                difficulty_class: DifficultyClass::Moderate,
                access_requirements: vec![
                    AccessRequirement::EnvironmentalResistance,
                    AccessRequirement::BioHazardProtection,
                ],
                special_properties: vec![
                    SpecialProperty::BioHazards,
                    SpecialProperty::UnpredictableWeather,
                    SpecialProperty::RareResources,
                ],
                resource_distribution: ResourceDistribution::new_secondary(),
            },
            
            // Ring 3: The Forbidden (Tertiary Wastes)
            PlanetaryRing {
                ring_number: 3,
                ring_name: "The Forbidden".to_string(),
                radius: 3.0,
                dominant_culture: Some(Culture::Amber),
                ring_type: RingType::Forbidden,
                environmental_conditions: EnvironmentalConditions::new_forbidden(),
                difficulty_class: DifficultyClass::Hard,
                access_requirements: vec![
                    AccessRequirement::AdvancedProtection,
                    AccessRequirement::SpecializedEquipment,
                    AccessRequirement::CulturalAffinity,
                ],
                special_properties: vec![
                    SpecialProperty::AtmosphericStress,
                    SpecialProperty::ExtremeDC,
                    SpecialProperty::ExoticResources,
                ],
                resource_distribution: ResourceDistribution::new_tertiary(),
            },
            
            // Ring 4: The Horizon (Void Threshold)
            PlanetaryRing {
                ring_number: 4,
                ring_name: "The Horizon".to_string(),
                radius: 4.0,
                dominant_culture: Some(Culture::Void),
                ring_type: RingType::Horizon,
                environmental_conditions: EnvironmentalConditions::new_horizon(),
                difficulty_class: DifficultyClass::Extreme,
                access_requirements: vec![
                    AccessRequirement::VoidResonance,
                    AccessRequirement::CompleteCulturalKey,
                    AccessRequirement::ElderBlessing,
                ],
                special_properties: vec![
                    SpecialProperty::VoidThreshold,
                    SpecialProperty::EndgameZone,
                    SpecialProperty::AscensionPoint,
                ],
                resource_distribution: ResourceDistribution::new_void(),
            },
        ]
    }
}
```

### Ring-Specific Mechanics

#### Ring 0: The Hidden Meadow

```rust
impl PlanetaryRing {
    pub fn generate_meadow_mechanics(&self) -> MeadowMechanics {
        MeadowMechanics {
            safe_zone: SafeZone {
                radius: 0.5,
                protection_level: 1.0,
                elder_presence: true,
                bio_sanctuary: true,
            },
            garden_system: GardenSystem {
                auto_growth: true,
                elder_nurturing: true,
                bio_luminescence: true,
                peaceful_environment: true,
            },
            crash_site: CrashSite {
                debris_field: true,
                salvage_opportunities: true,
                historical_significance: true,
                repair_materials: true,
            },
            elder_interaction: ElderInteraction {
                daily_blessing: true,
                mentorship_available: true,
                wisdom_sharing: true,
                protection_granted: true,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeadowMechanics {
    pub safe_zone: SafeZone,
    pub garden_system: GardenSystem,
    pub crash_site: CrashSite,
    pub elder_interaction: ElderInteraction,
}

#[derive(Debug, Clone)]
pub struct SafeZone {
    pub radius: f32,
    pub protection_level: f32,
    pub elder_presence: bool,
    pub bio_sanctuary: bool,
}
```

#### Ring 1: Primary Heartlands

```rust
impl PlanetaryRing {
    pub fn generate_primary_mechanics(&self) -> PrimaryMechanics {
        PrimaryMechanics {
            cultural_heartlands: vec![
                CulturalHeartland {
                    culture: Culture::Ember,
                    territory: "Volcanic Core".to_string(),
                    resource_focus: vec![ResourceType::Scrap, ResourceType::Energy],
                    environmental_traits: vec![
                        EnvironmentalTrait::HighTemperature,
                        EnvironmentalTrait::VolcanicActivity,
                        EnvironmentalTrait::MineralRich,
                    ],
                },
                CulturalHeartland {
                    culture: Culture::Tide,
                    territory: "Aquatic Belt".to_string(),
                    resource_focus: vec![ResourceType::Biomass, ResourceType::Research],
                    environmental_traits: vec![
                        EnvironmentalTrait::HighHumidity,
                        EnvironmentalTrait::AquaticEnvironment,
                        EnvironmentalTrait::BioDiverse,
                    ],
                },
                CulturalHeartland {
                    culture: Culture::Gale,
                    territory: "Wind Plains".to_string(),
                    resource_focus: vec![ResourceType::Research, ResourceType::Energy],
                    environmental_traits: vec![
                        EnvironmentalTrait::HighWinds,
                        EnvironmentalTrait::AtmosphericInstability,
                        EnvironmentalTrait::EnergyRich,
                    ],
                },
            ],
            basic_scavenging: BasicScavenging {
                success_rate: 0.8,
                resource_yield: 1.2,
                risk_level: 0.2,
                time_required: Duration::from_secs(300), // 5 minutes
            },
            high_stability: StabilityMetrics {
                environmental_stability: 0.9,
                cultural_stability: 0.8,
                resource_stability: 0.7,
                time_stability: 0.8,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimaryMechanics {
    pub cultural_heartlands: Vec<CulturalHeartland>,
    pub basic_scavenging: BasicScavenging,
    pub high_stability: StabilityMetrics,
}

#[derive(Debug, Clone)]
pub struct CulturalHeartland {
    pub culture: Culture,
    pub territory: String,
    pub resource_focus: Vec<ResourceType>,
    pub environmental_traits: Vec<EnvironmentalTrait>,
}
```

#### Ring 2: The Wilds

```rust
impl PlanetaryRing {
    pub fn generate_wilds_mechanics(&self) -> WildsMechanics {
        WildsMechanics {
            secondary_zones: vec![
                SecondaryZone {
                    culture: Culture::Orange,
                    territory: "Industrial Wastes".to_string(),
                    resource_focus: vec![ResourceType::Scrap, ResourceType::Research],
                    environmental_traits: vec![
                        EnvironmentalTrait::ToxicAtmosphere,
                        EnvironmentalTrait::IndustrialPollution,
                        EnvironmentalTrait::HeavyMetalRich,
                    ],
                    bio_hazards: vec![
                        BioHazard::ToxicSpores,
                        BioHazard::CorrosiveAgents,
                        BioHazard::RadiationPockets,
                    ],
                },
                SecondaryZone {
                    culture: Culture::Marsh,
                    territory: "Toxic Swamps".to_string(),
                    resource_focus: vec![ResourceType::Biomass, ResourceType::Research],
                    environmental_traits: vec![
                        EnvironmentalTrait::HighHumidity,
                        EnvironmentalTrait::ToxicWater,
                        EnvironmentalTrait::DecompositionRich,
                    ],
                    bio_hazards: vec![
                        BioHazard::PathogenicBacteria,
                        BioHazard::AllergicReactions,
                        BioHazard::Neurotoxins,
                    ],
                },
                SecondaryZone {
                    culture: Culture::Crystal,
                    territory: "Crystal Fields".to_string(),
                    resource_focus: vec![ResourceType::Energy, ResourceType::Research],
                    environmental_traits: vec![
                        EnvironmentalTrait::CrystallineStructures,
                        EnvironmentalTrait::EnergyAnomalies,
                        EnvironmentalTrait::ResonantFields,
                    ],
                    bio_hazards: vec![
                        BioHazard::EnergyDischarge,
                        BioHazard::CrystallineGrowth,
                        BioHazard::ResonanceSickness,
                    ],
                },
            ],
            bio_hazard_system: BioHazardSystem {
                hazard_types: vec![
                    BioHazard::ToxicSpores,
                    BioHazard::CorrosiveAgents,
                    BioHazard::RadiationPockets,
                    BioHazard::PathogenicBacteria,
                    BioHazard::AllergicReactions,
                    BioHazard::Neurotoxins,
                    BioHazard::EnergyDischarge,
                    BioHazard::CrystallineGrowth,
                    BioHazard::ResonanceSickness,
                ],
                resistance_requirements: vec![
                    ResistanceType::Toxicity,
                    ResistanceType::Radiation,
                    ResistanceType::Energy,
                    ResistanceType::Biological,
                ],
                protection_equipment: vec![
                    EquipmentType::HazmatSuit,
                    EquipmentType::Respirator,
                    EquipmentType::RadiationShield,
                    EquipmentType::BioFilter,
                ],
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct WildsMechanics {
    pub secondary_zones: Vec<SecondaryZone>,
    pub bio_hazard_system: BioHazardSystem,
}

#[derive(Debug, Clone)]
pub struct SecondaryZone {
    pub culture: Culture,
    pub territory: String,
    pub resource_focus: Vec<ResourceType>,
    pub environmental_traits: Vec<EnvironmentalTrait>,
    pub bio_hazards: Vec<BioHazard>,
}
```

#### Ring 3: The Forbidden

```rust
impl PlanetaryRing {
    pub fn generate_forbidden_mechanics(&self) -> ForbiddenMechanics {
        ForbiddenMechanics {
            tertiary_wastes: vec![
                TertiaryWaste {
                    culture: Culture::Amber,
                    territory: "Industrial Graveyard".to_string(),
                    resource_focus: vec![ResourceType::Scrap, ResourceType::Energy],
                    environmental_traits: vec![
                        EnvironmentalTrait::ExtremePollution,
                        EnvironmentalTrait::StructuralCollapse,
                        EnvironmentalTrait::ToxicAtmosphere,
                    ],
                    atmospheric_stress: AtmosphericStress {
                        pressure: 0.3, // Very low pressure
                        oxygen_level: 0.1, // Very low oxygen
                        toxicity: 0.9, // High toxicity
                        temperature: 0.2, // Extreme temperature
                    },
                },
                TertiaryWaste {
                    culture: Culture::Teal,
                    territory: "Mist Shrouded Ruins".to_string(),
                    resource_focus: vec![ResourceType::Research, ResourceType::Energy],
                    environmental_traits: vec![
                        EnvironmentalTrait::MysteriousFog,
                        EnvironmentalTrait::TemporalDistortion,
                        EnvironmentalTrait::EnergyAnomalies,
                    ],
                    atmospheric_stress: AtmosphericStress {
                        pressure: 0.5,
                        oxygen_level: 0.3,
                        toxicity: 0.6,
                        temperature: 0.4,
                    },
                },
                TertiaryWaste {
                    culture: Culture::Tundra,
                    territory: "Frozen Wastes".to_string(),
                    resource_focus: vec![ResourceType::Energy, ResourceType::Research],
                    environmental_traits: vec![
                        EnvironmentalTrait::ExtremeCold,
                        EnvironmentalTrait::Permafrost,
                        EnvironmentalTrait::CryogenicAtmosphere,
                    ],
                    atmospheric_stress: AtmosphericStress {
                        pressure: 0.7,
                        oxygen_level: 0.4,
                        toxicity: 0.3,
                        temperature: 0.1, // Extreme cold
                    },
                },
            ],
            extreme_dc_system: ExtremeDCSystem {
                base_dc: 25, // Starting DC
                dc_modifiers: vec![
                    DCModifier::EnvironmentalStress,
                    DCModifier::AtmosphericPressure,
                    DCModifier::Toxicity,
                    DCModifier::Temperature,
                ],
                special_challenges: vec![
                    ChallengeType::Survival,
                    ChallengeType::Navigation,
                    ChallengeType::ResourceManagement,
                    ChallengeType::EnvironmentalResistance,
                ],
                failure_consequences: vec![
                    Consequence::EquipmentDamage,
                    Consequence::HealthLoss,
                    Consequence::MissionFailure,
                    Consequence::ResourceLoss,
                ],
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForbiddenMechanics {
    pub tertiary_wastes: Vec<TertiaryWaste>,
    pub extreme_dc_system: ExtremeDCSystem,
}

#[derive(Debug, Clone)]
pub struct TertiaryWaste {
    pub culture: Culture,
    pub territory: String,
    pub resource_focus: Vec<ResourceType>,
    pub environmental_traits: Vec<EnvironmentalTrait>,
    pub atmospheric_stress: AtmosphericStress,
}

#[derive(Debug, Clone)]
pub struct AtmosphericStress {
    pub pressure: f32,        // 0.0-1.0
    pub oxygen_level: f32,    // 0.0-1.0
    pub toxicity: f32,        // 0.0-1.0
    pub temperature: f32,     // 0.0-1.0
}
```

#### Ring 4: The Horizon

```rust
impl PlanetaryRing {
    pub fn generate_horizon_mechanics(&self) -> HorizonMechanics {
        HorizonMechanics {
            void_threshold: VoidThreshold {
                resonance_requirement: 1.0,
                cultural_key_required: true,
                elder_blessing_required: true,
                ascension_ready: false,
            },
            endgame_system: EndgameSystem {
                ascension_sequence: AscensionSequence {
                    phase_1: AscensionPhase {
                        name: "Resonance Alignment".to_string(),
                        duration: Duration::from_secs(1800), // 30 minutes
                        requirements: vec![
                            Requirement::CompleteCulturalKey,
                            Requirement::ElderBlessing,
                            Requirement::ShipIntegrity(0.8),
                        ],
                        effects: vec![
                            Effect::ResonanceFieldGeneration,
                            Effect::PlanetaryAlignment,
                            Effect::VoidActivation,
                        ],
                    },
                    phase_2: AscensionPhase {
                        name: "Gravity Well Escape".to_string(),
                        duration: Duration::from_secs(600), // 10 minutes
                        requirements: vec![
                            Requirement::ResonanceFieldStability,
                            Requirement::ShipSystemsReady,
                            Requirement::ElderCooperation,
                        ],
                        effects: vec![
                            Effect::GravityNeutralization,
                            Effect::PropulsionActivation,
                            Effect::EscapeTrajectoryCalculation,
                        ],
                    },
                    phase_3: AscensionPhase {
                        name: "Planetary Departure".to_string(),
                        duration: Duration::from_secs(300), // 5 minutes
                        requirements: vec![
                            Requirement::EscapeVelocity,
                            Requirement::NavigationReady,
                            Requirement::LifeSupportStable,
                        ],
                        effects: vec![
                            Effect::PlanetaryEscape,
                            Effect::SpaceTransition,
                            Effect::MissionComplete,
                        ],
                    },
                },
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct HorizonMechanics {
    pub void_threshold: VoidThreshold,
    pub endgame_system: EndgameSystem,
}

#[derive(Debug, Clone)]
pub struct VoidThreshold {
    pub resonance_requirement: f32,
    pub cultural_key_required: bool,
    pub elder_blessing_required: bool,
    pub ascension_ready: bool,
}
```

## Implementation Tasks

### Core System Development

1. **Create Ring System**: Implement concentric ring architecture
2. **Build Environmental System**: Create environmental conditions
3. **Develop Difficulty Progression**: Implement difficulty gradient
4. **Implement Cultural Distribution**: Balance culture placement
5. **Create Ring-Specific Mechanics**: Develop unique mechanics for each ring

### Visual Implementation

1. **Ring Visualization**: Create visual representation of rings
2. **Environmental Effects**: Visual feedback for environmental conditions
3. **Transition Effects**: Smooth transitions between rings
4. **Cultural Indicators**: Visual markers for cultural territories
5. **Difficulty Visualization**: Visual representation of difficulty

### Integration Points

1. **Mission System**: Connect ring mechanics to mission generation
2. **Character System**: Apply environmental effects to slimes
3. **Equipment System**: Implement ring-specific equipment requirements
4. **Audio System**: Add environmental audio effects
5. **UI Integration**: Display ring information in Command Deck

## Validation Criteria

- [ ] Ring system provides natural difficulty progression
- [ Environmental conditions create meaningful gameplay challenges
- [ Cultural distribution creates balanced gameplay
- [ Ring-specific mechanics provide unique experiences
- [ Difficulty progression feels natural and rewarding
- [ Endgame mechanics provide compelling conclusion

The Ripple Map creates a cohesive planetary topology where the Astronaut's journey from the crash site to the Void Threshold is a natural progression through increasingly challenging environments, with the Elder serving as the central hub that anchors the entire ecosystem.
