# ADR-020: The "Shepherd's Hand" Interaction

**Status:** ACCEPTED | **Date:** 2026-03-04 | **Author:** Gemini (via PyPro SDD-Edition)

## Context

The Command Deck interface needs to feel tactile and industrial rather than like a flat spreadsheet. Traditional UI interactions lack the physical feedback that makes the player feel connected to their slimes. The Shepherd's Hand interaction system provides immediate haptic and visual feedback for all micro-interactions, creating a sense of physical presence in the command deck.

## Decision

Implement a "Shepherd's Hand" interaction system where all micro-interactions (petting, splicing, feeding) provide immediate haptic/visual feedback using the Dice Engine's "Squash and Stretch" logic for UI buttons. This ensures the Command Deck feels "Tactile" and "Industrial" while maintaining the mathematical visual identity.

## Architecture

### Interaction Feedback System

```rust
#[derive(Debug, Clone)]
pub struct ShepherdHandSystem {
    pub interaction_types: HashMap<InteractionType, InteractionFeedback>,
    pub haptic_engine: HapticEngine,
    pub visual_feedback: VisualFeedbackEngine,
    pub audio_feedback: AudioFeedbackEngine,
    pub tactile_sensitivity: f32,
    pub feedback_intensity: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InteractionType {
    Pet,             // Gentle touch to increase affection
    Feed,             // Provide resources
    Splice,           // Breeding interaction
    Heal,             // Medical attention
    Play,             // Playful interaction
    Inspect,          // Detailed examination
    Deploy,           // Mission deployment
    Recall,           // Return from mission
}

#[derive(Debug, Clone)]
pub struct InteractionFeedback {
    pub haptic_pattern: HapticPattern,
    pub visual_effect: VisualEffect,
    pub sound_effect: SoundEffect,
    pub duration: Duration,
    pub intensity: f32,
    pub response_curve: ResponseCurve,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HapticPattern {
    GentleTap,        // Light, quick feedback
    SoftSquash,       // Soft, compressing sensation
    FirmPress,        // Strong, sustained pressure
    QuickVibration,   // Short, sharp vibration
    SustainedBuzz,    // Longer, continuous feedback
    PulseWave,        // Rhythmic pulsing
    RollingWave,      // Wave-like sensation
}

#[derive(Debug, Clone)]
pub enum VisualEffect {
    ButtonSquash,     // Button compresses visually
    ButtonStretch,    // Button stretches when pressed
    ColorShift,       // Color changes on interaction
    GlowPulse,        // Glowing pulse effect
    ParticleBurst,    // Particle explosion
    RippleEffect,     // Ripple from touch point
    IridescentShift,  // Color shimmer effect
    ScaleMorph,       // Size morphing animation
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoundEffect {
    SoftClick,        // Gentle click sound
    Squish,           // Soft squishing sound
    Chime,            // Pleasant chime
    Buzz,             // Low buzzing sound
    Hum,              // Soft humming
    Pop,              // Quick pop sound
    Whoosh,           // Whoosh effect
    Resonance,        // Resonant hum
}

#[derive(Debug, Clone)]
pub enum ResponseCurve {
    Linear,           // Straight line response
    Exponential,      // Exponential curve
    Logarithmic,      // Logarithmic curve
    Sigmoid,          // S-curve response
    Bounce,           // Bounce effect
    Elastic,          // Elastic deformation
}
```

### Squash and Stretch Mechanics

```rust
#[derive(Debug, Clone)]
pub struct SquashStretchEngine {
    pub squash_factor: f32,     // How much to compress (0.0 to 1.0)
    pub stretch_factor: f32,    // How much to stretch (0.0 to 1.0)
    pub recovery_speed: f32,    // Recovery animation speed
    pub damping: f32,           // Damping factor for oscillation
    pub elasticity: f32,        // Material elasticity
}

impl SquashStretchEngine {
    pub fn new() -> Self {
        Self {
            squash_factor: 0.15,   // 15% compression
            stretch_factor: 0.05,   // 5% stretch
            recovery_speed: 2.0,     // Recovery speed
            damping: 0.8,           // Damping factor
            elasticity: 0.7,        // Material elasticity
        }
    }
    
    pub fn calculate_deformation(&self, pressure: f32, time: f32) -> DeformationResult {
        // Calculate squash based on pressure
        let squash_amount = pressure * self.squash_factor;
        
        // Calculate stretch based on squash (conservation of volume)
        let stretch_amount = squash_amount * self.stretch_factor;
        
        // Apply recovery over time
        let recovery_factor = (-self.recovery_speed * time).exp();
        let current_squash = squash_amount * recovery_factor;
        let current_stretch = stretch_amount * recovery_factor;
        
        // Apply damping to prevent oscillation
        let damped_squash = current_squash * self.damping;
        let damped_stretch = current_stretch * self.damping;
        
        DeformationResult {
            squash: damped_squash,
            stretch: damped_stretch,
            recovery_time: self.calculate_recovery_time(squash_amount),
        }
    }
    
    fn calculate_recovery_time(&self, initial_deformation: f32) -> f32 {
        // Time to recover to 5% of initial deformation
        let target_ratio = 0.05;
        (target_ratio.ln() / (-self.recovery_speed)).max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct DeformationResult {
    pub squash: f32,
    pub stretch: f32,
    pub recovery_time: f32,
}
```

