# Chromatic Framework: The 9-Point Wheel

> **Status:** VISUAL SYSTEM SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-022, TRINARY_SYSTEM_ARCHITECTURE.md, SPEC.md §3

## Overview

The Chromatic Framework establishes the visual and mathematical foundation for the 9-Point Color Wheel system that scales the "Slime Planet" from a basic garden to a complex planetary ecosystem. This framework provides the exact mathematical and visual structure needed for sophisticated chromatic interactions.

## Color Wheel Architecture

### Primary Layer (Inner Triangle)

| Color | Culture | RGB Hex | Dominant Stat | Symbol | Narrative Role |
|-------|---------|--------|---------------|--------|----------------|
| Red | EMBER | #FF4444 | ATK | 🔥 | Power / Heat / Combat |
| Blue | TIDE | #4444FF | CHM | 💧 | Energy / Flow / Diplomacy |
| Yellow | GALE | #FFFF44 | SPD | 🌪️ | Speed / Wind / Scouting |

### Secondary Layer (Middle Triangle)

| Color | Culture | RGB Hex | Dominant Stat | Symbol | Narrative Role |
|-------|---------|--------|---------------|--------|----------------|
| Green | MARSH | #44FF44 | RES | 🌿 | Balance / Toxic / Survival |
| Orange | Orange | #FF8844 | MND | ⚙️ | Logic / Construction / Engineering |
| Purple | CRYSTAL | #FF44FF | HP | 💎 | Tank / Focus / Armor |

### Tertiary Layer (Outer Triangle)

| Color | Culture | RGB Hex | Dominant Stat | Symbol | Narrative Role |
|-------|---------|--------|---------------|--------|----------------|
| Teal | Teal | #44FFAA | STB | 🛡️ | Stability / Support |
| Amber | Amber | #FFAA44 | DUR | 🏭 | Durability / Industry |
| Frost | TUNDRA | #AAFFFF | END | ❄️ | Preservation / Slow / Stasis |

### Exception Layer

| Color | Culture | RGB Hex | Dominant Stat | Symbol | Narrative Role |
|-------|---------|--------|---------------|--------|----------------|
| White | VOID | #FFFFFF | — | ⚫ | Universal Constant / White Light |

## Mathematical Relationships

### Angular Positioning

```rust
pub struct ChromaticWheel {
    pub center: (f32, f32),
    pub radius: f32,
    pub cultures: [CulturePosition; 10],
}

#[derive(Debug, Clone)]
pub struct CulturePosition {
    pub culture: Culture,
    pub angle: f32,        // Degrees from 0° (Red)
    pub radius_factor: f32, // Distance from center (layer)
    pub color: [u8; 3],     // RGB values
}

impl ChromaticWheel {
    pub fn new() -> Self {
        let cultures = [
            // Primary Layer (radius_factor = 0.33)
            CulturePosition { culture: Culture::Ember, angle: 0.0, radius_factor: 0.33, color: [255, 68, 68] },
            CulturePosition { culture: Culture::Gale, angle: 120.0, radius_factor: 0.33, color: [255, 255, 68] },
            CulturePosition { culture: Culture::Tide, angle: 240.0, radius_factor: 0.33, color: [68, 68, 255] },
            
            // Secondary Layer (radius_factor = 0.66)
            CulturePosition { culture: Culture::Orange, angle: 60.0, radius_factor: 0.66, color: [255, 136, 68] },
            CulturePosition { culture: Culture::Marsh, angle: 180.0, radius_factor: 0.66, color: [68, 255, 68] },
            CulturePosition { culture: Culture::Crystal, angle: 300.0, radius_factor: 0.66, color: [255, 68, 255] },
            
            // Tertiary Layer (radius_factor = 1.0)
            CulturePosition { culture: Culture::Teal, angle: 30.0, radius_factor: 1.0, color: [68, 255, 170] },
            CulturePosition { culture: Culture::Amber, angle: 150.0, radius_factor: 1.0, color: [255, 170, 68] },
            CulturePosition { culture: Culture::Tundra, angle: 270.0, radius_factor: 1.0, color: [170, 255, 255] },
            
            // Void (center)
            CulturePosition { culture: Culture::Void, angle: 0.0, radius_factor: 0.0, color: [255, 255, 255] },
        ];
        
        Self { center: (0.5, 0.5), radius: 0.4, cultures }
    }
}
```

### Color Mixing Mathematics

