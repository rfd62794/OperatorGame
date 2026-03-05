# UNIFIED SYSTEMS MAP — rpgCore v2 / OPERATOR
> **Version:** 1.0 | **Date:** 2026-03-04
> **Narrative Frame:** The Astronaut's crashed ship console (OPERATOR) governs the Slime Planet ecosystem (rpgCore).
> This document is the cross-project audit. It maps every system in rpgCore to its OPERATOR counterpart or integration point.

---

## The Architecture in One Diagram

```
                    ┌─────────────────────────────────────────────────────────────┐
                    │              COMMAND DECK (OPERATOR — Rust)                 │
                    │                                                             │
                    │  War Room GUI ←── Narrative Log ←── AAR Resolution         │
                    │       │                                    │                │
                    │  [Operators]    [Slime Stable]    [Bank / Resources]       │
                    └───────┬─────────────────┬─────────────────────────────────┘
                            │                 │
              ┌─────────────┘                 └─────────────────┐
              ▼                                                   ▼
┌─────────────────────────┐                   ┌──────────────────────────────┐
│  LAYER A: HUMAN OPS     │                   │  LAYER B: GENETICS ENGINE    │
│  (Mercenary Dispatch)   │                   │  (Slime Breeding Program)    │
│                         │                   │                              │
│  Operators → Missions   │                   │  Hatch → Level → Splice      │
│  D20 Resolution (S2)    │                   │  Culture Hex-Wheel           │
│  Risk: 5% KIA floor     │                   │  GeneticTier 1→8             │
│  Reward: Credits        │                   │  Ratchet Effect (no regress) │
└──────────┬──────────────┘                   └──────────────┬───────────────┘
           │                                                   │
           └──────────────┬────────────────────────────────────┘
                          ▼
           ┌──────────────────────────────────────────┐
           │     LAYER C: ISLAND EXPEDITION (S3)      │
           │     (Multi-node roguelike floors)        │
           │                                          │
           │  Slime culture → Zone bonus (Advantage)  │
           │  Operator job → Node type bonus          │
           │  DungeonEngine: COMBAT/REST/TRAP/BOSS    │
           └──────────────────────────────────────────┘
```

---

## Section 1 — Full rpgCore Repository Inventory

### 1.1 Application Layer (`src/apps/`)

| App | Status | Genre | Description |
|-----|--------|-------|-------------|
| **slime_breeder** | development | simulation | The main breeding garden: genetics, racing, sumo, dungeon path, tower defense |
| **dungeon_crawler** | development | rpg | Standalone REPL + Pygame dungeon: HUB → floor → combat → loot loop |
| **slime_clan** | playable | strategy | Faction territory simulation. Slime armies claim hexagonal grid tiles |
| **asteroids** | playable | action | Roguelike Asteroids with AI trainer and physics simulation |
| **last_appointment** | playable | narrative | Conversation graph demo (Death interviewing a client) |
| **space_trader** | development | simulation | Economy simulation — buy low, sell high across routes |
| **turbo_shells** | stub | management | Turtle breeding and racing (genetics + season management) |
| **space** | — | — | Stub/unnamed |
| **tycoon** | — | — | Stub/unnamed |
| **interface** | — | — | Shared UI launcher |
| **dgt_launcher.py** | — | — | 29KB DGT-based launcher entry point |

---

### 1.2 Shared Systems Inventory (`src/shared/`)

#### 🧬 Genetics (`src/shared/genetics/`) — **FULLY PORTED TO RUST**

| File | Purpose |
|------|---------|
| `genome.py` (9.9KB) | `SlimeGenome` dataclass — all visual, personality, cultural, and base-stat fields |
| `inheritance.py` (6.2KB) | `generate_random()` + `breed()` — stat inheritance rules, mutation logic |
| `breeding_system.py` (9.3KB) | `BreedingSystem` — culture blending, visual dominance, personality blend |
| `cultural_base.py` (3.5KB) | `CulturalBase` enum + `CulturalParameters` — stat modifiers per culture |
| `cultural_archetypes.py` (337B) | Thin archetype alias layer |
| `entity_template.py` (4.0KB) | `SlimeEntityTemplate` — migration/validation wrapper |

