/// dice.rs — Die Animation State Machine (Sprint 5)
///
/// Port of rpgCore `demos/dice_roller_v2.py` — pure math, zero render dependency.
/// The egui/WASM render layer reads `DieAnimState` fields each frame and draws
/// the polygon + pips accordingly. This module owns only the physics.
///
/// All timings match the Python blueprint exactly (see docs/DESIGN_BLUEPRINT.md §1).
use rand::Rng;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Animation constants — from dice_roller_v2.py, verbatim
// ---------------------------------------------------------------------------

pub const FAST_DURATION:  f32 = 0.55;
pub const DECEL_DURATION: f32 = 0.30;
pub const LAND_DURATION:  f32 = 0.28;
pub const SETTLE_GLOW:    f32 = 1.4;
pub const STUTTER_FRAMES: usize = 4;
pub const STUTTER_HOLD:   f32 = DECEL_DURATION / STUTTER_FRAMES as f32; // 0.075s
pub const SHAKE_MAX:      f32 = 7.0;

// ---------------------------------------------------------------------------
// DiePhase — the 5-state roll lifecycle
// ---------------------------------------------------------------------------

/// Maps 1:1 to the state machine in dice_roller_v2.py.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DiePhase {
    /// Waiting to be rolled — no animation.
    #[default]
    Idle,
    /// Fast spin + face blur phase (0.55s).
    Fast,
    /// 4-frame stutter-settle phase (0.30s, ~75ms per frame).
    Decel,
    /// Squash/bounce landing phase (0.28s).
    Landing,
    /// Glow-decay after landing (1.4s).
    Settled,
}

// ---------------------------------------------------------------------------
// DieSides — the canonical 7 die types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DieSides {
    D4   = 4,
    D6   = 6,
    D8   = 8,
    D10  = 10,
    D12  = 12,
    D20  = 20,
    D100 = 100,
}

impl DieSides {
    pub fn faces(self) -> u32 { self as u32 }

    /// Body + glow RGB palette from DESIGN_BLUEPRINT §1.
    pub fn colors(self) -> DieColors {
        match self {
            DieSides::D4   => DieColors { body: [170, 55,  18 ], edge: [255, 125, 55 ], pip: [255, 215, 170], glow: [255, 140, 60 ] },
            DieSides::D6   => DieColors { body: [40,  120, 195], edge: [90,  170, 255], pip: [215, 238, 255], glow: [100, 180, 255] },
            DieSides::D8   => DieColors { body: [35,  150, 72 ], edge: [72,  215, 110], pip: [195, 255, 215], glow: [80,  220, 120] },
            DieSides::D10  => DieColors { body: [120, 45,  175], edge: [185, 95,  255], pip: [238, 195, 255], glow: [190, 100, 255] },
            DieSides::D12  => DieColors { body: [155, 135, 18 ], edge: [228, 208, 55 ], pip: [255, 248, 195], glow: [235, 215, 60 ] },
            DieSides::D20  => DieColors { body: [55,  55,  175], edge: [115, 115, 255], pip: [215, 215, 255], glow: [120, 120, 255] },
            DieSides::D100 => DieColors { body: [155, 28,  75 ], edge: [228, 75,  138], pip: [255, 195, 218], glow: [235, 80,  145] },
        }
    }
}

/// All color layers for a single die type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DieColors {
    pub body: [u8; 3],
    pub edge: [u8; 3],
    pub pip:  [u8; 3],
    pub glow: [u8; 3],
}

// ---------------------------------------------------------------------------
// DieAnimState — the full frame-state for one die
// ---------------------------------------------------------------------------

/// Complete per-frame animation state for a single die.
/// The render layer reads these fields; this module writes them via `tick()`.
///
/// Serializable so an in-progress D20 roll inside a Dungeon node can be
/// saved to `GameState` and resumed after the app restarts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DieAnimState {
    pub sides:         DieSides,
    pub phase:         DiePhase,

    // Result tracking
    pub result:        u32,        // true roll — not revealed until Landing
    pub display:       u32,        // currently shown face
    stutter_seq:       Vec<u32>,   // decoy values + result at end
    stutter_idx:       usize,
    stutter_timer:     f32,

    // Physics
    pub timer:         f32,
    pub spin:          f32,        // degrees
    pub spin_speed:    f32,        // degrees/sec
    pub squash:        f32,        // y scale factor (< 1 = flattened)
    pub stretch:       f32,        // x scale factor (> 1 = widened)
    pub glow:          f32,        // 0.0–1.0 glow ring intensity
    pub shake_x:       f32,        // px offset
    pub shake_y:       f32,        // px offset
    pub y_offset:      f32,        // drop/rise offset for landing arc
}

