# SDD-035: Mission Difficulty & Stat Balance (Structural Rebalance)

> [!IMPORTANT]
> **Supersedes previous values.** All prior DC/Req values in git history are deprecated. This document is the single source of truth for the Sprint G.1c rebalance.

## §1 — Ground Truth Foundation

The Operator stat system is a hierarchical transformation of genomic base values. All mission difficulty (DC) and requirements are derived from these baseline ranges.

### 1.1 Fresh Recruit Specs (L1 Hatchling)
- **Genome Base Stat (STR/AGI/INT)**: **8** to **12** (randomised at recruitment).
- **LifeStage Multiplier (Hatchling)**: `0.6x`.
- **XP Growth Factor (0 XP)**: `0.8x`.
- **Actual Computed Stat**: `base * 0.48`.
  - **Min (Base 8)**: **3.84** (rounded to 4 in UI).
  - **Max (Base 12)**: **5.76** (rounded to 6 in UI).
  - **Squad Power (2x L1)**: **~8 to 12** total.
  - **Squad Power (3x L1)**: **~12 to 18** total.

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
| **Squad (2) L1s** | 95% (Gtd) | **60% (Risky)** | 10% (Wall) | 5% (Brick) |
| **Squad (3) L1s** | 98% (Gtd) | 85% (Good) | 20% (Wall) | 5% (Brick) |
| **Squad (3) L3s** | 98% (Gtd) | 98% (Gtd) | 55% (Risky) | 10% (Wall) |
| **Squad (3) L6s** | 98% (Gtd) | 98% (Gtd) | 95% (Gtd) | 60% (Risky) |

---

## §3 — Derived Tier Parameters (3-Operator Squad Cap)

Working backward from **§2** targets. All requirements are calibrated against the hard **3-operator squad cap**.

### 3.1 Tier: Starter
- **Design Target**: Solo L1 (Stat 4) = 75% Success.
- **Math**: 75% Success @ DC 5 needs `Mod 0`. Req must match Stat.
- **LOCKED VAL**: **DC 5** | **Primary Req: 4 - 8**

### 3.2 Tier: Standard
- **Design Target**: Squad (2) L1s (Stat 10) = 60% Success.
- **Math**: 60% Success @ DC 6 needs `Mod -3`. `Modifier = round((10/Req - 1) * 10) = -3`. `coverage ≈ 0.7`. `Req = 10 / 0.7 ≈ 14`.
- **LOCKED VAL**: **DC 6** | **Primary Req: 8 - 12**

### 3.3 Tier: Advanced
- **Design Target**: Squad (3) L3s (Stat 18) = 55% Success.
- **Math**: 55% Success @ DC 10 needs `Mod 0`. Req must match Stat.
- **LOCKED VAL**: **DC 10** | **Primary Req: 14 - 20**

### 3.4 Tier: Elite
- **Design Target**: Squad (3) L6s (Stat 34) = 60% Success.
- **Math**: 60% Success @ DC 12 needs `Mod -3` (Target 9). `round((34/Req - 1) * 10) = -3`. `Coverage ≈ 0.7`. `Req = 34 / 0.7 ≈ 48`.
- **LOCKED VAL**: **DC 12** | **Primary Req: 28 - 36**
- **Note**: Previous Elite Req (45-55) was mathematically impossible for a 3-operator cap. A max-base L6 squad (3x 17) only reaches 51, and a typical L6 squad at growth floor is 34-36. 36 is the correct ceiling.

---

## §4 — Progression & XP Pacing

### 4.1 Level-Up Gates
- **Hatchling (L1) → Juvenile (L2)**: 100 XP.
- **Standard Gate**: **Level 1** (Hatchling) so high-skill staging (2-3 slimes) is immediately relevant.

### 4.2 Stat Growth Table (Mean - Growth Floor)
| Level | Stage | Squad (3) Primary Stat | Unlock Tier |
| :--- | :--- | :--- | :--- |
| L1 | Hatchling | 12 - 15 | Starter / Standard |
| L3 | Juvenile | 15 - 18 | Advanced |
| L6 | Prime | 30 - 34 | Elite |

### 4.3 Tier Dwell Time (Estimates)
- **Hatchling (L1) → Juvenile (L3)**: 4 Wins (200 XP).
- **Juvenile (L3) → Prime (L6)**: 6 Wins (300 XP).
- **Total to Elite Tier**: 10 Wins.

---

## §5 — Locked Reference Table

| Mission Tier | DC Range | Primary Req | Level Gate | Max Squad Stat |
| :--- | :--- | :--- | :--- | :--- |
| **Starter** | 4 - 6 | 4 - 8 | 1 | 18 (3x L1) |
| **Standard** | 6 - 8 | 8 - 12 | 1 | 18 (3x L1) |
| **Advanced** | 10 - 14 | 14 - 20 | 3 | 24 (3x L3) |
| **Elite** | 12 - 15 | 28 - 36 | 6 | 36 (3x L6) |

---

## §6 — Formula Notes

### 6.1 Coverage Floor
In `Mission::calculate_success_chance()`, the stat coverage ratio for each attribute must be clamped at a minimum floor:
`ratio = (stat / req).max(0.3)`.

This ensures that a single-stat deficit (e.g., bringing a squad with 0 INT to an INT mission) does not crash the overall success probability into "mathematically unwinnable" territory for early-game recruits. This provides a "pity floor" for suboptimal staging, while still rewarding correct affinity matching (which achieves 1.0+).
