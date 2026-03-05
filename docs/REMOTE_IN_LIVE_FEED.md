# Remote-In Live Feed System

> **Status**: REAL-TIME VISUALIZATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-020, SPEC.md §8, DICE_ENGINE.md

## Overview

The Remote-In Live Feed transforms static text logs into dynamic, real-time visualizations of dungeon and race engine activities. This system provides immediate visual feedback for missions, allowing the Astronaut to watch their "Pikmin" squads navigate through nodes in real-time, with interactive D20 combat resolution that takes over the screen during critical moments.

## Live Feed Architecture

### Feed Display System

```rust
#[derive(Debug, Clone)]
pub struct LiveFeedSystem {
    pub active_feeds: Vec<ActiveFeed>,
    pub feed_display: FeedDisplay,
    pub rendering_engine: LiveFeedRenderer,
    pub interaction_handler: FeedInteractionHandler,
    pub audio_system: FeedAudioSystem,
}

#[derive(Debug, Clone)]
pub struct ActiveFeed {
    pub id: Uuid,
    pub feed_type: FeedType,
    pub mission_id: Uuid,
    pub participants: Vec<FeedParticipant>,
    pub current_state: FeedState,
    pub visual_representation: VisualRepresentation,
    pub timeline: Vec<TimelineEvent>,
    pub camera: FeedCamera,
    pub zoom_level: f32,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedType {
    DungeonRun,      // Dungeon engine visualization
    Race,           // Race engine visualization
    Expedition,     // Exploration mission
    Combat,          // Combat encounter
    Research,        // Research activity
    Construction,    // Building/repair
}

#[derive(Debug, Clone)]
pub struct FeedParticipant {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub visual_data: VisualData,
    pub current_position: egui::Vec2,
    pub target_position: Option<egui::Vec2>,
    pub movement_path: Vec<egui::Vec2>,
    pub current_action: ParticipantAction,
    pub status: ParticipantStatus,
    pub health: f32,
    pub energy: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityType {
    Slime,          // Player slime
    Enemy,          // Enemy entity
    NPC,            // Non-player character
    Object,         // Interactive object
    Environment,    // Environmental element
    Effect,         // Visual effect
}

#[derive(Debug, Clone)]
pub struct VisualData {
    pub sprite_id: String,
    pub color: egui::Color32,
    pub size: egui::Vec2,
    pub animation_state: AnimationState,
    pub effects: Vec<VisualEffect>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParticipantAction {
    Moving,
    Attacking,
    Defending,
    Interacting,
    Idle,
    Dead,
    Spawning,
    Despawning,
    UsingAbility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParticipantStatus {
    Active,
    Inactive,
    Hidden,
    Stunned,
    Confused,
    Dead,
}

impl LiveFeedSystem {
    pub fn new() -> Self {
        Self {
            active_feeds: Vec::new(),
            feed_display: FeedDisplay::new(),
            rendering_engine: LiveFeedRenderer::new(),
            interaction_handler: FeedInteractionHandler::new(),
            audio_system: FeedAudioSystem::new(),
        }
    }
    
    pub fn create_feed(&mut self, feed_type: FeedType, mission_id: Uuid) -> Uuid {
        let feed_id = Uuid::new_v4();
        
        let feed = ActiveFeed {
            id: feed_id,
            feed_type,
            mission_id,
            participants: Vec::new(),
            current_state: FeedState::Initializing,
            visual_representation: VisualRepresentation::new(feed_type),
            timeline: Vec::new(),
            camera: FeedCamera::new(),
            zoom_level: 1.0,
            is_active: true,
        };
        
        self.active_feeds.push(feed);
        feed_id
    }
    
    pub fn update_feed(&mut self, feed_id: Uuid, delta_time: f32) -> FeedUpdateResult {
        if let Some(feed) = self.active_feeds.iter_mut().find(|f| f.id == feed_id) {
            // Update visual representation
            feed.visual_representation.update(delta_time);
            
            // Update participants
            for participant in &mut feed.participants {
                participant.update(delta_time);
            }
            
            // Update camera
            feed.camera.update(delta_time, &feed.participants);
            
            // Update timeline
            self.update_timeline(feed);
            
            // Check for significant events
            let events = self.detect_significant_events(feed);
            
            FeedUpdateResult {
                updated: true,
                events,
                participants_updated: feed.participants.len(),
                camera_updated: true,
                timeline_updated: !feed.timeline.is_empty(),
            }
        } else {
            FeedUpdateResult::FeedNotFound
        }
    }
    
    pub fn add_participant(&mut self, feed_id: Uuid, participant: FeedParticipant) -> Result<(), FeedError> {
        if let Some(feed) = self.active_feeds.iter_mut().find(|f| f.id == feed_id) {
            feed.participants.push(participant);
            Ok(())
        } else {
            Err(FeedError::FeedNotFound)
        }
    }
    
    pub fn remove_participant(&mut self, feed_id: Uuid, participant_id: Uuid) -> Result<(), FeedError> {
        if let Some(feed) = self.active_feeds.iter_mut().find(|f| f.id == feed_id) {
            if let Some(index) = feed.participants.iter().position(|p| p.id == participant_id) {
                feed.participants.remove(index);
                Ok(())
            } else {
                Err(FeedError::ParticipantNotFound)
            }
        } else {
            Err(FeedError::FeedNotFound)
        }
    }
    
    pub fn handle_interaction(&mut self, feed_id: Uuid, interaction: FeedInteraction) -> InteractionResult {
        if let Some(feed) = self.active_feeds.iter_mut().find(|f| f.id == feed_id) {
            match interaction.interaction_type {
                InteractionType::Click(position) => {
                    // Handle click on feed
                    let clicked_participant = self.get_participant_at_position(feed, position);
                    
                    if let Some(participant) = clicked_participant {
                        self.interaction_handler.handle_participant_click(feed, participant)
                    } else {
                        InteractionResult::NoTarget
                    }
                },
                InteractionType::Drag(start_pos, end_pos) => {
                    // Handle drag interaction
                    self.interaction_handler.handle_drag(feed, start_pos, end_pos)
                },
                InteractionType::Zoom(direction, factor) => {
                    // Handle zoom
                    feed.camera.handle_zoom(direction, factor);
                    InteractionResult::ZoomChanged
                },
                InteractionType::Pan(offset) => {
                    // Handle pan
                    feed.camera.handle_pan(offset);
                    InteractionResult::PanChanged
                },
            }
        } else {
            InteractionResult::FeedNotFound
        }
    }
    
    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut egui::Context) {
        // Render active feeds
        for feed in &self.active_feeds {
            if feed.is_active {
                self.render_feed(ui, ctx, feed);
            }
        }
        
        // Render feed controls
        self.render_feed_controls(ui);
    }
    
    fn render_feed(&self, ui: &mut egui::Ui, ctx: &mut egui::Context, feed: &ActiveFeed) {
        let available_size = ui.available_size();
        let feed_rect = ui.allocate_exact_size(available_size, egui::Sense::click_and_drag()).1;
        
        // Set up rendering context
        let render_context = RenderContext {
            feed_rect,
            camera: &feed.camera,
            zoom_level: feed.zoom_level,
            time: ctx.input(|i| i.time).unwrap_or(0.0),
            participants: &feed.participants,
        };
        
        // Render background
        self.rendering_engine.render_background(ctx, &render_context);
        
        // Render participants
        for participant in &feed.participants {
            self.render_participant(ctx, participant, &render_context);
        }
        
        // Render effects
        self.rendering_engine.render_effects(ctx, &render_context);
        
        // Render UI overlay
        self.render_feed_overlay(ui, feed);
        
        // Handle input
        if let Some(response) = ui.interact(|ui| {
            // Handle feed interactions
            let pointer_pos = ui.pointer_pos();
            let interaction = FeedInteraction {
                interaction_type: InteractionType::Click(pointer_pos),
                timestamp: std::time::Instant::now(),
            };
            
            self.handle_interaction(feed.id, interaction)
        }).inner {
            match response {
                InteractionResult::ParticipantClicked(participant_id) => {
                    self.show_participant_details(ui, participant_id);
                },
                InteractionResult::NoTarget => {
                    // Clear selection
                },
                _ => {}
            }
        }
    }
    
    fn render_feed_overlay(&self, ui: &mut egui::Ui, feed: &ActiveFeed) {
        // Render mission information
        ui.horizontal(|ui| {
            ui.heading(format!("Mission: {}", feed.mission_id));
            ui.label(format!("Type: {:?}", feed.feed_type));
            ui.label(format!("Participants: {}", feed.participants.len()));
            ui.label(format!("State: {:?}", feed.current_state));
        });
        
        // Render timeline
        ui.separator();
        self.render_timeline(ui, feed);
        
        // Render participant list
        ui.separator();
        self.render_participant_list(ui, feed);
        
        // Render controls
        ui.separator();
        self.render_feed_controls(ui);
    }
}
```

