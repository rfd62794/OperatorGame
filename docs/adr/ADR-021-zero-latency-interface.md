# ADR-021: The "Zero-Latency" Interface

**Status:** ACCEPTED | **Date:** 2026-03-04 | **Author:** Gemini (via PyPro SDD-Edition)

## Context

The existing UI approach was becoming too focused on "Visual Jazz" and losing sight of the core "Mafia Wars" gameplay loop: rapid-fire progression, menu-driven control, and deep math. The Astronaut needs to make quick tactical decisions without being slowed by transition animations or complex navigation patterns.

## Decision

Implement a "Zero-Latency" Interface with immediate state-switching, no transition animations, and a 3-column layout optimized for high-density input and rapid decision-making.

## Architecture

### Core UI Philosophy

1. **Input Density Over Visual Polish**: Prioritize click efficiency over aesthetic transitions
2. **Immediate State Switching**: No animations between menu states
3. **Simultaneous Information Display**: All critical panels visible at once
4. **High-Pressure Terminal Feel**: Maintain urgency and responsiveness
5. **30-Second Cycle Time**: Complete breeding/mission cycle in under 30 seconds

### Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│                    HEADER (Resources)                      │
├─────────────┬─────────────────┬─────────────────────────────┤
│             │                 │                             │
│  MANIFEST   │  COMMAND DECK   │        PLANET MAP           │
│  (Roster)   │  (Operations)   │       (Cell War)            │
│             │                 │                             │
│  • Slime    │  • Incubator    │  • 15 Nodes                 │
│    Cards    │  • Expeditions  │  • Click to Deploy          │
│  • Sample   │  • Research     │  • Resource Display         │
│  • Select   │  • Logs         │                             │
│             │                 │                             │
└─────────────┴─────────────────┴─────────────────────────────┘
                    BACKDROP (Garden)
```

## Implementation

### Zero-Latency State Management

```rust
pub struct ZeroLatencyUI {
    pub current_state: UIState,
    pub previous_state: UIState,
    pub transition_immediate: bool,
    pub input_buffer: Vec<InputEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIState {
    ManifestView,      // Left panel active
    CommandDeckView,   // Center panel active
    PlanetMapView,     // Right panel active
    SquadSelection,    // Overlay for deployment
    BreedingPreview,   // Overlay for incubator
    ResearchPanel,     // Tech tree overlay
}

impl ZeroLatencyUI {
    pub fn switch_state(&mut self, new_state: UIState) {
        // Immediate state switch - no animation
        self.previous_state = self.current_state.clone();
        self.current_state = new_state;
        
        // Process any pending input immediately
        self.process_input_buffer();
    }
    