**Port status:** `src/genetics.rs` in OPERATOR is a faithful high-fidelity transplant. Python version has no persistence (confirmed gap). Rust version fixes this via `serde`.

---

#### 🗡️ Combat (`src/shared/combat/`)

| File | Purpose |
|------|---------|
| `d20_resolver.py` (4.9KB) | `D20Resolver` — roll_standard, roll_with_advantage/disadvantage, resolve_attack (hit/crit), check vs DC |
| `stance.py` (2.4KB) | `CombatStance` (AGGRESSIVE/DEFENSIVE/FLEEING) — HP% triggers, stat multipliers |
| `turn_order.py` (2.9KB) | `TurnOrderManager` — SPD-sorted initiative, combatant add/remove/cycle |

**Key formulas:**
```
roll_standard()     = d20 (1–20)
roll_with_advantage = max(d20, d20)
resolve_attack(roll, atk, defense):
    → hit if  roll + atk > defense
    → crit if roll == 20 (natural)
    → miss if roll == 1  (natural)
```

**Sprint 2 target:** Transplant D20Resolver + stance system to `src/combat.rs`.

---

#### 📊 Stats (`src/shared/stats/`)

| File | Purpose |
|------|---------|
| `stat_block.py` (8.9KB) | `StatBlock` — layered 6-stat system (HP/ATK/SPD/MND/RES/CHM) with culture weights, equipment modifiers, training XP growth factors, stage scaling |
| `culture_table.py` (1.0KB) | Culture → stat weight lookup |

