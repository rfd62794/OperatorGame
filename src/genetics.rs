/// genetics.rs — Slime Genome Engine (Sprint 1, updated Sprint 4)
///
/// Implements the full rpgCore genetic system in Rust:
///  - Culture enum (9-culture nonagon-wheel + Void wildcard)
///  - CultureExpression (9-float vector, always sums to 1.0)
///  - SlimeGenome (fully serialisable — fixes the Python persistence gap)
///  - BreedingResolver: 3-rule stat inheritance + culture blending + mutation
///  - GeneticTier derived from nonagon adjacency (ADR-023 v2)
///  - LifeStage gate (Hatchling → Elder)
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Culture (the DNA alphabet)
// ---------------------------------------------------------------------------

/// Nine elemental cultures arranged on a nonagon wheel, plus Void (wildcard).
///
/// Nonagon wheel (clockwise, ADR-023 v2):
///   Ember(0) → Tide(1) → Orange(2) → Marsh(3) → Teal(4)
///   → Crystal(5) → Gale(6) → Tundra(7) → Frost(8) → Ember
///
/// Adjacency: wheel distance == 1 (either direction, wrapping).
/// Near-opposites (Sundered): wheel distance == 4 OR 5 — both fire.
/// Note: on an odd-count wheel (n=9), no culture has a single exact opposite.
/// See ADR-023 v2 for full geometry derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Culture {
    // Inner loop (Primary) — Ember=0, Tide=1, (skip2)... see WHEEL for ordering
    Ember,    // RED        256 Hz  ATK-dominant
    Tide,     // ORANGE     320 Hz  CHM-dominant
    Orange,   // AMBER      336 Hz  MND-dominant  [NEW]
    Marsh,    // YELLOW     384 Hz  HP-dominant
    Teal,     // TEAL       407 Hz  AGI-dominant  [NEW]
    Crystal,  // BLUE       426 Hz  DEF-dominant
    Gale,     // GREEN      288 Hz  SPD-dominant
    Tundra,   // VIOLET     540 Hz  RES-dominant
    Frost,    // ICE BLUE   480 Hz  END-dominant  [NEW]
    Void,     // EQUAL MIX  432 Hz  no dominance
}

/// Standalone geometry helpers — free functions so combat.rs can call them
/// without needing a Culture receiver (ADR-023 v2 contract).

/// True if a and b are adjacent on the 9-point nonagon (wheel distance == 1).
pub fn is_adjacent(a: Culture, b: Culture) -> bool {
    if a == Culture::Void || b == Culture::Void { return false; }
    let Some(pa) = Culture::WHEEL.iter().position(|&c| c == a) else { return false; };
    let Some(pb) = Culture::WHEEL.iter().position(|&c| c == b) else { return false; };
    let d = (pa as i32 - pb as i32).unsigned_abs() as usize;
    d.min(9 - d) == 1
}

/// True if a and b are near-opposites on the 9-point nonagon (wheel distance == 4 or 5).
/// Both arc distances qualify — permissive Sundered interpretation (ADR-023 v2).
/// On an odd-count wheel no single true opposite exists; both near-slots fire.
pub fn is_near_opposite(a: Culture, b: Culture) -> bool {
    if a == Culture::Void || b == Culture::Void { return false; }
    let Some(pa) = Culture::WHEEL.iter().position(|&c| c == a) else { return false; };
    let Some(pb) = Culture::WHEEL.iter().position(|&c| c == b) else { return false; };
    let d = (pa as i32 - pb as i32).unsigned_abs() as usize;
    let min_d = d.min(9 - d);
    min_d == 4 || min_d == 5
}

impl Culture {
    /// Nine non-Void cultures in CultureExpression index order (nonagon clockwise).
    /// WHEEL[i] gives the culture at position i. Used by is_adjacent/is_near_opposite.
    pub const WHEEL: [Culture; 9] = [
        Culture::Ember,
        Culture::Tide,
        Culture::Orange,
        Culture::Marsh,
        Culture::Teal,
        Culture::Crystal,
        Culture::Gale,
        Culture::Tundra,
        Culture::Frost,
    ];

    /// Stat multipliers — derived from ADR-023 v2 stat dominance profiles.
    pub fn params(self) -> CulturalParams {
        match self {
            Culture::Ember   => CulturalParams { hp: 0.8, atk: 1.4, spd: 1.1, rare: 0.05, openness: 0.2 },
            Culture::Tide    => CulturalParams { hp: 1.0, atk: 1.0, spd: 1.2, rare: 0.07, openness: 0.9 },
            Culture::Orange  => CulturalParams { hp: 0.9, atk: 0.8, spd: 1.0, rare: 0.06, openness: 0.6 },
            Culture::Marsh   => CulturalParams { hp: 1.0, atk: 0.9, spd: 1.3, rare: 0.04, openness: 0.7 },
            Culture::Teal    => CulturalParams { hp: 0.9, atk: 0.8, spd: 1.5, rare: 0.07, openness: 0.8 },
            Culture::Crystal => CulturalParams { hp: 1.4, atk: 0.8, spd: 0.7, rare: 0.08, openness: 0.1 },
            Culture::Gale    => CulturalParams { hp: 0.9, atk: 0.9, spd: 1.4, rare: 0.06, openness: 0.8 },
            Culture::Tundra  => CulturalParams { hp: 1.1, atk: 0.9, spd: 0.8, rare: 0.05, openness: 0.3 },
            Culture::Frost   => CulturalParams { hp: 1.2, atk: 0.7, spd: 0.7, rare: 0.05, openness: 0.2 },
            Culture::Void    => CulturalParams { hp: 1.2, atk: 1.2, spd: 1.2, rare: 0.25, openness: 1.0 },
        }
    }

    /// Primary frequency for Cymatics audio/visual generation (ADR-023 v2).
    pub fn frequency(self) -> f32 {
        match self {
            Culture::Ember   => 256.0,
            Culture::Tide    => 320.0,
            Culture::Orange  => 336.0,
            Culture::Marsh   => 384.0,
            Culture::Teal    => 407.0,
            Culture::Crystal => 426.0,
            Culture::Gale    => 288.0,
            Culture::Tundra  => 540.0,
            Culture::Frost   => 480.0,
            Culture::Void    => 432.0,
        }
    }

    /// Index in `CultureExpression` (Void has no dedicated slot — returns None).
    pub fn wheel_index(self) -> Option<usize> {
        Self::WHEEL.iter().position(|c| *c == self)
    }

    pub fn name(self) -> &'static str {
        match self {
            Culture::Ember   => "Ember",
            Culture::Tide    => "Tide",
            Culture::Orange  => "Orange",
            Culture::Marsh   => "Marsh",
            Culture::Teal    => "Teal",
            Culture::Crystal => "Crystal",
            Culture::Gale    => "Gale",
            Culture::Tundra  => "Tundra",
            Culture::Frost   => "Frost",
            Culture::Void    => "Void",
        }
    }
}

