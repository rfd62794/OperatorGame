use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::combat::{D20Result, D20, RollMode};

// ---------------------------------------------------------------------------
// Gear — Industrial Grade Tools
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Gear {
    HeavyVest, // +5 STR
    ScoutFins, // +5 AGI
    DataLens,  // +5 INT
}

impl Gear {
    pub fn name(&self) -> &'static str {
        match self {
            Gear::HeavyVest => "Heavy Vest",
            Gear::ScoutFins => "Scout Fins",
            Gear::DataLens => "Data Lens",
        }
    }

    pub fn stat_bonus(&self) -> (u32, u32, u32) {
        match self {
            Gear::HeavyVest => (5, 0, 0),
            Gear::ScoutFins => (0, 5, 0),
            Gear::DataLens => (0, 0, 5),
        }
    }
}

// ---------------------------------------------------------------------------
// SlimeState — tracks biological lifecycle
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SlimeState {
    Idle,
    Deployed(Uuid),          // UUID of active Mission
    Injured(DateTime<Utc>),  // Timestamp when recovery completes
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
        }
    }
}

// ---------------------------------------------------------------------------
// Mission
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub req_strength: u32,
    pub req_agility: u32,
    pub req_intelligence: u32,
    /// Scalar difficulty penalty: 0.0 (trivial) → 0.9 (near-impossible).
    /// Clamped to [0.0, 0.9] at creation.
    pub difficulty: f64,
    /// Wall-clock seconds this mission takes to complete.
    pub duration_secs: u64,
    pub reward: u64,
}

impl Mission {
    /// Build a mission, clamping difficulty to the safe range.
    pub fn new(
        name: impl Into<String>,
        req_str: u32,
        req_agi: u32,
        req_int: u32,
        difficulty: f64,
        duration_secs: u64,
        reward: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            req_strength: req_str,
            req_agility: req_agi,
            req_intelligence: req_int,
            difficulty: difficulty.clamp(0.0, 0.9),
            duration_secs,
            reward,
        }
    }

    /// Core "Mafia Wars" formula.
    ///
    /// Each attribute is scored independently (capped at 1.0), averaged,
    /// then penalised by difficulty. Returns a value in [0.0, 1.0].
    ///
    /// **Missing stat coverage is punishing**: a squad of three Breachers
    /// on a high-INT mission will score 0.0 on intelligence — the average
    /// pulls the result down hard. This is intentional game design friction.
    pub fn calculate_success_rate(&self, squad: &[&crate::genetics::SlimeGenome]) -> f64 {
        let (mut total_str, mut total_agi, mut total_int) = (0u32, 0u32, 0u32);

        for op in squad {
            let (s, a, i, _, _, _) = op.total_stats();
            total_str += s;
            total_agi += a;
            total_int += i;
        }

        // Guard: missions with zero threshold in an attribute are treated as
        // trivially met (score = 1.0) so they don't divide by zero.
        let score = |total: u32, req: u32| -> f64 {
            if req == 0 {
                1.0
            } else {
                (total as f64 / req as f64).min(1.0)
            }
        };

        let avg = (score(total_str, self.req_strength)
            + score(total_agi, self.req_agility)
            + score(total_int, self.req_intelligence))
            / 3.0;

        avg * (1.0 - self.difficulty)
    }
}

impl std::fmt::Display for Mission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} | STR:{} AGI:{} INT:{} | Diff:{:.0}% | Dur:{}s | Reward:${}",
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

// ---------------------------------------------------------------------------
// AAR (After Action Report) outcomes
// ---------------------------------------------------------------------------

/// After-Action Report outcome — produced by `Deployment::resolve()`.
///
/// Each variant carries the three per-stat D20 rolls (STR / AGI / INT) so the
/// narrative log can display individual check results.
#[derive(Debug, Clone)]
pub enum AarOutcome {
    Victory        { reward: u64,            rolls: Vec<D20Result> },
    Failure        { injured_ids: Vec<Uuid>, rolls: Vec<D20Result> },
    CriticalFailure { killed_id: Uuid,       rolls: Vec<D20Result> },
}

// ---------------------------------------------------------------------------
// Deployment — the "offline-safe" timer record
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deployment {
    pub id: Uuid,
    pub mission_id: Uuid,
    pub operator_ids: Vec<Uuid>,
    /// Absolute UTC wall-clock timestamp. Surviving app restarts. ✅
    pub completes_at: DateTime<Utc>,
    pub resolved: bool,
}

