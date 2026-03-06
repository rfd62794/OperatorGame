# Tactical Wiring Sprint Specifications

> **Status:** IMPLEMENTATION ROADMAP v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-021, THREE_COLUMN_LAYOUT_ARCHITECTURE.md, SPEC.md §8

## Overview

The Tactical Wiring Sprint focuses on implementing the "Click-to-Action" loop that connects the UI components into a cohesive, responsive system. This sprint establishes the core interaction patterns that enable the 30-second gameplay cycle and zero-latency interface performance.

## Sprint Objectives

### Primary Goals

1. **Zero-Latency Response**: All UI interactions complete within 100ms
2. **Click-to-Action Flow**: Seamless node → squad → deployment sequence
3. **Real-Time Updates**: Resource and state updates without delays
4. **Input Density**: Support 120+ actions per minute
5. **Mobile Compatibility**: Touch-friendly interactions

### Success Metrics

```rust
pub struct SprintMetrics {
    pub click_response_time_ms: f32,    // Target: <100ms
    pub ui_transition_time_ms: f32,    // Target: 0ms (instant)
    pub actions_per_minute: u32,        // Target: 120+
    pub mobile_touch_accuracy: f32,    // Target: >95%
    pub frame_rate_fps: f32,            // Target: 60fps
}

impl SprintMetrics {
    pub fn is_successful(&self) -> bool {
        self.click_response_time_ms < 100.0 &&
        self.ui_transition_time_ms == 0.0 &&
        self.actions_per_minute >= 120 &&
        self.mobile_touch_accuracy > 0.95 &&
        self.frame_rate_fps >= 60.0
    }
}
```

## Implementation Tasks

### Task 1: Refactor src/ui.rs - 3-Column Layout

#### File Structure

```rust
// src/ui/mod.rs
pub mod layout;
pub mod components;
pub mod input;
pub mod rendering;
pub mod state;

pub use layout::*;
pub use components::*;
pub use input::*;
pub use rendering::*;
pub use state::*;

// src/ui/layout.rs
pub struct LayoutManager {
    pub manifest_panel: ManifestPanel,
    pub command_deck: CommandDeckPanel,
    pub planet_map: PlanetMapPanel,
    pub resource_header: ResourceHeader,
    pub garden_backdrop: GardenBackdrop,
}

// src/ui/components/
pub mod manifest_panel;
pub mod command_deck_panel;
pub mod planet_map_panel;
pub mod resource_header;
pub mod garden_backdrop;

// src/ui/input/
pub mod input_processor;
pub mod event_handler;
pub mod click_tracker;

// src/ui/rendering/
pub mod rendering_pipeline;
pub mod performance_optimizer;
pub mod mobile_adapter;
```

#### Core Layout Implementation

```rust
// src/ui/layout.rs
impl LayoutManager {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            manifest_panel: ManifestPanel::new(),
            command_deck: CommandDeckPanel::new(),
            planet_map: PlanetMapPanel::new(),
            resource_header: ResourceHeader::new(),
            garden_backdrop: GardenBackdrop::new(),
        }
    }
    
    pub fn render(&mut self, ctx: &egui::Context, game_state: &mut GameState) {
        // Render backdrop first
        self.garden_backdrop.render(ctx, game_state);
        
        // Render header with resources
        egui::TopBottomPanel::top("resource_header").show(ctx, |ui| {
            self.resource_header.render(ui, game_state);
        });
        
        // Render main 3-column layout
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_three_columns(ui, game_state);
        });
        
        // Handle overlays
        self.handle_overlays(ctx, game_state);
    }
    
    fn render_three_columns(&mut self, ui: &mut egui::Ui, game_state: &mut GameState) {
        ui.horizontal(|ui| {
            // Left Column - Manifest (30%)
            ui.vertical(|ui| {
                ui.set_width(ui.available_width() * 0.30);
                self.manifest_panel.render(ui, game_state);
            });
            
            ui.separator();
            
            // Center Column - Command Deck (40%)
            ui.vertical(|ui| {
                ui.set_width(ui.available_width() * 0.40);
                self.command_deck_panel.render(ui, game_state);
            });
            
            ui.separator();
            
            // Right Column - Planet Map (30%)
            ui.vertical(|ui| {
                ui.set_width(ui.available_width() * 0.30);
                self.planet_map_panel.render(ui, game_state);
            });
        });
    }
}
```

