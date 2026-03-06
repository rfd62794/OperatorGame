# The Terminal Specs

> **System Design Documents and Technical Specifications**

## Overview

The Terminal Specs provide the detailed technical foundation for the OPERATOR rpgCore system. These specifications ensure that the frequency-driven design is implemented with mathematical precision and maintains the harmonic integrity of the planetary ecosystem.

## Core System Architecture

### Frequency-Driven Engine

```rust
pub struct FrequencyEngine {
    pub base_frequency: f32,           // 432 Hz - Elder's master frequency
    pub cultural_frequencies: HashMap<Culture, f32>,
    pub harmonic_series: HashMap<Culture, Vec<f32>>,
    pub resonance_matrix: HashMap<(Culture, Culture), f32>,
    pub planetary_harmony: PlanetaryHarmony,
}

impl FrequencyEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            base_frequency: 432.0,
            cultural_frequencies: HashMap::new(),
            harmonic_series: HashMap::new(),
            resonance_matrix: HashMap::new(),
            planetary_harmony: PlanetaryHarmony::new(),
        };
        
        engine.initialize_cultural_frequencies();
        engine.calculate_harmonic_series();
        engine.build_resonance_matrix();
        
        engine
    }
    
    fn initialize_cultural_frequencies(&mut self) {
        self.cultural_frequencies = HashMap::from([
            (Culture::Ember, 174.0),    // UT2 - Root chakra
            (Culture::Gale, 285.0),      // D4 - Solar plexus
            (Culture::Tide, 396.0),      // G4 - Heart chakra
            (Culture::Orange, 417.0),    // G#4 - Transformation
            (Culture::Marsh, 528.0),    // C5 - Love frequency
            (Culture::Crystal, 639.0),  // E5 - Intuition
            (Culture::Amber, 741.0),    // F#5 - Awakening
            (Culture::Teal, 852.0),      // G5 - Return
            (Culture::Tundra, 963.0),   // B5 - Oneness
            (Culture::Void, 432.0),      // A4 - Heart of Nature
        ]);
    }
    
    fn calculate_harmonic_series(&mut self) {
        for (culture, base_freq) in &self.cultural_frequencies {
            let mut harmonics = Vec::new();
            
            // Generate first 4 harmonics (fundamental, octave, fifth, major third)
            harmonics.push(*base_freq);                    // Fundamental
            harmonics.push(base_freq * 2.0);               // Octave
            harmonics.push(base_freq * 1.5);               // Perfect fifth
            harmonics.push(base_freq * 1.25);              // Major third
            
            // Add higher harmonics for complexity
            harmonics.push(base_freq * 2.5);               // Fifth above octave
            harmonics.push(base_freq * 3.0);               // Second octave
            harmonics.push(base_freq * 3.75);              // Major third above second octave
            
            self.harmonic_series.insert(*culture, harmonics);
        }
    }
    
    fn build_resonance_matrix(&mut self) {
        for (culture_a, freq_a) in &self.cultural_frequencies {
            for (culture_b, freq_b) in &self.cultural_frequencies {
                if culture_a != culture_b {
                    let ratio = freq_b / freq_a;
                    let resonance = Self::calculate_resonance_strength(ratio);
                    self.resonance_matrix.insert((*culture_a, *culture_b), resonance);
                }
            }
        }
    }
    
    fn calculate_resonance_strength(ratio: f32) -> f32 {
        // Check for harmonic relationships
        if (ratio - 2.0).abs() < 0.01 {  // Octave
            0.9
        } else if (ratio - 1.5).abs() < 0.01 {  // Perfect fifth
            0.8
        } else if (ratio - 1.25).abs() < 0.01 { // Major third
            0.7
        } else if (ratio - 1.5).abs() < 0.05 {  // Fifth tolerance
            0.6
        } else if (ratio - 1.25).abs() < 0.05 { // Major third tolerance
            0.5
        } else if (ratio - 2.0).abs() < 0.1 {  // Octave tolerance
            0.4
        } else {
            0.1 // Weak resonance
        }
    }
}
```

### Cymatic Rendering Pipeline

