// Moved from models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use rand::Rng;
use rand::seq::SliceRandom;
use crate::combat::{D20Result, D20, RollMode};
use crate::models::operator::{Operator, SlimeState, compute_final_stat};

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

/// Structured mission rewards.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct ResourceYield {
    pub biomass:  u32,
    pub scrap:    u32,
    pub reagents: u32,
}

impl ResourceYield {
    pub fn new(biomass: u32, scrap: u32, reagents: u32) -> Self {
        Self { biomass, scrap, reagents }
    }

    pub fn scrap(amount: u32) -> Self {
        Self { biomass: 0, scrap: amount, reagents: 0 }
    }

    /// Scale all yields by `factor`, rounding down.
    pub fn scaled(&self, factor: f32) -> Self {
        Self {
            biomass:  (self.biomass  as f32 * factor) as u32,
            scrap:    (self.scrap    as f32 * factor) as u32,
            reagents: (self.reagents as f32 * factor) as u32,
        }
    }

    pub fn apply_to_inventory(&self, inv: &mut crate::inventory::Inventory) {
        inv.biomass += self.biomass as u64;
        inv.scrap   += self.scrap as u64;
        inv.reagents += self.reagents;
    }

    pub fn total_value(&self) -> u32 {
        self.biomass + self.scrap + (self.reagents * 10)
    }
}

impl std::fmt::Display for ResourceYield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.biomass > 0 && self.scrap == 0 && self.reagents == 0 {
            write!(f, "{} Biomass", self.biomass)
        } else if self.scrap > 0 && self.biomass == 0 && self.reagents == 0 {
            write!(f, "${}", self.scrap)
        } else if self.reagents > 0 && self.biomass == 0 && self.scrap == 0 {
            write!(f, "{} Reagents", self.reagents)
        } else {
            write!(f, "B:{} S:{} R:{}", self.biomass, self.scrap, self.reagents)
        }
    }
}

// ---------------------------------------------------------------------------
// Targets & Enemies (G.5 Gauntlet)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Target {
    pub name: String,
    pub description: String,
    pub base_dc: u32,
    pub req_strength: u32,
    pub req_agility: u32,
    pub req_intelligence: u32,
}

impl Target {
    pub fn new(name: impl Into<String>, dc: u32, str: u32, agi: u32, int: u32) -> Self {
        Self {
            name: name.into(),
            description: "Enemy unit detected in local perimeter.".into(),
            base_dc: dc,
            req_strength: str,
            req_agility: agi,
            req_intelligence: int,
        }
    }
}

// ---------------------------------------------------------------------------
// Mission
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MissionTier {
    /// DC 5-8. Intro missions for L1-L2 slimes.
    Starter,
    /// DC 10-13. Standard contracts for L3-L5 slimes.
    Standard,
    /// DC 15-18. High-risk ops for L6-L8 slimes.
    Advanced,
    /// DC 20-25. Apex contracts for L9-L10 slimes.
    Elite,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub tier: MissionTier,
    pub targets: Vec<Target>,
    pub min_roster_level: u32,
    pub difficulty: f64,
    pub duration_secs: u64,
    pub reward: ResourceYield,
    pub affinity: Option<crate::genetics::Culture>,
    #[serde(default)]
    pub node_id: Option<usize>,
    #[serde(default)]
    pub is_scout: bool,
}