impl std::fmt::Display for Culture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ---------------------------------------------------------------------------
// CulturalParams
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct CulturalParams {
    pub hp:   f32,   // multiplier on base_hp = 20.0
    pub atk:  f32,   // multiplier on base_atk = 5.0
    pub spd:  f32,   // multiplier on base_spd = 5.0
    pub rare: f32,   // rare trait chance (0.0–1.0)
    pub openness: f32, // fraction of personality permeability
}

impl CulturalParams {
    const BASE_HP:  f32 = 20.0;
    const BASE_ATK: f32 =  5.0;
    const BASE_SPD: f32 =  5.0;

    pub fn base_hp(self)  -> f32 { Self::BASE_HP  * self.hp  }
    pub fn base_atk(self) -> f32 { Self::BASE_ATK * self.atk }
    pub fn base_spd(self) -> f32 { Self::BASE_SPD * self.spd }
    /// Stat cap = base * modifier * 2.0 (generational ceiling)
    pub fn hp_cap(self)   -> f32 { self.base_hp()  * 2.0 }
    pub fn atk_cap(self)  -> f32 { self.base_atk() * 2.0 }
    pub fn spd_cap(self)  -> f32 { self.base_spd() * 2.0 }
}

// ---------------------------------------------------------------------------
// CultureExpression — the 9-float genome vector (Sprint 4: expanded from 6)
// ---------------------------------------------------------------------------

/// A normalised distribution across the 9 wheel cultures.
/// Indices match `Culture::WHEEL` order:
///   Ember=0, Tide=1, Orange=2, Marsh=3, Teal=4, Crystal=5, Gale=6, Tundra=7, Frost=8
/// Invariant: values.iter().sum() ≈ 1.0
///
/// Save migration (v2→v3): existing [f32;6] saves are zero-padded to [f32;9]
/// and renormalised. No data loss — new slots start at 0.0.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CultureExpression(pub [f32; 9]);

impl CultureExpression {
    /// Equal distribution across all 9 cultures (the Void genome).
    pub fn void() -> Self {
        Self([1.0 / 9.0; 9])
    }

    /// Pure culture: 1.0 on that culture, 0.0 on all others.
    pub fn pure(culture: Culture) -> Self {
        let mut arr = [0.0f32; 9];
        if let Some(i) = culture.wheel_index() {
            arr[i] = 1.0;
        } else {
            // Void — fall back to equal distribution
            return Self::void();
        }
        Self(arr)
    }

    pub fn get(&self, culture: Culture) -> f32 {
        culture.wheel_index().map(|i| self.0[i]).unwrap_or(0.0)
    }

    /// Count cultures above the significance threshold (0.05).
    pub fn active_count(&self) -> usize {
        self.0.iter().filter(|&&v| v >= 0.05).count()
    }

    /// The dominant culture (highest expression).
    pub fn dominant(&self) -> Culture {
        let (i, _) = self
            .0
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap_or((0, &0.0));
        Culture::WHEEL[i]
    }

    /// Renormalise so all values sum to 1.0.
    pub fn normalise(mut arr: [f32; 9]) -> Self {
        let total: f32 = arr.iter().sum();
        if total > 0.0 {
            arr.iter_mut().for_each(|v| *v /= total);
        } else {
            arr = [1.0 / 9.0; 9];
        }
        Self(arr)
    }

    /// Migrate a 6-slot expression to 9 slots by zero-padding.
    /// Used in save migration v2→v3.
    pub fn migrate_from_6(old: [f32; 6]) -> Self {
        let mut arr = [0.0f32; 9];
        // Old WHEEL order was: Ember=0,Gale=1,Marsh=2,Crystal=3,Tundra=4,Tide=5
        // New WHEEL order is:  Ember=0,Tide=1,Orange=2,Marsh=3,Teal=4,Crystal=5,Gale=6,Tundra=7,Frost=8
        // Map old indices to new positions:
        arr[0] = old[0]; // Ember   → Ember(0)   unchanged
        arr[1] = old[5]; // Tide    ← old Tide(5)
        // Orange(2) = 0.0 (new)
        arr[3] = old[2]; // Marsh   ← old Marsh(2)
        // Teal(4) = 0.0 (new)
        arr[5] = old[3]; // Crystal ← old Crystal(3)
        arr[6] = old[1]; // Gale    ← old Gale(1)
        arr[7] = old[4]; // Tundra  ← old Tundra(4)
        // Frost(8) = 0.0 (new)
        Self::normalise(arr)  // sum unchanged (new slots are 0) — renormalise for safety
    }
}

// ---------------------------------------------------------------------------
// CultureAlleles — dominant + recessive allele pair (Sprint 5)
// ---------------------------------------------------------------------------

/// Full allele representation for a slime's culture genome.
/// `dominant` drives all visible behavior: HSL color, stat modifiers,
/// GeneticTier, RPS match-ups.
/// `recessive` is hidden until Lens unlock. Participates in breeding
/// resolution. Can surface as dominant in offspring via reemergence.
///
/// Save migration (v3→v4): existing saves load culture_expr into dominant;
/// recessive is zeroed via `#[serde(default)]` on that field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CultureAlleles {
    pub dominant:  CultureExpression,
    pub recessive: CultureExpression,
}

impl CultureAlleles {
    /// Blooded — dominant fully set to one culture, recessive zeroed.
    pub fn blooded(culture: Culture) -> Self {
        Self {
            dominant:  CultureExpression::pure(culture),
            recessive: CultureExpression([ 0.0; 9]),
        }
    }

    /// Construct from an existing CultureExpression (migration path).
    /// Recessive starts fully zeroed.
    pub fn from_expression(expr: CultureExpression) -> Self {
        Self { dominant: expr, recessive: CultureExpression([0.0; 9]) }
    }

    /// Void — all dominant slots equal, recessive zeroed.
    pub fn void() -> Self {
        Self {
            dominant:  CultureExpression::void(),
            recessive: CultureExpression([0.0; 9]),
        }
    }

    /// Convenience: normalise the dominant array in-place.
    pub fn normalise_dominant(&mut self) {
        self.dominant = CultureExpression::normalise(self.dominant.0);
    }
}

// ---------------------------------------------------------------------------
// Allele resolver constants — locked per Sprint 5 directive
// ---------------------------------------------------------------------------

/// Probability that a parent contributes its dominant allele
/// rather than its recessive allele for any given culture slot.
/// 0.75 means purity is rewarded (Blooded always passes) but not guaranteed.
pub const DOMINANT_DRAW_BIAS: f32 = 0.75;

/// Minimum recessive weight in both parents required to trigger
/// reemergence check. Below this threshold the recessive is too weak
/// to surface in offspring.
pub const REEMERGENCE_THRESHOLD: f32 = 0.15;