### Dungeon Engine Integration

```rust
#[derive(Debug, Clone)]
pub struct DungeonFeedIntegration {
    pub dungeon_engine: DungeonEngine,
    pub feed_converter: DungeonFeedConverter,
    pub combat_visualizer: CombatVisualizer,
    pub node_renderer: NodeRenderer,
    pub path_visualizer: PathVisualizer,
}

#[derive(Debug, Clone)]
pub struct DungeonFeedConverter {
    pub slime_converter: SlimeToParticipantConverter,
    pub enemy_converter: EnemyToParticipantConverter,
    pub object_converter: ObjectToParticipantConverter,
    pub environment_converter: EnvironmentToParticipantConverter,
}

impl DungeonFeedIntegration {
    pub fn new() -> Self {
        Self {
            dungeon_engine: DungeonEngine::new(),
            feed_converter: DungeonFeedConverter::new(),
            combat_visualizer: CombatVisualizer::new(),
            node_renderer: NodeRenderer::new(),
            path_visualizer::PathVisualizer::new(),
        }
    }
    
    pub fn create_dungeon_feed(&mut self, mission: &DungeonMission, slimes: &[SlimeGenome]) -> Uuid {
        let feed_id = Uuid::new_v4();
        
        // Create feed
        let feed = ActiveFeed {
            id: feed_id,
            feed_type: FeedType::DungeonRun,
            mission_id: mission.id,
            participants: Vec::new(),
            current_state: FeedState::Active,
            visual_representation: VisualRepresentation::new(FeedType::DungeonRun),
            timeline: Vec::new(),
            camera: FeedCamera::new(),
            zoom_level: 1.0,
            is_active: true,
        };
        
        // Convert dungeon nodes to participants
        for node in &mission.dungeon.nodes {
            let participants = self.feed_converter.convert_dungeon_node(node);
            for participant in participants {
                feed.participants.push(participant);
            }
        }
        
        // Convert slimes to participants
        for slime in slimes {
            if let Some(participant) = self.feed_converter.convert_slime(slime) {
                feed.participants.push(participant);
            }
        }
        
        // Convert enemies to participants
        for enemy in &mission.enemies {
            if let Some(participant) = self.feed_converter.convert_enemy(enemy) {
                feed.participants.push(participant);
            }
        }
        
        feed_id
    }
    
    pub fn update_dungeon_feed(&mut self, feed_id: Uuid, delta_time: f32, mission: &mut DungeonMission) -> DungeonUpdateResult {
        // Update dungeon engine
        let dungeon_update = self.dungeon_engine.update(delta_time, mission);
        
        // Update feed based on dungeon changes
        let mut feed_changes = Vec::new();
        
        for event in &dungeon_update.events {
            match event {
                DungeonEvent::SlimeMoved { slime_id, new_position } => {
                    if let Some(participant) = self.find_participant(feed_id, *slime_id) {
                        participant.current_position = self.world_to_feed_position(new_position);
                        participant.current_action = ParticipantAction::Moving;
                        participant.target_position = Some(self.world_to_feed_position(new_position));
                        feed_changes.push(FeedChange::ParticipantMoved {
                            participant_id: *slime_id,
                            old_position: participant.current_position,
                            new_position: participant.current_position,
                        });
                    }
                },
                DungeonEvent::CombatStarted { combat_id, participants } => {
                    // Switch to combat visualization
                    self.combat_visualizer.start_combat(*combat_id, participants.clone());
                    feed.current_state = FeedState::Combat;
                },
                DungeonEvent::CombatEnded { combat_id, result } => {
                    // End combat visualization
                    self.combat_visualizer.end_combat(*combat_id);
                    feed.current_state = FeedState::Active;
                    
                    // Update participant statuses based on combat result
                    for (participant_id, outcome) in result.participant_outcomes {
                        if let Some(participant) = self.find_participant(feed_id, participant_id) {
                            participant.status = match outcome {
                                CombatOutcome::Victorious => ParticipantStatus::Active,
                                CombatOutcome::Defeated => ParticipantStatus::Dead,
                                CombatOutcome::Retreated => ParticipantStatus::Active,
                                CombatOutcome::Captured => ParticipantStatus::Inactive,
                            };
                        }
                    }
                },
                DungeonEvent::NodeDiscovered { node_id, node } => {
                    // Add new node participant
                    if let Some(participant) = self.feed_converter.convert_dungeon_node(node) {
                        feed.participants.push(participant);
                        feed_changes.push(FeedChange::ParticipantAdded {
                            participant_id: participant.id,
                            participant_type: EntityType::Environment,
                        });
                    }
                },
                DungeonEvent::NodeCleared { node_id } => {
                    // Remove node participant
                    if let Some(index) = feed.participants.iter().position(|p| p.id == node_id) {
                        let participant = feed.participants.remove(index);
                        feed_changes.push(FeedChange::ParticipantRemoved {
                            participant_id: participant.id,
                        });
                    }
                },
                _ => {}
            }
        }
        
        DungeonUpdateResult {
            feed_changes,
            dungeon_update,
        }
    }
    
    fn world_to_feed_position(&self, world_pos: egui::Vec2) -> egui::Vec2 {
        // Convert world coordinates to feed coordinates
        // This depends on the specific dungeon layout
        // For now, assume 1:1 mapping with bounds checking
        let feed_bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(1.0, 1.0));
        
        let normalized_x = (world_pos.x + 1.0) / 2.0; // Assuming world coords are -1 to 1
        let normalized_y = (world_pos.y + 1.0) / 2.0;
        
        egui::Vec2::new(
            normalized_x.clamp(0.0, 1.0),
            normalized_y.clamp(0.0, 1.0)
        )
    }
    
    fn find_participant(&self, feed_id: Uuid, participant_id: Uuid) -> Option<&mut FeedParticipant> {
        if let Some(feed) = self.active_feeds.iter_mut().find(|f| f.id == feed_id) {
            feed.participants.iter_mut().find(|p| p.id == participant_id)
        } else {
            None
        }
    }
}
```

