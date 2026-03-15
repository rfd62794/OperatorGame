# SlimeGarden --- Core Loop Balance & Color/Pattern Progression (Phase E)

**Directive Type:** IMPLEMENTATION + BALANCE  
**Scope:** Initial recruit stats, zone progression, pattern regent system, dispatch difficulty, party size  
**Test Floor:** 186 → 210 passing (24 new balance + progression tests)  
**Acceptance:** Player can recruit 1 slime → breed for colors → unlock zones via dispatch → discover patterns via breeding + regents  

---

## Goal

Implement the SlimeGarden MVP core loop: **Recruit → Breed for Colors/Patterns → Dispatch to Zones → Level Up → Repeat**

Focus: Color breeding requirements, zone unlock mechanics, pattern regent system, party composition (max 3).

---

## Phase E.1: Starting State & Initial Recruit

### Task 1.1: Player Starts with 1 Slime

**Location:** `src/persistence.rs` → `GameState::new()`

```rust
impl GameState {
    pub fn new() -> Self {
        // Create starting slime: Red primary, Basic pattern
        let starter_slime = SlimeState {
            id: "starter_001".to_string(),
            culture: Culture::Red,        // Primary color
            pattern: Pattern::Basic,      // Basic pattern (no regent used)
            stats: SlimeStats {
                hp: 15,
                atk: 8,
                def: 6,
                agi: 7,
                int: 5,
            },
            level: 1,
            xp: 0,
            xp_to_level: 100,
            genetics: SlimeGenome::new_red_primary(),  // Pure Red genetics
        };
        
        Self {
            slimes: vec![starter_slime],
            party: vec!["starter_001".to_string()],  // In active party
            regents: HashMap::new(),  // No regents yet
            zones_unlocked: vec![],   // No zones unlocked
            ..Default::default()
        }
    }
}
```