### Task 2: Wire Dispatch Action - Node Click Flow

#### Click Event System

```rust
// src/ui/input/event_handler.rs
pub struct EventHandler {
    pub click_targets: Vec<ClickTarget>,
    pub action_queue: VecDeque<UIAction>,
    pub response_time_tracker: ResponseTimeTracker,
}

#[derive(Debug, Clone)]
pub struct ClickTarget {
    pub target_type: TargetType,
    pub bounds: egui::Rect,
    pub action: UIAction,
    pub priority: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetType {
    Node(Uuid),
    SlimeCard(Uuid),
    Incubator,
    ExpeditionTab,
    ResearchTab,
    DeployButton,
    SampleButton,
    SelectButton,
}

#[derive(Debug, Clone)]
pub enum UIAction {
    SelectNode { node_id: Uuid },
    SelectSlime { slime_id: Uuid },
    OpenSquadSelection { node_id: Uuid },
    OpenBreedingPreview { slime_id: Uuid },
    SwitchToIncubator,
    SwitchToExpeditions,
    SwitchToResearch,
    StartMixing { parent_a: Uuid, parent_b: Uuid },
    DeploySquad { node_id: Uuid, squad: Vec<Uuid> },
}

impl EventHandler {
    pub fn handle_click(&mut self, click_pos: egui::Pos2) -> Option<UIAction> {
        let start_time = std::time::Instant::now();
        
        // Find clicked target (highest priority first)
        let clicked_target = self.click_targets
            .iter()
            .filter(|target| target.bounds.contains(click_pos))
            .max_by_key(|target| target.priority);
        
        if let Some(target) = clicked_target {
            let action = target.action.clone();
            self.action_queue.push_back(action.clone());
            
            // Track response time
            let response_time = start_time.elapsed().as_millis() as f32;
            self.response_time_tracker.record_response(response_time);
            
            Some(action)
        } else {
            None
        }
    }
    
    pub fn register_click_target(&mut self, target: ClickTarget) {
        self.click_targets.push(target);
    }
    
    pub fn clear_click_targets(&mut self) {
        self.click_targets.clear();
    }
}
```

#### Squad Selection Flow

