# OperatorGame — Sprint G.3: Equipment (Hat) System

**Directive Type:** IMPLEMENTATION  
**Authority:** SDD-036 (EQUIPMENT_SDD.md) — read it before writing a single line of code  
**Pre-flight:** Run full test suite. Report passing count. Stop if anything fails or regresses below 219.

---

## 0. Goal

Implement the minimal hat equipment system defined in SDD-036. Players can purchase hats from a new Quartermaster sub-tab under the Map tab, equip them to operators, and see stat bonuses reflected in mission success calculations and on roster cards.

Four hats. One slot per operator. Scrap as currency. Map-unlock gating. No crafting, no abilities, no additional slots.

Every number, every unlock condition, every UI label in this directive comes from SDD-036. If something is not in SDD-036, it does not go in this sprint.

---

## 1. Scope

| File | Change Type | Task |
|------|-------------|------|
| `src/models.rs` | Add `Hat` struct, `HatId` enum, `Hat::from_id()`, `Hat::catalog()`, update `total_stats()` | A |
| `src/persistence.rs` | Add `hat_inventory` to GameState, `equipped_hat` to Operator, SAVE_VERSION 11, migration | B |
| `src/ui/quartermaster.rs` | **NEW FILE** — Quartermaster shop render, equip flow, operator selection | C |
| `src/ui/mod.rs` | Add Quartermaster sub-tab to Map tab sidebar, route to new render function | D |
| `src/ui/manifest.rs` | Add hat name label to roster card below operator name | E |
| `tests/g3_equipment.rs` | **NEW FILE** — min 8 tests | All |

> ⚠ All other files are read-only. Do not modify `radar.rs`, `contracts.rs`, `ops.rs`, `world_map.rs`, `genetics.rs`, `platform.rs`, or `persistence.rs` beyond the fields specified in Task B.

---

## 2. Authority: SDD-036 Reference Tables

### Hat Catalog (implement exactly these values — no deviation)

| HatId | Name | STR | AGI | INT | Scrap Cost | Unlock Node ID |
|-------|------|-----|-----|-----|------------|----------------|
| `ScoutHood` | Scout Hood | 0 | 2 | 0 | 50 | 0 (Center — always available) |
| `KnightHelm` | Knight Helm | 2 | 0 | 0 | 100 | Ember Flats node ID |
| `MageHood` | Mage Hood | 0 | 0 | 2 | 100 | Tide Basin node ID |
| `CommanderCap` | Commander Cap | 1 | 1 | 1 | 250 | Gale Ridge node ID |

> ⚠ Before hardcoding node IDs for Ember Flats, Tide Basin, and Gale Ridge — look up their actual IDs in `world_map.rs`. Do not assume they are 1, 2, 3. Use the real IDs from the node definitions.

### Stat application formula (from SDD-036 §4.4)

```
final_stat = floor(base * stage_multiplier * growth_factor) + hat_bonus
```

Hat bonuses are **flat additions after all multipliers**. They are not multiplied by stage.

---

## 3. Tasks

### Task A — Models: Hat Struct and total_stats() Update

**File:** `src/models.rs`

#### A.1 — Add HatId enum

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HatId {
    ScoutHood,
    KnightHelm,
    MageHood,
    CommanderCap,
}
```

#### A.2 — Add Hat struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Hat {
    pub id: HatId,
    pub name: &'static str,
    pub str_bonus: u8,
    pub agi_bonus: u8,
    pub int_bonus: u8,
    pub scrap_cost: u32,
    pub unlock_node_id: usize,
}
```

Note: `Hat` is not `Serialize/Deserialize` — only `HatId` serializes. `Hat` is always reconstructed from the catalog via `Hat::from_id()`.

#### A.3 — Add Hat::from_id() and Hat::catalog()

```rust
impl Hat {
    pub fn from_id(id: &HatId) -> Hat {
        // match id to catalog entry and return full Hat
    }

    pub fn catalog() -> Vec<Hat> {
        // return all four hats
        // use REAL node IDs from world_map.rs for unlock_node_id
    }

    pub fn available_for(unlocked_nodes: &HashSet<usize>) -> Vec<Hat> {
        // filter catalog to hats whose unlock_node_id is in unlocked_nodes
        Self::catalog()
            .into_iter()
            .filter(|h| unlocked_nodes.contains(&h.unlock_node_id))
            .collect()
    }
}
```

#### A.4 — Update total_stats()

Locate `Operator::total_stats()`. Add hat bonus application at the end:

```rust
pub fn total_stats(&self) -> (u32, u32, u32) {
    let (base_str, base_agi, base_int) = self.computed_base_stats(); // existing logic
    let hat_bonus = self.equipped_hat.as_ref()
        .map(|id| Hat::from_id(id))
        .map(|h| (h.str_bonus as u32, h.agi_bonus as u32, h.int_bonus as u32))
        .unwrap_or((0, 0, 0));
    (
        base_str + hat_bonus.0,
        base_agi + hat_bonus.1,
        base_int + hat_bonus.2,
    )
}
```

> ⚠ `calculate_success_chance()` calls `total_stats()` — hat bonuses flow into mission calculations automatically. Do not modify `calculate_success_chance()`.