```rust
pub struct CymaticRenderer {
    pub frequency_table: FrequencyTable,
    pub geometry_generator: GeometryGenerator,
    pub visualizer: CymaticVisualizer,
    pub particle_system: CymaticParticleSystem,
}

pub struct FrequencyTable {
    pub frequency_to_vertices: HashMap<f32, Vec<egui::Vec2>>,
    pub frequency_to_edges: HashMap<f32, Vec<(usize, usize)>>,
    pub frequency_to_colors: HashMap<f32, egui::Color32>,
    pub complexity_cache: HashMap<f32, GeometryComplexity>,
}

impl CymaticRenderer {
    pub fn new() -> Self {
        Self {
            frequency_table: FrequencyTable::new(),
            geometry_generator: GeometryGenerator::new(),
            visualizer: CymaticVisualizer::new(),
            particle_system: CymaticParticleSystem::new(),
        }
    }
    
    pub fn generate_cymatic_pattern(&mut self, frequency: f32, energy_level: f32) -> CymaticPattern {
        let vertices = self.frequency_table.get_vertices(frequency);
        let edges = self.frequency_table.get_edges(frequency);
        let color = self.frequency_table.get_color(frequency);
        let complexity = self.frequency_table.get_complexity(frequency);
        
        CymaticPattern {
            vertices,
            edges,
            color,
            complexity,
            energy_level,
            animation_phase: 0.0,
            particle_effects: Vec::new(),
        }
    }
    
    pub fn render_slime_cymatics(&mut self, ctx: &egui::Context, slime: &SlimeGenome, position: egui::Pos2, time: f32) {
        let frequency = slime.culture.get_frequency();
        let energy_level = slime.energy_level;
        
        let pattern = self.generate_cymatic_pattern(frequency, energy_level);
        
        // Update animation
        pattern.animation_phase = time * frequency / 100.0;
        
        // Render cymatic pattern
        self.visualizer.render_pattern(ctx, pattern, position, time);
        
        // Render particle effects
        self.particle_system.update_particles(ctx, pattern, position, time);
    }
}

impl FrequencyTable {
    pub fn new() -> Self {
        let mut table = Self {
            frequency_to_vertices: HashMap::new(),
            frequency_to_edges: HashMap::new(),
            frequency_to_colors: HashMap::new(),
            complexity_cache: HashMap::new(),
        };
        
        table.initialize_frequency_mappings();
        table
    }
    
    fn initialize_frequency_mappings(&mut self) {
        let frequencies = vec![
            174.0, 285.0, 396.0, 417.0, 528.0, 639.0, 741.0, 852.0, 963.0, 432.0
        ];
        
        for frequency in frequencies {
            let vertices = self.generate_vertices(frequency);
            let edges = self.generate_edges(&vertices);
            let color = self.generate_color(frequency);
            let complexity = self.calculate_complexity(frequency);
            
            self.frequency_to_vertices.insert(frequency, vertices);
            self.frequency_to_edges.insert(frequency, edges);
            self.frequency_to_colors.insert(frequency, color);
            self.complexity_cache.insert(frequency, complexity);
        }
    }
    
    fn generate_vertices(&self, frequency: f32) -> Vec<egui::Vec2> {
        let complexity = self.calculate_complexity(frequency);
        let vertex_count = (complexity * 12.0) as usize;
        
        let mut vertices = Vec::new();
        
        for i in 0..vertex_count {
            let angle = (i as f32 / vertex_count as f32) * std::f32::consts::TAU;
            let radius = 0.1 + (frequency / 1000.0) * 0.2;
            
            vertices.push(egui::Vec2::new(
                angle.cos() * radius,
                angle.sin() * radius
            ));
        }
        
        vertices
    }
    
    fn generate_edges(&self, vertices: &[egui::Vec2]) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        
        // Connect vertices in a circular pattern
        for i in 0..vertices.len() {
            let next = (i + 1) % vertices.len();
            edges.push((i, next));
        }
        
        // Add radial connections for higher frequencies
        if self.calculate_complexity(vertices[0].length()) > 0.5 {
            let center = egui::Vec2::ZERO;
            for i in 0..vertices.len() {
                edges.push((vertices.len(), i)); // Connect to center
            }
        }
        
        edges
    }
    
    fn generate_color(&self, frequency: f32) -> egui::Color32 {
        let hue = (frequency / 1000.0) * 360.0;
        let saturation = 0.8;
        let value = 0.7;
        
        self.hsv_to_rgb(hue, saturation, value)
    }
    
    fn calculate_complexity(&self, frequency: f32) -> f32 {
        // Higher frequencies create more complex patterns
        (frequency / 1000.0).min(1.0)
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

### Harmonic Breeding System

```rust
pub struct HarmonicBreedingSystem {
    pub frequency_calculator: FrequencyCalculator,
    pub interval_analyzer: IntervalAnalyzer,
    pub harmonic_generator: HarmonicGenerator,
    pub visual_feedback: HarmonicVisualFeedback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MusicalInterval {
    Unison,              // Same frequency
    Octave,              // 2:1 ratio
    PerfectFifth,        // 3:2 ratio
    MajorThird,          // 5:4 ratio
    MinorThird,          // 6:5 ratio
    MajorSixth,          // 5:3 ratio
    MinorSixth,          // 8:5 ratio
    Tritone,             // 6:4:3 ratio
    Dissonant,            // Complex ratio
}

impl HarmonicBreedingSystem {
    pub fn new() -> Self {
        Self {
            frequency_calculator: FrequencyCalculator::new(),
            interval_analyzer: IntervalAnalyzer::new(),
            harmonic_generator: HarmonicGenerator::new(),
            visual_feedback: HarmonicVisualFeedback::new(),
        }
    }
    
