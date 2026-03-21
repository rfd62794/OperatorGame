# Planetary Map 9-Faction Layout

> **Status:** WORLD DESIGN SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-022, CHROMATIC_FRAMEWORK.md, MAFIA_WARS_STRATEGY_INTEGRATION.md

## Overview

The Planetary Map evolves from a simple territorial grid to a sophisticated 9-faction chromatic landscape where each culture controls distinct regions and strategic positions. This layout creates natural conflict zones, strategic depth, and visual representation of the "Shepherd's Dilemma" in territorial control.

## Map Architecture

### Planetary Structure

```rust
pub struct PlanetaryMap {
    pub diameter: f32,           // Planet size
    pub biomes: Vec<BiomeRegion>,
    pub cultural_zones: Vec<CulturalZone>,
    pub strategic_nodes: Vec<StrategicNode>,
    pub trade_routes: Vec<TradeRoute>,
    pub conflict_zones: Vec<ConflictZone>,
}

#[derive(Debug, Clone)]
pub struct BiomeRegion {
    pub id: Uuid,
    pub biome_type: BiomeType,
    pub position: PlanetaryPosition,
    pub size: f32,
    pub native_cultures: Vec<Culture>,
    pub resource_richness: f32,
    pub environmental_hazard: Option<EnvironmentalHazard>,
}

#[derive(Debug, Clone)]
pub struct CulturalZone {
    pub dominant_culture: Culture,
    pub influence_radius: f32,
    pub control_strength: f32,
    pub border_regions: Vec<BorderRegion>,
    pub internal_nodes: Vec<StrategicNode>,
}
```

### 9-Faction Distribution

#### Primary Layer (Inner Triangle - Core Regions)

| Culture | Region Type | Position | Strategic Value | Native Biome |
|---------|-------------|----------|-----------------|--------------|
| Ember | Volcanic Core | Northern Hemisphere | High combat zones | Volcanic |
| Tide | Aquatic Belt | Equatorial Region | Trade and diplomacy | Aquatic |
| Gale | Atmospheric Heights | Mountain Peaks | Scouting and speed | Arid Highlands |

#### Secondary Layer (Middle Triangle - Buffer Regions)

| Culture | Region Type | Position | Strategic Value | Native Biome |
|---------|-------------|----------|-----------------|--------------|
| Orange | Industrial Heartland | Transitional Zones | Engineering and construction | Mixed |
| Marsh | Toxic Swamplands | Low-lying Areas | Defensive positions | Toxic |
| Crystal | Crystal Mountains | Mountain Ranges | Resource extraction | Crystal |

#### Tertiary Layer (Outer Triangle - Frontier Regions)

| Culture | Region Type | Position | Strategic Value | Native Biome |
|---------|-------------|----------|-----------------|--------------|
| Teal | Coastal Shores | Coastlines | Stability and support | Coastal |
| Amber | Desert Wastelands | Arid Regions | Durability and industry | Desert |
| Frost | Polar Ice Caps | Polar Regions | Preservation and stasis | Frozen |

#### Exception Layer

| Culture | Region Type | Position | Strategic Value | Native Biome |
|---------|-------------|----------|-----------------|--------------|
| Void | Void Nexus | Planetary Core | Ultimate control | Void |

## Strategic Layout Design

### Hexagonal Grid System