    pub fn handle_click(&mut self, click_event: ClickEvent) -> UIResponse {
        match click_event.target {
            ClickTarget::Node(node_id) => {
                self.switch_state(UIState::SquadSelection);
                UIResponse::OpenSquadSelection(node_id)
            },
            ClickTarget::SlimeCard(slime_id) => {
                self.switch_state(UIState::BreedingPreview);
                UIResponse::OpenBreedingPreview(slime_id)
            },
            ClickTarget::Incubator => {
                self.switch_state(UIState::CommandDeckView);
                UIResponse::FocusIncubator
            },
            ClickTarget::Expedition => {
                self.switch_state(UIState::CommandDeckView);
                UIResponse::FocusExpeditions
            },
            _ => UIResponse::NoAction,
        }
    }
}
```

### 3-Column Layout System

```rust
pub struct ThreeColumnLayout {
    pub manifest_panel: ManifestPanel,
    pub command_deck: CommandDeckPanel,
    pub planet_map: PlanetMapPanel,
    pub header: ResourceHeader,
    pub backdrop: GardenBackdrop,
}

impl ThreeColumnLayout {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            manifest_panel: ManifestPanel::new(),
            command_deck: CommandDeckPanel::new(),
            planet_map: PlanetMapPanel::new(),
            header: ResourceHeader::new(),
            backdrop: GardenBackdrop::new(),
        }
    }
    
    pub fn render(&mut self, ctx: &egui::Context) {
        // Render backdrop first (background)
        self.backdrop.render(ctx);
        
        // Render main layout
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            self.header.render(ui);
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left Column - Manifest (30% width)
                ui.vertical(|ui| {
                    self.manifest_panel.render(ui);
                });
                
                ui.separator();
                
                // Center Column - Command Deck (40% width)
                ui.vertical(|ui| {
                    self.command_deck.render(ui);
                });
                
                ui.separator();
                
                // Right Column - Planet Map (30% width)
                ui.vertical(|ui| {
                    self.planet_map.render(ui);
                });
            });
        });
        
        // Handle overlays (if active)
        if let Some(overlay) = self.get_active_overlay() {
            self.render_overlay(ctx, overlay);
        }
    }
}
```

### Input Event System

```rust
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub event_type: InputType,
    pub target: ClickTarget,
    pub timestamp: std::time::Instant,
    pub modifier_keys: Vec<ModifierKey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputType {
    Click,
    DoubleClick,
    RightClick,
    Drag,
    KeyPress(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClickTarget {
    Node(Uuid),
    SlimeCard(Uuid),
    Incubator,
    Expeditions,
    Research,
    HeaderResource(ResourceType),
    Backdrop,
}

impl InputEvent {
    pub fn is_rapid_click(&self) -> bool {
        // Detect rapid clicking for power-user behavior
        self.event_type == InputType::Click
    }
    
    pub fn get_action_priority(&self) -> u8 {
        match self.target {
            ClickTarget::Node(_) => 1,      // Highest priority
            ClickTarget::SlimeCard(_) => 2,
            ClickTarget::Incubator => 3,
            ClickTarget::Expeditions => 4,
            ClickTarget::Research => 5,
            ClickTarget::HeaderResource(_) => 6,
            ClickTarget::Backdrop => 7,    // Lowest priority
        }
    }
}
```

## Component Specifications

### Manifest Panel (Left Column)

```rust
pub struct ManifestPanel {
    pub slime_cards: Vec<SlimeCard>,
    pub scroll_position: f32,
    pub selected_slime: Option<Uuid>,
    pub filter_options: ManifestFilter,
}

impl ManifestPanel {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Bio-Manifest");
        
        // Filter controls (minimal)
        ui.horizontal(|ui| {
            if ui.button("All").clicked() {
                self.filter_options = ManifestFilter::All;
            }
            if ui.button("Idle").clicked() {
                self.filter_options = ManifestFilter::Idle;
            }
            if ui.button("Ready").clicked() {
                self.filter_options = ManifestFilter::Ready;
            }
        });
        
        // Slime cards list
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for slime_card in &self.slime_cards {
                    if self.filter_options.matches(slime_card) {
                        self.render_slime_card(ui, slime_card);
                    }
                }
            });
    }
    
    fn render_slime_card(&mut self, ui: &mut egui::Ui, card: &SlimeCard) {
        let frame = egui::Frame {
            fill: if self.selected_slime == Some(card.id) {
                egui::Color32::from_rgb(100, 150, 200)
            } else {
                egui::Color32::from_rgb(50, 50, 50)
            },
            ..Default::default()
        };
        
        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Slime visual (small)
                let rect = ui.allocate_space([40.0, 40.0].into()).1;
                card.render_mini(ui, rect);
                
                // Slime info
                ui.vertical(|ui| {
                    ui.label(&card.name);
                    ui.label(format!("Lv.{} {}", card.level, card.culture));
                    ui.label(format!("Gen. {}", card.generation));
                });
                
                // Action buttons
                ui.vertical(|ui| {
                    if ui.button("Sample").clicked() {
                        // Add to breeding selection
                        self.handle_sample_action(card.id);
                    }
                    if ui.button("Select").clicked() {
                        // Add to squad selection
                        self.handle_select_action(card.id);
                    }
                });
            });
        });
    }
}
```

### Command Deck Panel (Center Column)

```rust
pub struct CommandDeckPanel {
    pub active_tab: CommandTab,
    pub incubator: IncubatorWidget,
    pub expeditions: ExpeditionWidget,
    pub research: ResearchWidget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandTab {
    Incubator,
    Expeditions,
    Research,
}

impl CommandDeckPanel {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Command Deck");
        
        // Tab selection (immediate switch)
        ui.horizontal(|ui| {
            if ui.button("Incubator").clicked() {
                self.active_tab = CommandTab::Incubator;
            }
            if ui.button("Expeditions").clicked() {
                self.active_tab = CommandTab::Expeditions;
            }
            if ui.button("Research").clicked() {
                self.active_tab = CommandTab::Research;
            }
        });
        