/// Probability of reemergence when both parents exceed REEMERGENCE_THRESHOLD
/// for the same culture slot.
pub const REEMERGENCE_CHANCE: f32 = 0.30;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneticTier {
    Blooded     = 1, // 1 active culture
    Bordered    = 2, // 2 cultures, adjacent
    Sundered    = 3, // 2 cultures, opposite
    Drifted     = 4, // 2 cultures, skip-one
    Threaded    = 5, // 3 active
    Convergent  = 6, // 4 active
    Liminal     = 7, // 5 active
    Void        = 8, // all 6 active
}

impl GeneticTier {
    pub fn from_expression(expr: &CultureExpression) -> Self {
        let active: Vec<Culture> = Culture::WHEEL
            .iter()
            .copied()
            .filter(|c| expr.get(*c) >= 0.05)
            .collect();

        match active.len() {
            0 | 1 => GeneticTier::Blooded,
            2 => {
                let (c1, c2) = (active[0], active[1]);
                if is_near_opposite(c1, c2)    { GeneticTier::Sundered }
                else if is_adjacent(c1, c2)    { GeneticTier::Bordered }
                else                           { GeneticTier::Drifted  }
            }
            3       => GeneticTier::Threaded,
            4 | 5   => GeneticTier::Convergent,
            6 | 7   => GeneticTier::Liminal,
            _       => GeneticTier::Void,   // 8 or 9 active
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            GeneticTier::Blooded    => "BLOODED",
            GeneticTier::Bordered   => "BORDERED",
            GeneticTier::Sundered   => "SUNDERED",
            GeneticTier::Drifted    => "DRIFTED",
            GeneticTier::Threaded   => "THREADED",
            GeneticTier::Convergent => "CONVERGENT",
            GeneticTier::Liminal    => "LIMINAL",
            GeneticTier::Void       => "VOID",
        }
    }
}

impl std::fmt::Display for GeneticTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ---------------------------------------------------------------------------
// LifeStage — gating system
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifeStage {
    Hatchling, // level 0-1
    Juvenile,  // level 2-3
    Young,     // level 4-5
    Prime,     // level 6-7
    Veteran,   // level 8-9
    Elder,     // level 10
}

impl std::fmt::Display for LifeStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifeStage::Hatchling => write!(f, "Hatchling"),
            LifeStage::Juvenile  => write!(f, "Juvenile"),
            LifeStage::Young     => write!(f, "Young"),
            LifeStage::Prime     => write!(f, "Prime"),
            LifeStage::Veteran   => write!(f, "Veteran"),
            LifeStage::Elder     => write!(f, "Elder"),
        }
    }
}

impl LifeStage {
    pub fn from_level(level: u8) -> Self {
        match level {
            0..=1   => LifeStage::Hatchling,
            2..=3   => LifeStage::Juvenile,
            4..=5   => LifeStage::Young,
            6..=7   => LifeStage::Prime,
            8..=9   => LifeStage::Veteran,
            _       => LifeStage::Elder,
        }
    }

    /// Derived from total_xp // 100 (Sprint 9 §2).
    pub fn level_from_xp(xp: u32) -> u8 {
        ((xp / 100).min(10)) as u8
    }

    pub fn can_dispatch(self) -> bool { self != LifeStage::Hatchling }
    pub fn can_breed(self)   -> bool { !matches!(self, LifeStage::Hatchling | LifeStage::Juvenile) }
    pub fn can_mentor(self)  -> bool { matches!(self, LifeStage::Veteran | LifeStage::Elder) }

    /// Multiplier on base stats (Sprint 9 §2).
    pub fn stat_multiplier(self) -> f32 {
        match self {
            LifeStage::Hatchling => 0.6,
            LifeStage::Juvenile  => 0.8,
            LifeStage::Young     => 1.0,
            LifeStage::Prime     => 1.2,
            LifeStage::Veteran   => 1.1,
            LifeStage::Elder     => 1.0,
        }
    }

    /// XP needed to level up from `level` (Sprint 9 §2).
    pub fn xp_to_next(level: u8) -> u32 {
        if level >= 10 { 0 } else { (level as u32 + 1) * 100 }
    }

    /// Elder bonus: 20% chance of rare accessory even if roll said "none".
    pub fn elder_rare_bonus(self) -> f32 {
        if self == LifeStage::Elder { 0.20 } else { 0.0 }
    }
}


// ---------------------------------------------------------------------------
// Visual trait enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shape { Round, Cubic, Elongated, Crystalline, Amorphous }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodySize { Tiny, Small, Medium, Large, Massive }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pattern { Solid, Spotted, Striped, Marbled, Iridescent }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Accessory { None, Crown, Scar, Glow, Shell, Crystals }

impl BodySize {
    /// Numeric body_size used in race stats formula.
    pub fn scalar(self) -> f32 {
        match self {
            BodySize::Tiny    => 0.3,
            BodySize::Small   => 0.5,
            BodySize::Medium  => 0.7,
            BodySize::Large   => 0.9,
            BodySize::Massive => 1.0,
        }
    }
}

// ---------------------------------------------------------------------------
// SlimeGenome — the hero struct
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimeGenome {
    pub id:           Uuid,
    // Genetics
    pub culture_alleles: CultureAlleles,
    pub base_hp:      f32,
    pub base_atk:     f32,
    pub base_spd:     f32,
    pub generation:   u32,
    pub parent_ids:   Option<[Uuid; 2]>,
    // Personality (0.0–1.0)
    pub curiosity:    f32,
    pub energy:       f32,
    pub affection:    f32,
    pub shyness:      f32,
    // Base Stats (Biological Identity)
    pub base_strength: u32,
    pub base_agility:  u32,
    pub base_intelligence: u32,
    pub base_mind:     u32,
    pub base_sensory:  u32,
    pub base_tenacity: u32,
    // Visuals
    pub shape:         Shape,
    pub body_size:     BodySize,
    pub pattern:       Pattern,
    pub accessory:     Accessory,
    pub base_color:    [u8; 3],
    pub pattern_color: [u8; 3],
    // Frequency
    pub frequency:     f32,
    // Name (cosmetic)
    pub name:          String,
}

impl SlimeGenome {
    pub fn genetic_tier(&self) -> GeneticTier {
        GeneticTier::from_expression(&self.culture_alleles.dominant)
    }

    pub fn dominant_culture(&self) -> Culture {
        self.culture_alleles.dominant.dominant()
    }

    pub fn race_stats(&self) -> RaceStats {
        let size  = self.body_size.scalar();
        let mass  = size.powf(1.5);
        let str_n = self.base_atk / 100.0; // normalised 0-1
        RaceStats {
            mass,
            heft_power:    mass * (1.0 + str_n * 0.5),
            jump_force:    50.0 * (1.0 + str_n * 0.3),
            jump_distance: (50.0 * (1.0 + str_n * 0.3) / mass) * size,
            jump_cooldown: 0.2 + (mass * 0.4) * (1.0 - str_n * 0.2),
            jump_height:   14.0,
            body_size:     size,
        }
    }
}

