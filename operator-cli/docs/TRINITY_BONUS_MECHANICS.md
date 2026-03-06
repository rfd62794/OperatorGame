# Trinity Bonus Mechanics

> **Status:** COMBAT SYSTEM ENHANCEMENT v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-022, CHROMATIC_FRAMEWORK.md, COUNTER_STRATEGY_MATRIX.md

## Overview

The Trinity Bonus provides a strategic reward for assembling balanced squads that represent all three chromatic layers. When a squad contains cultures from the Primary, Secondary, and Tertiary layers, they gain a "Full Spectrum" bonus that enhances their combat effectiveness and strategic flexibility.

## Core Mechanics

### Trinity Composition Requirements

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrinityStatus {
    Complete,           // All three layers present
    Partial,            // Two layers present
    Incomplete,         // One layer only
    VoidEnhanced,       // Complete + Void member
}

pub struct TrinityAnalyzer {
    pub squad_composition: SquadComposition,
    pub trinity_status: TrinityStatus,
    pub bonus_modifier: f32,
}

impl TrinityAnalyzer {
    pub fn analyze_squad(squad: &[Culture]) -> TrinityAnalyzer {
        let has_primary = squad.iter().any(|c| c.layer() == ChromaticLayer::Primary);
        let has_secondary = squad.iter().any(|c| c.layer() == ChromaticLayer::Secondary);
        let has_tertiary = squad.iter().any(|c| c.layer() == ChromaticLayer::Tertiary);
        let has_void = squad.iter().any(|c| c.layer() == ChromaticLayer::Void);
        
        let (status, modifier) = match (has_primary, has_secondary, has_tertiary, has_void) {
            (true, true, true, true) => (TrinityStatus::VoidEnhanced, 1.4),
            (true, true, true, false) => (TrinityStatus::Complete, 1.3),
            (true, true, false, false) | (true, false, true, false) | (false, true, true, false) => {
                (TrinityStatus::Partial, 1.15)
            },
            _ => (TrinityStatus::Incomplete, 1.0),
        };
        
        TrinityAnalyzer {
            squad_composition: SquadComposition::new(squad),
            trinity_status: status,
            bonus_modifier: modifier,
        }
    }
}
```

### Bonus Application System

```rust
pub struct TrinityBonus {
    pub d20_modifier: i32,        // Direct roll bonus
    pub damage_multiplier: f32,   // Combat damage enhancement
    pub survival_bonus: f32,      // Environmental resistance
    pub discovery_boost: f32,      // Enhanced exploration
    pub economic_bonus: f32,      // Resource generation
}

