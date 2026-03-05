# Elder Living Interface

> **Status:** INTERACTIVE MENTOR SYSTEM v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-025, PLANETARY_TOPOLOGY_RIPPLE_MAP.md, SHEPHERD_GARDEN_WANDERING.md

## Overview

The Elder Living Interface transforms the massive Void Slime from a passive background element into an active, interactive mentor figure that provides contextual advantages, wisdom, and guidance. The Elder serves as the "Living Interface" that bridges the gap between the Astronaut's chaotic energy and the planet's ancient wisdom, creating a dynamic relationship that evolves throughout the game.

## Interface Architecture

### Elder State Management

```rust
#[derive(Debug, Clone)]
pub struct ElderLivingInterface {
    pub elder: ElderSlime,
    pub current_state: ElderState,
    pub interaction_history: Vec<ElderInteraction>,
    pub blessing_system: BlessingSystem,
    pub wisdom_system: WisdomSystem,
    pub mentorship_system: MentorshipSystem,
    pub visual_system: ElderVisualSystem,
    pub audio_system: ElderAudioSystem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElderState {
    Sleeping,            // Dormant, massive - Tier 1-3
    Stirring,            // Beginning to wake - Tier 4-6
    Awakening,           // One eye opens - Tier 6-7
    Active,              // Fully awake - Tier 8
    Mentoring,           // Providing guidance - Tier 8+
    Blessing,           // Providing bonuses - Active
    Communicating,       // Sharing wisdom - Active
    Ascending,          // Preparing for escape - Endgame
}

#[derive(Debug, Clone)]
pub struct ElderInteraction {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub interaction_type: InteractionType,
    pub player_action: PlayerAction,
    pub elder_response: ElderResponse,
    pub context: InteractionContext,
    pub outcome: InteractionOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractionType {
    DailyBlessing,       // Daily interaction for bonuses
    Offering,            // Offering resources
    Question,            // Seeking wisdom
    Meditation,          // Spiritual connection
    Celebration,         // Celebrating achievements
    Disturbance,        // Causing disturbance
    Emergency,          // Seeking help
    Ascension,          // Endgame interaction
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    OfferScrap { amount: u64 },
    OfferBiomass { amount: u64 },
    OfferEnergy { amount: u64 },
    OfferResearch { amount: u64 },
    OfferSlime { slime_id: Uuid },
    AskQuestion { question: String },
    Meditate { duration: Duration },
    Celebrate { achievement: String },
    Disturb { disturbance_type: DisturbanceType },
    RequestHelp { help_type: HelpType },
}

#[derive(Debug, Clone)]
pub struct ElderResponse {
    pub response_type: ResponseType,
    pub message: String,
    pub blessing: Option<BlessingEffect>,
    pub wisdom: Option<WisdomInsight>,
    pub mentorship: Option<MentorshipGuidance>,
    pub emotional_state: ElderEmotionalState,
    pub visual_effect: ElderVisualEffect,
    pub audio_effect: ElderAudioEffect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseType {
    Blessing,            // Provides blessing
    Wisdom,              // Shares wisdom
    Mentorship,          // Provides guidance
    Warning,             // Gives warning
    Encouragement,       // Offers encouragement
    Disappointment,       // Expresses disappointment
    Concern,             // Shows concern
    Approval,            // Shows approval
    Gratitude,           // Expresses gratitude
    Silence,             // No response
}

impl ElderLivingInterface {
    pub fn new() -> Self {
        let elder = ElderSlime::new();
        
        Self {
            elder,
            current_state: ElderState::Sleeping,
            interaction_history: Vec::new(),
            blessing_system: BlessingSystem::new(),
            wisdom_system: WisdomSystem::new(),
            mentorship_system: MentorshipSystem::new(),
            visual_system: ElderVisualSystem::new(),
            audio_system: ElderAudioSystem::new(),
        }
    }
    
    pub fn handle_interaction(&mut self, player_action: PlayerAction) -> ElderResponse {
        let interaction_type = self.determine_interaction_type(&player_action);
        let context = self.create_interaction_context(&player_action);
        
        // Update Elder state based on interaction
        self.update_elder_state(&player_action);
        
        // Generate response
        let response = self.generate_response(&player_action, &context);
        
        // Record interaction
        let interaction = ElderInteraction {
            timestamp: chrono::Utc::now(),
            interaction_type,
            player_action: player_action.clone(),
            elder_response: response.clone(),
            context,
            outcome: InteractionOutcome::Success,
        };
        
        self.interaction_history.push(interaction);
        
        response
    }
    
    fn determine_interaction_type(&self, player_action: &PlayerAction) -> InteractionType {
        match player_action {
            PlayerAction::OfferScrap { .. } => InteractionType::Offering,
            PlayerAction::OfferBiomass { .. } => InteractionType::Offering,
            PlayerAction::OfferEnergy { .. } => InteractionType::Offering,
            PlayerAction::OfferResearch { .. } => InteractionType::Offering,
            PlayerAction::OfferSlime { .. } => InteractionType::Offering,
            PlayerAction::AskQuestion { .. } => InteractionType::Question,
            PlayerAction::Meditate { .. } => InteractionType::Meditation,
            PlayerAction::Celebrate { .. } => InteractionType::Celebration,
            PlayerAction::Disturb { .. } => InteractionType::Disturbance,
            PlayerAction::RequestHelp { .. } => InteractionType::Emergency,
        }
    }
    
    fn update_elder_state(&mut self, player_action: &PlayerAction) {
        let tier_progress = self.calculate_tier_progress();
        
        match (tier_progress, &self.current_state) {
            (1..=3, ElderState::Sleeping) => {
                // Still sleeping
            },
            (4..=6, ElderState::Sleeping) => {
                // Begin to stir
                self.current_state = ElderState::Stirring;
                self.visual_system.play_stirring_animation();
                self.audio_system.play_stirring_sound();
            },
            (6..=7, ElderState::Stirring) => {
                // One eye opens
                self.current_state = ElderState::Awakening;
                self.visual_system.play_eye_opening_animation();
                self.audio_system.play_eye_opening_sound();
            },
            (8..=9, ElderState::Awakening) => {
                // Fully awake
                self.current_state = ElderState::Active;
                self.visual_system.play_awakening_animation();
                self.audio_system.play_awakening_sound();
            },
            (10.., ElderState::Active) => {
                // Begin mentoring
                self.current_state = ElderState::Mentoring;
                self.visual_system.play_mentoring_animation();
                self.audio_system.play_mentoring_sound();
            },
            _ => {}
        }
        
        // Handle specific actions
        match player_action {
            PlayerAction::Disturb { disturbance_type } => {
                self.handle_disturbance(disturbance_type);
            },
            PlayerAction::OfferSlime { slime_id } => {
                self.handle_slime_offering(*slime_id);
            },
            _ => {}
        }
    }
    
    fn generate_response(&mut self, player_action: &PlayerAction, context: &InteractionContext) -> ElderResponse {
        match player_action {
            PlayerAction::OfferScrap { amount } => {
                self.generate_scrap_response(*amount, context)
            },
            PlayerAction::OfferBiomass { amount } => {
                self.generate_biomass_response(*amount, context)
            },
            PlayerAction::OfferEnergy { amount } => {
                self.generate_energy_response(*amount, context)
            },
            PlayerAction::OfferResearch { amount } => {
                self.generate_research_response(*amount, context)
            },
            PlayerAction::OfferSlime { slime_id } => {
                self.generate_slime_response(*slime_id, context)
            },
            PlayerAction::AskQuestion { question } => {
                self.generate_question_response(question, context)
            },
            PlayerAction::Meditate { duration } => {
                self.generate_meditation_response(*duration, context)
            },
            PlayerAction::Celebrate { achievement } => {
                self.generate_celebration_response(achievement, context)
            },
            PlayerAction::Disturb { disturbance_type } => {
                self.generate_disturbance_response(disturbance_type, context)
            },
            PlayerAction::RequestHelp { help_type } => {
                self.generate_help_response(help_type, context)
            },
        }
    }
}
```

