# SDD-037: Gauntlet Mission System (Multi-Target)

## Overview
Transitions the mission system from a single-target "Total Squad vs Total Req" model to a sequential "Gauntlet" format. This increases tactical depth, risk management (injury decay over time), and reward scaling.

## Data Model (src/models/mission.rs)

### Target Struct
```rust
pub struct Target {
    pub name: String,
    pub description: String,
    pub base_dc: u32,
    pub req_strength: u32,
    pub req_agility: u32,
    pub req_intelligence: u32,
}
```

### Mission Struct
```rust
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub targets: Vec<Target>,
    // ... metadata ...
}
```

## Resolution Logic

### Sequential Encounters
When a deployment is resolved, the engine iterates through the `targets` vector.
1. Each target is a separate D20 roll suite (Str, Agi, Int checks).
2. If all checks for a target pass, the squad proceeds to the next target.
3. If any check fails, the mission terminates immediately.

### Reward Scaling
Rewards are proportional to the number of targets defeated.
- **Victory**: All targets defeated. 100% rewards.
- **Partial Failure**: Some targets defeated. Rewards = `(defeated / total) * base_reward`.
- **Critical Failure**: First target failed. 0% rewards.

### Injury Risk
Sequential resolution increases the "Nat 1" window. Each target encounter carries a risk of injury. Longer gauntlets (3+ targets) are significantly more dangerous for low-stamina squads.

## UI Integration (src/ui/ops.rs)
- The AAR (After Action Report) displays "Targets Defeated: X / Y".
- Progress bars or status labels indicate which target caused the failure.
