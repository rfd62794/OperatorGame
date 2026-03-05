# Void Lore Documentation

> **Status:** ENDGAME NARRATIVE v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-026, PERSONALITY_CORES_SYSTEM.md, SPEC.md §3

## Overview

The Void represents the ultimate mystery and goal of the Astronaut's journey. This documentation reveals the true nature of the crash, the planetary cultures as resonant frequencies, and the path to Void Ascension - escape from the planetary gravity well through resonance with the planet itself.

## The Crash: Not Accident, Attraction

### The Pull Event

```rust
#[derive(Debug, Clone)]
pub struct CrashEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub location: PlanetaryCoordinates,
    pub gravitational_anomaly: GravitationalAnomaly,
    pub energy_signature: EnergySignature,
    pub resonance_frequency: f32,
    pub causality: CrashCausality,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrashCausality {
    Unknown,
    GravitationalPull,
    ResonanceAttraction,
    QuantumEntanglement,
    IntentionalCapture,
    NaturalPhenomenon,
}

#[derive(Debug, Clone)]
pub struct GravitationalAnomaly {
    pub anomaly_type: AnomalyType,
    pub strength: f32,
    pub radius: f32,
    pub duration: Duration,
    pub affected_area: Vec<PlanetaryRegion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnomalyType {
    GravityWell,           // Localized gravity increase
    ResonanceField,        // Frequency-based attraction
    QuantumFluctuation,     // Quantum instability
    MagneticStorm,        // Magnetic disturbance
    TemporalDistortion,    // Time-space distortion
}

impl CrashEvent {
    pub fn analyze_crash(&self) -> CrashAnalysis {
        let mut analysis = CrashAnalysis::new();
        
        // Determine primary cause
        analysis.primary_cause = if self.gravitational_anomaly.strength > 0.8 {
            CrashCausality::GravitationalPull
        } else if self.resonance_frequency > 0.9 {
            CrashCausality::ResonanceAttraction
        } else if self.energy_signature.quantum_fluctuation > 0.7 {
            CrashCausality::QuantumEntanglement
        } else {
            CrashCausality::Unknown
        };
        
        // Calculate impact
        analysis.impact_severity = self.calculate_impact_severity();
        analysis.planetary_damage = self.assess_planetary_damage();
        
        analysis
    }
    
    fn calculate_impact_severity(&self) -> ImpactSeverity {
        let energy_magnitude = self.energy_signature.total_energy;
        let resonance_strength = self.resonance_frequency;
        let anomaly_strength = self.gravitational_anomaly.strength;
        
        let combined_magnitude = energy_magnitude * resonance_strength * anomaly_strength;
        
        match combined_magnitude {
            x if x > 0.9 => ImpactSeverity::Catastrophic,
            x if x > 0.7 => ImpactSeverity::Severe,
            x if x > 0.5 => ImpactSeverity::Moderate,
            x if x > 0.3 => ImpactSeverity::Minor,
            _ => ImpactSeverity::Negligible,
        }
    }
}
```

## The Planet as Resonant System