**Rationale:**
- Player starts with 1 Red slime (primary color)
- Basic stats are viable for early missions (see Task 1.3)
- No regents yet (they're earned, not starting currency)

---

### Task 1.2: Party Size Cap (Max 3)

**Location:** `src/models.rs` → Add constant

```rust
pub const MAX_PARTY_SIZE: usize = 3;
```

**Location:** `src/persistence.rs` → `GameState` methods

```rust
impl GameState {
    pub fn add_slime_to_party(&mut self, slime_id: String) -> Result<(), String> {
        if self.party.len() >= MAX_PARTY_SIZE {
            return Err(format!("Party is full (max {}).", MAX_PARTY_SIZE));
        }
        if self.party.contains(&slime_id) {
            return Err("Slime already in party.".to_string());
        }
        self.party.push(slime_id);
        Ok(())
    }
    
    pub fn remove_slime_from_party(&mut self, slime_id: &str) -> Result<(), String> {
        if self.party.len() <= 1 {
            return Err("Cannot remove last slime from party.".to_string());
        }
        self.party.retain(|id| id != slime_id);
        Ok(())
    }
}
```

**Rationale:** Party size constraint prevents over-power early, balances roster management.

---

## Phase E.2: Color System & Breeding Mechanics

### Task 2.1: Define Color Requirements for Zones

**Location:** `src/world_map.rs` → Update `ExpeditionTarget`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoneColor {
    Red,
    Blue,
    Yellow,
    Green,      // Secondary: Red + Yellow
    Purple,     // Secondary: Red + Blue
    Orange,     // Secondary: Blue + Yellow
    // Tertiary colors can be added post-MVP
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub color_requirement: ZoneColor,
    pub difficulty_base: u8,
    pub resources: ResourceYield,
    pub unlocked: bool,
}

impl Zone {
    pub fn new_jungle() -> Self {
        Self {
            id: "zone_jungle".to_string(),
            name: "Emerald Jungle".to_string(),
            color_requirement: ZoneColor::Green,
            difficulty_base: 2,
            resources: ResourceYield {
                biomass: 50,
                scrap: 10,
                reagents: 5,
            },
            unlocked: false,
        }
    }
    
    pub fn new_volcano() -> Self {
        Self {
            id: "zone_volcano".to_string(),
            name: "Crimson Volcano".to_string(),
            color_requirement: ZoneColor::Red,
            difficulty_base: 1,
            resources: ResourceYield {
                biomass: 30,
                scrap: 5,
                reagents: 2,
            },
            unlocked: false,
        }
    }
    
    // Add more zones: Ocean (Blue), Plains (Yellow), etc.
}
```

**Rationale:** Each zone has a color requirement and difficulty tier. Early zones (Red, Blue, Yellow primaries) have low difficulty. Secondary colors unlock after breeding.

---

### Task 2.2: Breeding Outcomes & Pattern Chance

**Location:** `src/genetics.rs` → `BreedingResolver`

```rust
#[derive(Debug, Clone)]
pub struct BreedingOutcome {
    pub child_culture: Culture,
    pub child_pattern: Pattern,
    pub pattern_is_new: bool,  // True if pattern hasn't been seen before
    pub pattern_regent_reward: Option<String>,  // Regent ID if new pattern
}

impl BreedingResolver {
    pub fn resolve_breeding(
        parent1: &SlimeGenome,
        parent2: &SlimeGenome,
    ) -> BreedingOutcome {
        // Determine child culture (simplified: average of parents)
        let child_culture = Self::blend_cultures(parent1, parent2);
        
        // Determine pattern based on color combo + parent tier
        let (child_pattern, pattern_is_new) = Self::roll_pattern(
            &parent1.culture,
            &parent2.culture,
            parent1.genetic_tier,
            parent2.genetic_tier,
        );
        
        // If pattern is new, grant regent
        let pattern_regent_reward = if pattern_is_new {
            Some(format!("regent_{:?}_{}", child_pattern, uuid::Uuid::new_v4()))
        } else {
            None
        };
        
        BreedingOutcome {
            child_culture,
            child_pattern,
            pattern_is_new,
            pattern_regent_reward,
        }
    }
    
    fn roll_pattern(
        culture1: &Culture,
        culture2: &Culture,
        tier1: u8,
        tier2: u8,
    ) -> (Pattern, bool) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Base pattern: Basic
        let mut pattern = Pattern::Basic;
        
        // Specific combos have higher chance for certain patterns
        match (culture1, culture2) {
            (Culture::Red, Culture::Blue) | (Culture::Blue, Culture::Red) => {
                // Purple slime: high chance for Striped pattern
                if rng.gen::<f32>() < 0.6 {
                    pattern = Pattern::Striped;
                } else if rng.gen::<f32>() < 0.2 {
                    pattern = Pattern::Speckled;
                }
            }
            (Culture::Red, Culture::Yellow) | (Culture::Yellow, Culture::Red) => {
                // Orange slime: high chance for Spotted pattern
                if rng.gen::<f32>() < 0.6 {
                    pattern = Pattern::Spotted;
                } else if rng.gen::<f32>() < 0.15 {
                    pattern = Pattern::Striped;
                }
            }
            (Culture::Blue, Culture::Yellow) | (Culture::Yellow, Culture::Blue) => {
                // Green slime: balanced pattern distribution
                let roll = rng.gen::<f32>();
                if roll < 0.3 {
                    pattern = Pattern::Striped;
                } else if roll < 0.6 {
                    pattern = Pattern::Speckled;
                } else if roll < 0.85 {
                    pattern = Pattern::Spotted;
                }
            }
            _ => {
                // Same color or mixed: low chance for patterns
                if rng.gen::<f32>() < 0.1 {
                    pattern = Pattern::Speckled;
                }
            }
        }
        
        // Tier boost: higher tier parents = higher chance for rare patterns
        let tier_avg = (tier1 + tier2) as f32 / 2.0;
        if tier_avg >= 5.0 && rng.gen::<f32>() < 0.2 {
            pattern = Pattern::Rare;
        }
        
        // Check if pattern is new (not in player's collection)
        // TODO: This needs access to GameState. Pass as parameter or refactor.
        let pattern_is_new = true;  // Stub for now
        
        (pattern, pattern_is_new)
    }
}
```

**Rationale:** Specific color combos have pattern affinities. Parent tier influences rarity. New patterns grant regents.

---

## Phase E.3: Pattern Regent System

### Task 3.1: Regent Storage & Usage

**Location:** `src/persistence.rs` → `GameState`

```rust
pub struct GameState {
    pub slimes: Vec<SlimeState>,
    pub party: Vec<String>,
    pub regents: HashMap<String, usize>,  // pattern_name → count
    pub patterns_discovered: HashSet<String>,  // Track which patterns player has seen
    pub zones_unlocked: Vec<String>,
    // ... existing fields
}

