# Harmonic Resonance & Geometric Archetypes

> **Status:** ACOUSTIC ECOSYSTEM SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** RENDERED_MATH_VISUAL_IDENTITY.md, PERSONALITY_CORES_SYSTEM.md, SPEC.md §3

## Overview

The Harmonic Resonance system transforms the game's audio from electronic beeps into an organic harmonic ecosystem where each culture resonates at specific frequencies designed to soothe or stimulate the human psyche. This creates a "Vocal Logic" where the planet harmonizes with the Astronaut's hooting and hollering, establishing a deep acoustic connection between player and world.

## Sonic Palette Architecture

### The Healing Scale Framework

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SonicPalette {
    pub culture: Culture,
    pub base_frequency: f32,        // Hz
    pub harmonic_series: Vec<f32>,  // Harmonic overtones
    pub geometric_form: GeometricForm,
    pub sonic_archetype: SonicArchetype,
    pub human_feeling: HumanFeeling,
    pub wave_type: WaveType,
    pub timbre: TimbreProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeometricForm {
    Tetrahedron,      // 4 faces - Ember
    Octahedron,       // 8 faces - Gale
    Icosahedron,      // 20 faces - Tide
    Hexagon,          // 6 sides - Orange
    Torus,            // Donut shape - Marsh
    Dodecahedron,      // 12 faces - Crystal
    Cube,             // 6 faces - Amber
    StarFractal,      // Fractal star - Teal
    SpikedStar,       // Sharp star - Tundra
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SonicArchetype {
    DeepCello,         // Ember
    WindChimes,        // Gale
    TibetanBowl,       // Tide
    WoodenXylophone,   // Orange
    SoftRainfall,      // Marsh
    CrystalSingingBowl, // Crystal
    LowThroatSinging,  // Amber
    AeolianHarp,       // Teal
    CrackingIce,       // Tundra
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanFeeling {
    PrimalGrounding,   // Ember
    AiryFleeting,       // Gale
    FluidMeditative,    // Tide
    RhythmicStructural, // Orange
    NurturingDamp,      // Marsh
    PureSharp,         // Crystal
    DenseResonant,      // Amber
    SparklingDistant,   // Teal
    BrittleHighTension, // Tundra
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaveType {
    Sine,              // Pure tone
    Triangle,          // Harmonic rich
    Square,            // Complex harmonics
    Sawtooth,          // Bright, harsh (avoided)
    Pulse,             // Rhythmic
    Noise,             // Textural
    Complex,            // Multiple waves
    Organic,            // Natural variation
}

#[derive(Debug, Clone)]
pub struct TimbreProfile {
    pub fundamental: f32,
    pub overtones: Vec<(f32, f32)>, // (frequency, amplitude)
    pub attack_time: f32,           // Attack duration
    pub decay_time: f32,            // Decay duration
    pub sustain_level: f32,         // Sustain level
    pub release_time: f32,          // Release duration
    pub vibrato_rate: f32,          // Vibrato frequency
    pub vibrato_depth: f32,         // Vibrato amplitude
}
```

### Culture-Specific Sonic Profiles

```rust
impl Culture {
    pub fn get_sonic_palette(&self) -> SonicPalette {
        match self {
            Culture::Ember => SonicPalette {
                culture: Culture::Ember,
                base_frequency: 130.81,  // C3 - Deep, grounding
                harmonic_series: vec![261.63, 392.44, 523.25, 654.26], // C4, G4, C5, E5
                geometric_form: GeometricForm::Tetrahedron,
                sonic_archetype: SonicArchetype::DeepCello,
                human_feeling: HumanFeeling::PrimalGrounding,
                wave_type: WaveType::Sine,
                timbre: TimbreProfile {
                    fundamental: 130.81,
                    overtones: vec![
                        (261.63, 0.8),  // First octave
                        (392.44, 0.6),  // Fifth
                        (523.25, 0.4),  // Octave + fifth
                        (654.26, 0.3),  // Octave + major third
                    ],
                    attack_time: 0.1,
                    decay_time: 0.3,
                    sustain_level: 0.7,
                    release_time: 0.2,
                    vibrato_rate: 2.0,
                    vibrato_depth: 0.05,
                },
            },
            
            Culture::Gale => SonicPalette {
                culture: Culture::Gale,
                base_frequency: 329.63,  // E4 - Airy, energetic
                harmonic_series: vec![659.25, 987.77, 1318.51, 1648.14], // E5, B5, E6, G6
                geometric_form: GeometricForm::Octahedron,
                sonic_archetype: SonicArchetype::WindChimes,
                human_feeling: HumanFeeling::AiryFleeting,
                wave_type: WaveType::Triangle,
                timbre: TimbreProfile {
                    fundamental: 329.63,
                    overtones: vec![
                        (659.25, 0.6),  // First octave
                        (987.77, 0.4),  // Fifth
                        (1318.51, 0.3), // Octave + fifth
                        (1648.14, 0.2), // Octave + major third
                    ],
                    attack_time: 0.05,
                    decay_time: 0.2,
                    sustain_level: 0.5,
                    release_time: 0.15,
                    vibrato_rate: 4.0,
                    vibrato_depth: 0.03,
                },
            },
            
            Culture::Tide => SonicPalette {
                culture: Culture::Tide,
                base_frequency: 196.00,  // G3 - Fluid, meditative
                harmonic_series: vec![392.00, 588.00, 784.00, 980.00], // G4, D5, G5, B5
                geometric_form: GeometricForm::Icosahedron,
                sonic_archetype: SonicArchetype::TibetanBowl,
                human_feeling: HumanFeeling::FluidMeditative,
                wave_type: WaveType::Sine,
                timbre: TimbreProfile {
                    fundamental: 196.00,
                    overtones: vec![
                        (392.00, 0.7),  // First octave
                        (588.00, 0.5),  // Fifth
                        (784.00, 0.4),  // Octave + fifth
                        (980.00, 0.3),  // Octave + major third
                    ],
                    attack_time: 0.2,
                    decay_time: 0.4,
                    sustain_level: 0.8,
                    release_time: 0.3,
                    vibrato_rate: 1.0,
                    vibrato_depth: 0.02,
                },
            },
            
            Culture::Orange => SonicPalette {
                culture: Culture::Orange,
                base_frequency: 293.66,  // D4 - Rhythmic, structural
                harmonic_series: vec![587.33, 880.00, 1174.66, 1468.32], // D5, A5, D6, F6
                geometric_form: GeometricForm::Hexagon,
                sonic_archetype: SonicArchetype::WoodenXylophone,
                human_feeling: HumanFeeling::RhythmicStructural,
                wave_type: WaveType::Triangle,
                timbre: TimbreProfile {
                    fundamental: 293.66,
                    overtones: vec![
                        (587.33, 0.8),  // First octave
                        (880.00, 0.6),  // Fifth
                        (1174.66, 0.4), // Octave + fifth
                        (1468.32, 0.3), // Octave + major third
                    ],
                    attack_time: 0.02,
                    decay_time: 0.1,
                    sustain_level: 0.4,
                    release_time: 0.05,
                    vibrato_rate: 0.0,
                    vibrato_depth: 0.0,
                },
            },
            
            Culture::Marsh => SonicPalette {
                culture: Culture::Marsh,
                base_frequency: 174.61,  // F3 - Nurturing, damp
                harmonic_series: vec![349.23, 523.25, 698.46, 872.07], // F4, C5, F5, A5
                geometric_form: GeometricForm::Torus,
                sonic_archetype: SonicArchetype::SoftRainfall,
                human_feeling: HumanFeeling::NurturingDamp,
                wave_type: WaveType::Sine,
                timbre: TimbreProfile {
                    fundamental: 174.61,
                    overtones: vec![
                        (349.23, 0.6),  // First octave
                        (523.25, 0.4),  // Fifth
                        (698.46, 0.3),  // Octave + fifth
                        (872.07, 0.2),  // Octave + major third
                    ],
                    attack_time: 0.3,
                    decay_time: 0.5,
                    sustain_level: 0.9,
                    release_time: 0.4,
                    vibrato_rate: 0.5,
                    vibrato_depth: 0.01,
                },
            },
            
            Culture::Crystal => SonicPalette {
                culture: Culture::Crystal,
                base_frequency: 523.25,  // C5 - Pure, sharp
                harmonic_series: vec![1046.50, 1569.75, 2093.00, 2616.25], // C6, G6, C7, E7
                geometric_form: GeometricForm::Dodecahedron,
                sonic_archetype: SonicArchetype::CrystalSingingBowl,
                human_feeling: HumanFeeling::PureSharp,
                wave_type: WaveType::Sine,
                timbre: TimbreProfile {
                    fundamental: 523.25,
                    overtones: vec![
                        (1046.50, 0.9), // First octave
                        (1569.75, 0.7), // Fifth
                        (2093.00, 0.5), // Octave + fifth
                        (2616.25, 0.4), // Octave + major third
                    ],
                    attack_time: 0.1,
                    decay_time: 0.2,
                    sustain_level: 0.6,
                    release_time: 0.1,
                    vibrato_rate: 0.0,
                    vibrato_depth: 0.0,
                },
            },
            
            Culture::Amber => SonicPalette {
                culture: Culture::Amber,
                base_frequency: 110.00,  // A2 - Dense, resonant
                harmonic_series: vec![220.00, 330.00, 440.00, 550.00], // A3, E4, A4, C5
                geometric_form: GeometricForm::Cube,
                sonic_archetype: SonicArchetype::LowThroatSinging,
                human_feeling: HumanFeeling::DenseResonant,
                wave_type: WaveType::Complex,
                timbre: TimbreProfile {
                    fundamental: 110.00,
                    overtones: vec![
                        (220.00, 0.8),  // First octave
                        (330.00, 0.6),  // Fifth
                        (440.00, 0.5),  // Octave + fifth
                        (550.00, 0.4),  // Octave + major third
                    ],
                    attack_time: 0.4,
                    decay_time: 0.6,
                    sustain_level: 0.7,
                    release_time: 0.5,
                    vibrato_rate: 1.5,
                    vibrato_depth: 0.04,
                },
            },
            
            Culture::Teal => SonicPalette {
                culture: Culture::Teal,
                base_frequency: 440.00,  // A4 - Sparkling, distant
                harmonic_series: vec![880.00, 1320.00, 1760.00, 2200.00], // A5, E6, A7, C7
                geometric_form: GeometricForm::StarFractal,
                sonic_archetype: SonicArchetype::AeolianHarp,
                human_feeling: HumanFeeling::SparklingDistant,
                wave_type: WaveType::Organic,
                timbre: TimbreProfile {
                    fundamental: 440.00,
                    overtones: vec![
                        (880.00, 0.5),  // First octave
                        (1320.00, 0.3), // Fifth
                        (1760.00, 0.2), // Octave + fifth
                        (2200.00, 0.1), // Octave + major third
                    ],
                    attack_time: 0.1,
                    decay_time: 0.3,
                    sustain_level: 0.4,
                    release_time: 0.2,
                    vibrato_rate: 3.0,
                    vibrato_depth: 0.02,
                },
            },
            
            Culture::Tundra => SonicPalette {
                culture: Culture::Tundra,
                base_frequency: 783.99,  // G5 - Brittle, high-tension
                harmonic_series: vec![1567.98, 2351.97, 3135.96, 3919.95], // G6, D7, G8, B8
                geometric_form: GeometricForm::SpikedStar,
                sonic_archetype: SonicArchetype::CrackingIce,
                human_feeling: HumanFeeling::BrittleHighTension,
                wave_type: WaveType::Noise,
                timbre: TimbreProfile {
                    fundamental: 783.99,
                    overtones: vec![
                        (1567.98, 0.7), // First octave
                        (2351.97, 0.5), // Fifth
                        (3135.96, 0.4), // Octave + fifth
                        (3919.95, 0.3), // Octave + major third
                    ],
                    attack_time: 0.01,
                    decay_time: 0.05,
                    sustain_level: 0.2,
                    release_time: 0.02,
                    vibrato_rate: 0.0,
                    vibrato_depth: 0.0,
                },
            },
            
            Culture::Void => SonicPalette {
                culture: Culture::Void,
                base_frequency: 432.00,  // "Heart of Nature" frequency
                harmonic_series: vec![864.00, 1296.00, 1728.00, 2160.00], // A5, E6, A7, C7
                geometric_form: GeometricForm::Sphere, // Perfect form
                sonic_archetype: SonicArchetype::UniversalResonance,
                human_feeling: HumanFeeling::Transcendent,
                wave_type: WaveType::Complex,
                timbre: TimbreProfile {
                    fundamental: 432.00,
                    overtones: vec![
                        (864.00, 1.0), // First octave
                        (1296.00, 0.8), // Fifth
                        (1728.00, 0.6), // Octave + fifth
                        (2160.00, 0.4), // Octave + major third
                    ],
                    attack_time: 0.2,
                    decay_time: 0.4,
                    sustain_level: 0.8,
                    release_time: 0.3,
                    vibrato_rate: 0.0,
                    vibrato_depth: 0.0,
                },
            },
        }
    }
}
```

## Elder's Master Frequency System

### The Tuning Fork Mechanism

```rust
#[derive(Debug, Clone)]
pub struct ElderResonanceSystem {
    pub master_frequency: f32,        // 432 Hz
    pub transformation_frequency: f32,  // 528 Hz
    pub resonance_field: ResonanceField,
    pub harmonic_stabilization: HarmonicStabilization,
    pub planetary_alignment: PlanetaryAlignment,
    pub acoustic_ecosystem: AcousticEcosystem,
}

#[derive(Debug, Clone)]
pub struct ResonanceField {
    pub field_radius: f32,
    pub field_strength: f32,
    pub resonance_pattern: ResonancePattern,
    pub affected_cultures: Vec<Culture>,
    pub stabilization_duration: Duration,
    pub visual_cymatics: CymaticPattern,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResonancePattern {
    PerfectHarmony,      // All cultures in harmony
    DominantFrequency,   // One culture dominates
    DissonantConflict,    // Cultural conflict
    Transitional,         // Shifting patterns
    Ascending,           // Rising frequencies
    Descending,           // Lowering frequencies
}

#[derive(Debug, Clone)]
pub struct HarmonicStabilization {
    pub stabilization_strength: f32,
    pub affected_slimes: Vec<Uuid>,
    pub stabilization_duration: Duration,
    pub bonus_effects: Vec<StabilizationBonus>,
    pub visual_feedback: StabilizationFeedback,
}

impl ElderResonanceSystem {
    pub fn new() -> Self {
        Self {
            master_frequency: 432.0,  // Heart of Nature frequency
            transformation_frequency: 528.0,  // Transformation/Miracle frequency
            resonance_field: ResonanceField::new(),
            harmonic_stabilization: HarmonicStabilization::new(),
            planetary_alignment: PlanetaryAlignment::new(),
            acoustic_ecosystem: AcousticEcosystem::new(),
        }
    }
    
    pub fn generate_master_resonance(&mut self, quiet_state: bool) -> ResonanceResult {
        if quiet_state {
            // Perfect 432 Hz drone
            self.resonance_field = ResonanceField {
                field_radius: 1.0,
                field_strength: 1.0,
                resonance_pattern: ResonancePattern::PerfectHarmony,
                affected_cultures: ALL_CULTURES.to_vec(),
                stabilization_duration: Duration::MAX,
                visual_cymatics: CymaticPattern::new_perfect_sphere(),
            };
            
            ResonanceResult::PerfectHarmony {
                frequency: self.master_frequency,
                field_radius: self.resonance_field.field_radius,
                stabilization_duration: self.resonance_field.stabilization_duration,
                visual_pattern: self.resonance_field.visual_cymatics.clone(),
            }
        } else {
            // Astronaut hooting - 528 Hz pulse
            self.generate_transformation_pulse()
        }
    }
    
    fn generate_transformation_pulse(&mut self) -> ResonanceResult {
        self.resonance_field = ResonanceField {
            field_radius: 0.8,
            field_strength: 0.7,
            resonance_pattern: ResonancePattern::DominantFrequency,
            affected_cultures: vec![Culture::Void], // Void culture dominant
            stabilization_duration: Duration::from_secs(300), // 5 minutes
            visual_cymatics: CymaticPattern::new_transformation_pulse(),
        };
        
        // Apply harmonic stabilization
        self.harmonic_stabilization = HarmonicStabilization {
            stabilization_strength: 0.8,
            affected_slimes: self.get_nearby_slimes(),
            stabilization_duration: Duration::from_secs(300),
            bonus_effects: vec![
                StabilizationBonus::HarmonyBonus,
                StabilizationBonus::GrowthBonus,
                StabilizationBonus::EnergyBonus,
            ],
            visual_feedback: StabilizationFeedback::new(),
        };
        
        ResonanceResult::TransformationPulse {
            frequency: self.transformation_frequency,
            pulse_duration: self.resonance_field.stabilization_duration,
            affected_cultures: self.resonance_field.affected_cultures.clone(),
            stabilization_effects: self.harmonic_stabilization.clone(),
            visual_pattern: self.resonance_field.visual_cymatics.clone(),
        }
    }
    
    fn get_nearby_slimes(&self) -> Vec<Uuid> {
        // Get all slimes within resonance field
        let game_state = get_current_game_state();
        
        game_state.roster
            .iter()
            .filter(|slime| {
                let distance = self.calculate_distance_from_elder(&slime.position);
                distance <= self.resonance_field.field_radius
            })
            .map(|slime| slime.id)
            .collect()
    }
    
    fn calculate_distance_from_elder(&self, slime_position: &egui::Vec2) -> f32 {
        let elder_position = egui::Vec2::new(0.5, 0.5); // Center of meadow
        slime_position.distance(elder_position)
    }
    
    pub fn update_resonance_field(&mut self, delta_time: Duration) {
        // Update resonance field based on time
        let elapsed = delta_time.as_secs_f32();
        
        // Gradually decay transformation pulse
        if self.resonance_field.resonance_pattern == ResonancePattern::DominantFrequency {
            self.resonance_field.field_strength *= 0.99; // Gradual decay
            
            if self.resonance_field.field_strength < 0.1 {
                // Return to perfect harmony
                self.generate_master_resonance(true);
            }
        }
        
        // Update harmonic stabilization
        self.harmonic_stabilization.update(delta_time);
        
        // Update visual cymatics
        self.resonance_field.visual_cymatics.update(elapsed);
    }
    
    pub fn get_current_frequency(&self) -> f32 {
        match self.resonance_field.resonance_pattern {
            ResonancePattern::PerfectHarmony => self.master_frequency,
            ResonancePattern::DominantFrequency => self.transformation_frequency,
            _ => self.master_frequency,
        }
    }
}
```

## Geometry of Sound System

### Visual Cymatics Implementation

```rust
#[derive(Debug, Clone)]
pub struct CymaticSystem {
    pub active_patterns: Vec<CymaticPattern>,
    pub frequency_visualizer: FrequencyVisualizer,
    pub geometric_resonance: GeometricResonance,
    pub energy_visualization: EnergyVisualization,
    pub particle_system: CymaticParticleSystem,
}

#[derive(Debug, Clone)]
pub struct CymaticPattern {
    pub pattern_type: PatternType,
    pub frequency: f32,
    pub amplitude: f32,
    pub geometric_form: GeometricForm,
    pub complexity: u8,
    pub symmetry: SymmetryType,
    pub nodes: Vec<CymaticNode>,
    pub connections: Vec<CymaticConnection>,
    pub color_palette: CymaticColorPalette,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternType {
    PerfectSphere,       // Elder's perfect harmony
    StarPattern,         // Crystal's sharp patterns
    FlowerOfLife,        // Marsh's organic patterns
    HexagonalGrid,        // Orange's structural patterns
    ToroidalFlow,         // Tide's fluid patterns
    TetrahedralStructure,  // Ember's primal patterns
    OctahedralDance,      // Gale's airy patterns
    CubicFoundation,      // Amber's dense patterns
    FractalStar,          // Teal's ethereal patterns
    SpikedCrystal,        // Tundra's brittle patterns
    ChaoticInterference,  // Dissonant patterns
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymmetryType {
    Radial,              // Perfect circular symmetry
    Bilateral,           // Mirror symmetry
    Rotational,          // Rotational symmetry
    Translational,       // Repeating patterns
    Fractal,             // Self-similar patterns
    Asymmetric,          // No symmetry
}

#[derive(Debug, Clone)]
pub struct CymaticNode {
    pub position: egui::Vec2,
    pub radius: f32,
    pub intensity: f32,
    pub phase: f32,
    pub color: egui::Color32,
    pub vibration_frequency: f32,
}

impl CymaticSystem {
    pub fn new() -> Self {
        Self {
            active_patterns: Vec::new(),
            frequency_visualizer: FrequencyVisualizer::new(),
            geometric_resonance: GeometricResonance::new(),
            energy_visualization: EnergyVisualization::new(),
            particle_system: CymaticParticleSystem::new(),
        }
    }
    
    pub fn generate_pattern(&mut self, frequency: f32, culture: Culture, energy_level: f32) -> CymaticPattern {
        let sonic_palette = culture.get_sonic_palette();
        let pattern_type = self.determine_pattern_type(culture, energy_level);
        let geometric_form = sonic_palette.geometric_form;
        let complexity = self.calculate_complexity(frequency, energy_level);
        let symmetry = self.determine_symmetry(culture);
        
        let nodes = self.generate_cymatic_nodes(geometric_form, complexity, energy_level);
        let connections = self.generate_cymatic_connections(&nodes, geometric_form);
        let color_palette = self.generate_color_palette(culture, energy_level);
        
        CymaticPattern {
            pattern_type,
            frequency,
            amplitude: energy_level,
            geometric_form,
            complexity,
            symmetry,
            nodes,
            connections,
            color_palette,
        }
    }
    
    fn determine_pattern_type(&self, culture: Culture, energy_level: f32) -> PatternType {
        match (culture, energy_level) {
            (Culture::Void, _) => PatternType::PerfectSphere,
            (Culture::Crystal, _) => PatternType::StarPattern,
            (Culture::Marsh, _) => PatternType::FlowerOfLife,
            (Culture::Orange, _) => PatternType::HexagonalGrid,
            (Culture::Tide, _) => PatternType::ToroidalFlow,
            (Culture::Ember, _) => PatternType::TetrahedralStructure,
            (Culture::Gale, _) => PatternType::OctahedralDance,
            (Culture::Amber, _) => PatternType::CubicFoundation,
            (Culture::Teal, _) => PatternType::FractalStar,
            (Culture::Tundra, _) => PatternType::SpikedCrystal,
            (_, _) => PatternType::ChaoticInterference,
        }
    }
    
    fn calculate_complexity(&self, frequency: f32, energy_level: f32) -> u8 {
        let base_complexity = (frequency / 100.0).min(10.0) as u8;
        let energy_modifier = (energy_level * 5.0) as u8;
        
        (base_complexity + energy_modifier).min(20)
    }
    
    fn determine_symmetry(&self, culture: Culture) -> SymmetryType {
        match culture {
            Culture::Void => SymmetryType::Radial,
            Culture::Crystal => SymmetryType::Rotational,
            Culture::Marsh => SymmetryType::Fractal,
            Culture::Orange => SymmetryType::Translational,
            Culture::Tide => SymmetryType::Radial,
            Culture::Ember => SymmetryType::Bilateral,
            Culture::Gale => SymmetryType::Rotational,
            Culture::Amber => SymmetryType::Bilateral,
            Culture::Teal => SymmetryType::Fractal,
            Culture::Tundra => SymmetryType::Asymmetric,
        }
    }
    
    fn generate_cymatic_nodes(&self, geometric_form: GeometricForm, complexity: u8, energy_level: f32) -> Vec<CymaticNode> {
        let mut nodes = Vec::new();
        
        match geometric_form {
            GeometricForm::Tetrahedron => {
                // Generate 4 nodes for tetrahedron
                for i in 0..4 {
                    let angle = (i as f32 / 4.0) * std::f32::consts::TAU;
                    let radius = 0.2 + energy_level * 0.1;
                    
                    nodes.push(CymaticNode {
                        position: egui::Vec2::new(angle.cos(), angle.sin()),
                        radius,
                        intensity: energy_level,
                        phase: angle,
                        color: self.get_node_color(i, energy_level),
                        vibration_frequency: 432.0 + (i as f32 * 100.0),
                    });
                }
            },
            GeometricForm::Octahedron => {
                // Generate 8 nodes for octahedron
                for i in 0..8 {
                    let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
                    let radius = 0.15 + energy_level * 0.08;
                    
                    nodes.push(CymaticNode {
                        position: egui::Vec2::new(angle.cos(), angle.sin()),
                        radius,
                        intensity: energy_level * 0.8,
                        phase: angle,
                        color: self.get_node_color(i, energy_level),
                        vibration_frequency: 329.63 + (i as f32 * 50.0),
                    });
                }
            },
            GeometricForm::Icosahedron => {
                // Generate 20 nodes for icosahedron
                for i in 0..20 {
                    let angle = (i as f32 / 20.0) * std::f32::consts::TAU;
                    let radius = 0.1 + energy_level * 0.05;
                    
                    nodes.push(CymaticNode {
                        position: egui::Vec2::new(angle.cos(), angle.sin()),
                        radius,
                        intensity: energy_level * 0.9,
                        phase: angle,
                        color: self.get_node_color(i, energy_level),
                        vibration_frequency: 196.0 + (i as f32 * 25.0),
                    });
                }
            },
            // ... other geometric forms
            _ => {
                // Default circular pattern
                let node_count = (complexity as usize).min(12);
                for i in 0..node_count {
                    let angle = (i as f32 / node_count as f32) * std::f32::consts::TAU;
                    let radius = 0.2 + energy_level * 0.1;
                    
                    nodes.push(CymaticNode {
                        position: egui::Vec2::new(angle.cos(), angle.sin()),
                        radius,
                        intensity: energy_level,
                        phase: angle,
                        color: self.get_node_color(i, energy_level),
                        vibration_frequency: 440.0 + (i as f32 * 20.0),
                    });
                }
            },
        }
        
        nodes
    }
    
    fn generate_cymatic_connections(&self, nodes: &[CymaticNode], geometric_form: GeometricForm) -> Vec<CymaticConnection> {
        let mut connections = Vec::new();
        
        match geometric_form {
            GeometricForm::Tetrahedron => {
                // Connect all nodes to all other nodes
                for i in 0..nodes.len() {
                    for j in (i + 1)..nodes.len() {
                        connections.push(CymaticConnection {
                            from_node: i,
                            to_node: j,
                            strength: 0.5,
                            frequency: 432.0,
                            color: self.get_connection_color(432.0),
                        });
                    }
                }
            },
            GeometricForm::Octahedron => {
                // Connect adjacent nodes
                for i in 0..nodes.len() {
                    let next = (i + 1) % nodes.len();
                    connections.push(CymaticConnection {
                        from_node: i,
                        to_node: next,
                        strength: 0.7,
                        frequency: 329.63,
                        color: self.get_connection_color(329.63),
                    });
                }
            },
            // ... other geometric forms
            _ => {
                // Connect to nearest neighbors
                for i in 0..nodes.len() {
                    let nearest = self.find_nearest_nodes(i, nodes, 2);
                    for &neighbor in &nearest {
                        connections.push(CymaticConnection {
                            from_node: i,
                            to_node: neighbor,
                            strength: 0.6,
                            frequency: 440.0,
                            color: self.get_connection_color(440.0),
                        });
                    }
                }
            },
        }
        
        connections
    }
    
    fn find_nearest_nodes(&self, node_index: usize, nodes: &[CymaticNode], count: usize) -> Vec<usize> {
        let mut distances: Vec<(usize, f32)> = nodes.iter()
            .enumerate()
            .filter(|(i, _)| *i != node_index)
            .map(|(i, node)| {
                let distance = nodes[node_index].position.distance(node.position);
                (i, distance)
            })
            .collect();
        
        distances.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        
        distances.iter().take(count).map(|(i, _)| *i).collect()
    }
    
    fn get_node_color(&self, node_index: usize, energy_level: f32) -> egui::Color32 {
        let base_hue = (node_index as f32 / 12.0) * 360.0;
        let saturation = 0.8;
        let value = 0.5 + energy_level * 0.5;
        
        self.hsv_to_rgb(base_hue, saturation, value)
    }
    
    fn get_connection_color(&self, frequency: f32) -> egui::Color32 {
        let hue = (frequency / 1000.0) * 360.0;
        let saturation = 0.6;
        let value = 0.7;
        
        self.hsv_to_rgb(hue, saturation, value)
    }
    
    fn hsv_to_rgb(&self, h: f32, s: f32, v: f32) -> egui::Color32 {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        egui::Color32::from_rgb(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}
```

## Implementation Tasks

### Core System Development

1. **Create Sonic Palette System**: Implement culture-specific sonic profiles
2. **Build Elder Resonance System**: Create master frequency mechanics
3. **Develop Cymatic System**: Implement visual sound patterns
4. **Create Harmonic Mixing**: Implement breeding as harmonic intervals
5. **Build Audio Engine**: Create procedural audio generation

### Visual Implementation

1. **Cymatic Visualization**: Create visual sound patterns
2. **Geometric Resonance**: Implement shape-frequency relationships
3. **Energy Visualization**: Show energy levels visually
4. **Particle System**: Create cymatic particle effects
5. **Color Harmonics**: Implement color-frequency relationships

### Audio Implementation

1. **Wave Generation**: Generate organic waveforms
2. **Harmonic Synthesis**: Create harmonic overtones
3. **Timbre Modeling**: Implement realistic instrument sounds
4. **Spatial Audio**: Create 3D audio positioning
5. **Dynamic Mixing**: Create procedural sound mixing

## Validation Criteria

- [ ] Each culture has distinct sonic personality
- [ Elder's master frequency creates planetary harmony
- [ Cymatic patterns visualize sound accurately
- [ Harmonic mixing creates pleasant intervals
- [ Audio system maintains 60fps performance
- [ Visual-audio sync is precise and responsive

The Harmonic Resonance system creates a deep acoustic connection between the Astronaut and the planet, where every interaction produces harmonious responses that soothe and stimulate the human psyche, making the game world feel alive and responsive to the player's actions.