### Blessing System

```rust
#[derive(Debug, Clone)]
pub struct BlessingSystem {
    pub active_blessings: Vec<ActiveBlessing>,
    pub blessing_cooldowns: HashMap<BlessingType, Duration>,
    pub daily_blessing_available: bool,
    pub blessing_history: Vec<BlessingRecord>,
    pub blessing_effects: HashMap<BlessingType, BlessingEffect>,
}

#[derive(Debug, Clone)]
pub struct ActiveBlessing {
    pub id: Uuid,
    pub blessing_type: BlessingType,
    pub target_culture: Option<Culture>,
    pub ring_number: Option<u8>,
    pub strength: f32,
    pub duration: Duration,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub effects: Vec<BlessingEffect>,
    pub visual_effect: BlessingVisualEffect,
    pub audio_effect: BlessingAudioEffect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlessingType {
    CulturalBonus,        // Bonus to specific culture
    RingBlessing,        // Bonus to entire ring
    UniversalBonus,       // Bonus to all activities
    ProtectionBlessing,   // Protection from hazards
    WisdomBlessing,      // Enhanced wisdom
    MentorshipBlessing,   // Enhanced mentorship
    AscensionBlessing,    // Endgame blessing
}

impl BlessingSystem {
    pub fn new() -> Self {
        Self {
            active_blessings: Vec::new(),
            blessing_cooldowns: HashMap::new(),
            daily_blessing_available: true,
            blessing_history: Vec::new(),
            blessing_effects: Self::initialize_blessing_effects(),
        }
    }
    
    pub fn grant_daily_blessing(&mut self, player_action: &PlayerAction) -> Option<ActiveBlessing> {
        if !self.daily_blessing_available {
            return None;
        }
        
        let blessing_type = self.determine_blessing_type(player_action);
        let blessing = self.create_blessing(blessing_type, player_action);
        
        self.active_blessings.push(blessing.clone());
        self.daily_blessing_available = false;
        
        // Set cooldown
        self.blessing_cooldowns.insert(blessing_type, Duration::from_secs(86400)); // 24 hours
        
        Some(blessing)
    }
    
    fn determine_blessing_type(&self, player_action: &PlayerAction) -> BlessingType {
        match player_action {
            PlayerAction::OfferScrap { amount } => {
                if *amount > 1000 {
                    BlessingType::CulturalBonus
                } else {
                    BlessingType::ProtectionBlessing
                }
            },
            PlayerAction::OfferBiomass { amount } => {
                BlessingType::RingBlessing
            },
            PlayerAction::OfferEnergy { amount } => {
                BlessingType::UniversalBonus
            },
            PlayerAction::OfferResearch { amount } => {
                BlessingType::WisdomBlessing
            },
            PlayerAction::OfferSlime { slime_id } => {
                BlessingType::MentorshipBlessing
            },
            _ => BlessingType::ProtectionBlessing,
        }
    }
    
    fn create_blessing(&self, blessing_type: BlessingType, player_action: &PlayerAction) -> ActiveBlessing {
        let blessing_effect = self.blessing_effects.get(&blessing_type).unwrap();
        
        ActiveBlessing {
            id: Uuid::new_v4(),
            blessing_type,
            target_culture: self.determine_target_culture(player_action),
            ring_number: self.determine_target_ring(player_action),
            strength: blessing_effect.base_strength,
            duration: blessing_effect.base_duration,
            start_time: chrono::Utc::now(),
            effects: blessing_effect.effects.clone(),
            visual_effect: blessing_effect.visual_effect.clone(),
            audio_effect: blessing_effect.audio_effect.clone(),
        }
    }
    
    fn determine_target_culture(&self, player_action: &PlayerAction) -> Option<Culture> {
        match player_action {
            PlayerAction::OfferSlime { slime_id } => {
                if let Some(slime) = get_slime_by_id(*slime_id) {
                    Some(slime.culture)
                } else {
                    None
                }
            },
            _ => None,
        }
    }
    
    fn determine_target_ring(&self, player_action: &PlayerAction) -> Option<u8> {
        match player_action {
            PlayerAction::AskQuestion { question } => {
                // Analyze question to determine ring
                if question.contains("primary") {
                    Some(1)
                } else if question.contains("secondary") {
                    Some(2)
                } else if question.contains("tertiary") {
                    Some(3)
                } else {
                    None
                }
            },
            _ => None,
        }
    }
    
    pub fn update_blessings(&mut self, delta_time: Duration) {
        let now = chrono::Utc::now();
        
        // Update active blessings
        self.active_blessings.retain(|blessing| {
            let elapsed = now.signed_duration_since(blessing.start_time);
            elapsed < blessing.duration
        });
        
        // Update cooldowns
        for (blessing_type, cooldown) in self.blessing_cooldowns.iter_mut() {
            *cooldown = cooldown.saturating_sub(delta_time);
        }
        
        // Check if daily blessing is available
        if let Some(cooldown) = self.blessing_cooldowns.get(&BlessingType::CulturalBonus) {
            if *cooldown == Duration::ZERO {
                self.daily_blessing_available = true;
            }
        }
    }
    
    pub fn get_active_blessings_for_culture(&self, culture: Culture) -> Vec<&ActiveBlessing> {
        self.active_blessings.iter()
            .filter(|blessing| {
                blessing.target_culture == Some(culture) || 
                blessing.blessing_type == BlessingType::UniversalBonus
            })
            .collect()
    }
    
    pub fn get_active_blessings_for_ring(&self, ring_number: u8) -> Vec<&ActiveBlessing> {
        self.active_blessings.iter()
            .filter(|blessing| {
                blessing.ring_number == Some(ring_number) || 
                blessing.blessing_type == BlessingType::UniversalBonus
            })
            .collect()
    }
}
```