impl GameState {
    pub fn use_regent_to_force_pattern(
        &mut self,
        slime_id: &str,
        pattern_name: String,
    ) -> Result<(), String> {
        // Check if regent exists
        if self.regents.get(&pattern_name).unwrap_or(&0) == &0 {
            return Err(format!("No regent for pattern: {}", pattern_name));
        }
        
        // Find slime and apply pattern
        if let Some(slime) = self.slimes.iter_mut().find(|s| s.id == slime_id) {
            slime.pattern = Pattern::from_string(&pattern_name)?;
            self.regents.entry(pattern_name).and_modify(|count| *count -= 1);
            Ok(())
        } else {
            Err("Slime not found.".to_string())
        }
    }
    
    pub fn grant_pattern_regent(&mut self, pattern_name: String) {
        self.regents.entry(pattern_name.clone()).or_insert(0);
        *self.regents.get_mut(&pattern_name).unwrap() += 1;
        self.patterns_discovered.insert(pattern_name);
    }
}
```

**Rationale:** Regents are one-time-use tokens. Using a regent guarantees a pattern without losing it to future breeding.

---

## Phase E.4: Zone Unlock Mechanics

### Task 4.1: Dispatch with Color Requirement Check

**Location:** `src/models.rs` → `Mission` struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: String,
    pub zone_id: String,
    pub zone_name: String,
    pub color_requirement: ZoneColor,
    pub party_sent: Vec<String>,
    pub difficulty_class: u8,
    pub status: MissionStatus,
    pub created_at: i64,
    pub estimated_return: i64,
}

impl Mission {
    pub fn validate_party(party: &[SlimeState], color_req: &ZoneColor) -> Result<(), String> {
        let has_required_color = party.iter().any(|slime| {
            Self::culture_matches_zone_color(&slime.culture, color_req)
        });
        
        if !has_required_color {
            return Err(format!("Party must contain a slime of the required color: {:?}", color_req));
        }
        
        Ok(())
    }
    
    fn culture_matches_zone_color(culture: &Culture, zone_color: &ZoneColor) -> bool {
        match (culture, zone_color) {
            (Culture::Red, ZoneColor::Red) => true,
            (Culture::Blue, ZoneColor::Blue) => true,
            (Culture::Yellow, ZoneColor::Yellow) => true,
            (Culture::Red, ZoneColor::Orange) | (Culture::Yellow, ZoneColor::Orange) => true,
            (Culture::Red, ZoneColor::Purple) | (Culture::Blue, ZoneColor::Purple) => true,
            (Culture::Blue, ZoneColor::Green) | (Culture::Yellow, ZoneColor::Green) => true,
            _ => false,
        }
    }
}
```

