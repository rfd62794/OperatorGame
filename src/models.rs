use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Job — Operator specialisation with a flat stat bonus
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Job {
    Breacher,    // +Strength
    Infiltrator, // +Agility
    Analyst,     // +Intelligence
}

impl Job {
    /// Returns (str_bonus, agi_bonus, int_bonus)
    pub fn stat_bonus(self) -> (u32, u32, u32) {
        match self {
            Job::Breacher => (10, 0, 0),
            Job::Infiltrator => (0, 10, 0),
            Job::Analyst => (0, 0, 10),
        }
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Job::Breacher => write!(f, "Breacher"),
            Job::Infiltrator => write!(f, "Infiltrator"),
            Job::Analyst => write!(f, "Analyst"),
        }
    }
}

// ---------------------------------------------------------------------------
// OperatorState — tracks lifecycle: Idle → Deployed → Injured → Idle
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OperatorState {
    Idle,
    Deployed(Uuid),          // UUID of active Mission
    Injured(DateTime<Utc>),  // Timestamp when recovery completes
}

impl std::fmt::Display for OperatorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorState::Idle => write!(f, "Idle"),
            OperatorState::Deployed(id) => write!(f, "Deployed (Mission {})", &id.to_string()[..8]),
            OperatorState::Injured(until) => {
                let remaining = (*until - Utc::now()).num_seconds().max(0);
                write!(f, "Injured (recovers in {}s)", remaining)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Operator
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Operator {
    pub id: Uuid,
    pub name: String,
    pub job: Job,
    pub base_strength: u32,
    pub base_agility: u32,
    pub base_intelligence: u32,
    pub state: OperatorState,
}

impl Operator {
    /// Construct a new Operator ready to hire.
    pub fn new(name: impl Into<String>, job: Job, str: u32, agi: u32, int: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            job,
            base_strength: str,
            base_agility: agi,
            base_intelligence: int,
            state: OperatorState::Idle,
        }
    }

    /// Effective stats after applying the Job specialisation bonus.
    pub fn effective_stats(&self) -> (u32, u32, u32) {
        let (bs, ba, bi) = self.job.stat_bonus();
        (
            self.base_strength + bs,
            self.base_agility + ba,
            self.base_intelligence + bi,
        )
    }

    /// An operator is deployable only when fully Idle.
    pub fn is_available(&self) -> bool {
        matches!(self.state, OperatorState::Idle)
    }

    /// Tick: clear Injured state if recovery timestamp has passed.
    pub fn tick_recovery(&mut self) {
        if let OperatorState::Injured(until) = self.state {
            if Utc::now() >= until {
                self.state = OperatorState::Idle;
            }
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (s, a, i) = self.effective_stats();
        write!(
            f,
            "[{}] {} | {} | STR:{} AGI:{} INT:{} | {}",
            &self.id.to_string()[..8],
            self.name,
            self.job,
            s,
            a,
            i,
            self.state,
        )
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
    pub fn calculate_success_rate(&self, squad: &[&Operator]) -> f64 {
        let (mut total_str, mut total_agi, mut total_int) = (0u32, 0u32, 0u32);

        for op in squad {
            let (s, a, i) = op.effective_stats();
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

#[derive(Debug, Clone, PartialEq)]
pub enum AarOutcome {
    Victory { reward: u64 },
    Failure { injured_ids: Vec<Uuid> },
    CriticalFailure { killed_id: Uuid },
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
    /// This consumes a random roll and applies the mission formula.
    /// Call only after `is_complete()` returns `true`.
    pub fn resolve<R: Rng>(
        &self,
        mission: &Mission,
        squad: &[&Operator],
        rng: &mut R,
    ) -> AarOutcome {
        let success_rate = mission.calculate_success_rate(squad);
        let roll: f64 = rng.gen();

        if roll < success_rate {
            AarOutcome::Victory {
                reward: mission.reward,
            }
        } else if roll >= 0.95 {
            // Critical failure — pick a random casualty
            let idx = rng.gen_range(0..self.operator_ids.len());
            AarOutcome::CriticalFailure {
                killed_id: self.operator_ids[idx],
            }
        } else {
            // Standard failure — all deployed operators are injured
            AarOutcome::Failure {
                injured_ids: self.operator_ids.clone(),
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
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    fn make_op(job: Job, s: u32, a: u32, i: u32) -> Operator {
        Operator::new("Test Op", job, s, a, i)
    }

    fn make_mission(rs: u32, ra: u32, ri: u32, diff: f64) -> Mission {
        Mission::new("Test Mission", rs, ra, ri, diff, 60, 100)
    }

    #[test]
    fn test_success_perfect_squad_no_difficulty() {
        let op = make_op(Job::Breacher, 100, 100, 100);
        let m = make_mission(100, 100, 100, 0.0);
        let rate = m.calculate_success_rate(&[&op]);
        assert!((rate - 1.0).abs() < 1e-9, "Expected ~1.0, got {rate}");
    }

    #[test]
    fn test_success_difficulty_applied() {
        let op = make_op(Job::Analyst, 100, 100, 100);
        let m = make_mission(100, 100, 100, 0.5);
        let rate = m.calculate_success_rate(&[&op]);
        assert!((rate - 0.5).abs() < 1e-9, "Expected ~0.5, got {rate}");
    }

    #[test]
    fn test_success_capped_at_one() {
        // Squad stats far exceed requirements
        let op = make_op(Job::Breacher, 200, 200, 200);
        let m = make_mission(50, 50, 50, 0.0);
        let rate = m.calculate_success_rate(&[&op]);
        assert!(rate <= 1.0, "Rate must not exceed 1.0, got {rate}");
    }

    #[test]
    fn test_success_zero_str_coverage_punishes() {
        // Squad has no Intelligence — Analyst mission — expect low rate
        let op = make_op(Job::Breacher, 100, 100, 0);
        let m = make_mission(10, 10, 80, 0.0);
        let rate = m.calculate_success_rate(&[&op]);
        // int_score = 0/80 = 0 → average ≤ 0.667
        assert!(rate < 0.7, "Expected below 0.7 with zero INT, got {rate}");
    }

    #[test]
    fn test_job_bonus_applied() {
        let op = make_op(Job::Analyst, 30, 30, 30);
        let (s, a, i) = op.effective_stats();
        assert_eq!(s, 30, "Strength should be un-bonused");
        assert_eq!(a, 30, "Agility should be un-bonused");
        assert_eq!(i, 40, "Intelligence should be boosted by 10");
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
    fn test_resolve_victory_with_seeded_rng() {
        // Seed RNG to produce a known low roll → Victory
        let mut rng = SmallRng::seed_from_u64(42);
        let op = make_op(Job::Analyst, 100, 100, 100);
        let m = make_mission(50, 50, 50, 0.0); // 100% success rate
        let mut d = Deployment::start(&m, vec![op.id]);
        d.completes_at = Utc::now() - Duration::seconds(1);

        let outcome = d.resolve(&m, &[&op], &mut rng);
        assert!(
            matches!(outcome, AarOutcome::Victory { .. }),
            "Expected Victory with guaranteed success rate"
        );
    }
}
