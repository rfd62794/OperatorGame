// Moved from models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::item::{Gear, Hat, HatId};

// ---------------------------------------------------------------------------
// SlimeState — tracks biological lifecycle
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SlimeState {
    Idle,
    Deployed(Uuid),          // UUID of active Mission
    Injured(DateTime<Utc>),  // Timestamp when recovery completes
    Training(TrainingAssignment), // Current training session
}

/// Operational state for training diminishing returns (Sprint 9 §5.2).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TrainingState {
    /// Number of sessions per method today.
    pub session_counts: std::collections::HashMap<TrainingMethod, u32>,
    /// Last time the counts were reset (00:00 UTC).
    pub last_reset: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TrainingAssignment {
    pub method: TrainingMethod,
    pub started_at: DateTime<Utc>,
    pub duration_secs: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrainingMethod {
    EmberSparring,
    MarshEndurance,
    CrystalFocus,
    TideNegotiation,
    GaleSprint,
    TundraMeditation,
    OrangeFieldStudy,
    TealPrecision,
    FrostRecovery,
}

impl TrainingMethod {
    pub fn culture(self) -> crate::genetics::Culture {
        match self {
            TrainingMethod::EmberSparring    => crate::genetics::Culture::Ember,
            TrainingMethod::MarshEndurance   => crate::genetics::Culture::Marsh,
            TrainingMethod::CrystalFocus     => crate::genetics::Culture::Crystal,
            TrainingMethod::TideNegotiation  => crate::genetics::Culture::Tide,
            TrainingMethod::GaleSprint       => crate::genetics::Culture::Gale,
            TrainingMethod::TundraMeditation => crate::genetics::Culture::Tundra,
            TrainingMethod::OrangeFieldStudy => crate::genetics::Culture::Orange,
            TrainingMethod::TealPrecision    => crate::genetics::Culture::Teal,
            TrainingMethod::FrostRecovery    => crate::genetics::Culture::Frost,
        }
    }

    pub fn base_duration(self) -> u32 {
        match self {
            TrainingMethod::EmberSparring    => 7200,   // 2h
            TrainingMethod::MarshEndurance   => 10800,  // 3h
            TrainingMethod::CrystalFocus     => 7200,   // 2h
            TrainingMethod::TideNegotiation  => 3600,   // 1h
            TrainingMethod::GaleSprint       => 5400,   // 1.5h
            TrainingMethod::TundraMeditation => 14400,  // 4h
            TrainingMethod::OrangeFieldStudy => 7200,   // 2h
            TrainingMethod::TealPrecision    => 5400,   // 1.5h
            TrainingMethod::FrostRecovery    => 21600,  // 6h
        }
    }
}

/// The operational layer for an operator (Sprint 9).
/// Wraps the biological SlimeGenome with persistent state and progression.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Operator {
    pub genome: crate::genetics::SlimeGenome,
    // Progression (Operational)
    pub level:        u8,
    pub total_xp:     u32,
    /// Per-stat XP pools (ATK, HP, DEF, CHM, SPD, RES, MND, AGI, END).
    /// Indices match Culture::WHEEL order.
    pub stat_xp:      [u32; 9],
    // Operational State
    pub state:        SlimeState,
    pub training:     TrainingState,
    pub equipped_gear: Vec<Gear>,
    #[serde(default)]
    pub equipped_hat: Option<HatId>,
    #[serde(default)]
    pub synthesis_cooldown_until: Option<DateTime<Utc>>,
}

impl Operator {
    pub fn new(genome: crate::genetics::SlimeGenome) -> Self {
        Self {
            genome,
            level: 1,
            total_xp: 100, // Start at Level 1 (Sprint 9 §2 logic)
            stat_xp: [0; 9],
            state: SlimeState::Idle,
            training: TrainingState::default(),
            equipped_gear: Vec::new(),
            equipped_hat: None,
            synthesis_cooldown_until: None,
        }
    }

    pub fn id(&self) -> Uuid { self.genome.id }
    pub fn name(&self) -> &str { &self.genome.name }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Lvl {})", self.name(), self.level)
    }
}

impl Operator {
    pub fn life_stage(&self) -> crate::genetics::LifeStage {
        crate::genetics::LifeStage::from_level(self.level)
    }

    pub fn xp_to_next(&self) -> u32 {
        crate::genetics::LifeStage::xp_to_next(self.level)
    }

    pub fn is_dispatched(&self) -> bool {
        matches!(self.state, SlimeState::Deployed(_))
    }

    pub fn is_injured(&self) -> bool {
        matches!(self.state, SlimeState::Injured(_))
    }

