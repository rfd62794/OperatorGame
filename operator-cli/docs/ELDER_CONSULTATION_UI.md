# Elder Consultation UI

> **Status**: STRATEGIC ADVISOR INTERFACE v1.0 | **Date**: 2026-03-04  
> **Related**: ADR-026, PERSONALITY_CORES_SYSTEM.md, BARTER_WEB_TRADE_SYSTEM.md

## Overview

The Elder Consultation UI transforms the Elder from a simple NPC into the Shepherd's Strategic HUD - a living, breathing advisor who translates the "Math of the Stars" (ship systems) into the "Frequency of the Soil" (planetary cultures). This interface provides real-time market analysis, cultural intelligence, and diplomatic recommendations that turn every "Mafia Wars" click into an informed strategic decision.

## UI Architecture

### Main Consultation Window

```rust
#[derive(Debug, Clone)]
pub struct ElderConsultationUI {
    pub active_tab: ConsultationTab,
    pub market_analysis: MarketAnalysisWidget,
    pub cultural_intelligence: CulturalIntelligenceWidget,
    pub diplomatic_advisor: DiplomaticAdvisorWidget,
    pub trade_opportunities: TradeOpportunitiesWidget,
    pub personal_recommendations: PersonalRecommendationsWidget,
    pub conversation_history: Vec<ElderMessage>,
    pub current_mood: ElderMood,
    pub wisdom_level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsultationTab {
    MarketReport,        // Economic analysis
    CulturalAnalysis,    // Cultural behavior
    DiplomaticAdvice,    // Relationship guidance
    TradeOpportunities,  // Trade recommendations
    PersonalGuidance,    // Shepherd-specific advice
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElderMood {
    Wise,               // Clear, insightful advice
    Weary,              // Tired but knowledgeable
    Concerned,          // Worried about planetary state
    Pleased,            // Happy with Shepherd's progress
    Urgent,             // Time-sensitive information
}

impl ElderConsultationUI {
    pub fn new() -> Self {
        Self {
            active_tab: ConsultationTab::MarketReport,
            market_analysis: MarketAnalysisWidget::new(),
            cultural_intelligence: CulturalIntelligenceWidget::new(),
            diplomatic_advisor: DiplomaticAdvisorWidget::new(),
            trade_opportunities: TradeOpportunitiesWidget::new(),
            personal_recommendations: PersonalRecommendationsWidget::new(),
            conversation_history: Vec::new(),
            current_mood: ElderMood::Wise,
            wisdom_level: 1,
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, context: &ElderContext) {
        // Elder avatar and mood indicator
        self.render_elder_header(ui, context);
        
        // Tab navigation
        self.render_tab_navigation(ui);
        
        ui.separator();
        
        // Content area
        match self.active_tab {
            ConsultationTab::MarketReport => self.market_analysis.render(ui, context),
            ConsultationTab::CulturalAnalysis => self.cultural_intelligence.render(ui, context),
            ConsultationTab::DiplomaticAdvice => self.diplomatic_advisor.render(ui, context),
            ConsultationTab::TradeOpportunities => self.trade_opportunities.render(ui, context),
            ConsultationTab::PersonalGuidance => self.personal_recommendations.render(ui, context),
        }
        
        // Conversation input
        self.render_conversation_input(ui, context);
    }
    
    fn render_elder_header(&self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.horizontal(|ui| {
            // Elder avatar
            let avatar_rect = ui.allocate_space([60.0, 60.0].into()).1;
            self.render_elder_avatar(ui, avatar_rect);
            
            // Elder status
            ui.vertical(|ui| {
                ui.heading("Elder Consultation");
                
                // Mood indicator
                let mood_text = match self.current_mood {
                    ElderMood::Wise => "🧘 Wise",
                    ElderMood::Weary => "😔 Weary",
                    ElderMood::Concerned => "😟 Concerned",
                    ElderMood::Pleased => "😊 Pleased",
                    ElderMood::Urgent => "⚠️ Urgent",
                };
                ui.label(mood_text);
                
                // Wisdom level
                ui.label(format!("Wisdom Level: {}", self.wisdom_level));
                
                // Last update time
                ui.label(format!("Last Analysis: {:?}", context.last_analysis));
            });
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Quick advice button
                if ui.button("Quick Advice").clicked() {
                    self.generate_quick_advice(context);
                }
                
                // Urgent alerts
                if self.has_urgent_alerts(context) {
                    if ui.button("🚨 Urgent").clicked() {
                        self.active_tab = ConsultationTab::DiplomaticAdvice;
                    }
                }
            });
        });
    }
    
    fn render_elder_avatar(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        // Render elder avatar based on mood
        let avatar_color = match self.current_mood {
            ElderMood::Wise => egui::Color32::from_rgb(150, 150, 200),
            ElderMood::Weary => egui::Color32::from_rgb(100, 100, 100),
            ElderMood::Concerned => egui::Color32::from_rgb(200, 150, 100),
            ElderMood::Pleased => egui::Color32::from_rgb(100, 200, 100),
            ElderMood::Urgent => egui::Color32::from_rgb(200, 100, 100),
        };
        
        ui.painter().circle_filled(rect.center(), rect.width() / 2.0, avatar_color);
        
        // Add elder symbol
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "👴",
            egui::FontId::default(),
            egui::Color32::WHITE,
        );
    }
    
    fn render_tab_navigation(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let tabs = [
                (ConsultationTab::MarketReport, "📊 Market"),
                (ConsultationTab::CulturalAnalysis, "🌍 Cultures"),
                (ConsultationTab::DiplomaticAdvice, "🤝 Diplomacy"),
                (ConsultationTab::TradeOpportunities, "💰 Trade"),
                (ConsultationTab::PersonalGuidance, "👤 Guidance"),
            ];
            
            for (tab, text) in tabs {
                let button = if self.active_tab == tab {
                    egui::Button::new(text).fill(egui::Color32::from_rgb(100, 150, 200))
                } else {
                    egui::Button::new(text)
                };
                
                if ui.add(button).clicked() {
                    self.active_tab = tab;
                }
            }
        });
    }
}
```