impl DieAnimState {
    pub fn new(sides: DieSides) -> Self {
        Self {
            sides,
            phase:         DiePhase::Idle,
            result:        1,
            display:       1,
            stutter_seq:   Vec::new(),
            stutter_idx:   0,
            stutter_timer: 0.0,
            timer:         0.0,
            spin:          0.0,
            spin_speed:    0.0,
            squash:        1.0,
            stretch:       1.0,
            glow:          0.0,
            shake_x:       0.0,
            shake_y:       0.0,
            y_offset:      0.0,
        }
    }

    /// True if the die is mid-animation (not Idle).
    pub fn is_rolling(&self) -> bool { self.phase != DiePhase::Idle }

    /// True when result is a natural max (CRIT) or natural 1 (FUMBLE) — only valid
    /// during Settled/Landing phases.
    pub fn is_crit(&self)   -> bool { self.display == self.sides.faces() && matches!(self.phase, DiePhase::Settled | DiePhase::Landing) }
    pub fn is_fumble(&self) -> bool { self.display == 1                  && matches!(self.phase, DiePhase::Settled | DiePhase::Landing) }

    // -----------------------------------------------------------------------
    // Roll
    // -----------------------------------------------------------------------

    /// Trigger a new roll. Picks a result and starts the Fast phase.
    pub fn roll<R: Rng>(&mut self, rng: &mut R) {
        let faces          = self.sides.faces();
        self.result        = rng.gen_range(1..=faces);
        self.stutter_seq   = Self::make_stutter(self.result, faces, rng);
        self.stutter_idx   = 0;
        self.stutter_timer = 0.0;
        self.display       = rng.gen_range(1..=faces);
        self.phase         = DiePhase::Fast;
        self.timer         = 0.0;
        self.spin          = rng.gen_range(0.0f32..360.0);
        self.spin_speed    = rng.gen_range(420.0f32..740.0) * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        self.glow          = 0.0;
        self.squash        = 1.0;
        self.stretch       = 1.0;
    }

    /// Build a stutter sequence: 3 unique decoy values + result at the end.
    fn make_stutter<R: Rng>(result: u32, faces: u32, rng: &mut R) -> Vec<u32> {
        let mut seq  = Vec::with_capacity(STUTTER_FRAMES);
        let mut used = std::collections::HashSet::new();
        used.insert(result);
        for _ in 0..(STUTTER_FRAMES - 1) {
            let mut v = rng.gen_range(1..=faces);
            for _ in 0..10 { if !used.contains(&v) { break; } v = rng.gen_range(1..=faces); }
            used.insert(v);
            seq.push(v);
        }
        seq.push(result); // final frame always resolves to truth
        seq
    }

    // -----------------------------------------------------------------------
    // Tick — advance by dt seconds
    // -----------------------------------------------------------------------

    /// Advance the animation by `dt` seconds. Returns `true` if the phase
    /// changed this frame (useful for triggering sound or VFX callbacks).
    pub fn tick<R: Rng>(&mut self, dt: f32, rng: &mut R) -> bool {
        match self.phase {
            DiePhase::Idle    => false,
            DiePhase::Fast    => self.tick_fast(dt, rng),
            DiePhase::Decel   => self.tick_decel(dt, rng),
            DiePhase::Landing => self.tick_landing(dt),
            DiePhase::Settled => self.tick_settled(dt),
        }
    }