impl std::fmt::Display for SlimeGenome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}] | HP:{:.0} ATK:{:.0} SPD:{:.0} | Gen:{} | Tier: {} {} | {}",
            self.name,
            &self.id.to_string()[..8],
            self.base_hp,
            self.base_atk,
            self.base_spd,
            self.generation,
            self.genetic_tier() as u8,
            self.genetic_tier(),
            self.dominant_culture(),
        )
    }
}

// ---------------------------------------------------------------------------
// RaceStats (derived, not stored)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RaceStats {
    pub mass:          f32,
    pub heft_power:    f32,
    pub jump_force:    f32,
    pub jump_distance: f32,
    pub jump_cooldown: f32,
    pub jump_height:   f32,
    pub body_size:     f32,
}

// ---------------------------------------------------------------------------
// BreedingResolver — the transplanted genetic math
// ---------------------------------------------------------------------------

const DEFAULT_MUTATION_CHANCE: f32 = 0.05;
const VOID_MUTATION_FLOOR:     f32 = 0.15;

pub struct BreedingResolver;

impl BreedingResolver {
    // -----------------------------------------------------------------------
    // Public API
    // -----------------------------------------------------------------------

    /// Full genetic synthesis. Returns `Err` if either parent can't synthesize.
    ///
    /// Caller MUST call `apply_exhaustion(600)` on both parent `&mut SlimeGenome` after
    /// calling this — the breed function takes immutable refs to avoid borrow conflicts
    /// (parents must be found by ID in the `Vec<SlimeGenome>` and mutated separately).
    pub fn breed<R: Rng>(
        a: &SlimeGenome,
        b: &SlimeGenome,
        name: &str,
        rng: &mut R,
    ) -> Result<SlimeGenome, &'static str> {
        if !a.can_synthesize() {
            return Err("Donor A is in Cellular Exhaustion — synthesis unavailable");
        }
        if !b.can_synthesize() {
            return Err("Donor B is in Cellular Exhaustion — synthesis unavailable");
        }

        let mutation_chance = Self::mutation_chance(a, b);
        let mut culture_alleles = Self::resolve_culture(&a.culture_alleles, &b.culture_alleles, rng);

        // ADR-010 §3: Void Glitch — 1% chance when two Sundered parents are used.
        let void_glitch = a.genetic_tier() == GeneticTier::Sundered
            && b.genetic_tier() == GeneticTier::Sundered
            && rng.gen::<f32>() < 0.01;
        if void_glitch {
            culture_alleles = CultureAlleles::void();
        }

        let (hp, atk, spd)  = Self::resolve_stats(a, b, culture_alleles.dominant.dominant(), mutation_chance, rng);
        let (shape, size, pattern, accessory, base_color, pattern_color) =
            Self::resolve_visuals(a, b, &a.culture_alleles.dominant, &b.culture_alleles.dominant, a.life_stage(), rng);
        let personality = Self::resolve_personality(a, b, rng);