impl TrinityBonus {
    pub fn calculate_bonus(status: TrinityStatus) -> TrinityBonus {
        match status {
            TrinityStatus::VoidEnhanced => TrinityBonus {
                d20_modifier: 3,
                damage_multiplier: 1.4,
                survival_bonus: 1.25,
                discovery_boost: 1.3,
                economic_bonus: 1.2,
            },
            TrinityStatus::Complete => TrinityBonus {
                d20_modifier: 2,
                damage_multiplier: 1.3,
                survival_bonus: 1.2,
                discovery_boost: 1.2,
                economic_bonus: 1.15,
            },
            TrinityStatus::Partial => TrinityBonus {
                d20_modifier: 1,
                damage_multiplier: 1.15,
                survival_bonus: 1.1,
                discovery_boost: 1.1,
                economic_bonus: 1.05,
            },
            TrinityStatus::Incomplete => TrinityBonus {
                d20_modifier: 0,
                damage_multiplier: 1.0,
                survival_bonus: 1.0,
                discovery_boost: 1.0,
                economic_bonus: 1.0,
            },
        }
    }
}
```

## Combat Integration

### Enhanced Combat Resolution

```rust
impl CombatResolver {
    pub fn resolve_with_trinity(
        attacker_squad: &[SlimeGenome],
        defender_squad: &[SlimeGenome],
        base_success_chance: f64,
    ) -> CombatResult {
        // Analyze Trinity status for both squads
        let attacker_trinity = TrinityAnalyzer::analyze_squad(
            &attacker_squad.iter().map(|s| s.culture).collect::<Vec<_>>()
        );
        let defender_trinity = TrinityAnalyzer::analyze_squad(
            &defender_squad.iter().map(|s| s.culture).collect::<Vec<_>>()
        );
        
        // Apply Trinity bonuses
        let attacker_bonus = TrinityBonus::calculate_bonus(attacker_trinity.trinity_status);
        let defender_bonus = TrinityBonus::calculate_bonus(defender_trinity.trinity_status);
        
        // Calculate modified success chance
        let trinity_advantage = attacker_bonus.damage_multiplier / defender_bonus.damage_multiplier;
        let modified_chance = base_success_chance * trinity_advantage;
        
        // Apply D20 modifier
        let d20_roll = roll_d20() + attacker_bonus.d20_modifier;
        
        CombatResult {
            success_chance: modified_chance,
            d20_roll,
            trinity_status_attacker: attacker_trinity.trinity_status,
            trinity_status_defender: defender_trinity.trinity_status,
            applied_bonuses: (attacker_bonus, defender_bonus),
        }
    }
}
```

### Squad Composition Strategies

#### Optimal Trinity Formations

| Formation | Cultures | Strength | Weakness | Use Case |
|-----------|----------|----------|----------|----------|
| **Balanced Trinity** | Primary + Secondary + Tertiary | All-around effectiveness | No specialization | General missions |
| **Void Trinity** | Primary + Secondary + Tertiary + Void | Maximum versatility | Higher cost | Critical objectives |
| **Dual Layer** | Primary + Primary + Secondary | Strong layer focus | Vulnerable to counter | Specialized attacks |
| **Single Layer** | Primary + Primary + Primary | Maximum layer power | Trinity bonus lost | Breakthrough attempts |

## Strategic Applications

### Mission Type Optimization

```rust
pub enum MissionType {
    Assault,      // High combat, low survival
    Exploration,  // High discovery, moderate combat
    Defense,      // High survival, moderate combat
    Economic,     // High economic, low combat
    Diplomatic,   // High discovery, low combat
}

impl MissionType {
    pub fn recommend_trinity_composition(&self) -> Vec<Culture> {
        match self {
            MissionType::Assault => vec![
                Culture::Ember,    // Primary - ATK
                Culture::Crystal,  // Secondary - HP
                Culture::Amber,    // Tertiary - DUR
            ],
            MissionType::Exploration => vec![
                Culture::Gale,     // Primary - SPD
                Culture::Orange,   // Secondary - MND
                Culture::Teal,     // Tertiary - STB
            ],
            MissionType::Defense => vec![
                Culture::Crystal,  // Secondary - HP
                Culture::Marsh,     // Secondary - RES
                Culture::Frost,    // Tertiary - END
            ],
            MissionType::Economic => vec![
                Culture::Orange,   // Secondary - MND
                Culture::Amber,    // Tertiary - DUR
                Culture::Marsh,    // Secondary - RES
            ],
            MissionType::Diplomatic => vec![
                Culture::Tide,     // Primary - CHM
                Culture::Teal,     // Tertiary - STB
                Culture::Gale,     // Primary - SPD
            ],
        }
    }
}
```

### AI Trinity Logic

```rust
impl CommandDeckAI {
    pub fn evaluate_trinity_opportunity(&self, situation: &TacticalSituation) -> TrinityRecommendation {
        let current_squad = &self.available_slimes;
        let mission_requirements = &situation.mission_requirements;
        
        // Check if Trinity is possible
        let trinity_candidates = self.find_trinity_combinations(current_squad);
        
        if trinity_candidates.is_empty() {
            return TrinityRecommendation::NotAvailable;
        }
        
        // Evaluate each Trinity option
        let mut best_option = None;
        let mut best_score = 0.0;
        
        for combination in trinity_candidates {
            let score = self.calculate_trinity_score(&combination, mission_requirements);
            if score > best_score {
                best_score = score;
                best_option = Some(combination);
            }
        }
        
        match best_option {
            Some(optimal) => TrinityRecommendation::Deploy {
                squad: optimal,
                expected_bonus: best_score,
                confidence: self.calculate_confidence(best_score),
            },
            None => TrinityRecommendation::NotAvailable,
        }
    }
    