### Wisdom System

```rust
#[derive(Debug, Clone)]
pub struct WisdomSystem {
    pub wisdom_insights: Vec<WisdomInsight>,
    pub wisdom_categories: HashMap<WisdomCategory, Vec<WisdomInsight>>,
    pub wisdom_progress: f32,
    pub wisdom_level: u8,
    pub mentorship_available: bool,
}

#[derive(Debug, Clone)]
pub struct WisdomInsight {
    pub id: Uuid,
    pub category: WisdomCategory,
    pub title: String,
    pub content: String,
    pub context: WisdomContext,
    pub emotional_tone: EmotionalTone,
    pub applicability: Vec<ApplicableSituation>,
    pub tier_requirement: u8,
    pub unlock_conditions: Vec<UnlockCondition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WisdomCategory {
    Cultural,           // Cultural wisdom
    Environmental,      // Environmental wisdom
    Strategic,          // Strategic wisdom
    Technical,          // Technical wisdom
    Philosophical,      // Philosophical wisdom
    Historical,         // Historical wisdom
    Prophetic,          // Prophetic wisdom
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WisdomContext {
    General,           // General advice
    Crisis,            // Crisis situation
    Discovery,         // Discovery moment
    Decision,          // Decision point
    Transition,        // Period of change
    Achievement,       // Achievement celebration
    Loss,              // Loss or failure
    Growth,            // Growth period
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmotionalTone {
    Encouraging,       // Positive and uplifting
    Warning,           // Cautionary
    Reflective,         // Thoughtful
    Urgent,            // Time-sensitive
    Comforting,         // Reassuring
    Challenging,       // Thought-provoking
    Mystical,           // Mysterious
    Practical,         // Actionable
}

impl WisdomSystem {
    pub fn new() -> Self {
        let mut system = Self {
            wisdom_insights: Vec::new(),
            wisdom_categories: HashMap::new(),
            wisdom_progress: 0.0,
            wisdom_level: 1,
            mentorship_available: false,
        };
        
        system.initialize_wisdom_insights();
        system
    }
    
    fn initialize_wisdom_insights(&mut self) {
        // Cultural wisdom
        let cultural_wisdom = vec![
            WisdomInsight {
                id: Uuid::new_v4(),
                category: WisdomCategory::Cultural,
                title: "The Balance of Fire and Water".to_string(),
                content: "Ember's heat and Tide's flow create the fundamental rhythm of this world. Where they meet, life flourishes. Where they clash, change occurs.".to_string(),
                context: WisdomContext::General,
                emotional_tone: EmotionalTone::Reflective,
                applicability: vec![
                    ApplicableSituation::CulturalConflict,
                    ApplicableSituation::ResourceManagement,
                    ApplicableSituation::StrategicPlanning,
                ],
                tier_requirement: 1,
                unlock_conditions: vec![],
            },
            WisdomInsight {
                id: Uuid::new_v4(),
                category: WisdomCategory::Cultural,
                title: "The Crystal's Memory".to_string(),
                content: "Crystal remembers everything that has ever happened in its vicinity. It holds the planet's history in its facets. Listen carefully, and you will hear the echoes of ancient times.".to_string(),
                context: WisdomContext::Discovery,
                emotional_tone: EmotionalTone::Mystical,
                applicability: vec![
                    ApplicableSituation::Research,
                    ApplicableSituation::Exploration,
                    ApplicableSituation::HistoricalInvestigation,
                ],
                tier_requirement: 2,
                unlock_conditions: vec![UnlockCondition::DiscoverCrystal],
            },
        ];
        
        // Environmental wisdom
        let environmental_wisdom = vec![
            WisdomInsight {
                id: Uuid::new_v4(),
                category: WisdomCategory::Environmental,
                title: "The Breath of the Planet".to_string(),
                content: "The planet breathes in cycles of expansion and contraction. What you call 'weather' is the planet's respiration. Learn to work with its rhythm, not against it.".to_string(),
                context: WisdomContext::General,
                emotional_tone: EmotionalTone::Practical,
                applicability: vec![
                    ApplicableSituation::WeatherPrediction,
                    ApplicableSituation::ResourceGathering,
                    ApplicableSituation::MissionPlanning,
                ],
                tier_requirement: 1,
                unlock_conditions: vec![],
            },
            WisdomInsight {
                id: Uuid::new_v4(),
                category: WisdomCategory::Environmental,
                title: "The Toxic Wastes' Warning".to_string(),
                content: "The wastes are not naturally toxic. They are the planet's wounds, festering from ancient conflicts. Treat them with care, for they hold lessons in what happens when balance is lost.".to_string(),
                context: WisdomContext::Warning,
                emotional_tone: EmotionalTone::Warning,
                applicability: vec![
                    ApplicableSituation::HazardousEnvironment,
                    ApplicableSituation::EnvironmentalProtection,
                    ApplicableSituation::RiskAssessment,
                ],
                tier_requirement: 3,
                unlock_conditions: vec![UnlockCondition::EnterForbiddenZone],
            },
        ];
        
        // Strategic wisdom
        let strategic_wisdom = vec![
            WisdomInsight {
                id: Uuid::new_v4(),
                category: WisdomCategory::Strategic,
                title: "The Shepherd's Dilemma".to_string(),
                content: "You are both caretaker and exploiter. The more you nurture, the more you can harvest. The more you harvest, the more you must nurture. This is the eternal balance of the Shepherd.".to_string(),
                context: WisdomContext::Decision,
                emotional_tone: EmotionalTone::Challenging,
                applicability: vec![
                    ApplicableSituation::ResourceManagement,
                    ApplicableSituation::StrategicPlanning,
                    ApplicableSituation::MoralDecision,
                ],
                tier_requirement: 2,
                unlock_conditions: vec![UnlockCondition::EstablishColony],
            },
            WisdomInsight {
                id: Uuid::new_v4(),
                category: WisdomCategory::Strategic,
                title: "The Void's Promise".to_string(),
                content: "The Void is not emptiness, but potential. It is the canvas upon which all cultures paint their existence. To understand the Void is to understand the nature of creation itself.".to_string(),
                context: WisdomContext::Discovery,
                emotional_tone: EmotionalTone::Mystical,
                applicability: vec![
                    ApplicableSituation::EndgamePlanning,
                    ApplicableSituation::AscensionPreparation,
                    ApplicableSituation::PhilosophicalReflection,
                ],
                tier_requirement: 8,
                unlock_conditions: vec![UnlockCondition::ReachTier8],
            },
        ];
        
        // Add all wisdom insights
        self.wisdom_insights.extend(cultural_wisdom);
        self.wisdom_insights.extend(environmental_wisdom);
        self.wisdom_insights.extend(strategic_wisdom);
        
        // Organize by category
        for insight in &self.wisdom_insights {
            self.wisdom_categories
                .entry(insight.category)
                .or_insert_with(Vec::new)
                .push(insight.clone());
        }
    }
    
    pub fn get_wisdom_for_context(&self, context: &WisdomContext) -> Vec<&WisdomInsight> {
        self.wisdom_insights.iter()
            .filter(|insight| {
                insight.context == *context || insight.context == WisdomContext::General
            })
            .filter(|insight| insight.tier_requirement <= self.wisdom_level)
            .collect()
    }
    
    pub fn get_wisdom_for_situation(&self, situation: &ApplicableSituation) -> Vec<&WisdomInsight> {
        self.wisdom_insights.iter()
            .filter(|insight| {
                insight.applicability.contains(situation)
            })
            .filter(|insight| insight.tier_requirement <= self.wisdom_level)
            .collect()
    }
    
    pub fn unlock_wisdom(&mut self, condition: &UnlockCondition) {
        // Check if any wisdom insights can be unlocked
        for insight in &mut self.wisdom_insights {
            if insight.unlock_conditions.contains(condition) {
                insight.unlock_conditions.retain(|c| c != condition);
            }
        }
        
        // Update wisdom level
        self.update_wisdom_level();
    }
    
    fn update_wisdom_level(&mut self) {
        let unlocked_count = self.wisdom_insights.iter()
            .filter(|insight| insight.unlock_conditions.is_empty())
            .count();
        
        self.wisdom_level = match unlocked_count {
            0..=2 => 1,
            3..=5 => 2,
            6..=8 => 3,
            9..=12 => 4,
            13..=16 => 5,
            17..=20 => 6,
            21..=25 => 7,
            26..=30 => 8,
            _ => 9,
        };
        
        self.mentorship_available = self.wisdom_level >= 4;
    }
}
```