### Button Interaction System

```rust
#[derive(Debug, Clone)]
pub struct InteractiveButton {
    pub id: Uuid,
    pub bounds: egui::Rect,
    pub base_size: egui::Vec2,
    pub current_deformation: DeformationResult,
    pub interaction_type: InteractionType,
    pub press_time: Option<std::time::Instant>,
    pub is_pressed: bool,
    pub hover_state: HoverState,
    pub tactile_feedback: TactileFeedback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverState {
    None,
    Hovering,
    Pressing,
    Releasing,
}

#[derive(Debug, Clone)]
pub struct TactileFeedback {
    pub haptic_enabled: bool,
    pub visual_enabled: bool,
    pub audio_enabled: bool,
    pub intensity_multiplier: f32,
    pub response_curve: ResponseCurve,
}

impl InteractiveButton {
    pub fn new(id: Uuid, bounds: egui::Rect, interaction_type: InteractionType) -> Self {
        Self {
            id,
            bounds,
            base_size: bounds.size(),
            current_deformation: DeformationResult { squash: 0.0, stretch: 0.0, recovery_time: 0.0 },
            interaction_type,
            press_time: None,
            is_pressed: false,
            hover_state: HoverState::None,
            tactile_feedback: TactileFeedback::default(),
        }
    }
    
    pub fn handle_interaction(&mut self, interaction: &InteractionEvent, engine: &SquashStretchEngine) -> InteractionResponse {
        match interaction.event_type {
            InteractionEventType::Press => {
                self.is_pressed = true;
                self.press_time = Some(std::time::Instant::now());
                self.hover_state = HoverState::Pressing;
                
                // Calculate deformation
                let pressure = self.calculate_pressure(interaction.pressure);
                let time = 0.0; // Initial press
                self.current_deformation = engine.calculate_deformation(pressure, time);
                
                // Generate feedback
                self.generate_interaction_feedback(interaction)
            },
            InteractionEventType::Release => {
                self.is_pressed = false;
                self.hover_state = HoverState::Releasing;
                
                // Start recovery animation
                let recovery_time = self.current_deformation.recovery_time;
                
                InteractionResponse {
                    feedback_type: FeedbackType::Release,
                    haptic_pattern: HapticPattern::GentleTap,
                    visual_effect: VisualEffect::ButtonStretch,
                    sound_effect: SoundEffect::SoftClick,
                    duration: Duration::from_millis((recovery_time * 1000.0) as u64),
                }
            },
            InteractionEventType::Hover => {
                if !self.is_pressed {
                    self.hover_state = HoverState::Hovering;
                }
                
                InteractionResponse {
                    feedback_type: FeedbackType::Hover,
                    haptic_pattern: HapticPattern::QuickVibration,
                    visual_effect: VisualEffect::ColorShift,
                    sound_effect: SoundEffect::Hum,
                    duration: Duration::from_millis(100),
                }
            },
            InteractionEventType::Move => {
                // Handle drag or move interaction
                self.handle_move_interaction(interaction)
            },
        }
    }
    
    pub fn update_animation(&mut self, delta_time: f32, engine: &SquashStretchEngine) {
        if self.hover_state == HoverState::Releasing {
            // Continue recovery animation
            let time_since_release = self.press_time
                .map(|pt| pt.elapsed().as_secs_f32())
                .unwrap_or(0.0);
            
            let pressure = 0.0; // No pressure during recovery
            self.current_deformation = engine.calculate_deformation(pressure, time_since_release);
            
            // Check if recovery is complete
            if time_since_release > self.current_deformation.recovery_time {
                self.hover_state = HoverState::None;
                self.current_deformation = DeformationResult { squash: 0.0, stretch: 0.0, recovery_time: 0.0 };
            }
        }
    }
    
    pub fn render(&self, ui: &mut egui::Ui) -> egui::Response {
        let painter = ui.painter();
        
        // Calculate deformed bounds
        let deformed_bounds = self.calculate_deformed_bounds();
        
        // Render button with deformation
        self.render_deformed_button(painter, deformed_bounds);
        
        // Handle input
        ui.allocate_rect(deformed_bounds, egui::Sense::click_and_drag())
    }
    
    fn calculate_deformed_bounds(&self) -> egui::Rect {
        let center = self.bounds.center();
        let base_size = self.base_size;
        
        // Apply squash (vertical compression, horizontal expansion)
        let squashed_height = base_size.y * (1.0 - self.current_deformation.squash);
        let squashed_width = base_size.x * (1.0 + self.current_deformation.squash * 0.5); // Volume conservation
        
        // Apply stretch (horizontal expansion, vertical compression)
        let stretched_width = squashed_width * (1.0 + self.current_deformation.stretch);
        let stretched_height = squashed_height * (1.0 - self.current_deformation.stretch * 0.5);
        
        let final_size = egui::Vec2::new(stretched_width, stretched_height);
        
        egui::Rect::from_center_size(center, final_size)
    }
    
    fn render_deformed_button(&self, painter: &egui::Painter, bounds: egui::Rect) {
        // Base button appearance
        let base_color = self.get_base_color();
        let border_color = self.get_border_color();
        
        // Apply deformation to color
        let deformed_color = self.apply_deformation_to_color(base_color);
        
        // Render button shape
        let rounding = 5.0; // Industrial aesthetic
        painter.rect_filled(bounds, rounding, deformed_color);
        painter.rect_stroke(bounds, rounding, egui::Stroke::new(2.0, border_color));
        
        // Render visual effects
        self.render_visual_effects(painter, bounds);
        
        // Render content
        self.render_content(painter, bounds);
    }
    
    fn apply_deformation_to_color(&self, base_color: egui::Color32) -> egui::Color32 {
        let deformation_intensity = (self.current_deformation.squash + self.current_deformation.stretch).min(1.0);
        
        // Darken color when compressed
        let compression_factor = 1.0 - (self.current_deformation.squash * 0.3);
        
        // Brighten color when stretched
        let stretch_factor = 1.0 + (self.current_deformation.stretch * 0.2);
        
        let combined_factor = compression_factor * stretch_factor;
        
        egui::Color32::from_rgba_unmultiplied(
            (base_color.r() as f32 * combined_factor) as u8,
            (base_color.g() as f32 * combined_factor) as u8,
            (base_color.b() as f32 * combined_factor) as u8,
            base_color.a(),
        )
    }
}
```