```rust
pub struct HexagonalGrid {
    pub radius: u32,              // Grid radius from center
    pub hex_size: f32,            // Size of each hex
    pub nodes: HashMap<HexCoord, MapNode>,
    pub adjacency_matrix: AdjacencyMatrix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexagonalGrid {
    pub fn new(radius: u32) -> Self {
        let mut nodes = HashMap::new();
        
        for q in -radius as i32..=radius as i32 {
            for r in -radius as i32..=radius as i32 {
                let s = -q - r;
                if s.abs() <= radius as i32 {
                    let coord = HexCoord { q, r, s };
                    let node = Self::generate_node_at_coord(coord);
                    nodes.insert(coord, node);
                }
            }
        }
        
        Self {
            radius,
            hex_size: 50.0,
            nodes,
            adjacency_matrix: Self::build_adjacency_matrix(&nodes),
        }
    }
    
    fn generate_node_at_coord(coord: HexCoord) -> MapNode {
        let culture = Self::assign_culture_by_position(coord);
        let biome = Self::assign_biome_by_culture(culture);
        let strategic_value = Self::calculate_strategic_value(coord, culture);
        
        MapNode {
            id: Uuid::new_v4(),
            coord,
            culture,
            biome,
            strategic_value,
            control_strength: 0.5,
            owner: None,
            resource_value: Self::calculate_resource_value(culture, biome),
        }
    }
    
    fn assign_culture_by_position(coord: HexCoord) -> Culture {
        let distance_from_center = (coord.q.abs() + coord.r.abs() + coord.s.abs()) as f32 / 2.0;
        let angle = (coord.q as f32).atan2(coord.r as f32);
        
        match distance_from_center {
            0.0..=2.0 => Culture::Void, // Center - Void Nexus
            3.0..=5.0 => Self::assign_primary_culture(angle), // Inner ring
            6.0..=8.0 => Self::assign_secondary_culture(angle), // Middle ring
            _ => Self::assign_tertiary_culture(angle), // Outer ring
        }
    }
}
```

### Cultural Zone Generation

```rust
impl CulturalZone {
    pub fn generate_cultural_zones(grid: &HexagonalGrid) -> Vec<CulturalZone> {
        let mut zones = Vec::new();
        
        for culture in ALL_CULTURES.iter().filter(|c| **c != Culture::Void) {
            let zone = Self::create_cultural_zone(grid, *culture);
            zones.push(zone);
        }
        
        zones
    }
    
    fn create_cultural_zone(grid: &HexagonalGrid, culture: Culture) -> CulturalZone {
        let culture_nodes: Vec<_> = grid.nodes
            .values()
            .filter(|node| node.culture == culture)
            .collect();
        
        let center_of_mass = Self::calculate_center_of_mass(&culture_nodes);
        let influence_radius = Self::calculate_influence_radius(&culture_nodes);
        let border_regions = Self::identify_border_regions(&culture_nodes, grid);
        
        CulturalZone {
            dominant_culture: culture,
            influence_radius,
            control_strength: 0.7, // Initial control
            border_regions,
            internal_nodes: culture_nodes,
        }
    }
    
    fn identify_border_regions(nodes: &[&MapNode], grid: &HexagonalGrid) -> Vec<BorderRegion> {
        let mut border_regions = Vec::new();
        
        for node in nodes {
            let neighbors = grid.get_neighbors(node.coord);
            let has_different_culture_neighbor = neighbors
                .iter()
                .any(|neighbor| neighbor.culture != node.culture);
            
            if has_different_culture_neighbor {
                border_regions.push(BorderRegion {
                    node: *node,
                    contested_neighbors: neighbors
                        .into_iter()
                        .filter(|n| n.culture != node.culture)
                        .collect(),
                    conflict_intensity: Self::calculate_conflict_intensity(node, &neighbors),
                });
            }
        }
        
        border_regions
    }
}
```

## Strategic Node Types

### Node Classification

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Capital,           // Cultural capital - highest value
    Resource,          // Resource extraction node
    Strategic,         // Military/strategic importance
    Trade,             // Commercial hub
    Gateway,           // Passage between regions
    VoidNexus,         // Central control point
    Frontier,          // Expansion opportunity
    Contested,         // High-conflict zone
}

#[derive(Debug, Clone)]
pub struct MapNode {
    pub id: Uuid,
    pub coord: HexCoord,
    pub node_type: NodeType,
    pub culture: Culture,
    pub biome: BiomeType,
    pub strategic_value: f32,
    pub control_strength: f32,
    pub owner: Option<PlayerId>,
    pub resource_value: u64,
    pub fortification_level: u8,
    pub infrastructure: Vec<Infrastructure>,
}

