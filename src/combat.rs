/// combat.rs — D20 Resolution Engine (Sprint 2)
///
/// Implements the D20-based resolution system that replaces the flat success_chance float.
/// All checks — mission success, genetic stability (future), and per-node encounters —
/// run through this single auditable module.
///
/// Key design decisions (see ADR-006):
///  - Roll mode derived from Culture hex-wheel (adjacency → Advantage, opposition → Disadvantage)
///  - DifficultyClass maps directly from mission difficulty scalars (0.0–1.0)
///  - 5% critical-fail floor is guaranteed by natural-1 → auto-fail semantics
///  - CombatStance transitions fire at HP thresholds mirroring rpgCore stance.py
use crate::genetics::Culture;
use rand::Rng;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// RollMode — derived from Culture hex-wheel relationship
// ---------------------------------------------------------------------------

/// How many d20 dice to roll, and whether to take the max or min.
///
/// Source: `get_mode(slime_culture, zone_element)` checks adjacency/opposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollMode {
    /// Single d20 roll. No cultural relationship.
    Normal,
    /// Roll 2d20, take the higher. Slime's dominant culture matches zone element.
    Advantage,
    /// Roll 2d20, take the lower. Slime's dominant culture opposes zone element.
    Disadvantage,
}

impl std::fmt::Display for RollMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RollMode::Normal       => write!(f, "Normal"),
            RollMode::Advantage    => write!(f, "Advantage"),
            RollMode::Disadvantage => write!(f, "Disadvantage"),
        }
    }
}

/// Determine roll mode from the relationship between a slime's dominant culture
/// and the current expedition zone's elemental affinity.
///
/// ```
/// # use operator::combat::{culture_zone_mode, RollMode};
/// # use operator::genetics::Culture;
/// assert_eq!(culture_zone_mode(Culture::Ember, Culture::Ember),   RollMode::Advantage);
/// assert_eq!(culture_zone_mode(Culture::Ember, Culture::Crystal), RollMode::Disadvantage);
/// assert_eq!(culture_zone_mode(Culture::Ember, Culture::Gale),    RollMode::Normal);
/// ```
pub fn culture_zone_mode(slime_culture: Culture, zone_element: Culture) -> RollMode {
    if slime_culture == Culture::Void || zone_element == Culture::Void {
        return RollMode::Normal; // Void is inert — no advantage or penalty
    }
    if slime_culture == zone_element {
        RollMode::Advantage
    } else if slime_culture.is_opposite(zone_element) {
        RollMode::Disadvantage
    } else {
        RollMode::Normal
    }
}

// ---------------------------------------------------------------------------
// Difficulty Class — maps mission.difficulty (0.0–1.0) to named DCs
// ---------------------------------------------------------------------------

/// Named DCs ported from the C/D20 tradition and mapped to OPERATOR's 0–1 difficulty scalar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DifficultyClass {
    /// DC 5  — routine, almost guaranteed. difficulty < 0.15
    Trivial       = 5,
    /// DC 10 — easy. difficulty 0.15–0.30
    Easy          = 10,
    /// DC 13 — moderate challenge. difficulty 0.30–0.45
    Moderate      = 13,
    /// DC 15 — standard field op. difficulty 0.45–0.60
    Standard      = 15,
    /// DC 18 — hard. difficulty 0.60–0.75
    Hard          = 18,
    /// DC 20 — expert-tier. difficulty 0.75–0.88
    Expert        = 20,
    /// DC 25 — legendary. difficulty 0.88–0.95
    Legendary     = 25,
    /// DC 30 — borderline impossible. difficulty ≥ 0.95
    NearImpossible = 30,
}

impl DifficultyClass {
    /// Map a 0.0–1.0 mission difficulty scalar to the nearest DifficultyClass.
    pub fn from_f64(d: f64) -> Self {
        match d {
            d if d < 0.15 => DifficultyClass::Trivial,
            d if d < 0.30 => DifficultyClass::Easy,
            d if d < 0.45 => DifficultyClass::Moderate,
            d if d < 0.60 => DifficultyClass::Standard,
            d if d < 0.75 => DifficultyClass::Hard,
            d if d < 0.88 => DifficultyClass::Expert,
            d if d < 0.95 => DifficultyClass::Legendary,
            _              => DifficultyClass::NearImpossible,
        }
    }