### Market Analysis Widget

```rust
#[derive(Debug, Clone)]
pub struct MarketAnalysisWidget {
    pub market_overview: MarketOverview,
    pub price_trends: HashMap<ResourceType, PriceTrend>,
    pub supply_demand_analysis: SupplyDemandAnalysis,
    pub trade_volume_metrics: TradeVolumeMetrics,
    pub market_predictions: MarketPredictions,
}

#[derive(Debug, Clone)]
pub struct MarketOverview {
    pub overall_sentiment: MarketSentiment,
    pub volatility_index: f32,
    pub trade_activity: f32,
    pub resource_scarcity: HashMap<ResourceType, f32>,
    pub market_health: f32,
}

#[derive(Debug, Clone)]
pub struct PriceTrend {
    pub current_price: f32,
    pub trend_direction: TrendDirection,
    pub trend_strength: f32,
    pub predicted_price: f32,
    pub confidence: f32,
    pub key_factors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrendDirection {
    Rising,
    Falling,
    Stable,
    Volatile,
}

impl MarketAnalysisWidget {
    pub fn new() -> Self {
        Self {
            market_overview: MarketOverview::default(),
            price_trends: HashMap::new(),
            supply_demand_analysis: SupplyDemandAnalysis::default(),
            trade_volume_metrics: TradeVolumeMetrics::default(),
            market_predictions: MarketPredictions::default(),
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Market Analysis");
        
        // Market overview
        self.render_market_overview(ui, context);
        
        ui.separator();
        
        // Price trends
        self.render_price_trends(ui, context);
        
        ui.separator();
        
        // Supply and demand
        self.render_supply_demand(ui, context);
        
        ui.separator();
        
        // Market predictions
        self.render_market_predictions(ui, context);
    }
    
    fn render_market_overview(&self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Market Sentiment:");
                let sentiment_text = match self.market_overview.overall_sentiment {
                    MarketSentiment::Bullish => "📈 Bullish",
                    MarketSentiment::Bearish => "📉 Bearish",
                    MarketSentiment::Stable => "➡️ Stable",
                    MarketSentiment::Volatile => "📊 Volatile",
                    MarketSentiment::Uncertain => "❓ Uncertain",
                };
                ui.label(sentiment_text);
            });
            
            ui.vertical(|ui| {
                ui.label("Volatility:");
                ui.label(format!("{:.1}%", self.market_overview.volatility_index * 100.0));
            });
            
            ui.vertical(|ui| {
                ui.label("Trade Activity:");
                ui.label(format!("{:.1}%", self.market_overview.trade_activity * 100.0));
            });
            
            ui.vertical(|ui| {
                ui.label("Market Health:");
                let health_color = if self.market_overview.market_health > 0.7 {
                    egui::Color32::GREEN
                } else if self.market_overview.market_health > 0.4 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::RED
                };
                ui.colored_color(health_color, format!("{:.1}%", self.market_overview.market_health * 100.0));
            });
        });
    }
    
    fn render_price_trends(&mut self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Price Trends");
        
        let resources = [ResourceType::Biomass, ResourceType::Scrap, ResourceType::Energy, ResourceType::Research];
        
        for resource in resources {
            if let Some(trend) = self.price_trends.get(&resource) {
                ui.horizontal(|ui| {
                    // Resource icon and name
                    let icon = match resource {
                        ResourceType::Biomass => "🌱",
                        ResourceType::Scrap => "⚙️",
                        ResourceType::Energy => "⚡",
                        ResourceType::Research => "🔬",
                    };
                    ui.label(format!("{} {}:", icon, format!("{:?}", resource)));
                    
                    // Current price
                    ui.label(format!("Current: {:.2}", trend.current_price));
                    
                    // Trend direction
                    let trend_icon = match trend.trend_direction {
                        TrendDirection::Rising => "📈",
                        TrendDirection::Falling => "📉",
                        TrendDirection::Stable => "➡️",
                        TrendDirection::Volatile => "📊",
                    };
                    ui.label(trend_icon);
                    
                    // Predicted price
                    ui.label(format!("Predicted: {:.2}", trend.predicted_price));
                    
                    // Confidence
                    ui.label(format!("Conf: {:.0}%", trend.confidence * 100.0));
                });
                
                // Key factors
                if !trend.key_factors.is_empty() {
                    ui.indent(|ui| {
                        ui.label("Key factors:");
                        for factor in &trend.key_factors {
                            ui.label(format!("• {}", factor));
                        }
                    });
                }
            }
        }
    }
    
    fn render_market_predictions(&self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Market Predictions");
        
        ui.horizontal(|ui| {
            ui.label("Time Horizon:");
            ui.label("Next Hour");
            ui.label("Next Day");
            ui.label("Next Week");
        });
        
        // Prediction confidence
        ui.horizontal(|ui| {
            ui.label("Prediction Confidence:");
            ui.label(format!("{:.1}%", self.market_predictions.short_term_confidence * 100.0));
            ui.label(format!("{:.1}%", self.market_predictions.medium_term_confidence * 100.0));
            ui.label(format!("{:.1}%", self.market_predictions.long_term_confidence * 100.0));
        });
        
        // Key predictions
        ui.heading("Key Predictions:");
        for prediction in &self.market_predictions.key_predictions {
            ui.horizontal(|ui| {
                ui.label(format!("• {}", prediction.description));
                
                let urgency_color = match prediction.urgency {
                    Urgency::Low => egui::Color32::GREEN,
                    Urgency::Medium => egui::Color32::YELLOW,
                    Urgency::High => egui::Color32::RED,
                    Urgency::Critical => egui::Color32::from_rgb(255, 0, 0),
                };
                ui.colored_color(urgency_color, format!("{:?}", prediction.urgency));
                
                ui.label(format!("({:.0}% confidence)", prediction.confidence * 100.0));
            });
        }
    }
}
```