---

### Task B — Persistence: Hat Inventory and Operator Slot

**File:** `src/persistence.rs`

#### B.1 — Add equipped_hat to Operator

```rust
#[serde(default)]
pub equipped_hat: Option<HatId>,
```

#### B.2 — Add hat_inventory to GameState

```rust
#[serde(default)]
pub hat_inventory: Vec<HatId>,
```

`hat_inventory` contains all purchased hats including duplicates and unequipped hats. When an operator equips a hat, it is removed from `hat_inventory`. When a hat is unequipped or replaced, it returns to `hat_inventory`.

#### B.3 — Bump SAVE_VERSION to 11

```rust
// Migration from v10:
if loaded.version < 11 {
    // serde(default) handles both new fields automatically
    // no explicit field migration needed
    loaded.version = 11;
}
```

#### B.4 — Add purchase_hat() and equip_hat() to GameState

```rust
impl GameState {
    /// Purchase a hat. Returns Err if insufficient Scrap or hat not available.
    pub fn purchase_hat(&mut self, hat_id: HatId, unlocked_nodes: &HashSet<usize>) 
        -> Result<(), String> 
    {
        let hat = Hat::from_id(&hat_id);
        if !unlocked_nodes.contains(&hat.unlock_node_id) {
            return Err("Node not scouted".into());
        }
        if self.inventory.scrap < hat.scrap_cost {
            return Err(format!("Insufficient Scrap ({} / {})", 
                self.inventory.scrap, hat.scrap_cost));
        }
        self.inventory.scrap -= hat.scrap_cost;
        self.hat_inventory.push(hat_id);
        Ok(())
    }

    /// Equip a hat from inventory to an operator.
    /// Returns the previously equipped hat to inventory if one exists.
    pub fn equip_hat(&mut self, operator_id: Uuid, hat_id: HatId) 
        -> Result<(), String> 
    {
        let op = self.operators.iter_mut()
            .find(|o| o.id == operator_id)
            .ok_or("Operator not found")?;

        // Remove hat from inventory
        let pos = self.hat_inventory.iter().position(|h| *h == hat_id)
            .ok_or("Hat not in inventory")?;
        self.hat_inventory.remove(pos);

        // Return previous hat to inventory
        if let Some(prev) = op.equipped_hat.take() {
            self.hat_inventory.push(prev);
        }

        op.equipped_hat = Some(hat_id);
        Ok(())
    }
}
```

---

### Task C — New File: Quartermaster UI

**File:** `src/ui/quartermaster.rs` (create new)

Implement `render_quartermaster(ui, app_state)` with two sections:

#### C.1 — Header

```
QUARTERMASTER          Scrap: [current_scrap]
```

#### C.2 — Shop section

For each hat in `Hat::available_for(&self.state.unlocked_nodes)`:

```
┌─────────────────────────────────────┐
│ Knight Helm              100 Scrap  │
│ +2 STR — Bruiser                    │
│ In inventory: 1   [BUY] [EQUIP →]  │
└─────────────────────────────────────┘
```

- `[BUY]` calls `self.state.purchase_hat(hat_id, &self.state.unlocked_nodes)`
  - Disabled (grayed) if `self.state.inventory.scrap < hat.scrap_cost`
  - Shows error message for one frame if purchase fails
- `[EQUIP →]` sets `self.pending_equip_hat = Some(hat_id)` which triggers the operator selection view (C.3)
  - Disabled if hat count in inventory is 0 (none owned)

If no hats are available (all nodes locked — impossible since Center is always unlocked but handle gracefully): show "Scout territories to unlock gear."

#### C.3 — Operator selection view

Triggered when `pending_equip_hat: Option<HatId>` is `Some`. Renders in place of the shop:

```
Equipping: Scout Hood (+2 AGI)

[CANCEL]

── SELECT OPERATOR ──────────────────
Echo   Gale  Lv3   AGI:5  [No hat]   [EQUIP]
Spark  Ember Lv2   AGI:3  Scout Hood [EQUIP]
── DEPLOYED ─────────────────────────
Tide   Tide  Lv4   (Deployed — unavailable)
```

- Only non-deployed, non-injured operators are selectable
- Deployed/injured operators are shown but grayed with a status label
- `[EQUIP]` calls `self.state.equip_hat(operator_id, hat_id)`
- After equip, clear `pending_equip_hat` and return to shop view
- `[CANCEL]` clears `pending_equip_hat` without changes

Add `pending_equip_hat: Option<HatId>` as a `#[serde(skip)]` field on `OperatorApp`.

#### C.4 — Wrap in ScrollArea

The entire Quartermaster view (both shop and operator selection) must be wrapped in a single `ScrollArea::vertical()` with a unique ID `"quartermaster_scroll"`.

---

### Task D — Map Tab: Add Quartermaster Sub-Tab

**File:** `src/ui/mod.rs`

#### D.1 — Add MapSubTab enum

```rust
pub enum MapSubTab {
    Zones,
    Quartermaster,
}
```

Add `map_sub_tab: MapSubTab` to `OperatorApp` with default `MapSubTab::Zones`.

