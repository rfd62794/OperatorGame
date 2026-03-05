# OPERATOR — Feature Specification
> **Status:** DRAFT v1.0 | 2026-03-04
> **Tier:** 1 (Headlong CLI Prototype) + Tier 2 (Persistence)

---

## 1. Domain Entities

### 1.1 Operator
| Field | Type | Description |
|-------|------|-------------|
| `id` | `Uuid` | Stable unique identifier |
| `name` | `String` | Display name |
| `job` | `Job` | Enum: `Breacher`, `Infiltrator`, `Analyst` |
| `strength` | `u8` | 1–100 |
| `agility` | `u8` | 1–100 |
| `intelligence` | `u8` | 1–100 |
| `state` | `OperatorState` | `Idle \| Deployed(mission_id) \| Injured(cooldown_until)` |

**Job Bonuses (applied at squad assembly):**
- `Breacher` → +10 effective Strength
- `Infiltrator` → +10 effective Agility
- `Analyst` → +10 effective Intelligence

### 1.2 Mission
| Field | Type | Description |
|-------|------|-------------|
| `id` | `Uuid` | Stable unique identifier |
| `name` | `String` | Display name |
| `strength_threshold` | `u8` | Required effective Strength |
| `agility_threshold` | `u8` | Required effective Agility |
| `intelligence_threshold` | `u8` | Required effective Intelligence |
| `difficulty` | `f64` | 0.0 (easy) → 0.9 (insane). Scalar penalty on success%. |
| `duration_secs` | `u64` | Wall-clock seconds until completion (30–300 for MVP) |

### 1.3 Deployment
| Field | Type | Description |
|-------|------|-------------|
| `mission_id` | `Uuid` | Which mission |
| `operator_ids` | `Vec<Uuid>` | Assigned squad |
| `completes_at` | `DateTime<Utc>` | Absolute wall-clock completion timestamp |
| `resolved` | `bool` | Has the AAR been collected? |

---

## 2. Success Formula

```
success_chance = (Σ effective_stats / Σ thresholds).clamp(0.0, 1.0) × (1.0 - difficulty)
roll           = random f64 in [0.0, 1.0)
```

| Outcome | Condition |
|---------|-----------|
| **Victory** | `roll < success_chance` |
| **Failure + Injury** | `roll >= success_chance` AND `roll < 0.95` |
| **Critical Failure** | `roll >= 0.95` (Operator removed from roster permanently) |

**Injury cooldown:** `duration_secs × 2` seconds from resolution time.

---

## 3. Persistence Contract
- Single save file: `save.json` in the working directory
- Schema: `{ "roster": [...], "bank": u64, "active_deployments": [...] }`
- Load on start → mutate in memory → save on every state change
- If `save.json` is absent, start a fresh game

---

## 4. CLI Commands (Tier 1 + 2)

| Command | Description |
|---------|-------------|
| `operator roster` | List all operators and their current state |
| `operator hire <name> <job>` | Add a new operator to the roster |
| `operator mission list` | Show available missions |
| `operator deploy <mission_id> <op1> [op2] [op3]` | Assemble squad and deploy |
| `operator aar` | Check all completed missions and collect rewards |
| `operator status` | Show bank balance and active deployments |

---

## 5. Acceptance Criteria (Tier 1)
- [x] `models.rs` compiles with zero warnings
- [x] `calculate_success_probability()` returns value in `[0.0, 1.0]`
- [x] `is_complete()` returns `true` when `Utc::now() >= completes_at`
- [x] Unit tests pass for success formula boundary conditions
- [x] Hardcoded squads can be "run" via `cargo run`
