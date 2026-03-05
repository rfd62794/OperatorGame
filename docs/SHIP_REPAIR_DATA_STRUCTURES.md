# Ship Repair Data Structures

> **Status:** RESOURCE SYSTEM SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-021, MAFIA_WARS_UI_STRATEGY.md, SPEC.md §7

## Overview

The Ship Repair system provides the economic foundation for the Astronaut's mission, converting collected scrap and biomass into ship integrity and operational capability. This system creates meaningful resource management decisions while maintaining the rapid progression required for the 30-second gameplay cycle.

## Core Resource Architecture

### Resource Types and Properties

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Biomass,        // 🌱 Organic material for breeding
    Scrap,          // ⚙️ Metal for repairs and construction
    ShipIntegrity,  // 🚀 Hull integrity percentage
    Research,       // 🔬 Tech points for upgrades
    Energy,         // ⚡ Power for operations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceState {
    pub biomass: u64,
    pub scrap: u64,
    pub ship_integrity: f32,    // 0.0 to 1.0
    pub research_points: u64,
    pub energy: u64,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl ResourceState {
    pub fn new() -> Self {
        Self {
            biomass: 1000,
            scrap: 500,
            ship_integrity: 0.8,  // Start at 80% integrity
            research_points: 0,
            energy: 100,
            last_update: chrono::Utc::now(),
        }
    }
    
    pub fn update_resources(&mut self, production_rates: &ProductionRates, delta_time: f32) {
        let biomass_production = (production_rates.biomass_per_second * delta_time) as u64;
        let scrap_production = (production_rates.scrap_per_second * delta_time) as u64;
        let research_production = (production_rates.research_per_second * delta_time) as u64;
        let energy_production = (production_rates.energy_per_second * delta_time) as u64;
        let integrity_decay = production_rates.integrity_decay_per_second * delta_time;
        
        self.biomass = self.biomass.saturating_add(biomass_production);
        self.scrap = self.scrap.saturating_add(scrap_production);
        self.research_points = self.research_points.saturating_add(research_production);
        self.energy = self.energy.saturating_add(energy_production);
        
        // Ship integrity decays over time
        self.ship_integrity = (self.ship_integrity - integrity_decay).max(0.0);
        
        self.last_update = chrono::Utc::now();
    }
    
    pub fn can_afford(&self, cost: &ResourceCost) -> bool {
        self.biomass >= cost.biomass &&
        self.scrap >= cost.scrap &&
        self.energy >= cost.energy &&
        self.research_points >= cost.research_points
    }
    
    pub fn deduct_cost(&mut self, cost: &ResourceCost) -> Result<(), ResourceError> {
        if !self.can_afford(cost) {
            return Err(ResourceError::InsufficientResources);
        }
        
        self.biomass -= cost.biomass;
        self.scrap -= cost.scrap;
        self.energy -= cost.energy;
        self.research_points -= cost.research_points;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCost {
    pub biomass: u64,
    pub scrap: u64,
    pub energy: u64,
    pub research_points: u64,
}

impl ResourceCost {
    pub fn free() -> Self {
        Self {
            biomass: 0,
            scrap: 0,
            energy: 0,
            research_points: 0,
        }
    }
    
    pub fn scrap_only(scrap_amount: u64) -> Self {
        Self {
            biomass: 0,
            scrap: scrap_amount,
            energy: 0,
            research_points: 0,
        }
    }
    
    pub fn biomass_only(biomass_amount: u64) -> Self {
        Self {
            biomass: biomass_amount,
            scrap: 0,
            energy: 0,
            research_points: 0,
        }
    }
}
```

### Production Rate System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionRates {
    pub biomass_per_second: f64,
    pub scrap_per_second: f64,
    pub research_per_second: f64,
    pub energy_per_second: f64,
    pub integrity_decay_per_second: f32,
}

impl ProductionRates {
    pub fn new() -> Self {
        Self {
            biomass_per_second: 0.5,      // 30 biomass per minute
            scrap_per_second: 0.2,        // 12 scrap per minute
            research_per_second: 0.1,     // 6 research per minute
            energy_per_second: 0.3,       // 18 energy per minute
            integrity_decay_per_second: 0.0001, // 0.6% per minute
        }
    }
    
    pub fn calculate_from_deployments(&self, deployments: &[Deployment]) -> ProductionRates {
        let mut rates = ProductionRates::new();
        
        for deployment in deployments {
            let deployment_bonus = self.calculate_deployment_bonus(deployment);
            
            rates.biomass_per_second += deployment_bonus.biomass_per_second;
            rates.scrap_per_second += deployment_bonus.scrap_per_second;
            rates.research_per_second += deployment_bonus.research_per_second;
            rates.energy_per_second += deployment_bonus.energy_per_second;
        }
        
        rates
    }
    
    fn calculate_deployment_bonus(&self, deployment: &Deployment) -> ProductionRates {
        let base_bonus = match deployment.mission_type {
            MissionType::Scavenging => ProductionBonus {
                biomass_per_second: 0.1,
                scrap_per_second: 0.3,  // Higher scrap from scavenging
                research_per_second: 0.0,
                energy_per_second: 0.1,
            },
            MissionType::Research => ProductionBonus {
                biomass_per_second: 0.0,
                scrap_per_second: 0.1,
                research_per_second: 0.4,  // Higher research from research missions
                energy_per_second: 0.2,
            },
            MissionType::Exploration => ProductionBonus {
                biomass_per_second: 0.2,  // Higher biomass from exploration
                scrap_per_second: 0.1,
                research_per_second: 0.1,
                energy_per_second: 0.1,
            },
            MissionType::Defense => ProductionBonus {
                biomass_per_second: 0.05,
                scrap_per_second: 0.15,
                research_per_second: 0.05,
                energy_per_second: 0.2,
            },
        };
        
        // Apply squad composition bonus
        let squad_multiplier = self.calculate_squad_multiplier(&deployment.squad);
        
        ProductionRates {
            biomass_per_second: base_bonus.biomass_per_second * squad_multiplier,
            scrap_per_second: base_bonus.scrap_per_second * squad_multiplier,
            research_per_second: base_bonus.research_per_second * squad_multiplier,
            energy_per_second: base_bonus.energy_per_second * squad_multiplier,
            integrity_decay_per_second: 0.0001, // Decay is constant
        }
    }
    
    fn calculate_squad_multiplier(&self, squad: &[SlimeGenome]) -> f32 {
        let squad_size = squad.len() as f32;
        let base_multiplier = 1.0 + (squad_size - 1.0) * 0.2; // +20% per additional slime
        
        // Trinity bonus
        let trinity_bonus = if self.has_trinity_bonus(squad) {
            1.5 // 50% bonus for Trinity squads
        } else {
            1.0
        };
        
        base_multiplier * trinity_bonus
    }
    
    fn has_trinity_bonus(&self, squad: &[SlimeGenome]) -> bool {
        let has_primary = squad.iter().any(|s| s.culture.layer() == ChromaticLayer::Primary);
        let has_secondary = squad.iter().any(|s| s.culture.layer() == ChromaticLayer::Secondary);
        let has_tertiary = squad.iter().any(|s| s.culture.layer() == ChromaticLayer::Tertiary);
        
        has_primary && has_secondary && has_tertiary
    }
}

#[derive(Debug, Clone)]
pub struct ProductionBonus {
    pub biomass_per_second: f64,
    pub scrap_per_second: f64,
    pub research_per_second: f64,
    pub energy_per_second: f64,
}
```

## Ship Repair System

### Hull Integrity Management

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipIntegrity {
    pub current_integrity: f32,        // 0.0 to 1.0
    pub max_integrity: f32,           // Always 1.0
    pub repair_cost_per_percent: u64, // Scrap cost per 1% repair
    pub critical_threshold: f32,      // Below this = critical damage
    pub warning_threshold: f32,        // Below this = warning state
    pub decay_rate: f32,              // Integrity loss per second
    pub last_repair: chrono::DateTime<chrono::Utc>,
}

impl ShipIntegrity {
    pub fn new() -> Self {
        Self {
            current_integrity: 0.8,     // Start at 80%
            max_integrity: 1.0,
            repair_cost_per_percent: 10,  // 10 scrap per 1% repair
            critical_threshold: 0.2,      // Critical below 20%
            warning_threshold: 0.5,      // Warning below 50%
            decay_rate: 0.0001,          // 0.6% per minute decay
            last_repair: chrono::Utc::now(),
        }
    }
    
    pub fn update_decay(&mut self, delta_time: f32) {
        self.current_integrity = (self.current_integrity - self.decay_rate * delta_time).max(0.0);
    }
    
    pub fn get_integrity_status(&self) -> IntegrityStatus {
        if self.current_integrity < self.critical_threshold {
            IntegrityStatus::Critical
        } else if self.current_integrity < self.warning_threshold {
            IntegrityStatus::Warning
        } else if self.current_integrity < 0.9 {
            IntegrityStatus::Damaged
        } else {
            IntegrityStatus::Healthy
        }
    }
    
    pub fn calculate_repair_cost(&self, target_integrity: f32) -> u64 {
        if target_integrity <= self.current_integrity {
            return 0;
        }
        
        let integrity_to_repair = target_integrity - self.current_integrity;
        let percent_to_repair = integrity_to_repair * 100.0;
        
        (percent_to_repair * self.repair_cost_per_percent as f32) as u64
    }
    
    pub fn can_repair_to(&self, target_integrity: f32, available_scrap: u64) -> bool {
        let repair_cost = self.calculate_repair_cost(target_integrity);
        repair_cost <= available_scrap
    }
    
    pub fn repair(&mut self, target_integrity: f32, cost: u64) -> Result<(), RepairError> {
        if target_integrity <= self.current_integrity {
            return Err(RepairError::NoRepairNeeded);
        }
        
        let actual_cost = self.calculate_repair_cost(target_integrity);
        
        if actual_cost != cost {
            return Err(RepairError::CostMismatch);
        }
        
        self.current_integrity = target_integrity.min(self.max_integrity);
        self.last_repair = chrono::Utc::now();
        
        Ok(())
    }
    
    pub fn get_max_repairable_integrity(&self, available_scrap: u64) -> f32 {
        let max_percent_repairable = (available_scrap as f32 / self.repair_cost_per_percent as f32) / 100.0;
        let integrity_to_repair = max_percent_repairable.min(1.0 - self.current_integrity);
        
        self.current_integrity + integrity_to_repair
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityStatus {
    Critical,    // < 20% - Red alert
    Warning,     // 20-50% - Yellow alert
    Damaged,     // 50-90% - Orange status
    Healthy,     // > 90% - Green status
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairError {
    InsufficientScrap,
    NoRepairNeeded,
    CostMismatch,
    RepairFailed,
}
```

### Repair Actions

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairAction {
    pub action_type: RepairType,
    pub target_integrity: f32,
    pub cost: ResourceCost,
    pub duration: Duration,
    pub requirements: Vec<RepairRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepairType {
    Emergency,     // Quick, expensive repair
    Standard,      // Normal repair
    Comprehensive, // Full repair with bonus
    Upgrade,       // Improves max integrity
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairRequirement {
    MinimumIntegrity(f32),
    AvailableSlimes(usize),
    SpecificCulture(Culture),
    ResearchLevel(u8),
}

impl RepairAction {
    pub fn generate_repair_options(current_integrity: f32, available_scrap: u64) -> Vec<RepairAction> {
        let mut options = Vec::new();
        
        // Emergency repair (quick, expensive)
        if current_integrity < 0.5 {
            let emergency_target = (current_integrity + 0.2).min(1.0);
            let emergency_cost = ResourceCost::scrap_only(
                ((emergency_target - current_integrity) * 100.0 * 15.0) as u64 // 50% more expensive
            );
            
            options.push(RepairAction {
                action_type: RepairType::Emergency,
                target_integrity: emergency_target,
                cost: emergency_cost,
                duration: Duration::from_secs(30),
                requirements: vec![RepairRequirement::MinimumIntegrity(0.1)],
            });
        }
        
        // Standard repair
        let standard_target = (current_integrity + 0.3).min(1.0);
        let standard_cost = ResourceCost::scrap_only(
            ((standard_target - current_integrity) * 100.0 * 10.0) as u64
        );
        
        options.push(RepairAction {
            action_type: RepairType::Standard,
            target_integrity: standard_target,
            cost: standard_cost,
            duration: Duration::from_secs(60),
            requirements: vec![],
        });
        
        // Comprehensive repair (full repair with bonus)
        if current_integrity < 0.8 {
            let comprehensive_cost = ResourceCost {
                biomass: 100,
                scrap: ((1.0 - current_integrity) * 100.0 * 12.0) as u64,
                energy: 50,
                research_points: 25,
            };
            
            options.push(RepairAction {
                action_type: RepairType::Comprehensive,
                target_integrity: 1.0,
                cost: comprehensive_cost,
                duration: Duration::from_secs(120),
                requirements: vec![
                    RepairRequirement::AvailableSlimes(3),
                    RepairRequirement::ResearchLevel(2),
                ],
            });
        }
        
        // Upgrade (requires high integrity)
        if current_integrity > 0.9 {
            let upgrade_cost = ResourceCost {
                biomass: 500,
                scrap: 1000,
                energy: 200,
                research_points: 100,
            };
            
            options.push(RepairAction {
                action_type: RepairType::Upgrade,
                target_integrity: 1.0,
                cost: upgrade_cost,
                duration: Duration::from_secs(300),
                requirements: vec![
                    RepairRequirement::MinimumIntegrity(0.9),
                    RepairRequirement::ResearchLevel(5),
                    RepairRequirement::SpecificCulture(Culture::Crystal), // Need engineering culture
                ],
            });
        }
        
        options
    }
}
```

## Economic Integration

### Resource Flow System

```rust
pub struct ResourceFlowManager {
    pub inflows: Vec<ResourceInflow>,
    pub outflows: Vec<ResourceOutflow>,
    pub conversions: Vec<ResourceConversion>,
    pub storage_limits: HashMap<ResourceType, u64>,
}

#[derive(Debug, Clone)]
pub struct ResourceInflow {
    pub source: InflowSource,
    pub resource_type: ResourceType,
    pub rate: f64, // per second
    pub active: bool,
    pub conditions: Vec<InflowCondition>,
}

#[derive(Debug, Clone)]
pub enum InflowSource {
    Deployment(Uuid),           // From active missions
    Building(Uuid),             // From buildings/structures
    Research(Uuid),             // From research projects
    Automatic,                  // Base production
    Event(String),              // Special events
}

#[derive(Debug, Clone)]
pub enum InflowCondition {
    MinimumIntegrity(f32),
    ActiveDeployments(usize),
    ResearchLevel(u8),
    TimeOfDay(TimeWindow),
}

#[derive(Debug, Clone)]
pub struct ResourceOutflow {
    pub sink: OutflowSink,
    pub resource_type: ResourceType,
    pub rate: f64,
    pub active: bool,
    pub priority: u8, // Lower number = higher priority
}

#[derive(Debug, Clone)]
pub enum OutflowSink {
    ShipDecay,                 // Natural integrity loss
    DeploymentCost(Uuid),       // Mission deployment costs
    BuildingMaintenance(Uuid),  // Building upkeep
    ResearchCost(Uuid),         // Research expenses
    Conversion,                 // Resource conversion
}

#[derive(Debug, Clone)]
pub struct ResourceConversion {
    pub input_resources: HashMap<ResourceType, u64>,
    pub output_resources: HashMap<ResourceType, u64>,
    pub conversion_time: Duration,
    pub efficiency: f32, // 0.0 to 1.0
    pub requirements: Vec<ConversionRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionRequirement {
    MinimumLevel(u8),
    SpecificCulture(Culture),
    BuildingType(String),
    ResearchUnlock(Uuid),
}

impl ResourceFlowManager {
    pub fn new() -> Self {
        Self {
            inflows: Vec::new(),
            outflows: Vec::new(),
            conversions: Vec::new(),
            storage_limits: HashMap::new(),
        }
    }
    
    pub fn calculate_net_flow(&self, resource_type: ResourceType) -> f64 {
        let total_inflow: f64 = self.inflows
            .iter()
            .filter(|inflow| inflow.resource_type == resource_type && inflow.active)
            .map(|inflow| inflow.rate)
            .sum();
        
        let total_outflow: f64 = self.outflows
            .iter()
            .filter(|outflow| outflow.resource_type == resource_type && outflow.active)
            .map(|outflow| outflow.rate)
            .sum();
        
        total_inflow - total_outflow
    }
    
    pub fn update_conditions(&mut self, game_state: &GameState) {
        // Update inflow conditions
        for inflow in &mut self.inflows {
            inflow.active = self.check_inflow_conditions(&inflow.conditions, game_state);
        }
        
        // Update outflow conditions
        for outflow in &mut self.outflows {
            outflow.active = self.check_outflow_conditions(outflow, game_state);
        }
    }
    
    fn check_inflow_conditions(&self, conditions: &[InflowCondition], game_state: &GameState) -> bool {
        conditions.iter().all(|condition| {
            match condition {
                InflowCondition::MinimumIntegrity(min_integrity) => {
                    game_state.ship_integrity >= *min_integrity
                },
                InflowCondition::ActiveDeployments(min_deployments) => {
                    game_state.deployments.len() >= *min_deployments
                },
                InflowCondition::ResearchLevel(min_level) => {
                    game_state.research_level >= *min_level
                },
                InflowCondition::TimeOfDay(window) => {
                    window.contains(chrono::Utc::now().time())
                },
            }
        })
    }
    
    fn check_outflow_conditions(&self, outflow: &ResourceOutflow, game_state: &GameState) -> bool {
        match outflow.sink {
            OutflowSink::ShipDecay => true, // Always active
            OutflowSink::DeploymentCost(_) => false, // One-time costs, not continuous
            OutflowSink::BuildingMaintenance(building_id) => {
                game_state.buildings.contains_key(&building_id)
            },
            OutflowSink::ResearchCost(research_id) => {
                game_state.active_research.contains(&research_id)
            },
            OutflowSink::Conversion => false, // Handled separately
        }
    }
}
```

## UI Integration

### Resource Display Components

```rust
pub struct ResourceDisplayWidget {
    pub resources: ResourceState,
    pub production_rates: ProductionRates,
    pub show_detailed: bool,
    pub animation_enabled: bool,
}

impl ResourceDisplayWidget {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Biomass
            self.render_resource_item(ui, ResourceType::Biomass, "🌱", egui::Color32::GREEN);
            
            ui.separator();
            
            // Scrap
            self.render_resource_item(ui, ResourceType::Scrap, "⚙️", egui::Color32::LIGHT_BLUE);
            
            ui.separator();
            
            // Ship Integrity
            self.render_integrity_item(ui);
            
            ui.separator();
            
            // Research
            self.render_resource_item(ui, ResourceType::Research, "🔬", egui::Color32::PURPLE);
            
            ui.separator();
            
            // Energy
            self.render_resource_item(ui, ResourceType::Energy, "⚡", egui::Color32::YELLOW);
        });
        
        if self.show_detailed {
            self.render_detailed_info(ui);
        }
    }
    
    fn render_resource_item(&self, ui: &mut egui::Ui, resource_type: ResourceType, icon: &str, color: egui::Color32) {
        let amount = self.get_resource_amount(resource_type);
        let rate = self.get_production_rate(resource_type);
        
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(icon);
                ui.colored_color(color, format!("{}", amount));
            });
            
            if rate != 0.0 {
                let rate_text = if rate > 0.0 {
                    format!("+{:.1}/min", rate * 60.0)
                } else {
                    format!("{:.1}/min", rate * 60.0)
                };
                ui.colored_color(color, rate_text);
            }
        });
    }
    
    fn render_integrity_item(&self, ui: &mut egui::Ui) {
        let integrity = self.resources.ship_integrity;
        let integrity_color = if integrity > 0.7 {
            egui::Color32::GREEN
        } else if integrity > 0.3 {
            egui::Color32::YELLOW
        } else {
            egui::Color32::RED
        };
        
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("🚀");
                ui.colored_color(integrity_color, format!("{:.1}%", integrity * 100.0));
            });
            
            let decay_rate = self.production_rates.integrity_decay_per_second * 60.0;
            ui.colored_color(egui::Color32::RED, format!("{:.2}%/min", decay_rate * 100.0));
        });
    }
    
    fn get_resource_amount(&self, resource_type: ResourceType) -> u64 {
        match resource_type {
            ResourceType::Biomass => self.resources.biomass,
            ResourceType::Scrap => self.resources.scrap,
            ResourceType::ShipIntegrity => (self.resources.ship_integrity * 100.0) as u64,
            ResourceType::Research => self.resources.research_points,
            ResourceType::Energy => self.resources.energy,
        }
    }
    
    fn get_production_rate(&self, resource_type: ResourceType) -> f64 {
        match resource_type {
            ResourceType::Biomass => self.production_rates.biomass_per_second,
            ResourceType::Scrap => self.production_rates.scrap_per_second,
            ResourceType::ShipIntegrity => -self.production_rates.integrity_decay_per_second as f64,
            ResourceType::Research => self.production_rates.research_per_second,
            ResourceType::Energy => self.production_rates.energy_per_second,
        }
    }
}
```

## Validation Criteria

### Economic Balance

```rust
pub struct EconomicValidator {
    pub target_production_rates: ProductionRates,
    pub acceptable_variance: f32,
    pub max_storage_limits: HashMap<ResourceType, u64>,
}

impl EconomicValidator {
    pub fn validate_economy(&self, current_rates: &ProductionRates, resources: &ResourceState) -> EconomicValidation {
        let production_balance = self.validate_production_balance(current_rates);
        let storage_usage = self.validate_storage_usage(resources);
        let decay_sustainability = self.validate_decay_sustainability(current_rates, resources);
        
        EconomicValidation {
            production_balanced: production_balance,
            storage_acceptable: storage_usage,
            decay_sustainable: decay_sustainability,
            overall_healthy: production_balance && storage_usage && decay_sustainability,
        }
    }
    
    fn validate_production_balance(&self, current_rates: &ProductionRates) -> bool {
        let biomass_variance = (current_rates.biomass_per_second - self.target_production_rates.biomass_per_second).abs() / self.target_production_rates.biomass_per_second;
        let scrap_variance = (current_rates.scrap_per_second - self.target_production_rates.scrap_per_second).abs() / self.target_production_rates.scrap_per_second;
        
        biomass_variance < self.acceptable_variance && scrap_variance < self.acceptable_variance
    }
    
    fn validate_decay_sustainability(&self, rates: &ProductionRates, resources: &ResourceState) -> bool {
        // Check if scrap production can keep up with integrity decay
        let scrap_per_integrity_point = 10.0; // 10 scrap per 1% integrity
        let integrity_loss_per_minute = rates.integrity_decay_per_second * 60.0;
        let scrap_needed_per_minute = integrity_loss_per_minute * scrap_per_integrity_point;
        let scrap_production_per_minute = rates.scrap_per_second * 60.0;
        
        scrap_production_per_minute >= scrap_needed_per_minute * 1.1 // 10% buffer
    }
}

#[derive(Debug, Clone)]
pub struct EconomicValidation {
    pub production_balanced: bool,
    pub storage_acceptable: bool,
    pub decay_sustainable: bool,
    pub overall_healthy: bool,
}
```

The Ship Repair Data Structures provide the economic foundation that creates meaningful resource management decisions while maintaining the rapid progression required for the 30-second gameplay cycle. This system ensures players must balance resource collection, ship maintenance, and strategic investments to succeed.