```rust
impl Culture {
    /// Calculate the angular distance between two cultures on the wheel
    pub fn angular_distance(self, other: Culture) -> f32 {
        let wheel = ChromaticWheel::new();
        let self_pos = wheel.get_position(self);
        let other_pos = wheel.get_position(other);
        
        let diff = (self_pos.angle - other_pos.angle).abs();
        if diff > 180.0 { 360.0 - diff } else { diff }
    }
    
    /// Calculate color mixing probability based on angular distance
    pub fn mixing_probability(self, other: Culture) -> f32 {
        let distance = self.angular_distance(other);
        
        // Adjacent colors (60°) have highest probability
        // Same color (0°) has moderate probability
        // Opposite colors (180°) have lowest probability
        match distance {
            0.0 => 0.5,    // Same color
            60.0 => 0.8,   // Adjacent (optimal mixing)
            120.0 => 0.3,  // Skip-one mixing
            180.0 => 0.1,  // Opposite mixing (rare)
            _ => 0.0,      // Invalid mixing
        }
    }
}
```

## Visual System Design

### Slime Profile Card Integration

```rust
pub struct SlimeProfileCard {
    pub slime: SlimeGenome,
    pub chromatic_info: ChromaticInfo,
}

#[derive(Debug, Clone)]
pub struct ChromaticInfo {
    pub culture: Culture,
    pub layer: ChromaticLayer,
    pub color_hex: String,
    pub border_color: String,
    pub icon: char,
    pub wheel_position: (f32, f32),
}

impl SlimeProfileCard {
    pub fn render_border(&self) -> String {
        format!(
            "border: 3px solid {}; background: linear-gradient(135deg, {}, {});",
            self.chromatic_info.border_color,
            self.chromatic_info.color_hex,
            self.lighten_color(&self.chromatic_info.color_hex, 0.3)
        )
    }
    
    pub fn render_layer_indicator(&self) -> String {
        match self.chromatic_info.layer {
            ChromaticLayer::Primary => "🔥".to_string(),
            ChromaticLayer::Secondary => "⚙️".to_string(),
            ChromaticLayer::Tertiary => "🏔️".to_string(),
            ChromaticLayer::Void => "⚫".to_string(),
        }
    }
}
```

### World Map Node Visualization

```rust
pub struct PlanetaryNode {
    pub id: Uuid,
    pub position: (f32, f32),
    pub culture: Culture,
    pub control_strength: f32,
    pub resource_value: u64,
}

impl PlanetaryNode {
    pub fn render_node(&self) -> NodeVisualization {
        NodeVisualization {
            color: self.culture.get_color_hex(),
            size: self.calculate_node_size(),
            border_width: self.control_strength * 5.0,
            pulse_animation: self.resource_value > 100,
            cultural_symbol: self.culture.get_symbol(),
        }
    }
    
    fn calculate_node_size(&self) -> f32 {
        let base_size = match self.culture.layer() {
            ChromaticLayer::Primary => 20.0,
            ChromaticLayer::Secondary => 25.0,
            ChromaticLayer::Tertiary => 30.0,
            ChromaticLayer::Void => 35.0,
        };
        
        base_size * (1.0 + self.resource_value as f32 / 1000.0)
    }
}
```

## Color Wheel UI Component

### Interactive Color Wheel

```rust
pub struct ColorWheelComponent {
    pub selected_culture: Option<Culture>,
    pub mixing_candidates: Vec<Culture>,
    pub show_probabilities: bool,
}

impl ColorWheelComponent {
    pub fn render(&self) -> Html {
        html! {
            <div class="color-wheel-container">
                <svg width="300" height="300" viewBox="0 0 100 100">
                    { self.render_wheel_segments() }
                    { self.render_center_void() }
                    { self.render_selection_indicators() }
                    { self.render_mixing_arcs() }
                </svg>
                <div class="color-legend">
                    { self.render_layer_legend() }
                    { self.render_mixing_guide() }
                </div>
            </div>
        }
    }
    
    fn render_wheel_segments(&self) -> Html {
        let segments = vec![
            // Primary segments
            self.create_segment("Ember", "#FF4444", 0.0, 120.0, 0.33),
            self.create_segment("Gale", "#FFFF44", 120.0, 240.0, 0.33),
            self.create_segment("Tide", "#4444FF", 240.0, 360.0, 0.33),
            
            // Secondary segments
            self.create_segment("Orange", "#FF8844", 60.0, 180.0, 0.66),
            self.create_segment("Marsh", "#44FF44", 180.0, 300.0, 0.66),
            self.create_segment("Crystal", "#FF44FF", 300.0, 420.0, 0.66),
            
            // Tertiary segments
            self.create_segment("Teal", "#44FFAA", 30.0, 150.0, 1.0),
            self.create_segment("Amber", "#FFAA44", 150.0, 270.0, 1.0),
            self.create_segment("Frost", "#AAFFFF", 270.0, 390.0, 1.0),
        ];
        
        Html::from_iter(segments)
    }
}
```

