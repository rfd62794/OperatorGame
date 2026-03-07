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
    Victory {
        reward: u64,
        rolls: Vec<D20Result>,
    },
    Failure {
        injured_ids: Vec<Uuid>,
        rolls: Vec<D20Result>,
    },
    CriticalFailure {
        injured_ids: Vec<Uuid>,
        rolls: Vec<D20Result>,
    },
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
            AarOutcome::Victory {
                reward: mission.reward,
                rolls,
            }
        } else if any_crit_fail && successes == 0 {
            AarOutcome::CriticalFailure {
                injured_ids: Vec::new(), // Populated by apply_outcome_injuries
                rolls,
            }
        } else {
            AarOutcome::Failure {
                injured_ids: Vec::new(), // Populated by apply_outcome_injuries
                rolls,
            }
        }
    }
}

/// Phase A — Data Layer (Sprint 7A)
/// Applies injuries to the roster based on the deployment outcome.
/// - Critical Failure: 1-2 operators (capped at squad size), 4-8 hours recovery.
/// - Failure: 10% chance for 1 operator, 2-4 hours recovery.
/// - Returns the IDs of operators who were newly injured for logging.
pub fn apply_outcome_injuries(
    outcome: &mut AarOutcome,
    roster: &mut [crate::genetics::SlimeGenome],
    squad_ids: &[Uuid],
    rng: &mut impl Rng,
) -> Vec<Uuid> {
    let mut injured = Vec::new();

    match outcome {
        AarOutcome::Victory { .. } => {}
        AarOutcome::CriticalFailure { injured_ids, .. } => {
            // min(rng.gen_range(1..=2), squad_size)
            let mut pool = squad_ids.to_vec();
            use rand::seq::SliceRandom;
            pool.shuffle(rng);

            let count = if pool.len() >= 2 {
                rng.gen_range(1..=2)
            } else {
                1
            };

            let hours = rng.gen_range(4..=8);
            let until = Utc::now() + Duration::hours(hours);

            for id in pool.into_iter().take(count) {
                if let Some(op) = roster.iter_mut().find(|s| s.id == id) {
                    op.state = SlimeState::Injured(until);
                    injured.push(id);
                }
            }
            *injured_ids = injured.clone();
        }
        AarOutcome::Failure { injured_ids, .. } => {
            if rng.gen_bool(0.1) && !squad_ids.is_empty() {
                let id = squad_ids[rng.gen_range(0..squad_ids.len())];
                let hours = rng.gen_range(2..=4);
                let until = Utc::now() + Duration::hours(hours);

                if let Some(op) = roster.iter_mut().find(|s| s.id == id) {
                    op.state = SlimeState::Injured(until);
                    injured.push(id);
                }
            }
            *injured_ids = injured.clone();
        }
    }

    injured
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
// Expedition — slime dispatch to the 19-node planet map (Sprint 3)
// ---------------------------------------------------------------------------

/// An active or resolved slime expedition to an island target.
///
/// Follows the ADR-002 wall-clock timer pattern identical to `Deployment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expedition {
    pub id:          Uuid,
    pub slime_ids:   Vec<Uuid>,
    pub target:      crate::world_map::ExpeditionTarget,
    pub departed_at: DateTime<Utc>,
    /// Wall-clock time when the expedition returns (departed_at + distance*2).
    pub returns_at:  DateTime<Utc>,
    pub resolved:    bool,
}

impl Expedition {
    /// Launch an expedition. Round-trip duration = `distance_secs * 2`.
    pub fn launch(slimes: Vec<Uuid>, target: crate::world_map::ExpeditionTarget) -> Self {
        let now      = Utc::now();
        let duration = chrono::Duration::seconds((target.distance_secs * 2) as i64);
        Self {
            id:          Uuid::new_v4(),
            slime_ids:   slimes,
            target,
            departed_at: now,
            returns_at:  now + duration,
            resolved:    false,
        }
    }

    /// True once the wall-clock return time has passed.
    pub fn is_complete(&self) -> bool {
        Utc::now() >= self.returns_at
    }