#### D.2 — Add sidebar to Map tab

The Map tab currently has no sidebar. Add one following the same pattern as the Roster sidebar:

```
Zones
─────
Quartermaster
```

Two buttons. Same styling as existing sub-tab buttons.

#### D.3 — Route sub-tabs

```rust
MapSubTab::Zones => self.render_radar(ui),
MapSubTab::Quartermaster => self.render_quartermaster(ui),
```

> ⚠ `render_radar()` must not be affected by the sidebar addition. The map tab's available width changes when a sidebar is added — confirm ring 3 still renders without clipping after adding the sidebar. If it clips, reduce the sidebar width or make it collapsible. Do not break the map to add the sidebar.

---

### Task E — Roster Card: Hat Name Label

**File:** `src/ui/manifest.rs` — `render_operator_card()`

Below the operator name, add a hat label if equipped:

```rust
if let Some(hat_id) = &op.equipped_hat {
    let hat = Hat::from_id(hat_id);
    ui.label(
        egui::RichText::new(hat.name)
            .size(11.0)
            .color(egui::Color32::from_rgb(160, 160, 200))
    );
}
```

If no hat equipped, show nothing — do not show "No hat" or any placeholder.

---

## 4. Test Anchors

**File:** `tests/g3_equipment.rs` (create new). Minimum 8 tests. Zero regressions from 219.

1. `test_hat_catalog_has_four_entries` — `Hat::catalog()` returns exactly 4 hats
2. `test_scout_hood_always_available` — `Hat::available_for({0})` includes ScoutHood
3. `test_knight_helm_locked_without_ember` — KnightHelm not in `available_for` when Ember node not unlocked
4. `test_knight_helm_available_after_ember_unlock` — KnightHelm in `available_for` after Ember node added
5. `test_purchase_hat_deducts_scrap` — `purchase_hat()` reduces scrap by correct amount
6. `test_purchase_hat_insufficient_scrap_returns_err` — `purchase_hat()` returns Err when scrap < cost
7. `test_equip_hat_applies_to_operator` — `equip_hat()` sets `equipped_hat` on correct operator
8. `test_equip_hat_returns_previous_to_inventory` — equipping over existing hat returns old hat to `hat_inventory`
9. `test_total_stats_includes_hat_bonus` — operator with KnightHelm has STR 2 higher than without
10. `test_save_version_migration_v10_to_v11` — loading v10 save populates `hat_inventory` as empty vec, `equipped_hat` as None

---

## 5. Completion Checklist

- [ ] Pre-sprint test count reported (must be 219)
- [ ] `HatId` enum and `Hat` struct defined in `models.rs`
- [ ] `Hat::from_id()`, `Hat::catalog()`, `Hat::available_for()` implemented
- [ ] Real node IDs used for unlock conditions (not assumed values)
- [ ] `total_stats()` applies hat bonus as flat post-multiplier addition
- [ ] `calculate_success_chance()` unchanged — hat flows in via `total_stats()` automatically
- [ ] `equipped_hat: Option<HatId>` on Operator with `serde(default)`
- [ ] `hat_inventory: Vec<HatId>` on GameState with `serde(default)`
- [ ] `purchase_hat()` and `equip_hat()` implemented on GameState
- [ ] SAVE_VERSION bumped to 11, migration tested
- [ ] `quartermaster.rs` created — shop renders available hats, BUY and EQUIP → functional
- [ ] Operator selection view renders non-deployed operators, deployed ones grayed
- [ ] Equipping a hat over an existing hat returns old hat to inventory
- [ ] Map tab has Zones | Quartermaster sidebar
- [ ] Radar map (Zones sub-tab) still renders without clipping after sidebar added
- [ ] Roster card shows hat name label when equipped, nothing when unequipped
- [ ] All 10 new tests passing
- [ ] Zero regressions from 219 pre-sprint floor
- [ ] APK builds for aarch64 and armv7 without warnings
- [ ] Manual verify on Moto G: buy Scout Hood → equip to operator → check roster card shows label → check mission odds increased

---

## 6. Notes for Agent

> ⚠ Task A causes the most downstream impact — `total_stats()` is called in many places. Make the change, run `cargo check`, fix any type errors before moving to Task B.

> ⚠ Task D adds a sidebar to the Map tab. The radar map currently uses `ui.available_size()` for scaling. Adding a sidebar reduces available width. Test the map render after adding the sidebar — if Ring 3 nodes clip, reduce sidebar width to 80dp minimum.

The `Hat` struct is not serialized — only `HatId` is. `Hat::from_id()` is the reconstruction path. This is intentional: hat definitions may change between versions without breaking saves.

`purchase_hat()` and `equip_hat()` are `GameState` methods, not UI logic. Keep business logic in persistence, keep rendering in UI. Do not put purchase/equip logic inside the `render_quartermaster()` function directly — call the GameState methods from the UI event handlers.

The `pending_equip_hat` field on `OperatorApp` is session state only — `#[serde(skip)]`. It should not survive app restart, only tab switches.

---

*RFD IT Services Ltd. | OperatorGame | Sprint G.3 | March 2026*