impl MapNode {
    pub fn calculate_income(&self) -> u64 {
        let base_income = match self.node_type {
            NodeType::Capital => 500,
            NodeType::Resource => 300,
            NodeType::Strategic => 200,
            NodeType::Trade => 250,
            NodeType::Gateway => 150,
            NodeType::VoidNexus => 1000,
            NodeType::Frontier => 100,
            NodeType::Contested => 180,
        };
        
        let culture_bonus = if self.owner_culture_matches() { 1.25 } else { 1.0 };
        let fortification_bonus = 1.0 + (self.fortification_level as f32 * 0.1);
        let control_modifier = self.control_strength;
        
        (base_income as f64 * culture_bonus * fortification_bonus * control_modifier) as u64
    }
    
    pub fn owner_culture_matches(&self) -> bool {
        // Check if owner's dominant culture matches node culture
        self.owner
            .and_then(|owner_id| get_player_state(owner_id))
            .map(|player| player.dominant_culture() == self.culture)
            .unwrap_or(false)
    }
}
```

### Specialized Nodes

#### Void Nexus (Center)

```rust
impl MapNode {
    pub fn create_void_nexus(coord: HexCoord) -> Self {
        Self {
            id: Uuid::new_v4(),
            coord,
            node_type: NodeType::VoidNexus,
            culture: Culture::Void,
            biome: BiomeType::Void,
            strategic_value: 10.0, // Maximum value
            control_strength: 1.0, // Always fully controlled
            owner: None, // Initially neutral
            resource_value: 1000,
            fortification_level: 5, // Maximum fortification
            infrastructure: vec![
                Infrastructure::SynthesisChamber,
                Infrastructure::Observatory,
                Infrastructure::TradingHub,
            ],
        }
    }
}
```

#### Cultural Capitals

```rust
impl MapNode {
    pub fn create_cultural_capital(culture: Culture, coord: HexCoord) -> Self {
        let biome = match culture {
            Culture::Ember => BiomeType::Volcanic,
            Culture::Tide => BiomeType::Aquatic,
            Culture::Gale => BiomeType::Arid,
            Culture::Marsh => BiomeType::Toxic,
            Culture::Orange => BiomeType::Industrial,
            Culture::Crystal => BiomeType::Crystal,
            Culture::Teal => BiomeType::Coastal,
            Culture::Amber => BiomeType::Desert,
            Culture::Tundra => BiomeType::Frozen,
            Culture::Void => BiomeType::Void,
        };
        
        Self {
            id: Uuid::new_v4(),
            coord,
            node_type: NodeType::Capital,
            culture,
            biome,
            strategic_value: 8.0,
            control_strength: 0.9,
            owner: None,
            resource_value: 500,
            fortification_level: 4,
            infrastructure: vec![
                Infrastructure::Palace,
                Infrastructure::ResearchLab,
                Infrastructure::MilitaryBase,
            ],
        }
    }
}
```

## Conflict Zone Design

### Natural Conflict Areas

```rust
#[derive(Debug, Clone)]
pub struct ConflictZone {
    pub id: Uuid,
    pub zone_type: ConflictType,
    pub involved_cultures: Vec<Culture>,
    pub contested_nodes: Vec<MapNode>,
    pub intensity_modifier: f32,
    pub special_rules: Vec<SpecialRule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictType {
    CulturalRivalry,    // Adjacent different cultures
    ResourceDispute,    // High-value resource nodes
    StrategicPassage,   // Gateway nodes
    VoidInfluence,     // Areas near Void Nexus
    FrontierConflict,  // Expansion zones
}

impl ConflictZone {
    pub fn identify_conflict_zones(grid: &HexagonalGrid) -> Vec<ConflictZone> {
        let mut zones = Vec::new();
        
        // Cultural rivalry zones
        zones.extend(Self::find_cultural_rivalries(grid));
        
        // Resource dispute zones
        zones.extend(Self::find_resource_disputes(grid));
        
        // Strategic passage zones
        zones.extend(Self::find_strategic_passages(grid));
        
        // Void influence zones
        zones.extend(Self::find_void_influence(grid));
        
        zones
    }
    