impl Mission {
    pub fn new(
        name: impl Into<String>,
        tier: MissionTier,
        targets: Vec<Target>,
        min_level: u32,
        difficulty: f64,
        duration_secs: u64,
        reward: ResourceYield,
        affinity: Option<crate::genetics::Culture>,
        node_id: Option<usize>,
        is_scout: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: "High-priority contract from the Orbitals.".into(),
            tier,
            targets,
            min_roster_level: min_level,
            difficulty,
            duration_secs,
            reward,
            affinity,
            node_id,
            is_scout,
        }
    }

    pub fn get_affinity_bonus(&self, squad: &[&Operator]) -> f64 {
        if let Some(aff) = self.affinity {
            if squad.iter().any(|s| s.genome.dominant_culture() == aff) {
                return -15.0;
            }
        }
        0.0
    }

    pub fn calculate_success_chance(&self, squad: &[&Operator]) -> (String, f64) {
        if squad.is_empty() {
            return ("UNSTAFFED".to_string(), 0.0);
        }

        let mut total_str = 0u32;
        let mut total_agi = 0u32;
        let mut total_int = 0u32;

        for op in squad {
            let (s, a, i, _, _, _) = op.total_stats();
            total_str += s;
            total_agi += a;
            total_int += i;
        }

        // Cumulative Success Probability across all targets
        let mut cumulative_chance = 1.0;
        
        for target in &self.targets {
            let coverage = (
                (total_str as f64 / target.req_strength.max(1) as f64).clamp(0.3, 2.0) +
                (total_agi as f64 / target.req_agility.max(1) as f64).clamp(0.3, 2.0) +
                (total_int as f64 / target.req_intelligence.max(1) as f64).clamp(0.3, 2.0)
            ) / 3.0;

            let modifier = crate::combat::D20::modifier_from_coverage(coverage);
            let dc = target.base_dc as i32;
            let target_roll = (dc - modifier).clamp(1, 21);
            let success_faces = (21 - target_roll).clamp(1, 19);
            let target_chance = success_faces as f64 / 20.0;
            
            cumulative_chance *= target_chance;
        }
        
        let label = match cumulative_chance {
            c if c >= 0.90 => "GUARANTEED",
            c if c >= 0.75 => "GOOD ODDS",
            c if c >= 0.50 => "RISKY",
            c if c >= 0.25 => "DANGEROUS",
            _              => "DESPERATE",
        };

        (label.to_string(), cumulative_chance)
    }
}

impl std::fmt::Display for Mission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} | Targets:{} | Diff:{:.0}% | Dur:{}s | Reward:{}",
            &self.id.to_string()[..8],
            self.name,
            self.targets.len(),
            self.difficulty * 100.0,
            self.duration_secs,
            self.reward,
        )
    }
}

