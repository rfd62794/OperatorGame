# Operator Lifecycle, Stat Progression & Training System
## System Design Document (SDD) — v1.0 Draft
March 2026  |  Authority: Design Contract  |  Pre-implementation specification

Companion documents: SPEC.md — entity contracts  |  GDD.md — game feel & tone  |  STAT_SYSTEM.md (rpgCore) — upstream reference  |  CONSTITUTION.md — governing principles

## 1. Overview
This document specifies the Operator Lifecycle system for OperatorGame: how operators grow through experience, age into Elder status, reproduce through the breeding mechanic, and train individual stats between deployments.
The system draws from three sources:
1. The rpgCore 10-level lifecycle with stage modifiers (STAT_SYSTEM.md)
2. KairoSoft-style between-deployment training with diminishing returns (Game Dev Story, Mega Mall Story)
3. The MercWars operational framing — operators are personnel with HR records, not creatures

The lifecycle system exists to answer one design question: why does an operator matter beyond their current stats? The answer is threefold — accumulated stage power, breeding value, and Elder passives. Every operator has a trajectory. The player's job is to manage that trajectory across the squad.

## 2. Stage Ladder
Operator level is derived from total_xp // 100, maximum level 10. Stage is derived from level. Stage determines the stat multiplier applied during all mission calculations.

| Level Range | Stage Name | Stat Modifier | Operational Notes |
| :--- | :--- | :--- | :--- |
| 0–1 | Hatchling | 0.6x | Fragile. High injury probability. Low upkeep. |
| 2–3 | Juvenile | 0.8x | Learning. Standard injury rates. |
| 4–5 | Young | 1.0x | Baseline. Full deployment capability. |
| 6–7 | Prime | 1.2x | Peak performance. Breeding window opens. |
| 8–9 | Veteran | 1.1x | Battle-hardened. Slight decline from Prime. |
| 10 | Elder | 1.0x + passive | Stat growth plateaued. Elder passive active. Upkeep exempt. |

> [!NOTE]
> Prime (1.2x) intentionally exceeds Elder (1.0x). The Elder passive compensates — Elder is not strictly better in combat, but better economically (upkeep exempt) and socially (squad morale). This preserves the breed-or-keep tension.

## 3. Elder Passive Bonus
When an operator reaches Level 10 (Elder stage), they gain a permanent passive bonus that persists for the rest of their operational life. The passive is determined by the operator's dominant culture at the time of Elder promotion.

| Culture | Elder Passive | Mechanical Effect |
| :--- | :--- | :--- |
| Ember | Combat Hardening | Injury probability halved on this operator permanently. |
| Marsh | Endurance Protocol | Upkeep exempt AND recovers 10% of mission reward as bonus. |
| Crystal | Tactical Clarity | -5 DC on all missions when this operator is squad lead. |
| Tide | Morale Anchor | +10 XP to all squad members on victory (incl. self). |
| Gale | Rapid Response | Recovery time from injury halved permanently. |
| Tundra | Ironwall Protocol | RES stat counts double for squad DC calculations. |
| Orange | Field Intelligence | +15% XP gain on all missions for this operator. |
| Teal | Precision Doctrine | Critical success threshold reduced by 2 (easier crits). |
| Frost | Ancient Resilience | Cannot be critically failed — worst outcome is Failure. |
| Mixed/Void | Adaptive Protocol | Passive chosen from dominant culture at promotion time. |

> [!IMPORTANT]
> Upkeep exemption applies to ALL Elder operators regardless of culture — it is a base Elder benefit, not a passive. The culture passives above are additional bonuses on top of upkeep exemption.

## 4. XP Architecture
### 4.1 XP Pools
Every operator carries two XP categories:
- **total_xp** — drives level and stage. Accumulated from all sources.
- **Per-stat XP pools** — six pools (ATK, HP, DEF, CHM, SPD, RES) that drive individual stat growth factors within the current stage.

*Note: AGI, MND, END are deferred to Sprint 9+.*

### 4.2 XP Award by Mission Outcome
| Activity | total_xp | ATK | HP | DEF | CHM | SPD/RES |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Mission Victory | +50 | +20 | +15 | +10 | +5 | +5 each |
| Mission Failure | +20 | +8 | +6 | +4 | +2 | +2 each |
| Critical Failure| +10 | +4 | +3 | +2 | +1 | +1 each |
| Culture Match  | +0 | +5 | +5 | +5 | +5 | +5 each |
| Training Session| varies | varies | varies | varies | varies | varies |
| Garden Idle    | +5 | +1 | +2 | +1 | +2 | +1 each |

### 4.3 Stat Growth Factor
Inherited from STAT_SYSTEM.md:
```rust
fn stat_growth_factor(stat_xp: u32, level: u32) -> f32 {
    let xp_ceiling = level * 50;
    if xp_ceiling == 0 { return 1.0; }
    let ratio = stat_xp as f32 / xp_ceiling as f32;
    f32::max(0.8, f32::min(1.5, 0.8 + (ratio * 0.7)))
}
```

### 4.4 Final Stat Computation
`final_stat = (base_value * stage_modifier * stat_growth_factor + culture_modifier + equipment_modifier).max(1) as u32`

## 5. Training System (KairoSoft Model)
### 5.1 Training Methods
| Training Method | Culture | Primary Stat | Duration |
| :--- | :--- | :--- | :--- |
| Ember Sparring | Ember | ATK | 2h |
| Marsh Endurance | Marsh | HP | 3h |
| Crystal Focus | Crystal | DEF | 2h |
| Tide Negotiation | Tide | CHM | 1h |
| Gale Sprint Circuit | Gale | SPD | 1.5h |
| Tundra Meditation | Tundra | RES | 4h |

### 5.2 Diminishing Returns
Resets at 00:00 UTC.
```rust
fn training_xp_yield(base_xp: u32, sessions_today: u32) -> u32 {
    let multiplier = 0.5_f32.powi(sessions_today as i32);
    (base_xp as f32 * multiplier).round() as u32
}
```

## 6. Breeding & Lifecycle Cost
Breeding is a sacrifice decision:
- Parents regressions: Prime (L6-7) -> Young (L4), Veteran (L8-9) -> Young (L5), Elder (L10) -> Prime (L7).
- Parental Leave: 24–48 real hours.
- Offspring genome resolved by existing `BreedingResolver`.

## 7. Lifespan & Retirement
- **Elder (Level 10)**: 20 deployments remaining counter starts.
- **Veteran**: Unlimited until Elder.
- **Advisory Role**: Inactive status for retired operators.

## 8. Open Questions
- Q1. Squad synergy for lineage (parent-child)?
- Q2. Diminishing returns reset (UTC vs per-operator)?
- Q3. Frost Elder passive vs Emergency Protocol stacking?
- Q4. Passive suspension during parental leave?
- Q5. Crit fails costing 2 lifespan?