        ui.separator();
        
        // Render active tab content
        match self.active_tab {
            CommandTab::Incubator => self.incubator.render(ui),
            CommandTab::Expeditions => self.expeditions.render(ui),
            CommandTab::Research => self.research.render(ui),
        }
    }
}
```

### Planet Map Panel (Right Column)

```rust
pub struct PlanetMapPanel {
    pub nodes: Vec<MapNode>,
    pub selected_node: Option<Uuid>,
    pub zoom_level: f32,
    pub show_conflict_zones: bool,
}

impl PlanetMapPanel {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Planet Map");
        
        // Map controls (minimal)
        ui.horizontal(|ui| {
            if ui.button("Zoom In").clicked() {
                self.zoom_level *= 1.2;
            }
            if ui.button("Zoom Out").clicked() {
                self.zoom_level /= 1.2;
            }
            ui.checkbox(&mut self.show_conflict_zones, "Conflicts");
        });
        
        // Render map
        let available_size = ui.available_size();
        let rect = ui.allocate_space(available_size).1;
        
        self.render_map_nodes(ui, rect);
    }
    
    fn render_map_nodes(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        for node in &self.nodes {
            let node_pos = self.calculate_node_position(node, rect);
            let node_size = self.calculate_node_size(node);
            
            // Render node
            let color = node.get_display_color();
            ui.painter().circle_filled(node_pos, node_size, color);
            
            // Render node info on hover/click
            if ui.rect_contains_pointer(node_pos, node_size) {
                self.render_node_tooltip(ui, node, node_pos);
            }
        }
    }
    
    fn handle_node_click(&mut self, node_id: Uuid) {
        self.selected_node = Some(node_id);
        // Trigger squad selection overlay
    }
}
```

### Resource Header

```rust
pub struct ResourceHeader {
    pub biomass: u64,
    pub scrap: u64,
    pub ship_integrity: f32,
    pub last_update: std::time::Instant,
}

impl ResourceHeader {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Biomass
            ui.label("🌱 Biomass:");
            ui.colored_color(egui::Color32::GREEN, format!("{}", self.biomass));
            
            ui.separator();
            
            // Scrap
            ui.label("⚙️ Scrap:");
            ui.colored_color(egui::Color32::LIGHT_BLUE, format!("{}", self.scrap));
            
            ui.separator();
            
            // Ship Integrity
            ui.label("🚀 Integrity:");
            let integrity_color = if self.ship_integrity > 0.7 {
                egui::Color32::GREEN
            } else if self.ship_integrity > 0.3 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::RED
            };
            ui.colored_color(integrity_color, format!("{:.1}%", self.ship_integrity * 100.0));
        });
    }
    
    pub fn update_resources(&mut self, delta_time: f32) {
        // Update resources based on production rates
        self.biomass += (self.calculate_biomass_rate() * delta_time) as u64;
        self.scrap += (self.calculate_scrap_rate() * delta_time) as u64;
        
        // Ship integrity decay (if not repaired)
        self.ship_integrity -= 0.01 * delta_time / 60.0; // 1% per minute
        self.ship_integrity = self.ship_integrity.max(0.0);
    }
}
```

### Garden Backdrop

```rust
pub struct GardenBackdrop {
    pub idle_slimes: Vec<SlimeGenome>,
    pub particle_system: ParticleSystem,
    pub time: f32,
}

impl GardenBackdrop {
    pub fn render(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.screen_rect();
        
        // Render floating slimes in background
        for slime in &self.idle_slimes {
            let pos = self.calculate_slime_position(slime, screen_rect, self.time);
            let size = egui::Vec2::new(30.0, 30.0);
            
            // Render with transparency for background effect
            ctx.painter().circle_filled(pos, size.x / 2.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 100));
            
            // Render slime details
            self.draw_slime_background(ctx, slime, pos, size, self.time);
        }
        