### Cultural Intelligence Widget

```rust
#[derive(Debug, Clone)]
pub struct CulturalIntelligenceWidget {
    pub cultural_states: HashMap<Culture, CulturalState>,
    pub relationship_matrix: HashMap<(Culture, Culture), RelationshipStatus>,
    pub cultural_events: Vec<CulturalEvent>,
    pub mood_predictions: HashMap<Culture, MoodPrediction>,
    pub conflict_warnings: Vec<ConflictWarning>,
}

#[derive(Debug, Clone)]
pub struct CulturalState {
    pub culture: Culture,
    pub current_openness: f32,
    pub mood: CulturalMood,
    pub recent_events: Vec<CulturalEvent>,
    pub economic_status: EconomicStatus,
    pub territorial_changes: Vec<TerritorialChange>,
    pub diplomatic_status: DiplomaticStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CulturalMood {
    Prosperous,
    Content,
    Anxious,
    Hostile,
    Isolated,
    Expansive,
}

#[derive(Debug, Clone)]
pub struct ConflictWarning {
    pub cultures: (Culture, Culture),
    pub conflict_probability: f32,
    pub conflict_type: ConflictType,
    pub estimated_time: Option<Duration>,
    pub recommended_action: String,
    pub shepherd_intervention_opportunity: bool,
}

impl CulturalIntelligenceWidget {
    pub fn new() -> Self {
        Self {
            cultural_states: HashMap::new(),
            relationship_matrix: HashMap::new(),
            cultural_events: Vec::new(),
            mood_predictions: HashMap::new(),
            conflict_warnings: Vec::new(),
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Cultural Intelligence");
        
        // Conflict warnings first (most urgent)
        if !self.conflict_warnings.is_empty() {
            self.render_conflict_warnings(ui, context);
            ui.separator();
        }
        
        // Cultural overview
        self.render_cultural_overview(ui, context);
        
        ui.separator();
        
        // Relationship matrix
        self.render_relationship_matrix(ui, context);
        
        ui.separator();
        
        // Recent events
        self.render_recent_events(ui, context);
    }
    
    fn render_conflict_warnings(&mut self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("⚠️ Conflict Warnings");
        
        for warning in &self.conflict_warnings {
            let warning_color = if warning.conflict_probability > 0.8 {
                egui::Color32::RED
            } else if warning.conflict_probability > 0.5 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::from_rgb(255, 165, 0)
            };
            
            ui.horizontal(|ui| {
                ui.colored_color(warning_color, "⚠️");
                ui.label(format!("{} ↔ {}", warning.cultures.0, warning.cultures.1));
                ui.label(format!("({:.0}% chance)", warning.conflict_probability * 100.0));
            });
            
            ui.indent(|ui| {
                ui.label(format!("Type: {:?}", warning.conflict_type));
                ui.label(format!("Recommended: {}", warning.recommended_action));
                
                if warning.shepherd_intervention_opportunity {
                    ui.colored_color(egui::Color32::GREEN, "🤝 Shepherd intervention possible");
                }
                
                if let Some(estimated_time) = warning.estimated_time {
                    ui.label(format!("Estimated: {:?}", estimated_time));
                }
            });
            
            ui.separator();
        }
    }
    
    fn render_cultural_overview(&self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Cultural Overview");
        
        let cultures = [Culture::Ember, Culture::Tide, Culture::Gale, Culture::Orange, 
                        Culture::Marsh, Culture::Crystal, Culture::Amber, Culture::Teal, Culture::Tundra];
        
        for culture in cultures {
            if let Some(state) = self.cultural_states.get(&culture) {
                ui.horizontal(|ui| {
                    // Culture icon and name
                    let icon = culture.get_symbol();
                    ui.label(format!("{} {}", icon, culture));
                    
                    // Current mood
                    let mood_icon = match state.mood {
                        CulturalMood::Prosperous => "😊",
                        CulturalMood::Content => "🙂",
                        CulturalMood::Anxious => "😟",
                        CulturalMood::Hostile => "😠",
                        CulturalMood::Isolated => "🏔️",
                        CulturalMood::Expansive => "📈",
                    };
                    ui.label(mood_icon);
                    
                    // Openness
                    ui.label(format!("Open: {:.0}%", state.current_openness * 100.0));
                    
                    // Economic status
                    let economy_icon = match state.economic_status {
                        EconomicStatus::Thriving => "💰",
                        EconomicStatus::Stable => "💵",
                        EconomicStatus::Struggling => "📉",
                        EconomicStatus::Crisis => "🚨",
                    };
                    ui.label(economy_icon);
                });
                
                // Recent events
                if !state.recent_events.is_empty() {
                    ui.indent(|ui| {
                        ui.label("Recent events:");
                        for event in state.recent_events.iter().take(3) {
                            ui.label(format!("• {}", event.description));
                        }
                    });
                }
            }
        }
    }
    
    fn render_relationship_matrix(&self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Relationship Matrix");
        
        // Create a simple relationship grid
        let cultures = [Culture::Ember, Culture::Tide, Culture::Gale, Culture::Marsh, Culture::Crystal];
        
        for i in 0..cultures.len() {
            ui.horizontal(|ui| {
                for j in 0..cultures.len() {
                    if i == j {
                        // Diagonal - show culture itself
                        ui.label(format!("{}", cultures[i].get_symbol()));
                    } else {
                        // Off-diagonal - show relationship
                        let relationship = self.relationship_matrix.get(&(cultures[i], cultures[j]));
                        
                        if let Some(rel) = relationship {
                            let relationship_icon = match rel.status {
                                RelationshipStatus::Allied => "🤝",
                                RelationshipStatus::Friendly => "😊",
                                RelationshipStatus::Neutral => "😐",
                                RelationshipStatus::Suspicious => "🤔",
                                RelationshipStatus::Hostile => "⚔️",
                            };
                            ui.label(relationship_icon);
                        } else {
                            ui.label("❓");
                        }
                    }
                }
            });
        }
    }
}
```