    pub fn calculate_breeding_result(&self, parent_a: Culture, parent_b: Culture) -> BreedingResult {
        let freq_a = parent_a.get_frequency();
        let freq_b = parent_b.get_frequency();
        
        let interval = self.interval_analyzer.analyze_interval(freq_a, freq_b);
        let offspring_freq = self.frequency_calculator.calculate_offspring_frequency(freq_a, freq_b);
        let offspring_culture = self.determine_offspring_culture(parent_a, parent_b, offspring_freq);
        
        let visual_feedback = self.visual_feedback.generate_breeding_visual(
            parent_a, parent_b, offspring_culture, interval
        );
        
        BreedingResult {
            parent_a,
            parent_b,
            offspring_culture,
            offspring_frequency: offspring_freq,
            interval,
            visual_feedback,
            success_probability: self.calculate_success_probability(interval),
            genetic_traits: self.calculate_genetic_traits(interval),
        }
    }
    
    fn determine_offspring_culture(&self, parent_a: Culture, parent_b: Culture, offspring_freq: f32) -> Culture {
        // Check if offspring frequency matches any existing culture
        for culture in ALL_CULTURES.iter() {
            if (culture.get_frequency() - offspring_freq).abs() < 5.0 {
                return *culture;
            }
        }
        
        // If no match, create hybrid culture
        Culture::Hybrid {
            parent_a,
            parent_b,
            frequency: offspring_freq,
        }
    }
    
    fn calculate_success_probability(&self, interval: MusicalInterval) -> f32 {
        match interval {
            MusicalInterval::Unison => 0.95,           // Very high success
            MusicalInterval::Octave => 0.90,           // High success
            MusicalInterval::PerfectFifth => 0.85,    // Good success
            MusicalInterval::MajorThird => 0.80,     // Good success
            MusicalInterval::MinorThird => 0.75,     // Moderate success
            MusicalInterval::MajorSixth => 0.70,     // Moderate success
            MusicalInterval::MinorSixth => 0.65,     // Moderate success
            MusicalInterval::Tritone => 0.60,       // Lower success
            MusicalInterval::Dissonant => 0.40,     // Low success
        }
    }
    