    fn find_cultural_rivalries(grid: &HexagonalGrid) -> Vec<ConflictZone> {
        let mut rivalry_zones = Vec::new();
        
        for (coord, node) in &grid.nodes {
            let neighbors = grid.get_neighbors(*coord);
            let different_culture_neighbors: Vec<_> = neighbors
                .into_iter()
                .filter(|n| n.culture != node.culture)
                .collect();
            
            if !different_culture_neighbors.is_empty() {
                let involved_cultures: Vec<_> = different_culture_neighbors
                    .iter()
                    .map(|n| n.culture)
                    .chain(std::iter::once(node.culture))
                    .collect();
                
                rivalry_zones.push(ConflictZone {
                    id: Uuid::new_v4(),
                    zone_type: ConflictType::CulturalRivalry,
                    involved_cultures,
                    contested_nodes: vec![*node],
                    intensity_modifier: 1.2,
                    special_rules: vec![
                        SpecialRule::EnhancedCombat,
                        SpecialRule::CultureBonus,
                    ],
                });
            }
        }
        
        rivalry_zones
    }
}
```

## Trade Route System

### Chromatic Trade Networks

```rust
#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub id: Uuid,
    pub route_type: RouteType,
    pub connected_nodes: Vec<Uuid>,
    pub dominant_cultures: Vec<Culture>,
    pub trade_value: u64,
    pub security_level: f32,
    pub special_conditions: Vec<TradeCondition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteType {
    CulturalExchange,   // Between same culture nodes
    InterCulturalTrade, // Between different cultures
    VoidSupply,         // To/from Void Nexus
    ResourcePipeline,   // Resource extraction routes
}

impl TradeRoute {
    pub fn generate_trade_routes(grid: &HexagonalGrid) -> Vec<TradeRoute> {
        let mut routes = Vec::new();
        
        // Cultural exchange routes
        routes.extend(Self::create_cultural_exchange_routes(grid));
        
        // Inter-cultural trade routes
        routes.extend(Self::create_inter_cultural_routes(grid));
        
        // Void supply routes
        routes.extend(Self::create_void_supply_routes(grid));
        
        routes
    }
    
    fn create_cultural_exchange_routes(grid: &HexagonalGrid) -> Vec<TradeRoute> {
        let mut routes = Vec::new();
        
        for culture in ALL_CULTURES.iter().filter(|c| **c != Culture::Void) {
            let culture_nodes: Vec<_> = grid.nodes
                .values()
                .filter(|node| node.culture == *culture)
                .collect();
            
            // Connect nodes of same culture
            for (i, node_a) in culture_nodes.iter().enumerate() {
                for node_b in culture_nodes.iter().skip(i + 1) {
                    if grid.are_adjacent(node_a.coord, node_b.coord) {
                        routes.push(TradeRoute {
                            id: Uuid::new_v4(),
                            route_type: RouteType::CulturalExchange,
                            connected_nodes: vec![node_a.id, node_b.id],
                            dominant_cultures: vec![*culture],
                            trade_value: 150,
                            security_level: 0.8,
                            special_conditions: vec![
                                TradeCondition::CulturalBonus,
                                TradeCondition::ReducedTaxes,
                            ],
                        });
                    }
                }
            }
        }
        
        routes
    }
}
```

## UI Visualization

### Map Rendering System

```rust
pub struct PlanetaryMapRenderer {
    pub grid: HexagonalGrid,
    pub zoom_level: f32,
    pub center_position: PlanetaryPosition,
    pub show_cultural_zones: bool,
    pub show_trade_routes: bool,
    pub show_conflict_zones: bool,
}

impl PlanetaryMapRenderer {
    pub fn render(&self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        let center = available_size / 2.0;
        
        // Render hexagonal grid
        for (coord, node) in &self.grid.nodes {
            let screen_pos = self.hex_to_screen(coord, center);
            let size = self.calculate_hex_size();
            
            self.render_hex(ui, screen_pos, size, node);
        }
        
        // Render cultural zones
        if self.show_cultural_zones {
            self.render_cultural_zones(ui, center);
        }
        
        // Render trade routes
        if self.show_trade_routes {
            self.render_trade_routes(ui, center);
        }
        
        // Render conflict zones
        if self.show_conflict_zones {
            self.render_conflict_zones(ui, center);
        }
    }
    
