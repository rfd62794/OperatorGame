# Mafia Wars Minimalist UI Strategy

> **Status:** UI DESIGN PHILOSOPHY v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-021, TACTICAL_WIRING_SPRINT.md, SPEC.md §8

## Overview

The Mafia Wars UI Strategy prioritizes **Input Density** and **Rapid-Fire Progression** over visual polish. This approach creates an addictive, high-pressure terminal experience where the Astronaut can cycle through breeding and missions in under 30 seconds, maintaining the core gameplay loop that made Mafia Wars compelling.

## Core Design Philosophy

### Input Density Over Visual Jazz

```rust
pub struct InputDensityMetrics {
    pub actions_per_minute: u32,
    pub decision_time_ms: u32,
    pub click_efficiency: f32,
    pub cognitive_load: f32,
}

impl InputDensityMetrics {
    pub fn target_metrics() -> Self {
        Self {
            actions_per_minute: 120,    // 2 actions per second
            decision_time_ms: 250,      // Quarter-second decisions
            click_efficiency: 0.9,      // 90% of clicks are meaningful
            cognitive_load: 0.6,        // Moderate mental load
        }
    }
}
```

### The 30-Second Cycle

The core gameplay loop should complete in under 30 seconds:

1. **Breeding Phase** (10 seconds): Select parents → Start incubation
2. **Dispatch Phase** (10 seconds): Select node → Deploy squad  
3. **Management Phase** (10 seconds): Check resources → Plan next actions

```rust
pub struct GameCycleTimer {
    pub breeding_phase: Duration,
    pub dispatch_phase: Duration,
    pub management_phase: Duration,
    pub total_cycle_time: Duration,
}

impl GameCycleTimer {
    pub fn new() -> Self {
        Self {
            breeding_phase: Duration::from_secs(10),
            dispatch_phase: Duration::from_secs(10),
            management_phase: Duration::from_secs(10),
            total_cycle_time: Duration::from_secs(30),
        }
    }
    
    pub fn is_optimal_cycle(&self, actual_time: Duration) -> bool {
        actual_time <= self.total_cycle_time
    }
}
```

## Layout Architecture

### 3-Column Information Density

```
┌─────────────────────────────────────────────────────────────┐
│                    HEADER (Resources)                      │
├─────────────┬─────────────────┬─────────────────────────────┤
│             │                 │                             │
│  MANIFEST   │  COMMAND DECK   │        PLANET MAP           │
│  (30%)      │    (40%)        │         (30%)              │
│             │                 │                             │
│ • Slime     │ • Incubator     │ • 15 Nodes                  │
│   Cards     │ • Expeditions   │ • Click Deploy              │
│ • Sample    │ • Research      │ • Resource Display          │
│ • Select    │ • Logs          │ • Conflict Zones            │
│             │                 │                             │
│  Vertical   │   Tab Switch    │    Interactive Map          │
│  Scroll     │   (Instant)     │    (Click Actions)          │
│             │                 │                             │
└─────────────┴─────────────────┴─────────────────────────────┘
                    BACKDROP (Garden)
```

### Space Allocation Strategy

```rust
pub struct LayoutProportions {
    pub manifest_width: f32,    // 30% of screen width
    pub command_deck_width: f32, // 40% of screen width  
    pub planet_map_width: f32,   // 30% of screen width
    pub header_height: f32,      // 10% of screen height
    pub backdrop_opacity: f32,   // 20% opacity for background
}

impl LayoutProportions {
    pub fn mobile_optimized() -> Self {
        Self {
            manifest_width: 0.35,     // Slightly wider on mobile
            command_deck_width: 0.35, // Reduced for mobile
            planet_map_width: 0.30,   // Same proportion
            header_height: 0.12,      // Slightly taller on mobile
            backdrop_opacity: 0.15,   // Less prominent on mobile
        }
    }
    
    pub fn desktop_optimized() -> Self {
        Self {
            manifest_width: 0.25,     // Narrower on desktop
            command_deck_width: 0.50, // More space for operations
            planet_map_width: 0.25,   // Balanced with manifest
            header_height: 0.08,      // Compact on desktop
            backdrop_opacity: 0.25,   // More visible on desktop
        }
    }
}
```

