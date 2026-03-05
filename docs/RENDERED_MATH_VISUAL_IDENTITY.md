# Rendered Math Visual Identity System

> **Status:** VISUAL ENGINEERING SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-020, SHEPHERD_GARDEN_WANDERING.md, SPEC.md §3

## Overview

The Rendered Math Visual Identity system defines how every visual element in OPERATOR is mathematically derived from the genome, eliminating the need for external image assets. This creates a cohesive, procedurally generated visual language where slimes, UI elements, and environmental effects all share the same mathematical foundation.

## Core Visual Principles

### Zero External Assets Philosophy

```rust
#[derive(Debug, Clone)]
pub struct VisualIdentitySystem {
    pub rendering_engine: MathRenderingEngine,
    pub color_calculator: ColorCalculator,
    pub shape_generator: ShapeGenerator,
    pub animation_system: AnimationSystem,
    pub effect_processor: EffectProcessor,
}

impl VisualIdentitySystem {
    pub fn new() -> Self {
        Self {
            rendering_engine: MathRenderingEngine::new(),
            color_calculator: ColorCalculator::new(),
            shape_generator: ShapeGenerator::new(),
            animation_system: AnimationSystem::new(),
            effect_processor: EffectProcessor::new(),
        }
    }
    
    pub fn render_slime(&self, slime: &SlimeGenome, ctx: &mut egui::Context, pos: egui::Pos2, time: f32) {
        // All visual elements derived from genome
        let color = self.color_calculator.calculate_slime_color(slime);
        let shape = self.shape_generator.generate_slime_shape(slime);
        let animation = self.animation_system.calculate_slime_animation(slime, time);
        let effects = self.effect_processor.generate_slime_effects(slime);
        
        // Render with mathematical precision
        self.rendering_engine.draw_slime(ctx, pos, &color, &shape, &animation, &effects);
    }
}
```

## Slime Anatomy: Mathematical Derivation

### 12-Point Parametric Spline System

```rust
#[derive(Debug, Clone)]
pub struct SlimeAnatomy {
    pub control_points: [egui::Vec2; 12],
    pub spline_tension: f32,
    pub roundness_factor: f32,
    pub asymmetry: f32,
    pub body_deformation: BodyDeformation,
}

#[derive(Debug, Clone)]
pub struct BodyDeformation {
    pub compression_factor: f32,
    pub stretch_factor: f32,
    pub wobble_amplitude: f32,
    pub wobble_frequency: f32,
    pub breathing_phase: f32,
}

impl SlimeAnatomy {
    pub fn generate_from_genome(slime: &SlimeGenome) -> Self {
        let base_shape = Self::calculate_base_shape(slime.culture);
        let personality_modulation = Self::calculate_personality_modulation(slime);
        let size_factor = Self::calculate_size_factor(slime);
        
        Self {
            control_points: Self::generate_control_points(base_shape, personality_modulation, size_factor),
            spline_tension: Self::calculate_spline_tension(slime.culture),
            roundness_factor: Self::calculate_roundness_factor(slime.culture),
            asymmetry: Self::calculate_asymmetry(slime),
            body_deformation: BodyDeformation::new(slime),
        }
    }
    
    fn calculate_roundness_factor(culture: Culture) -> f32 {
        match culture {
            Culture::Crystal => 0.3,    // Low roundness - angular
            Culture::Marsh => 0.9,      // High roundness - blobby
            Culture::Tide => 0.7,       // Medium-high roundness - fluid
            Culture::Ember => 0.5,      // Medium roundness - balanced
            Culture::Gale => 0.6,       // Medium roundness - dynamic
            Culture::Orange => 0.4,    // Low-medium roundness - structured
            Culture::Teal => 0.8,      // High roundness - ethereal
            Culture::Amber => 0.5,     // Medium roundness - solid
            Culture::Tundra => 0.4,    // Low roundness - crystalline
            Culture::Void => 0.6,      // Medium roundness - perfect sphere
        }
    }
    
    fn generate_control_points(
        base_shape: f32,
        personality_modulation: f32,
        size_factor: f32
    ) -> [egui::Vec2; 12] {
        let mut points = [egui::Vec2::ZERO; 12];
        let radius = base_shape * size_factor;
        
        // Generate 12 points around circle with personality modulation
        for i in 0..12 {
            let angle = (i as f32 / 12.0) * std::f32::consts::TAU;
            let modulation = 1.0 + personality_modulation * (angle.sin() * 0.2);
            let point_radius = radius * modulation;
            
            points[i] = egui::Vec2::new(
                angle.cos() * point_radius,
                angle.sin() * point_radius,
            );
        }
        
        points
    }
    
    pub fn calculate_spline_path(&self, time: f32) -> Vec<egui::Pos2> {
        let mut path = Vec::new();
        
        // Generate smooth spline through control points
        for i in 0..12 {
            let p0 = self.control_points[i];
            let p1 = self.control_points[(i + 1) % 12];
            let p2 = self.control_points[(i + 2) % 12];
            let p3 = self.control_points[(i + 3) % 12];
            
            // Catmull-Rom spline
            for t in 0..10 {
                let t = t as f32 / 10.0;
                let point = self.catmull_rom_point(p0, p1, p2, p3, t);
                path.push(point);
            }
        }
        
        // Apply wobble animation
        self.apply_wobble_to_path(&mut path, time);
        
        path
    }
    
    fn catmull_rom_point(
        p0: egui::Vec2,
        p1: egui::Vec2,
        p2: egui::Vec2,
        p3: egui::Vec2,
        t: f32
    ) -> egui::Vec2 {
        let t2 = t * t;
        let t3 = t2 * t;
        
        0.5 * (
            (2.0 * p1) +
            (-p0 + p2) * t +
            (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2 +
            (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3
        )
    }
    
    fn apply_wobble_to_path(&self, path: &mut Vec<egui::Pos2>, time: f32) {
        for point in path.iter_mut() {
            let wobble_x = self.body_deformation.wobble_amplitude * 
                (time * self.body_deformation.wobble_frequency + point.x).sin();
            let wobble_y = self.body_deformation.wobble_amplitude * 
                (time * self.body_deformation.wobble_frequency + point.y).cos();
            
            point.x += wobble_x;
            point.y += wobble_y;
        }
    }
}
```

