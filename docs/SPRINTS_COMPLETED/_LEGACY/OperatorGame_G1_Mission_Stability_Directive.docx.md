**OperatorGame**

Sprint G.1 — Mission Stability, Difficulty Scaling & Squad Persistence

*Directive Type: IMPLEMENTATION  |  Pre-flight: run tests, report count, stop if failing*

**⛔ STOP:** Run the full test suite before any changes. Report the passing count. If any test fails or skips, stop and notify the architect before proceeding.

# **0\. Goal**

This sprint addresses four problems discovered during live play on the Moto G:

* A critical bug where deployed squads are permanently locked if their mission is cycled out of the pool

* Squad composition resetting when navigating between Roster and Missions tabs

* No visible mission difficulty or chance of success to guide squad selection

* Upkeep costs draining the bank while core loop balance is still being validated

No new game systems. No UI architecture changes. Every task is either a bug fix, a balance pass, or surfacing existing data.

# **1\. Scope**

| File | Change Type | Tasks |
| :---- | :---- | :---- |
| src/persistence.rs | Modify — orphan deployment recovery, upkeep disable | A, E |
| src/models.rs | Modify — mission difficulty tier fields, success chance calc | B, C |
| src/world\_map.rs | Modify — static tiered mission pool generation | B |
| src/ui/ops.rs | Modify — chance of success display on mission cards | C |
| src/ui/mod.rs | Modify — squad state persistence across tab switches | D |
| src/ui/manifest.rs | Read only — no changes unless Task D requires wiring | D |
| tests/g1\_stability.rs | Create — new test file, min 10 tests | All |

**⚠** *All other files are read-only. Do not modify radar.rs, platform.rs, CONSTITUTION.md, SPEC.md, or Cargo.toml.*

# **2\. Tasks**

## **Task A — CRITICAL: Orphan Deployment Recovery**

Priority: fix this first. A deployed squad whose mission is cycled out of the active pool becomes permanently unreachable. The squad is locked in DEPLOYED state with no way to process the AAR or retrieve the operators.

### **A.1 — Protect active deployments from mission pool cycling**

File: src/persistence.rs — mission pool refresh logic

When the mission pool cycles or refreshes, check for active deployments before removing any mission. A mission with an active deployment attached must never be removed from the pool until the deployment is resolved.

// Before removing a mission from the pool:

let has\_active\_deployment \= self.active\_expeditions

    .iter()

    .any(|dep| dep.mission\_id \== mission.id);

if has\_active\_deployment { continue; } // skip — do not remove

### **A.2 — Orphan recovery on save load**

File: src/persistence.rs — GameState load path

On every load, scan active\_expeditions for any deployment whose mission\_id does not exist in the current mission pool. For each orphan found:

* Reconstruct a minimal mission record from the deployment data (name, id, reward) sufficient to render the AAR collect button

* Add it back to the mission pool marked as ORPHANED — it will not appear in the quest board but will be reachable from Active Ops

* Push a System log entry: "\[Mission name\] recovered from orphaned state — collect AAR to release squad"

**⚠** *Do not auto-resolve orphaned deployments. The player must manually collect the AAR. Auto-resolving discards roll data and XP.*

### **A.3 — Orphan indicator in Active Ops**

File: src/ui/ops.rs — render\_active\_ops()

If a deployment's mission is marked ORPHANED, show a yellow warning label on the deployment card: "⚠ Mission data recovered — collect to release squad". The PROCESS AAR button must still be present and functional.

## **Task B — Static Tiered Mission Pool (Option A)**

Replace the current cycling mission pool with a static tiered pool. Missions are pre-defined with fixed difficulty tiers. The quest board shows only missions within a winnable range of the player's current average roster level.

### **B.1 — Add difficulty tier fields to Mission**

File: src/models.rs — Mission struct

Add:

pub tier: MissionTier,       // which difficulty band this mission belongs to

pub base\_dc: u8,             // difficulty class (target number for D20 rolls)

pub min\_roster\_level: u8,    // minimum average roster level to see this mission

\#\[derive(Debug, Clone, Serialize, Deserialize, PartialEq)\]

pub enum MissionTier {

    Starter,    // DC 4-6,  always visible