## Implementation Tasks

### Core System Development

1. **Create Elder Interface**: Implement interaction system
2. **Build Blessing System**: Create blessing mechanics
3. **Develop Wisdom System**: Implement wisdom sharing
4. **Create Mentorship System**: Build mentorship mechanics
5. **Implement Visual System**: Create Elder visual effects

### Visual Implementation

1. **Elder Rendering**: Massive, multi-layer visual rendering
2. **State Transitions**: Smooth transitions between Elder states
3. **Blessing Effects**: Visual feedback for blessings
4. **Wisdom Visualization**: Visual representation of wisdom
5. **Interaction Feedback**: Visual feedback for interactions

### Audio Implementation

1. **Elder Audio**: Deep, resonant audio for Elder
2. **State Audio**: Different audio for each Elder state
3. **Blessing Audio**: Audio feedback for blessings
4. **Wisdom Audio**: Audio for wisdom sharing
5. **Interaction Audio**: Audio feedback for interactions

## Validation Criteria

- [ ] Elder provides meaningful interactions throughout the game
- [ Blessing system creates strategic advantages
- [ Wisdom system provides valuable guidance
- [ Mentorship system enhances player experience
- [ Visual system creates impressive Elder presence
- [ Audio system enhances Elder's personality

The Elder Living Interface transforms the massive Void Slime into an active, interactive mentor figure that provides contextual advantages, wisdom, and guidance, creating a dynamic relationship that evolves throughout the game and serves as the central hub for the Astronaut's journey.