### Wobble Animation System

```rust
impl BodyDeformation {
    pub fn new(slime: &SlimeGenome) -> Self {
        let (frequency, amplitude) = Self::calculate_wobble_parameters(slime);
        
        Self {
            compression_factor: 0.0,
            stretch_factor: 0.0,
            wobble_amplitude: amplitude,
            wobble_frequency: frequency,
            breathing_phase: 0.0,
        }
    }
    
    fn calculate_wobble_parameters(slime: &SlimeGenome) -> (f32, f32) {
        let base_frequency = slime.base_spd as f32 / 100.0; // Normalize SPD
        let base_amplitude = 0.05; // 5% of body size
        
        // Mood affects wobble
        let mood_modifier = match slime.get_mood() {
            SlimeMood::Playful => (1.5, 1.2),    // Higher frequency, larger amplitude
            SlimeMood::Sleepy => (0.5, 0.3),    // Lower frequency, smaller amplitude
            SlimeMood::Curious => (1.2, 0.8),    // Moderate increase
            SlimeMood::Happy => (1.0, 1.0),      // Normal
            SlimeMood::Scared => (2.0, 0.6),     // High frequency, moderate amplitude
            SlimeMood::Angry => (0.8, 0.4),      // Lower frequency, small amplitude
        };
        
        let frequency = base_frequency * mood_modifier.0;
        let amplitude = base_amplitude * mood_modifier.1;
        
        (frequency, amplitude)
    }
    
    pub fn update(&mut self, delta_time: f32, slime: &SlimeGenome) {
        // Update breathing animation
        self.breathing_phase += delta_time * 0.5; // 0.5 Hz breathing
        
        // Update wobble
        let (frequency, amplitude) = Self::calculate_wobble_parameters(slime);
        self.wobble_frequency = frequency;
        self.wobble_amplitude = amplitude;
        
        // Apply compression/stretch from interactions
        self.compression_factor *= 0.95; // Gradual recovery
        self.stretch_factor *= 0.95;
    }
    
    pub fn get_deformation_at_time(&self, time: f32) -> (f32, f32) {
        let breathing = (self.breathing_phase + time).sin() * 0.02; // 2% breathing
        let wobble_x = self.wobble_amplitude * (time * self.wobble_frequency).sin();
        let wobble_y = self.wobble_amplitude * (time * self.wobble_frequency * 1.3).cos();
        
        let total_deformation = breathing + wobble_x.abs().max(wobble_y.abs());
        
        let compression = self.compression_factor + total_deformation;
        let stretch = self.stretch_factor + total_deformation * 0.5;
        
        (compression, stretch)
    }
}
```