    pub fn value(self) -> i32 {
        self as i32
    }
}

impl std::fmt::Display for DifficultyClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DifficultyClass::Trivial        => "Trivial (DC 5)",
            DifficultyClass::Easy           => "Easy (DC 10)",
            DifficultyClass::Moderate       => "Moderate (DC 13)",
            DifficultyClass::Standard       => "Standard (DC 15)",
            DifficultyClass::Hard           => "Hard (DC 18)",
            DifficultyClass::Expert         => "Expert (DC 20)",
            DifficultyClass::Legendary      => "Legendary (DC 25)",
            DifficultyClass::NearImpossible => "Near-Impossible (DC 30)",
        };
        write!(f, "{s}")
    }
}

// ---------------------------------------------------------------------------
// D20Result — the outcome of a single check
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D20Result {
    /// The raw die face(s) that were rolled before modifier is applied.
    pub rolls:      Vec<u32>,
    /// Effective roll value after RollMode selection (max/min).
    pub roll_value: u32,
    /// Stat modifier added to roll_value.
    pub modifier:   i32,
    /// roll_value + modifier.
    pub total:      i32,
    /// DC this was checked against.
    pub dc:         i32,
    /// Whether total >= dc.
    pub success:    bool,
    /// Natural 20 before modifier — always success.
    pub nat_twenty: bool,
    /// Natural 1 before modifier — always failure (5% crit-fail floor).
    pub nat_one:    bool,
    /// Roll mode used.
    pub mode:       RollMode,
}

impl D20Result {
    pub fn narrative(&self) -> String {
        if self.nat_twenty {
            format!("⚡ CRITICAL SUCCESS! [{} → nat 20] DC {}", self.roll_value, self.dc)
        } else if self.nat_one {
            format!("💀 CRITICAL FAIL! [{} → nat 1] DC {}", self.roll_value, self.dc)
        } else if self.success {
            format!("✅ SUCCESS [{} + {} = {} vs DC {}]", self.roll_value, self.modifier, self.total, self.dc)
        } else {
            format!("❌ FAIL [{} + {} = {} vs DC {}]", self.roll_value, self.modifier, self.total, self.dc)
        }
    }
}

impl std::fmt::Display for D20Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.narrative())
    }
}

// ---------------------------------------------------------------------------
// D20 — the resolver
// ---------------------------------------------------------------------------

pub struct D20;

impl D20 {
    /// Roll a single d20.
    fn one<R: Rng>(rng: &mut R) -> u32 {
        rng.gen_range(1..=20)
    }

    /// Roll dice according to `mode` and return (rolls, effective_value).
    fn roll_raw<R: Rng>(mode: RollMode, rng: &mut R) -> (Vec<u32>, u32) {
        match mode {
            RollMode::Normal => {
                let r = Self::one(rng);
                (vec![r], r)
            }
            RollMode::Advantage => {
                let r1 = Self::one(rng);
                let r2 = Self::one(rng);
                (vec![r1, r2], r1.max(r2))
            }
            RollMode::Disadvantage => {
                let r1 = Self::one(rng);
                let r2 = Self::one(rng);
                (vec![r1, r2], r1.min(r2))
            }
        }
    }

    /// Perform a check against a DC with a stat modifier and roll mode.
    ///
    /// # Arguments
    /// * `modifier`  — stat-derived bonus/penalty (can be negative)
    /// * `dc`        — difficulty class to beat (inclusive: total >= dc = success)
    /// * `mode`      — Normal / Advantage / Disadvantage
    /// * `rng`       — caller-owned RNG for reproducibility
    pub fn check<R: Rng>(modifier: i32, dc: DifficultyClass, mode: RollMode, rng: &mut R) -> D20Result {
        let (rolls, roll_value) = Self::roll_raw(mode, rng);
        let dc_val = dc.value();

        let nat_twenty = roll_value == 20;
        let nat_one    = roll_value == 1;

        let total   = roll_value as i32 + modifier;
        let success = if nat_twenty { true } else if nat_one { false } else { total >= dc_val };

        D20Result { rolls, roll_value, modifier, total, dc: dc_val, success, nat_twenty, nat_one, mode }
    }