impl Deployment {
    /// Create a new deployment starting right now.
    pub fn start(mission: &Mission, operator_ids: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            mission_id: mission.id,
            operator_ids,
            completes_at: Utc::now() + Duration::seconds(mission.duration_secs as i64),
            resolved: false,
        }
    }

    /// The central "secret sauce": compare wall-clock, not a running timer.
    pub fn is_complete(&self) -> bool {
        Utc::now() >= self.completes_at
    }

    /// Resolve a completed deployment into an AAR outcome.
    ///
    /// Runs three independent D20 checks (STR / AGI / INT). The squad's
    /// aggregate stat coverage ratio determines the modifier for each check.
    /// Outcome: 2-of-3 successes → Victory; crit-fail on all failing checks → CriticalFailure;
    /// otherwise → Failure. Call only after `is_complete()` returns `true`.
    pub fn resolve<R: Rng>(
        &self,
        mission: &Mission,
        squad: &[&crate::genetics::SlimeGenome],
        rng: &mut R,
    ) -> AarOutcome {
        // --- Aggregate squad stats -------------------------------------------
        let (mut total_str, mut total_agi, mut total_int) = (0u32, 0u32, 0u32);
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

        // --- Three per-stat D20 checks ----------------------------------------
        // RollMode::Normal — culture synergy wired in Sprint 4 post 9-culture migration
        let str_roll = D20::mission_check(str_cov, mission.difficulty, RollMode::Normal, rng);
        let agi_roll = D20::mission_check(agi_cov, mission.difficulty, RollMode::Normal, rng);
        let int_roll = D20::mission_check(int_cov, mission.difficulty, RollMode::Normal, rng);

        let successes = [str_roll.success, agi_roll.success, int_roll.success]
            .iter().filter(|&&s| s).count();

        // Crit-fail: nat-1 on a check that itself failed
        let any_crit_fail = (str_roll.nat_one && !str_roll.success)
            || (agi_roll.nat_one && !agi_roll.success)
            || (int_roll.nat_one && !int_roll.success);

        let rolls = vec![str_roll, agi_roll, int_roll];

        if successes >= 2 {
            AarOutcome::Victory { reward: mission.reward, rolls }
        } else if any_crit_fail && successes == 0 {
            let idx = rng.gen_range(0..self.operator_ids.len().max(1));
            AarOutcome::CriticalFailure {
                killed_id: self.operator_ids[idx],
                rolls,
            }
        } else {
            AarOutcome::Failure {
                injured_ids: self.operator_ids.clone(),
                rolls,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Seed data — static mission pool for MVP
// ---------------------------------------------------------------------------

pub fn seed_missions() -> Vec<Mission> {
    vec![
        Mission::new("Bank Heist Recon",    20, 30, 10, 0.10, 60,  500),
        Mission::new("Corporate Espionage", 10, 20, 50, 0.25, 120, 1200),
        Mission::new("Harbour Extraction",  40, 20, 10, 0.20, 90,  800),
        Mission::new("Zero-Day Exploit",    10, 10, 70, 0.40, 180, 2500),
        Mission::new("Black Site Breach",   60, 40, 20, 0.50, 300, 5000),
    ]
}

// ---------------------------------------------------------------------------
// Unit tests removed to accommodate Operator refactor into SlimeGenome
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    fn make_mission(rs: u32, ra: u32, ri: u32, diff: f64) -> Mission {
        Mission::new("Test Mission", rs, ra, ri, diff, 60, 100)
    }

    #[test]
    fn test_deployment_is_complete_past() {
        let mut d = Deployment::start(&make_mission(10, 10, 10, 0.0), vec![]);
        // Force completion into the past
        d.completes_at = Utc::now() - Duration::seconds(1);
        assert!(d.is_complete(), "Should be complete when timestamp is past");
    }

    #[test]
    fn test_deployment_is_complete_future() {
        let m = Mission::new("Far Future", 10, 10, 10, 0.0, 9999, 0);
        let d = Deployment::start(&m, vec![]);
        assert!(!d.is_complete(), "Should not be complete for future timestamp");
    }

    #[test]
    fn test_resolve_d20_victory_with_slime() {
        use crate::genetics::generate_random;
        use crate::genetics::Culture;
        // A slime with ample stats vs a low-req mission at trivial difficulty
        // should win 2-of-3 D20 checks with high probability (seeded for determinism).
        let mut rng = SmallRng::seed_from_u64(99);
        let slime = generate_random(Culture::Ember, "TestSlime", &mut rng);
        // req = 1 so coverage ≈ slime's total stat → +modifier well above Trivial DC 5
        let m = make_mission(1, 1, 1, 0.0);
        let mut d = Deployment::start(&m, vec![slime.id]);
        d.completes_at = Utc::now() - Duration::seconds(1);

        let mut rng2 = SmallRng::seed_from_u64(99);
        let outcome = d.resolve(&m, &[&slime], &mut rng2);
        // With DC 5 and a very high coverage modifier, should always succeed
        assert!(
            matches!(outcome, AarOutcome::Victory { .. }),
            "Expected Victory on trivial mission with capable slime"
        );
    }

    #[test]
    fn test_resolve_rolls_has_three_entries() {
        use crate::genetics::generate_random;
        use crate::genetics::Culture;
        let mut rng = SmallRng::seed_from_u64(7);
        let slime = generate_random(Culture::Gale, "RollsTest", &mut rng);
        let m = make_mission(50, 50, 50, 0.5);
        let mut d = Deployment::start(&m, vec![slime.id]);
        d.completes_at = Utc::now() - Duration::seconds(1);

        let mut rng2 = SmallRng::seed_from_u64(7);
        let outcome = d.resolve(&m, &[&slime], &mut rng2);
        let rolls = match outcome {
            AarOutcome::Victory        { rolls, .. } => rolls,
            AarOutcome::Failure        { rolls, .. } => rolls,
            AarOutcome::CriticalFailure { rolls, .. } => rolls,
        };
        assert_eq!(rolls.len(), 3, "resolve() must produce exactly 3 D20 rolls (STR/AGI/INT)");
    }
}