## Component Design Principles

### Manifest Panel (Left Column)

#### Rapid Card Scanning

```rust
pub struct SlimeCard {
    pub id: Uuid,
    pub name: String,
    pub level: u8,
    pub generation: u32,
    pub culture: Culture,
    pub state: OperatorState,
    pub stats: SlimeStats,
    pub quick_actions: Vec<QuickAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuickAction {
    Sample,    // Add to breeding
    Select,    // Add to squad
    Deploy,    // Quick deploy
    Heal,      // Quick healing
}

impl SlimeCard {
    pub fn render_compact(&self, ui: &mut egui::Ui, width: f32) {
        let card_height = 60.0; // Fixed height for rapid scanning
        
        ui.horizontal(|ui| {
            // Visual indicator (small, colorful)
            let visual_rect = ui.allocate_space([30.0, 30.0].into()).1;
            self.render_culture_icon(ui, visual_rect);
            
            // Core info (compact)
            ui.vertical(|ui| {
                ui.label(format!("{} Lv.{}", self.name, self.level));
                ui.label(format!("{} Gen.{}", self.culture, self.generation));
                ui.label(format!("{:?}", self.state));
            });
            
            // Quick actions (buttons)
            ui.vertical(|ui| {
                for action in &self.quick_actions {
                    match action {
                        QuickAction::Sample => {
                            if ui.add_sized([40.0, 20.0], egui::Button::new("S")).clicked() {
                                self.handle_sample();
                            }
                        },
                        QuickAction::Select => {
                            if ui.add_sized([40.0, 20.0], egui::Button::new("✓")).clicked() {
                                self.handle_select();
                            }
                        },
                        QuickAction::Deploy => {
                            if ui.add_sized([40.0, 20.0], egui::Button::new("→")).clicked() {
                                self.handle_deploy();
                            }
                        },
                        QuickAction::Heal => {
                            if ui.add_sized([40.0, 20.0], egui::Button::new("♥")).clicked() {
                                self.handle_heal();
                            }
                        },
                    }
                }
            });
        });
    }
    
    fn render_culture_icon(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let color = self.culture.get_rgb_color();
        let culture_color = egui::Color32::from_rgb(color[0], color[1], color[2]);
        
        ui.painter().circle_filled(rect.center(), rect.width() / 2.0, culture_color);
        
        // Add culture symbol
        let symbol = self.culture.get_symbol();
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            symbol,
            egui::FontId::default(),
            egui::Color32::WHITE,
        );
    }
}
```

#### Filtering and Sorting

```rust
pub struct ManifestFilter {
    pub filter_type: FilterType,
    pub sort_by: SortCriteria,
    pub show_only_ready: bool,
    pub show_only_idle: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterType {
    All,
    Idle,
    Ready,
    Injured,
    Deployed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortCriteria {
    Level,
    Generation,
    Culture,
    Name,
    State,
}

impl ManifestFilter {
    pub fn apply_filter(&self, slimes: &[SlimeCard]) -> Vec<&SlimeCard> {
        slimes
            .iter()
            .filter(|slime| self.matches_filter(slime))
            .collect()
    }
    
    fn matches_filter(&self, slime: &SlimeCard) -> bool {
        match self.filter_type {
            FilterType::All => true,
            FilterType::Idle => slime.state == OperatorState::Idle,
            FilterType::Ready => slime.state == OperatorState::Idle && slime.level >= 2,
            FilterType::Injured => matches!(slime.state, OperatorState::Injured(_)),
            FilterType::Deployed => matches!(slime.state, OperatorState::Deployed(_)),
        }
    }
}
```

### Command Deck Panel (Center Column)

#### Instant Tab Switching