**The 6-Stat System (extends Rust's current 3):**
```
HP  (vit)  ← base_hp  × stage_mod + culture_hp + equipment_hp
ATK (pwr)  ← base_atk × stage_mod + culture_atk + equipment_atk
SPD (agi)  ← base_spd × stage_mod + culture_spd + equipment_spd
MND (mnd)  ← (curiosity×8 + energy×4) × stage_mod + culture_mnd    [NEW]
RES (res)  ← ((1-shyness)×8 + energy×2) × stage_mod + culture_res  [NEW]
CHM (chm)  ← (affection×8 + (1-shyness)×4) × stage_mod + culture_chm [NEW]
```
**Culture weights (from `stat_block.py`):**
| Culture | ATK | HP | SPD | MND | RES | CHM |
|---------|-----|----|-----|-----|-----|-----|
| Ember   | +3.0| +0.5| +0.5| -0.5| -0.5| 0.0|
| Gale    | +0.5| +0.5| +3.0| +1.0| 0.0| 0.0|
| Marsh   | +0.5| +3.0| +0.5| -0.5| +0.5| +1.5|
| Crystal | +1.0| +1.0| +1.0| +2.0| +1.5| +0.5|
| Tundra  | +0.5| +2.0| -1.0| +0.5| +2.0| -0.5|
| Tide    | +2.0| +0.5| +0.5| +0.5| -0.5| +2.5|
| Void    | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |

**Stage modifiers:**
```
Hatchling (≤1): ×0.6 | Juvenile (≤3): ×0.8 | Young (≤5): ×1.0
Prime     (≤7): ×1.2 | Veteran  (≤9): ×1.1 | Elder (10): ×1.0
```

---

#### 🧪 Entities (`src/shared/entities/`)

| File | Purpose |
|------|---------|
| `creature.py` (11.5KB) | `Creature` — unified entity across ALL demos. Genome + physics kinematics + progression + 6 stat XP pools + team role + garden wander AI |
| `fracture.py` (5.5KB) | Particle/shatter effect on death |
| `kinetics.py` (4.6KB) | Kinetic body state |
| `projectiles.py` (4.9KB) | Projectile entity |
| `game_state.py` (1.4KB) | Macro game state holder |
| `spawner_base.py` (1.3KB) | Abstract spawner |

**The Creature pipeline:**
```
SlimeGenome → StatBlock.from_slime(creature) → used by ALL scenes
             ↕ training produces vit_xp, pwr_xp, agi_xp, mnd_xp, res_xp, chm_xp
             ↕ which feeds back into StatBlock growth factors [0.8 → 1.5]
```

---

#### 🗺️ Dispatch System (`src/shared/dispatch/`)

| File | Purpose |
|------|---------|
| `dispatch_system.py` (10.3KB) | `DispatchSystem` — async tick-based dispatch, zone resolution, squad power calc |
| `zone_types.py` (3.3KB) | `ZoneType` enum + `ZoneConfig` — 6 mission types with stage requirements, risk, resources, duration |
| `dispatch_record.py` (3.2KB) | `DispatchRecord` — individual dispatch state (id, zone, return_tick, status) |

**The 6 Zone Types (direct blueprint for OPERATOR zones):**

| Zone | Min Stage | Risk | Primary Resource | Duration | Target Stat Growth |
|------|-----------|------|-----------------|----------|--------------------|
| RACING | Juvenile | Low | Gold | 300 ticks | Dexterity/Speed |
| DUNGEON | Young | High | Scrap | 600 ticks | Strength/Defense |
| FORAGING | Juvenile | Low | Food | 400 ticks | Constitution |
| TRADE | Young | Standard | Gold | 500 ticks | Charisma |
| MISSION | **Prime** | Standard | Gold+Scrap | 800 ticks | Intelligence |
| ARENA | Young | Standard | Gold | 450 ticks | Strength/Defense |

**Statistical resolution formula:**
```
base_success = {none:1.0, low:0.9, standard:0.7, high:0.5, critical:0.3}[risk]
success_rate = clamp(base_success × (0.5 + squad_power), 0.10, 0.95)
squad_power  = mean(level×0.1 + atk×0.02 + hp×0.01 + spd×0.02 + tier×0.05)
```

---

#### 🏰 Dungeon Engine (`src/shared/dungeon/`)

| File | Purpose |
|------|---------|
| `dungeon_engine.py` (3.3KB) | `DungeonEngine` — scrolling party along track, zone event triggers |
| `dungeon_track.py` (2.7KB) | `DungeonTrack` + `DungeonZone` — track with typed zone segments |
| `enemy_squads.py` (4.0KB) | Pre-built enemy squad compositions |

**Zone event types:** COMBAT → pauses party | REST → heals 10% HP after 2s | TRAP → instant effect | TREASURE → pause + loot | BOSS → final encounter

---

#### 📈 Progression (`src/shared/progression/`)

| File | Purpose |
|------|---------|
| `xp_system.py` (7.3KB) | `award_xp()` — 11 activities, culture amplification, training efficiency decay, critical breakthrough |

**XP Activity Table:**

| Activity | Total XP | Primary Stat XP |
|----------|----------|----------------|
| Sumo win | 50 | vit+30, pwr+20, res+10 |
| Race win | 60 | agi+50, mnd+10 |
| Dungeon room clear | 30 | pwr+20, mnd+10 |
| Dungeon boss clear | 80 | pwr+40, mnd+20 |
| Foraging return | 20 | agi+10, mnd+15 |
| Garden idle (daily) | 5 | mnd+3, res+2, chm+3 |
| Player interaction | 10 | chm+15 |

**Training efficiency decay:** `eff = max(0.7, 1.0 - (stat_xp / 200.0) × 0.3)` — resets on stage advance.
**Critical breakthrough:** 5–10% chance of 2× XP on culture-matched activity.

---

#### 🏎️ Racing (`src/shared/racing/`)

| File | Purpose |
|------|---------|
| `race_engine.py` (8.5KB) | `RaceEngine` — physics simulation, terrain speed modifiers, jump mechanics |
| `race_track.py` (4.2KB) | `generate_track()`, `generate_zones()` — procedural terrain (GRASS/WATER/ROCK/MUD) |
| `movement_profiles.py` (1.6KB) | Per-culture race movement profiles |
| `racing_session.py` (4.2KB) | Persistent session state |
| `race_camera.py` (2.0KB) | Rubber-band camera |
| `race_hud.py` (4.3KB) | HUD overlay |
| `minimap.py` (3.8KB) | Track minimap |

**Race mechanics:**
```
velocity       ← base_spd + terrain_modifier
jump_height    ← agi stat — used for obstacle/jump mechanics
terrain effect: WATER → -30% spd | MUD → -20% spd | ROCK → random stumble
Results:       rank 1→4 returned; winner gets 60 txp + 50 agi_xp
```

---

#### ⚔️ Sumo (`scene_sumo.py`)

**Combat formula:**
```
score = (atk × 0.5) + (hp × 0.3) + (spd × 0.2)
if culture_advantage: score × 1.15
```

**Culture advantage (two RPS triangles):**
```
Aggressive: Ember > Gale > Tundra > Ember
Tactical:   Tide > Marsh > Crystal > Tide
Void:       no advantages or weaknesses
```

**XP on result:** Winner +50 txp/pwr/vit | Loser +20 txp (consolation)

---

#### 🏰 Tower Defense (`scene_tower_defense.py`)

**ADR-008: "Slimes Are Towers"**
```
Slimes placed on 10×10 grid → become tower entities via ECS
TowerComponent: damage, range, fire_rate (upgradeable with Gold)
WaveSystem:     procedural enemy waves with path following
ECS Systems:    TowerBehaviorSystem + CollisionSystem + UpgradeSystem
Culture bonus:  planned (Ember towers → more ATK; Tide towers → more range)
```

**Session feedback:** Tower defense XP feeds back into roster via `end_tower_defense_session()`.

---

#### 🗡️ Dungeon Crawler (Full REPL — `run_dungeon_crawler.py`)

**State machine:**
```
HUB (The Room) → CRAWLING → COMBAT → LOOTING → HUB
                    │
                    └── escape (lose gold/items) | ascend (keep all, boss cleared)
```

**The Room** — persistent hub with chest (safe storage), escape ropes (consumable), and banking.
**Combat:** D20 to-hit versus DEF, natural 20 = critical (×2 damage), natural 1 = miss. TurnOrderManager (SPD-sorted initiative).
**Death penalty:** lose all backpack items and run gold. Return to hub revived.
**Loot:** `LootTable` with weighted random drops scaled by floor depth.

---

#### 🌐 World (`src/shared/world/`)

| File | Purpose |
|------|---------|
| `faction.py` (5.5KB) | `FactionManager` — territory grid simulation: ALLIED/NEUTRAL/HOSTILE/WAR relations, aggression-based expansion |

**Slime Clan integration:** Factions are slime armies, each culture maps to a faction archetype (Ember → Military, Crystal → Economic, Tide → Religious, etc.)

---

#### 📖 Narrative (`src/shared/narrative/`)

| File | Purpose |
|------|---------|
| `conversation_graph.py` (2.8KB) | `ConversationGraph` — branching dialogue with keyword routing |
| `keyword_registry.py` (1.2KB) | Keyword → response mapping |
| `state_tracker.py` (739B) | Narrative state persistence |

Used by: **Last Appointment** (Death interview); blueprint for future Crashed Astronaut monologue.

---

#### 🎒 Items (`src/shared/items/`)

| File | Purpose |
|------|---------|
| `item.py` (929B) | `Item` dataclass — id, name, type, slot, stat_modifiers, value, identified |
| `inventory.py` (2.7KB) | `Inventory` — equipped slots + backpack (capacity-limited) |
| `loot_table.py` (1.7KB) | `LootTable` — weighted random drops, depth-scaled |

**Item stat_modifiers** map directly to StatBlock fields. Equipment layer = `StatBlock.with_equipment(hp, atk, spd, mnd, res, chm)`.

---

#### 🤝 Teams (`src/shared/teams/`)

| File | Purpose |
|------|---------|
| `roster.py` (26.5KB) | `Roster` — master entity registry. `TeamRole` enum (RACING/DUNGEON/SUMO/TOWER/UNASSIGNED). Team assignment, reference layer, creature lookup |
| `stat_calculator.py` (1.6KB) | `calculate_hp(genome, level)` — HP from base × culture × level |
| `roster_save.py` (521B) | `load_roster()` / `save_roster()` — JSON wrapper |

**Team roles are exclusive** — a slime can only be on one team at a time. `locked = True` when dispatched.

---

#### 💾 Persistence (`src/shared/persistence/`)

| File | Purpose |
|------|---------|
| `save_manager.py` (3.8KB) | `SaveManager` — load/save JSON with optional path. No atomic write (persistence gap). |

**Gap:** Python save is not atomic — power failure can corrupt. OPERATOR's Rust persistence (`.tmp` → rename) is strictly better.

---

#### ⚙️ Physics (`src/shared/physics/`)

| File | Purpose |
|------|---------|
| `kinematics.py` (2.3KB) | `Kinematics` + `Vector2` — position/velocity with Euler integration |
| `gravity.py` (757B) | Constant gravity for jump physics |
| `toroidal.py` (273B) | Wrap-around bounds (used in Asteroids) |

---

#### 🎭 Simulation (`src/shared/simulation/`)

| File | Purpose |
|------|---------|
| `base_engine.py` (1.1KB) | Abstract engine base |
| `base_track.py` (1.2KB) | Abstract track/layout base |

---

### 1.3 DGT Engine Inventory (`src/dgt_engine/`)

21-subsystem game engine (29KB `main.py`). The engine that rpgCore `shared/` was extracted from.

| Subsystem | Notable Contents |
|-----------|-----------------|
| `dgt_core/` | Core game loop, scene management |
| `game_engine/` | Engine tick, update pipeline |
| `mechanics/` | Action resolvers, dice systems |
| `models/` | Domain model layer |
| `narrative/` | Story system (pre-shared/ extraction) |
| `systems/` | Entity systems |
| `ui/` | DGT UI framework |
| `graphics/` | Rendering layer |
| `logic/` | Game logic layer |
| `foundation/` | Base types, interfaces |
| `interfaces/` | Abstract contracts |
| `di/` | Dependency injection |
| `config/` | Configuration loading |
| `common/` | Shared utilities |
| `tools/` | Dev tooling |
| `views/` | View layer |
| `vector_libraries/` | Math vectors |
| `assets/` | Asset loading |
| `__compat__.py` | DGT→shared compatibility shim (4.2KB) |
| `c_style_facades.py` | C-style API facade (19.2KB) |
| `factories.py` | DI factory registrations |

---

## Section 2 — The Unified Systems Map (Crash Site → Planet)

### The "Maturity to Resource" Pipeline

```
Slime lifecycle feeding the Ship's progression:

HATCHLING (L0-1)
    ↓ Garden idle daily XP (+5 txp, +2 vit, +3 mnd, +2 res, +3 chm)
    ↓ Player interaction (+10 txp, +15 chm)
JUVENILE (L2-3) — can dispatch on RACING / FORAGING
    ↓ Foraging → Food resource (powers life support?)
    ↓ Racing  → Gold resource (parts salvage?)
YOUNG (L4-5) — can breed + dispatch ARENA / TRADE / DUNGEON
    ↓ Sumo Arena → Gold + combat stat training
    ↓ Trade runs → Gold + charisma growth
    ↓ Dungeon    → High-risk Scrap (ship repair materials)
PRIME (L6-7) — highest stage modifier (×1.2)
    ↓ Can dispatch MISSION zones (most valuable returns)
    ↓ Best breeding window (Ratchet at peak performance)
VETERAN (L8-9) — high risk dispatch
    ↓ Tower Defense use drops stat efficiency
    ↓ Breeding still active, Elder approaching
ELDER (L10) — can MENTOR offspring
    ↓ Retire to mentor role (+20% accessory chance on offspring)
    ↓ Entering dungeon at this stage is maximal risk / reward
```

### The Demo-to-Mission Integration Map

| rpgCore Demo | OPERATOR Mission Type | Dispatch Zone | Stat Growth | Narrative Frame |
|-------------|----------------------|---------------|-------------|-----------------|
| **Race** | Scouting Run | RACING | AGI/SPD | Slime scouts terrain ahead of crew |
| **Sumo** | Territory Dispute | ARENA | ATK/HP | Local fauna contest — claim crash site |
| **Tower Defense** | Defend the Crash Site | MISSION (special) | ATK/RES | Slimes hold perimeter vs wave fauna |
| **Dungeon Path** | Excavation Run | DUNGEON | ATK/MND | Deep-zone for ship components (Scrap) |
| **Foraging** | Biome Survey | FORAGING | MND/CHM | Passive resupply, food production |
| **Trade** | Market Liaison | TRADE | CHM | Slime as "diplomat" to local factions |
| **Slime Clan** | Faction Campaign | — | — | Territory simulation, political map |

### The Tech Tree ↔ Genetic Tier Map

```
SHIP RESTORATION PROGRESSION (narrative: Crashed Astronaut)

TIER 1 — Blooded slime (pure breed)
    → Restores: Emergency lights, basic sensors
    → Unlocks: FORAGING dispatches (food production)

TIER 2 — Bordered (2 adjacent cultures)
    → Restores: Water recycler (requires Marsh + any adjacent culture)
    → Unlocks: RACING dispatches (terrain mapping)

TIER 3 — Sundered (2 opposite = high energy)
    → Restores: Comm Array power (Ember × Crystal opposition = voltage)
    → Unlocks: Contact Other Survivors (narrative event)

TIER 4 — Drifted (2 skip-one)
    → Restores: Navigation AI core
    → Unlocks: DUNGEON dispatches (excavation for hull plating)

TIER 5 — Threaded (3 cultures)
    → Restores: Engine thruster #1
    → Unlocks: ARENA and MISSION dispatches

TIER 6 — Convergent (4 cultures)
    → Restores: Full propulsion
    → Unlocks: Expedition to other island biomes

TIER 7 — Liminal (5 cultures)
    → Restores: Jump drive capacitors
    → Unlocks: Slime Clan faction campaigns

TIER 8 — Void (all 6 cultures)
    → Restores: The Ship — ENDGAME
    → Unlocks: Depart the planet (roguelike "ascension")
```

---

## Section 3 — What Is NOT Yet Ported to Rust

| System | rpgCore Location | Priority |
|--------|-----------------|----------|
| D20Resolver | `shared/combat/d20_resolver.py` | ✅ Sprint 2 |
| DungeonEngine (scroll + zones) | `shared/dungeon/dungeon_engine.py` | ✅ Sprint 3 |
| LootTable | `shared/items/loot_table.py` | 🔄 Sprint 3 |
| StatBlock (6-stat layered) | `shared/stats/stat_block.py` | 🔄 Sprint 2+ |
| CombatStance | `shared/combat/stance.py` | 🔄 Sprint 2 |
| TurnOrderManager | `shared/combat/turn_order.py` | 🔄 Sprint 2 |
| XP system (11 activities) | `shared/progression/xp_system.py` | 🔄 Sprint 2+ |
| ZoneType dispatch | `shared/dispatch/dispatch_system.py` | 🔄 Sprint 3 |
| FactionManager | `shared/world/faction.py` | ⬜ Future |
| ConversationGraph | `shared/narrative/conversation_graph.py` | ⬜ Future |
| RaceEngine (physics) | `shared/racing/race_engine.py` | ⬜ Post-S3 |
| TowerDefense ECS | `shared/ecs/` | ⬜ Post-S3 |
| InventorySystem | `shared/items/inventory.py` | ⬜ Future |

---

## Section 4 — Sprint Integration Targets

### Sprint 2: D20 Combat Core
Port from `shared/combat/d20_resolver.py` + `stance.py` + `turn_order.py` to `src/combat.rs`.
Replace OPERATOR's flat `success_chance` with per-operator D20 rolls.

### Sprint 3: Island Expedition
Port from `shared/dungeon/dungeon_engine.py` → `src/world_gen.rs`.
Culture zone bonus = slime's dominant culture matches floor element → Advantage.

### Sprint 4: Extended Stats
Port `shared/stats/stat_block.py` 6-stat system → extend `SlimeGenome` in `src/genetics.rs`.
Add MND/RES/CHM to culture expression weight table.

### Sprint 5: Dispatch Zones
Port `shared/dispatch/zone_types.py` → `src/dispatch.rs`.
RACING/FORAGING/TRADE as passive off-screen dispatches alongside human operator missions.

---

## Section 5 — The "Crashed Ship" UX Rename Map

| Current UI Label | Proposed "Command Deck" Label |
|-----------------|-------------------------------|
| "War Room" | "Command Deck" |
| "Roster" | "Crew Manifest" |
| "Hire" | "Recruit Operative" |
| "Slime Stable" | "Bio-Incubator Archive" |
| "Hatch" | "Synthesize Sample" |
| "Splice" | "Genetic Fusion Protocol" |
| "Missions" | "Planetary Contracts" |
| "Deploy" | "LAUNCH EXPEDITION" |
| "AAR" | "Debrief Report" |
| "Bank" | "Ship Reserves" |
| "KIA" | "Status: LOST" |