    fn calculate_trinity_score(&self, squad: &[Culture], requirements: &MissionRequirements) -> f64 {
        let trinity_bonus = TrinityBonus::calculate_bonus(
            TrinityAnalyzer::analyze_squad(squad).trinity_status
        );
        
        let mission_fit = self.calculate_mission_fit(squad, requirements);
        let resource_cost = self.calculate_squad_cost(squad);
        
        (trinity_bonus.damage_multiplier * mission_fit) / resource_cost
    }
}
```

## UI Integration

### Trinity Status Display

```rust
pub struct TrinityStatusWidget {
    pub squad: Vec<SlimeGenome>,
    pub show_detailed_breakdown: bool,
}

impl TrinityStatusWidget {
    pub fn render(&self, ui: &mut egui::Ui) {
        let analyzer = TrinityAnalyzer::analyze_squad(
            &self.squad.iter().map(|s| s.culture).collect::<Vec<_>>()
        );
        
        ui.horizontal(|ui| {
            // Trinity indicator
            match analyzer.trinity_status {
                TrinityStatus::VoidEnhanced => {
                    ui.colored_color(egui::Color32::GOLD, "⚫ Void Trinity");
                },
                TrinityStatus::Complete => {
                    ui.colored_color(egui::Color32::GREEN, "🔺 Complete Trinity");
                },
                TrinityStatus::Partial => {
                    ui.colored_color(egui::Color32::YELLOW, "△ Partial Trinity");
                },
                TrinityStatus::Incomplete => {
                    ui.colored_color(egui::Color32::GRAY, "✕ No Trinity");
                },
            }
            
            // Bonus breakdown
            ui.separator();
            ui.label(format!("Bonus: {:.0}%", (analyzer.bonus_modifier - 1.0) * 100.0));
        });
        
        if self.show_detailed_breakdown {
            self.render_detailed_breakdown(ui, &analyzer);
        }
    }
    
    fn render_detailed_breakdown(&self, ui: &mut egui::Ui, analyzer: &TrinityAnalyzer) {
        let bonus = TrinityBonus::calculate_bonus(analyzer.trinity_status);
        
        ui.horizontal(|ui| {
            ui.label("D20 Bonus:");
            ui.colored_color(egui::Color32::LIGHT_BLUE, format!("+{}", bonus.d20_modifier));
        });
        
        ui.horizontal(|ui| {
            ui.label("Damage:");
            ui.colored_color(egui::Color32::RED, format!("×{:.2}", bonus.damage_multiplier));
        });
        
        ui.horizontal(|ui| {
            ui.label("Survival:");
            ui.colored_color(egui::Color32::GREEN, format!("×{:.2}", bonus.survival_bonus));
        });
    }
}
```

### Squad Builder Integration

```rust
pub struct SquadBuilderWidget {
    pub available_slimes: Vec<SlimeGenome>,
    pub selected_squad: Vec<SlimeGenome>,
    pub trinity_hints: bool,
}