```rust
pub struct CommandDeckPanel {
    pub active_tab: CommandTab,
    pub tab_content: HashMap<CommandTab, Box<dyn TabContent>>,
    pub animation_disabled: bool, // No transitions
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandTab {
    Incubator,
    Expeditions,
    Research,
    Logs,
}

impl CommandDeckPanel {
    pub fn switch_tab(&mut self, new_tab: CommandTab) {
        // Immediate switch - no animation
        self.active_tab = new_tab;
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Tab bar (compact)
        ui.horizontal(|ui| {
            for tab in [CommandTab::Incubator, CommandTab::Expeditions, CommandTab::Research, CommandTab::Logs] {
                let button_text = match tab {
                    CommandTab::Incubator => "🧬 Breed",
                    CommandTab::Expeditions => "🚀 Missions",
                    CommandTab::Research => "🔬 Tech",
                    CommandTab::Logs => "📜 Logs",
                };
                
                let button = if self.active_tab == tab {
                    egui::Button::new(button_text).fill(egui::Color32::from_rgb(100, 150, 200))
                } else {
                    egui::Button::new(button_text)
                };
                
                if ui.add(button).clicked() {
                    self.switch_tab(tab);
                }
            }
        });
        
        ui.separator();
        
        // Render tab content immediately
        if let Some(content) = self.tab_content.get_mut(&self.active_tab) {
            content.render(ui);
        }
    }
}
```

#### Incubator Widget

```rust
pub struct IncubatorWidget {
    pub selected_parents: [Option<Uuid>; 2],
    pub mixing_progress: f32,
    pub current_recipe: Option<MixingRecipe>,
    pub quick_recipes: Vec<QuickRecipe>,
}

#[derive(Debug, Clone)]
pub struct QuickRecipe {
    pub name: String,
    pub parent_a: Culture,
    pub parent_b: Culture,
    pub result: Culture,
    pub success_rate: f32,
    pub available: bool,
}

impl IncubatorWidget {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Incubator");
        
        // Quick recipe selection
        if !self.quick_recipes.is_empty() {
            ui.label("Quick Recipes:");
            for recipe in &self.quick_recipes {
                let button_text = format!("{} + {} = {} ({:.0}%)", 
                    recipe.parent_a, recipe.parent_b, recipe.result, recipe.success_rate * 100.0);
                
                let button = if recipe.available {
                    egui::Button::new(button_text)
                } else {
                    egui::Button::new(button_text).fill(egui::Color32::DARK_GRAY)
                };
                
                if ui.add(button).clicked() && recipe.available {
                    self.select_quick_recipe(recipe);
                }
            }
            ui.separator();
        }
        
        // Parent selection
        ui.horizontal(|ui| {
            ui.label("Parents:");
            
            // Parent A
            if let Some(parent_a_id) = self.selected_parents[0] {
                ui.label(format!("A: {}", get_slime_name(parent_a_id)));
                if ui.button("✕").clicked() {
                    self.selected_parents[0] = None;
                }
            } else {
                if ui.button("Select A").clicked() {
                    // Open parent selection overlay
                }
            }
            
            // Parent B
            if let Some(parent_b_id) = self.selected_parents[1] {
                ui.label(format!("B: {}", get_slime_name(parent_b_id)));
                if ui.button("✕").clicked() {
                    self.selected_parents[1] = None;
                }
            } else {
                if ui.button("Select B").clicked() {
                    // Open parent selection overlay
                }
            }
        });
        
        // Mixing progress
        if self.mixing_progress > 0.0 {
            ui.horizontal(|ui| {
                ui.label("Progress:");
                ui.add(egui::ProgressBar::new(self.mixing_progress).show_percentage());
            });
            
            if ui.button("Cancel").clicked() {
                self.cancel_mixing();
            }
        } else if self.selected_parents[0].is_some() && self.selected_parents[1].is_some() {
            if ui.button("🧬 SYNTHESIZE").clicked() {
                self.start_mixing();
            }
        }
    }
}
```

### Planet Map Panel (Right Column)

#### Interactive Node System