    Standard,   // DC 8-12, visible from avg level 2+

    Advanced,   // DC 14-16, visible from avg level 4+

    Elite,      // DC 18-20, visible from avg level 6+

}

### **B.2 — Define the static mission pool**

File: src/world\_map.rs — mission pool initialisation

Replace the dynamically cycled pool with a fixed pool of 12-16 missions across all four tiers. Missions in this pool never cycle out. New missions may be added in future sprints but existing ones are never removed.

Tier distribution:

* Starter (4 missions): DC 4-6, always visible, \~80-90% success rate for a single level-1 slime

* Standard (4 missions): DC 8-12, visible avg level 2+, \~60-75% success for a level-2 squad

* Advanced (4 missions): DC 14-16, visible avg level 4+, \~40-60% success for a level-4 squad

* Elite (2-4 missions): DC 18-20, visible avg level 6+, \~40-50% success for a level-6 squad — this is the balance ceiling

**⚠** *The hardest visible mission must not exceed 50% success rate for an average squad at the minimum roster level required to see it. This is the balance ceiling for this sprint.*

### **B.3 — Filter quest board by roster level**

File: src/ui/ops.rs — quest board render

Calculate the player's current average operator level across all non-deployed, non-injured operators. Only show missions where mission.min\_roster\_level \<= average\_level. Always show at least the Starter tier regardless of roster state.

If the roster is empty or all operators are deployed/injured, show only Starter missions with a note: "Recruit operators to unlock harder contracts."

## **Task C — Live Chance of Success Display**

Each mission card on the quest board shows a calculated success percentage based on the currently staged squad. Updates live when squad composition changes.

### **C.1 — Success chance calculation function**

File: src/models.rs — new free function

pub fn calculate\_success\_chance(

    squad: &\[\&Operator\],

    mission\_dc: u8,

) \-\> f32 {

    // For each operator in squad:

    // P(success) \= (20 \- (dc \- atk\_modifier).max(1) \+ 1\) / 20

    // Overall mission success \= majority of squad succeeds

    // Return as 0.0..=1.0

}

Use the existing D20 resolution math from combat.rs as reference. Do not duplicate logic — call the existing roll probability helper if one exists, or extract it.

### **C.2 — Display on mission card**

File: src/ui/ops.rs — quest board mission card render

Below the mission name and reward, add a success chance row:

// If squad is staged (at least 1 operator):

let chance \= calculate\_success\_chance(\&staged\_squad, mission.base\_dc);

let pct \= (chance \* 100.0).round() as u32;

let (label, color) \= success\_label(pct);

ui.label(format\!("{} — {}%", label, pct));

If no squad is staged, show: "— Stage a squad to see odds"

### **C.3 — Label and color mapping**

