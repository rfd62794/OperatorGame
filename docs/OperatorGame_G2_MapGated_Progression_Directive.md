# OperatorGame — Sprint G.2: Map-Gated Progression

**Directive Type:** IMPLEMENTATION  
**Authority:** AUDIT_REPORT_MapProgression.md | SDD-035  
**Pre-flight:** Run full test suite. Report passing count. Stop if anything fails.

---

## 0. Goal

Connect the world map to the mission loop. Currently the map is a visual decoration — nodes render but carry no gameplay meaning. This sprint makes the map the progression system.

A player starts with only the Center node (Hidden Meadow) unlocked. Six scouting missions — one per Ring 1 node — are available from day one. Completing a scout mission unlocks that node permanently, grants its one-time heritage resource yield, and opens its culture-specific mission pool. The map becomes a record of conquest. Locked nodes render gray. Unlocked nodes render in their culture color.

This is the purpose loop: **Scout → Unlock → Access missions → Earn resources → Fund recruitment → Scout further.**

No new game systems. No architectural changes. Every piece needed exists in the codebase — this sprint connects them.

---

## 1. Scope

| File | Change Type | Task |
|------|-------------|------|
| `src/models.rs` | Modify — add `node_id: Option<usize>` to Mission struct | A |
| `src/persistence.rs` | Modify — add `unlocked_nodes: HashSet<usize>` to GameState, bump SAVE_VERSION, migration | B |
| `src/world_map.rs` | Modify — add scout missions, link missions to nodes, unlock logic, heritage yield on completion | C |
| `src/ui/radar.rs` | Modify — locked node grayscale render, unlock pulse animation | D |
| `src/ui/contracts.rs` | Modify — filter quest board by node unlock state, show node name on mission card | E |
| `tests/g2_progression.rs` | Create — new test file, min 10 tests | All |

> ⚠ All other files are read-only. Do not modify `manifest.rs`, `ops.rs`, `mod.rs`, `genetics.rs`, or `platform.rs` unless Task B requires wiring `unlocked_nodes` into an existing save path.

---

## 2. Reference Data

The following ExpeditionTarget heritage data is the authority for scout mission generation. Do not invent new values — use these exactly.

### Ring 1 Scout Missions (generated at game start, always visible)

| Node Name | Culture | Danger → DC | Yield (B/S/R) | Mission Tier |
|-----------|---------|-------------|---------------|--------------|
| Ember Flats | Ember | 0.20 → DC 5 | 15 / 5 / 2 | Starter |
| Gale Ridge | Gale | 0.35 → DC 7 | 8 / 3 / 8 | Starter |
| Tide Basin | Tide | 0.30 → DC 6 | 12 / 10 / 6 | Starter |
| Marsh Delta | Marsh | 0.15 → DC 4 | 25 / 2 / 3 | Starter |
| Crystal Spire | Crystal | 0.50 → DC 9 | 5 / 8 / 15 | Standard |
| Tundra Shelf | Tundra | 0.55 → DC 10 | 10 / 20 / 5 | Standard |

> Note: Marsh Delta and Crystal Spire/Tundra Shelf are Ring 2 cultures but their scout missions are Ring 1 access gates — the player scouts toward them from the center.

### Danger → DC Conversion
`DC = round(danger * 20).clamp(4, 20)`  
This formula must be used consistently. Do not hand-tune individual DCs.

---

## 3. Tasks

### Task A — Link Missions to Nodes

**File:** `src/models.rs`

Add `node_id: Option<usize>` to the `Mission` struct:

```rust
pub struct Mission {
    // ... existing fields ...
    pub node_id: Option<usize>,  // None = general contract, Some(id) = scout/node mission
    pub is_scout: bool,           // true = unlocks the linked node on completion
}
```

Add `#[serde(default)]` to both new fields so existing saves deserialise without error.

Update `Mission::new()` signature to include both fields. Fix all call sites — most will pass `None` and `false`.

> ⚠ This will cause compile errors at every `Mission::new()` call site. Fix all of them before moving to Task B. Confirm it compiles cleanly.

---

### Task B — Unlock State in GameState

**File:** `src/persistence.rs`

Add to `GameState`:

```rust
#[serde(default)]
pub unlocked_nodes: std::collections::HashSet<usize>,
```

On first load (or when field is missing via `serde(default)`), `unlocked_nodes` contains only `{0}` — the Center node ID. This is the starting state.

Add initialisation logic in `GameState::new()`:

```rust
let mut unlocked_nodes = HashSet::new();
unlocked_nodes.insert(0); // Center node always unlocked
```