        // Update time for animation
        self.time += ctx.input(|i| i.stable_dt).unwrap_or(0.016);
    }
    
    fn calculate_slime_position(&self, slime: &SlimeGenome, screen_rect: egui::Rect, time: f32) -> egui::Pos2 {
        // Floating animation based on slime ID and time
        let base_x = (slime.id.as_u128() as f32 * 137.5).sin() * 0.3 + 0.5;
        let base_y = (slime.id.as_u128() as f32 * 89.7).cos() * 0.3 + 0.5;
        
        let float_x = (time * 0.5 + slime.id.as_u128() as f32 * 0.1).sin() * 20.0;
        let float_y = (time * 0.3 + slime.id.as_u128() as f32 * 0.15).cos() * 15.0;
        
        egui::Pos2::new(
            screen_rect.min.x + screen_rect.width() * base_x + float_x,
            screen_rect.min.y + screen_rect.height() * base_y + float_y,
        )
    }
    
    fn draw_slime_background(&self, ctx: &egui::Context, slime: &SlimeGenome, pos: egui::Pos2, size: egui::Vec2, time: f32) {
        // Use existing draw_slime function with background rendering
        let breathing = (time * 2.0 + slime.id.as_u128() as f32 * 0.1).sin() * 0.1 + 1.0;
        let animated_size = size * breathing;
        
        // Render slime shape with cultural color
        let color = slime.culture.get_rgb_color();
        let slime_color = egui::Color32::from_rgb(color[0], color[1], color[2]);
        
        ctx.painter().circle_filled(pos, animated_size.x / 2.0, slime_color);
    }
}
```

## Performance Optimizations

### Input Processing

```rust
impl ZeroLatencyUI {
    pub fn process_input_buffer(&mut self) {
        // Process high-priority inputs first
        self.input_buffer.sort_by_key(|event| event.get_action_priority());
        
        for event in self.input_buffer.drain(..) {
            if self.should_process_event(&event) {
                self.handle_click(event.into());
            }
        }
    }
    
    fn should_process_event(&self, event: &InputEvent) -> bool {
        // Filter out redundant or low-priority events
        match event.target {
            ClickTarget::Backdrop => false, // Ignore backdrop clicks
            _ => true,
        }
    }
}
```

### Rendering Optimization

```rust
impl ThreeColumnLayout {
    pub fn render_optimized(&mut self, ctx: &egui::Context) {
        // Only render changed panels
        if self.manifest_panel.needs_redraw() {
            self.manifest_panel.render_cached(ctx);
        }
        
        if self.command_deck.needs_redraw() {
            self.command_deck.render_cached(ctx);
        }
        
        if self.planet_map.needs_redraw() {
            self.planet_map.render_cached(ctx);
        }
        
        // Always render backdrop (animated)
        self.backdrop.render(ctx);
    }
}
```

## Consequences

### Positive
- **Ultra-Responsive Interface**: No animation delays
- **High Input Density**: Maximum actions per minute
- **Clear Information Hierarchy**: All critical data visible
- **Mobile-Friendly**: Works well on touch devices
- **Addictive Loop**: 30-second cycle time achievable

### Negative
- **Reduced Visual Polish**: Less impressive animations
- **Learning Curve**: Dense interface may overwhelm new players
- **Screen Space Constraints**: Limited real estate for each panel
- **Accessibility**: May be challenging for users with motor impairments

### Risks
- **Information Overload**: Too much data at once
- **Click Fatigue**: High-frequency clicking may strain users
- **Mobile Performance**: Complex UI may impact mobile performance
- **User Retention**: Minimalist approach may not appeal to all players

## Implementation Tasks

1. **Refactor `src/ui.rs`**: Implement 3-column layout system
2. **Wire Dispatch Action**: Node click → Squad selection flow
3. **Add Resource Header**: Biomass, Scrap, Ship Integrity display
4. **Implement Garden Backdrop**: Animated slime background rendering
5. **Optimize Input Handling**: Zero-latency click processing
6. **Test Performance**: Ensure 60fps on target devices

## Validation

- [ ] All panel switches complete within 16ms (one frame)
- [ ] Click-to-action response time < 100ms
- [ ] 30-second breeding/mission cycle achievable
- [ ] UI remains responsive with 100+ slimes in roster
- [ ] Mobile touch controls function properly
- [ ] Resource updates display in real-time

## Future Enhancements

1. **Keyboard Shortcuts**: Power-user navigation
2. **Customizable Layout**: User-adjustable panel sizes
3. **Themes**: Multiple visual themes for accessibility
4. **Tooltips**: Contextual help on hover
5. **Action History**: Undo/redo functionality

The Zero-Latency Interface creates the high-pressure, responsive terminal experience that captures the addictive core of Mafia Wars while maintaining the strategic depth of the chromatic system.
