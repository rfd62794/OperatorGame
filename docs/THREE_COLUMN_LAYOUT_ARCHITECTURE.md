# 3-Column Layout Architecture

> **Status:** UI STRUCTURE SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-021, MAFIA_WARS_UI_STRATEGY.md, SPEC.md §8

## Overview

The 3-Column Layout Architecture provides the structural foundation for the Mafia Wars UI strategy, maximizing information density and enabling rapid decision-making. This layout ensures the Astronaut can simultaneously view the Bio-Manifest, Command Deck, and Planet Map to execute the 30-second gameplay cycle efficiently.

## Layout Mathematics

### Screen Division Formula

```rust
pub struct LayoutCalculator {
    pub screen_width: f32,
    pub screen_height: f32,
    pub header_ratio: f32,
    pub column_ratios: [f32; 3],
    pub gutter_width: f32,
}

impl LayoutCalculator {
    pub fn new(screen_size: egui::Vec2) -> Self {
        Self {
            screen_width: screen_size.x,
            screen_height: screen_size.y,
            header_ratio: 0.08,        // 8% of screen height
            column_ratios: [0.30, 0.40, 0.30], // 30%/40%/30% split
            gutter_width: 4.0,          // 4px gutters between columns
        }
    }
    
    pub fn calculate_layout(&self) -> LayoutDimensions {
        let header_height = self.screen_height * self.header_ratio;
        let content_height = self.screen_height - header_height;
        
        let available_width = self.screen_width - (self.gutter_width * 2.0);
        
        let manifest_width = available_width * self.column_ratios[0];
        let command_deck_width = available_width * self.column_ratios[1];
        let planet_map_width = available_width * self.column_ratios[2];
        
        LayoutDimensions {
            header_rect: egui::Rect::from_min_size(
                egui::Pos2::new(0.0, 0.0),
                egui::Vec2::new(self.screen_width, header_height)
            ),
            manifest_rect: egui::Rect::from_min_size(
                egui::Pos2::new(0.0, header_height),
                egui::Vec2::new(manifest_width, content_height)
            ),
            command_deck_rect: egui::Rect::from_min_size(
                egui::Pos2::new(manifest_width + self.gutter_width, header_height),
                egui::Vec2::new(command_deck_width, content_height)
            ),
            planet_map_rect: egui::Rect::from_min_size(
                egui::Pos2::new(manifest_width + self.gutter_width + command_deck_width + self.gutter_width, header_height),
                egui::Vec2::new(planet_map_width, content_height)
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayoutDimensions {
    pub header_rect: egui::Rect,
    pub manifest_rect: egui::Rect,
    pub command_deck_rect: egui::Rect,
    pub planet_map_rect: egui::Rect,
}
```

### Responsive Scaling

```rust
impl LayoutCalculator {
    pub fn get_responsive_layout(&self, device_type: DeviceType) -> LayoutDimensions {
        match device_type {
            DeviceType::Mobile => self.calculate_mobile_layout(),
            DeviceType::Tablet => self.calculate_tablet_layout(),
            DeviceType::Desktop => self.calculate_desktop_layout(),
        }
    }
    
    fn calculate_mobile_layout(&self) -> LayoutDimensions {
        let mut mobile_calc = self.clone();
        mobile_calc.column_ratios = [0.35, 0.35, 0.30]; // Wider manifest on mobile
        mobile_calc.gutter_width = 2.0; // Smaller gutters on mobile
        mobile_calc.calculate_layout()
    }
    
    fn calculate_tablet_layout(&self) -> LayoutDimensions {
        let mut tablet_calc = self.clone();
        tablet_calc.column_ratios = [0.25, 0.50, 0.25]; // More space for command deck
        tablet_calc.gutter_width = 6.0; // Larger gutters on tablet
        tablet_calc.calculate_layout()
    }
    
    fn calculate_desktop_layout(&self) -> LayoutDimensions {
        let mut desktop_calc = self.clone();
        desktop_calc.column_ratios = [0.25, 0.45, 0.30]; // Balanced desktop layout
        desktop_calc.gutter_width = 8.0; // Largest gutters on desktop
        desktop_calc.calculate_layout()
    }
}
```

## Column Specifications

### Left Column: Bio-Manifest (30%)

#### Purpose and Function