**Rationale:** Dispatch validates that party has the required color. Prevents soft-locking (can't dispatch without the right genetics).

---

### Task 4.2: Zone Unlock on First Successful Dispatch

**Location:** `src/persistence.rs` → Mission completion handler

```rust
impl GameState {
    pub fn complete_mission(&mut self, mission_id: &str, outcome: &AarOutcome) -> Result<(), String> {
        // Find mission
        let mission = self.active_expeditions
            .iter_mut()
            .find(|m| m.id == mission_id)
            .ok_or("Mission not found")?;
        
        // If mission succeeded, unlock zone
        if outcome.success {
            if !self.zones_unlocked.contains(&mission.zone_id) {
                self.zones_unlocked.push(mission.zone_id.clone());
                // Log: "Zone unlocked: [zone_name]"
            }
        }
        
        // Grant loot (resources, xp, pattern chances)
        self.grant_mission_rewards(outcome)?;
        
        Ok(())
    }
}
```

**Rationale:** First successful dispatch to a zone unlocks it permanently. No regents or currency needed—just genetics + combat success.

---

## Phase E.5: Dispatch Difficulty & Balance

### Task 5.1: Mission Difficulty Calculation

**Location:** `src/combat.rs` → Difficulty scaling

```rust
pub fn calculate_mission_difficulty(
    zone_base_difficulty: u8,
    player_party: &[SlimeState],
) -> u8 {
    // Base DC from zone
    let mut dc = zone_base_difficulty as i16;
    
    // Adjust based on party average level
    let avg_level = player_party.iter().map(|s| s.level).sum::<u8>() as f32
        / player_party.len() as f32;
    
    // For every 2 levels above zone base, reduce DC by 1
    dc -= ((avg_level - zone_base_difficulty as f32) / 2.0) as i16;
    
    // Floor at 1
    dc.max(1) as u8
}

pub fn roll_mission_outcomes(
    party: &[SlimeState],
    difficulty_class: u8,
) -> AarOutcome {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let mut rolls = Vec::new();
    let mut success_count = 0;
    
    for slime in party {
        // D20 + (ATK stat) vs DC
        let roll = rng.gen_range(1..=20) + (slime.stats.atk as u16);
        let success = roll >= difficulty_class as u16;
        
        rolls.push(D20Result {
            slime_id: slime.id.clone(),
            roll_value: roll as u8,
            difficulty_class,
            success,
        });
        
        if success {
            success_count += 1;
        }
    }
    
    // Mission succeeds if majority of party rolls succeed
    let overall_success = success_count > party.len() / 2;
    
    AarOutcome {
        success: overall_success,
        rolls,
        // ... other fields
    }
}
```

**Rationale:**
- Early zones (difficulty 1-2) are winnable with 1 starter slime (ATK 8 + D20 likely beats DC 1-2)
- Party size (3) allows buffer for failures
- Leveling reduces DC relative to player power (natural progression)

---

### Task 5.2: First Mission Tuning

**Acceptance Criterion:** With 1 starter Red slime (stats: HP 15, ATK 8, DEF 6, AGI 7, INT 5), first mission (Volcano, difficulty 1) should have ~70-80% win rate.

**Calculation:**
- Starter ATK: 8
- D20 roll + 8 vs DC 1
- Probability: (20 possible rolls, almost all beat DC 1 - 8 = -7, which is always success)
- **Result: ~95% win rate (maybe too easy)**

**Adjustment:** Increase starter zone difficulty to 2-3 for slightly more tension.

---

## Phase E.6: Equipment Integration (Light)

### Task 6.1: Equipment Stats & Dispatch Impact

**Location:** `src/models.rs` → `Gear`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gear {
    pub id: String,
    pub name: String,
    pub stat_bonus: StatBonus,  // +2 ATK, +1 DEF, etc.
    pub cost: u16,
    pub slot: GearSlot,  // Weapon, Armor, Accessory
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatBonus {
    pub atk: Option<u8>,
    pub def: Option<u8>,
    pub hp: Option<u8>,
}

impl SlimeState {
    pub fn effective_atk(&self, equipped_gear: &[Gear]) -> u8 {
        let mut atk = self.stats.atk;
        for gear in equipped_gear {
            if let Some(bonus) = gear.stat_bonus.atk {
                atk += bonus;
            }
        }
        atk
    }
}
```

**Rationale:** Equipment is optional but helpful. Early game doesn't require it, but can tip close missions in player's favor.

---

## Phase E.7: Leveling & Progression

### Task 7.1: XP Grant on Mission Return

**Location:** `src/persistence.rs` → Mission completion

```rust
impl GameState {
    fn grant_mission_rewards(&mut self, outcome: &AarOutcome) -> Result<(), String> {
        let xp_reward = if outcome.success { 50 } else { 10 };  // Success grants more
        
        for slime_id in &self.party {
            if let Some(slime) = self.slimes.iter_mut().find(|s| s.id == slime_id) {
                slime.xp += xp_reward;
                
                // Level up if XP >= threshold
                while slime.xp >= slime.xp_to_level {
                    slime.xp -= slime.xp_to_level;
                    slime.level += 1;
                    
                    // Stat growth on level
                    slime.stats.hp += 2;
                    slime.stats.atk += 1;
                    slime.stats.def += 1;
                    
                    // Reset XP threshold (scales with level)
                    slime.xp_to_level = (slime.xp_to_level as f32 * 1.1) as u16;
                }
            }
        }
        
        Ok(())
    }
}
```

**Rationale:** Leveling provides steady power growth, makes later zones progressively easier to access.

---

## Test Floor

**Before:** 186 passing  
**Target:** 210 passing (24 new tests)

### New Tests Required:

1. `test_starting_slime_red_primary` — Verify game starts with 1 Red slime
2. `test_party_size_max_3` — Verify can't add 4th slime to party
3. `test_party_size_min_1` — Verify can't remove last slime
4. `test_zone_color_requirement_green` — Green slime satisfies Jungle zone requirement
5. `test_zone_color_requirement_mismatch` — Blue slime fails to satisfy Green requirement
6. `test_breeding_red_blue_purple` — Red + Blue produce Purple slime
7. `test_breeding_pattern_striped_chance` — Purple + Purple have high chance for Striped
8. `test_pattern_new_grants_regent` — First time pattern discovered grants regent
9. `test_regent_storage_and_use` — Use regent to force pattern on slime
10. `test_regent_consumed_after_use` — Regent count decreases after use
11. `test_mission_validates_color_requirement` — Dispatch with wrong color fails
12. `test_mission_success_unlocks_zone` — First successful dispatch unlocks zone permanently
13. `test_mission_failure_no_unlock` — Failed dispatch doesn't unlock zone
14. `test_difficulty_calculation_level_scaling` — Higher level party lowers effective DC
15. `test_first_mission_winnable_70_percent` — Starter slime vs. Volcano DC 2 has ~70% win rate
16. `test_equipment_bonus_atk` — Gear +2 ATK correctly adds to slime attack
17. `test_xp_grant_on_success` — Success mission grants 50 XP
18. `test_xp_grant_on_failure` — Failure mission grants 10 XP
19. `test_levelup_threshold` — Slime levels up when XP >= threshold
20. `test_stat_growth_on_levelup` — Leveling increases HP, ATK, DEF
21. `test_xp_threshold_scales_with_level` — XP to next level increases as player levels
22. `test_color_blending_red_yellow_orange` — Red + Yellow produce Orange
23. `test_pattern_tier_affects_rarity` — High tier parents increase rare pattern chance
24. `test_mission_completes_grants_rewards` — Mission completion grants XP + patterns

---

## Acceptance Criteria

✓ Game starts with 1 Red slime (Basic pattern)  
✓ Party limited to 3 slimes max  
✓ Breeding Red+Blue creates Purple slime (color blending works)  
✓ Pattern combos have weighted distribution (specific colors → specific patterns)  
✓ New patterns grant Pattern Regents (one-time discovery)  
✓ Regents can be used to force patterns (consumable)  
✓ Zones have color requirements (Jungle requires Green, etc.)  
✓ Can't dispatch to zone without required color  
✓ First successful dispatch unlocks zone permanently  
✓ Mission difficulty scales with party level  
✓ First mission (Volcano, difficulty 2) winnable with starter slime (~70% win rate)  
✓ Equipment provides stat bonuses  
✓ Winning mission grants XP + allows leveling  
✓ Leveling increases stats gradually (progression feels meaningful)  
✓ All 24 new tests passing  
✓ Test floor: 210 / 210 (zero regressions)  
✓ Code compiles to desktop + aarch64 + armv7 without warnings  

---

## Notes for Agent

- **Balance is empirical.** After implementation, run the game multiple times. First mission should succeed ~70% of the time (not 95%, not 30%).
- **Regent is a one-time reward.** Once used, it's gone. This incentivizes strategic breeding planning.
- **Zone unlock is permanent.** Once unlocked, zone stays available (encourages exploration).
- **Party size is hard cap.** Max 3; can't go higher without code change (deferred to post-MVP).
- **XP scaling:** Leveling should feel gradual but rewarding. Test that moving from level 1→2→3 feels like progression (harder zones become winnable).

---

## Deliverables

1. **Updated `src/persistence.rs`** — GameState: starting slime, regent system, zone tracking
2. **Updated `src/models.rs`** — Zone color requirements, Mission validation, Gear system
3. **Updated `src/genetics.rs`** — BreedingResolver pattern rolls (weighted by combo + tier)
4. **Updated `src/combat.rs`** — Mission difficulty calculation, outcome resolution
5. **Updated `src/world_map.rs`** — Zone definitions (Volcano, Jungle, Ocean, etc.) with color requirements
6. **New `tests/core_loop_balance.rs`** — 24 balance + progression tests
7. **APK build verification** — Test on Moto G (manual: start game, breed, dispatch, level up)

---

## Completion Checklist

- [ ] Starting slime created (Red, Basic, viable stats)
- [ ] Party size capped at 3
- [ ] Breeding outcomes implemented (color blending, pattern rolls)
- [ ] Pattern Regent system (grant on discovery, consume on use)
- [ ] Zone color requirements defined (Red, Blue, Yellow, secondary colors)
- [ ] Mission color validation (can't dispatch without required color)
- [ ] Zone unlock on first success (permanent)
- [ ] Mission difficulty scales with party level
- [ ] First mission tuned to ~70% win rate
- [ ] Equipment stat bonuses working
- [ ] XP/leveling system functional (stat growth on level)
- [ ] All 24 tests passing
- [ ] Test floor: 210 / 210
- [ ] APK builds and runs on Moto G
- [ ] Core loop verified (recruit → breed → dispatch → level → repeat)