    /// Resolve the expedition outcome using a D20 check on average AGI.
    ///
    /// Uses `D20::mission_check()` — same pattern as `Deployment::resolve()`.
    ///
    /// # Sprint 4 hook
    /// When a slime's `dominant_culture() == target.culture`, pass
    /// `culture_zone_mode(slime_culture, target.culture)` instead of
    /// `RollMode::Normal` to grant Advantage to matched dispatches.
    /// This is the core gameplay hook: Tide slime → Tide Basin = Advantage.
    pub fn resolve<R: rand::Rng>(
        &self,
        squad:  &[&crate::genetics::SlimeGenome],
        rng:    &mut R,
    ) -> ExpeditionOutcome {
        // Average AGI coverage ratio as the primary stat for field movement.
        let avg_agi: u32 = if squad.is_empty() {
            0
        } else {
            squad.iter().map(|s| s.base_agility).sum::<u32>() / squad.len() as u32
        };

        // Treat AGI as "coverage" against a notional requirement of 20 base points.
        // TODO Sprint 4: replace 20 with target zone's base AGI requirement.
        let coverage = (avg_agi as f64 / 20.0).min(1.0);

        // TODO Sprint 4: replace Normal with culture_zone_mode(dominant, target.culture)
        let roll = D20::mission_check(coverage, self.target.danger_level, RollMode::Normal, rng);

        let report = self.generate_report(roll.nat_twenty);

        if roll.nat_twenty {
            ExpeditionOutcome::BonusHaul {
                yield_:  self.target.resource_yield.scaled(1.5),
                roll,
                report,
            }
        } else if roll.success {
            ExpeditionOutcome::Success {
                yield_:  self.target.resource_yield.clone(),
                roll,
                report,
            }
        } else if roll.nat_one {
            // Critical fail — random slime takes the hit
            let victim = self.slime_ids[rng.gen_range(0..self.slime_ids.len().max(1))];
            ExpeditionOutcome::SlimeInjured {
                slime_id:      victim,
                partial_yield: self.target.resource_yield.scaled(0.25),
                roll,
                report,
            }
        } else {
            ExpeditionOutcome::Failure { roll, report }
        }
    }

    /// Per-culture flavor narrative — Sprint 3 static templates.
    /// Sprint 5+: generative via log_engine expansion.
    fn generate_report(&self, exceptional: bool) -> String {
        use crate::genetics::Culture;
        let flavor = match self.target.culture {
            Culture::Ember   => if exceptional { "uncovered a smouldering cache no one was meant to find" }
                                else           { "navigated the thermal vents and returned with singed margins" },
            Culture::Tide    => if exceptional { "caught the basin at low ebb and found what the water hides" }
                                else           { "read the tide schedule correctly and returned on schedule" },
            Culture::Orange  => if exceptional { "decoded the amber lattice and returned with warm-harvest surplus" }
                                else           { "traced the amber paths and came back heavy with yield" },
            Culture::Marsh   => if exceptional { "found a submerged cache beneath the root network" }
                                else           { "waded through the delta and surfaced intact" },
            Culture::Teal    => if exceptional { "located a precision cache at depth-zero and extracted cleanly" }
                                else           { "crossed the teal shelf with surgical accuracy" },
            Culture::Crystal => if exceptional { "resonated with the spire lattice and extracted pure nodes" }
                                else           { "navigated the refraction corridors without incident" },
            Culture::Gale    => if exceptional { "rode the updrafts to a ridge no survey had mapped" }
                                else           { "outpaced the storm front and made it back before it turned" },
            Culture::Tundra  => if exceptional { "broke into a preserved vault beneath the permafrost" }
                                else           { "crossed the shelf in the cold window and came back clean" },
            Culture::Frost   => if exceptional { "found the still-point beneath the ice and listened" }
                                else           { "moved through the frost zone without disturbing the ancient layer" },
            Culture::Void    => "returned. No further details.",
        };
        format!("The team reached {} and {}.", self.target.name, flavor)
    }
}