```rust
pub struct ManifestColumn {
    pub layout: ColumnLayout,
    pub content: ManifestContent,
    pub scroll_state: ScrollState,
    pub filter_state: FilterState,
}

#[derive(Debug, Clone)]
pub struct ColumnLayout {
    pub width: f32,
    pub padding: egui::Vec2,
    pub item_height: f32,
    pub visible_items: usize,
    pub scrollbar_width: f32,
}

impl ManifestColumn {
    pub fn new(dimensions: &LayoutDimensions) -> Self {
        let layout = ColumnLayout {
            width: dimensions.manifest_rect.width(),
            padding: egui::Vec2::new(8.0, 8.0),
            item_height: 60.0, // Optimized for rapid scanning
            visible_items: ((dimensions.manifest_rect.height() - 16.0) / 60.0) as usize,
            scrollbar_width: 12.0,
        };
        
        Self {
            layout,
            content: ManifestContent::new(),
            scroll_state: ScrollState::new(),
            filter_state: FilterState::new(),
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Header
        self.render_header(ui);
        
        // Filter bar (compact)
        self.render_filter_bar(ui);
        
        // Slime list
        self.render_slime_list(ui);
    }
    
    fn render_header(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Bio-Manifest");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("{} slimes", self.content.total_slimes()));
                ui.label(format!("{} ready", self.content.ready_slimes()));
            });
        });
    }
    
    fn render_filter_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing.x = 4.0;
            
            let filters = [
                ("All", FilterType::All),
                ("Idle", FilterType::Idle),
                ("Ready", FilterType::Ready),
                ("Injured", FilterType::Injured),
            ];
            
            for (text, filter_type) in filters {
                let button = if self.filter_state.active_filter == filter_type {
                    egui::Button::new(text).fill(egui::Color32::from_rgb(100, 150, 200))
                } else {
                    egui::Button::new(text)
                };
                
                if ui.add(button).clicked() {
                    self.filter_state.set_active_filter(filter_type);
                }
            }
        });
    }
    
    fn render_slime_list(&mut self, ui: &mut egui::Ui) {
        let available_height = ui.available_height() - 20.0;
        
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let filtered_slimes = self.filter_state.apply_filters(&self.content.slimes);
                
                for slime in filtered_slimes {
                    self.render_slime_card(ui, slime);
                }
            });
    }
    
    fn render_slime_card(&mut self, ui: &mut egui::Ui, slime: &SlimeCard) {
        let card_rect = ui.allocate_exact_size(
            egui::Vec2::new(self.layout.width - 16.0, self.layout.item_height),
            egui::Sense::click()
        ).1;
        
        let frame = egui::Frame {
            fill: if slime.selected {
                egui::Color32::from_rgb(80, 120, 160)
            } else {
                egui::Color32::from_rgb(40, 40, 40)
            },
            inner_margin: egui::Margin::symmetric(8.0, 4.0),
            ..Default::default()
        };
        
        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Visual indicator (culture icon)
                self.render_culture_icon(ui, slime);
                
                // Slime information
                ui.vertical(|ui| {
                    ui.label(&slime.name);
                    ui.label(format!("Lv.{} {} Gen.{}", slime.level, slime.culture, slime.generation));
                    ui.label(format!("{:?}", slime.state));
                });
                
                // Quick actions
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_sized([40.0, 20.0], egui::Button::new("✓")).clicked() {
                        self.handle_select_action(slime.id);
                    }
                    if ui.add_sized([40.0, 20.0], egui::Button::new("S")).clicked() {
                        self.handle_sample_action(slime.id);
                    }
                });
            });
        });
        
        if ui.rect_contains_pointer(card_rect) && ui.input(|i| i.pointer.primary_clicked()) {
            self.handle_card_click(slime.id);
        }
    }
}
```

### Center Column: Command Deck (40%)

#### Tab-Based Operations