**SAVE_VERSION:** Increment by 1. Add migration:

```rust
if loaded.version < SAVE_VERSION {
    if loaded.unlocked_nodes.is_empty() {
        loaded.unlocked_nodes.insert(0); // ensure center always unlocked
    }
    loaded.version = SAVE_VERSION;
}
```

---

### Task C — Scout Missions and Unlock Logic

**File:** `src/world_map.rs`

#### C.1 — Generate scout missions at game start

Add a new function `generate_scout_missions() -> Vec<Mission>` that produces exactly 6 scout missions — one per Ring 1 node — using the heritage data table in §2.

Each scout mission:
- `name`: `"Scout: [Node Name]"` e.g. `"Scout: Ember Flats"`
- `node_id`: `Some(node_id)` — the ID of the target Ring 1 node
- `is_scout`: `true`
- `tier`: `MissionTier::Starter` or `MissionTier::Standard` per §2 table
- `base_dc`: derived from danger via conversion formula in §2
- `reward`: heritage yield values from §2 table (Biomass/Scrap/Reagents)
- `min_roster_level`: 1 — scout missions are always accessible

Scout missions are generated once and added to the static mission pool. They do not cycle. They do not expire. They are removed from the quest board only after their node is unlocked — at that point the node's own mission pool becomes available instead.

#### C.2 — Node unlock on scout completion

In the mission resolution path (called from `persistence.rs` after AAR is processed):

```rust
if mission.is_scout {
    if let Some(node_id) = mission.node_id {
        self.unlocked_nodes.insert(node_id);
        // Grant heritage yield
        self.inventory.biomass += mission.reward.biomass;
        self.inventory.scrap += mission.reward.scrap;
        self.inventory.reagents += mission.reward.reagents;
        // Push system log entry
        // "Zone unlocked: [Node Name] — [Culture] territory now accessible"
        self.push_log(LogEntry {
            message: format!("Zone unlocked: {} — {} territory now accessible",
                node_name, culture_name),
            outcome: LogOutcome::System,
            timestamp: current_unix_time(),
        });
    }
}
```

#### C.3 — Node mission pools (post-unlock content)

Each unlocked Ring 1 node exposes 2-3 culture-specific missions from the existing static pool. Link existing Standard-tier missions to their home node using `node_id`.

Assignment guidance (use existing mission names from the pool where culture matches):
- Ember node → 2 Ember-themed Standard missions
- Gale node → 2 Gale-themed Standard missions  
- Tide node → 2 Tide-themed Standard missions
- Secondary culture nodes → 1-2 matching missions each

If no matching missions exist in the current pool for a culture, leave `node_id: None` on those missions for now — they remain in the general pool. Do not create new missions in this sprint.

---

### Task D — Locked Node Visuals

**File:** `src/ui/radar.rs`

#### D.1 — Grayscale locked nodes

When rendering each node, check `app.state.unlocked_nodes.contains(&node.id)`:

```rust
let color = if unlocked_nodes.contains(&node.id) {
    // existing culture color
    let [r, g, b, _] = culture_accent(node.owner);
    egui::Color32::from_rgb(r, g, b)
} else {
    // locked — grayscale
    egui::Color32::from_rgb(80, 80, 80)
};
```

Locked nodes:
- Render in `Color32::from_rgb(80, 80, 80)` — dark gray
- Stroke: `Color32::from_rgb(120, 120, 120)` — slightly lighter gray
- No pulse animation (pulse is for contested nodes, locked nodes are inert)
- Radius: same as unlocked — do not shrink locked nodes

#### D.2 — Unlock pulse (one-time animation)

When a node transitions from locked to unlocked within a session, trigger a brief glow pulse. Store `recently_unlocked: HashSet<usize>` on `OperatorApp` (not GameState — session only, `#[serde(skip)]`).

When a node is in `recently_unlocked`, render it with an expanding ring animation for 2 seconds, then remove it from the set. Use `ctx.request_repaint()` during animation.

> ⚠ If the animation proves complex to implement cleanly, skip D.2 and note it as a future polish task. D.1 (grayscale) is the critical deliverable. Do not let D.2 block the sprint.

---

### Task E — Quest Board Filtering

**File:** `src/ui/contracts.rs`

#### E.1 — Filter missions by unlock state

The quest board renders missions in two sections:

**Section 1 — SCOUT MISSIONS** (always at top):
- Show all scout missions where `mission.is_scout == true` AND the target node is not yet in `unlocked_nodes`
- Label the section header: `"SCOUT MISSIONS — Explore new territory"`
- Once a node is unlocked, its scout mission disappears from this section

