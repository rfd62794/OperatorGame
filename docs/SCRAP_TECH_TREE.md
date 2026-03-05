# Scrap Tech Tree System

> **Status:** SHIP REPAIR ECONOMICS v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-020, SHIP_REPAIR_DATA_STRUCTURES.md, SPEC.md §7

## Overview

The Scrap Tech Tree replaces traditional currency systems with a salvage-based economy where the Astronaut repairs their crashed ship using scavenged materials and specialized slime abilities. This creates a meaningful progression system where every repair decision impacts the ship's operational capabilities and the Astronaut's survival chances.

## Ship Architecture

### Component System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipComponent {
    pub id: Uuid,
    pub name: String,
    pub component_type: ComponentType,
    pub current_integrity: f32,        // 0.0 to 1.0
    pub max_integrity: f32,
    pub functionality_level: u8,        // 0-10
    pub repair_requirements: Vec<RepairRequirement>,
    pub operational_effects: Vec<OperationalEffect>,
    pub damage_state: DamageState,
    pub position: ShipPosition,
    pub repair_history: Vec<RepairRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    // Core Systems
    MainReactor,          // Power generation
    LifeSupport,          // Air and water recycling
    Navigation,           // Navigation and guidance
    Communications,        // Long-range communication
    Propulsion,           // Movement and thrust
    
    // Support Systems
    CargoBay,            // Storage and transport
    ResearchLab,          // Research and analysis
    MedBay,              // Medical facilities
    CrewQuarters,         // Living quarters
    MessHall,            // Food preparation
    
    // External Systems
    CommArray,           // Primary communication array
    SensorArray,          // External sensors
    ShieldGenerator,      // Defensive shields
    LandingGear,          // Landing and takeoff
    CargoDoors,          // Cargo access
    SolarPanels,          // Power generation
    
    // Specialized Systems
    SlimeContainment,    // Slime habitat systems
    BioReactor,          // Bio-energy generation
    QuantumDrive,         // FTL drive (endgame)
    TemporalStabilizer,   // Time field stabilization
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageState {
    Operational,         // Fully functional
    Degraded,            // Reduced functionality
    Damaged,             // Major damage
    Critical,            // Critical failure imminent
    Offline,             // Completely non-functional
    Destroyed,           // Beyond repair
}