```rust
pub struct CommandDeckColumn {
    pub layout: ColumnLayout,
    pub active_tab: CommandTab,
    pub tabs: HashMap<CommandTab, Box<dyn TabContent>>,
    pub tab_buttons: Vec<TabButton>,
}

impl CommandDeckColumn {
    pub fn new(dimensions: &LayoutDimensions) -> Self {
        let layout = ColumnLayout {
            width: dimensions.command_deck_rect.width(),
            padding: egui::Vec2::new(12.0, 8.0),
            item_height: 0.0, // Variable height for tab content
            visible_items: 0,
            scrollbar_width: 12.0,
        };
        
        let mut tabs: HashMap<CommandTab, Box<dyn TabContent>> = HashMap::new();
        tabs.insert(CommandTab::Incubator, Box::new(IncubatorTab::new()));
        tabs.insert(CommandTab::Expeditions, Box::new(ExpeditionsTab::new()));
        tabs.insert(CommandTab::Research, Box::new(ResearchTab::new()));
        tabs.insert(CommandTab::Logs, Box::new(LogsTab::new()));
        
        Self {
            layout,
            active_tab: CommandTab::Incubator,
            tabs,
            tab_buttons: Self::create_tab_buttons(),
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Header
        self.render_header(ui);
        
        // Tab bar
        self.render_tab_bar(ui);
        
        ui.separator();
        
        // Tab content
        if let Some(tab_content) = self.tabs.get_mut(&self.active_tab) {
            tab_content.render(ui, &self.layout);
        }
    }
    
    fn render_header(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Command Deck");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(tab_content) = self.tabs.get(&self.active_tab) {
                    ui.label(tab_content.get_status_text());
                }
            });
        });
    }
    
    fn render_tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing.x = 2.0;
            
            for button in &mut self.tab_buttons {
                let is_active = button.tab_type == self.active_tab;
                let button_style = if is_active {
                    egui::Button::new(&button.text)
                        .fill(egui::Color32::from_rgb(100, 150, 200))
                } else {
                    egui::Button::new(&button.text)
                };
                
                if ui.add(button_style).clicked() {
                    self.switch_tab(button.tab_type);
                }
            }
        });
    }
    
    fn switch_tab(&mut self, new_tab: CommandTab) {
        // Immediate switch - no animation
        self.active_tab = new_tab;
        
        // Initialize tab content if needed
        if !self.tabs.contains_key(&new_tab) {
            self.tabs.insert(new_tab, self.create_tab_content(new_tab));
        }
    }
}

#[derive(Debug, Clone)]
pub struct TabButton {
    pub tab_type: CommandTab,
    pub text: String,
    pub icon: char,
}

impl CommandDeckColumn {
    fn create_tab_buttons() -> Vec<TabButton> {
        vec![
            TabButton { tab_type: CommandTab::Incubator, text: "🧬 Breed".to_string(), icon: '🧬' },
            TabButton { tab_type: CommandTab::Expeditions, text: "🚀 Missions".to_string(), icon: '🚀' },
            TabButton { tab_type: CommandTab::Research, text: "🔬 Tech".to_string(), icon: '🔬' },
            TabButton { tab_type: CommandTab::Logs, text: "📜 Logs".to_string(), icon: '📜' },
        ]
    }
}
```

### Right Column: Planet Map (30%)

#### Interactive Map Display