```rust
pub struct PlanetMapWidget {
    pub nodes: Vec<MapNode>,
    pub selected_node: Option<Uuid>,
    pub deployment_mode: bool,
    pub show_resource_overlay: bool,
}

impl PlanetMapWidget {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Planet Map");
        
        // Quick controls
        ui.horizontal(|ui| {
            if ui.button("🎯 Deploy").clicked() {
                self.deployment_mode = !self.deployment_mode;
            }
            ui.checkbox(&mut self.show_resource_overlay, "Resources");
        });
        
        // Map rendering
        let available_size = ui.available_size();
        let map_rect = ui.allocate_space(available_size).1;
        
        self.render_node_grid(ui, map_rect);
        
        // Node information panel
        if let Some(node_id) = self.selected_node {
            self.render_node_info(ui, node_id);
        }
    }
    
    fn render_node_grid(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        // Render 15 nodes in a strategic layout
        let node_positions = self.calculate_node_positions(rect);
        
        for (i, node) in self.nodes.iter().enumerate() {
            if let Some(&pos) = node_positions.get(i) {
                let node_size = self.calculate_node_size(node);
                let node_color = self.get_node_color(node);
                
                // Render node
                ui.painter().circle_filled(pos, node_size, node_color);
                
                // Render node icon
                let icon = node.culture.get_symbol();
                ui.painter().text(
                    pos,
                    egui::Align2::CENTER_CENTER,
                    icon,
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
                
                // Handle click
                if ui.rect_contains_pointer(pos, node_size) && ui.input(|i| i.pointer.primary_clicked()) {
                    self.handle_node_click(node.id);
                }
                
                // Render resource overlay if enabled
                if self.show_resource_overlay {
                    self.render_resource_info(ui, node, pos);
                }
            }
        }
    }
    
    fn handle_node_click(&mut self, node_id: Uuid) {
        self.selected_node = Some(node_id);
        
        if self.deployment_mode {
            // Open squad selection overlay
            self.open_squad_selection(node_id);
        }
    }
    
    fn render_node_info(&self, ui: &mut egui::Ui, node_id: Uuid) {
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            ui.heading(format!("Node: {}", node.culture));
            
            ui.horizontal(|ui| {
                ui.label("Resources:");
                ui.label(format!("{} scrap/hr", node.resource_value));
            });
            
            ui.horizontal(|ui| {
                ui.label("Control:");
                ui.label(format!("{:.1}%", node.control_strength * 100.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Owner:");
                if let Some(owner) = node.owner {
                    ui.label(get_player_name(owner));
                } else {
                    ui.label("Unclaimed");
                }
            });
            
            if self.deployment_mode {
                if ui.button("🚀 Deploy Squad").clicked() {
                    self.open_squad_selection(node_id);
                }
            }
        }
    }
}
```

## Resource Header Design

### Real-Time Resource Display

```rust
pub struct ResourceHeader {
    pub resources: HashMap<ResourceType, u64>,
    pub production_rates: HashMap<ResourceType, f64>,
    pub last_update: std::time::Instant,
    pub animation_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Biomass,
    Scrap,
    ShipIntegrity,
    Research,
    Energy,
}

impl ResourceHeader {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Biomass
            self.render_resource(ui, ResourceType::Biomass, "🌱", egui::Color32::GREEN);
            
            ui.separator();
            
            // Scrap
            self.render_resource(ui, ResourceType::Scrap, "⚙️", egui::Color32::LIGHT_BLUE);
            
            ui.separator();
            
            // Ship Integrity
            self.render_resource(ui, ResourceType::ShipIntegrity, "🚀", egui::Color32::YELLOW);
            
            ui.separator();
            
            // Research
            self.render_resource(ui, ResourceType::Research, "🔬", egui::Color32::PURPLE);
        });
    }
    
    fn render_resource(&mut self, ui: &mut egui::Ui, resource_type: ResourceType, icon: &str, color: egui::Color32) {
        let amount = self.resources.get(&resource_type).unwrap_or(&0);
        let rate = self.production_rates.get(&resource_type).unwrap_or(&0.0);
        
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(icon);
                ui.colored_color(color, format!("{}", amount));
            });
            
            if *rate != 0.0 {
                let rate_text = if *rate > 0.0 {
                    format!("+{:.1}/min", rate * 60.0)
                } else {
                    format!("{:.1}/min", rate * 60.0)
                };
                ui.colored_color(color, rate_text);
            }
        });
    }
    
    pub fn update_resources(&mut self, delta_time: f32) {
        for (resource_type, rate) in &self.production_rates {
            let current_amount = self.resources.entry(*resource_type).or_insert(0);
            *current_amount += (*rate * delta_time as f64) as u64;
        }
        
        // Special handling for Ship Integrity
        if let Some(integrity) = self.resources.get_mut(&ResourceType::ShipIntegrity) {
            *integrity = (*integrity as f64 - 0.1 * delta_time as f64).max(0.0) as u64;
        }
    }
}
```