#[derive(Debug, Clone)]
pub struct RepairRequirement {
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub quality_requirement: ResourceQuality,
    pub specialized_components: Vec<SpecializedComponent>,
    pub tool_requirement: Option<ToolRequirement>,
    pub skill_requirement: Option<SkillRequirement>,
    pub time_requirement: Duration,
    pub difficulty: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecializedComponent {
    PowerConduit,         // Power distribution
    DataCable,             // Data transmission
    CoolantPump,          // Cooling system
    StructuralBeam,        // Structural support
    InsulationPanel,       // Thermal insulation
    ControlCircuit,        // Control systems
    SensorModule,         // Sensor arrays
    ShieldEmitter,         // Shield emitters
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolRequirement {
    WeldingKit,            // For metal repairs
    PlasmaCutter,          // For precision cutting
    NanoAssembler,         // For molecular assembly
    QuantumCalibrator,     // For quantum components
    BioScanner,           // For biological systems
    DiagnosticTool,        // For fault detection
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillRequirement {
    Engineering,           // Technical expertise
    Biology,              // Biological expertise
    QuantumPhysics,        // Advanced physics
    Xenobotany,            // Alien biology
    Navigation,            // Ship navigation
    Communications,        // Communication systems
}
```

### Tech Tree Structure

```rust
#[derive(Debug, Clone)]
pub struct TechTree {
    pub tiers: Vec<TechTier>,
    pub unlocked_nodes: HashSet<TechNode>,
    pub research_progress: HashMap<TechNode, f32>,
    pub available_blueprints: Vec<Blueprint>,
    pub current_tier: u8,
    pub total_tiers: u8,
}

#[derive(Debug, Clone)]
pub struct TechTier {
    pub tier_number: u8,
    pub tier_name: String,
    pub description: String,
    pub unlock_requirements: Vec<UnlockRequirement>,
    pub nodes: Vec<TechNode>,
    pub tier_bonuses: Vec<TierBonus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TechNode {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub node_type: TechNodeType,
    pub prerequisites: Vec<TechNode>,
    pub costs: ResourceCost,
    pub research_time: Duration,
    pub blueprint: Option<Blueprint>,
    pub unlocked: bool,
    pub research_progress: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TechNodeType {
    ComponentRepair,       // Repair specific component
    SystemUpgrade,        // Upgrade entire system
    NewTechnology,        // Unlock new technology
    EfficiencyBoost,       // Improve efficiency
    SpecialAbility,       // Unlock special ability
    ResourceProcessor,    // New resource processing
    DefensiveSystem,      // Defensive capabilities
    PropulsionUpgrade,    // Movement improvements
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub component_type: ComponentType,
    pub construction_requirements: Vec<ConstructionRequirement>,
    pub operational_bonuses: Vec<OperationalBonus>,
    pub quality_level: QualityLevel,
    pub durability: f32,
    pub maintenance_interval: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualityLevel {
    Basic,        // Standard quality
    Advanced,     // High quality
    Superior,     // Very high quality
    Experimental, // Cutting edge
    Quantum,      // Quantum technology
    BioEnhanced,  // Bio-integrated
    VoidPowered,   // Void technology
}
```

### Repair System

```rust
#[derive(Debug, Clone)]
pub struct RepairSystem {
    pub available_tools: Vec<Tool>,
    pub available_skills: Vec<Skill>,
    pub repair_queue: Vec<RepairJob>,
    pub active_repairs: Vec<ActiveRepair>,
    pub repair_history: Vec<RepairRecord>,
    pub efficiency_modifiers: Vec<EfficiencyModifier>,
}

#[derive(Debug, Clone)]
pub struct RepairJob {
    pub id: Uuid,
    pub component_id: Uuid,
    pub priority: RepairPriority,
    pub assigned_slimes: Vec<Uuid>,
    pub required_tools: Vec<ToolRequirement>,
    pub required_skills: Vec<SkillRequirement>,
    pub estimated_time: Duration,
    pub resource_cost: ResourceCost,
    pub difficulty: f32,
    pub status: RepairStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairPriority {
    Critical,      // Essential for survival
    High,          // Important for operations
    Medium,        // Improves functionality
    Low,           // Minor improvements
    Cosmetic,      // Visual improvements only
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairStatus {
    Queued,        // Waiting to start
    InProgress,    // Currently being repaired
    AwaitingResources, // Waiting for materials
    AwaitingSlimes,   // Waiting for slime assistance
    Completed,      // Successfully repaired
    Failed,         // Repair failed
    Cancelled,       // Repair cancelled
}

impl RepairSystem {
    pub fn new() -> Self {
        Self {
            available_tools: vec![
                Tool::WeldingKit,
                Tool::PlasmaCutter,
                Tool::NanoAssembler,
                Tool::DiagnosticTool,
            ],
            available_skills: vec![
                Skill::Engineering,
                Skill::Biology,
                Skill::QuantumPhysics,
            ],
            repair_queue: Vec::new(),
            active_repairs: Vec::new(),
            repair_history: Vec::new(),
            efficiency_modifiers: Vec::new(),
        }
    }
    
    pub fn create_repair_job(
        &mut self,
        component: &ShipComponent,
        priority: RepairPriority
    ) -> Result<Uuid, RepairError> {
        if component.damage_state == DamageState::Destroyed {
            return Err(RepairError::ComponentDestroyed);
        }
        
        let repair_job = RepairJob {
            id: Uuid::new_v4(),
            component_id: component.id,
            priority,
            assigned_slimes: Vec::new(),
            required_tools: component.repair_requirements
                .iter()
                .filter_map(|req| req.tool_requirement.clone())
                .collect(),
            required_skills: component.repair_requirements
                .iter()
                .filter_map(|req| req.skill_requirement.clone())
                .collect(),
            estimated_time: self.estimate_repair_time(component),
            resource_cost: self.calculate_repair_cost(component),
            difficulty: self.calculate_repair_difficulty(component),
            status: RepairStatus::Queued,
        };
        
        let job_id = repair_job.id;
        self.repair_queue.push(repair_job);
        
        Ok(job_id)
    }
    
    pub fn assign_slimes_to_repair(&mut self, job_id: Uuid, slime_ids: Vec<Uuid>) -> Result<(), RepairError> {
        let job = self.repair_queue.iter_mut()
            .find(|job| job.id == job_id)
            .ok_or(RepairError::JobNotFound)?;
        
        // Check if slimes are suitable
        for slime_id in &slime_ids {
            if !self.is_slime_suitable_for_repair(*slime_id, job) {
                return Err(RepairError::UnsuitableSlime);
            }
        }
        
        job.assigned_slimes = slime_ids;
        job.status = RepairStatus::InProgress;
        
        // Move to active repairs
        if let Some(index) = self.repair_queue.iter().position(|job| job.id == job_id) {
            let repair_job = self.repair_queue.remove(index);
            self.active_repairs.push(repair_job);
        }
        
        Ok(())
    }
    
    pub fn is_slime_suitable_for_repair(&self, slime_id: Uuid, job: &RepairJob) -> bool {
        let slime = get_slime_by_id(slime_id).unwrap();
        
        // Check if slime has required skills
        for skill_requirement in &job.required_skills {
            match skill_requirement {
                SkillRequirement::Engineering => {
                    if slime.culture != Culture::Orange && slime.culture != Culture::Amber {
                        return false; // Need engineering culture
                    }
                },
                SkillRequirement::Biology => {
                    if slime.culture != Culture::Marsh && slime.culture != Culture::Teal {
                        return false; // Need biological culture
                    }
                },
                SkillRequirement::QuantumPhysics => {
                    if slime.generation < 7 {
                        return false; // Need advanced slime
                    }
                },
                _ => {}
            }
        }
        
        // Check if slime is available
        matches!(slime.state, OperatorState::Idle)
    }
    
    pub fn process_repair(&mut self, job_id: Uuid, delta_time: Duration) -> RepairResult {
        let job_index = self.active_repairs.iter()
            .position(|job| job.id == job_id);
        
        if let Some(index) = job_index {
            let job = &mut self.active_repairs[index];
            
            // Update repair progress
            let progress_increment = self.calculate_progress_increment(job, delta_time);
            job.estimated_time = job.estimated_time.saturating_sub(Duration::from_secs(1));
            
            // Check if repair is complete
            if job.estimated_time.is_zero() {
                self.complete_repair(job_id)
            } else {
                RepairResult::InProgress {
                    progress: 1.0 - (job.estimated_time.as_secs_f32() / 
                        self.calculate_initial_repair_time(job).as_secs_f32()),
                    time_remaining: job.estimated_time,
                }
            }
        } else {
            RepairResult::JobNotFound
        }
    }
    
    fn complete_repair(&mut self, job_id: Uuid) -> RepairResult {
        let job_index = self.active_repairs.iter()
            .position(|job| job.id == job_id);
        
        if let Some(index) = job_index {
            let job = self.active_repairs.remove(index);
            
            // Update component
            if let Some(component) = get_component_by_id(job.component_id) {
                component.current_integrity = component.max_integrity;
                component.damage_state = DamageState::Operational;
                component.functionality_level = (component.functionality_level + 1).min(10);
                component.repair_history.push(RepairRecord {
                    timestamp: chrono::Utc::now(),
                    repair_type: job.repair_type,
                    time_taken: job.initial_time - job.estimated_time,
                    resources_used: job.resource_cost,
                    slimes_involved: job.assigned_slimes.clone(),
                });
            }
            
            // Apply operational effects
            self.apply_operational_effects(&job);
            
            // Record in history
            self.repair_history.push(RepairRecord {
                timestamp: chrono::Utc::now(),
                repair_type: job.repair_type,
                time_taken: job.initial_time - job.estimated_time,
                resources_used: job.resource_cost,
                slimes_involved: job.assigned_slimes.clone(),
            });
            
            RepairResult::Completed {
                component_id: job.component_id,
                new_integrity: 1.0,
                bonuses_applied: job.applied_bonuses,
            }
        } else {
            RepairResult::JobNotFound
        }
    }
}
```

### Resource Integration

```rust
#[derive(Debug, Clone)]
pub struct ScrapEconomy {
    pub scrap_tiers: Vec<ScrapTier>,
    pub salvage_rates: HashMap<ComponentType, f32>,
    pub processing_efficiency: f32,
    pub market_prices: HashMap<ResourceType, f32>,
    pub trading_posts: Vec<TradingPost>,
}

#[derive(Debug, Clone)]
pub struct ScrapTier {
    pub tier_number: u8,
    pub tier_name: String,
    pub quality_range: (f32, f32),
    pub processing_difficulty: f32,
    pub special_properties: Vec<ScrapProperty>,
    pub required_tools: Vec<ToolRequirement>,
    pub yield_multiplier: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrapProperty {
    Conductive,           // Good for electrical systems
    Structural,           // Good for structural repairs
    ThermalResistant,     // Good for heat systems
    BioCompatible,         // Good for biological systems
    QuantumStable,        // Good for quantum systems
    SelfRepairing,         // Can repair minor damage
}

impl ScrapEconomy {
    pub fn new() -> Self {
        Self {
            scrap_tiers: vec![
                ScrapTier {
                    tier_number: 1,
                    tier_name: "Raw Scrap".to_string(),
                    quality_range: (0.3, 0.5),
                    processing_difficulty: 0.2,
                    special_properties: vec![],
                    required_tools: vec![],
                    yield_multiplier: 1.0,
                },
                ScrapTier {
                    tier_number: 2,
                    tier_name: "Refined Scrap".to_string(),
                    quality_range: (0.5, 0.7),
                    processing_difficulty: 0.4,
                    special_properties: vec![ScrapProperty::Structural],
                    required_tools: vec![ToolRequirement::WeldingKit],
                    yield_multiplier: 1.2,
                },
                ScrapTier {
                    tier_number: 3,
                    tier_name: "Alloy Scrap".to_string(),
                    quality_range: (0.7, 0.85),
                    processing_difficulty: 0.6,
                    special_properties: vec![
                        ScrapProperty::Conductive,
                        ScrapProperty::ThermalResistant
                    ],
                    required_tools: vec![
                        ToolRequirement::WeldingKit,
                        ToolRequirement::PlasmaCutter
                    ],
                    yield_multiplier: 1.5,
                },
                ScrapTier {
                    tier_number: 4,
                    tier_name: "Composite Scrap".to_string(),
                    quality_range: (0.85, 0.95),
                    processing_difficulty: 0.8,
                    special_properties: vec![
                        ScrapProperty::Conductive,
                        ScrapProperty::Structural,
                        ScrapProperty::BioCompatible
                    ],
                    required_tools: vec![
                        ToolRequirement::WeldingKit,
                        ToolRequirement::PlasmaCutter,
                        ToolRequirement::NanoAssembler
                    ],
                    yield_multiplier: 2.0,
                },
                ScrapTier {
                    tier_number: 5,
                    tier_name: "Quantum Scrap".to_string(),
                    quality_range: (0.95, 1.0),
                    processing_difficulty: 0.9,
                    special_properties: vec![
                        ScrapProperty::QuantumStable,
                        ScrapProperty::SelfRepairing
                    ],
                    required_tools: vec![
                        ToolRequirement::PlasmaCutter,
                        ToolRequirement::NanoAssembler,
                        ToolRequirement::QuantumCalibrator
                    ],
                    yield_multiplier: 3.0,
                },
            ],
            salvage_rates: HashMap::from([
                (ComponentType::MainReactor, 0.8),
                (ComponentType::LifeSupport, 0.6),
                (ComponentType::Navigation, 0.7),
                (ComponentType::Communications, 0.5),
                (ComponentType::Propulsion, 0.9),
                (ComponentType::ShieldGenerator, 0.4),
            ]),
            processing_efficiency: 1.0,
            market_prices: HashMap::new(),
            trading_posts: Vec::new(),
        }
    }
    
    pub fn salvage_from_component(&self, component: &ShipComponent) -> Vec<ScrapPiece> {
        let base_scrap_amount = self.calculate_base_scrap_amount(component);
        let tier = self.determine_scrap_tier(component);
        
        let mut scrap_pieces = Vec::new();
        
        for i in 0..base_scrap_amount {
            let quality = rand::random::<f32>() * (tier.quality_range.1 - tier.quality_range.0) + tier.quality_range.0;
            
            scrap_pieces.push(ScrapPiece {
                id: Uuid::new_v4(),
                tier: tier.tier_number,
                quality,
                source_component: component.component_type,
                properties: tier.special_properties.clone(),
                processing_required: tier.required_tools.clone(),
                estimated_value: self.calculate_scrap_value(tier.tier_number, quality),
            });
        }
        
        scrap_pieces
    }
    
    pub fn process_scrap(&self, scrap_pieces: Vec<ScrapPiece>) -> Vec<ProcessedScrap> {
        let mut processed = Vec::new();
        
        for piece in scrap_pieces {
            let tier = self.scrap_tiers.iter()
                .find(|t| t.tier_number == piece.tier)
                .unwrap();
            
            let quality_modifier = piece.quality / tier.quality_range.1;
            let efficiency_modifier = self.processing_efficiency;
            let yield_modifier = tier.yield_multiplier;
            
            let final_quality = quality_modifier * efficiency_modifier;
            let final_quantity = (1.0 * yield_modifier) as u64;
            
            processed.push(ProcessedScrap {
                original_piece: piece,
                final_quality: final_quality,
                final_quantity,
                processing_time: Duration::from_secs(
                    (tier.processing_difficulty * 3600.0) as u64
                ),
                special_properties_applied: tier.special_properties.clone(),
            });
        }
        
        processed
    }
}

#[derive(Debug, Clone)]
pub struct ScrapPiece {
    pub id: Uuid,
    pub tier: u8,
    pub quality: f32,
    pub source_component: ComponentType,
    pub properties: Vec<ScrapProperty>,
    pub processing_required: Vec<ToolRequirement>,
    pub estimated_value: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessedScrap {
    pub original_piece: ScrapPiece,
    pub final_quality: f32,
    pub final_quantity: u64,
    pub processing_time: Duration,
    pub special_properties_applied: Vec<ScrapProperty>,
}
```

## Implementation Tasks

### Core System Development

1. **Create Ship Component System**: Define all ship components
2. **Build Tech Tree Structure**: Implement tiered unlock system
3. **Develop Repair System**: Create repair job management
4. **Implement Scrap Economy**: Salvage and processing system
5. **Create Integration Layer**: Connect to game state

### UI Integration

1. **Ship Map UI**: Visual representation of ship components
2. **Tech Tree Interface**: Research and unlock interface
3. **Repair Queue Display**: Active repair management
4. **Scrap Inventory**: Resource management interface

### Balance and Testing

1. **Repair Balance**: Ensure repair costs are meaningful
2. **Tech Tree Progression**: Balance unlock requirements
3. **Economy Balance**: Ensure scrap economy is sustainable
4. **Component Interactions**: Test component dependencies

## Validation Criteria

- [ ] All ship components have appropriate repair requirements
- [ ] Tech tree provides meaningful progression
- [ ] Repair system creates strategic decisions
- [ ] Scrap economy is balanced and sustainable
- [ ] Component interactions work correctly
- [ ] System scales to endgame content

The Scrap Tech Tree System creates a meaningful progression system where the Astronaut must strategically repair their ship using scavenged materials and specialized slime abilities, creating a compelling connection between resource management and survival.