### 3-Layer Radial Gradient System

```rust
#[derive(Debug, Clone)]
pub struct GlowEffect {
    pub layers: [GlowLayer; 3],
    pub intensity: f32,
    pub color_shift: egui::Color32,
    pub pulse_frequency: f32,
    pub pulse_phase: f32,
}

#[derive(Debug, Clone)]
pub struct GlowLayer {
    pub radius: f32,
    pub color: egui::Color32,
    pub alpha: f32,
    pub blend_mode: BlendMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlendMode {
    Additive,
    Multiply,
    Screen,
    Overlay,
}

impl GlowEffect {
    pub fn generate_from_genome(slime: &SlimeGenome, time: f32) -> Self {
        let base_color = Self::calculate_base_color(slime);
        let tier_intensity = Self::calculate_tier_intensity(slime.generation);
        let mood_shift = Self::calculate_mood_shift(slime);
        
        let layers = [
            GlowLayer {
                radius: 1.0,    // Core layer
                color: base_color,
                alpha: 1.0,
                blend_mode: BlendMode::Additive,
            },
            GlowLayer {
                radius: 1.3,    // Middle layer
                color: Self::shift_color(base_color, mood_shift),
                alpha: 0.6,
                blend_mode: BlendMode::Additive,
            },
            GlowLayer {
                radius: 1.6,    // Outer layer
                color: Self::shift_color(base_color, mood_shift * 2.0),
                alpha: 0.3,
                blend_mode: BlendMode::Screen,
            },
        ];
        
        Self {
            layers,
            intensity: tier_intensity,
            color_shift: mood_shift,
            pulse_frequency: Self::calculate_pulse_frequency(slime),
            pulse_phase: time * Self::calculate_pulse_frequency(slime),
        }
    }
    
    fn calculate_tier_intensity(generation: u32) -> f32 {
        // Higher tiers glow more intensely
        match generation {
            0..=1 => 0.2,   // T1-T2: Low glow
            2..=3 => 0.4,   // T3-T4: Medium glow
            4..=5 => 0.6,   // T5-T6: High glow
            6..=7 => 0.8,   // T7-T8: Very high glow
            _ => 1.0,       // T9+: Maximum glow
        }
    }
    
    fn calculate_base_color(slime: &SlimeGenome) -> egui::Color32 {
        let culture_color = slime.culture.get_rgb_color();
        let personality_shift = Self::calculate_personality_color_shift(slime);
        
        egui::Color32::from_rgb(
            ((culture_color[0] as f32 * personality_shift.0) as u8).min(255),
            ((culture_color[1] as f32 * personality_shift.1) as u8).min(255),
            ((culture_color[2] as f32 * personality_shift.2) as u8).min(255),
        )
    }
    
    fn calculate_mood_shift(slime: &SlimeGenome) -> egui::Color32 {
        match slime.get_mood() {
            SlimeMood::Happy => egui::Color32::from_rgb(255, 255, 200), // Warm yellow
            SlimeMood::Playful => egui::Color32::from_rgb(255, 200, 150), // Warm orange
            SlimeMood::Curious => egui::Color32::from_rgb(200, 200, 255), // Cool blue
            SlimeMood::Sleepy => egui::Color32::from_rgb(150, 150, 200), // Cool purple
            SlimeMood::Scared => egui::Color32::from_rgb(255, 150, 150), // Light red
            SlimeMood::Angry => egui::Color32::from_rgb(255, 100, 100), // Strong red
        }
    }
    
    pub fn render(&self, ctx: &mut egui::Context, center: egui::Pos2, base_size: f32, time: f32) {
        let pulse_intensity = (self.pulse_phase + time * self.pulse_frequency).sin() * 0.3 + 0.7;
        
        for (i, layer) in self.layers.iter().enumerate() {
            let layer_intensity = pulse_intensity * self.intensity * layer.alpha;
            let layer_size = base_size * layer.radius;
            let layer_color = Self::apply_alpha(layer.color, layer_intensity);
            
            // Render glow layer
            ctx.painter().circle_filled(
                center,
                layer_size,
                layer_color
            );
        }
    }
    
    fn apply_alpha(color: egui::Color32, alpha: f32) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            (color.a() as f32 * alpha) as u8,
        )
    }
}
```