```rust
// src/ui/components/squad_selection_overlay.rs
pub struct SquadSelectionOverlay {
    pub visible: bool,
    pub target_node: Option<Uuid>,
    pub selected_slimes: Vec<Uuid>,
    pub max_squad_size: usize,
    pub available_slimes: Vec<SlimeGenome>,
}

impl SquadSelectionOverlay {
    pub fn new() -> Self {
        Self {
            visible: false,
            target_node: None,
            selected_slimes: Vec::new(),
            max_squad_size: 3,
            available_slimes: Vec::new(),
        }
    }
    
    pub fn open_for_node(&mut self, node_id: Uuid, game_state: &GameState) {
        self.visible = true;
        self.target_node = Some(node_id);
        self.selected_slimes.clear();
        
        // Filter available slimes (idle and ready)
        self.available_slimes = game_state.roster
            .iter()
            .filter(|slime| matches!(slime.state, OperatorState::Idle))
            .filter(|slime| slime.level >= 2) // Minimum level for deployment
            .cloned()
            .collect();
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, game_state: &mut GameState) -> egui::InnerResponse<Option<UIAction>> {
        if !self.visible {
            return egui::InnerResponse::new(None, *ui.available_rect_before_wrap());
        }
        
        let mut action = None;
        
        // Overlay background
        let overlay_rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(
            overlay_rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180)
        );
        
        // Modal window
        egui::Window::new("Deploy Squad")
            .fixed_size(egui::Vec2::new(400.0, 300.0))
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .collapsible(false)
            .show(ui, |ui| {
                ui.heading(format!("Deploy to Node: {:?}", self.target_node));
                
                ui.separator();
                
                // Selected squad display
                ui.horizontal(|ui| {
                    ui.label("Selected Squad:");
                    for slime_id in &self.selected_slimes {
                        if let Some(slime) = game_state.get_slime(*slime_id) {
                            ui.label(format!("{} ", slime.name));
                        }
                    }
                    ui.label(format!("({}/{})", self.selected_slimes.len(), self.max_squad_size));
                });
                
                ui.separator();
                
                // Available slimes list
                ui.heading("Available Slimes:");
                
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for slime in &self.available_slimes {
                            let is_selected = self.selected_slimes.contains(&slime.id);
                            
                            let row_response = ui.horizontal(|ui| {
                                // Selection checkbox
                                if ui.checkbox(&mut is_selected, "").clicked() {
                                    if is_selected {
                                        self.selected_slimes.push(slime.id);
                                    } else {
                                        self.selected_slimes.retain(|&id| id != slime.id);
                                    }
                                }
                                
                                // Slime info
                                ui.label(&slime.name);
                                ui.label(format!("Lv.{} {}", slime.level, slime.culture));
                                
                                // Stats preview
                                ui.label(format!("ATK:{} SPD:{} CHM:{}", 
                                    slime.effective_stats().strength,
                                    slime.effective_stats().agility,
                                    slime.effective_stats().intelligence
                                ));
                            });
                        }
                    });
                
                ui.separator();
                
                // Action buttons
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.visible = false;
                        action = Some(UIAction::CancelSquadSelection);
                    }
                    
                    if ui.button("Deploy").clicked() && !self.selected_slimes.is_empty() {
                        if let Some(node_id) = self.target_node {
                            action = Some(UIAction::DeploySquad {
                                node_id,
                                squad: self.selected_slimes.clone(),
                            });
                            self.visible = false;
                        }
                    }
                });
            });
        
        egui::InnerResponse::new(action, *ui.available_rect_before_wrap())
    }
}
```

### Task 3: Add Resource Header

#### Resource Display System