#[derive(Debug, Clone)]
pub enum AarOutcome {
    Victory {
        reward: ResourceYield,
        success_chance: f64,
        rolls: Vec<crate::combat::D20Result>,
        xp_gained: u32,
        targets_defeated: usize,
        total_targets: usize,
    },
    Failure {
        injured_ids: Vec<Uuid>,
        rolls: Vec<crate::combat::D20Result>,
        xp_gained: u32,
        targets_defeated: usize,
        total_targets: usize,
    },
    CriticalFailure {
        injured_ids: Vec<Uuid>,
        rolls: Vec<crate::combat::D20Result>,
        xp_gained: u32,
        targets_defeated: usize,
        total_targets: usize,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deployment {
    pub id: Uuid,
    pub mission_id: Uuid,
    pub operator_ids: Vec<Uuid>,
    pub completes_at: DateTime<Utc>,
    pub resolved: bool,
    pub is_emergency: bool,
}

impl Deployment {
    pub fn start(mission: &Mission, operator_ids: Vec<Uuid>, is_emergency: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            mission_id: mission.id,
            operator_ids,
            completes_at: Utc::now() + Duration::seconds(mission.duration_secs as i64),
            resolved: false,
            is_emergency,
        }
    }

    pub fn is_complete(&self) -> bool {
        Utc::now() >= self.completes_at
    }

    pub fn resolve<R: rand::Rng>(
        &self,
        mission: &Mission,
        squad: &[&Operator],
        rng: &mut R,
    ) -> AarOutcome {
        let mut total_str = 0u32;
        let mut total_agi = 0u32;
        let mut total_int = 0u32;
        for op in squad {
            let (s, a, i, _, _, _) = op.total_stats();
            total_str += s;
            total_agi += a;
            total_int += i;
        }

        let coverage = |stat: u32, req: u32| -> f64 {
            if req == 0 { 2.0 } else { (stat as f64 / req as f64).clamp(0.0, 2.0) }
        };

        let affinity_bonus = mission.get_affinity_bonus(squad);
        let difficulty = if self.is_emergency {
            mission.difficulty + 15.0 + affinity_bonus
        } else {
            mission.difficulty + affinity_bonus
        };

        let mut targets_defeated = 0;
        let mut all_rolls = Vec::new();
        let mut stopped_by_failure = false;
        let mut stopped_by_crit_fail = false;

        for target in &mission.targets {
            use crate::combat::{D20, RollMode};
            let str_cov = coverage(total_str, target.req_strength);
            let agi_cov = coverage(total_agi, target.req_agility);
            let int_cov = coverage(total_int, target.req_intelligence);

            let str_roll = D20::mission_check(str_cov, difficulty, RollMode::Normal, rng);
            let agi_roll = D20::mission_check(agi_cov, difficulty, RollMode::Normal, rng);
            let int_roll = D20::mission_check(int_cov, difficulty, RollMode::Normal, rng);

            let successes = [str_roll.success, agi_roll.success, int_roll.success]
                .iter().filter(|&&s| s).count();

            let any_crit_fail = (str_roll.nat_one && !str_roll.success)
                || (agi_roll.nat_one && !agi_roll.success)
                || (int_roll.nat_one && !int_roll.success);

            all_rolls.push(str_roll);
            all_rolls.push(agi_roll);
            all_rolls.push(int_roll);

            if successes >= 2 {
                targets_defeated += 1;
            } else {
                stopped_by_failure = true;
                if any_crit_fail { stopped_by_crit_fail = true; }
                break; // Stop on first failure
            }
        }

        let total_targets = mission.targets.len();
        let full_clear = targets_defeated == total_targets;
        
        // Reward Scaling (Designer Directive)
        let base_multiplier = if total_targets == 0 { 0.0 } else { 
            targets_defeated as f32 / total_targets as f32 
        };
        
        let reward_multiplier = if full_clear {
            1.10 // 10% Bonus
        } else if targets_defeated == 0 {
            0.10 // 10% Consolation
        } else {
            base_multiplier
        };

        if full_clear {
            AarOutcome::Victory {
                reward: mission.reward.scaled(reward_multiplier),
                success_chance: mission.calculate_success_chance(squad).1,
                rolls: all_rolls,
                xp_gained: 0,
                targets_defeated,
                total_targets,
            }
        } else if stopped_by_crit_fail && targets_defeated == 0 {
            AarOutcome::CriticalFailure {
                injured_ids: Vec::new(),
                rolls: all_rolls,
                xp_gained: 0,
                targets_defeated,
                total_targets,
            }
        } else {
            // Partial Success or simple Failure
            AarOutcome::Failure {
                injured_ids: Vec::new(),
                rolls: all_rolls,
                xp_gained: 0, // award_squad_xp will fill this
                targets_defeated,
                total_targets,
            }
        }
    }

    pub fn award_squad_xp(&self, mission: &Mission, squad: &mut [&mut Operator], outcome: &AarOutcome) -> Vec<(Uuid, u32, bool)> {
        let mut results = Vec::new();

        let (defeated, total) = match outcome {
            AarOutcome::Victory { targets_defeated, total_targets, .. } => (*targets_defeated, *total_targets),
            AarOutcome::Failure { targets_defeated, total_targets, .. } => (*targets_defeated, *total_targets),
            AarOutcome::CriticalFailure { targets_defeated, total_targets, .. } => (*targets_defeated, *total_targets),
        };

        // G.6 XP Formula: 35 XP per target defeated on victory, 10 XP on failure/partial.
        // Consolation (0 targets): 5 XP flat. Eliminates reward-value coupling.
        // Design target: 2-3 starter victories reach Level 2 (need 100 XP from base 100).
        // Starter (1 target): 35 XP / victory → Level 2 in 3 wins. ✓
        // Standard (1-3 targets): 35-105 XP / victory. ✓
        let full_clear = defeated == total && total > 0;
        let base_xp: u32 = if full_clear {
            // Full clear bonus: extra 10% rounding up
            ((defeated as f32 * 35.0) * 1.10).ceil() as u32
        } else if defeated == 0 {
            5  // Consolation: showed up, learned nothing useful
        } else {
            defeated as u32 * 10  // Partial: 10 XP per target cleared
        };

        if base_xp == 0 { return results; }

        for op in squad {
            let mut op_xp = base_xp;
            // Cultural affinity bonus: 25% XP boost for matching culture
            if let Some(aff) = mission.affinity {
                if op.genome.dominant_culture() == aff {
                    op_xp = (op_xp as f64 * 1.25) as u32;
                }
            }
            let leveled = op.award_xp(op_xp);
            results.push((op.id(), op_xp, leveled));
        }
        results
    }
}

pub fn apply_outcome_injuries(
    outcome: &mut AarOutcome,
    roster: &mut [Operator],
    squad_ids: &[Uuid],
    rng: &mut impl Rng,
) -> Vec<(Uuid, DateTime<Utc>)> {
    let mut injured = Vec::new();
    let already_idle = roster.iter().filter(|s| matches!(s.state, SlimeState::Idle)).count();
    let mut will_be_available = already_idle + squad_ids.len();

    match outcome {
        AarOutcome::Victory { .. } => {}
        AarOutcome::CriticalFailure { injured_ids, .. } => {
            let mut pool = squad_ids.to_vec();
            pool.shuffle(rng);
            let count = if pool.len() >= 2 { rng.gen_range(1..=2) } else { 1 };
            let hours = rng.gen_range(4..=8);
            let until = Utc::now() + Duration::hours(hours);

            for id in pool.into_iter().take(count) {
                if will_be_available > 1 {
                    if let Some(op) = roster.iter_mut().find(|s| s.genome.id == id) {
                        op.state = SlimeState::Injured(until);
                        injured.push((id, until));
                        will_be_available -= 1;
                    }
                }
            }
            *injured_ids = injured.iter().map(|(id, _)| *id).collect();
        }
        AarOutcome::Failure { injured_ids, .. } => {
            if rng.gen_bool(0.1) && !squad_ids.is_empty() {
                let id = squad_ids[rng.gen_range(0..squad_ids.len())];
                let hours = rng.gen_range(2..=4);
                let until = Utc::now() + Duration::hours(hours);

                if will_be_available > 1 {
                    if let Some(op) = roster.iter_mut().find(|s| s.genome.id == id) {
                        op.state = SlimeState::Injured(until);
                        injured.push((id, until));
                        will_be_available -= 1;
                    }
                }
            }
            *injured_ids = injured.iter().map(|(id, _)| *id).collect();
        }
    }
    injured
}

pub fn seed_missions() -> Vec<Mission> {
    vec![
        Mission::new("Bank Heist Recon",    MissionTier::Starter,  vec![Target::new("Local Security", 5, 20, 30, 10)], 1, 0.10, 60,  ResourceYield::scrap(500),  Some(crate::genetics::Culture::Teal), None, false),
        Mission::new("Corporate Espionage", MissionTier::Standard, vec![Target::new("Data Sentry", 10, 10, 20, 50)], 1, 0.25, 120, ResourceYield::scrap(1200), Some(crate::genetics::Culture::Tide), None, false),
        Mission::new("Harbour Extraction",  MissionTier::Standard, vec![Target::new("Heavy Loader", 12, 40, 20, 10)], 2, 0.20, 90,  ResourceYield::scrap(800),  Some(crate::genetics::Culture::Marsh), None, false),
        Mission::new("Zero-Day Exploit",    MissionTier::Advanced, vec![Target::new("Cyber Grid", 15, 10, 10, 70)], 3, 0.40, 180, ResourceYield::scrap(2500), Some(crate::genetics::Culture::Orange), None, false),
        Mission::new("Black Site Breach",   MissionTier::Elite,    vec![Target::new("Alpha Squad", 20, 60, 40, 20)], 5, 0.50, 300, ResourceYield::scrap(5000), Some(crate::genetics::Culture::Ember), None, false),
    ]
}

pub(crate) fn blueprint<R: Rng>(rng: &mut R) -> (String, Vec<Target>, crate::genetics::Culture) {
    let adjs = ["Industrial", "Corporate", "Stealth", "Deep-Sea", "Orbital", "Thermal", "Sub-Zero", "Clandestine"];
    let nouns = ["Extraction", "Espionage", "Sabotage", "Data-Siphon", "Recon", "Breach", "Harvest", "Surveillance"];
    let name = format!("{} {}", adjs.choose(rng).unwrap(), nouns.choose(rng).unwrap());
    
    use crate::genetics::Culture;
    let (stat_idx, cult) = match rng.gen_range(0..3) {
        0 => (0, [Culture::Ember, Culture::Marsh, Culture::Frost].choose(rng).unwrap()),
        1 => (1, [Culture::Teal, Culture::Gale, Culture::Crystal].choose(rng).unwrap()),
        _ => (2, [Culture::Orange, Culture::Tundra, Culture::Tide].choose(rng).unwrap()),
    };

    // G.5: Generate 1-3 targets
    let target_count = rng.gen_range(1..=3);
    let mut targets = Vec::new();
    for i in 1..=target_count {
        let dc = rng.gen_range(5..=15);
        let s = if stat_idx == 0 { rng.gen_range(20..50) } else { rng.gen_range(5..15) };
        let a = if stat_idx == 1 { rng.gen_range(20..50) } else { rng.gen_range(5..15) };
        let i_stat = if stat_idx == 2 { rng.gen_range(20..50) } else { rng.gen_range(5..15) };
        
        let target_name = if target_count == 1 {
            "Main Objective".to_string()
        } else {
            format!("Layer {} Security", i)
        };
        targets.push(Target::new(target_name, dc, s, a, i_stat));
    }
    
    (name, targets, *cult)
}