File: src/ui/ops.rs — new helper function success\_label(pct: u32) \-\> (&'static str, Color32)

| % Range | Label | Color | Meaning |
| :---- | :---- | :---- | :---- |
| 80–100% | FAVORABLE | rgb(100, 220, 100\) green | Routine contract |
| 60–79% | MODERATE | rgb(80, 200, 180\) teal | Standard risk |
| 40–59% | RISKY | rgb(220, 200, 80\) yellow | Meaningful challenge |
| 20–39% | DANGEROUS | rgb(220, 140, 60\) orange | High risk |
| 0–19% | CRITICAL | rgb(220, 80, 80\) red | Near-suicidal |

Both the label and the percentage must be visible on the card. Label for scanning, number for optimizers.

## **Task D — Squad Persistence Across Tab Navigation**

Currently navigating from Roster to Missions resets the staged squad. The staged squad must persist on OperatorApp state, not on the tab.

### **D.1 — Audit staged squad storage**

File: src/ui/mod.rs

Locate where staged\_squad or the equivalent squad selection state is stored. If it is a local variable inside a tab render function, move it to a field on OperatorApp with \#\[serde(skip)\] so it persists across tab switches but does not serialize to save.json.

### **D.2 — Do not clear squad on tab switch**

Find every location where the staged squad is reset or cleared. Remove any reset that is triggered by tab navigation. Squad should only clear when:

* The player explicitly taps a CLEAR SQUAD button

* A mission is successfully launched (deployment created)

* An operator in the squad is deployed or injured by another action

### **D.3 — Launch bar reflects squad at all times**

The Launch Bar at the bottom of the Missions tab must read from the same OperatorApp field regardless of which tab was most recently active. Confirm the Launch Bar re-renders correctly after returning from the Roster tab with a modified squad.

**⚠** *Do not move squad state into GameState. It does not need to survive app restart — only tab switches within a session. \#\[serde(skip)\] is correct.*

## **Task E — Disable Upkeep Costs**

The daily upkeep deduction is draining the bank while core loop balance is still being validated. Disable it temporarily.

### **E.1 — Zero out upkeep deduction**

File: src/persistence.rs — apply\_daily\_upkeep()

Comment out or return early from the upkeep deduction logic:

pub fn apply\_daily\_upkeep(\&mut self) {

    // TODO: re-enable when economy balance is validated (Sprint G.2+)

    return;

    // ... existing upkeep logic below

}

Do not delete the existing logic. Comment it out so it is easy to restore. The TODO comment is required — it must reference the sprint when this should be revisited.

# **3\. Test Anchors**

File: tests/g1\_stability.rs (create new). Minimum 10 tests. All must pass. Zero regressions from pre-sprint floor.

1. test\_active\_deployment\_blocks\_mission\_removal — a mission with an active deployment is not removed during pool refresh

2. test\_orphan\_deployment\_recovered\_on\_load — a deployment whose mission\_id is missing from pool is recovered and marked ORPHANED

3. test\_orphan\_system\_log\_entry\_created — loading an orphaned deployment pushes a System log entry

4. test\_mission\_tier\_starter\_always\_visible — Starter missions appear regardless of roster level

5. test\_mission\_tier\_elite\_hidden\_below\_level6 — Elite missions do not appear when average roster level is below 6

6. test\_success\_chance\_favorable\_range — calculate\_success\_chance returns \>= 0.80 for DC4 with level-3 squad

7. test\_success\_chance\_elite\_cap — calculate\_success\_chance returns \<= 0.50 for DC18 with level-6 squad

8. test\_success\_label\_risky — success\_label(45) returns ("RISKY", \_)

9. test\_squad\_persists\_across\_tab\_switch — staged squad is not cleared when active\_tab changes

10. test\_upkeep\_disabled — apply\_daily\_upkeep() does not reduce bank balance

# **4\. Completion Checklist**

* Pre-flight test count reported and passing

* Deployed squad on a cycled-out mission is recoverable — PROCESS AAR button visible and functional

* Loading a save with an orphaned deployment does not crash and shows recovery log entry

* Quest board shows only missions appropriate to current average roster level

* Starter missions always visible regardless of roster state

* Each mission card shows LABEL — XX% success chance when a squad is staged

* Success display updates when squad composition changes

* No squad staged shows '— Stage a squad to see odds'

* Navigating Roster → Missions → Roster does not clear the staged squad

* Launch Bar reflects current squad after returning from Roster tab

* apply\_daily\_upkeep() is disabled with TODO comment referencing Sprint G.2+

* All 10 new tests passing

* Zero regressions from pre-sprint test floor

* APK builds for aarch64 and armv7 without warnings

* Manual verify on Moto G: stage squad → check odds → navigate to Roster → return → odds still show → launch mission

# **5\. Notes for Agent**

**⚠** *Task A is the highest priority. Do it first. An orphaned squad is a save-corrupting bug — it must be fixed before any balance or UI work.*

**⚠** *Task B replaces the cycling mission pool. Confirm the old cycling logic is fully disabled — do not leave both systems active simultaneously. One pool, static, tiered.*

The success chance calculation in Task C must use the same D20 math as combat.rs resolution. Do not invent a parallel formula — inconsistency between preview and actual resolution destroys player trust.

Task D is audit-first. Read the code before changing anything. If staged\_squad is already on OperatorApp and the reset is a single line in a tab switch handler, the fix is one deletion, not a refactor.

Task E is one line. Do it last so it does not mask economy bugs during testing of Tasks A-D.

*RFD IT Services Ltd.  |  OperatorGame  |  Sprint G.1  |  March 2026*