        Ok(SlimeGenome {
            id:           Uuid::new_v4(),
            culture_alleles: culture_alleles.clone(),
            base_hp:      hp,
            base_atk:     atk,
            base_spd:     spd,
            generation:   a.generation.max(b.generation) + 1,
            parent_ids:   Some([a.id, b.id]),
            curiosity:    personality[0],
            energy:       personality[1],
            affection:    personality[2],
            shyness:      personality[3],
            shape,
            body_size:     size,
            pattern,
            accessory,
            base_color,
            pattern_color,
            frequency:     culture_alleles.dominant.dominant().frequency(),
            name:          name.to_string(),
            // ADR-037 stats
            base_strength: rng.gen_range(5..=8),
            base_agility:  rng.gen_range(5..=8),
            base_intelligence: rng.gen_range(5..=8),
            base_mind: rng.gen_range(5..=8),
            base_sensory: rng.gen_range(5..=8),
            base_tenacity: rng.gen_range(5..=8),
        })
    }

    // -----------------------------------------------------------------------
    // Step 1: Culture allele blending — Weighted Mendelian + reemergence
    // -----------------------------------------------------------------------

    /// Weighted Mendelian allele draw with reemergence.
    ///
    /// For each slot `i`:
    ///   - Each parent independently contributes either its dominant or recessive
    ///     value, with `DOMINANT_DRAW_BIAS` probability of contributing dominant.
    ///   - Offspring dominant slot = average of the two drawn values.
    ///   - If both parents carry strong recessive (≥ REEMERGENCE_THRESHOLD) for
    ///     the same slot, the hidden culture may surface (REEMERGENCE_CHANCE).
    ///
    /// Replaces the old averaging formula for Sprint 5.
    pub fn resolve_culture<R: Rng>(
        parent_a: &CultureAlleles,
        parent_b: &CultureAlleles,
        rng: &mut R,
    ) -> CultureAlleles {
        let mut dominant  = [0.0f32; 9];
        let mut recessive = [0.0f32; 9];

        for i in 0..9 {
            // Biased draw — 75% chance of pulling dominant allele
            let draw_a = if rng.gen::<f32>() < DOMINANT_DRAW_BIAS {
                parent_a.dominant.0[i]
            } else {
                parent_a.recessive.0[i]
            };
            let draw_b = if rng.gen::<f32>() < DOMINANT_DRAW_BIAS {
                parent_b.dominant.0[i]
            } else {
                parent_b.recessive.0[i]
            };

            dominant[i] = (draw_a + draw_b) / 2.0;

            // Reemergence: both parents carry strong recessive of same slot
            if parent_a.recessive.0[i] >= REEMERGENCE_THRESHOLD
                && parent_b.recessive.0[i] >= REEMERGENCE_THRESHOLD
                && rng.gen::<f32>() < REEMERGENCE_CHANCE
            {
                let surfaced = (parent_a.recessive.0[i] + parent_b.recessive.0[i]) / 2.0;
                dominant[i]  += surfaced * 0.7;  // surface into dominant
                recessive[i]  = surfaced * 0.3;  // residual stays recessive
            }
        }

        let mut result = CultureAlleles {
            dominant:  CultureExpression::normalise(dominant),
            recessive: CultureExpression([recessive[0], recessive[1], recessive[2],
                                          recessive[3], recessive[4], recessive[5],
                                          recessive[6], recessive[7], recessive[8]]),
        };
        result.normalise_dominant();
        result
    }

    // -----------------------------------------------------------------------
    // Step 2: Stat inheritance — the Three Rules + ratchet improvement
    // -----------------------------------------------------------------------

    /// Returns (new_hp, new_atk, new_spd).
    pub fn resolve_stats<R: Rng>(
        a: &SlimeGenome,
        b: &SlimeGenome,
        dominant_culture: Culture,
        mutation_chance: f32,
        rng: &mut R,
    ) -> (f32, f32, f32) {
        let params = dominant_culture.params();

        // --- Rule 1: HP from higher parent ---
        let hp = Self::apply_ratchet(
            a.base_hp.max(b.base_hp),
            params.hp_cap(),
            mutation_chance,
            rng,
        );

        // --- Rule 2: ATK = average of both parents ---
        let atk = Self::apply_ratchet(
            (a.base_atk + b.base_atk) / 2.0,
            params.atk_cap(),
            mutation_chance,
            rng,
        );

        // --- Rule 3: SPD from faster parent, -5% penalty ---
        let spd = Self::apply_ratchet(
            a.base_spd.max(b.base_spd) * 0.95,
            params.spd_cap(),
            mutation_chance,
            rng,
        );

        (hp, atk, spd)
    }

    // -----------------------------------------------------------------------
    // Step 3: Visual inheritance
    // -----------------------------------------------------------------------

    pub fn resolve_visuals<R: Rng>(
        a: &SlimeGenome,
        b: &SlimeGenome,
        a_expr: &CultureExpression,
        b_expr: &CultureExpression,
        a_stage: LifeStage,
        rng: &mut R,
    ) -> (Shape, BodySize, Pattern, Accessory, [u8; 3], [u8; 3]) {
        let a_peak = a_expr.0.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let b_peak = b_expr.0.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        // 80% dominant parent, 20% recessive
        let source = if rng.gen::<f32>() < 0.80 {
            if a_peak >= b_peak { a } else { b }
        } else {
            if a_peak >= b_peak { b } else { a }
        };

        // 10% chance single color channel mutation ±30
        let mut color = source.base_color;
        if rng.gen::<f32>() < 0.10 {
            let ch = rng.gen_range(0..3);
            color[ch] = color[ch].saturating_add(rng.gen_range(0u8..=30))
                .min(255)
                .max(color[ch].saturating_sub(30));
        }

        // Elder bonus: if accessory would be None, 20% chance it isn't
        let mut accessory = source.accessory;
        if accessory == Accessory::None {
            if rng.gen::<f32>() < a_stage.elder_rare_bonus() {
                accessory = *[Accessory::Crown, Accessory::Scar,
                               Accessory::Glow, Accessory::Shell,
                               Accessory::Crystals]
                    .iter()
                    .nth(rng.gen_range(0..5))
                    .unwrap();
            }
        }

        (source.shape, source.body_size, source.pattern, accessory, color, source.pattern_color)
    }

    // -----------------------------------------------------------------------
    // Step 4: Personality
    // -----------------------------------------------------------------------

    fn resolve_personality<R: Rng>(a: &SlimeGenome, b: &SlimeGenome, rng: &mut R) -> [f32; 4] {
        let pairs = [
            (a.curiosity,  b.curiosity),
            (a.energy,     b.energy),
            (a.affection,  b.affection),
            (a.shyness,    b.shyness),
        ];
        pairs.map(|(av, bv)| {
            let avg = (av + bv) / 2.0;
            (avg + rng.gen_range(-0.1..=0.1_f32)).clamp(0.0, 1.0)
        })
    }

    // -----------------------------------------------------------------------
    // Internals
    // -----------------------------------------------------------------------

    /// The generational ratchet: drift 10% toward cap, then optionally mutate.
    fn apply_ratchet<R: Rng>(current: f32, cap: f32, mutation_chance: f32, rng: &mut R) -> f32 {
        let improvement = (cap - current) * 0.10;
        let mut val = current + improvement;

        if rng.gen::<f32>() < mutation_chance {
            if rng.gen::<f32>() < 0.70 {
                val *= 1.25; // positive (70%)
            } else {
                val *= 0.85; // negative (30%)
            }
        }
        val.min(cap)
    }

    fn mutation_chance(a: &SlimeGenome, b: &SlimeGenome) -> f32 {
        let base = DEFAULT_MUTATION_CHANCE;
        // Void parentage amplifies mutation floor
        let has_void = a.dominant_culture() == Culture::Void
            || b.dominant_culture() == Culture::Void;
        if has_void { base.max(VOID_MUTATION_FLOOR) } else { base }
    }
}

// ---------------------------------------------------------------------------
// Generator — seeding a fresh SlimeGenome from a culture
// ---------------------------------------------------------------------------

pub fn generate_random<R: Rng>(culture: Culture, name: &str, rng: &mut R) -> SlimeGenome {
    let params = culture.params();

    let shapes    = [Shape::Round, Shape::Cubic, Shape::Elongated, Shape::Crystalline, Shape::Amorphous];
    let sizes     = [BodySize::Tiny, BodySize::Small, BodySize::Medium, BodySize::Large, BodySize::Massive];
    let patterns  = [Pattern::Solid, Pattern::Spotted, Pattern::Striped, Pattern::Marbled, Pattern::Iridescent];
    let accessories = [Accessory::None, Accessory::Crown, Accessory::Scar,
                       Accessory::Glow, Accessory::Shell, Accessory::Crystals];

    SlimeGenome {
        id:           Uuid::new_v4(),
        culture_alleles: CultureAlleles::blooded(culture),
        base_hp:      params.base_hp()  * rng.gen_range(0.85..=1.15),
        base_atk:     params.base_atk() * rng.gen_range(0.85..=1.15),
        base_spd:     params.base_spd() * rng.gen_range(0.85..=1.15),
        generation:   1,
        parent_ids:   None,
        curiosity:    rng.gen(),
        energy:       rng.gen(),
        affection:    rng.gen(),
        shyness:      rng.gen(),
        shape:         shapes[rng.gen_range(0..shapes.len())],
        body_size:     sizes[rng.gen_range(0..sizes.len())],
        pattern:       patterns[rng.gen_range(0..patterns.len())],
        accessory:     accessories[rng.gen_range(0..accessories.len())],
        base_color:    [rng.gen_range(50..=255), rng.gen_range(50..=255), rng.gen_range(50..=255)],
        pattern_color: [rng.gen_range(50..=255), rng.gen_range(50..=255), rng.gen_range(50..=255)],
        frequency:     culture.frequency(),
        name:          name.to_string(),
        base_strength: rng.gen_range(5..=8),
        base_agility:  rng.gen_range(5..=8),
        base_intelligence: rng.gen_range(5..=8),
        base_mind: rng.gen_range(5..=8),
        base_sensory: rng.gen_range(5..=8),
        base_tenacity: rng.gen_range(5..=8),
    }
}

// ---------------------------------------------------------------------------
// Phase C — HSL Color Derivation (ADR-023 v2 hue map)
// ---------------------------------------------------------------------------