### Iridescent Pattern System

```rust
#[derive(Debug, Clone)]
pub struct IridescentEffect {
    pub base_pattern: IridescentPattern,
    pub shift_speed: f32,
    pub color_palette: Vec<egui::Color32>,
    pub pattern_complexity: u8,
    pub animation_phase: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IridescentPattern {
    Rainbow,           // Full spectrum rainbow
    OilSlick,         // Oil slick colors
    MotherOfPearl,    // Mother of pearl
    Metallic,         // Metallic sheen
    Crystal,          // Crystal refraction
    Bioluminescent,   // Bio-luminescent
    Plasma,           // Plasma effect
    Aurora,           // Aurora borealis
}

impl IridescentEffect {
    pub fn generate_from_culture(culture: Culture) -> Self {
        let pattern = Self::select_pattern_for_culture(culture);
        let color_palette = Self::generate_color_palette(pattern);
        let shift_speed = Self::calculate_shift_speed(culture);
        let complexity = Self::calculate_complexity(culture);
        
        Self {
            base_pattern: pattern,
            shift_speed,
            color_palette,
            pattern_complexity: complexity,
            animation_phase: 0.0,
        }
    }
    
    fn select_pattern_for_culture(culture: Culture) -> IridescentPattern {
        match culture {
            Culture::Tide => IridescentPattern::Rainbow,
            Culture::Crystal => IridescentPattern::Crystal,
            Culture::Marsh => IridescentPattern::Bioluminescent,
            Culture::Ember => IridescentPattern::Plasma,
            Culture::Gale => IridescentPattern::Aurora,
            Culture::Orange => IridescentPattern::Metallic,
            Culture::Teal => IridescentPattern::MotherOfPearl,
            Culture::Amber => IridescentPattern::OilSlick,
            Culture::Tundra => IridescentPattern::Crystal,
            Culture::Void => IridescentPattern::Plasma,
        }
    }
    
    fn generate_color_palette(pattern: IridescentPattern) -> Vec<egui::Color32> {
        match pattern {
            IridescentPattern::Rainbow => vec![
                egui::Color32::from_rgb(255, 0, 0),     // Red
                egui::Color32::from_rgb(255, 127, 0),   // Orange
                egui::Color32::from_rgb(255, 255, 0),   // Yellow
                egui::Color32::from_rgb(0, 255, 0),     // Green
                egui::Color32::from_rgb(0, 255, 255),   // Cyan
                egui::Color32::from_rgb(0, 0, 255),     // Blue
                egui::Color32::from_rgb(127, 0, 255),   // Indigo
                egui::Color32::from_rgb(255, 0, 255),   // Violet
            ],
            IridescentPattern::OilSlick => vec![
                egui::Color32::from_rgb(50, 50, 50),     // Dark gray
                egui::Color32::from_rgb(100, 80, 60),    // Brown
                egui::Color32::from_rgb(150, 120, 90),   // Light brown
                egui::Color32::from_rgb(200, 180, 140),  // Tan
                egui::Color32::from_rgb(180, 160, 120),  // Light tan
            ],
            IridescentPattern::MotherOfPearl => vec![
                egui::Color32::from_rgb(255, 253, 250),  // White
                egui::Color32::from_rgb(255, 248, 240),  // Ivory
                egui::Color32::from_rgb(255, 240, 230),  // Pearl
                egui::Color32::from_rgb(240, 230, 220),  // Light pearl
                egui::Color32::from_rgb(230, 220, 210),  // Pearl pink
            ],
            IridescentPattern::Metallic => vec![
                egui::Color32::from_rgb(192, 192, 192),  // Silver
                egui::Color32::from_rgb(184, 184, 184),  // Steel
                egui::Color32::from_rgb(176, 176, 176),  // Dark steel
                egui::Color32::from_rgb(255, 215, 0),    // Gold
                egui::Color32::from_rgb(205, 127, 50),    // Bronze
            ],
            IridescentPattern::Crystal => vec![
                egui::Color32::from_rgb(200, 200, 255),  // Light blue
                egui::Color32::from_rgb(150, 150, 255),  // Medium blue
                egui::Color32::from_rgb(100, 100, 255),  // Dark blue
                egui::Color32::from_rgb(255, 200, 255),  // Light purple
                egui::Color32::from_rgb(255, 150, 255),  // Medium purple
            ],
            IridescentPattern::Bioluminescent => vec![
                egui::Color32::from_rgb(50, 255, 50),     // Green
                egui::Color32::from_rgb(100, 255, 100),   // Light green
                egui::Color32::from_rgb(150, 255, 150),  // Pale green
                egui::Color32::from_rgb(200, 255, 200),  // Very pale green
                egui::Color32::from_rgb(255, 255, 100),  // Yellow-green
            ],
            IridescentPattern::Plasma => vec![
                egui::Color32::from_rgb(255, 100, 100),   // Red
                egui::Color32::from_rgb(255, 200, 100),   // Yellow
                egui::Color32::from_rgb(100, 255, 100),   // Green
                egui::Color32::from_rgb(100, 200, 255),   // Blue
                egui::Color32::from_rgb(200, 100, 255),   // Purple
            ],
            IridescentPattern::Aurora => vec![
                egui::Color32::from_rgb(100, 255, 200),   // Green
                egui::Color32::from_rgb(200, 255, 100),   // Yellow
                egui::Color32::from_rgb(255, 200, 100),   // Orange
                egui::Color32::from_rgb(255, 100, 200),   // Pink
                egui::Color32::from_rgb(200, 100, 255),   // Purple
            ],
        }
    }
    
    pub fn render(&mut self, ctx: &mut egui::Context, mesh: &mut egui::Mesh, time: f32) {
        self.animation_phase = time * self.shift_speed;
        
        // Generate iridescent colors
        let colors = self.generate_animated_colors(time);
        
        // Apply colors to mesh vertices
        for (i, vertex) in mesh.vertices.iter_mut().enumerate() {
            let color_index = i % colors.len();
            vertex.color = colors[color_index];
        }
    }
    
    fn generate_animated_colors(&self, time: f32) -> Vec<egui::Color32> {
        let mut animated_colors = Vec::new();
        
        for (i, &base_color) in self.color_palette.iter().enumerate() {
            let phase = self.animation_phase + (i as f32 * 0.1);
            let shift = (phase.sin() * 0.3 + 0.7).max(0.0); // 70-100% of base color
            
            animated_colors.push(Self::apply_alpha(base_color, shift));
        }
        
        animated_colors
    }
}
```

