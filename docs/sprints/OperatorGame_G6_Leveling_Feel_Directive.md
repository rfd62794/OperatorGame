# OperatorGame — Sprint G.6: Leveling Feel & First Impression

**Directive Type:** IMPLEMENTATION  
**Authority:** GAME_DESIGN.md v2.0 §5 — The 10-Level Arc  
**Pre-flight:** Run full test suite. Report passing count. Stop if anything fails.

---

## 0. Goal

The Hatchling → Juvenile transition is a 33% power spike. It does not currently feel like one. A player who levels up for the first time should feel something — a visible number change, a moment of recognition, a sense that their operator just became meaningfully stronger.

This sprint makes that moment real.

No new systems. No new content. Pure feedback, presentation, and tuning. Every change in this sprint is either:
- A number that makes progression feel correct
- A UI moment that makes progression feel visible

---

## 1. Scope

| File | Change Type | Task |
|------|-------------|------|
| `src/models/operator.rs` | Modify — XP rate tuning, stat growth per level | A |
| `src/persistence.rs` | Modify — level-up detection, stat delta capture | B |
| `src/ui/ops.rs` | Modify — level-up moment in AAR panel | C |
| `src/ui/manifest.rs` | Modify — stat display on roster card, stage label | D |
| `tests/g6_leveling.rs` | Create — new test file, min 8 tests | All |

> ⚠ All other files are read-only. Do not modify `world_map.rs`, `contracts.rs`, `radar.rs`, `quartermaster.rs`, `genetics.rs`, or `platform.rs`.

---

## 2. Design Authority

From GAME_DESIGN.md §5.1:

| Level | Stage | Stat Multiplier |
|-------|-------|----------------|
| 1 | Hatchling | 0.6x |
| 2-3 | Juvenile | 0.8x |
| 4-5 | Young | 1.0x |
| 6-7 | Prime | 1.2x |
| 8-9 | Veteran | 1.1x |
| 10 | Elder | 1.0x + passive |

**The design target:** Level 1 → 2 must feel strong. The 0.6x → 0.8x multiplier transition means a slime with base STR 10 goes from effective STR 6 to effective STR 8. That is a real change. The player must see it and feel it.

---

## 3. Tasks

### Task A — XP Rate Tuning

**File:** `src/models/operator.rs`

#### A.1 — Audit current XP awards and thresholds

Read the current values for:
- XP awarded on mission Victory
- XP awarded on mission Failure  
- XP threshold per level (is it flat 100? Does it scale?)
- How many missions does it currently take to reach Level 2?

Report these values before changing anything. If it currently takes more than 3 victories to reach Level 2, the rate is too slow for the early game hook.

#### A.2 — Target pacing (from ROADMAP.md)

- Level 1 → 2: 2-3 victories (fast, rewarding, immediate feedback)
- Level 2 → 3: 3-4 victories (slight increase, still fast)
- Level 3 → 4: 4-5 victories (slowing, feeling the climb)
- Level 10: approximately 20+ total victories (meaningful investment)

Adjust XP awards or thresholds to hit this pacing. Show the math. Do not guess — calculate.

#### A.3 — Stat growth on level-up

Confirm that stat growth on level-up is driven by the stage multiplier table, not by flat additions. If flat additions are currently used (+2 HP, +1 ATK per level), replace with the multiplier system from the design table. The multiplier means a high-base slime benefits more from leveling than a low-base one — that's intentional genetic differentiation.

> ⚠ If the multiplier system is already implemented correctly, do not change it. Report and move on.

---

### Task B — Level-Up Detection and Stat Delta Capture

**File:** `src/persistence.rs`

#### B.1 — Capture stat delta on level-up

When `resolve_deployment()` awards XP and detects a level-up, capture the stat change:

```rust
pub struct LevelUpEvent {
    pub operator_id: Uuid,
    pub operator_name: String,
    pub old_level: u8,
    pub new_level: u8,
    pub old_stage: LifeStage,
    pub new_stage: LifeStage,        // may be same stage if within range
    pub stat_delta: StatDelta,        // how much each stat changed
}

pub struct StatDelta {
    pub str_change: i32,
    pub agi_change: i32,
    pub int_change: i32,
}
```

Add `#[serde(skip)]` — these are session events, not persisted state.

Return `Vec<LevelUpEvent>` from the resolution path alongside `AarOutcome`. The UI layer uses this to display the level-up moment.

#### B.2 — Stage transition detection

If a level-up crosses a stage boundary (e.g., Level 1 → 2 is Hatchling → Juvenile), flag `old_stage != new_stage` on the `LevelUpEvent`. Stage transitions get special treatment in the UI.

---

### Task C — Level-Up Moment in AAR Panel

**File:** `src/ui/ops.rs`

This is the most important task. The AAR panel is where the player learns their operators leveled up. Currently this is a text line in the log. It needs to be a moment.

#### C.1 — Level-up section in AAR panel

After the mission outcome and XP display, add a dedicated PROMOTIONS section if any level-up events occurred:

