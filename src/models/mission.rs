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
    pub base_dc: u32,
    pub min_roster_level: u32,
    pub req_strength: u32,
    pub req_agility: u32,
    pub req_intelligence: u32,
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
        base_dc: u32,
        min_level: u32,
        req_str: u32,
        req_agi: u32,
        req_int: u32,
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
            base_dc,
            min_roster_level: min_level,
            req_strength: req_str,
            req_agility: req_agi,
            req_intelligence: req_int,
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

        let coverage = (
            (total_str as f64 / self.req_strength.max(1) as f64).clamp(0.3, 2.0) +
            (total_agi as f64 / self.req_agility.max(1) as f64).clamp(0.3, 2.0) +
            (total_int as f64 / self.req_intelligence.max(1) as f64).clamp(0.3, 2.0)
        ) / 3.0;

        let modifier = crate::combat::D20::modifier_from_coverage(coverage);
        let dc = self.base_dc as i32;
        let target_roll = (dc - modifier).clamp(1, 21);
        let success_faces = (21 - target_roll).clamp(1, 19);
        let chance = success_faces as f64 / 20.0;
        
        let label = match chance {
            c if c >= 0.90 => "GUARANTEED",
            c if c >= 0.75 => "GOOD ODDS",
            c if c >= 0.50 => "RISKY",
            c if c >= 0.25 => "DANGEROUS",
            _              => "DESPERATE",
        };

        (label.to_string(), chance)
    }
}

impl std::fmt::Display for Mission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} | STR:{} AGI:{} INT:{} | Diff:{:.0}% | Dur:{}s | Reward:{}",
            &self.id.to_string()[..8],
            self.name,
            self.req_strength,
            self.req_agility,
            self.req_intelligence,
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
        rolls: Vec<D20Result>,
        xp_gained: u32,
    },
    Failure {
        injured_ids: Vec<Uuid>,
        rolls: Vec<D20Result>,
        xp_gained: u32,
    },
    CriticalFailure {
        injured_ids: Vec<Uuid>,
        rolls: Vec<D20Result>,
        xp_gained: u32,
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

    pub fn resolve<R: Rng>(
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

        let str_cov = coverage(total_str, mission.req_strength);
        let agi_cov = coverage(total_agi, mission.req_agility);
        let int_cov = coverage(total_int, mission.req_intelligence);

        let affinity_bonus = mission.get_affinity_bonus(squad);
        let difficulty = if self.is_emergency {
            mission.difficulty + 15.0 + affinity_bonus
        } else {
            mission.difficulty + affinity_bonus
        };

        let str_roll = D20::mission_check(str_cov, difficulty, RollMode::Normal, rng);
        let agi_roll = D20::mission_check(agi_cov, difficulty, RollMode::Normal, rng);
        let int_roll = D20::mission_check(int_cov, difficulty, RollMode::Normal, rng);

        let successes = [str_roll.success, agi_roll.success, int_roll.success]
            .iter().filter(|&&s| s).count();

        let any_crit_fail = (str_roll.nat_one && !str_roll.success)
            || (agi_roll.nat_one && !agi_roll.success)
            || (int_roll.nat_one && !int_roll.success);

        let rolls = vec![str_roll, agi_roll, int_roll];

        if successes >= 2 {
            AarOutcome::Victory {
                reward: mission.reward,
                success_chance: mission.calculate_success_chance(squad).1,
                rolls,
                xp_gained: 0,
            }
        } else if any_crit_fail && successes == 0 {
            AarOutcome::CriticalFailure {
                injured_ids: Vec::new(),
                rolls,
                xp_gained: 0,
            }
        } else {
            AarOutcome::Failure {
                injured_ids: Vec::new(),
                rolls,
                xp_gained: 0,
            }
        }
    }

    pub fn award_squad_xp(&self, mission: &Mission, squad: &mut [&mut Operator], outcome: &AarOutcome) -> Vec<(Uuid, u32, bool)> {
        let mut results = Vec::new();
        let total_val = mission.reward.total_value();
        let base_xp = match outcome {
            AarOutcome::Victory { .. } => (total_val / 100).max(1) as u32,
            _ => (total_val / 400).max(0) as u32,
        };

        if base_xp == 0 { return results; }

        for op in squad {
            let mut op_xp = base_xp;
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
        Mission::new("Bank Heist Recon",    MissionTier::Starter,  5,  1, 20, 30, 10, 0.10, 60,  ResourceYield::scrap(500),  Some(crate::genetics::Culture::Teal), None, false),
        Mission::new("Corporate Espionage", MissionTier::Standard, 10, 1, 10, 20, 50, 0.25, 120, ResourceYield::scrap(1200), Some(crate::genetics::Culture::Tide), None, false),
        Mission::new("Harbour Extraction",  MissionTier::Standard, 12, 2, 40, 20, 10, 0.20, 90,  ResourceYield::scrap(800),  Some(crate::genetics::Culture::Marsh), None, false),
        Mission::new("Zero-Day Exploit",    MissionTier::Advanced, 15, 3, 10, 10, 70, 0.40, 180, ResourceYield::scrap(2500), Some(crate::genetics::Culture::Orange), None, false),
        Mission::new("Black Site Breach",   MissionTier::Elite,    20, 5, 60, 40, 20, 0.50, 300, ResourceYield::scrap(5000), Some(crate::genetics::Culture::Ember), None, false),
    ]
}

pub(crate) fn blueprint<R: Rng>(rng: &mut R) -> (String, usize, crate::genetics::Culture) {
    let adjs = ["Industrial", "Corporate", "Stealth", "Deep-Sea", "Orbital", "Thermal", "Sub-Zero", "Clandestine"];
    let nouns = ["Extraction", "Espionage", "Sabotage", "Data-Siphon", "Recon", "Breach", "Harvest", "Surveillance"];
    let name = format!("{} {}", adjs.choose(rng).unwrap(), nouns.choose(rng).unwrap());
    
    use crate::genetics::Culture;
    let (stat, cult) = match rng.gen_range(0..3) {
        0 => (0, [Culture::Ember, Culture::Marsh, Culture::Frost].choose(rng).unwrap()),
        1 => (1, [Culture::Teal, Culture::Gale, Culture::Crystal].choose(rng).unwrap()),
        _ => (2, [Culture::Orange, Culture::Tundra, Culture::Tide].choose(rng).unwrap()),
    };
    
    (name, stat, *cult)
}