```rust
// src/ui/components/resource_header.rs
pub struct ResourceHeader {
    pub resources: ResourceState,
    pub production_rates: ProductionRates,
    pub last_update: std::time::Instant,
    pub animation_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ResourceState {
    pub biomass: u64,
    pub scrap: u64,
    pub ship_integrity: f32,
    pub research_points: u64,
    pub energy: u64,
}

#[derive(Debug, Clone)]
pub struct ProductionRates {
    pub biomass_per_minute: f64,
    pub scrap_per_minute: f64,
    pub integrity_decay_per_minute: f64,
    pub research_per_minute: f64,
    pub energy_per_minute: f64,
}

impl ResourceHeader {
    pub fn new() -> Self {
        Self {
            resources: ResourceState::new(),
            production_rates: ProductionRates::new(),
            last_update: std::time::Instant::now(),
            animation_enabled: true,
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, game_state: &GameState) {
        // Update resources
        self.update_resources(game_state);
        
        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing.x = 12.0;
            
            // Biomass
            self.render_resource(ui, "🌱", "Biomass", self.resources.biomass, self.production_rates.biomass_per_minute, egui::Color32::GREEN);
            
            // Scrap
            self.render_resource(ui, "⚙️", "Scrap", self.resources.scrap, self.production_rates.scrap_per_minute, egui::Color32::LIGHT_BLUE);
            
            // Ship Integrity
            self.render_integrity(ui, "🚀", "Integrity", self.resources.ship_integrity, self.production_rates.integrity_decay_per_minute);
            
            // Research
            self.render_resource(ui, "🔬", "Research", self.resources.research_points, self.production_rates.research_per_minute, egui::Color32::PURPLE);
            
            // Energy
            self.render_resource(ui, "⚡", "Energy", self.resources.energy, self.production_rates.energy_per_minute, egui::Color32::YELLOW);
        });
    }
    
    fn render_resource(&self, ui: &mut egui::Ui, icon: &str, label: &str, amount: u64, rate: f64, color: egui::Color32) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(icon);
                ui.colored_color(color, format!("{}", amount));
            });
            
            if rate != 0.0 {
                let rate_text = if rate > 0.0 {
                    format!("+{:.1}/min", rate)
                } else {
                    format!("{:.1}/min", rate)
                };
                ui.colored_color(color, rate_text);
            }
        });
    }
    
    fn render_integrity(&self, ui: &mut egui::Ui, icon: &str, label: &str, integrity: f32, decay_rate: f64) {
        let integrity_color = if integrity > 0.7 {
            egui::Color32::GREEN
        } else if integrity > 0.3 {
            egui::Color32::YELLOW
        } else {
            egui::Color32::RED
        };
        
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(icon);
                ui.colored_color(integrity_color, format!("{:.1}%", integrity * 100.0));
            });
            
            if decay_rate != 0.0 {
                ui.colored_color(egui::Color32::RED, format!("{:.1}%/min", decay_rate * 100.0));
            }
        });
    }
    
    fn update_resources(&mut self, game_state: &GameState) {
        let now = std::time::Instant::now();
        let delta_time = (now - self.last_update).as_secs_f32();
        self.last_update = now;
        
        // Update based on game state production
        self.resources.biomass = game_state.bank;
        self.resources.scrap = game_state.scrap;
        self.resources.ship_integrity = game_state.ship_integrity;
        self.resources.research_points = game_state.research_points;
        self.resources.energy = game_state.energy;
        
        // Calculate production rates
        self.production_rates = self.calculate_production_rates(game_state);
    }
    
    fn calculate_production_rates(&self, game_state: &GameState) -> ProductionRates {
        // Calculate based on active deployments, research, etc.
        let biomass_rate = game_state.calculate_biomass_production();
        let scrap_rate = game_state.calculate_scrap_production();
        let integrity_decay = game_state.calculate_integrity_decay();
        let research_rate = game_state.calculate_research_production();
        let energy_rate = game_state.calculate_energy_production();
        
        ProductionRates {
            biomass_per_minute: biomass_rate * 60.0,
            scrap_per_minute: scrap_rate * 60.0,
            integrity_decay_per_minute: integrity_decay * 60.0,
            research_per_minute: research_rate * 60.0,
            energy_per_minute: energy_rate * 60.0,
        }
    }
}
```

### Task 4: Garden Backdrop Implementation

#### Animated Slime Background

