# SLIME → OPERATOR: Functional Blueprint
> **Source**: `C:\Github\rpgCore` systemic audit — 2026-03-04  
> **Deliverable**: Rust trait/enum spec for transplanting rpgCore's soul into the OPERATOR chassis.

---

## 1. The Genetic Math — Rendered Formulas

### 1.1 Cultural Archetypes (The DNA Alphabet)

Six cultures arranged on a **hexagon wheel** — position determines tier when cultures blend.

```
         GALE (speed, blue)
    EMBER              CRYSTAL
  (attack, red)      (tank, white)
    MARSH              TIDE
  (balanced, green)  (energetic, blue)
         TUNDRA (endurance, cool)

Opposites: Ember↔Crystal, Gale↔Tundra, Marsh↔Tide
```

**Exact stat modifiers from `cultural_base.py`:**

| Culture | HP mod | ATK mod | SPD mod | Rare trait % |
|---------|--------|---------|---------|--------------|
| Ember   | ×0.8   | ×1.4    | ×1.1    | 5%           |
| Gale    | ×0.9   | ×0.9    | ×1.4    | 6%           |
| Marsh   | ×1.0   | ×0.9    | ×1.3    | 4%           |
| Crystal | ×1.4   | ×0.8    | ×0.7    | 8%           |
| Tundra  | ×1.1   | ×0.9    | ×0.8    | 5%           |
| Tide    | ×1.0   | ×1.0    | ×1.2    | 7%           |
| Void    | ×1.2   | ×1.2    | ×1.2    | 25%          |

**Base stats before modifiers**: HP=20.0, ATK=5.0, SPD=5.0

**Rust enum target:**
```rust
pub enum Culture { Ember, Gale, Marsh, Crystal, Tundra, Tide, Void }
pub struct CulturalParams { hp_mod: f32, atk_mod: f32, spd_mod: f32, rare_chance: f32 }
```

---

### 1.2 Stat Inheritance — The Three Rules

From `inheritance.py`, `breed()` function (lines 50–140):

```python
# HP: Takes from the HIGHER parent, then applies improvement
new_hp = apply_improvement_and_cap(max(parent_a.base_hp, parent_b.base_hp), hp_cap)

# ATK: AVERAGE of both parents, then improvement
new_atk = apply_improvement_and_cap((a.base_atk + b.base_atk) / 2.0, atk_cap)

# SPD: Higher parent MINUS 5% penalty, then improvement
new_spd = apply_improvement_and_cap(max(a.base_spd, b.base_spd) * 0.95, spd_cap)
```

**Improvement formula** (generational ratchet — stats can never go down):
```python
improvement = (cap - current_val) * 0.10   # 10% drift toward cap per generation
new_val = current_val + improvement
```

**Cap formula:**
```python
hp_cap  = base_hp  * culture_modifier * 2.0   # e.g. Crystal HP cap = 20 * 1.4 * 2 = 56
atk_cap = base_atk * culture_modifier * 2.0
spd_cap = base_spd * culture_modifier * 2.0
```

**Mutation trigger** (applied to the improved value):
```python
if rand < mutation_chance:           # default 5%; Void parents force ≥15%
    if rand < 0.70: val *= 1.25      # 70% chance — POSITIVE mutation (+25%)
    else:           val *= 0.85      # 30% chance — NEGATIVE mutation (-15%)
val = min(val, cap)                  # hard cap enforced after mutation
```

---

### 1.3 Culture Expression Blending (The Advanced System)

From `BreedingSystem.resolve_culture_expression()` in `breeding_system.py`:

Each genome carries a **6-float vector** summing to 1.0.  
Offspring inherits the weighted blend with variance noise:

```python
for each culture:
    blended  = (parent_a[culture] + parent_b[culture]) / 2.0
    variance = uniform(-0.15, 0.15)          # VARIANCE_RANGE = 0.15
    raw      = max(0.0, blended + variance * blended)

# Renormalize so all 6 values sum to 1.0
offspring[culture] = raw / sum(all_raw_values)
```

**Visual dominance rule:**
- 80% chance offspring inherits shape/pattern from the parent with the **higher peak culture expression**
- 20% chance recessive parent wins
- 10% chance of **color mutation** (±30 on a single RGB channel)

---

### 1.4 Genetic Tier System (The Progression Gate)

From `SlimeGenome.tier` property — count of cultures with `expression ≥ 0.05`:

| Active cultures | Adjacency check | Tier | Name        |
|----------------|-----------------|------|-------------|
| 1              | —               | 1    | Blooded     |
| 2              | Adjacent on hex | 2    | Bordered    |
| 2              | Opposite on hex | 3    | Sundered    |
| 2              | Skip-one        | 4    | Drifted     |
| 3              | —               | 5    | Threaded    |
| 4              | —               | 6    | Convergent  |
| 5              | —               | 7    | Liminal     |
| 6 (all)        | —               | 8    | Void        |

**Special `stage_modifier` bonuses** (tier × lifecycle combos):
- Sundered + Prime → `volatile_peak`
- Liminal + Elder  → `threshold_legacy`
- Any Void tier    → `primordial_{stage}`

---