### Micro-Interaction Implementations

```rust
impl ShepherdHandSystem {
    pub fn handle_pet_interaction(&self, slime: &mut SlimeGenome, pressure: f32) -> InteractionResult {
        // Petting increases affection
        let affection_increase = pressure * 0.1; // Scale by pressure
        slime.affection = (slime.affection + affection_increase).min(1.0);
        
        // Generate feedback
        let feedback = self.interaction_types.get(&InteractionType::Pet).unwrap();
        
        InteractionResult {
            success: true,
            slime_response: SlimeResponse::Happy,
            feedback: feedback.clone(),
            stat_changes: vec![
                StatChange::Affection(affection_increase),
                StatChange::Charisma(affection_increase * 0.5), // Affection affects CHM
            ],
        }
    }
    
    pub fn handle_feed_interaction(&self, slime: &mut SlimeGenome, resource: ResourceType, amount: u64) -> InteractionResult {
        // Feeding affects different stats based on resource type
        let stat_changes = match resource {
            ResourceType::Biomass => vec![
                StatChange::Health(amount as f32 * 0.1),
                StatChange::Energy(amount as f32 * 0.05),
            ],
            ResourceType::Scrap => vec![
                StatChange::Strength(amount as f32 * 0.02),
                StatChange::Defense(amount as f32 * 0.03),
            ],
            ResourceType::Energy => vec![
                StatChange::Energy(amount as f32 * 0.2),
                StatChange::Speed(amount as f32 * 0.1),
            ],
            ResourceType::Research => vec![
                StatChange::Intelligence(amount as f32 * 0.05),
                StatChange::Experience(amount as f32 * 0.1),
            ],
        };
        
        // Apply stat changes
        for change in &stat_changes {
            slime.apply_stat_change(change);
        }
        
        // Generate feedback
        let feedback = self.interaction_types.get(&InteractionType::Feed).unwrap();
        
        InteractionResult {
            success: true,
            slime_response: SlimeResponse::Satisfied,
            feedback: feedback.clone(),
            stat_changes,
        }
    }
    
    pub fn handle_splice_interaction(&self, parent_a: &SlimeGenome, parent_b: &SlimeGenome) -> InteractionResult {
        // Splicing creates new slime
        let offspring = self.create_offspring(parent_a, parent_b);
        
        // Generate feedback
        let feedback = self.interaction_types.get(&InteractionType::Splice).unwrap();
        
        InteractionResult {
            success: true,
            slime_response: SlimeResponse::Excited,
            feedback: feedback.clone(),
            stat_changes: vec![
                StatChange::NewSlime(offspring.id),
                StatChange::GenerationChange(offspring.generation),
            ],
        }
    }
    
    pub fn handle_play_interaction(&self, slime: &mut SlimeGenome, play_type: PlayType) -> InteractionResult {
        let stat_changes = match play_type {
            PlayType::Chase => vec![
                StatChange::Speed(0.1),
                StatChange::Agility(0.1),
                StatChange::Happiness(0.2),
            ],
            PlayType::Toss => vec![
                StatChange::Strength(0.05),
                StatChange::Coordination(0.1),
                StatChange::Trust(0.15),
            ],
            PlayType::Hide => vec![
                StatChange::Stealth(0.15),
                StatChange::Perception(0.1),
                StatChange::Curiosity(0.2),
            ],
        };
        
        for change in &stat_changes {
            slime.apply_stat_change(change);
        }
        
        let feedback = self.interaction_types.get(&InteractionType::Play).unwrap();
        
        InteractionResult {
            success: true,
            slime_response: SlimeResponse::Playful,
            feedback: feedback.clone(),
            stat_changes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InteractionResult {
    pub success: bool,
    pub slime_response: SlimeResponse,
    pub feedback: InteractionFeedback,
    pub stat_changes: Vec<StatChange>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlimeResponse {
    Happy,
    Satisfied,
    Excited,
    Playful,
    Scared,
    Angry,
    Curious,
    Sleepy,
}

#[derive(Debug, Clone)]
pub enum StatChange {
    Health(f32),
    Energy(f32),
    Strength(f32),
    Agility(f32),
    Intelligence(f32),
    Affection(f32),
    Charisma(f32),
    Happiness(f32),
    Trust(f32),
    Curiosity(f32),
    Speed(f32),
    Stealth(f32),
    Perception(f32),
    Coordination(f32),
    NewSlime(Uuid),
    GenerationChange(u32),
    Experience(f32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayType {
    Chase,
    Toss,
    Hide,
}
```

