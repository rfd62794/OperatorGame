# Shepherd Garden Wandering System

> **Status:** HABITAT SIMULATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-020, RENDERED_MATH_VISUAL_IDENTITY.md, SPEC.md §3

## Overview

The Shepherd Garden transforms the static slime roster into a living, breathing habitat where slimes wander according to their personality axes. This meso-view of the ship's internal habitat creates an emotional connection between the Astronaut and their slimes through petting, observation, and natural behavioral patterns.

## Garden Architecture

### Habitat Space Definition

```rust
#[derive(Debug, Clone)]
pub struct ShepherdGarden {
    pub bounds: egui::Rect,
    pub wandering_slimes: Vec<WanderingSlime>,
    pub environment: GardenEnvironment,
    pub interaction_zones: Vec<InteractionZone>,
    pub social_dynamics: SocialDynamics,
    pub time_system: GardenTimeSystem,
}

#[derive(Debug, Clone)]
pub struct WanderingSlime {
    pub slime_id: Uuid,
    pub position: egui::Vec2,        // Normalized 0.0-1.0 within garden
    pub velocity: egui::Vec2,
    pub target_position: Option<egui::Vec2>,
    pub personality_state: PersonalityState,
    pub social_state: SocialState,
    pub mood_state: MoodState,
    pub behavior_tree: BehaviorTree,
    pub current_action: SlimeAction,
    pub animation_state: AnimationState,
}

#[derive(Debug, Clone)]
pub struct PersonalityState {
    pub curiosity: f32,            // 0.0-1.0 - drives exploration
    pub shyness: f32,             // 0.0-1.0 - drives avoidance
    pub playfulness: f32,          // 0.0-1.0 - drives social interaction
    pub territoriality: f32,       // 0.0-1.0 - drives space defense
    pub energy: f32,               // 0.0-1.0 - affects movement speed
    pub affection: f32,            // 0.0-1.0 - affects interaction willingness
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoodState {
    Happy,
    Playful,
    Curious,
    Sleepy,
    Hungry,
    Scared,
    Angry,
    Lonely,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlimeAction {
    Wandering,
    Seeking,
    Playing,
    Resting,
    Hiding,
    Interacting { target: Uuid },
    Eating,
    Sleeping,
    Exploring,
}

impl ShepherdGarden {
    pub fn new(bounds: egui::Rect) -> Self {
        Self {
            bounds,
            wandering_slimes: Vec::new(),
            environment: GardenEnvironment::new(bounds),
            interaction_zones: Vec::new(),
            social_dynamics: SocialDynamics::new(),
            time_system: GardenTimeSystem::new(),
        }
    }
    
    pub fn add_slime(&mut self, slime: &SlimeGenome) {
        let wandering_slime = WanderingSlime::from_slime(slime, &self.bounds);
        self.wandering_slimes.push(wandering_slime);
    }
    
    pub fn update(&mut self, delta_time: f32, game_state: &GameState) {
        // Update time
        self.time_system.update(delta_time);
        
        // Update environment
        self.environment.update(&self.time_system);
        
        // Update social dynamics
        self.social_dynamics.update(&mut self.wandering_slimes, delta_time);
        
        // Update each slime
        for slime in &mut self.wandering_slimes {
            slime.update(delta_time, &self.environment, &self.social_dynamics);
        }
        
        // Handle interactions
        self.handle_interactions(game_state);
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, ctx: &mut egui::Context) {
        // Render garden background
        self.render_background(ui, ctx);
        
        // Render wandering slimes
        for slime in &self.wandering_slimes {
            slime.render(ui, ctx, &self.bounds);
        }
        
        // Render interaction zones
        for zone in &self.interaction_zones {
            zone.render(ui, ctx, &self.bounds);
        }
    }
}
```

### Boids-Style Steering Engine