```rust
pub struct PlanetMapColumn {
    pub layout: ColumnLayout,
    pub map_state: MapState,
    pub nodes: Vec<MapNode>,
    pub selected_node: Option<Uuid>,
    pub zoom_level: f32,
    pub view_mode: ViewMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewMode {
    Strategic,   // Overview of all nodes
    Tactical,    // Focused on selected node
    Resource,    // Resource overlay
    Conflict,    // Conflict zone highlight
}

impl PlanetMapColumn {
    pub fn new(dimensions: &LayoutDimensions) -> Self {
        let layout = ColumnLayout {
            width: dimensions.planet_map_rect.width(),
            padding: egui::Vec2::new(8.0, 8.0),
            item_height: 0.0,
            visible_items: 0,
            scrollbar_width: 0.0, // No scrollbar for map
        };
        
        Self {
            layout,
            map_state: MapState::new(),
            nodes: Self::generate_initial_nodes(),
            selected_node: None,
            zoom_level: 1.0,
            view_mode: ViewMode::Strategic,
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Header
        self.render_header(ui);
        
        // Controls
        self.render_controls(ui);
        
        // Map area
        self.render_map(ui);
        
        // Node info panel (if node selected)
        if let Some(node_id) = self.selected_node {
            self.render_node_info(ui, node_id);
        }
    }
    
    fn render_header(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Planet Map");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("{} nodes", self.nodes.len()));
                ui.label(format!("Zoom: {:.0}%", self.zoom_level * 100.0));
            });
        });
    }
    
    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Zoom controls
            if ui.add_sized([30.0, 20.0], egui::Button::new("+")).clicked() {
                self.zoom_level = (self.zoom_level * 1.2).min(3.0);
            }
            if ui.add_sized([30.0, 20.0], egui::Button::new("-")).clicked() {
                self.zoom_level = (self.zoom_level / 1.2).max(0.5);
            }
            
            ui.separator();
            
            // View mode controls
            let view_modes = [
                ("Strategic", ViewMode::Strategic),
                ("Tactical", ViewMode::Tactical),
                ("Resources", ViewMode::Resource),
                ("Conflict", ViewMode::Conflict),
            ];
            
            for (text, mode) in view_modes {
                let button = if self.view_mode == mode {
                    egui::Button::new(text).fill(egui::Color32::from_rgb(100, 150, 200))
                } else {
                    egui::Button::new(text)
                };
                
                if ui.add(button).clicked() {
                    self.view_mode = mode;
                }
            }
        });
    }
    
    fn render_map(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        let map_rect = ui.allocate_space(available_size).1;
        
        // Render background
        self.render_map_background(ui, map_rect);
        
        // Render nodes
        self.render_nodes(ui, map_rect);
        
        // Render connections
        self.render_connections(ui, map_rect);
        
        // Handle interactions
        self.handle_map_interactions(ui, map_rect);
    }
    
    fn render_nodes(&self, ui: &mut egui::Ui, map_rect: egui::Rect) {
        for node in &self.nodes {
            let node_pos = self.calculate_node_position(node, map_rect);
            let node_size = self.calculate_node_size(node);
            let node_color = self.get_node_color(node);
            
            // Render node circle
            ui.painter().circle_filled(node_pos, node_size, node_color);
            
            // Render node border (if selected)
            if self.selected_node == Some(node.id) {
                ui.painter().circle_stroke(node_pos, node_size + 2.0, egui::Stroke::new(2.0, egui::Color32::YELLOW));
            }
            
            // Render node icon
            let icon = node.culture.get_symbol();
            ui.painter().text(
                node_pos,
                egui::Align2::CENTER_CENTER,
                icon,
                egui::FontId::default(),
                egui::Color32::WHITE,
            );
            
            // Render resource value (if in resource mode)
            if self.view_mode == ViewMode::Resource {
                let resource_text = format!("{}", node.resource_value);
                ui.painter().text(
                    node_pos + egui::Vec2::new(0.0, node_size + 10.0),
                    egui::Align2::CENTER_CENTER,
                    resource_text,
                    egui::FontId::default(),
                    egui::Color32::LIGHT_BLUE,
                );
            }
        }
    }
    
    fn calculate_node_position(&self, node: &MapNode, map_rect: egui::Rect) -> egui::Pos2 {
        // Use predefined positions for 15 nodes in strategic layout
        let positions = self.get_node_positions();
        
        if let Some(relative_pos) = positions.get(&node.id) {
            egui::Pos2::new(
                map_rect.min.x + map_rect.width() * relative_pos.x,
                map_rect.min.y + map_rect.height() * relative_pos.y,
            )
        } else {
            // Fallback to grid position
            map_rect.center()
        }
    }
    
    fn calculate_node_size(&self, node: &MapNode) -> f32 {
        let base_size = 20.0 * self.zoom_level;
        
        // Size based on node type and control strength
        let size_multiplier = match node.node_type {
            NodeType::Capital => 1.5,
            NodeType::VoidNexus => 2.0,
            NodeType::Resource => 1.2,
            NodeType::Strategic => 1.3,
            _ => 1.0,
        };
        
        base_size * size_multiplier * node.control_strength.sqrt()
    }
    
    fn get_node_color(&self, node: &MapNode) -> egui::Color32 {
        let base_color = node.culture.get_rgb_color();
        let control_factor = node.control_strength;
        
        // Apply control strength to color intensity
        egui::Color32::from_rgb(
            (base_color[0] as f32 * control_factor) as u8,
            (base_color[1] as f32 * control_factor) as u8,
            (base_color[2] as f32 * control_factor) as u8,
        )
    }
}
```

## Integration System

### Layout Manager

