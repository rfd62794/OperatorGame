# Living Economy & Sub-Factions

> **Status:** ECONOMIC SYSTEM SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-026, PERSONALITY_CORES_SYSTEM.md, BARTER_WEB_TRADE_SYSTEM.md

## Overview

The Living Economy transforms static resource management into a dynamic Barter Web where cultures trade based on their needs, personalities, and relationships. Sub-Factions emerge naturally from cultural overlap, creating hybrid entities that enable advanced breeding and unique economic opportunities while the Shepherd serves as the essential middleman.

## Sub-Faction Generation

### Hybrid Node Creation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubFaction {
    pub id: Uuid,
    pub name: String,
    pub parent_cultures: (Culture, Culture),
    pub hybrid_culture: HybridCulture,
    pub generation: u8,
    pub stability: f32,              // 0.0 to 1.0
    pub influence_radius: f32,
    pub unique_traits: Vec<HybridTrait>,
    pub trade_specialization: TradeSpecialization,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HybridCulture {
    pub primary_influence: Culture,
    pub secondary_influence: Culture,
    pub mixed_traits: Vec<MixedTrait>,
    pub cultural_balance: f32,      // 0.0 = pure primary, 1.0 = pure secondary
    pub mutation_level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixedTrait {
    InheritedFromPrimary(CultureTrait),
    InheritedFromSecondary(CultureTrait),
    Emergent(EmergentTrait),
    Conflicted(ConflictedTrait),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CultureTrait {
    Dominant,       // From Ember
    Fluid,          // From Tide
    Erratic,        // From Gale
    Analytical,     // From Orange
    Nurturing,      // From Marsh
    Judgmental,     // From Crystal
    Industrious,    // From Amber
    Serene,         // From Teal
    Isolating,      // From Frost
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmergentTrait {
    Adaptive,       // Can change behavior
    Symbiotic,      // Benefits from cooperation
    Parasitic,      // Exploits hosts
    Neutral,        // Balances influences
    Chaotic,        // Unpredictable mixing
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictedTrait {
    Bipolar,        // Switches between parent behaviors
    Schizophrenic,  // Multiple conflicting personalities
    Unstable,       // Randomly changes traits
    Dysfunctional,  // Poorly functioning hybrid
}

impl SubFaction {
    pub fn generate_from_overlap(
        primary_node: &MapNode, 
        secondary_node: &MapNode,
        overlap_duration: Duration
    ) -> Option<Self> {
        // Check if overlap is sufficient for sub-faction formation
        let min_overlap_duration = Duration::from_secs(3600); // 1 hour
        if overlap_duration < min_overlap_duration {
            return None;
        }
        
        // Calculate compatibility
        let compatibility = Self::calculate_cultural_compatibility(
            primary_node.culture, 
            secondary_node.culture
        );
        
        if compatibility < 0.3 {
            return None; // Too incompatible for stable sub-faction
        }
        
        // Generate hybrid culture
        let hybrid_culture = HybridCulture {
            primary_influence: primary_node.culture,
            secondary_influence: secondary_node.culture,
            mixed_traits: Self::generate_mixed_traits(primary_node.culture, secondary_node.culture),
            cultural_balance: rand::random::<f32>(), // Random balance
            mutation_level: 1,
        };
        
        // Generate unique traits
        let unique_traits = Self::generate_unique_traits(&hybrid_culture);
        
        // Determine trade specialization
        let trade_specialization = Self::determine_trade_specialization(&hybrid_culture);
        
        Some(SubFaction {
            id: Uuid::new_v4(),
            name: Self::generate_subfaction_name(primary_node.culture, secondary_node.culture),
            parent_cultures: (primary_node.culture, secondary_node.culture),
            hybrid_culture,
            generation: 1,
            stability: compatibility,
            influence_radius: 0.5, // Smaller than parent nodes
            unique_traits,
            trade_specialization,
        })
    }
    
    fn calculate_cultural_compatibility(primary: Culture, secondary: Culture) -> f32 {
        // Define compatibility matrix
        let compatibility_matrix = HashMap::from([
            // High compatibility pairs
            ((Culture::Tide, Culture::Marsh), 0.9),
            ((Culture::Ember, Culture::Crystal), 0.8),
            ((Culture::Orange, Culture::Amber), 0.8),
            ((Culture::Gale, Culture::Teal), 0.7),
            
            // Medium compatibility pairs
            ((Culture::Tide, Culture::Orange), 0.6),
            ((Culture::Marsh, Culture::Teal), 0.6),
            ((Culture::Ember, Culture::Amber), 0.5),
            ((Culture::Crystal, Culture::Frost), 0.5),
            
            // Low compatibility pairs
            ((Culture::Ember, Culture::Marsh), 0.3),
            ((Culture::Crystal, Culture::Gale), 0.2),
            ((Culture::Frost, Culture::Tide), 0.2),
        ]);
        
        *compatibility_matrix.get(&(primary, secondary))
            .or_else(|| compatibility_matrix.get(&(secondary, primary)))
            .unwrap_or(0.4) // Default compatibility
    }
    
    fn generate_mixed_traits(primary: Culture, secondary: Culture) -> Vec<MixedTrait> {
        let mut traits = Vec::new();
        
        // Inherit traits from parents
        traits.push(MixedTrait::InheritedFromPrimary(primary.get_core_trait()));
        traits.push(MixedTrait::InheritedFromSecondary(secondary.get_core_trait()));
        
        // Generate emergent traits based on combination
        let emergent_chance = rand::random::<f32>();
        if emergent_chance > 0.5 {
            traits.push(MixedTrait::Emergent(Self::generate_emergent_trait(primary, secondary)));
        }
        
        // Check for conflicts
        if Self::traits_conflict(primary.get_core_trait(), secondary.get_core_trait()) {
            traits.push(MixedTrait::Conflicted(Self::generate_conflicted_trait()));
        }
        
        traits
    }
    
    fn generate_emergent_trait(primary: Culture, secondary: Culture) -> EmergentTrait {
        match (primary.personality_core(), secondary.personality_core()) {
            (PersonalityCore::Fluid, PersonalityCore::Nurturing) => EmergentTrait::Symbiotic,
            (PersonalityCore::Dominant, PersonalityCore::Industrious) => EmergentTrait::Parasitic,
            (PersonalityCore::Erratic, PersonalityCore::Serene) => EmergentTrait::Chaotic,
            _ => EmergentTrait::Adaptive,
        }
    }
    
    fn generate_conflicted_trait() -> ConflictedTrait {
        match rand::random::<u8>() % 4 {
            0 => ConflictedTrait::Bipolar,
            1 => ConflictedTrait::Schizophrenic,
            2 => ConflictedTrait::Unstable,
            _ => ConflictedTrait::Dysfunctional,
        }
    }
}
```

### Sub-Faction Evolution

```rust
impl SubFaction {
    pub fn evolve(&mut self, events: &[SubFactionEvent]) {
        for event in events {
            match event {
                SubFactionEvent::SuccessfulTrade { partner, profit } => {
                    self.stability += profit * 0.1;
                    self.influence_radius += 0.05;
                },
                SubFactionEvent::CulturalConflict { severity } => {
                    self.stability -= severity * 0.2;
                    if self.stability < 0.3 {
                        self.mutate();
                    }
                },
                SubFactionEvent::EnvironmentalStress => {
                    self.stability -= 0.1;
                },
                SubFactionEvent::ShepherdIntervention { effect } => {
                    self.apply_shepherd_effect(effect);
                },
            }
        }
        
        // Clamp values
        self.stability = self.stability.clamp(0.0, 1.0);
        self.influence_radius = self.influence_radius.clamp(0.1, 1.5);
    }
    
    fn mutate(&mut self) {
        self.mutation_level += 1;
        
        // Randomly modify traits
        if rand::random::<f32>() > 0.5 {
            let trait_index = rand::random::<usize>() % self.hybrid_culture.mixed_traits.len();
            self.hybrid_culture.mixed_traits[trait_index] = MixedTrait::Emergent(
                EmergentTrait::Chaotic
            );
        }
        
        // Potentially change cultural balance
        if rand::random::<f32>() > 0.7 {
            self.hybrid_culture.cultural_balance = rand::random::<f32>();
        }
        
        // Update name to reflect mutation
        self.name = format!("Mutated {}", self.name);
    }
    
    pub fn can_breed_cross_culture(&self) -> bool {
        self.stability > 0.5 && 
        self.mutation_level < 3 &&
        self.unique_traits.contains(&HybridTrait::CrossBreedingEnabled)
    }
    
    pub fn get_breeding_bonuses(&self) -> Vec<BreedingBonus> {
        let mut bonuses = vec![BreedingBonus::CrossCultureSuccess];
        
        if self.unique_traits.contains(&HybridTrait::EnhancedMutation) {
            bonuses.push(BreedingBonus::IncreasedMutation);
        }
        
        if self.unique_traits.contains(&HybridTrait::StableGenetics) {
            bonuses.push(BreedingBonus::ReducedFailure);
        }
        
        bonuses
    }
}
```

## Barter Web System

### Dynamic Trade Network

```rust
#[derive(Debug, Clone)]
pub struct BarterWeb {
    pub nodes: HashMap<Uuid, BarterNode>,
    pub trade_routes: Vec<TradeRoute>,
    pub market_prices: HashMap<ResourceType, f32>,
    pub supply_demand: HashMap<ResourceType, SupplyDemand>,
    pub shepherd_commissions: Vec<TradeCommission>,
}

#[derive(Debug, Clone)]
pub struct BarterNode {
    pub id: Uuid,
    pub culture: Culture,
    pub is_sub_faction: bool,
    pub inventory: HashMap<ResourceType, u64>,
    pub needs: Vec<ResourceNeed>,
    pub offers: Vec<ResourceOffer>,
    pub trade_preferences: TradePreferences,
    pub reputation: HashMap<Culture, f32>,
}

#[derive(Debug, Clone)]
pub struct ResourceNeed {
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub urgency: Urgency,
    pub preferred_partners: Vec<Culture>,
    pub max_price: f32,
}

#[derive(Debug, Clone)]
pub struct ResourceOffer {
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub min_price: f32,
    pub preferred_partners: Vec<Culture>,
    pub quality: ResourceQuality,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Urgency {
    Critical,    // Need immediately
    High,        // Need soon
    Medium,      // Would like
    Low,         // Optional
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceQuality {
    Poor,        // Low quality, low price
    Standard,    // Normal quality
    Premium,     // High quality, high price
    Exotic,      // Very rare, very high price
}

impl BarterWeb {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            trade_routes: Vec::new(),
            market_prices: Self::initialize_market_prices(),
            supply_demand: HashMap::new(),
            shepherd_commissions: Vec::new(),
        }
    }
    
    pub fn update_trade_network(&mut self, context: &PlanetaryContext) {
        // Update node needs and offers
        self.update_node_needs(context);
        self.update_node_offers(context);
        
        // Calculate supply and demand
        self.calculate_supply_demand();
        
        // Update market prices
        self.update_market_prices();
        
        // Generate trade opportunities
        self.generate_trade_routes();
        
        // Create shepherd commissions
        self.generate_shepherd_commissions();
    }
    
    fn update_node_needs(&mut self, context: &PlanetaryContext) {
        for node in self.nodes.values_mut {
            node.needs.clear();
            
            // Generate needs based on culture personality
            let base_needs = node.culture.get_base_needs();
            
            for need in base_needs {
                let current_stock = node.inventory.get(&need.resource_type).unwrap_or(&0);
                let need_quantity = if *current_stock < need.min_quantity {
                    need.min_quantity - current_stock
                } else {
                    0
                };
                
                if need_quantity > 0 {
                    node.needs.push(ResourceNeed {
                        resource_type: need.resource_type,
                        quantity: need_quantity,
                        urgency: need.urgency,
                        preferred_partners: need.preferred_partners.clone(),
                        max_price: need.max_price,
                    });
                }
            }
        }
    }
    
    fn generate_trade_routes(&mut self) {
        self.trade_routes.clear();
        
        // Find matching needs and offers
        for (buyer_id, buyer) in &self.nodes {
            for need in &buyer.needs {
                for (seller_id, seller) in &self.nodes {
                    if buyer_id == seller_id {
                        continue;
                    }
                    
                    // Check if seller has matching offer
                    if let Some(offer) = seller.find_matching_offer(need) {
                        // Check if trade is compatible
                        if self.is_trade_compatible(buyer, seller, need, offer) {
                            let route = TradeRoute {
                                id: Uuid::new_v4(),
                                buyer_id: *buyer_id,
                                seller_id: *seller_id,
                                resource: need.resource_type,
                                quantity: need.quantity.min(offer.quantity),
                                price: self.calculate_trade_price(need, offer),
                                duration: self.calculate_trade_duration(buyer, seller),
                                risk_level: self.calculate_trade_risk(buyer, seller),
                            };
                            
                            self.trade_routes.push(route);
                        }
                    }
                }
            }
        }
    }
    
    fn generate_shepherd_commissions(&mut self) {
        self.shepherd_commissions.clear();
        
        // Find trade opportunities that require shepherd intervention
        for route in &self.trade_routes {
            let buyer = self.nodes.get(&route.buyer_id).unwrap();
            let seller = self.nodes.get(&route.seller_id).unwrap();
            
            // Check if direct trade is blocked
            if !self.can_trade_directly(buyer, seller) {
                let commission = TradeCommission {
                    id: Uuid::new_v4(),
                    commission_type: CommissionType::TradeFacilitation,
                    buyer_id: route.buyer_id,
                    seller_id: route.seller_id,
                    resource: route.resource,
                    quantity: route.quantity,
                    shepherd_fee: route.price * 0.1, // 10% commission
                    difficulty: self.calculate_commission_difficulty(buyer, seller),
                    time_limit: Duration::from_secs(3600), // 1 hour
                    description: format!("Facilitate trade between {} and {}", 
                        buyer.culture, seller.culture),
                };
                
                self.shepherd_commissions.push(commission);
            }
        }
    }
    
    fn can_trade_directly(&self, buyer: &BarterNode, seller: &BarterNode) -> bool {
        // Check cultural compatibility
        let buyer_openness = buyer.culture.base_openness();
        let seller_openness = seller.culture.base_openness();
        
        // Check relationship
        let relationship = buyer.reputation.get(&seller.culture).unwrap_or(&0.5);
        
        // Both cultures need to be reasonably open and have good relationship
        buyer_openness > 0.4 && seller_openness > 0.4 && relationship > &0.3
    }
}
```

### Shepherd Commission System

```rust
#[derive(Debug, Clone)]
pub struct TradeCommission {
    pub id: Uuid,
    pub commission_type: CommissionType,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub resource: ResourceType,
    pub quantity: u64,
    pub shepherd_fee: u64,
    pub difficulty: f32,
    pub time_limit: Duration,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommissionType {
    TradeFacilitation,    // Help two cultures trade
    ResourceDelivery,     // Deliver resources
    DiplomaticMission,    // Improve relationships
    CulturalExchange,     // Enable cultural exchange
    SubFactionMediation,  // Help sub-factions
}

impl TradeCommission {
    pub fn execute(&self, shepherd_state: &mut ShepherdState) -> CommissionResult {
        match self.commission_type {
            CommissionType::TradeFacilitation => {
                self.execute_trade_facilitation(shepherd_state)
            },
            CommissionType::ResourceDelivery => {
                self.execute_resource_delivery(shepherd_state)
            },
            CommissionType::DiplomaticMission => {
                self.execute_diplomatic_mission(shepherd_state)
            },
            CommissionType::CulturalExchange => {
                self.execute_cultural_exchange(shepherd_state)
            },
            CommissionType::SubFactionMediation => {
                self.execute_sub_faction_mediation(shepherd_state)
            },
        }
    }
    
    fn execute_trade_facilitation(&self, shepherd_state: &mut ShepherdState) -> CommissionResult {
        // Check if shepherd has required resources
        let required_resources = self.calculate_required_resources();
        
        if !shepherd_state.can_afford(&required_resources) {
            return CommissionResult::Failed("Insufficient resources".to_string());
        }
        
        // Execute trade
        shepherd_state.deduct_resources(&required_resources);
        
        // Improve relationships
        shepherd_state.improve_relationship(self.buyer_id, 0.1);
        shepherd_state.improve_relationship(self.seller_id, 0.1);
        
        // Earn commission
        shepherd_state.add_resources(ResourceType::Scrap, self.shepherd_fee);
        
        CommissionResult::Success {
            rewards: vec![ResourceType::Scrap],
            reward_amounts: vec![self.shepherd_fee],
            reputation_gain: 0.05,
            experience_gained: self.difficulty * 10.0,
        }
    }
    
    fn calculate_required_resources(&self) -> ResourceCost {
        // Base cost depends on difficulty and quantity
        let base_cost = (self.quantity as f32 * self.difficulty * 0.1) as u64;
        
        ResourceCost {
            biomass: base_cost / 2,
            scrap: base_cost,
            energy: base_cost / 3,
            research_points: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommissionResult {
    Success {
        rewards: Vec<ResourceType>,
        reward_amounts: Vec<u64>,
        reputation_gain: f32,
        experience_gained: f32,
    },
    Failed(String),
    Expired,
}
```

## Multi-Culture Slime System

### Utility Tags and Cross-Breeding

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCultureSlime {
    pub base: SlimeGenome,
    pub utility_tags: Vec<UtilityTag>,
    pub cross_culture_abilities: Vec<CrossCultureAbility>,
    pub hybrid_traits: Vec<HybridTrait>,
    pub cultural_affinity: HashMap<Culture, f32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UtilityTag {
    HeavyLift,         // Can carry heavy objects
    ConductPower,      // Can conduct electricity
    PurifyWater,       // Can purify water
    NavigateTerrain,    // Can navigate difficult terrain
    ProcessScrap,      // Can process scrap metal
    HealOthers,        // Can heal other slimes
    DetectResources,   // Can detect resources
    Communicate,        // Can communicate with cultures
    BuildStructures,    // Can construct buildings
    GenerateEnergy,    // Can generate energy
}

#[derive(Debug, Clone)]
pub struct CrossCultureAbility {
    pub ability_type: AbilityType,
    pub source_culture: Culture,
    pub proficiency: f32,
    pub cooldown: Duration,
    pub energy_cost: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbilityType {
    CulturalTranslation,  // Can translate between cultures
    DiplomaticImmunity,  // Immune to cultural hostility
    ResourceConversion,   // Can convert one resource to another
    EnvironmentalAdaptation, // Adapt to any environment
    TradeNegotiation,    // Better trade deals
    ConflictResolution,   // Can resolve conflicts
}

impl MultiCultureSlime {
    pub fn generate_from_sub_faction(sub_faction: &SubFaction, base_slime: SlimeGenome) -> Self {
        let utility_tags = Self::generate_utility_tags(&sub_faction.hybrid_culture);
        let cross_culture_abilities = Self::generate_cross_culture_abilities(&sub_faction.hybrid_culture);
        let cultural_affinity = Self::calculate_cultural_affinity(&sub_faction.hybrid_culture);
        
        Self {
            base: base_slime,
            utility_tags,
            cross_culture_abilities,
            hybrid_traits: sub_faction.hybrid_culture.mixed_traits.clone(),
            cultural_affinity,
        }
    }
    
    fn generate_utility_tags(hybrid_culture: &HybridCulture) -> Vec<UtilityTag> {
        let mut tags = Vec::new();
        
        // Tags from primary culture
        tags.extend(hybrid_culture.primary_influence.get_utility_tags());
        
        // Tags from secondary culture
        tags.extend(hybrid_culture.secondary_influence.get_utility_tags());
        
        // Hybrid-specific tags
        if hybrid_culture.mixed_traits.contains(&MixedTrait::Emergent(EmergentTrait::Adaptive)) {
            tags.push(UtilityTag::EnvironmentalAdaptation);
        }
        
        if hybrid_culture.mixed_traits.contains(&MixedTrait::Emergent(EmergentTrait::Symbiotic)) {
            tags.push(UtilityTag::HealOthers);
        }
        
        // Remove duplicates and limit to 3 tags
        tags.sort();
        tags.dedup();
        tags.truncate(3);
        
        tags
    }
    
    fn generate_cross_culture_abilities(hybrid_culture: &HybridCulture) -> Vec<CrossCultureAbility> {
        let mut abilities = Vec::new();
        
        // Cultural translation ability
        abilities.push(CrossCultureAbility {
            ability_type: AbilityType::CulturalTranslation,
            source_culture: hybrid_culture.primary_influence,
            proficiency: 0.8,
            cooldown: Duration::from_secs(60),
            energy_cost: 10,
        });
        
        // Trade negotiation (if compatible cultures)
        if Self::can_trade_negotiate(hybrid_culture) {
            abilities.push(CrossCultureAbility {
                ability_type: AbilityType::TradeNegotiation,
                source_culture: hybrid_culture.secondary_influence,
                proficiency: 0.6,
                cooldown: Duration::from_secs(120),
                energy_cost: 20,
            });
        }
        
        abilities
    }
    
    pub fn can_utilize_ability(&self, ability_type: AbilityType, context: &MissionContext) -> bool {
        if let Some(ability) = self.cross_culture_abilities.iter()
            .find(|a| a.ability_type == ability_type) {
            // Check cooldown
            if context.current_time < ability.last_used + ability.cooldown {
                return false;
            }
            
            // Check energy cost
            if context.current_energy < ability.energy_cost {
                return false;
            }
            
            // Check cultural context
            match ability_type {
                AbilityType::CulturalTranslation => {
                    context.involved_cultures.len() > 1
                },
                AbilityType::TradeNegotiation => {
                    context.mission_type == MissionType::Trade
                },
                AbilityType::ConflictResolution => {
                    context.mission_type == MissionType::Diplomatic
                },
                _ => true,
            }
        } else {
            false
        }
    }
}
```

## Implementation Tasks

### Core System Development

1. **Update Node Structure**: Add openness and personality_core fields
2. **Implement Sub-Faction Generation**: Create hybrid node logic
3. **Build Barter Web**: Dynamic trade network system
4. **Create Commission System**: Shepherd intervention mechanics
5. **Develop Multi-Culture Slimes**: Utility tags and cross-breeding

### UI Integration

1. **Sub-Faction Display**: Visual indicators for hybrid nodes
2. **Trade Route Visualization**: Show active trade opportunities
3. **Commission Interface**: Shepherd mission board
4. **Multi-Culture Slime Cards**: Display utility tags and abilities

### Balance and Testing

1. **Sub-Faction Stability**: Ensure hybrid nodes are balanced
2. **Trade Flow Validation**: Verify barter web creates meaningful decisions
3. **Commission Difficulty**: Scale challenges appropriately
4. **Multi-Culture Slime Power**: Balance utility tags and abilities

## Validation Criteria

- [ ] Sub-factions generate logically from compatible culture overlaps
- [ ] Barter web creates dynamic and interesting trade opportunities
- [ ] Shepherd commissions provide meaningful gameplay choices
- [ ] Multi-culture slimes have balanced utility tags and abilities
- [ ] Economic system remains stable and predictable
- [ ] Cultural relationships evolve based on player actions

The Living Economy & Sub-Factions system creates a dynamic, interconnected world where the Shepherd serves as the essential middleman, facilitating trade, resolving conflicts, and enabling the emergence of hybrid cultures that unlock advanced breeding opportunities and unique economic possibilities.