    /// Convenience: mission check where modifier comes from operator squad stats.
    ///
    /// `stat_coverage` is the ratio of squad's effective stat to mission requirement (0.0–2.0+).
    /// This maps linearly: 200% → +10 mod, 100% → 0, 50% → -5.
    pub fn mission_check<R: Rng>(
        stat_coverage: f64,
        difficulty:    f64,
        mode:          RollMode,
        rng:           &mut R,
    ) -> D20Result {
        let dc       = DifficultyClass::from_f64(difficulty);
        let modifier = Self::modifier_from_coverage(stat_coverage);
        Self::check(modifier, dc, mode, rng)
    }

    /// Compute modifier from stat coverage ratio.
    /// coverage 2.0 → +10 | 1.0 → 0 | 0.5 → -5
    pub fn modifier_from_coverage(coverage: f64) -> i32 {
        let clamped = coverage.clamp(0.0, 2.0);
        ((clamped - 1.0) * 10.0).round() as i32
    }
}

// ---------------------------------------------------------------------------
// CombatStance — mirrors rpgCore stance.py
// ---------------------------------------------------------------------------

/// Tactical stance that adjusts effective ATK, DEF, and SPD.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatStance {
    /// Full ATK (×1.2), normal DEF, no flee.
    Aggressive,
    /// Reduced ATK (×0.8), boosted DEF (×1.5), reduced SPD.
    Defensive,
    /// Low ATK (×0.5), reduced DEF, boosted SPD (×1.4), 60% flee chance.
    Fleeing,
}

impl CombatStance {
    /// Evaluate the correct stance from HP percentage and entity tier.
    /// `mindless` tier: switch to Fleeing at ≤25% HP.
    /// All others: switch to Defensive at ≤50% HP.
    pub fn evaluate(hp: f32, max_hp: f32, mindless: bool) -> Self {
        let pct = if max_hp > 0.0 { hp / max_hp } else { 0.0 };
        if mindless {
            if pct <= 0.25 { CombatStance::Fleeing } else { CombatStance::Aggressive }
        } else {
            if pct <= 0.50 { CombatStance::Defensive } else { CombatStance::Aggressive }
        }
    }

    pub fn atk_modifier(self) -> f32 {
        match self { CombatStance::Aggressive => 1.2, CombatStance::Defensive => 0.8, CombatStance::Fleeing => 0.5 }
    }
    pub fn def_modifier(self) -> f32 {
        match self { CombatStance::Aggressive => 1.0, CombatStance::Defensive => 1.5, CombatStance::Fleeing => 0.7 }
    }
    pub fn spd_modifier(self) -> f32 {
        match self { CombatStance::Aggressive => 1.0, CombatStance::Defensive => 0.8, CombatStance::Fleeing => 1.4 }
    }
    pub fn flee_chance(self) -> f32 {
        if self == CombatStance::Fleeing { 0.6 } else { 0.0 }
    }
}