    fn tick_fast<R: Rng>(&mut self, dt: f32, rng: &mut R) -> bool {
        self.timer += dt;
        let t = self.timer / FAST_DURATION;

        // Blur: random face at 50% probability each tick
        if rng.gen_bool(0.5) {
            self.display = rng.gen_range(1..=self.sides.faces());
        }

        // Fast spin
        self.spin       += self.spin_speed * dt;
        self.spin_speed *= (1.0 - dt * 1.2).max(0.0);

        // Shake decreases as we approach decel
        let intensity = (1.0 - t).max(0.0) * SHAKE_MAX;
        if intensity > 0.0 {
            self.shake_x = rng.gen_range(-intensity..intensity);
            self.shake_y = rng.gen_range(-intensity..intensity);
        } else {
            self.shake_x = 0.0;
            self.shake_y = 0.0;
        }

        // Wobble squash
        self.squash  = 1.0 + (self.timer * 22.0).sin() * 0.07 * (1.0 - t);
        self.stretch = 1.0 / self.squash.max(0.82);

        if self.timer >= FAST_DURATION {
            self.phase         = DiePhase::Decel;
            self.timer         = 0.0;
            self.stutter_idx   = 0;
            self.stutter_timer = 0.0;
            self.shake_x       = 0.0;
            self.shake_y       = 0.0;
            self.squash        = 1.0;
            self.stretch       = 1.0;
            self.display       = self.stutter_seq.first().copied().unwrap_or(self.result);
            return true;
        }
        false
    }

    fn tick_decel<R: Rng>(&mut self, dt: f32, _rng: &mut R) -> bool {
        self.timer        += dt;
        self.stutter_timer += dt;

        // Advance stutter frame
        if self.stutter_timer >= STUTTER_HOLD {
            self.stutter_timer -= STUTTER_HOLD;
            let next = (self.stutter_idx + 1).min(self.stutter_seq.len().saturating_sub(1));
            self.stutter_idx = next;
            self.display = self.stutter_seq[next];
        }

        // Slow residual spin
        self.spin       += self.spin_speed * dt;
        self.spin_speed *= (1.0 - dt * 4.0).max(0.0);

        // Gentle wobble
        let t = self.timer / DECEL_DURATION;
        self.squash  = 1.0 + (self.timer * 10.0).sin() * 0.03 * (1.0 - t);
        self.stretch = 1.0 / self.squash.max(0.95);

        if self.timer >= DECEL_DURATION {
            self.phase      = DiePhase::Landing;
            self.timer      = 0.0;
            self.display    = self.result;
            self.spin       = 0.0;
            self.spin_speed = 0.0;
            return true;
        }
        false
    }

    fn tick_landing(&mut self, dt: f32) -> bool {
        self.timer += dt;
        let t = self.timer / LAND_DURATION;

        // Impact compression → ease_out_bounce recovery
        if t < 0.28 {
            let st = t / 0.28;
            self.squash  = 1.0 - st * 0.32;
            self.stretch = 1.0 + st * 0.22;
        } else {
            let bt = (t - 0.28) / 0.72;
            self.squash  = 0.68 + ease_out_bounce(bt) * 0.32;
            self.stretch = 1.0  + (1.0 - ease_out_bounce(bt)) * 0.22;
        }

        self.y_offset = (1.0 - (t * 2.2).min(1.0)) * 10.0;
        self.glow     = (t * 2.5).min(1.0);

        if self.timer >= LAND_DURATION {
            self.phase    = DiePhase::Settled;
            self.timer    = 0.0;
            self.squash   = 1.0;
            self.stretch  = 1.0;
            self.y_offset = 0.0;
            self.glow     = 1.0;
            return true;
        }
        false
    }

    fn tick_settled(&mut self, dt: f32) -> bool {
        self.timer += dt;
        self.glow   = (1.0 - self.timer / SETTLE_GLOW).max(0.0);
        if self.timer > SETTLE_GLOW * 1.6 {
            self.phase = DiePhase::Idle;
            return true;
        }
        false
    }
}

// ---------------------------------------------------------------------------
// ease_out_bounce — verbatim from dice_roller_v2.py
// ---------------------------------------------------------------------------

pub fn ease_out_bounce(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984375
    }
}

// ---------------------------------------------------------------------------
// DiceEngine — manages a tray of dice for a single encounter node
// ---------------------------------------------------------------------------

/// Manages a set of dice mid-roll. An encounter node can own one of these
/// to run a D20 check with full animation feedback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceEngine {
    pub dice: Vec<DieAnimState>,
}