### Race Engine Integration

```rust
#[derive(Debug, Clone)]
pub struct RaceFeedIntegration {
    pub race_engine: RaceEngine,
    pub feed_converter: RaceFeedConverter,
    pub track_visualizer: TrackVisualizer,
    pub checkpoint_visualizer: CheckpointVisualizer,
    pub progress_visualizer: ProgressVisualizer,
}

impl RaceFeedIntegration {
    pub fn new() -> Self {
        Self {
            race_engine: RaceEngine::new(),
            feed_converter: RaceFeedConverter::new(),
            track_visualizer: TrackVisualizer::new(),
            checkpoint_visualizer: CheckpointVisualizer::new(),
            progress_visualizer: ProgressVisualizer::new(),
        }
    }
    
    pub fn create_race_feed(&mut self, race: &Race, slimes: &[SlimeGenome]) -> Uuid {
        let feed_id = Uuid::new_v4();
        
        // Create feed
        let feed = ActiveFeed {
            id: feed_id,
            feed_type: FeedType::Race,
            mission_id: race.id,
            participants: Vec::new(),
            current_state: FeedState::Active,
            visual_representation: VisualRepresentation::new(FeedType::Race),
            timeline: Vec::new(),
            camera: FeedCamera::new(),
            zoom_level: 1.0,
            is_active: true,
        };
        
        // Convert track elements to participants
        for checkpoint in &race.checkpoints {
            if let Some(participant) = self.feed_converter.convert_checkpoint(checkpoint) {
                feed.participants.push(participant);
            }
        }
        
        // Convert slimes to participants
        for slime in slimes {
            if let Some(participant) = self.feed_converter.convert_slime(slime) {
                feed.participants.push(participant);
            }
        }
        
        feed_id
    }
    
    pub fn update_race_feed(&mut self, feed_id: Uuid, delta_time: f32, race: &mut Race) -> RaceUpdateResult {
        // Update race engine
        let race_update = self.race_engine.update(delta_time, race);
        
        // Update feed based on race changes
        let mut feed_changes = Vec::new();
        
        for event in &race_update.events {
            match event {
                RaceEvent::SlimeMoved { slime_id, new_position, lap_time } => {
                    if let Some(participant) = self.find_participant(feed_id, *slime_id) {
                        participant.current_position = self.track_to_feed_position(new_position);
                        participant.current_action = ParticipantAction::Moving;
                        participant.target_position = Some(self.track_to_feed_position(new_position));
                        
                        feed_changes.push(FeedChange::ParticipantMoved {
                            participant_id: *slime_id,
                            old_position: participant.current_position,
                            new_position: participant.current_position,
                        });
                    }
                },
                RaceEvent::CheckpointPassed { slime_id, checkpoint_id, position } => {
                    // Update checkpoint visualizer
                    self.checkpoint_visualizer.checkpoint_passed(*checkpoint_id, position);
                    
                    if let Some(participant) = self.find_participant(feed_id, *slime_id) {
                        participant.current_position = self.track_to_feed_position(position);
                        participant.current_action = ParticipantAction::Interacting;
                    }
                },
                RaceEvent::SlimeFinished { slime_id, final_position, final_time } => {
                    if let Some(participant) = self.find_participant(feed_id, *slime_id) {
                        participant.current_position = self.track_to_feed_position(final_position);
                        participant.current_action = ParticipantAction::Idle;
                        participant.status = ParticipantStatus::Active;
                    }
                },
                _ => {}
            }
        }
        
        RaceUpdateResult {
            feed_changes,
            race_update,
        }
    }
    
    fn track_to_feed_position(&self, track_pos: egui::Vec2) -> egui::Vec2 {
        // Convert track coordinates to feed coordinates
        let track_bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(1.0, 1.0));
        
        let normalized_x = track_pos.x / track_bounds.width();
        let normalized_y = track_pos.y / track_bounds.height();
        
        egui::Vec2::new(
            normalized_x.clamp(0.0, 1.0),
            normalized_y.clamp(0.0, 1.0)
        )
    }
}
```

