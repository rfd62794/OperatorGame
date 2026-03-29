# OPERATOR — Feature Specification (SPEC) [DEPRECATED]
> **Status:** [DEPRECATED] | 2026-03-28
> **Note:** This document is significantly stale and reflects the legacy "Human Mercenary" version of the project. 
> **Authoritative Source:** Refer to [GAME_DESIGN.md](file:///c:/Github/OperatorGame/docs/GAME_DESIGN.md) v2.0 for the current "Corporate-Absurdist" vision and core systems.
> A full rewrite of the technical specification is scheduled for Sprint G.9.

---

## 1. Domain Entities

### 1.1 Operator (Human Mercenary)

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Uuid` | Stable unique identifier |
| `name` | `String` | Display name |
| `job` | `Job` | `Breacher \| Infiltrator \| Analyst` |
| `base_strength` | `u32` | 1–100 |
| `base_agility` | `u32` | 1–100 |
| `base_intelligence` | `u32` | 1–100 |
| `state` | `OperatorState` | `Idle \| Deployed(mission_id) \| Injured(until: DateTime<Utc>)` |

**`effective_stats()`** adds job bonus to base: `Breacher` +10 STR, `Infiltrator` +10 AGI, `Analyst` +10 INT.

---

### 1.2 Mission

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Uuid` | Stable unique identifier |
| `name` | `String` | Display name |
| `req_strength` | `u32` | Required effective STR from squad |
| `req_agility` | `u32` | Required effective AGI from squad |
| `req_intelligence` | `u32` | Required effective INT from squad |
| `difficulty` | `f64` | 0.0–0.9 penalty scalar |
| `duration_secs` | `u64` | Wall-clock seconds to completion |
| `reward` | `u64` | Credits awarded on Victory |

---

### 1.3 Deployment

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Uuid` | Deployment identifier |
| `mission_id` | `Uuid` | Which mission |
| `operator_ids` | `Vec<Uuid>` | Assigned squad (1–3) |
| `completes_at` | `DateTime<Utc>` | Absolute wall-clock completion time |
| `resolved` | `bool` | AAR collected flag |

**`is_complete()`**: `Utc::now() >= completes_at`

---

### 1.4 SlimeGenome (Genetic Entity)

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Uuid` | Stable unique identifier |
| `name` | `String` | Display name |
| `culture_expr` | `CultureExpression` | 6-float vector, sums to 1.0 |
| `base_hp` | `f32` | Hit points (20.0 × culture_mod × 0.85–1.15) |
| `base_atk` | `f32` | Attack (5.0 × culture_mod) |
| `base_spd` | `f32` | Speed (5.0 × culture_mod) |
| `level` | `u8` | 0–10 |
| `xp` | `u32` | Current XP toward next level |
| `generation` | `u32` | Breeding depth (starts at 1) |
| `parent_ids` | `Option<[Uuid; 2]>` | Lineage tracking |
| `shape` | `Shape` | Visual enum |
| `body_size` | `BodySize` | `Tiny/Small/Medium/Large/Massive` |
| `pattern` | `Pattern` | Visual enum |
| `accessory` | `Accessory` | `None/Crown/Scar/Glow/Shell/Crystals` |
| `base_color` | `[u8; 3]` | RGB |
| `curiosity / energy / affection / shyness` | `f32` | 0.0–1.0 personality axes |

**`genetic_tier()`**: derived from `CultureExpression` — see §3.
**`life_stage()`**: derived from `level` — see §4.

---

### 1.5 CultureExpression

`[f32; 6]` indexed as `[Ember, Gale, Marsh, Crystal, Tundra, Tide]`.
**Invariant:** `values.iter().sum() ≈ 1.0` (enforced on construction and after every breed).

**`dominant()`**: returns the `Culture` with the highest expression value.
**`active_count()`**: cultures with expression ≥ 0.05.

---

### 1.6 GameState (Persistence Root)

```json
{
  "roster":      [Operator, ...],
  "bank":        u64,
  "deployments": [Deployment, ...],
  "missions":    [Mission, ...],
  "slimes":      [SlimeGenome, ...]   // Sprint 1 — was missing in Python
}
```

`slimes` uses `#[serde(default)]` — missing on old saves → empty `Vec` (backward compatible).

---

## 2. Success Formula (Operator Missions)

```
per_attr_score   = min(squad_total / threshold, 1.0)
                   [0.0 if threshold > 0 and squad provides zero]
average_score    = (str_score + agi_score + int_score) / 3.0
success_chance   = average_score × (1.0 − difficulty)
```

| Roll vs. success_chance | Outcome |
|------------------------|---------|
| `roll < success_chance` | **Victory** → `bank += reward`; all operators → `Idle` |
| `roll ≥ success_chance AND roll < 0.95` | **Failure** → all operators `Injured(now + duration × 2s)` |
| `roll ≥ 0.95` | **Critical Failure** → one random operator removed from roster |

> **5% floor is permanent.** Cannot be reduced by statistics.

---

## 3. Genetic Tier Resolution

Tier is computed from the number of **active cultures** (`expression ≥ 0.05`) and their hexagon relationship.

```
active_count == 1                          → Tier 1  Blooded
active_count == 2, cultures are adjacent   → Tier 2  Bordered
active_count == 2, cultures are opposite   → Tier 3  Sundered
active_count == 2, cultures skip-one       → Tier 4  Drifted
active_count == 3                          → Tier 5  Threaded
active_count == 4                          → Tier 6  Convergent
active_count == 5                          → Tier 7  Liminal
active_count == 6                          → Tier 8  Void
```

**Hex adjacency map:**
```
Ember → [Gale, Marsh]      Crystal → [Gale, Tide]
Gale  → [Ember, Tundra]    Marsh   → [Ember, Tide]
Tide  → [Crystal, Marsh]   Tundra  → [Gale, Marsh]
```

**Opposites:** Ember↔Crystal, Gale↔Tundra, Marsh↔Tide

---

## 4. LifeStage Gate

| Level | Stage | can_dispatch | can_breed | can_mentor |
|-------|-------|:---:|:---:|:---:|
| 0–1 | Hatchling | ✗ | ✗ | ✗ |
| 2–3 | Juvenile | ✓ | ✗ | ✗ |
| 4–5 | Young | ✓ | ✓ | ✗ |
| 6–7 | Prime | ✓ | ✓ | ✗ |
| 8–9 | Veteran | ✓ | ✓ | ✗ |
| 10 | Elder | ✓ | ✓ | ✓ |

**XP curve:** `xp_to_next = (level + 1) × 100`

---

## 5. Breeding Resolution (`BreedingResolver::breed`)

Error if either parent's `life_stage().can_breed() == false`.

**Step 1 — Culture blending:**
```
for each culture i:
    blended    = (a.expr[i] + b.expr[i]) / 2.0
    variance   = uniform(−0.15, +0.15)
    raw[i]     = max(0.0, blended + variance × blended)
normalise(raw) so sum == 1.0
```

**Step 2 — Stat inheritance (three rules + ratchet):**
```
HP  = ratchet(max(a.hp,   b.hp),                cap, mutation)
ATK = ratchet((a.atk + b.atk) / 2.0,            cap, mutation)
SPD = ratchet(max(a.spd, b.spd) * 0.95,          cap, mutation)

cap      = base_stat × culture_modifier × 2.0
ratchet  = current + (cap − current) × 0.10
           then maybe mutate: 70% → ×1.25, 30% → ×0.85
           then clamp to cap
```

Default mutation chance = **5%**. Void parentage forces ≥ **15%**.

**Step 3 — Visual dominance:**
- Dominant parent = higher peak culture expression
- 80% chance dominant's shape/pattern used; 20% recessive
- 10% chance one RGB channel ±30 color mutation
- Elder bonus: +20% chance rare accessory even if roll said `None`

**Step 4 — Personality:**
- Each of `[curiosity, energy, affection, shyness]`: `avg ± uniform(−0.10, +0.10)`, clamped `[0.0, 1.0]`

**Output:** New `SlimeGenome` with `generation = max(a, b) + 1`, `level = 0`, `xp = 0`.

---

## 6. Narrative Engine (`log_engine::generate_narrative`)

Classifies mission by dominant stat requirement:

| Dominant stat | `MissionType` | Narrative pool |
|--------------|---------------|----------------|
| STR | Assault | 5 "breaching" templates |
| AGI | Stealth | 5 "shadow" templates |
| INT | Cyber | 5 "hacking" templates |
| Balanced | Balanced | 4 general templates |

`{op}` token → first operator's name.
`format_log_entry` wraps with `[mission_name] OUTCOME — narrative`.
Log capped at 50 entries (newest first). UI: resizable scrollable panel, color-coded by outcome.

---

## 7. Persistence Contract

- **Save file:** `save.json` (working directory)
- **Write strategy:** atomic → `.json.tmp` → `fs::rename` (see ADR-003)
- **Load policy:** absent file → fresh `GameState::new_with_seed_missions()`
- **Corrupt file:** surface `PersistenceError::Json` — never silently overwrite

---

## 8. CLI Commands

| Command | Description |
|---------|-------------|
| `operator roster` | List all operators and state |
| `operator hire <name> <job>` | Add operator |
| `operator missions` | Show all contracts |
| `operator deploy <mission_prefix> <op1> [op2] [op3]` | Assemble and deploy squad |
| `operator aar` | Resolve completed deployments |
| `operator status` | Show bank + active ops |
| `operator gui` | Open egui War Room window |
| `operator slimes` | List slime stable |
| `operator hatch <name> <culture>` | Seed new slime |
| `operator splice <id_a> <id_b> <offspring_name>` | Breed two slimes |

---

## 9. Test Coverage (Current)

| Module | Tests | Status |
|--------|-------|--------|
| `models` | 8 | ✅ |
| `persistence` | 3 | ✅ |
| `log_engine` | 3 | ✅ |
| `genetics` | 9 | ✅ (Sprint 1) |
| `cli` | 0 | manual smoke only |
| `ui` | 0 | visual only |
| **Total** | **23** | **23 / 23** |

> Note: `cargo test` reports 24 — one extra from the binary test harness.

---

## 10. Acceptance Criteria

### Tiers 1–2
- [x] `calculate_success_rate()` returns value in `[0.05, 1.0]`
- [x] `is_complete()` is wall-clock safe
- [x] Atomic save prevents corrupt state
- [x] All operators persisted across restart

### Tier 3
- [x] `operator gui` opens War Room window
- [x] Progress bars animate without a background thread
- [x] AAR resolves in GUI and saves atomically

### Sprint 1
- [x] `CultureExpression` always sums to 1.0
- [x] Ratchet never exceeds stat cap
- [x] Hatchlings cannot breed
- [x] `operator hatch` and `operator splice` persist to `save.json`
- [x] Old `save.json` files without `slimes` field load cleanly