impl DiceEngine {
    pub fn new(sides: &[DieSides]) -> Self {
        Self { dice: sides.iter().map(|&s| DieAnimState::new(s)).collect() }
    }

    /// Roll all dice.
    pub fn roll_all<R: Rng>(&mut self, rng: &mut R) {
        for d in &mut self.dice { d.roll(rng); }
    }

    /// Advance all dice. Returns true if any dice changed phase.
    pub fn tick<R: Rng>(&mut self, dt: f32, rng: &mut R) -> bool {
        self.dice.iter_mut().fold(false, |changed, d| changed | d.tick(dt, rng))
    }

    /// True once all dice are in Idle (fully settled).
    pub fn is_resolved(&self) -> bool {
        self.dice.iter().all(|d| d.phase == DiePhase::Idle)
    }

    /// Sum of all final results.
    pub fn total(&self) -> u32 {
        self.dice.iter().map(|d| d.result).sum()
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

    fn rng() -> SmallRng { SmallRng::seed_from_u64(101) }

    #[test]
    fn roll_starts_fast_phase() {
        let mut die = DieAnimState::new(DieSides::D20);
        die.roll(&mut rng());
        assert_eq!(die.phase, DiePhase::Fast);
    }

    #[test]
    fn result_in_valid_range() {
        let mut r = rng();
        for _ in 0..100 {
            let mut die = DieAnimState::new(DieSides::D20);
            die.roll(&mut r);
            assert!(die.result >= 1 && die.result <= 20);
        }
    }

    #[test]
    fn stutter_seq_ends_with_result() {
        let mut die = DieAnimState::new(DieSides::D20);
        die.roll(&mut rng());
        assert_eq!(*die.stutter_seq.last().unwrap(), die.result);
    }

    #[test]
    fn stutter_seq_length_is_stutter_frames() {
        let mut die = DieAnimState::new(DieSides::D20);
        die.roll(&mut rng());
        assert_eq!(die.stutter_seq.len(), STUTTER_FRAMES);
    }

    #[test]
    fn full_animation_cycle_reaches_idle() {
        let mut die = DieAnimState::new(DieSides::D6);
        let mut r   = rng();
        die.roll(&mut r);
        // Total cycle time: 0.55 + 0.30 + 0.28 + 1.4×1.6 = 3.37s → 500 steps at 0.01s = 5s
        for _ in 0..500 { die.tick(0.01, &mut r); }
        assert_eq!(die.phase, DiePhase::Idle, "Die must return to Idle after full cycle");
    }

    #[test]
    fn display_equals_result_after_landing() {
        let mut die = DieAnimState::new(DieSides::D12);
        let mut r   = rng();
        die.roll(&mut r);
        // Fast-forward past Fast+Decel+Landing into Settled
        // FAST=0.55 + DECEL=0.30 + LAND=0.28 = 1.13s → 120 steps at 0.01s
        for _ in 0..200 {
            die.tick(0.01, &mut r);
            if die.phase == DiePhase::Settled { break; }
        }
        assert_eq!(die.display, die.result, "Settled display must show actual result");
    }

    #[test]
    fn ease_bounce_starts_near_zero_ends_near_one() {
        assert!(ease_out_bounce(0.0) < 0.01);
        assert!((ease_out_bounce(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn dice_engine_total_is_sum_of_results() {
        let mut eng = DiceEngine::new(&[DieSides::D6, DieSides::D6]);
        let mut r   = rng();
        eng.roll_all(&mut r);
        let total = eng.total();
        let manual: u32 = eng.dice.iter().map(|d| d.result).sum();
        assert_eq!(total, manual);
    }

    #[test]
    fn crit_flag_set_on_max_value() {
        let mut die = DieAnimState::new(DieSides::D20);
        // let mut r   = rng();
        die.result  = 20;
        die.display = 20;
        die.phase   = DiePhase::Settled;
        assert!(die.is_crit(), "Max result in Settled phase must be CRIT");
    }

    #[test]
    fn fumble_flag_set_on_one() {
        let mut die = DieAnimState::new(DieSides::D20);
        die.result  = 1;
        die.display = 1;
        die.phase   = DiePhase::Settled;
        assert!(die.is_fumble());
    }
}