### Diplomatic Advisor Widget

```rust
#[derive(Debug, Clone)]
pub struct DiplomaticAdvisorWidget {
    pub current_situations: Vec<DiplomaticSituation>,
    pub recommended_actions: Vec<DiplomaticRecommendation],
    pub success_probabilities: HashMap<(Culture, Culture), f32>,
    pub diplomatic_options: Vec<DiplomaticOption>,
    pub intervention_opportunities: Vec<InterventionOpportunity>,
}

#[derive(Debug, Clone)]
pub struct DiplomaticSituation {
    pub id: Uuid,
    pub involved_cultures: (Culture, Culture),
    pub situation_type: SituationType,
    pub urgency: Urgency,
    pub description: String,
    pub background: String,
    pub potential_outcomes: Vec<PotentialOutcome>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SituationType {
    TradeDispute,
    BorderConflict,
    CulturalMisunderstanding,
    ResourceCompetition,
    SubFactionDispute,
    ExternalThreat,
}

#[derive(Debug, Clone)]
pub struct DiplomaticRecommendation {
    pub situation_id: Uuid,
    pub recommended_action: RecommendedAction,
    pub success_probability: f32,
    pub resource_cost: ResourceCost,
    pub time_required: Duration,
    pub risks: Vec<String>,
    pub benefits: Vec<String>,
    pub elder_confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendedAction {
    MediateTrade,
    FacilitateDialogue,
    SendGift,
    ProposeAlliance,
    OrganizeSummit,
    InterveneDirectly,
    ProvideArbitration,
}

impl DiplomaticAdvisorWidget {
    pub fn new() -> Self {
        Self {
            current_situations: Vec::new(),
            recommended_actions: Vec::new(),
            success_probabilities: HashMap::new(),
            diplomatic_options: Vec::new(),
            intervention_opportunities: Vec::new(),
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Diplomatic Advisor");
        
        // Current situations
        self.render_current_situations(ui, context);
        
        ui.separator();
        
        // Recommendations
        self.render_recommendations(ui, context);
        
        ui.separator();
        
        // Intervention opportunities
        self.render_intervention_opportunities(ui, context);
    }
    
    fn render_current_situations(&self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Current Situations");
        
        for situation in &self.current_situations {
            let urgency_color = match situation.urgency {
                Urgency::Critical => egui::Color32::RED,
                Urgency::High => egui::Color32::from_rgb(255, 165, 0),
                Urgency::Medium => egui::Color32::YELLOW,
                Urgency::Low => egui::Color32::GREEN,
            };
            
            ui.horizontal(|ui| {
                ui.colored_color(urgency_color, format!("{:?}", situation.urgency));
                ui.label(format!("{} ↔ {}", situation.involved_cultures.0, situation.involved_cultures.1));
                ui.label(format!("{:?}", situation.situation_type));
            });
            
            ui.indent(|ui| {
                ui.label(&situation.description);
                
                if !situation.background.is_empty() {
                    ui.collapsing("Background", |ui| {
                        ui.label(&situation.background);
                    });
                }
                
                // Potential outcomes
                ui.collapsing("Potential Outcomes", |ui| {
                    for outcome in &situation.potential_outcomes {
                        ui.horizontal(|ui| {
                            ui.label(format!("• {}", outcome.description));
                            ui.label(format!("({:.0}% chance)", outcome.probability * 100.0));
                        });
                    }
                });
            });
            
            ui.separator();
        }
    }
    
    fn render_recommendations(&self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("Elder Recommendations");
        
        for recommendation in &self.recommended_actions {
            ui.horizontal(|ui| {
                // Action icon
                let action_icon = match recommendation.recommended_action {
                    RecommendedAction::MediateTrade => "💰",
                    RecommendedAction::FacilitateDialogue => "💬",
                    RecommendedAction::SendGift => "🎁",
                    RecommendedAction::ProposeAlliance => "🤝",
                    RecommendedAction::OrganizeSummit => "🏛️",
                    RecommendedAction::InterveneDirectly => "⚔️",
                    RecommendedAction::ProvideArbitration => "⚖️",
                };
                ui.label(action_icon);
                
                // Success probability
                let success_color = if recommendation.success_probability > 0.7 {
                    egui::Color32::GREEN
                } else if recommendation.success_probability > 0.4 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::RED
                };
                ui.colored_color(success_color, format!("{:.0}%", recommendation.success_probability * 100.0));
                
                // Time required
                ui.label(format!("{:?}", recommendation.time_required));
                
                // Elder confidence
                ui.label(format!("Conf: {:.0}%", recommendation.elder_confidence * 100.0));
            });
            
            ui.indent(|ui| {
                // Resource cost
                ui.horizontal(|ui| {
                    ui.label("Cost:");
                    if recommendation.resource_cost.biomass > 0 {
                        ui.label(format!("🌱 {}", recommendation.resource_cost.biomass));
                    }
                    if recommendation.resource_cost.scrap > 0 {
                        ui.label(format!("⚙️ {}", recommendation.resource_cost.scrap));
                    }
                    if recommendation.resource_cost.energy > 0 {
                        ui.label(format!("⚡ {}", recommendation.resource_cost.energy));
                    }
                });
                
                // Risks
                if !recommendation.risks.is_empty() {
                    ui.collapsing("Risks", |ui| {
                        for risk in &recommendation.risks {
                            ui.label(format!("• {}", risk));
                        }
                    });
                }
                
                // Benefits
                if !recommendation.benefits.is_empty() {
                    ui.collapsing("Benefits", |ui| {
                        for benefit in &recommendation.benefits {
                            ui.label(format!("• {}", benefit));
                        }
                    });
                }
            });
            
            ui.separator();
        }
    }
    
    fn render_intervention_opportunities(&self, ui: &mut egui::Ui, context: &ElderContext) {
        ui.heading("🤝 Shepherd Intervention Opportunities");
        
        for opportunity in &self.intervention_opportunities {
            ui.horizontal(|ui| {
                ui.label(format!("{} ↔ {}", opportunity.cultures.0, opportunity.cultures.1));
                ui.label(format!("{:?}", opportunity.opportunity_type));
                ui.label(format!("Reward: {} {}", opportunity.reward_type, opportunity.reward_amount));
            });
            
            ui.indent(|ui| {
                ui.label(&opportunity.description);
                ui.label(format!("Difficulty: {:?}", opportunity.difficulty));
                ui.label(format!("Time Limit: {:?}", opportunity.time_limit));
            });
            
            ui.separator();
        }
    }
}
```