    pub fn is_available(&self) -> bool {
        if self.is_dispatched() { return false; }
        if let SlimeState::Injured(until) = self.state {
            if until > Utc::now() { return false; }
        }
        true
    }

    pub fn can_synthesize(&self) -> bool {
        // Synthesis possible if not deployed, not injured, and cooldown expired.
        if self.is_dispatched() || self.is_injured() { return false; }
        if let Some(until) = self.synthesis_cooldown_until {
            if until > Utc::now() { return false; }
        }
        true
    }

    /// Add XP and return true if levelled up (Sprint 9 §2).
    pub fn award_xp(&mut self, amount: u32) -> bool {
        self.total_xp += amount;
        let old_level = self.level;
        self.level = crate::genetics::LifeStage::level_from_xp(self.total_xp);
        self.level > old_level
    }

    /// Awards specific stat XP (Sprint 9 §4.2).
    pub fn award_stat_xp(&mut self, culture: crate::genetics::Culture, amount: u32) {
        if let Some(idx) = culture.wheel_index() {
            self.stat_xp[idx] += amount;
        }
    }

    /// Total stats including base, training, gear, and hats (G.3).
    pub fn total_stats(&self) -> (u32, u32, u32, u32, u32, u32) {
        let genome = &self.genome;
        let s = compute_final_stat(genome.base_strength, self.stat_xp[crate::genetics::Culture::Ember.wheel_index().unwrap()], self.level);
        let a = compute_final_stat(genome.base_agility, self.stat_xp[crate::genetics::Culture::Teal.wheel_index().unwrap()], self.level);
        let i = compute_final_stat(genome.base_intelligence, self.stat_xp[crate::genetics::Culture::Orange.wheel_index().unwrap()], self.level);
        let m = compute_final_stat(genome.base_mind, self.stat_xp[crate::genetics::Culture::Orange.wheel_index().unwrap()], self.level); // MND deferred to Orange
        let se = compute_final_stat(genome.base_sensory, self.stat_xp[crate::genetics::Culture::Teal.wheel_index().unwrap()], self.level); // Sensory deferred to Teal
        let t = compute_final_stat(genome.base_tenacity, self.stat_xp[crate::genetics::Culture::Frost.wheel_index().unwrap()], self.level); // Tenacity deferred to Frost
        
        // Target: Final stats are post-multiplier flat bonuses
        let mut fs = s;
        let mut fa = a;
        let mut fi = i;
        
        // Gear bonuses
        for gear in &self.equipped_gear {
            let (gs, ga, gi) = gear.stat_bonus();
            fs += gs;
            fa += ga;
            fi += gi;
        }

        // Hat bonuses (G.3)
        if let Some(hat_id) = &self.equipped_hat {
            let hat = Hat::from_id(hat_id);
            fs += hat.str_bonus as u32;
            fa += hat.agi_bonus as u32;
            fi += hat.int_bonus as u32;
        }

        (fs, fa, fi, m, se, t)
    }

    /// Tick: clear Injured state if recovery timestamp has passed.
    pub fn tick_recovery(&mut self) -> Option<String> {
        if let SlimeState::Injured(until) = self.state {
            if Utc::now() >= until {
                self.state = SlimeState::Idle;
                return Some(self.name().to_string());
            }
        }
        None
    }
}

impl std::fmt::Display for SlimeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlimeState::Idle => write!(f, "Idle"),
            SlimeState::Deployed(id) => write!(f, "Deployed (Mission {})", &id.to_string()[..8]),
            SlimeState::Injured(until) => {
                let remaining = (*until - Utc::now()).num_seconds().max(0);
                write!(f, "Injured (recovers in {}s)", remaining)
            }
            SlimeState::Training(_) => write!(f, "Training"),
        }
    }
}

/// Derived from stats XP and level (Sprint 9 §4.3).
pub fn stat_growth_factor(stat_xp: u32, level: u8) -> f32 {
    let xp_ceiling = level as u32 * 50;
    if xp_ceiling == 0 { return 1.0; }
    let ratio = stat_xp as f32 / xp_ceiling as f32;
    f32::max(0.8, f32::min(1.5, 0.8 + (ratio * 0.7)))
}

/// Final computed stat logic (Sprint 9 §4.4).
pub fn compute_final_stat(base: u32, xp: u32, level: u8) -> u32 {
    let stage_mod = crate::genetics::LifeStage::from_level(level).stat_multiplier();
    let growth = stat_growth_factor(xp, level);
    
    (base as f32 * stage_mod * growth).max(1.0) as u32
}