### 1.5 Lifecycle State Machine

From `SlimeGenome.stage` (derived from `level: 0–10`):

| Level | Stage     | Can Dispatch | Can Breed | Can Mentor | Dispatch Risk |
|-------|-----------|:---:|:---:|:---:|---|
| 0–1   | Hatchling | ✗   | ✗   | ✗   | none     |
| 2–3   | Juvenile  | ✓   | ✗   | ✗   | low      |
| 4–5   | Young     | ✓   | ✓   | ✗   | standard |
| 6–7   | Prime     | ✓   | ✓   | ✗   | standard |
| 8–9   | Veteran   | ✓   | ✓   | ✗   | high     |
| 10    | Elder     | ✓   | ✓   | ✓   | critical |

**XP curve**: `xp_to_next = (level + 1) * 100`  
*(Level 1 needs 200 XP, Level 9 needs 1000 XP — linear.)*

**Elder breeding bonus**: `+20%` chance of rare accessory inheritance if offspring would have gotten `none`.

---

### 1.6 Race Stats Formula

From `calculate_race_stats()` in `genome.py`:

```python
body_size   = {"tiny":0.3, "small":0.5, "medium":0.7, "large":0.9, "massive":1.0}[size]
mass        = body_size ** 1.5                           # non-linear: large is much heavier
strength    = base_atk / 100.0                           # normalize ATK → 0.0–1.0
heft_power  = mass * (1.0 + strength * 0.5)             # obstacle-pushing power
jump_force  = 50.0 * (1.0 + strength * 0.3)
jump_dist   = (jump_force / mass) * body_size
jump_cool   = 0.2 + (mass * 0.4) * (1.0 - strength * 0.2)
jump_height = 14  # fixed 14px regardless of stats
```

---

## 2. The Elemental Matrix (Status Effects)

> **Note**: The status/elemental system is embedded in dungeon scenes (`scene_dungeon_path.py`, `run_dungeon_crawler.py`) rather than a dedicated module. The D20 resolver drives all checks.

### 2.1 Combat Resolution Engine

From `d20_resolver.py`:

```
roll = d20 + modifier
success = roll >= difficulty_class (DC)
```

**Difficulty Class ladder:**
```
TRIVIAL=5 | EASY=10 | MODERATE=15 | HARD=20 | VERY_HARD=25 | NEAR_IMPOSSIBLE=30
```

**Advantage/Disadvantage:**
- Advantage: `max(roll1, roll2) + modifier`
- Disadvantage: `min(roll1, roll2) + modifier`
- Both cancel out: `single roll + modifier`

### 2.2 Elemental Interaction (Inferred from scene files)

The dungeon path (`scene_dungeon_path.py`, 18KB) and combat scenes drive status effects through the D20 system. The cultural identity maps directly to element type and attack flavor:

| Culture | Element  | Combat style         |
|---------|----------|----------------------|
| Ember   | Fire     | Burn DoT, high ATK   |
| Gale    | Wind     | Speed burst, evasion |
| Marsh   | Poison   | Bleed/Poison DoT     |
| Crystal | Ice      | Armor/Shield         |
| Tundra  | Frost    | Slow debuff          |
| Tide    | Electric | Chain lightning      |
| Void    | Null     | All elements ×0.75, rare procs ×4 |

> **Rust transplant directive**: These should become a `StatusEffect` enum with a `tick()` method that applies damage/modifier each turn via the D20 resolver.

---

## 3. The Progression Loop (Conquest Tiers)

### 3.1 The Garden → Island Expedition Flow

From `game.py` and scene inventory (`slime_breeder/scenes/`):

```
[ GARDEN ]          ← Base of operations
     │ breed + level up slimes
     ▼
[ TEAM SCENE ]      ← team_scene.py — Roster management, squad picking
     │ select squad (max 3)
     ▼
[ DUNGEON PATH ]    ← scene_dungeon_path.py — The "Island Expedition"
     │ roguelike encounter chain
     ▼
[ COMBAT ]          ← d20 resolver drives all checks
     │ win/lose per node
     ▼
[ RACE SCENE ]      ← race_scene.py — Economic unlock loop
     │ race for prize money
     ▼
[ SUMO / TOWER ]    ← scene_sumo.py, scene_tower_defense.py — PvP/Defense
     │ advanced content
     ▼
[ AAR + REWARDS ]   ← back to garden with XP, gold, new breeds
```

### 3.2 Dispatch Gating (The OPERATOR Connection)

This maps **directly** onto the OPERATOR missions system:

| rpgCore concept        | OPERATOR equivalent              |
|------------------------|----------------------------------|
| Slime `stage`          | `OperatorState` (Idle/Deployed)  |
| `can_dispatch`         | `is_available()`                 |
| Dungeon floor nodes    | `Mission` difficulty tiers        |
| Garden                 | War Room roster                  |
| XP + level up          | **Missing — add to Operator**    |
| Breeding result        | `Operator::new()` with inherited stats |

### 3.3 Island Expedition Modifiers (Roguelike Layer)