```
── FIELD PROMOTIONS ────────────────────
⬆ Echo        Lv 1 → 2    HATCHLING → JUVENILE
   STR: 3 → 4  (+1)   AGI: 3 → 4  (+1)   INT: 2 → 3  (+1)

⬆ Spark       Lv 3 → 4    JUVENILE → YOUNG
   STR: 5 → 6  (+1)   AGI: 4 → 5  (+1)   INT: 4 → 5  (+1)
────────────────────────────────────────
```

- Show operator name, old level → new level
- Show old stage → new stage (if changed) or just the current stage
- Show each stat before and after with the delta
- If no level-ups occurred: do not show this section at all

#### C.2 — Stage transition emphasis

If the level-up crosses a stage boundary (Hatchling → Juvenile, Juvenile → Young, etc.), add a visual emphasis. The stage name change is the meaningful moment — the operator is genuinely different now.

Use a distinct color or a separator to make stage transitions stand out from same-stage level-ups.

#### C.3 — Corporate tone

The section header should fit the astronaut's frame. Options:
- `FIELD PROMOTIONS`
- `PERSONNEL ADVANCEMENT NOTICES`
- `HR NOTIFICATIONS`

Pick one that fits the established tone. The astronaut is filing paperwork. The slimes are becoming stronger.

> ⚠ The DISMISS button must remain always visible. Do not let the promotions section push it off screen. The promotions section goes inside the existing ScrollArea.

---

### Task D — Roster Card Stage Label

**File:** `src/ui/manifest.rs`

#### D.1 — Add stage label to roster card

The roster card currently shows level and XP bar. Add the stage name below the level:

```
Echo   Gale  Lv 2
       JUVENILE
STR:4  AGI:4  INT:3
```

Stage label: small, muted color, all caps. Same 11.0pt size as the hat label. If the operator is at a stage transition (just leveled), highlight the stage label briefly — but do not add animation complexity. A color change is sufficient.

#### D.2 — Stage color coding

Each stage gets a distinct muted color for its label:

| Stage | Color |
|-------|-------|
| Hatchling | Gray (160, 160, 160) |
| Juvenile | Green-tinted (140, 200, 140) |
| Young | Teal (100, 200, 180) |
| Prime | Gold (220, 180, 80) |
| Veteran | Orange (200, 140, 60) |
| Elder | Purple (180, 120, 220) |

These are muted — they should not dominate the card. They signal stage at a glance.

---

## 4. Test Anchors

**File:** `tests/g6_leveling.rs` (create new). Minimum 8 tests. Zero regressions.

1. `test_xp_rate_level1_to_2` — operator reaches Level 2 after exactly N victories matching the design target (2-3)
2. `test_stat_multiplier_hatchling` — Level 1 operator total_stats() reflects 0.6x multiplier correctly
3. `test_stat_multiplier_juvenile` — Level 2 operator total_stats() reflects 0.8x multiplier correctly
4. `test_stat_delta_captured_on_levelup` — resolve_deployment() returns LevelUpEvent with correct stat delta
5. `test_stage_transition_detected` — LevelUpEvent.old_stage != new_stage when crossing stage boundary
6. `test_no_levelup_event_on_failure` — mission failure awards reduced XP but may not trigger level-up
7. `test_elder_at_level_10` — operator at level 10 has LifeStage::Elder
8. `test_level_up_section_hidden_when_no_promotions` — AAR panel does not render FIELD PROMOTIONS when no level-ups occurred

---

## 5. Completion Checklist

- [ ] Pre-sprint test count reported
- [ ] Current XP rate audited and reported before any changes
- [ ] XP pacing matches design target: 2-3 victories to Level 2
- [ ] Stat growth uses multiplier system, not flat additions
- [ ] `LevelUpEvent` struct captures operator name, old/new level, old/new stage, stat delta
- [ ] Stage transition detected and flagged on `LevelUpEvent`
- [ ] AAR panel shows FIELD PROMOTIONS section when level-ups occurred
- [ ] FIELD PROMOTIONS shows: name, level change, stage change, each stat before/after with delta
- [ ] Stage transitions visually distinct from same-stage level-ups
- [ ] DISMISS button always visible — promotions section inside ScrollArea
- [ ] Roster card shows stage label below level number
- [ ] Stage label uses color coding from design table
- [ ] All 8 new tests passing
- [ ] Zero regressions from pre-sprint floor
- [ ] APK builds without warnings
- [ ] Manual verify on Moto G: complete a mission → AAR shows FIELD PROMOTIONS → roster card shows new stage label

---

## 6. Notes for Agent

> ⚠ Task A.1 is audit-first. Read the current XP values and report them before changing anything. If the pacing is already correct, say so and move to Task B. Do not change numbers that don't need changing.

> ⚠ Task C is the highest priority deliverable. The stat numbers and pacing (Tasks A and B) matter, but the player never sees the math — they see the AAR panel. If the FIELD PROMOTIONS section is clear, readable, and exciting, this sprint succeeds.

The tone throughout is corporate-absurdist. "FIELD PROMOTIONS" is the astronaut's HR vocabulary for what is actually a slime becoming biologically more powerful. Keep that gap alive in the language choices.

Do not add animation, particle effects, or sound. Those are future polish. A clear, well-formatted text moment is the deliverable.

---

*RFD IT Services Ltd. | OperatorGame | Sprint G.6 | March 2026*