    fn calculate_genetic_traits(&self, interval: MusicalInterval) -> Vec<GeneticTrait> {
        match interval {
            MusicalInterval::Unison => vec![
                GeneticTrait::Stability,
                GeneticTrait::Purity,
                GeneticTrait::Resonance,
            ],
            MusicalInterval::Octave => vec![
                GeneticTrait::Amplification,
                GeneticTrait::Clarity,
                GeneticTrait::Energy,
            ],
            MusicalInterval::PerfectFifth => vec![
                GeneticTrait::Balance,
                GeneticTrait::Harmony,
                GeneticTrait::Wisdom,
            ],
            MusicalInterval::MajorThird => vec![
                GeneticTrait::Creativity,
                GeneticTrait::Joy,
                GeneticTrait::Expression,
            ],
            MusicalInterval::MinorThird => vec![
                GeneticTrait::EmotionalDepth,
                GeneticTrait::Intuition,
                GeneticTrait::Sensitivity,
            ],
            MusicalInterval::Dissonant => vec![
                GeneticTrait::Instability,
                GeneticTrait::Mutation,
                GeneticTrait::Adaptation,
            ],
            _ => vec![],
        }
    }
}
```

## Performance Specifications

### Real-Time Requirements

#### Frame Rate Targets

- **60 FPS**: Minimum for smooth cymatic animation
- **120 FPS**: Target for high-end systems
- **30 FPS**: Acceptable for low-end systems

#### Memory Constraints

- **Cymatic Patterns**: Max 20 active patterns simultaneously
- **Particle Systems**: Max 500 particles per pattern
- **Audio Channels**: 16 concurrent audio channels
- **Frequency Calculations**: Cached for 100+ frequencies

#### CPU Budget

- **Cymatic Rendering**: 2ms per frame
- **Audio Processing**: 1ms per frame
- **Frequency Analysis**: 0.5ms per frame
- **Visual Effects**: 1ms per frame

### Quality Settings

#### High Quality (Recommended)

- **Cymatic Resolution**: 1024 vertices per pattern
- **Particle Count**: 500 particles per pattern
- **Audio Sample Rate**: 48 kHz
- **Audio Bit Depth**: 24-bit
- **Color Depth**: 32-bit RGBA

#### Medium Quality

- **Cymatic Resolution**: 512 vertices per pattern
- **Particle Count**: 200 particles per pattern
- **Audio Sample Rate**: 44.1 kHz
- **Audio Bit Depth**: 16-bit
- **Color Depth**: 24-bit RGB

#### Low Quality

- **Cymatic Resolution**: 256 vertices per pattern
- **Particle Count**: 50 particles per pattern
- **Audio Sample Rate**: 22.05 kHz
- **Audio Bit Depth**: 16-bit
- **Color Depth**: 16-bit RGB

## Integration Points

### Audio System Integration

```rust
pub struct AudioSystem {
    pub frequency_synthesizer: FrequencySynthesizer,
    pub cymatic_processor: CymaticProcessor,
    pub spatial_audio: SpatialAudio,
    pub dynamic_mixer: DynamicMixer,
}

impl AudioSystem {
    pub fn play_cultural_sound(&mut self, culture: Culture, volume: f32) {
        let frequency = culture.get_frequency();
        let waveform = culture.get_waveform();
        let harmonics = culture.get_harmonics();
        
        self.frequency_synthesizer.play_frequency(frequency, waveform, harmonics, volume);
    }
    
    pub fn play_cymatic_pattern(&mut self, pattern: &CymaticPattern) {
        let cymatic_audio = self.cymatic_processor.generate_audio(pattern);
        self.spatial_audio.play_at_position(cymatic_audio, pattern.position);
    }
}
```

### Visual System Integration

```rust
pub struct VisualSystem {
    pub cymatic_renderer: CymaticRenderer,
    pub geometry_animator: GeometryAnimator,
    pub color_harmonizer: ColorHarmonizer,
    pub particle_renderer: ParticleRenderer,
}

impl VisualSystem {
    pub fn render_slime(&mut self, ctx: &egui::Context, slime: &SlimeGenome, time: f32) {
        let frequency = slime.culture.get_frequency();
        let energy_level = slime.energy_level;
        
        // Render cymatic pattern
        self.cymatic_renderer.render_slime_cymatics(ctx, slime, slime.position, time);
        
        // Animate geometry based on frequency
        self.geometry_animator.animate_slime(slime, frequency, time);
        
        // Harmonize colors with frequency
        self.color_harmonizer.harmonize_slime_color(slime, frequency);
    }
}
```

## Validation Criteria

### Frequency Accuracy

- [ ] All cultural frequencies match Solfeggio scale within ±1 Hz
- [ ] Harmonic intervals are mathematically precise
- [ ] Frequency calculations maintain 64-bit precision
- [ ] Audio synthesis produces accurate waveforms

### Visual Fidelity

- [ ] Cymatic patterns accurately represent sound waves
- [ ] Geometric complexity scales with frequency
- [ ] Color harmonization matches frequency relationships
- [ ] Animation maintains 60 FPS performance

### Performance Standards

- [ ] Real-time cymatic rendering at 60 FPS
- [ ] Audio processing latency < 10ms
- [ ] Memory usage stays within budget
- [ ] CPU usage stays within 70% of available resources

The Terminal Specs provide the technical foundation for implementing the frequency-driven OPERATOR rpgCore system, ensuring that the harmonic resonance and cymatic rendering are mathematically precise and performant.