## Garden Backdrop System

### Live Wallpaper Implementation

```rust
pub struct GardenBackdrop {
    pub idle_slimes: Vec<IdleSlime>,
    pub particle_effects: Vec<ParticleEffect>,
    pub ambient_animation: AmbientAnimation,
    pub performance_mode: PerformanceMode,
}

#[derive(Debug, Clone)]
pub struct IdleSlime {
    pub slime: SlimeGenome,
    pub position: egui::Pos2,
    pub velocity: egui::Vec2,
    pub animation_phase: f32,
    pub scale: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerformanceMode {
    High,    // Full effects
    Medium,  // Reduced particles
    Low,     // Minimal animation
}

impl GardenBackdrop {
    pub fn render(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        match self.performance_mode {
            PerformanceMode::High => self.render_full_effects(ctx, screen_rect),
            PerformanceMode::Medium => self.render_medium_effects(ctx, screen_rect),
            PerformanceMode::Low => self.render_minimal_effects(ctx, screen_rect),
        }
    }
    
    fn render_full_effects(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        // Render particle effects
        for effect in &mut self.particle_effects {
            effect.update(ctx.input(|i| i.stable_dt).unwrap_or(0.016));
            effect.render(ctx, screen_rect);
        }
        
        // Render idle slimes with full animation
        for slime in &mut self.idle_slimes {
            slime.update(ctx.input(|i| i.stable_dt).unwrap_or(0.016));
            slime.render_full(ctx, screen_rect);
        }
        
        // Render ambient effects
        self.ambient_animation.render(ctx, screen_rect);
    }
    
    fn render_minimal_effects(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        // Only render basic slime positions
        for slime in &self.idle_slimes {
            slime.render_minimal(ctx, screen_rect);
        }
    }
}

impl IdleSlime {
    pub fn update(&mut self, delta_time: f32) {
        // Update floating animation
        self.animation_phase += delta_time * 2.0;
        
        // Update position (slow floating)
        self.position += self.velocity * delta_time;
        
        // Boundary check
        if self.position.x < 0.0 || self.position.x > 1.0 {
            self.velocity.x *= -1.0;
        }
        if self.position.y < 0.0 || self.position.y > 1.0 {
            self.velocity.y *= -1.0;
        }
    }
    
    pub fn render_full(&self, ctx: &egui::Context, screen_rect: egui::Rect) {
        let screen_pos = egui::Pos2::new(
            screen_rect.min.x + screen_rect.width() * self.position.x,
            screen_rect.min.y + screen_rect.height() * self.position.y,
        );
        
        let breathing = (self.animation_phase).sin() * 0.1 + 1.0;
        let size = egui::Vec2::new(25.0, 25.0) * breathing * self.scale;
        
        // Render slime with cultural color
        let color = self.slime.culture.get_rgb_color();
        let slime_color = egui::Color32::from_rgba_unmultiplied(color[0], color[1], color[2], 80);
        
        ctx.painter().circle_filled(screen_pos, size.x / 2.0, slime_color);
    }
    
    pub fn render_minimal(&self, ctx: &egui::Context, screen_rect: egui::Rect) {
        let screen_pos = egui::Pos2::new(
            screen_rect.min.x + screen_rect.width() * self.position.x,
            screen_rect.min.y + screen_rect.height() * self.position.y,
        );
        
        let size = egui::Vec2::new(15.0, 15.0) * self.scale;
        
        // Simple colored dot
        let color = self.slime.culture.get_rgb_color();
        let slime_color = egui::Color32::from_rgba_unmultiplied(color[0], color[1], color[2], 60);
        
        ctx.painter().circle_filled(screen_pos, size.x / 2.0, slime_color);
    }
}
```

