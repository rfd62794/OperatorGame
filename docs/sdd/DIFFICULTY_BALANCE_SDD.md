# SDD-035: Mission Difficulty & Stat Balance

## §1 — Ground Truth Foundation

The Operator stat system is a hierarchical transformation of genomic base values. All mission difficulty (DC) and requirements are derived from these baseline ranges.

### 1.1 Fresh Recruit Specs (L1 Hatchling)
- **Genome Base Stat (STR/AGI/INT)**: `5` to `8` (randomised at recruitment).
- **LifeStage Multiplier (Hatchling)**: `0.6x`.
- **XP Growth Factor (0 XP)**: `0.8x`.
- **Actual Computed Stat**: `base * 0.48`.
  - **Min (Base 5)**: **2.4** (rounded to 2 in UI).
  - **Max (Base 8)**: **3.84** (rounded to 3-4 in UI).
  - **Squad Power (2x L1)**: **~5 to 6** total.
  - **Squad Power (3x L1)**: **~8 to 9** total.

### 1.2 LifeStage Multiplier Table
| Stage | Level | Multiplier | Power vs Hatchling |
| :--- | :--- | :--- | :--- |
| Hatchling | 0 - 1 | 0.6x | 100% |
| Juvenile | 2 - 3 | 0.8x | 133% |
| Young | 4 - 5 | 1.0x | 166% |
| Prime | 6 - 7 | 1.2x | 200% |
| Veteran | 8 - 9 | 1.1x | 183% |
| Elder | 10+ | 1.0x | 166% |

---

## §2 — Design Success Targets

| Configuration | Tier: Starter | Tier: Standard | Tier: Advanced | Tier: Elite |
| :--- | :--- | :--- | :--- | :--- |
| **Solo L1 Hatchling** | 75% (Good) | 35% (Danger) | 5% (Brick) | 5% (Brick) |
| **Squad (2) L1s** | 90% (Gtd) | **60% (Risky)** | 10% (Wall) | 5% (Brick) |
| **Squad (3) L1s** | 95% (Gtd) | 80% (Good) | 20% (Wall) | 5% (Brick) |
| **Squad (3) L3s** | 95% (Gtd) | 95% (Gtd) | 55% (Risky) | 10% (Wall) |
| **Squad (3) L6s** | 95% (Gtd) | 95% (Gtd) | 90% (Gtd) | 60% (Risky) |

---

## §3 — Derived Tier Parameters

Working backward from **§2** targets using the D20 success formula.

### 3.1 Tier: Starter
- **Design Target**: Solo L1 (Stat 3) = 75% Success.
- **Math**: 75% Success @ DC 5 needs `Mod 0`. Req must match Stat.
- **LOCKED VAL**: **DC 5** | **Primary Req: 3 - 5**

### 3.2 Tier: Standard
- **Design Target**: Squad (2) L1s (Stat 6) = 60% Success.
- **Math**: 60% Success @ DC 6 needs `Mod -3`. `Modifier = round((6/Req - 1) * 10) = -3`. `coverage ≈ 0.7`. `Req = 6 / 0.7 ≈ 8.5`.
- **LOCKED VAL**: **DC 6** | **Primary Req: 8 - 12**

### 3.3 Tier: Advanced
- **Design Target**: Squad (3) L3s (Stat 15) = 55% Success.
- **Math**: 55% Success @ DC 10 needs `Mod 0`. Req must match Stat.
- **LOCKED VAL**: **DC 10** | **Primary Req: 15 - 20**

### 3.4 Tier: Elite
- **Design Target**: Squad (3) L6s (Stat 30) = 60% Success.
- **Math**: 60% Success @ DC 12 needs `Mod -3`. `Coverage ≈ 0.7`. `Req = 30 / 0.7 ≈ 42`.
- **LOCKED VAL**: **DC 15** | **Primary Req: 40 - 60**

---

## §4 — Progression & XP Pacing

### 4.1 Level-Up Gates
- **Hatchling (L1) → Juvenile (L2)**: 100 XP.
- **Juvenile (L2) → Juvenile (L3)**: 100 XP.
- **Pacing**: 2 Victory Missions (50 XP ea) = 1 Level.
- **Standard Gate**: Currently level-gated at L3. Player must run **4 Starter victories** to unlock Standard missions. This is too slow for MVPs.
- **REVISION**: Unlock Standard at **Level 1** (Hatchling) so high-skill staging (2-3 slimes) is immediately relevant.

### 4.2 Stat Growth Table (Mean)
| Level | Stage | Squad (3) Primary Stat | Unlock Tier |
| :--- | :--- | :--- | :--- |
| L1 | Hatchling | 8 - 9 | Starter / Standard |
| L3 | Juvenile | 15 - 18 | Advanced |
| L6 | Prime | 28 - 32 | Elite |

---

## §5 — Locked Reference Table

These values represent the single source of truth for the Sprint G.1 re-balance implementation.

| Mission Tier | DC Range | Primary Requirement | Level Gate |
| :--- | :--- | :--- | :--- |
| **Starter** | 4 - 6 | 3 - 5 | 1 |
| **Standard** | 6 - 8 | 8 - 12 | 1 |
| **Advanced** | 10 - 14 | 15 - 20 | 3 |
| **Elite** | 15 - 20 | 40 - 60 | 6 |