```rust
#[derive(Debug, Clone)]
pub struct SteeringEngine {
    pub separation_radius: f32,
    pub alignment_radius: f32,
    pub cohesion_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub wander_weight: f32,
    pub personality_modifiers: PersonalityModifiers,
}

#[derive(Debug, Clone)]
pub struct PersonalityModifiers {
    pub curiosity_modifier: f32,
    pub shyness_modifier: f32,
    pub playfulness_modifier: f32,
    pub territoriality_modifier: f32,
}

#[derive(Debug, Clone)]
pub struct SteeringForce {
    pub separation: egui::Vec2,
    pub alignment: egui::Vec2,
    pub cohesion: egui::Vec2,
    pub wander: egui::Vec2,
    pub personality: egui::Vec2,
    pub environmental: egui::Vec2,
    pub social: egui::Vec2,
}

impl SteeringEngine {
    pub fn new() -> Self {
        Self {
            separation_radius: 0.1,    // 10% of garden width
            alignment_radius: 0.15,    // 15% of garden width
            cohesion_radius: 0.2,      // 20% of garden width
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            wander_weight: 0.5,
            personality_modifiers: PersonalityModifiers::new(),
        }
    }
    
    pub fn calculate_steering_force(
        &self,
        slime: &WanderingSlime,
        all_slimes: &[WanderingSlime],
        environment: &GardenEnvironment
    ) -> SteeringForce {
        let separation = self.calculate_separation(slime, all_slimes);
        let alignment = self.calculate_alignment(slime, all_slimes);
        let cohesion = self.calculate_cohesion(slime, all_slimes);
        let wander = self.calculate_wander(slime);
        let personality = self.calculate_personality_steering(slime);
        let environmental = self.calculate_environmental_steering(slime, environment);
        let social = self.calculate_social_steering(slime, all_slimes);
        
        SteeringForce {
            separation,
            alignment,
            cohesion,
            wander,
            personality,
            environmental,
            social,
        }
    }
    
    fn calculate_separation(&self, slime: &WanderingSlime, all_slimes: &[WanderingSlime]) -> egui::Vec2 {
        let mut separation_force = egui::Vec2::ZERO;
        
        for other in all_slimes {
            if other.slime_id == slime.slime_id {
                continue;
            }
            
            let distance = slime.position.distance(other.position);
            
            if distance < self.separation_radius && distance > 0.0 {
                // Calculate repulsion force
                let diff = slime.position - other.position;
                let force_magnitude = (self.separation_radius - distance) / distance;
                separation_force += diff.normalize() * force_magnitude;
            }
        }
        
        separation_force * self.separation_weight
    }
    
    fn calculate_alignment(&self, slime: &WanderingSlime, all_slimes: &[WanderingSlime]) -> egui::Vec2 {
        let mut alignment_force = egui::Vec2::ZERO;
        let mut neighbor_count = 0;
        
        for other in all_slimes {
            if other.slime_id == slime.slime_id {
                continue;
            }
            
            let distance = slime.position.distance(other.position);
            
            if distance < self.alignment_radius && distance > 0.0 {
                // Calculate alignment force
                alignment_force += other.velocity;
                neighbor_count += 1;
            }
        }
        
        if neighbor_count > 0 {
            alignment_force /= neighbor_count as f32;
            alignment_force -= slime.velocity;
        }
        
        alignment_force * self.alignment_weight
    }
    
    fn calculate_cohesion(&self, slime: &WanderingSlime, all_slimes: &[WanderingSlime]) -> egui::Vec2 {
        let mut center_of_mass = egui::Vec2::ZERO;
        let mut neighbor_count = 0;
        
        for other in all_slimes {
            if other.slime_id == slime.slime_id {
                continue;
            }
            
            let distance = slime.position.distance(other.position);
            
            if distance < self.cohesion_radius {
                center_of_mass += other.position;
                neighbor_count += 1;
            }
        }
        
        if neighbor_count > 0 {
            center_of_mass /= neighbor_count as f32;
            let cohesion_force = (center_of_mass - slime.position) * 0.01;
            cohesion_force * self.cohesion_weight
        } else {
            egui::Vec2::ZERO
        }
    }
    
    fn calculate_wander(&self, slime: &WanderingSlime) -> egui::Vec2 {
        // Random wandering force
        let angle = rand::random::<f32>() * std::f32::consts::TAU;
        let wander_force = egui::Vec2::new(angle.cos(), angle.sin()) * 0.01;
        
        wander_force * self.wander_weight
    }
    
    fn calculate_personality_steering(&self, slime: &WanderingSlime) -> egui::Vec2 {
        let mut personality_force = egui::Vec2::ZERO;
        
        // Curiosity drives exploration
        if slime.personality_state.curiosity > 0.5 {
            let exploration_direction = self.get_exploration_direction(slime);
            personality_force += exploration_direction * slime.personality_state.curiosity;
        }
        
        // Shyness drives avoidance
        if slime.personality_state.shyness > 0.5 {
            let avoidance_direction = self.get_avoidance_direction(slime);
            personality_force += avoidance_direction * slime.personality_state.shyness;
        }
        
        // Playfulness drives social seeking
        if slime.personality_state.playfulness > 0.5 {
            let social_direction = self.get_social_direction(slime);
            personality_force += social_direction * slime.personality_state.playfulness;
        }
        
        // Territoriality drives space defense
        if slime.personality_state.territoriality > 0.5 {
            let territorial_direction = self.get_territorial_direction(slime);
            personality_force += territorial_direction * slime.personality_state.territoriality;
        }
        
        personality_force
    }
    
    fn get_exploration_direction(&self, slime: &WanderingSlime) -> egui::Vec2 {
        // Find unexplored area
        let exploration_target = self.find_unexplored_area(slime);
        
        if let Some(target) = exploration_target {
            (target - slime.position).normalize()
        } else {
            // Random exploration
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            egui::Vec2::new(angle.cos(), angle.sin())
        }
    }
    
    fn get_avoidance_direction(&self, slime: &WanderingSlime) -> egui::Vec2 {
        // Find direction away from other slimes
        let mut avoidance_force = egui::Vec2::ZERO;
        
        for other in slime.nearby_slimes.iter().take(3) {
            let diff = slime.position - other.position;
            let distance = diff.length();
            
            if distance > 0.0 && distance < 0.2 {
                avoidance_force += diff.normalize() / distance;
            }
        }
        
        if avoidance_force.length() > 0.0 {
            avoidance_force.normalize()
        } else {
            // Move toward corners
            let corner = self.find_nearest_corner(slime.position);
            (corner - slime.position).normalize()
        }
    }
    
    fn get_social_direction(&self, slime: &WanderingSlime) -> egui::Vec2 {
        // Find nearest friendly slime
        if let Some(friend) = self.find_nearest_friend(slime) {
            (friend.position - slime.position).normalize()
        } else {
            egui::Vec2::ZERO
        }
    }
    
    fn get_territorial_direction(&self, slime: &WanderingSlime) -> egui::Vec2 {
        // Move toward center of territory
        let territory_center = self.calculate_territory_center(slime);
        
        if slime.position.distance(territory_center) > 0.1 {
            (territory_center - slime.position).normalize()
        } else {
            // Patrol territory
            let angle = slime.position.angle() + std::f32::consts::PI / 4.0;
            egui::Vec2::new(angle.cos(), angle.sin())
        }
    }
}
```