impl SquadBuilderWidget {
    pub fn render_trinity_hints(&self, ui: &mut egui::Ui) {
        if !self.trinity_hints {
            return;
        }
        
        let current_layers: HashSet<_> = self.selected_squad
            .iter()
            .map(|s| s.culture.layer())
            .collect();
        
        let missing_layers = vec![
            ChromaticLayer::Primary,
            ChromaticLayer::Secondary,
            ChromaticLayer::Tertiary,
        ]
        .into_iter()
        .filter(|layer| !current_layers.contains(layer))
        .collect::<Vec<_>>();
        
        if !missing_layers.is_empty() {
            ui.horizontal(|ui| {
                ui.label("Missing layers for Trinity:");
                for layer in missing_layers {
                    let (icon, color) = match layer {
                        ChromaticLayer::Primary => ("🔥", egui::Color32::RED),
                        ChromaticLayer::Secondary => ("⚙️", egui::Color32::YELLOW),
                        ChromaticLayer::Tertiary => ("🏔️", egui::Color32::BLUE),
                        ChromaticLayer::Void => ("⚫", egui::Color32::BLACK),
                    };
                    ui.colored_color(color, icon);
                }
            });
        }
        
        // Suggest candidates for missing layers
        for layer in missing_layers {
            if let Some(candidates) = self.find_layer_candidates(layer) {
                ui.horizontal(|ui| {
                    ui.label(format!("{} candidates:", layer));
                    for slime in candidates.iter().take(3) {
                        if ui.button(&slime.name).clicked() {
                            // Add to squad
                        }
                    }
                });
            }
        }
    }
}
```

## Balancing Considerations

### Bonus Tuning Parameters

| Parameter | Current Value | Range | Impact |
|-----------|---------------|-------|--------|
| **Complete Trinity D20** | +2 | +1 to +3 | Direct combat advantage |
| **Complete Trinity Damage** | ×1.3 | ×1.2 to ×1.5 | Combat effectiveness |
| **Void Trinity D20** | +3 | +2 to +4 | Ultimate squad bonus |
| **Void Trinity Damage** | ×1.4 | ×1.3 to ×1.6 | Endgame power scaling |

### Economic Balance

```rust
pub struct TrinityEconomics {
    pub squad_cost_multiplier: f32,
    pub maintenance_cost_modifier: f32,
    pub experience_bonus: f32,
    pub discovery_acceleration: f32,
}

impl TrinityEconomics {
    pub fn calculate_trinity_efficiency(status: TrinityStatus) -> TrinityEconomics {
        match status {
            TrinityStatus::VoidEnhanced => TrinityEconomics {
                squad_cost_multiplier: 1.5,  // 50% more expensive
                maintenance_cost_modifier: 1.3,
                experience_bonus: 1.4,
                discovery_acceleration: 1.5,
            },
            TrinityStatus::Complete => TrinityEconomics {
                squad_cost_multiplier: 1.2,  // 20% more expensive
                maintenance_cost_modifier: 1.15,
                experience_bonus: 1.25,
                discovery_acceleration: 1.3,
            },
            TrinityStatus::Partial => TrinityEconomics {
                squad_cost_multiplier: 1.1,  // 10% more expensive
                maintenance_cost_modifier: 1.05,
                experience_bonus: 1.1,
                discovery_acceleration: 1.15,
            },
            TrinityStatus::Incomplete => TrinityEconomics {
                squad_cost_multiplier: 1.0,  // No bonus or penalty
                maintenance_cost_modifier: 1.0,
                experience_bonus: 1.0,
                discovery_acceleration: 1.0,
            },
        }
    }
}
```

## Validation Metrics

### Performance Indicators

- [ ] Trinity squads achieve 15-30% higher success rates
- [ ] Player Trinity squad usage: 40-60% of deployments
- [ ] Void Trinity achievement rate: 5-10% of endgame players
- [ ] Mission completion time reduction with Trinity: 10-20%
- [ ] Player satisfaction with Trinity system: >80%

### Testing Scenarios

1. **Balanced Trinity**: Primary + Secondary + Tertiary vs. single-layer squads
2. **Void Trinity**: Complete Trinity + Void vs. standard Trinity
3. **Economic Analysis**: Cost-benefit of Trinity vs. specialized squads
4. **AI Behavior**: AI Trinity preference and counter-strategies
5. **Progression Scaling**: Trinity effectiveness throughout game stages

## Future Enhancements

1. **Advanced Trinity**: 4+ member squads with enhanced bonuses
2. **Dynamic Trinity**: Layer-specific bonuses based on mission context
3. **Trinity Evolution**: Trinity squads that evolve together
4. **Alliance Trinity**: Multi-player Trinity combinations
5. **Environmental Trinity**: Biome-specific Trinity enhancements

The Trinity Bonus system provides strategic depth that rewards thoughtful squad composition while maintaining balance through economic costs and counter-strategies. It creates meaningful choices between specialized power and versatile balance.