## Implementation Tasks

### Core Rendering Engine

1. **Implement MathRenderingEngine**: Central rendering system
2. **Create ShapeGenerator**: Procedural shape generation
3. **Build ColorCalculator**: Mathematical color calculation
4. **Develop AnimationSystem**: Time-based animation system
5. **Implement EffectProcessor**: Visual effects processing

### Slime Rendering Pipeline

1. **Create SlimeAnatomy**: 12-point spline system
2. **Implement WobbleAnimation**: Dynamic wobble system
3. **Build GlowEffect**: 3-layer gradient system
4. **Develop IridescentEffect**: Shifting color patterns
5. **Integrate with UI**: Connect to egui rendering pipeline

### Performance Optimization

1. **Caching System**: Cache expensive calculations
2. **LOD System**: Level of detail for distant slimes
3. **Batch Rendering**: Efficient rendering of multiple slimes
4. **Memory Management**: Efficient memory usage for animations

## Validation Criteria

- [ ] All visual elements are mathematically derived from genome
- [ ] No external image assets are required
- [ ] Slime animations feel natural and responsive
- [ ] Visual system maintains 60fps performance
- [ ] Different cultures have distinct visual identities
- [ ] Void slimes have maximum visual intensity

## Future Enhancements

1. **Advanced Materials**: More complex material rendering
2. **Environmental Effects**: Weather and environmental impact on visuals
3. **Procedural Textures**: Mathematical texture generation
4. **Dynamic Lighting**: Real-time lighting effects
5. **VR Support**: 3D rendering for virtual reality

The Rendered Math Visual Identity system creates a cohesive, procedurally generated visual world where every element is mathematically derived from the underlying genome, eliminating the need for external assets while maintaining a rich, dynamic visual experience.