**Section 2 — AVAILABLE CONTRACTS** (below scouts):
- Show all non-scout missions where either `node_id == None` OR `node_id == Some(id)` where `id` is in `unlocked_nodes`
- Label the section header: `"AVAILABLE CONTRACTS"`

If both sections are empty (all nodes unlocked, no contracts available): show `"All territories scouted. New contracts coming."`

#### E.2 — Node name on mission card

For scout missions, add a location tag below the mission name:

```
Scout: Ember Flats
📍 Ring 1 — Ember Territory
RISKY 58% | Squad: 8 / Req: 5
```

For node-linked contracts (post-unlock), show:

```
Ember Extraction Run
📍 Ember Flats (Unlocked)
GOOD ODDS 74% | Squad: 8 / Req: 6
```

---

## 4. Test Anchors

**File:** `tests/g2_progression.rs` (create new)

Minimum 10 tests. All must pass. Zero regressions from pre-sprint floor (213).

1. `test_center_node_always_unlocked` — GameState::new() contains node 0 in unlocked_nodes
2. `test_scout_mission_count` — generate_scout_missions() returns exactly 6 missions
3. `test_scout_missions_all_have_node_id` — all 6 scout missions have is_scout=true and node_id=Some(_)
4. `test_scout_dc_conversion` — Ember Flats (danger 0.20) produces DC 4-5, Tundra Shelf (danger 0.55) produces DC 10-11
5. `test_node_unlock_on_scout_completion` — resolving a scout mission victory inserts node_id into unlocked_nodes
6. `test_heritage_yield_granted_on_unlock` — completing Ember Flats scout grants 15 Biomass, 5 Scrap, 2 Reagents
7. `test_scout_mission_hidden_after_unlock` — quest board does not show scout mission for already-unlocked node
8. `test_node_missions_visible_after_unlock` — node-linked contracts appear in quest board after node is unlocked
9. `test_save_version_migration` — loading pre-G2 save populates unlocked_nodes with {0} without data loss
10. `test_locked_node_not_in_unlocked_set` — Ring 1 nodes are not in unlocked_nodes on fresh GameState

---

## 5. Completion Checklist

- [ ] Pre-sprint test count reported
- [ ] `Mission` struct has `node_id` and `is_scout` fields with `serde(default)`
- [ ] All `Mission::new()` call sites updated — compiles cleanly
- [ ] `GameState` has `unlocked_nodes: HashSet<usize>` with center node pre-populated
- [ ] SAVE_VERSION incremented, migration tested on previous save
- [ ] 6 scout missions generated using heritage data from §2
- [ ] DC values match danger conversion formula — not hand-tuned
- [ ] Node unlock fires on scout completion and persists to save
- [ ] Heritage yield granted on unlock (Biomass/Scrap/Reagents)
- [ ] System log entry pushed on unlock
- [ ] Locked nodes render gray (80,80,80) in radar.rs
- [ ] Unlocked nodes render in culture color
- [ ] Quest board shows SCOUT MISSIONS section above AVAILABLE CONTRACTS
- [ ] Scout missions disappear from board after node unlocked
- [ ] Node-linked contracts appear after unlock
- [ ] Location tag visible on mission cards
- [ ] All 10 new tests passing
- [ ] Zero regressions from 213 pre-sprint floor
- [ ] APK builds for aarch64 and armv7 without warnings
- [ ] Manual verify on Moto G: start game → see gray Ring 1 nodes → complete scout → node turns culture color → node missions appear

---

## 6. Notes for Agent

> ⚠ Task A causes the most compile churn — `Mission::new()` call sites will all break. Do Task A first, fix every call site, confirm the build compiles before touching anything else.

> ⚠ Task B (SAVE_VERSION bump) — test the migration explicitly. Load a pre-sprint save, confirm `unlocked_nodes` populates to `{0}` and no other data is lost.

The unlock pulse animation in D.2 is optional. If it requires more than 30 minutes to implement cleanly, skip it and add a `// TODO: unlock pulse animation` comment. The gray/color visual distinction in D.1 is the deliverable that matters.

Scout missions use the heritage yield table in §2 as the authority. Do not use the existing `reward` field from the static mission pool for scout missions — those are general contract rewards. Scout rewards come from the ExpeditionTarget yield data.

The quest board section split (SCOUT MISSIONS / AVAILABLE CONTRACTS) must use `ScrollArea::vertical()` wrapping both sections together — not two separate scroll areas. One scroll, two labeled sections.

---

*RFD IT Services Ltd. | OperatorGame | Sprint G.2 | March 2026*