## Interactive D20 Breaks

```rust
#[derive(Debug, Clone)]
pub struct D20BreakSystem {
    pub dice_engine: DiceEngine,
    pub visualizer: D20Visualizer,
    pub audio_system: D20AudioSystem,
    pub current_break: Option<ActiveD20Break>,
    pub break_history: Vec<D20BreakRecord>,
}

#[derive(Debug, Clone)]
pub struct ActiveD20Break {
    pub id: Uuid,
    pub participants: Vec<Uuid>,
    pub roll_type: RollType,
    pub difficulty: f32,
    pub target_number: u8,
    pub current_rolls: Vec<DiceRoll>,
    pub time_limit: Duration,
    pub start_time: std::time::Instant,
    pub is_critical: bool,
    pub outcome: Option<BreakOutcome>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RollType {
    Attack,
    Defense,
    Skill,
    SavingThrow,
    Damage,
    Initiative,
    AbilityCheck,
}

#[derive(Debug, Clone)]
pub struct DiceRoll {
    pub participant_id: Uuid,
    pub roll_type: RollType,
    pub roll_value: u8,
    pub modifier: i8,
    pub final_value: u8,
    pub success: bool,
    roll_time: std::time::Instant,
}

#[derive(Debug, Clone)]
pub enum BreakOutcome {
    Success { participant_id: Uuid, effect: String },
    Failure { participant_id: Uuid, reason: String },
    CriticalSuccess { participant_id: Uuid, effect: String },
    CriticalFailure { participant_id: Uuid, reason: String },
    PartialSuccess { participant_id: Uuid, effect: String },
}

impl D20BreakSystem {
    pub fn new() -> Self {
        Self {
            dice_engine: DiceEngine::new(),
            visualizer: D20Visualizer::new(),
            audio_system: D20AudioSystem::new(),
            current_break: None,
            break_history: Vec::new(),
        }
    }
    
    pub fn initiate_break(
        &mut self,
        participants: Vec<Uuid>,
        roll_type: RollType,
        difficulty: f32,
        target_number: u8,
        time_limit: Duration,
        is_critical: bool
    ) -> Uuid {
        let break_id = Uuid::new_v4();
        
        let active_break = ActiveD20Break {
            id: break_id,
            participants,
            roll_type,
            difficulty,
            target_number,
            current_rolls: Vec::new(),
            time_limit,
            start_time: std::Time::now(),
            is_critical,
            outcome: None,
        };
        
        self.current_break = Some(active_break);
        
        // Take over screen for critical breaks
        if is_critical {
            self.visualizer.take_over_screen();
        }
        
        break_id
    }
    
    pub fn execute_roll(&mut self, participant_id: Uuid, roll_type: RollType, modifier: i8) -> DiceRoll {
        let roll_value = self.dice_engine.roll_d20();
        let final_value = (roll_value as i8 + modifier).max(1).min(20) as u8;
        let success = final_value >= self.get_target_number();
        
        let roll = DiceRoll {
            participant_id,
            roll_type,
            roll_value,
            modifier,
            final_value,
            success,
            roll_time: std::time::Instant::now(),
        };
        
        if let Some(active_break) = &mut self.current_break {
            active_break.current_rolls.push(roll.clone());
            
            // Check if break is complete
            if active_break.current_rolls.len() >= active_break.target_number as usize {
                self.resolve_break();
            }
        }
        
        roll
    }
    
    pub fn resolve_break(&mut self) {
        if let Some(active_break) = self.current_break.take() {
            let outcome = self.calculate_break_outcome(active_break);
            
            // Record in history
            self.break_history.push(D20BreakRecord {
                id: active_break.id,
                participants: active_break.participants.clone(),
                roll_type: active_break.roll_type,
                difficulty: active_break.difficulty,
                target_number: active_break.target_number,
                rolls: active_break.current_rolls.clone(),
                outcome: outcome.clone(),
                duration: active_break.start_time.elapsed(),
                is_critical: active_break.is_critical,
            });
            
            // Release screen takeover
            self.visualizer.release_screen();
            
            // Play sound effect
            self.audio_system.play_break_sound(&outcome);
        }
    }
    
    fn calculate_break_outcome(&self, active_break: &ActiveD20Break) -> BreakOutcome {
        let success_count = active_break.current_rolls.iter()
            .filter(|roll| roll.success)
            .count();
        
        let failure_count = active_break.current_rolls.len() - success_count;
        
        let success_rate = success_count as f32 / active_break.current_rolls.len() as f32;
        
        // Apply difficulty modifier
        let adjusted_success_rate = success_rate * (1.0 - active_break.difficulty);
        
        if adjusted_success_rate >= 0.8 {
            if adjusted_success_rate >= 0.95 {
                BreakOutcome::CriticalSuccess {
                    participant_id: active_break.participants[0], // First participant
                    effect: "Massive success!".to_string(),
                }
            } else {
                BreakOutcome::Success {
                    participant_id: active_break.participants[0],
                    effect: "Success!".to_string(),
                }
            }
        } else if adjusted_success_rate >= 0.5 {
            BreakOutcome::PartialSuccess {
                participant_id: active_break.participants[0],
                effect: "Partial success".to_string(),
            }
        } else if adjusted_success_rate >= 0.05 {
            if failure_count >= active_break.target_number as usize - 1 {
                BreakOutcome::CriticalFailure {
                    participant_id: active_break.participants[0],
                    reason: "Catastrophic failure!".to_string(),
                }
            } else {
                BreakOutcome::Failure {
                    participant_id: active_break.participants[0],
                    reason: "Failure".to_string(),
                }
            }
        } else {
            BreakOutcome::CriticalFailure {
                participant_id: active_break.participants[0],
                reason: "Complete failure!".to_string(),
            }
        }
    }
}
```