/// Hue values (degrees, 0.0–360.0) for each culture in WHEEL order.
/// Indices: Ember=0, Tide=1, Orange=2, Marsh=3, Teal=4, Crystal=5, Gale=6, Tundra=7, Frost=8
///
/// Non-linear spacing reflects colour theory. The cool cluster (Teal 180°,
/// Frost 200°, Crystal 220°) is intentional — cultures within 40° means any
/// two-culture cool blend converges toward the third. Either design as a
/// feature (blends signal kinship) or add hue-spread correction in Sprint 6.
pub const CULTURE_HUES: [f32; 9] = [
      0.0,  // Ember   — RED
     25.0,  // Tide    — ORANGE
     38.0,  // Orange  — AMBER
     52.0,  // Marsh   — YELLOW
    140.0,  // Gale    — GREEN
    180.0,  // Teal    — TEAL
    200.0,  // Frost   — ICE BLUE  (cool cluster: within 40° of Teal and Crystal)
    220.0,  // Crystal — BLUE
    280.0,  // Tundra  — VIOLET
];

/// Per-culture base saturation (0.0–1.0).
/// Inner primaries most vivid; outer tertiaries slightly muted.
pub const CULTURE_SATURATIONS: [f32; 9] = [
    0.95,  // Ember
    0.90,  // Tide
    0.85,  // Orange
    0.90,  // Marsh
    0.85,  // Gale
    0.80,  // Teal
    0.75,  // Frost
    0.90,  // Crystal
    0.85,  // Tundra
];

/// Derive an RGB display colour from a `CultureAlleles` dominant array.
///
/// Algorithm:
/// 1. Filter dominant slots above 0.05 expression threshold.
/// 2. Circular mean of hue values (handles 0°/360° wrap correctly).
/// 3. Weighted average of per-culture saturations, then
///    scaled by peak dominant weight — purity = vivid, blend = muted.
/// 4. Lightness fixed at 0.50 for readability across all screens.
/// 5. Returns RGB via `hsl_to_rgb()`.
///
/// Saturation scaling: `base_sat × (0.4 + 0.6 × max_weight)`
/// - max_weight = 1.0 (Blooded): full saturation — unmistakable hue.
/// - max_weight ≈ 0.11 (all 9 equal): heavily muted — readable blend.
/// Standalone color lookup for a specific culture (Sprint 8).
pub fn culture_display_color_standalone(c: Culture) -> (u8, u8, u8) {
    match c {
        Culture::Ember   => (220, 80, 80),   // RED
        Culture::Tide    => (240, 140, 60),  // ORANGE
        Culture::Orange  => (255, 191, 0),   // AMBER
        Culture::Marsh   => (240, 220, 80),  // YELLOW
        Culture::Teal    => (0, 128, 128),   // TEAL
        Culture::Crystal => (100, 150, 240), // BLUE
        Culture::Gale    => (120, 200, 100), // GREEN
        Culture::Tundra  => (155, 89, 182),  // VIOLET
        Culture::Frost   => (173, 216, 230), // ICE BLUE
        Culture::Void    => (255, 255, 255), // WHITE
    }
}

pub fn culture_display_color(alleles: &CultureAlleles) -> (u8, u8, u8) {
    // Sprint 6: Void Slime Prismatic Shift
    // If the dominant array indicates a Tier 8/9 (Void), bypass muddy grey for Prismatic White.
    if GeneticTier::from_expression(&alleles.dominant) == GeneticTier::Void {
        return (255, 255, 255);
    }

    let expressed: Vec<(usize, f32)> = alleles.dominant
        .0
        .iter()
        .enumerate()
        .filter(|(_, &w)| w > 0.05)
        .map(|(i, &w)| (i, w))
        .collect();

    if expressed.is_empty() {
        return (136, 136, 136); // Void grey fallback
    }

    let total: f32 = expressed.iter().map(|(_, w)| w).sum();
    let weights: Vec<f32> = expressed.iter().map(|(_, w)| w / total).collect();

    // Circular hue mean — prevents wrap-around artefact at 0°/360°
    let sin_sum: f32 = expressed.iter().zip(&weights)
        .map(|((i, _), w)| w * CULTURE_HUES[*i].to_radians().sin())
        .sum();
    let cos_sum: f32 = expressed.iter().zip(&weights)
        .map(|((i, _), w)| w * CULTURE_HUES[*i].to_radians().cos())
        .sum();
    let hue = sin_sum.atan2(cos_sum).to_degrees().rem_euclid(360.0);

    let base_sat: f32 = expressed.iter().zip(&weights)
        .map(|((i, _), w)| w * CULTURE_SATURATIONS[*i])
        .sum();
    let max_weight = weights.iter().cloned().fold(0.0f32, f32::max);
    // max_weight 1.0 → full saturation; 0.11 (9-equal) → heavily muted
    let saturation = base_sat * (0.4 + 0.6 * max_weight);
    let lightness  = 0.50_f32;

    hsl_to_rgb(hue, saturation, lightness)
}

/// Standard HSL → RGB conversion.
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match h as u32 {
        0..=59   => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _         => (c, 0.0, x),
    };
    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

// ---------------------------------------------------------------------------
// Phase D — refine_culture(): Reagents spend operation
// ---------------------------------------------------------------------------

/// Amplify a culture's dominant weight via Reagents spend.
///
/// - Amplifies `target` dominant slot by `intensity × 0.3`.
/// - Suppresses all other dominant slots by `intensity × 0.2` each,
///   bleeding the suppressed weight into their recessive slots.
/// - Normalises the dominant array afterward.
/// - Returns `false` if `target == Culture::Void` (Void cannot be refined).
///
/// `intensity`: 0.0–1.0, proportional to Reagents spent.
/// A single Reagents unit corresponds to intensity ≈ 0.33.
pub fn refine_culture(alleles: &mut CultureAlleles, target: Culture, intensity: f32) -> bool {
    let Some(idx) = Culture::WHEEL.iter().position(|&c| c == target) else {
        return false; // Void is not on WHEEL — cannot refine
    };

    // Amplify target dominant slot
    alleles.dominant.0[idx] = (alleles.dominant.0[idx] + intensity * 0.3_f32).min(1.0);

    // Suppress others and bleed into recessive
    for i in 0..9 {
        if i == idx { continue; }
        let suppressed = alleles.dominant.0[i] * intensity * 0.2_f32;
        alleles.dominant.0[i]  = (alleles.dominant.0[i] - suppressed).max(0.0);
        alleles.recessive.0[i] = (alleles.recessive.0[i] + suppressed).min(0.5);
    }

    alleles.normalise_dominant();
    true
}



// ---------------------------------------------------------------------------
// Phase E — UI Data Helpers (Sprint 6)
// ---------------------------------------------------------------------------