/// Outcome of a resolved `Expedition`.
#[derive(Debug, Clone)]
pub enum ExpeditionOutcome {
    /// Nat-20 — 1.5× resource yield.
    BonusHaul    { yield_: crate::world_map::ResourceYield, roll: D20Result, report: String },
    /// Clean success — full yield.
    Success      { yield_: crate::world_map::ResourceYield, roll: D20Result, report: String },
    /// Nat-1 — one slime injured, 0.25× partial yield.
    SlimeInjured { slime_id: Uuid, partial_yield: crate::world_map::ResourceYield, roll: D20Result, report: String },
    /// Failure — no yield.
    Failure      {                                           roll: D20Result, report: String },
}



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

    // ------------------------------------------------------------------
    // Expedition tests (Phase D, Sprint 3)
    // ------------------------------------------------------------------

    fn marsh_target() -> crate::world_map::ExpeditionTarget {
        crate::world_map::seed_expedition_targets()
            .into_iter()
            .find(|t| t.name == "Marsh Delta")
            .expect("Marsh Delta must be in seed targets")
    }

    #[test]
    fn test_expedition_launch_sets_returns_at() {
        let target = marsh_target(); // distance_secs = 90, round-trip = 180
        let exp    = Expedition::launch(vec![], target);
        let elapsed = (exp.returns_at - exp.departed_at).num_seconds();
        assert_eq!(elapsed, 180, "returns_at must be exactly 2×distance_secs from departed_at");
    }

    #[test]
    fn test_expedition_complete_future() {
        let target = marsh_target();
        let exp    = Expedition::launch(vec![], target);
        assert!(!exp.is_complete(), "Freshly launched expedition should not be complete");
    }

    #[test]
    fn test_expedition_complete_past() {
        let target = marsh_target();
        let mut exp = Expedition::launch(vec![], target);
        // Backdate so it's already due
        exp.returns_at = Utc::now() - Duration::seconds(1);
        assert!(exp.is_complete(), "Expedition past returns_at should be complete");
    }

    #[test]
    fn test_expedition_resolve_success_yields_resources() {
        use crate::genetics::{generate_random, Culture};
        let mut rng   = SmallRng::seed_from_u64(42);
        let slime     = generate_random(Culture::Marsh, "Boggy", &mut rng);
        let target    = marsh_target();
        let exp       = Expedition::launch(vec![slime.id], target.clone());
        let mut found_yield = false;
        // Run several seeds to guarantee a Success or BonusHaul path is reachable
        for seed in 0u64..20 {
            let mut rng2 = SmallRng::seed_from_u64(seed);
            let outcome  = exp.resolve(&[&slime], &mut rng2);
            match outcome {
                ExpeditionOutcome::Success    { .. } |
                ExpeditionOutcome::BonusHaul  { .. } => {
                    found_yield = true;
                    break;
                }
                _ => {}
            }
        }
        assert!(found_yield, "resolve() should produce a Success/BonusHaul on at least one seed");
    }

    #[test]
    fn test_expedition_resolve_crit_injures_slime() {
        use crate::genetics::{generate_random, Culture};
        let mut rng  = SmallRng::seed_from_u64(1);
        let slime    = generate_random(Culture::Tundra, "IceBoy", &mut rng);
        // Use a very high danger target to maximise crit-fail probability
        let target   = crate::world_map::ExpeditionTarget {
            id:             uuid::Uuid::from_u128(0xFF),
            name:           "Death Zone".into(),
            culture:        Culture::Tundra,
            distance_secs:  60,
            danger_level:   1.0, // guaranteed nat-1 fails at DC Moderate+
            resource_yield: crate::world_map::ResourceYield { biomass: 0, scrap: 0, reagents: 0 },
        };
        let exp = Expedition::launch(vec![slime.id], target);
        let mut found_injury = false;
        for seed in 0u64..50 {
            let mut rng2 = SmallRng::seed_from_u64(seed);
            let outcome  = exp.resolve(&[&slime], &mut rng2);
            if let ExpeditionOutcome::SlimeInjured { slime_id, .. } = outcome {
                assert_eq!(slime_id, slime.id, "Injured slime ID must match dispatched slime");
                found_injury = true;
                break;
            }
        }
        assert!(found_injury, "SlimeInjured must be reachable on a danger_level=1.0 target");
    }

    #[test]
    fn test_resource_yield_scaled_halved() {
        let base = crate::world_map::ResourceYield { biomass: 20, scrap: 10, reagents: 4 };
        let half = base.scaled(0.5);
        assert_eq!(half.biomass,  10, "biomass should halve");
        assert_eq!(half.scrap,     5, "scrap should halve");
        assert_eq!(half.reagents,  2, "reagents should halve");
    }

    #[test]
    fn test_cargo_apply_increments_inventory() {
        let yield_ = crate::world_map::ResourceYield { biomass: 10, scrap: 5, reagents: 3 };
        let mut inv = crate::inventory::Inventory::default();
        yield_.apply_to_inventory(&mut inv);
        assert_eq!(inv.biomass,   10);
        assert_eq!(inv.scrap,      5);
        assert_eq!(inv.reagents,   3);
        // Apply again — verify accumulation
        yield_.apply_to_inventory(&mut inv);
        assert_eq!(inv.biomass,   20);
        assert_eq!(inv.scrap,     10);
        assert_eq!(inv.reagents,   6);
    }
}
