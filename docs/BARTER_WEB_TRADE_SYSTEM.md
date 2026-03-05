# Barter Web Trade System

> **Status:** ECONOMIC NETWORK SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-026, LIVING_ECONOMY_SUB_FACTIONS.md, SHIP_REPAIR_DATA_STRUCTURES.md

## Overview

The Barter Web Trade System replaces traditional shop mechanics with a dynamic, relationship-based trading network where cultures exchange resources based on their needs, personalities, and historical interactions. The Shepherd acts as the essential middleman, facilitating trades that cultures cannot conduct directly due to hostility, isolation, or lack of trust.

## Trade Network Architecture

### Barter Node System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarterNode {
    pub id: Uuid,
    pub culture: Culture,
    pub node_type: NodeType,
    pub inventory: ResourceInventory,
    pub trade_needs: Vec<TradeNeed>,
    pub trade_offers: Vec<TradeOffer>,
    pub trade_relationships: HashMap<Culture, TradeRelationship>,
    pub trade_policies: TradePolicies,
    pub market_position: MarketPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInventory {
    pub resources: HashMap<ResourceType, ResourceStack>,
    pub production_capacity: HashMap<ResourceType, f32>,
    pub consumption_rate: HashMap<ResourceType, f32>,
    pub storage_capacity: HashMap<ResourceType, u64>,
    pub quality_levels: HashMap<ResourceType, ResourceQuality>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStack {
    pub amount: u64,
    pub quality: ResourceQuality,
    pub origin: Option<Culture>,
    pub acquisition_date: chrono::DateTime<chrono::Utc>,
    pub perishable: bool,
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct TradeNeed {
    pub resource_type: ResourceType,
    pub required_quantity: u64,
    pub urgency: Urgency,
    pub max_price_per_unit: f32,
    pub preferred_quality: ResourceQuality,
    pub preferred_partners: Vec<Culture>,
    pub forbidden_partners: Vec<Culture>,
    pub need_type: NeedType,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct TradeOffer {
    pub resource_type: ResourceType,
    pub available_quantity: u64,
    pub min_price_per_unit: f32,
    pub quality: ResourceQuality,
    pub preferred_partners: Vec<Culture>,
    pub forbidden_partners: Vec<Culture>,
    pub trade_terms: TradeTerms,
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeedType {
    Critical,        // Essential for survival
    Production,      // Needed for production
    Consumption,     // Needed for consumption
    Luxury,          // Optional luxury item
    Strategic,       // Strategic importance
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeTerms {
    Immediate,       // Immediate exchange
    Deferred,        // Future delivery
    Recurring,       // Regular deliveries
    Conditional,     // Based on conditions
    Gift,            // No payment required
}
```

### Cultural Trade Patterns

```rust
impl Culture {
    pub fn get_trade_pattern(&self) -> TradePattern {
        match self {
            Culture::Ember => TradePattern {
                primary_exports: vec![ResourceType::Scrap, ResourceType::Energy],
                primary_imports: vec![ResourceType::Biomass, ResourceType::Research],
                trade_style: TradeStyle::Aggressive,
                price_strategy: PriceStrategy::HighMargin,
                negotiation_tendency: NegotiationTendency::Dominant,
                preferred_partners: vec![Culture::Crystal, Culture::Amber],
                avoided_partners: vec![Culture::Marsh, Culture::Tide],
            },
            Culture::Tide => TradePattern {
                primary_exports: vec![ResourceType::Biomass, ResourceType::Research],
                primary_imports: vec![ResourceType::Scrap, ResourceType::Energy],
                trade_style: TradeStyle::Cooperative,
                price_strategy: PriceStrategy::FairMarket,
                negotiation_tendency: NegotiationTendency::Flexible,
                preferred_partners: vec![Culture::Marsh, Culture::Teal],
                avoided_partners: vec![Culture::Ember, Culture::Crystal],
            },
            Culture::Marsh => TradePattern {
                primary_exports: vec![ResourceType::Biomass, ResourceType::Research],
                primary_imports: vec![ResourceType::Scrap, ResourceType::Energy],
                trade_style: TradeStyle::Generous,
                price_strategy: PriceStrategy::LowMargin,
                negotiation_tendency: NegotiationTendency::Accommodating,
                preferred_partners: vec![Culture::Tide, Culture::Teal],
                avoided_partners: vec![],
            },
            Culture::Crystal => TradePattern {
                primary_exports: vec![ResourceType::Energy, ResourceType::Research],
                primary_imports: vec![ResourceType::Scrap, ResourceType::Biomass],
                trade_style: TradeStyle::Selective,
                price_strategy: PriceStrategy::Premium,
                negotiation_tendency: NegotiationTendency::Rigid,
                preferred_partners: vec![Culture::Ember, Culture::Orange],
                avoided_partners: vec![Culture::Gale, Culture::Teal],
            },
            // ... other cultures
            _ => TradePattern::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradePattern {
    pub primary_exports: Vec<ResourceType>,
    pub primary_imports: Vec<ResourceType>,
    pub trade_style: TradeStyle,
    pub price_strategy: PriceStrategy,
    pub negotiation_tendency: NegotiationTendency,
    pub preferred_partners: Vec<Culture>,
    pub avoided_partners: Vec<Culture>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeStyle {
    Aggressive,      // High prices, expansionist
    Cooperative,     // Fair prices, collaborative
    Generous,        // Low prices, helpful
    Selective,       // Chooses partners carefully
    Isolationist,    // Limited trade
    Exploitative,    // Takes advantage of others
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriceStrategy {
    HighMargin,      // High profit margins
    FairMarket,      // Market-based pricing
    LowMargin,       // Low profit margins
    Premium,         // Premium pricing
    Variable,        // Prices fluctuate
    Fixed,           // Stable prices
}
```

## Dynamic Market Mechanics

### Supply and Demand System

```rust
#[derive(Debug, Clone)]
pub struct MarketDynamics {
    pub global_supply: HashMap<ResourceType, f64>,
    pub global_demand: HashMap<ResourceType, f64>,
    pub price_history: HashMap<ResourceType, Vec<PricePoint>>,
    pub market_sentiment: MarketSentiment,
    pub seasonal_modifiers: HashMap<ResourceType, f32>,
    pub event_modifiers: Vec<EventModifier>,
}

#[derive(Debug, Clone)]
pub struct PricePoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub price: f32,
    pub volume: u64,
    pub volatility: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketSentiment {
    Bullish,         // Prices rising
    Bearish,         // Prices falling
    Stable,          // Prices stable
    Volatile,        // High volatility
    Uncertain,       // Unclear direction
}

impl MarketDynamics {
    pub fn calculate_market_price(&self, resource_type: ResourceType, context: &MarketContext) -> f32 {
        let base_price = self.get_base_price(resource_type);
        let supply_demand_ratio = self.calculate_supply_demand_ratio(resource_type);
        let sentiment_modifier = self.get_sentiment_modifier();
        let seasonal_modifier = self.seasonal_modifiers.get(&resource_type).unwrap_or(&1.0);
        let event_modifier = self.calculate_event_modifier(resource_type);
        
        let market_price = base_price * 
            supply_demand_ratio * 
            sentiment_modifier * 
            seasonal_modifier * 
            event_modifier;
        
        market_price.max(0.1) // Minimum price floor
    }
    
    fn calculate_supply_demand_ratio(&self, resource_type: ResourceType) -> f32 {
        let supply = self.global_supply.get(&resource_type).unwrap_or(&0.0);
        let demand = self.global_demand.get(&resource_type).unwrap_or(&0.0);
        
        if *demand == 0.0 {
            1.0 // No demand, normal price
        } else {
            let ratio = supply / demand;
            // Apply logarithmic scaling for more realistic price changes
            (ratio.ln() + 1.0).max(0.1)
        }
    }
    
    fn get_sentiment_modifier(&self) -> f32 {
        match self.market_sentiment {
            MarketSentiment::Bullish => 1.2,
            MarketSentiment::Bearish => 0.8,
            MarketSentiment::Stable => 1.0,
            MarketSentiment::Volatile => 1.0 + (rand::random::<f32>() - 0.5) * 0.4,
            MarketSentiment::Uncertain => 0.95 + (rand::random::<f32>() - 0.5) * 0.1,
        }
    }
    
    fn calculate_event_modifier(&self, resource_type: ResourceType) -> f32 {
        let mut modifier = 1.0;
        
        for event in &self.event_modifiers {
            if event.affected_resources.contains(&resource_type) {
                modifier *= event.modifier;
            }
        }
        
        modifier
    }
    
    pub fn update_market_sentiment(&mut self, recent_trades: &[TradeRecord]) {
        if recent_trades.len() < 10 {
            return;
        }
        
        let price_changes: Vec<f32> = recent_trades
            .windows(2)
            .map(|window| {
                let old_price = window[0].price;
                let new_price = window[1].price;
                (new_price - old_price) / old_price
            })
            .collect();
        
        let avg_change = price_changes.iter().sum::<f32>() / price_changes.len() as f32;
        let volatility = price_changes.iter()
            .map(|change| (change - avg_change).abs())
            .sum::<f32>() / price_changes.len() as f32;
        
        self.market_sentiment = if volatility > 0.1 {
            MarketSentiment::Volatile
        } else if avg_change > 0.05 {
            MarketSentiment::Bullish
        } else if avg_change < -0.05 {
            MarketSentiment::Bearish
        } else {
            MarketSentiment::Stable
        };
    }
}
```

### Trade Route Generation

```rust
#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub price_per_unit: f32,
    pub total_value: f32,
    pub route_type: RouteType,
    pub difficulty: f32,
    pub duration: Duration,
    pub risk_factors: Vec<RiskFactor>,
    pub requirements: Vec<TradeRequirement>,
    pub shepherd_commission: Option<ShepherdCommission>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteType {
    Direct,          // Direct trade between cultures
    ShepherdMediated, // Requires shepherd intervention
    SubFaction,       // Sub-faction specific trade
    Emergency,        // Urgent trade
    LongTerm,         // Ongoing relationship
}

#[derive(Debug, Clone)]
pub struct RiskFactor {
    pub factor_type: RiskType,
    pub severity: f32,
    pub mitigation: Option<MitigationStrategy>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskType {
    CulturalConflict,  // Cultural hostility
    Environmental,     // Environmental hazards
    Distance,          // Long distance
    Security,          // Security threats
    Political,         // Political instability
    Economic,          // Economic instability
}

impl TradeRoute {
    pub fn calculate_route_difficulty(&self, buyer: &BarterNode, seller: &BarterNode) -> f32 {
        let cultural_difficulty = self.calculate_cultural_difficulty(buyer, seller);
        let distance_difficulty = self.calculate_distance_difficulty(buyer, seller);
        let environmental_difficulty = self.calculate_environmental_difficulty();
        let security_difficulty = self.calculate_security_difficulty();
        
        (cultural_difficulty + distance_difficulty + environmental_difficulty + security_difficulty) / 4.0
    }
    
    fn calculate_cultural_difficulty(&self, buyer: &BarterNode, seller: &BarterNode) -> f32 {
        let relationship = buyer.trade_relationships.get(&seller.culture).unwrap_or(&TradeRelationship::Neutral);
        
        match relationship {
            TradeRelationship::Allied { trust_level } => 0.1,
            TradeRelationship::Friendly { cooperation_level } => 0.3,
            TradeRelationship::Neutral => 0.5,
            TradeRelationship::Suspicious { hostility_level } => 0.7,
            TradeRelationship::Hostile { aggression_level } => 0.9,
        }
    }
    
    fn calculate_distance_difficulty(&self, buyer: &BarterNode, seller: &BarterNode) -> f32 {
        let distance = self.calculate_distance(buyer.position, seller.position);
        
        // Distance affects difficulty logarithmically
        (distance.ln() / 10.0).min(1.0)
    }
    
    pub fn requires_shepherd(&self) -> bool {
        matches!(self.route_type, RouteType::ShepherdMediated) ||
        self.difficulty > 0.7 ||
        self.risk_factors.iter().any(|risk| risk.severity > 0.8)
    }
}
```

## Shepherd Commission System

### Commission Generation

```rust
#[derive(Debug, Clone)]
pub struct ShepherdCommission {
    pub id: Uuid,
    pub commission_type: CommissionType,
    pub title: String,
    pub description: String,
    pub client_id: Uuid,
    pub target_id: Option<Uuid>,
    pub objectives: Vec<CommissionObjective>,
    pub rewards: CommissionRewards,
    pub requirements: CommissionRequirements,
    pub time_limit: Duration,
    pub difficulty: f32,
    pub urgency: Urgency,
    pub status: CommissionStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommissionType {
    TradeFacilitation,    // Help two cultures trade
    ResourceTransport,    // Transport resources
    DiplomaticMission,    // Improve relationships
    CulturalExchange,     // Enable cultural exchange
    ConflictResolution,   // Resolve conflicts
    SubFactionCreation,   // Help create sub-faction
    MarketStabilization,  // Stabilize market
}

#[derive(Debug, Clone)]
pub struct CommissionObjective {
    pub objective_type: ObjectiveType,
    pub target: ObjectiveTarget,
    pub quantity: u64,
    pub quality_requirement: Option<ResourceQuality>,
    pub time_constraint: Option<Duration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectiveType {
    DeliverResource,
    FacilitateTrade,
    ImproveRelationship,
    EstablishContact,
    ResolveConflict,
    GatherInformation,
}

#[derive(Debug, Clone)]
pub enum ObjectiveTarget {
    Culture(Culture),
    Node(Uuid),
    SubFaction(Uuid),
    Resource(ResourceType),
    Relationship((Culture, Culture)),
}

#[derive(Debug, Clone)]
pub struct CommissionRewards {
    pub monetary_reward: u64,
    pub resource_rewards: HashMap<ResourceType, u64>,
    pub reputation_gain: f32,
    pub relationship_improvements: HashMap<Culture, f32>,
    pub special_rewards: Vec<SpecialReward>,
    pub experience_points: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialReward {
    CulturalInsight(Culture),
    TradeDiscount(Culture),
    SubFactionAccess(Uuid),
    UniqueResource(ResourceType),
    TechnologyUnlock(Uuid),
}

impl ShepherdCommission {
    pub fn generate_trade_facilitation(
        buyer: &BarterNode,
        seller: &BarterNode,
        trade_route: &TradeRoute
    ) -> Option<Self> {
        // Check if trade requires shepherd intervention
        if !trade_route.requires_shepherd() {
            return None;
        }
        
        let commission = ShepherdCommission {
            id: Uuid::new_v4(),
            commission_type: CommissionType::TradeFacilitation,
            title: format!("Facilitate Trade: {} ↔ {}", buyer.culture, seller.culture),
            description: format!("Help {} and {} establish a trade relationship for {}",
                buyer.culture, seller.culture, trade_route.resource_type),
            client_id: buyer.id,
            target_id: Some(seller.id),
            objectives: vec![
                CommissionObjective {
                    objective_type: ObjectiveType::FacilitateTrade,
                    target: ObjectiveTarget::Relationship((buyer.culture, seller.culture)),
                    quantity: trade_route.quantity,
                    quality_requirement: None,
                    time_constraint: Some(trade_route.duration),
                }
            ],
            rewards: CommissionRewards {
                monetary_reward: (trade_route.total_value * 0.1) as u64, // 10% commission
                resource_rewards: HashMap::new(),
                reputation_gain: 0.05,
                relationship_improvements: HashMap::from([
                    (buyer.culture, 0.1),
                    (seller.culture, 0.1),
                ]),
                special_rewards: vec![],
                experience_points: (trade_route.difficulty * 100.0) as u32,
            },
            requirements: CommissionRequirements {
                minimum_reputation: 0.3,
                required_resources: HashMap::from([
                    (ResourceType::Energy, 100),
                    (ResourceType::Biomass, 50),
                ]),
                required_abilities: vec![],
                cultural_knowledge: vec![buyer.culture, seller.culture],
            },
            time_limit: trade_route.duration,
            difficulty: trade_route.difficulty,
            urgency: Urgency::Medium,
            status: CommissionStatus::Available,
        };
        
        Some(commission)
    }
    
    pub fn generate_diplomatic_mission(
        culture_a: &BarterNode,
        culture_b: &BarterNode,
        conflict_level: f32
    ) -> Option<Self> {
        if conflict_level < 0.5 {
            return None; // No diplomatic mission needed
        }
        
        let commission = ShepherdCommission {
            id: Uuid::new_v4(),
            commission_type: CommissionType::DiplomaticMission,
            title: format!("Diplomatic Mission: {} ↔ {}", culture_a.culture, culture_b.culture),
            description: format!("Mediate between {} and {} to reduce tensions and establish peaceful relations",
                culture_a.culture, culture_b.culture),
            client_id: culture_a.id,
            target_id: Some(culture_b.id),
            objectives: vec![
                CommissionObjective {
                    objective_type: ObjectiveType::ImproveRelationship,
                    target: ObjectiveTarget::Relationship((culture_a.culture, culture_b.culture)),
                    quantity: 1,
                    quality_requirement: None,
                    time_constraint: Some(Duration::from_secs(7200)), // 2 hours
                }
            ],
            rewards: CommissionRewards {
                monetary_reward: (conflict_level * 1000.0) as u64,
                resource_rewards: HashMap::new(),
                reputation_gain: conflict_level * 0.2,
                relationship_improvements: HashMap::from([
                    (culture_a.culture, 0.3),
                    (culture_b.culture, 0.3),
                ]),
                special_rewards: vec![
                    SpecialReward::CulturalInsight(culture_a.culture),
                    SpecialReward::CulturalInsight(culture_b.culture),
                ],
                experience_points: (conflict_level * 200.0) as u32,
            },
            requirements: CommissionRequirements {
                minimum_reputation: 0.5,
                required_resources: HashMap::from([
                    (ResourceType::Biomass, 200),
                    (ResourceType::Research, 100),
                ]),
                required_abilities: vec![AbilityType::CulturalTranslation],
                cultural_knowledge: vec![culture_a.culture, culture_b.culture],
            },
            time_limit: Duration::from_secs(7200), // 2 hours
            difficulty: conflict_level,
            urgency: Urgency::High,
            status: CommissionStatus::Available,
        };
        
        Some(commission)
    }
}
```

## Trade Execution System

### Transaction Processing

```rust
#[derive(Debug, Clone)]
pub struct TradeExecutor {
    pub pending_trades: Vec<PendingTrade>,
    pub completed_trades: Vec<TradeRecord>,
    pub failed_trades: Vec<FailedTrade>,
    pub trade_history: HashMap<Culture, Vec<TradeRecord>>,
}

#[derive(Debug, Clone)]
pub struct PendingTrade {
    pub id: Uuid,
    pub trade_route: TradeRoute,
    pub buyer_confirmation: bool,
    pub seller_confirmation: bool,
    pub shepherd_confirmation: bool,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub completion_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: TradeStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeStatus {
    PendingConfirmation,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct TradeRecord {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub price_per_unit: f32,
    pub total_value: f32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trade_duration: Duration,
    pub success: bool,
    pub shepherd_involved: bool,
    pub commission_paid: u64,
    pub quality: ResourceQuality,
}

impl TradeExecutor {
    pub fn execute_trade(&mut self, trade_route: &TradeRoute, context: &mut TradeContext) -> TradeResult {
        // Validate trade
        let validation_result = self.validate_trade(trade_route, context);
        if let Err(error) = validation_result {
            return TradeResult::Failed(error);
        }
        
        // Process payment
        let payment_result = self.process_payment(trade_route, context);
        if let Err(error) = payment_result {
            return TradeResult::Failed(error);
        }
        
        // Transfer resources
        let transfer_result = self.transfer_resources(trade_route, context);
        if let Err(error) = transfer_result {
            // Refund payment on transfer failure
            self.refund_payment(trade_route, context);
            return TradeResult::Failed(error);
        }
        
        // Update relationships
        self.update_trade_relationships(trade_route, context);
        
        // Record trade
        let trade_record = self.create_trade_record(trade_route, true);
        self.completed_trades.push(trade_record.clone());
        
        // Update market
        self.update_market_after_trade(&trade_record, context);
        
        TradeResult::Success(trade_record)
    }
    
    fn validate_trade(&self, trade_route: &TradeRoute, context: &TradeContext) -> Result<(), TradeError> {
        // Check if both parties have required resources
        let buyer = context.nodes.get(&trade_route.buyer_id)
            .ok_or(TradeError::BuyerNotFound)?;
        let seller = context.nodes.get(&trade_route.seller_id)
            .ok_or(TradeError::SellerNotFound)?;
        
        // Check buyer can afford
        let buyer_resources = buyer.inventory.resources.get(&trade_route.resource_type);
        let total_cost = trade_route.total_value;
        
        if buyer_resources.unwrap_or(&0) < &total_cost as u64 {
            return Err(TradeError::InsufficientBuyerResources);
        }
        
        // Check seller has sufficient quantity
        let seller_resources = seller.inventory.resources.get(&trade_route.resource_type);
        if seller_resources.unwrap_or(&0) < &trade_route.quantity {
            return Err(TradeError::InsufficientSellerResources);
        }
        
        // Check trade requirements
        for requirement in &trade_route.requirements {
            if !self.requirement_met(requirement, buyer, seller, context) {
                return Err(TradeError::RequirementNotMet);
            }
        }
        
        Ok(())
    }
    
    fn process_payment(&mut self, trade_route: &TradeRoute, context: &mut TradeContext) -> Result<(), TradeError> {
        let buyer = context.nodes.get_mut(&trade_route.buyer_id)
            .ok_or(TradeError::BuyerNotFound)?;
        
        // Deduct payment from buyer
        let buyer_resources = buyer.inventory.resources
            .get_mut(&ResourceType::Scrap)
            .ok_or(TradeError::ResourceNotFound)?;
        
        if *buyer_resources < trade_route.total_value as u64 {
            return Err(TradeError::InsufficientBuyerResources);
        }
        
        *buyer_resources -= trade_route.total_value as u64;
        
        // Pay seller
        let seller = context.nodes.get_mut(&trade_route.seller_id)
            .ok_or(TradeError::SellerNotFound)?;
        
        let seller_resources = seller.inventory.resources
            .entry(ResourceType::Scrap)
            .or_insert(0);
        
        *seller_resources += trade_route.total_value as u64;
        
        // Pay shepherd commission if applicable
        if let Some(commission) = &trade_route.shepherd_commission {
            let shepherd_share = (trade_route.total_value * commission.commission_rate) as u64;
            *seller_resources -= shepherd_share;
            
            // Add to shepherd's resources
            context.shepherd_resources.entry(ResourceType::Scrap)
                .or_insert(0)
                .add_assign(shepherd_share);
        }
        
        Ok(())
    }
    
    fn transfer_resources(&mut self, trade_route: &TradeRoute, context: &mut TradeContext) -> Result<(), TradeError> {
        let seller = context.nodes.get_mut(&trade_route.seller_id)
            .ok_or(TradeError::SellerNotFound)?;
        let buyer = context.nodes.get_mut(&trade_route.buyer_id)
            .ok_or(TradeError::BuyerNotFound)?;
        
        // Remove from seller
        let seller_resources = seller.inventory.resources
            .get_mut(&trade_route.resource_type)
            .ok_or(TradeError::ResourceNotFound)?;
        
        if *seller_resources < trade_route.quantity {
            return Err(TradeError::InsufficientSellerResources);
        }
        
        *seller_resources -= trade_route.quantity;
        
        // Add to buyer
        let buyer_resources = buyer.inventory.resources
            .entry(trade_route.resource_type)
            .or_insert(0);
        
        *buyer_resources += trade_route.quantity;
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TradeResult {
    Success(TradeRecord),
    Failed(TradeError),
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeError {
    BuyerNotFound,
    SellerNotFound,
    ResourceNotFound,
    InsufficientBuyerResources,
    InsufficientSellerResources,
    RequirementNotMet,
    PaymentFailed,
    TransferFailed,
    RelationshipBlocked,
    EnvironmentalHazard,
}
```

## Implementation Tasks

### Core System Development

1. **Implement Barter Nodes**: Create cultural trading entities
2. **Build Market Dynamics**: Supply/demand and pricing systems
3. **Create Trade Routes**: Dynamic trade opportunity generation
4. **Develop Shepherd Commissions**: Mission generation and tracking
5. **Implement Trade Execution**: Transaction processing and validation

### UI Integration

1. **Trade Network Visualization**: Show active trade routes
2. **Commission Board**: Display available shepherd missions
3. **Market Price Display**: Real-time price information
4. **Cultural Relationship View**: Show trade relationships

### Balance and Testing

1. **Market Stability**: Ensure prices remain balanced
2. **Trade Flow**: Verify trades create meaningful decisions
3. **Commission Difficulty**: Scale challenges appropriately
4. **Cultural Behavior**: Test that cultures trade according to personality

## Validation Criteria

- [ ] All cultures trade according to their personality patterns
- [ ] Market prices respond to supply and demand
- [ ] Trade routes generate logically from cultural needs
- [ ] Shepherd commissions provide meaningful gameplay
- [ ] Trade execution is reliable and secure
- [ ] Economic system remains stable over time

The Barter Web Trade System creates a living, breathing economy where cultural relationships, resource needs, and the Shepherd's intervention combine to create dynamic trading opportunities that reflect the complex interconnections of the planetary web.