### Personality-Based Behavior

```rust
impl WanderingSlime {
    pub fn from_slime(slime: &SlimeGenome, garden_bounds: &egui::Rect) -> Self {
        let personality_state = Self::extract_personality_from_genome(slime);
        let initial_position = Self::generate_initial_position(garden_bounds);
        
        Self {
            slime_id: slime.id,
            position: initial_position,
            velocity: egui::Vec2::ZERO,
            target_position: None,
            personality_state,
            social_state: SocialState::new(),
            mood_state: MoodState::Happy,
            behavior_tree: BehaviorTree::new(&personality_state),
            current_action: SlimeAction::Wandering,
            animation_state: AnimationState::new(),
        }
    }
    
    fn extract_personality_from_genome(slime: &SlimeGenome) -> PersonalityState {
        // Extract personality from genome traits
        let curiosity = slime.curiosity;
        let shyness = slime.shyness;
        let playfulness = (slime.energy + slime.affection) / 2.0;
        let territoriality = match slime.culture {
            Culture::Ember => 0.8,      // High territoriality
            Culture::Crystal => 0.7,    // High territoriality
            Culture::Marsh => 0.2,      // Low territoriality
            Culture::Tide => 0.3,       // Low territoriality
            Culture::Gale => 0.5,       // Medium territoriality
            _ => 0.4,                   // Default
        };
        let energy = slime.base_spd as f32 / 100.0;
        let affection = slime.affection;
        
        PersonalityState {
            curiosity: curiosity.clamp(0.0, 1.0),
            shyness: shyness.clamp(0.0, 1.0),
            playfulness: playfulness.clamp(0.0, 1.0),
            territoriality: territoriality.clamp(0.0, 1.0),
            energy: energy.clamp(0.0, 1.0),
            affection: affection.clamp(0.0, 1.0),
        }
    }
    
    pub fn update(&mut self, delta_time: f32, environment: &GardenEnvironment, social: &SocialDynamics) {
        // Update mood based on personality and environment
        self.update_mood(environment, social);
        
        // Update behavior tree
        self.behavior_tree.update(&self.personality_state, &self.mood_state);
        
        // Calculate steering forces
        let steering_force = self.calculate_steering_force(environment, social);
        
        // Update velocity and position
        self.velocity += steering_force * delta_time;
        
        // Apply speed limit based on energy
        let max_speed = 0.1 * self.personality_state.energy;
        if self.velocity.length() > max_speed {
            self.velocity = self.velocity.normalize() * max_speed;
        }
        
        // Update position
        self.position += self.velocity * delta_time;
        
        // Keep within bounds
        self.constrain_to_bounds();
        
        // Update current action
        self.current_action = self.determine_current_action();
        
        // Update animation state
        self.animation_state.update(delta_time, &self.current_action);
    }
    
    fn update_mood(&mut self, environment: &GardenEnvironment, social: &SocialDynamics) {
        let mood_influences = vec![
            self.calculate_environmental_mood(environment),
            self.calculate_social_mood(social),
            self.calculate_personality_mood(),
        ];
        
        let total_influence: f32 = mood_influences.iter().sum();
        
        // Update mood based on influences
        self.mood_state = match total_influence {
            x if x > 0.8 => MoodState::Happy,
            x if x > 0.6 => MoodState::Playful,
            x if x > 0.4 => MoodState::Curious,
            x if x > 0.2 => MoodState::Sleepy,
            x if x > 0.0 => MoodState::Hungry,
            x if x > -0.2 => MoodState::Lonely,
            x if x > -0.5 => MoodState::Scared,
            _ => MoodState::Angry,
        };
    }
    
    fn calculate_environmental_mood(&self, environment: &GardenEnvironment) -> f32 {
        let mut mood_influence = 0.0;
        
        // Temperature affects mood
        let temperature_preference = match self.slime_culture {
            Culture::Ember => 0.8,      // Likes heat
            Culture::Tundra => -0.8,     // Likes cold
            Culture::Marsh => 0.2,       // Likes humidity
            Culture::Tide => 0.0,        // Neutral
            _ => 0.0,
        };
        
        mood_influence += temperature_preference * environment.temperature_factor;
        
        // Light affects mood
        let light_preference = match self.mood_state {
            MoodState::Happy => 0.2,
            MoodState::Playful => 0.3,
            MoodState::Curious => 0.1,
            MoodState::Sleepy => -0.2,
            MoodState::Scared => -0.3,
            _ => 0.0,
        };
        
        mood_influence += light_preference * environment.light_factor;
        
        mood_influence.clamp(-1.0, 1.0)
    }
    
    fn calculate_social_mood(&self, social: &SocialDynamics) -> f32 {
        let social_connections = social.get_connections(self.slime_id);
        
        if social_connections.is_empty() {
            -0.3 // Lonely
        } else {
            let positive_interactions = social_connections.iter()
                .filter(|conn| conn.interaction_type == InteractionType::Play)
                .count() as f32;
            
            let total_interactions = social_connections.len() as f32;
            
            if total_interactions > 0.0 {
                positive_interactions / total_interactions * 0.5
            } else {
                0.0
            }
        }
    }
    
    fn determine_current_action(&self) -> SlimeAction {
        match self.mood_state {
            MoodState::Playful => SlimeAction::Playing,
            MoodState::Curious => SlimeAction::Exploring,
            MoodState::Sleepy => SlimeAction::Resting,
            MoodState::Hungry => SlimeAction::Eating,
            MoodState::Scared => SlimeAction::Hiding,
            MoodState::Angry => SlimeAction::Wandering, // Angry slimes pace
            MoodState::Lonely => SlimeAction::Seeking,
            MoodState::Happy => SlimeAction::Wandering,
        }
    }
    
    pub fn handle_interaction(&mut self, interaction: &GardenInteraction) -> InteractionResult {
        match interaction.interaction_type {
            InteractionType::Pet => self.handle_pet(interaction),
            InteractionType::Feed => self.handle_feed(interaction),
            InteractionType::Play => self.handle_play(interaction),
            InteractionType::Inspect => self.handle_inspect(interaction),
        }
    }
    
    fn handle_pet(&mut self, interaction: &GardenInteraction) -> InteractionResult {
        // Petting increases affection
        let affection_increase = 0.1 * interaction.intensity;
        self.personality_state.affection = (self.personality_state.affection + affection_increase).min(1.0);
        
        // Update mood
        self.mood_state = MoodState::Happy;
        
        // Generate positive response
        InteractionResult {
            success: true,
            slime_response: SlimeResponse::Happy,
            affection_change: affection_increase,
            mood_change: MoodState::Happy,
            visual_effect: VisualEffect::HappyBounce,
        }
    }
    
    fn render(&self, ui: &mut egui::Ui, ctx: &mut egui::Context, garden_bounds: &egui::Rect) {
        // Calculate screen position
        let screen_pos = egui::Pos2::new(
            garden_bounds.min.x + self.position.x * garden_bounds.width(),
            garden_bounds.min.y + self.position.y * garden_bounds.height(),
        );
        
        // Render slime with animations
        self.render_slime(ctx, screen_pos);
        
        // Render mood indicator
        self.render_mood_indicator(ui, screen_pos);
        
        // Render interaction feedback
        if self.has_recent_interaction() {
            self.render_interaction_feedback(ctx, screen_pos);
        }
    }
    
    fn render_slime(&self, ctx: &mut egui::Context, screen_pos: egui::Pos2) {
        // Get slime data from game state
        let slime = get_slime_by_id(self.slime_id).unwrap();
        
        // Render using rendered math system
        let size = 30.0; // Base size
        let time = ctx.input(|i| i.time).unwrap_or(0.0);
        
        // Apply deformation based on current action
        let deformation = match self.current_action {
            SlimeAction::Playing => BodyDeformation {
                compression_factor: 0.05,
                stretch_factor: 0.1,
                wobble_amplitude: 0.15,
                wobble_frequency: 2.0,
                breathing_phase: time * 1.0,
            },
            SlimeAction::Sleeping => BodyDeformation {
                compression_factor: 0.1,
                stretch_factor: 0.05,
                wobble_amplitude: 0.02,
                wobble_frequency: 0.5,
                breathing_phase: time * 0.3,
            },
            _ => BodyDeformation {
                compression_factor: 0.0,
                stretch_factor: 0.0,
                wobble_amplitude: 0.05,
                wobble_frequency: 1.0,
                breathing_phase: time * 0.5,
            },
        };
        
        // Render with visual identity system
        render_slime_with_deformation(slime, ctx, screen_pos, size, time, deformation);
    }
}
```