## Performance Optimization

### Input Processing Pipeline

```rust
pub struct InputProcessor {
    pub input_queue: VecDeque<InputEvent>,
    pub processing_budget: Duration,
    pub last_process_time: std::time::Instant,
}

impl InputProcessor {
    pub fn process_inputs(&mut self) -> Vec<UIAction> {
        let start_time = std::time::Instant::now();
        let mut actions = Vec::new();
        
        while let Some(input) = self.input_queue.pop_front() {
            if start_time.elapsed() < self.processing_budget {
                actions.push(self.process_input(input));
            } else {
                // Re-queue unprocessed inputs
                self.input_queue.push_front(input);
                break;
            }
        }
        
        self.last_process_time = start_time;
        actions
    }
    
    fn process_input(&self, input: InputEvent) -> UIAction {
        match input.target {
            ClickTarget::Node(node_id) => UIAction::SelectNode(node_id),
            ClickTarget::SlimeCard(slime_id) => UIAction::SelectSlime(slime_id),
            ClickTarget::Incubator => UIAction::OpenIncubator,
            ClickTarget::Expeditions => UIAction::OpenExpeditions,
            _ => UIAction::NoAction,
        }
    }
}
```

## Validation Metrics

### Performance Targets

```rust
pub struct PerformanceTargets {
    pub frame_time_ms: f32,        // Target: 16.67ms (60fps)
    pub input_latency_ms: f32,    // Target: <100ms
    pub ui_response_time_ms: f32,  // Target: <50ms
    pub memory_usage_mb: u32,      // Target: <500MB
    pub cpu_usage_percent: f32,    // Target: <30%
}

impl PerformanceTargets {
    pub fn is_acceptable(&self, metrics: &PerformanceMetrics) -> bool {
        metrics.frame_time_ms <= self.frame_time_ms &&
        metrics.input_latency_ms <= self.input_latency_ms &&
        metrics.ui_response_time_ms <= self.ui_response_time_ms &&
        metrics.memory_usage_mb <= self.memory_usage_mb &&
        metrics.cpu_usage_percent <= self.cpu_usage_percent
    }
}
```

### User Experience Metrics

```rust
pub struct UXMetrics {
    pub actions_per_minute: u32,
    pub average_session_duration: Duration,
    pub task_completion_rate: f32,
    pub error_rate: f32,
    pub user_satisfaction_score: f32,
}

impl UXMetrics {
    pub fn calculate_engagement_score(&self) -> f32 {
        let apm_score = (self.actions_per_minute as f32 / 120.0).min(1.0);
        let duration_score = (self.average_session_duration.as_secs_f32() / 300.0).min(1.0); // 5 min target
        let completion_score = self.task_completion_rate;
        let satisfaction_score = self.user_satisfaction_score;
        
        (apm_score + duration_score + completion_score + satisfaction_score) / 4.0
    }
}
```

## Implementation Guidelines

### Rapid Development Principles

1. **Function Over Form**: Prioritize responsive interactions
2. **Immediate Feedback**: No delayed responses
3. **Minimal Clicks**: Maximum actions per click
4. **Clear Visual Hierarchy**: Important elements prominent
5. **Consistent Patterns**: Same interaction patterns everywhere

### Mobile Optimization

1. **Touch-Friendly Targets**: Minimum 44x44px touch areas
2. **Simplified Controls**: Reduce complexity for touch input
3. **Performance First**: Maintain 60fps on mobile devices
4. **Readable Text**: Minimum 16px font size
5. **Gesture Support**: Swipe and tap gestures where appropriate

The Mafia Wars UI Strategy creates an addictive, high-density interaction pattern that rewards rapid decision-making while maintaining the strategic depth of the chromatic system. This approach captures the core gameplay loop that made Mafia Wars compelling while adapting it for the slime breeding and planetary conquest theme.