## Implementation Tasks

### Core System Development

1. **Create Live Feed System**: Central feed management
2. **Implement Feed Renderer**: Visual rendering system
3. **Build Dungeon Integration**: Connect to dungeon engine
4. **Develop Race Integration**: Connect to race engine
5. **Create D20 Break System**: Interactive combat resolution

### Visual Effects

1. **Implement Visualizer Classes**: D20, combat, movement visualizers
2. **Create Animation System**: Smooth animations and transitions
3. **Build Effect System**: Particle effects and visual feedback
4. **Develop Camera System**: Pan, zoom, and follow mechanics

### Integration Points

1. **Mission System**: Connect to mission management
2. **Dice Engine**: Integrate with D20 system
3. **Audio System**: Add sound effects and music
4. **UI Integration**: Connect to main Command Deck

## Validation Criteria

- [ ] Live feeds provide real-time visualization of missions
- [   ] D20 breaks take over screen during critical moments
- [ ] Visual feedback is immediate and responsive
- [   ] System maintains 60fps during active feeds
- [   ] Participant tracking is accurate and reliable
- [   ] Integration with both dungeon and race engines works correctly

The Remote-In Live Feed System transforms static mission logs into dynamic, engaging visualizations that provide immediate feedback and create an immersive connection between the Astronaut and their slime squads during missions.