## Discovery Mechanics

### Missing Frequency System

```rust
pub struct DiscoveryTracker {
    pub discovered_cultures: HashSet<Culture>,
    pub missing_frequencies: Vec<Culture>,
    pub discovery_progress: HashMap<Culture, f32>,
}

impl DiscoveryTracker {
    pub fn check_missing_frequencies(&self) -> Vec<Culture> {
        ALL_CULTURES
            .iter()
            .filter(|culture| !self.discovered_cultures.contains(culture))
            .cloned()
            .collect()
    }
    
    pub fn calculate_discovery_chance(&self, culture: Culture) -> f32 {
        // Discovery chance increases with:
        // 1. Number of adjacent discovered cultures
        // 2. Successful mixing attempts
        // 3. Exploration of appropriate biomes
        
        let adjacent_discovered = self.count_adjacent_discovered(culture);
        let mixing_attempts = self.discovery_progress.get(&culture).unwrap_or(&0.0);
        
        (adjacent_discovered as f32 * 0.2 + mixing_attempts * 0.1).min(0.8)
    }
}
```

### Biome-Culture Alignment

```rust
pub struct BiomeSystem {
    pub biomes: HashMap<BiomeType, Vec<Culture>>,
}

impl BiomeSystem {
    pub fn new() -> Self {
        let mut biomes = HashMap::new();
        
        biomes.insert(BiomeType::Volcanic, vec![Culture::Ember, Culture::Orange]);
        biomes.insert(BiomeType::Aquatic, vec![Culture::Tide, Culture::Teal]);
        biomes.insert(BiomeType::Arid, vec![Culture::Gale, Culture::Amber]);
        biomes.insert(BiomeType::Toxic, vec![Culture::Marsh, Culture::Crystal]);
        biomes.insert(BiomeType::Frozen, vec![Culture::Tundra, Culture::Frost]);
        biomes.insert(BiomeType::Void, vec![Culture::Void]);
        
        Self { biomes }
    }
    
    pub fn get_native_cultures(&self, biome: BiomeType) -> &[Culture] {
        self.biomes.get(&biome).unwrap_or(&[])
    }
}
```

## Implementation Guidelines

### Rendering Pipeline

```rust
pub fn draw_slime_chromatic(
    slime: &SlimeGenome,
    ctx: &mut egui::Context,
    pos: egui::Pos2,
    size: egui::Vec2,
) {
    // 1. Get culture color from 9-point wheel
    let base_color = slime.culture.get_rgb_color();
    
    // 2. Apply layer-based shading
    let layer_factor = match slime.culture.layer() {
        ChromaticLayer::Primary => 1.2,
        ChromaticLayer::Secondary => 1.0,
        ChromaticLayer::Tertiary => 0.8,
        ChromaticLayer::Void => 1.5,
    };
    
    // 3. Mix with body pattern and accessory colors
    let final_color = mix_colors(base_color, slime.pattern_color(), 0.7);
    
    // 4. Render with chromatic effects
    let mesh = create_slime_mesh(pos, size, final_color, layer_factor);
    ctx.painter().add(mesh);
}
```

### Performance Optimization

```rust
// Pre-computed color lookup table
static COLOR_WHEEL_CACHE: Lazy<ChromaticWheel> = Lazy::new(ChromaticWheel::new);

// Fast color mixing cache
static MIXING_CACHE: Lazy<HashMap<(Culture, Culture), Option<Culture>>> = 
    Lazy::new(|| {
        let mut cache = HashMap::new();
        for a in ALL_CULTURES {
            for b in ALL_CULTURES {
                cache.insert((a, b), calculate_color_mix(a, b));
            }
        }
        cache
    });
```

## Validation Criteria

- [ ] All 9 cultures render with correct colors and symbols
- [ ] Color wheel displays proper angular relationships
- [ ] Mixing probabilities reflect mathematical relationships
- [ ] UI components scale properly with different screen sizes
- [ ] Discovery mechanics align with biome distributions
- [ ] Performance maintains 60fps with full chromatic rendering

## Future Enhancements

1. **Animated Color Wheel**: Rotating wheel showing mixing probabilities
2. **Chromatic Music System**: Sound effects that match culture colors
3. **Color Blind Support**: Alternative patterns for accessibility
4. **Dynamic Lighting**: Environmental effects on slime colors
5. **Chromatic Evolution**: Visual evolution as slimes level up

The Chromatic Framework provides the visual and mathematical foundation that transforms the Slime Planet into a sophisticated planetary ecosystem where color becomes both a strategic resource and a discovery mechanic.