## Implementation Tasks

### Core System Development

1. **Create src/ui/garden.rs**: Main garden module
2. **Implement SteeringEngine**: Boids-style movement system
3. **Build PersonalitySystem**: Personality-based behavior
4. **Develop SocialDynamics**: Inter-slime relationships
5. **Create InteractionSystem**: Petting and interaction handling

### Integration Points

1. **UI Integration**: Connect to main Command Deck UI
2. **Game State Integration**: Connect to slime data
3. **Time System**: Synchronize with game time
4. **Audio Integration**: Add sound effects for interactions

### Performance Optimization

1. **Spatial Partitioning**: Efficient neighbor finding
2. **Behavior Caching**: Cache expensive calculations
3. **LOD System**: Level of detail for distant slimes
4. **Batch Updates**: Update slimes in batches

## Validation Criteria

- [ ] Slimes wander according to personality traits
- [ ] Boids steering creates natural movement patterns
- [ ] Petting interactions provide immediate feedback
- [ ] Garden maintains 60fps performance
- [ ] Social dynamics create realistic group behavior
- [ ] System scales to 50+ slimes efficiently

The Shepherd Garden Wandering System creates a living habitat where slimes exhibit natural behaviors based on their personality, providing an emotional connection between the Astronaut and their slimes through observation and interaction.