## Implementation Tasks

### UI Component Development

1. **Create Elder Avatar System**: Dynamic mood-based visualization
2. **Implement Market Analysis Widget**: Real-time economic intelligence
3. **Build Cultural Intelligence Widget**: Cultural behavior analysis
4. **Develop Diplomatic Advisor**: Relationship and conflict guidance
5. **Create Conversation System**: Interactive dialogue with Elder

### Data Integration

1. **Connect to Market System**: Pull real-time market data
2. **Integrate Cultural Behavior**: Access personality core data
3. **Link to Trade System**: Display trade opportunities
4. **Connect to Mission System**: Show diplomatic missions

### User Experience

1. **Responsive Design**: Adapt to different screen sizes
2. **Accessibility**: Screen reader support and high contrast
3. **Performance**: Efficient rendering of complex data
4. **Intuitive Navigation**: Clear tab organization

## Validation Criteria

- [ ] Elder provides accurate and timely market analysis
- [ ] Cultural intelligence reflects actual planetary state
- [ ] Diplomatic advice is reliable and actionable
- [ ] UI is responsive and performs well
- [ ] Conversation system feels natural and helpful
- [ ] Recommendations improve with Elder wisdom level

The Elder Consultation UI transforms the Elder into an indispensable strategic advisor who bridges the gap between the Shepherd's ship systems and the planet's cultural web, turning every click into an informed decision that considers the complex interconnections of the living world.