```rust
pub struct LayoutManager {
    pub calculator: LayoutCalculator,
    pub manifest_column: ManifestColumn,
    pub command_deck_column: CommandDeckColumn,
    pub planet_map_column: PlanetMapColumn,
    pub resource_header: ResourceHeader,
    pub garden_backdrop: GardenBackdrop,
    pub device_type: DeviceType,
}

impl LayoutManager {
    pub fn new(ctx: &egui::Context) -> Self {
        let screen_size = ctx.screen_rect().size();
        let calculator = LayoutCalculator::new(screen_size);
        let dimensions = calculator.get_responsive_layout(DeviceType::Desktop);
        
        Self {
            manifest_column: ManifestColumn::new(&dimensions),
            command_deck_column: CommandDeckColumn::new(&dimensions),
            planet_map_column: PlanetMapColumn::new(&dimensions),
            resource_header: ResourceHeader::new(),
            garden_backdrop: GardenBackdrop::new(),
            calculator,
            device_type: DeviceType::Desktop,
        }
    }
    
    pub fn render(&mut self, ctx: &egui::Context) {
        // Update layout based on current screen size
        self.update_layout(ctx);
        
        // Render backdrop first
        self.garden_backdrop.render(ctx, ctx.screen_rect());
        
        // Render header
        egui::TopBottomPanel::top("resource_header").show(ctx, |ui| {
            self.resource_header.render(ui);
        });
        
        // Render main content
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_columns(ui);
        });
        
        // Render overlays (if any)
        self.render_overlays(ctx);
    }
    
    fn update_layout(&mut self, ctx: &egui::Context) {
        let screen_size = ctx.screen_rect().size();
        self.calculator = LayoutCalculator::new(screen_size);
        
        // Detect device type if needed
        if self.device_type != DeviceType::Mobile && screen_size.x < 768.0 {
            self.device_type = DeviceType::Mobile;
        }
    }
    
    fn render_main_columns(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Manifest Column (30%)
            ui.vertical(|ui| {
                self.manifest_column.render(ui);
            });
            
            ui.separator();
            
            // Command Deck Column (40%)
            ui.vertical(|ui| {
                self.command_deck_column.render(ui);
            });
            
            ui.separator();
            
            // Planet Map Column (30%)
            ui.vertical(|ui| {
                self.planet_map_column.render(ui);
            });
        });
    }
}
```

## Performance Optimization

### Rendering Pipeline

```rust
pub struct RenderingPipeline {
    pub render_order: Vec<RenderLayer>,
    pub frame_budget: Duration,
    pub performance_mode: PerformanceMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderLayer {
    Backdrop,
    Header,
    ManifestColumn,
    CommandDeckColumn,
    PlanetMapColumn,
    Overlays,
}

impl RenderingPipeline {
    pub fn render_frame(&mut self, ctx: &egui::Context, layout_manager: &mut LayoutManager) {
        let start_time = std::time::Instant::now();
        
        for layer in &self.render_order {
            if start_time.elapsed() < self.frame_budget {
                match layer {
                    RenderLayer::Backdrop => layout_manager.garden_backdrop.render(ctx, ctx.screen_rect()),
                    RenderLayer::Header => self.render_header_layer(ctx, layout_manager),
                    RenderLayer::ManifestColumn => self.render_manifest_layer(ctx, layout_manager),
                    RenderLayer::CommandDeckColumn => self.render_command_deck_layer(ctx, layout_manager),
                    RenderLayer::PlanetMapColumn => self.render_planet_map_layer(ctx, layout_manager),
                    RenderLayer::Overlays => self.render_overlay_layer(ctx, layout_manager),
                }
            } else {
                // Skip remaining layers to maintain frame rate
                break;
            }
        }
    }
}
```

## Validation Criteria

### Layout Metrics

```rust
pub struct LayoutMetrics {
    pub column_alignment: f32,        // Pixels of misalignment
    pub content_fit: f32,             // How well content fits in columns
    pub touch_target_sizes: f32,      // Minimum touch target compliance
    pub readability_score: f32,       // Text readability metrics
    pub information_density: f32,     // Information per square pixel
}

impl LayoutMetrics {
    pub fn validate_layout(&self) -> LayoutValidation {
        LayoutValidation {
            is_well_aligned: self.column_alignment < 2.0,
            content_fits_well: self.content_fit > 0.9,
            touch_targets_compliant: self.touch_target_sizes > 0.95,
            readable_text: self.readability_score > 0.8,
            optimal_density: self.information_density > 0.7,
        }
    }
}
```

The 3-Column Layout Architecture provides the structural foundation for the Mafia Wars UI strategy, ensuring optimal information density and rapid interaction patterns while maintaining visual clarity and responsive performance across all device types.