From `scene_dungeon_path.py` (18KB — 6 encounter categories inferred):
- **Combat node**: D20 check vs DC scaled to dungeon depth
- **Treasure node**: reward multiplier if squad culture matches zone element
- **Trap node**: saving throw to avoid injury
- **Rest node**: recover HP (rate = `slime.base_hp * 0.3`)
- **Elite node**: boss with advantage rolls
- **Extraction node**: end dungeon, collect full rewards

---

## 4. Rust Trait/Enum Transplant Map

```rust
// ── Genetic Layer ──────────────────────────────────────────────
pub enum Culture { Ember, Gale, Marsh, Crystal, Tundra, Tide, Void }
pub enum LifeStage { Hatchling, Juvenile, Young, Prime, Veteran, Elder }
pub enum GeneticTier { Blooded=1, Bordered=2, Sundered=3, Drifted=4,
                       Threaded=5, Convergent=6, Liminal=7, Void=8 }

pub struct CultureExpression([f32; 6]); // sums to 1.0, order = enum discriminant

pub struct SlimeGenome {
    pub culture_expr: CultureExpression,
    pub base_hp:   f32,
    pub base_atk:  f32,
    pub base_spd:  f32,
    pub level:     u8,           // 0–10
    pub xp:        u32,
    pub generation: u32,
    pub parent_ids: Option<[Uuid; 2]>,
    pub mutations: Vec<MutationRecord>,
    // Visual traits (can be String enums or tagged u8s)
    pub shape:     Shape,
    pub size:      BodySize,
    pub pattern:   Pattern,
    pub accessory: Accessory,
    pub base_color:    [u8; 3],
    pub pattern_color: [u8; 3],
    // Personality (0.0–1.0)
    pub curiosity:  f32,
    pub energy:     f32,
    pub affection:  f32,
    pub shyness:    f32,
}

pub trait Breedable {
    fn can_breed(&self) -> bool;
    fn life_stage(&self) -> LifeStage;
    fn genetic_tier(&self) -> GeneticTier;
}

pub trait BreedingResolver {
    fn breed(a: &SlimeGenome, b: &SlimeGenome, rng: &mut impl Rng) -> SlimeGenome;
    fn resolve_culture(a: &CultureExpression, b: &CultureExpression, rng: &mut impl Rng) -> CultureExpression;
    fn resolve_stats(a: &SlimeGenome, b: &SlimeGenome, rng: &mut impl Rng) -> (f32, f32, f32);
}

// ── Combat Layer ──────────────────────────────────────────────
pub enum DifficultyClass { Trivial=5, Easy=10, Moderate=15, Hard=20,
                           VeryHard=25, NearImpossible=30 }

pub struct D20Roll { pub raw: u8, pub modifier: i32, pub total: i32 }

pub enum RollMode { Normal, Advantage, Disadvantage }

pub fn d20_check(modifier: i32, dc: DifficultyClass, mode: RollMode, rng: &mut impl Rng) -> D20Roll;

// ── Expedition Layer ──────────────────────────────────────────
pub enum ExpeditionNode { Combat, Treasure, Trap, Rest, Elite, Extraction }

pub struct DungeonFloor {
    pub depth:    u32,
    pub element:  Culture,         // zone element — culture match gives bonus
    pub nodes:    Vec<ExpeditionNode>,
}

pub struct ExpeditionResult {
    pub victory:    bool,
    pub xp_gained:  u32,
    pub gold_gained: u32,
    pub injured:    Vec<Uuid>,
    pub killed:     Vec<Uuid>,
}
```

---

## 5. The "Splicer" — Key Missing Piece

The genetics persistence gap noted in the inventory means:

> The Python code **calculates** offspring genomes correctly but **never saved them to disk**.  
> In OPERATOR (Rust), `SlimeGenome` must be `Serialize + Deserialize` (serde) and stored in `save.json` alongside the existing `GameState`.

**Recommended `GameState` extension:**
```rust
pub struct GameState {
    pub bank:        u32,
    pub operators:   Vec<Operator>,      // existing
    pub roster:      Vec<SlimeGenome>,   // NEW: slime stable
    pub missions:    Vec<Mission>,       // existing → becomes ExpeditionFloor
    pub deployments: Vec<Deployment>,    // existing
}
```

---

## 6. Audit Verdict

| System            | Python status   | Rust readiness   | Priority |
|-------------------|-----------------|------------------|----------|
| Stat inheritance  | ✅ Complete      | ✅ Blueprint ready | P0       |
| Culture blending  | ✅ Complete      | ✅ Blueprint ready | P0       |
| Tier system       | ✅ Complete      | ✅ Blueprint ready | P1       |
| Lifecycle stages  | ✅ Complete      | ✅ Blueprint ready | P1       |
| D20 combat        | ✅ Complete      | ✅ Blueprint ready | P1       |
| Race stats        | ✅ Complete      | ✅ Blueprint ready | P2       |
| Status effects    | 🔄 In scenes     | 🔄 Needs extraction | P2     |
| Island expedition | 🔄 18KB scene    | 🔄 Needs floor gen | P2     |
| Economy/market    | ❌ Missing       | ❌ Design needed   | P3     |
| Genetics persist  | ❌ Missing       | ✅ Rust = native   | P0     |