    fn render_hex(&self, ui: &mut egui::Ui, pos: egui::Pos2, size: f32, node: &MapNode) {
        let color = self.get_node_color(node);
        let border_color = self.get_border_color(node);
        
        // Draw hexagon
        let mesh = self.create_hex_mesh(pos, size, color, border_color);
        ui.painter().add(mesh);
        
        // Draw cultural symbol
        if size > 20.0 {
            let symbol = node.culture.get_symbol();
            let text_pos = pos + egui::Vec2::new(0.0, -size / 4.0);
            ui.painter().text(
                text_pos,
                egui::Align2::CENTER_CENTER,
                symbol,
                egui::FontId::default(),
                egui::Color32::BLACK,
            );
        }
        
        // Draw control indicator
        if let Some(owner) = node.owner {
            let owner_color = get_player_color(owner);
            let indicator_pos = pos + egui::Vec2::new(size / 3.0, -size / 3.0);
            ui.painter().circle_filled(indicator_pos, size / 8.0, owner_color);
        }
    }
    
    fn get_node_color(&self, node: &MapNode) -> egui::Color32 {
        let base_color = node.culture.get_rgb_color();
        let control_factor = node.control_strength;
        
        egui::Color32::from_rgb(
            (base_color[0] as f32 * control_factor) as u8,
            (base_color[1] as f32 * control_factor) as u8,
            (base_color[2] as f32 * control_factor) as u8,
        )
    }
}
```

## Implementation Tasks

### Core System Development

1. **Update `src/world_map.rs`**: Implement 9-faction distribution
2. **Create Hexagonal Grid**: New coordinate system for planetary layout
3. **Implement Cultural Zones**: Dynamic cultural influence system
4. **Add Conflict Detection**: Automatic conflict zone identification
5. **Design Trade Routes**: Chromatic trade network system

### Visual System Updates

1. **Map Renderer**: New 9-color visualization system
2. **Cultural Indicators**: Clear faction identification
3. **Conflict Zone Highlighting**: Visual conflict indicators
4. **Trade Route Visualization**: Dynamic trade network display
5. **Interactive Elements**: Clickable nodes and zones

## Validation Criteria

- [ ] All 9 cultures have distinct territorial regions
- [ ] Cultural zones create natural conflict boundaries
- [ ] Void Nexus positioned at planetary center
- [ ] Trade routes connect logical cultural pairs
- [ ] Conflict zones provide strategic opportunities
- [ ] Visual system clearly shows cultural distribution

## Future Enhancements

1. **Dynamic Map Evolution**: Cultural borders shift over time
2. **Environmental Events**: Natural disasters affect territories
3. **Player-Created Nodes**: Custom infrastructure development
4. **Advanced Trade Systems**: Complex economic networks
5. **Multi-layer Maps**: Subterranean and orbital layers

The Planetary Map 9-Faction Layout creates a rich, strategic environment where cultural identity, territorial control, and resource management intersect to provide deep, meaningful gameplay that reflects the complexity of the chromatic system.