```rust
// src/ui/components/garden_backdrop.rs
pub struct GardenBackdrop {
    pub idle_slimes: Vec<IdleSlime>,
    pub particle_system: ParticleSystem,
    pub time: f32,
    pub performance_mode: PerformanceMode,
}

#[derive(Debug, Clone)]
pub struct IdleSlime {
    pub slime: SlimeGenome,
    pub position: egui::Vec2, // Normalized 0.0-1.0
    pub velocity: egui::Vec2,
    pub animation_phase: f32,
    pub scale: f32,
    pub opacity: f32,
}

impl GardenBackdrop {
    pub fn new() -> Self {
        Self {
            idle_slimes: Vec::new(),
            particle_system: ParticleSystem::new(),
            time: 0.0,
            performance_mode: PerformanceMode::High,
        }
    }
    
    pub fn render(&mut self, ctx: &egui::Context, game_state: &GameState) {
        let screen_rect = ctx.screen_rect();
        
        // Update idle slimes from game state
        self.update_idle_slimes(game_state);
        
        // Update animation time
        self.time += ctx.input(|i| i.stable_dt).unwrap_or(0.016);
        
        // Render based on performance mode
        match self.performance_mode {
            PerformanceMode::High => self.render_full_effects(ctx, screen_rect),
            PerformanceMode::Medium => self.render_medium_effects(ctx, screen_rect),
            PerformanceMode::Low => self.render_minimal_effects(ctx, screen_rect),
        }
    }
    
    fn update_idle_slimes(&mut self, game_state: &GameState) {
        // Get all idle slimes from game state
        let idle_slimes: Vec<_> = game_state.roster
            .iter()
            .filter(|slime| matches!(slime.state, OperatorState::Idle))
            .cloned()
            .collect();
        
        // Update backdrop slime list
        self.idle_slimes.clear();
        
        for slime in idle_slimes {
            let position = self.calculate_initial_position(&slime);
            let velocity = self.calculate_initial_velocity(&slime);
            
            self.idle_slimes.push(IdleSlime {
                slime,
                position,
                velocity,
                animation_phase: rand::random::<f32>() * std::f32::consts::TAU,
                scale: 0.8 + rand::random::<f32>() * 0.4,
                opacity: 0.3 + rand::random::<f32>() * 0.3,
            });
        }
    }
    
    fn calculate_initial_position(&self, slime: &SlimeGenome) -> egui::Vec2 {
        // Use slime ID to generate consistent position
        let hash = slime.id.as_u128();
        let x = ((hash >> 64) as f32 / u64::MAX as f32) * 0.8 + 0.1; // Keep away from edges
        let y = ((hash as f32) / u64::MAX as f32) * 0.8 + 0.1;
        
        egui::Vec2::new(x, y)
    }
    
    fn calculate_initial_velocity(&self, slime: &SlimeGenome) -> egui::Vec2 {
        // Slow floating movement
        let hash = slime.id.as_u128();
        let vx = ((hash >> 32) as f32 / u32::MAX as f32 - 0.5) * 0.02; // Very slow
        let vy = ((hash >> 16) as f32 / u32::MAX as f32 - 0.5) * 0.02;
        
        egui::Vec2::new(vx, vy)
    }
    
    fn render_full_effects(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        // Render particle effects
        self.particle_system.update_and_render(ctx, screen_rect, self.time);
        
        // Render idle slimes with full animation
        for slime in &mut self.idle_slimes {
            self.update_slime_animation(slime);
            self.render_slime_full(ctx, screen_rect, slime);
        }
    }
    
    fn render_minimal_effects(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        // Only render basic slime positions
        for slime in &self.idle_slimes {
            self.render_slime_minimal(ctx, screen_rect, slime);
        }
    }
    
    fn update_slime_animation(&mut self, slime: &mut IdleSlime) {
        // Update position
        slime.position += slime.velocity;
        
        // Boundary check and bounce
        if slime.position.x < 0.05 || slime.position.x > 0.95 {
            slime.velocity.x *= -1.0;
            slime.position.x = slime.position.x.clamp(0.05, 0.95);
        }
        if slime.position.y < 0.05 || slime.position.y > 0.95 {
            slime.velocity.y *= -1.0;
            slime.position.y = slime.position.y.clamp(0.05, 0.95);
        }
        
        // Update animation phase
        slime.animation_phase += 0.02; // Breathing animation speed
    }
    
    fn render_slime_full(&self, ctx: &egui::Context, screen_rect: egui::Rect, slime: &IdleSlime) {
        let screen_pos = egui::Pos2::new(
            screen_rect.min.x + screen_rect.width() * slime.position.x,
            screen_rect.min.y + screen_rect.height() * slime.position.y,
        );
        
        // Breathing animation
        let breathing = (slime.animation_phase).sin() * 0.1 + 1.0;
        let size = egui::Vec2::new(25.0, 25.0) * breathing * slime.scale;
        
        // Get slime color
        let color = slime.slime.culture.get_rgb_color();
        let slime_color = egui::Color32::from_rgba_unmultiplied(
            color[0],
            color[1],
            color[2],
            (slime.opacity * 255.0) as u8
        );
        
        // Render slime circle
        ctx.painter().circle_filled(screen_pos, size.x / 2.0, slime_color);
        
        // Render culture symbol
        let symbol = slime.slime.culture.get_symbol();
        ctx.painter().text(
            screen_pos,
            egui::Align2::CENTER_CENTER,
            symbol,
            egui::FontId::default(),
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, (slime.opacity * 200.0) as u8),
        );
    }
    
    fn render_slime_minimal(&self, ctx: &egui::Context, screen_rect: egui::Rect, slime: &IdleSlime) {
        let screen_pos = egui::Pos2::new(
            screen_rect.min.x + screen_rect.width() * slime.position.x,
            screen_rect.min.y + screen_rect.height() * slime.position.y,
        );
        
        let size = egui::Vec2::new(15.0, 15.0) * slime.scale;
        
        // Simple colored dot
        let color = slime.slime.culture.get_rgb_color();
        let slime_color = egui::Color32::from_rgba_unmultiplied(
            color[0],
            color[1],
            color[2],
            (slime.opacity * 150.0) as u8
        );
        
        ctx.painter().circle_filled(screen_pos, size.x / 2.0, slime_color);
    }
}
```