## Implementation Tasks

### Core System Development

1. **Implement SquashStretchEngine**: Create deformation calculation system
2. **Build InteractiveButton**: Tactile button component with visual feedback
3. **Create ShepherdHandSystem**: Central interaction management
4. **Implement HapticEngine**: Device-specific haptic feedback
5. **Develop VisualFeedbackEngine**: Visual effects for interactions

### Integration Points

1. **UI Integration**: Replace standard egui buttons with interactive buttons
2. **Slime Interaction**: Connect interaction system to slime entities
3. **Audio System**: Integrate sound effects with interactions
4. **Settings System**: Allow customization of tactile sensitivity

### Performance Considerations

1. **Animation Performance**: Ensure smooth 60fps deformation animations
2. **Memory Management**: Efficient handling of multiple simultaneous interactions
3. **Input Latency**: Minimize delay between touch and feedback
4. **Battery Optimization**: Efficient haptic feedback to preserve battery

## Validation Criteria

- [ ] All interactions provide immediate visual and haptic feedback
- [ ] Squash and stretch animations feel natural and responsive
- [ ] Different interaction types have distinct feedback patterns
- [ ] System performs smoothly with multiple simultaneous interactions
- [ ] Tactile feedback enhances rather than distracts from gameplay
- [ ] Visual deformation accurately reflects interaction pressure

## Future Enhancements

1. **Advanced Haptics**: More sophisticated haptic patterns for different materials
2. **Pressure Sensitivity**: Support for pressure-sensitive touch devices
3. **Customization**: User-configurable feedback intensity and patterns
4. **Environmental Feedback**: Context-aware feedback based on slime mood and environment
5. **Multi-touch**: Support for complex multi-touch interactions

The "Shepherd's Hand" interaction system creates a tactile, industrial feel for the Command Deck that makes every interaction feel physical and meaningful, enhancing the player's connection to their slimes while maintaining the mathematical visual identity of the game.