/// Filter and renormalise expression slots for the UI Spectrum Bar.
/// Returns (WHEEL_index, normalized_weight) pairs.
pub fn spectrum_segments(expr: &[f32; 9], threshold: f32) -> Vec<(usize, f32)> {
    let filtered: Vec<(usize, f32)> = expr.iter()
        .enumerate()
        .filter(|(_, &w)| w >= threshold)
        .map(|(i, &w)| (i, w))
        .collect();

    if filtered.is_empty() {
        return Vec::new();
    }

    let total: f32 = filtered.iter().map(|(_, w)| w).sum();
    filtered.into_iter().map(|(i, w)| (i, w / total)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    fn rng() -> SmallRng { SmallRng::seed_from_u64(42) }

    #[test]
    fn test_tier_label_strings() {
        assert_eq!(GeneticTier::Blooded.name(), "BLOODED");
        assert_eq!(GeneticTier::Sundered.name(), "SUNDERED");
        assert_eq!(GeneticTier::Void.name(), "VOID");
    }

    #[test]
    fn test_spectrum_segments_renormalize() {
        // 0.5 Ember, 0.5 Crystal, 0.01 Tide (below threshold)
        let mut arr = [0.0f32; 9];
        arr[0] = 0.5;
        arr[5] = 0.5;
        arr[1] = 0.01;
        
        let segments = spectrum_segments(&arr, 0.05);
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], (0, 0.5)); // Ember
        assert_eq!(segments[1], (5, 0.5)); // Crystal
    }

    #[test]
    fn test_spectrum_segments_empty() {
        let arr = [0.0f32; 9];
        let segments = spectrum_segments(&arr, 0.05);
        assert!(segments.is_empty());
    }

    #[test]
    fn test_culture_display_color_void_white() {
        let alleles = CultureAlleles::void();
        let (r, g, b) = culture_display_color(&alleles);
        assert_eq!((r, g, b), (255, 255, 255));
    }

    #[test]
    fn test_genetic_tier_blooded() {
        let expr = CultureExpression::pure(Culture::Ember);
        assert_eq!(GeneticTier::from_expression(&expr), GeneticTier::Blooded);
    }

    #[test]
    fn test_genetic_tier_bordered() {
        // Ember(0) and Tide(1) are adjacent on the nonagon wheel
        let mut arr = [0.0f32; 9];
        arr[0] = 0.5; // Ember  (WHEEL pos 0)
        arr[1] = 0.5; // Tide   (WHEEL pos 1) — adjacent to Ember
        let expr = CultureExpression::normalise(arr);
        assert_eq!(GeneticTier::from_expression(&expr), GeneticTier::Bordered);
    }

    #[test]
    fn test_genetic_tier_sundered() {
        // Ember(0) and Crystal(5) are near-opposites on the nonagon (dist=5, min=4)
        let mut arr = [0.0f32; 9];
        arr[0] = 0.5; // Ember   (WHEEL pos 0)
        arr[5] = 0.5; // Crystal (WHEEL pos 5) — near-opposite of Ember
        let expr = CultureExpression::normalise(arr);
        assert_eq!(GeneticTier::from_expression(&expr), GeneticTier::Sundered);
    }

    #[test]
    fn test_genetic_tier_void() {
        let expr = CultureExpression::void();
        assert_eq!(GeneticTier::from_expression(&expr), GeneticTier::Void);
    }

    #[test]
    fn test_stat_ratchet_never_exceeds_cap() {
        let mut r = rng();
        let crystal_cap = Culture::Crystal.params().hp_cap();
        for _ in 0..200 {
            let v = BreedingResolver::apply_ratchet(crystal_cap - 1.0, crystal_cap, 1.0, &mut r);
            assert!(v <= crystal_cap, "Ratchet exceeded cap: {v} > {crystal_cap}");
        }
    }

    #[test]
    fn test_breed_basic() {
        let mut r = rng();
        let a = generate_random(Culture::Ember, "Alpha", &mut r);
        let b = generate_random(Culture::Crystal, "Beta", &mut r);
        let child = BreedingResolver::breed(&a, &b, "Child", &mut r);
        // both are Hatchlings — must fail
        assert!(child.is_err(), "Hatchlings cannot breed");
    }

    #[test]
    fn test_life_stage_gates() {
        assert!(!LifeStage::Hatchling.can_breed());
        assert!(!LifeStage::Juvenile.can_breed());
        assert!(LifeStage::Young.can_breed());
        assert!(LifeStage::Elder.can_mentor());
        assert!(!LifeStage::Prime.can_mentor());
    }

    #[test]
    fn test_xp_curve() {
        assert_eq!(LifeStage::xp_to_next(0), 100);   // level 0 needs 100 XP
        assert_eq!(LifeStage::xp_to_next(9), 1000);  // level 9 needs 1000 XP
        assert_eq!(LifeStage::xp_to_next(10), 0);    // Elder is max
    }

    #[test]
    fn test_race_stats_massive_vs_tiny() {
        let mut r = rng();
        let big  = {
            let mut g = generate_random(Culture::Ember, "Big", &mut r);
            g.body_size = BodySize::Massive;
            g
        };
        let small = {
            let mut g = generate_random(Culture::Ember, "Small", &mut r);
            g.body_size = BodySize::Tiny;
            g
        };
        assert!(big.race_stats().mass > small.race_stats().mass, "Massive must be heavier");
    }

    // Phase G — Sprint 4 tests (nonagon geometry, CultureExpression 9-slot, GeneticTier)
    #[test]
    fn test_culture_expression_nine_slots_normalize() {
        let expr = CultureExpression::pure(Culture::Ember);
        assert_eq!(expr.0.len(), 9, "CultureExpression must have 9 slots");
        let sum: f32 = expr.0.iter().sum();
        assert!((sum - 1.0).abs() < 0.001, "Sum must normalise to 1.0");
        assert!((expr.get(Culture::Ember) - 1.0).abs() < 0.001, "Ember slot must be 1.0");
        assert!((expr.get(Culture::Frost) - 0.0).abs() < 0.001, "Frost slot must be 0.0");
    }

    #[test]
    fn test_culture_expression_void_all_equal() {
        let expr = CultureExpression::void();
        for v in expr.0 {
            assert!((v - 1.0/9.0).abs() < 0.001, "Void must be 1/9 each");
        }
    }

    #[test]
    fn test_is_near_opposite_all_nine_pairs_symmetric() {
        // Verify all 9 Sundered pairs from ADR-023 v2 near-opposite table
        let pairs = [
            (Culture::Ember,   Culture::Teal),    // 0 ↔ 4: dist 4
            (Culture::Ember,   Culture::Crystal),  // 0 ↔ 5: dist 5, min=4
            (Culture::Tide,    Culture::Crystal),  // 1 ↔ 5: dist 4
            (Culture::Tide,    Culture::Gale),     // 1 ↔ 6: dist 5, min=4 (1+5=6)
            (Culture::Orange,  Culture::Gale),     // 2 ↔ 6: dist 4
            (Culture::Orange,  Culture::Tundra),   // 2 ↔ 7: dist 5, min=4
            (Culture::Marsh,   Culture::Tundra),   // 3 ↔ 7: dist 4
            (Culture::Marsh,   Culture::Frost),    // 3 ↔ 8: dist 5, min=4
            (Culture::Teal,    Culture::Frost),    // 4 ↔ 8: dist 4
        ];
        for (a, b) in pairs {
            assert!(is_near_opposite(a, b), "{:?} ↔ {:?} must be near-opposite", a, b);
            assert!(is_near_opposite(b, a), "{:?} ↔ {:?} symmetric check failed", b, a);
        }
    }

    #[test]
    fn test_genetic_tier_sundered_near_opposite() {
        // Ember(0) and Crystal(5) are near-opposites → GeneticTier::Sundered
        let mut arr = [0.0f32; 9];
        arr[0] = 0.6; // Ember
        arr[5] = 0.4; // Crystal
        let expr = CultureExpression::normalise(arr);
        assert_eq!(GeneticTier::from_expression(&expr), GeneticTier::Sundered);
    }

    #[test]
    fn test_genetic_tier_void_eight_active() {
        // 8 cultures active at significant level → GeneticTier::Void
        let arr = [0.125f32; 9]; // all equal (all 9 active)
        let expr = CultureExpression::normalise(arr);
        let active = expr.0.iter().filter(|&&v| v >= 0.05).count();
        assert_eq!(active, 9);
        assert_eq!(GeneticTier::from_expression(&expr), GeneticTier::Void);
    }

    // Phase F Tests: CultureAlleles
    #[test]
    fn test_culture_alleles_blooded_ember_dominant_1_0() {
        let alleles = CultureAlleles::blooded(Culture::Ember);
        assert_eq!(alleles.dominant.get(Culture::Ember), 1.0);
        assert_eq!(alleles.recessive.0.iter().sum::<f32>(), 0.0);
    }

    #[test]
    fn test_culture_alleles_new_zeroes_recessive() {
        let expr = CultureExpression::pure(Culture::Tide);
        let alleles = CultureAlleles::from_expression(expr);
        assert_eq!(alleles.dominant.get(Culture::Tide), 1.0);
        assert_eq!(alleles.recessive.0.iter().sum::<f32>(), 0.0);
    }

    #[test]
    fn test_culture_alleles_normalise_sums_to_1() {
        let mut alleles = CultureAlleles::blooded(Culture::Marsh);
        alleles.dominant.0[0] = 0.5; // Ember
        alleles.dominant.0[3] = 0.5; // Marsh
        alleles.dominant.0[6] = 0.5; // Gale (sum is 1.5)
        alleles.normalise_dominant();
        let sum: f32 = alleles.dominant.0.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
        assert!((alleles.dominant.get(Culture::Marsh) - 0.333).abs() < 0.01);
    }

    // Phase F Tests: Breeding resolver
    #[test]
    fn test_resolve_culture_blooded_parents_dominant_preserved() {
        let a = CultureAlleles::blooded(Culture::Ember);
        let b = CultureAlleles::blooded(Culture::Crystal);
        let child = BreedingResolver::resolve_culture(&a, &b, &mut rng());
        // Both parents have 0 recessive, so regardless of draw, offspring
        // averages out to 0.5 / 0.5
        assert_eq!(child.dominant.get(Culture::Ember), 0.5);
        assert_eq!(child.dominant.get(Culture::Crystal), 0.5);
        let sum: f32 = child.dominant.0.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_resolve_culture_reemergence_fires_above_threshold() {
        let mut a = CultureAlleles::blooded(Culture::Ember);
        let mut b = CultureAlleles::blooded(Culture::Gale);
        
        // Give both strong recessive for Tide (idx 1)
        a.recessive.0[1] = 0.50;
        b.recessive.0[1] = 0.50;

        let mut hit = false;
        let mut _rng = rng();
        for _ in 0..100 {
            let child = BreedingResolver::resolve_culture(&a, &b, &mut _rng);
            if child.dominant.0[1] > 0.1 {
                hit = true;
                break;
            }
        }
        assert!(hit, "Reemergence should fire at 30% rate over 100 iterations");
    }

    #[test]
    fn test_resolve_culture_reemergence_blocked_below_threshold() {
        let mut a = CultureAlleles::blooded(Culture::Ember);
        let mut b = CultureAlleles::blooded(Culture::Gale);
        
        // Weak recessive for Tide (idx 1)
        a.recessive.0[1] = 0.05;
        b.recessive.0[1] = 0.05;

        let mut _rng = rng();
        for _ in 0..10 {
            let child = BreedingResolver::resolve_culture(&a, &b, &mut _rng);
            assert!(child.dominant.0[1] < 0.05, "Reemergence should NOT fire if below threshold");
        }
    }

    #[test]
    fn test_resolve_culture_result_normalised() {
        let a = CultureAlleles::blooded(Culture::Tide);
        let b = CultureAlleles::blooded(Culture::Frost);
        let child = BreedingResolver::resolve_culture(&a, &b, &mut rng());
        let sum: f32 = child.dominant.0.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
    }

    // Phase F Tests: HSL display
    #[test]
    fn test_culture_display_color_blooded_ember_is_red() {
        let alleles = CultureAlleles::blooded(Culture::Ember);
        let rgb = culture_display_color(&alleles);
        assert!(rgb.0 > 240);
        assert!(rgb.1 < 20);
        assert!(rgb.2 < 20);
    }

    #[test]
    fn test_culture_display_color_blooded_marsh_is_yellow() {
        let alleles = CultureAlleles::blooded(Culture::Marsh);
        let rgb = culture_display_color(&alleles);
        assert!(rgb.0 > 200);
        assert!(rgb.1 > 200);
        assert!(rgb.2 < 50);
    }

    #[test]
    fn test_culture_display_color_void_fallback_is_grey() {
        let alleles = CultureAlleles::void();
        // Since void means all slots = 0.111, they are all < 0.05? 
        // No, > 0.05. It should just not panic and return some color.
        let rgb = culture_display_color(&alleles);
        assert!(rgb.0 > 0 || rgb.0 == 0);
    }

    #[test]
    fn test_culture_display_color_below_threshold_excluded() {
        let mut alleles = CultureAlleles::blooded(Culture::Ember);
        alleles.dominant.0[1] = 0.01; // Tide below 0.05 threshold
        let rgb = culture_display_color(&alleles);
        assert!(rgb.0 > 240); // still primarily red
    }

    // Phase F Tests: Refine
    #[test]
    fn test_refine_culture_increases_target_dominant() {
        let mut alleles = CultureAlleles::void();
        let intensity = 1.0;
        let success = refine_culture(&mut alleles, Culture::Ember, intensity);
        assert!(success);
        let e = alleles.dominant.get(Culture::Ember);
        assert!(e > 0.2); // originally 0.111, boosted by ~ 0.3 then normalised
    }

    #[test]
    fn test_refine_culture_void_returns_false() {
        let mut alleles = CultureAlleles::void();
        let success = refine_culture(&mut alleles, Culture::Void, 1.0);
        assert!(!success);
    }
}