## Integration Testing

### Performance Benchmarks

```rust
// tests/ui_performance.rs
#[cfg(test)]
mod ui_performance_tests {
    use super::*;
    
    #[test]
    fn test_click_response_time() {
        let mut event_handler = EventHandler::new();
        let start_time = std::time::Instant::now();
        
        // Simulate click
        let action = event_handler.handle_click(egui::Pos2::new(100.0, 100.0));
        
        let response_time = start_time.elapsed().as_millis() as f32;
        assert!(response_time < 100.0, "Response time should be <100ms, got {}", response_time);
    }
    
    #[test]
    fn test_actions_per_minute() {
        let mut layout_manager = LayoutManager::new(&create_test_context());
        let start_time = std::time::Instant::now();
        let mut action_count = 0;
        
        // Simulate rapid clicking for 1 minute
        while start_time.elapsed().as_secs() < 60 {
            layout_manager.handle_click(egui::Pos2::new(100.0, 100.0));
            action_count += 1;
        }
        
        assert!(action_count >= 120, "Should handle 120+ actions per minute, got {}", action_count);
    }
    
    #[test]
    fn test_mobile_touch_accuracy() {
        let mut layout_manager = LayoutManager::new(&create_test_context());
        let mut successful_touches = 0;
        let total_touches = 100;
        
        for i in 0..total_touches {
            let touch_pos = egui::Pos2::new(100.0 + i as f32, 100.0);
            if layout_manager.handle_touch(touch_pos) {
                successful_touches += 1;
            }
        }
        
        let accuracy = successful_touches as f32 / total_touches as f32;
        assert!(accuracy > 0.95, "Touch accuracy should be >95%, got {:.2}%", accuracy * 100.0);
    }
}
```

## Validation Checklist

### Core Functionality

- [ ] 3-column layout renders correctly on all screen sizes
- [ ] Click events register within 100ms
- [ ] Node click opens squad selection overlay
- [ ] Squad selection completes deployment flow
- [ ] Resource header updates in real-time
- [ ] Garden backdrop animates idle slimes

### Performance Requirements

- [ ] Frame rate maintains 60fps with 100+ slimes
- [ ] Memory usage stays below 500MB
- [ ] UI transitions complete in 0ms (instant)
- [ ] Input processing handles 120+ actions per minute
- [ ] Mobile touch targets are 44x44px minimum

### User Experience

- [ ] 30-second gameplay cycle achievable
- [ ] All interactions are intuitive and responsive
- [ ] Visual feedback is immediate and clear
- [ ] Information density supports rapid decision-making
- [ ] Error handling is graceful and informative

## Future Enhancements

### Post-Sprint Improvements

1. **Keyboard Shortcuts**: Power-user navigation
2. **Gesture Support**: Swipe and pinch gestures
3. **Accessibility**: Screen reader support and high contrast mode
4. **Themes**: Multiple visual themes
5. **Analytics**: User interaction tracking

The Tactical Wiring Sprint establishes the foundational interaction patterns that enable the Mafia Wars gameplay loop, ensuring the UI is responsive, intuitive, and capable of supporting the high-density input patterns required for addictive gameplay.