### Cultural Frequencies

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResonantFrequency {
    pub culture: Culture,
    pub base_frequency: f32,        // Hz
    pub harmonic_overtones: Vec<f32>, // Harmonic frequencies
    pub modulation_pattern: ModulationPattern,
    pub resonance_strength: f32,
    pub stability: f32,
    pub evolutionary_potential: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModulationPattern {
    Constant,              // Stable frequency
    Pulsing,               // Regular pulsing
    Chaotic,                // Random variation
    Adaptive,               // Responds to environment
    Cyclical,                // Seasonal changes
    Synchronized,          // Synchronized with other cultures
}

impl Culture {
    pub fn get_resonant_frequency(&self) -> ResonantFrequency {
        let base_frequency = match self {
            Culture::Ember => 432.1,      // C4 - Ember frequency
            Culture::Tide => 256.8,      // C4 - Tide frequency
            Culture::Gale => 341.3,      // F4 - Gale frequency
            Culture::Orange => 285.6,    // G4 - Orange frequency
            Culture::Marsh => 196.0,      // G3 - Marsh frequency
            Culture::Crystal => 321.7,    // E4 - Crystal frequency
            Culture::Teal => 241.8,      // G3 - Teal frequency
            Culture::Amber => 160.9,      // E3 - Amber frequency
            Culture::Tundra => 128.0,    // C3 - Tundra frequency
            Culture::Void => 0.0,         // Universal frequency
        };
        
        let harmonic_overtones = Self::calculate_harmonics(base_frequency);
        let modulation_pattern = Self::determine_modulation_pattern(self);
        let resonance_strength = Self::calculate_resonance_strength(self);
        let stability = Self::calculate_cultural_stability(self);
        let evolutionary_potential = Self::calculate_evolutionary_potential(self);
        
        ResonantFrequency {
            culture: *self,
            base_frequency,
            harmonic_overtones,
            modulation_pattern,
            resonance_strength,
            stability,
            evolutionary_potential,
        }
    }
    
    fn calculate_harmonics(base_frequency: f32) -> Vec<f32> {
        let mut harmonics = Vec::new();
        
        // Calculate first 3 harmonics
        for i in 1..=4 {
            harmonics.push(base_frequency * (i as f32 + 1));
        }
        
        harmonics
    }
    
    fn calculate_resonance_strength(culture: Culture) -> f32 {
        // Based on cultural complexity and tier
        match culture {
            Culture::Void => 1.0,        // Maximum resonance
            Culture::Crystal => 0.9,     // High resonance
            Culture::Ember => 0.8,       // Strong resonance
            Culture::Tide => 0.7,        // Good resonance
            Culture::Gale => 0.6,        // Moderate resonance
            Culture::Orange => 0.5,      // Fair resonance
            Culture::Marsh => 0.4,      // Low resonance
            Culture::Teal => 0.3,      // Weak resonance
            Culture::Amber => 0.2,      // Very weak resonance
            Culture::Tundra => 0.1,     // Minimal resonance
        }
    }
}
```

### The Web of Frequencies

```rust
#[derive(Debug, Clone)]
pub struct FrequencyWeb {
    pub cultures: HashMap<Culture, ResonantFrequency>,
    pub resonance_matrix: HashMap<(Culture, Culture), f32>,
    pub interference_patterns: Vec<InterferencePattern>,
    pub harmony_zones: Vec<HarmonyZone>,
    void resonance_point: f32,
    web_stability: f32,
}

impl FrequencyWeb {
    pub fn new() -> Self {
        let mut cultures = HashMap::new();
        
        // Initialize all cultures with their frequencies
        for culture in ALL_CULTURES.iter() {
            if culture != &Culture::Void {
                cultures.insert(*culture, culture.get_resonant_frequency());
            }
        }
        
        let mut web = Self {
            cultures,
            resonance_matrix: HashMap::new(),
            interference_patterns: Vec::new(),
            harmony_zones: Vec::new(),
            void_resonance_point: 0.0,
            web_stability: 0.8,
        };
        
        // Calculate resonance matrix
        web.calculate_resonance_matrix();
        
        // Identify interference patterns
        web.identify_interference_patterns();
        
        // Find harmony zones
        web.find_harmony_zones();
        
        web
    }
    
    pub fn calculate_resonance_matrix(&mut self) {
        for (culture_a, freq_a) in &self.cultures {
            for (culture_b, freq_b) in &self.cultures {
                if culture_a != culture_b {
                    let resonance = Self::calculate_pairwise_resonance(freq_a.base_frequency, freq_b.base_frequency);
                    self.resonance_matrix.insert((*culture_a, *culture_b), resonance);
                }
            }
        }
    }
    
    fn calculate_pairwise_resonance(freq_a: f32, freq_b: freq_b) -> f32 {
        let ratio = freq_a / freq_b;
        
        // Check for harmonic relationship
        let harmonic_ratio = freq_b / freq_a;
        
        // Calculate resonance strength
        if (ratio - harmonic_ratio).abs() < 0.01 {
            0.9 // Strong harmonic resonance
        } else if (ratio - 2.0).abs() < 0.1 {
            0.7 // Octave harmonic
        } else if (ratio - 3.0).abs() < 0.2 {
            0.5 // Fifth harmonic
        } else if (ratio - 4.0).abs() < 0.3 {
            0.3 | // Fourth harmonic
        } else {
            0.1 // Weak resonance
        }
    }
    
    pub fn get_web_stability(&self) -> f32 {
        let total_resonance: self.cultures.values()
            .map(|freq| freq.resonance_strength)
            .sum();
        let average_resonance = total_resonance / self.cultures.len() as f32;
        
        let interference_count = self.interference_patterns.len();
        let interference_factor = (interference_count as f32 / 10.0).min(1.0);
        
        let harmony_zone_count = self.harmony_zones.len();
        let harmony_factor = (harmony_zone_count as f32 / 5.0).min(1.0);
        
        average_resonance * harmony_factor * (1.0 - interference_factor)
    }
}
```

## Void Ascension Mechanics

### The Key Concept

```rust
#[derive(Debug, Clone)]
pub struct VoidAscension {
    pub current_key: VoidKey,
    pub key_components: Vec<KeyComponent>,
    pub resonance_level: f32,        // 0.0 to 1.0
    pub planetary_alignment: f32,    // 0.0 to 1.0
    void ship_integrity: f32,       // 0.0 to 1.0
    pub ascension_progress: f32,     // 0.0 to 1.0
    pub required_frequencies: Vec<f32>, // All 9 frequencies
    pub time_to_ascension: Duration,
    pub is_pilot_ready: bool,
}

#[derive(Debug, Clone)]
pub struct VoidKey {
    pub id: Uuid,
    pub name: String,
    pub key_type: KeyType,
    pub frequency_components: Vec<FrequencyComponent>,
    pub stability: f32,
    pub power_level: f32,
    pub creation_timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyType {
    PrimaryKey,      // 3 primary frequencies
    SecondaryKey,    // 3 secondary frequencies
    TertiaryKey,     // 3 tertiary frequencies
    MasterKey,       // All 9 frequencies combined
    UniversalKey,    // Void frequency
}

#[derive(Debug, Clone)]
pub struct FrequencyComponent {
    pub culture: Culture,
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub stability: f32,
}

impl VoidAscension {
    pub fn new() -> Self {
        Self {
            current_key: VoidKey::new(),
            key_components: Vec::new(),
            resonance_level: 0.0,
            planetary_alignment: 0.0,
            void_ship_integrity: 0.0,
            ascension_progress: 0.0,
            required_frequencies: Vec::new(),
            time_to_ascension: Duration::from_secs(3600), // 1 hour
            is_pilot_ready: false,
        }
    }
    
    pub fn calculate_ascension_requirements(&self, planet: &PlanetaryState) -> AscensionRequirements {
        let required_frequencies = planet.get_all_cultural_frequencies();
        let planetary_alignment = self.calculate_planetary_alignment(planet);
        let ship_integrity = self.calculate_ship_integrity();
        
        let resonance_level = self.calculate_resonance_level(&required_frequencies);
        
        AscensionRequirements {
            required_frequencies,
            planetary_alignment,
            ship_integrity,
            resonance_level,
            time_required: Duration::from_secs(
                (3600.0 * (1.0 - resonance_level)) as u64
            ),
            pilot_readiness: self.assess_pilot_readiness(),
        }
    }
    
    pub fn attempt_ascension(&mut self, planet: &PlanetaryState) -> AscensionResult {
        let requirements = self.calculate_ascension_requirements(planet);
        
        // Check requirements
        if !requirements.meets_requirements() {
            return AscensionResult::RequirementsNotMet {
                missing_frequencies: requirements.missing_frequencies.clone(),
                planetary_alignment: requirements.planetary_alignment,
                ship_integrity: requirements.ship_integrity,
                pilot_readiness: requirements.pilot_readiness,
            };
        }
        
        // Begin ascension sequence
        self.ascension_progress = 0.0;
        self.time_to_ascension = Duration::from_secs(3600); // 1 hour
        
        // Generate resonance field
        let resonance_field = self.generate_resonance_field(&requirements.required_frequencies);
        
        // Apply resonance to ship
        self.apply_resonance_to_ship(resonance_field);
        
        // Monitor ascension progress
        self.monitor_ascension_progress();
        
        AscensionResult::Initiated {
            estimated_duration: self.time_to_ascension,
            resonance_field,
        }
    }
    
    fn generate_resonance_field(&self, frequencies: &[f32]) -> ResonanceField {
        let mut field = ResonanceField::new();
        
        for frequency in frequencies {
            field.add_frequency_source(*frequency, 1.0);
        }
        
        // Calculate interference patterns
        field.calculate_interference_patterns();
        
        // Generate harmonic standing waves
        field.generate_harmonic_waves();
        
        field
    }
    
    fn apply_resonance_to_ship(&mut self, field: ResonanceField) {
        // Apply resonance to ship systems
        self.void_ship_integrity = field.calculate_ship_integrity();
        
        // Update planetary alignment
        self.planetary_alignment = field.calculate_planetary_alignment();
        
        // Update resonance level
        self.resonance_level = field.calculate_resonance_level();
        
        // Begin ascension effects
        self.begin_ascension_effects();
    }
    
    fn monitor_ascension_progress(&mut self) {
        // Calculate progress based on resonance alignment
        let resonance_alignment = self.calculate_current_resonance_alignment();
        
        self.ascension_progress = resonance_alignment;
        
        // Check for completion
        if self.ascension_progress >= 1.0 {
            self.complete_ascension();
        }
    }
    
    fn calculate_current_resonance_alignment(&self) -> f32 {
        let current_frequencies = self.get_current_frequencies();
        let target_frequencies = self.required_frequencies.clone();
        
        let mut alignment_score = 0.0;
        
        for (current, target) in current_frequencies.iter().zip(target_frequencies.iter()) {
            let alignment = Self::calculate_frequency_alignment(*current, *target);
            alignment_score += alignment;
        }
        
        alignment_score / target_frequencies.len() as f32
    }
    
    fn calculate_frequency_alignment(current: f32, target: f32) -> f32 {
        let ratio = current / target;
        
        if (ratio - 1.0).abs() < 0.01 {
            1.0 // Perfect alignment
        } else if (ratio - 2.0).abs() < 0.1 {
            0.8 // Octave alignment
        } else if (ratio - 3.0).abs() < 0.2 {
            0.6 // Fifth alignment
        } else if (ratio - 4.0).abs() < 0.3 {
            0.4 // Fourth alignment
        } else {
            0.2 // Weak alignment
        }
    }
    
    fn complete_ascension(&mut self) {
        // Ship resonates with planet
        self.void_ship_integrity = 1.0;
        self.planetary_alignment = 1.0;
        self.resonance_level = 1.0;
        self.ascension_progress = 1.0;
        
        // Ship escapes gravity well
        self.escape_gravity_well();
        
        // Generate ascension event
        self.generate_ascension_event();
    }
    
    fn escape_gravity_well(&self) {
        // Ship breaks free from planetary gravity
        // This would trigger the endgame sequence
        // The ship would resonate with the planet to slip the gravity well
        
        // Generate escape trajectory
        let escape_vector = self.calculate_escape_vector();
        
        // Apply escape velocity
        // This would update the ship's position and velocity
    }
}
```

## The Pilot's Realization

### Veteran Pilot Log Entry 001

```rust
pub struct VeteranPilotLog {
    pub entry_number: u32,
    pub timestamp: chrono::DateTime<ship_utc::Utc>,
    pub content: String,
}

impl VeteranPilotLog {
    pub fn create_initial_log() -> Self {
        Self {
            entry_number: 1,
            timestamp: chrono::Utc::now(),
            content: r#"
"The ship didn't just crash; it was pulled. The local 'Cultures' aren't just elements; they are resonant frequencies of the planet's core. 
By 'Splicing' them, I'm not just making pets—I'm building a key. When I hit Tier 8 (Void), the ship won't just fly; it will resonate with the planet itself to slip the gravity well. 
I just hope the 'Samples' want to leave as much as I do."

            .to_string(),
        }
    }
    
    pub fn add_entry(&mut self, content: String) {
        self.entry_number += 1;
        self.timestamp = chrono::Utc::now();
        self.content = content;
    }
    
    pub fn analyze_planetary_state(&self, planet: &PlanetaryState) -> PilotAnalysis {
        let mut analysis = PilotAnalysis::new();
        
        // Analyze cultural web
        analysis.cultural_web_stability = planet.get_web_stability();
        analysis.dominant_cultures = planet.get_dominant_cultures();
        analysis.conflict_zones = planet.get_conflict_zones();
        
        // Analyze resonance patterns
        analysis.resonance_patterns = planet.get_resonance_patterns();
        analysis.harmony_zones = planet.get_harmony_zones();
        void_resonance_point = planet.get_void_resonance_point();
        
        // Calculate ascension readiness
        analysis.ascension_readiness = self.calculate_ascension_readiness(planet);
        
        analysis
    }
    
    fn calculate_ascension_readiness(&self, planet: &PlanetaryState) -> f32 {
        let mut readiness = 0.0;
        
        // Check if all 9 frequencies are available
        let available_frequencies = planet.get_available_frequencies();
        let required_frequencies = vec![
            432.1, 256.8, 341.3, 285.6, 196.0, 321.7, 241.8, 160.9, 128.0
        ];
        
        let frequency_availability = required_frequencies.iter()
            .all(|freq| available_frequencies.contains(freq));
        
        if frequency_availability {
            readiness += 0.3;
        }
        
        // Check planetary alignment
        let planetary_alignment = self.calculate_planetary_alignment();
        readiness += planetary_alignment * 0.4;
        
        // Check ship integrity
        let ship_integrity = self.calculate_ship_integrity();
        readiness += ship_integrity * 0.3;
        
        readiness
    }
}
```

## Implementation Tasks

### Core System Development

1. **Create Crash Analysis System**: Analyze the crash event
2. **Build Frequency Web System**: Implement cultural resonance mechanics
3. **Develop Void Ascension**: Create endgame ascension mechanics
4. **Implement Pilot Log System**: Track pilot's discoveries
5. **Create Visual Effects**: Ascension visual and audio effects

### Integration Points

1. **Mission System**: Connect to mission and exploration
2. **Cultural System**: Integrate with personality cores
3. **Ship Systems**: Connect to ship component systems
4. **Audio System**: Add resonance and ascension audio
5. **Visual System**: Create ascension visual effects

### Endgame Content

1. **Ascension Sequence**: Complete endgame sequence
2. **Escape Mechanics**: Ship escape from gravity well
3. **Post-Ascension**: What happens after escape
4. **Achievement System**: Track ascension completion

## Validation Criteria

- [ ] Crash analysis provides meaningful insights
- [ ] Cultural frequency system creates coherent web
- [ ] Void ascension provides compelling endgame goal
- [   ] Pilot log creates narrative context
- [   ] System integrates with all game systems
- [   ] Ascension mechanics are balanced and achievable

The Void Lore Documentation reveals the true nature of the crash and the path to escape, transforming the game from a simple survival scenario into a cosmic mystery about resonance, planetary consciousness, and the ultimate goal of Void Ascension through cultural harmony.