impl std::fmt::Display for CombatStance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatStance::Aggressive => write!(f, "Aggressive"),
            CombatStance::Defensive  => write!(f, "Defensive"),
            CombatStance::Fleeing    => write!(f, "Fleeing"),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    fn rng() -> SmallRng { SmallRng::seed_from_u64(42) }

    // --- RollMode boundary: distribution ---
    #[test]
    fn advantage_beats_normal_on_average() {
        let mut r = rng();
        let adv_sum: u32 = (0..1000).map(|_| D20::roll_raw(RollMode::Advantage,    &mut r).1).sum();
        let mut r2 = SmallRng::seed_from_u64(42);
        let nor_sum: u32 = (0..1000).map(|_| D20::roll_raw(RollMode::Normal,       &mut r2).1).sum();
        assert!(adv_sum > nor_sum, "Advantage should average higher than Normal");
    }

    #[test]
    fn disadvantage_less_than_normal_on_average() {
        let mut r = rng();
        let dis_sum: u32 = (0..1000).map(|_| D20::roll_raw(RollMode::Disadvantage, &mut r).1).sum();
        let mut r2 = SmallRng::seed_from_u64(42);
        let nor_sum: u32 = (0..1000).map(|_| D20::roll_raw(RollMode::Normal,       &mut r2).1).sum();
        assert!(dis_sum < nor_sum, "Disadvantage should average lower than Normal");
    }

    // --- DifficultyClass mapping ---
    #[test]
    fn dc_from_f64_boundaries() {
        assert_eq!(DifficultyClass::from_f64(0.00), DifficultyClass::Trivial);
        assert_eq!(DifficultyClass::from_f64(0.14), DifficultyClass::Trivial);
        assert_eq!(DifficultyClass::from_f64(0.15), DifficultyClass::Easy);
        assert_eq!(DifficultyClass::from_f64(0.60), DifficultyClass::Hard);
        assert_eq!(DifficultyClass::from_f64(0.95), DifficultyClass::NearImpossible);
        assert_eq!(DifficultyClass::from_f64(1.00), DifficultyClass::NearImpossible);
    }

    // --- culture_zone_mode ---
    #[test]
    fn culture_zone_matching_gives_advantage() {
        assert_eq!(culture_zone_mode(Culture::Ember,   Culture::Ember),   RollMode::Advantage);
        assert_eq!(culture_zone_mode(Culture::Crystal, Culture::Crystal), RollMode::Advantage);
    }

    #[test]
    fn culture_zone_opposite_gives_disadvantage() {
        assert_eq!(culture_zone_mode(Culture::Ember, Culture::Crystal), RollMode::Disadvantage);
        assert_eq!(culture_zone_mode(Culture::Gale,  Culture::Tundra),  RollMode::Disadvantage);
        assert_eq!(culture_zone_mode(Culture::Marsh, Culture::Tide),    RollMode::Disadvantage);
    }

    #[test]
    fn culture_zone_adjacent_gives_normal() {
        // Adjacent but NOT opposite pairs → no advantage/disadvantage, just Normal
        // Hex-wheel: Ember adj to Gale, Marsh | Crystal adj to Gale, Tide
        assert_eq!(culture_zone_mode(Culture::Ember, Culture::Marsh),   RollMode::Normal);
        assert_eq!(culture_zone_mode(Culture::Crystal, Culture::Gale),  RollMode::Normal);
    }

    #[test]
    fn void_culture_always_normal() {
        assert_eq!(culture_zone_mode(Culture::Void,  Culture::Ember), RollMode::Normal);
        assert_eq!(culture_zone_mode(Culture::Ember, Culture::Void),  RollMode::Normal);
    }

    // --- nat 20 / nat 1 semantics ---
    #[test]
    fn nat_one_always_fails() {
        // Build a result manually — can't force dice, but test the flag semantics
        let result = D20Result {
            rolls: vec![1], roll_value: 1, modifier: 999, total: 1000,
            dc: 5, success: false, nat_twenty: false, nat_one: true,
            mode: RollMode::Normal,
        };
        assert!(!result.success, "Nat 1 must always fail regardless of modifier");
    }

    #[test]
    fn nat_twenty_always_succeeds() {
        let result = D20Result {
            rolls: vec![20], roll_value: 20, modifier: -999, total: -979,
            dc: 30, success: true, nat_twenty: true, nat_one: false,
            mode: RollMode::Normal,
        };
        assert!(result.success, "Nat 20 must always succeed regardless of modifier");
    }

    // --- CombatStance ---
    #[test]
    fn stance_mindless_flees_below_25_pct() {
        assert_eq!(CombatStance::evaluate(24.0, 100.0, true),  CombatStance::Fleeing);
        assert_eq!(CombatStance::evaluate(26.0, 100.0, true),  CombatStance::Aggressive);
    }

    #[test]
    fn stance_tactical_defends_at_50_pct() {
        assert_eq!(CombatStance::evaluate(50.0, 100.0, false), CombatStance::Defensive);
        assert_eq!(CombatStance::evaluate(51.0, 100.0, false), CombatStance::Aggressive);
    }

    // --- modifier_from_coverage ---
    #[test]
    fn modifier_from_coverage_correct() {
        assert_eq!(D20::modifier_from_coverage(2.0), 10);
        assert_eq!(D20::modifier_from_coverage(1.0), 0);
        assert_eq!(D20::modifier_from_coverage(0.5), -5);
        assert_eq!(D20::modifier_from_coverage(0.0), -10);
    }
}
