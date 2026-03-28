# SDD-036: Minimal Hat (Equipment) System

**Status:** Accepted  
**Authority:** Designer (Robert)  
**Companion Documents:** SDD-035 (Difficulty Balance) | AUDIT_REPORT_MapProgression.md | ADR-031 (Static Mission Board)  
**Created:** March 2026  
**Note:** Supersedes the agent draft of SDD-036. This document is the single source of truth for the Hat system.

---

## §1 — Purpose and Design Intent

The Hat system provides a minimal equipment layer that serves three goals simultaneously:

1. **Tactical differentiation** — operators feel distinct based on their role, not just their genetics
2. **Progression reward** — scouting nodes unlocks gear, making map exploration doubly meaningful
3. **Visual identity** — a hat name (and eventually icon) on the roster card is readable at a glance on a 360dp screen

The system is deliberately minimal. One slot. Four hats at launch. No crafting, no upgrade trees, no durability. A hat is a stat bonus with a name and a class identity. Complexity is deferred explicitly — this document defines what the system is, and what it is not, for MVP.

---

## §2 — Resource Economy (Canonical Mapping)

This section is the authoritative mapping of all resources to their consuming systems. No resource should be spent on a system not listed here without updating this document first.

| Resource | Primary System | Purpose | Source |
|----------|---------------|---------|--------|
| Cash ($) | Recruitment | Hiring operators from the Union | General contracts, starting balance |
| Biomass | Breeding | Base material for incubation | Scout yields (Marsh Delta primary), mission rewards |
| Scrap | Equipment | Purchasing Hats from the Quartermaster | Scout yields (Tundra Shelf primary), mission rewards |
| Reagents | Breeding | Catalysts for rare pattern unlocks and genetic lens | Scout yields (Crystal Spire primary), mission rewards |

**Design rule:** Cash is the only resource that funds recruitment. Biomass and Reagents are the only resources that fund breeding. Scrap is the only resource that funds equipment. Cross-system spending is not permitted without a design revision to this document.

---

## §3 — Hat Definitions (MVP Set)

Four hats at launch. All stats reference the operator's `total_stats()` output — bonuses apply after stage multiplier and growth factor, feeding directly into `calculate_success_chance()`.

### 3.1 Scout Hood
- **Class identity:** Recon specialist. Fast, light, mobile.
- **Stat bonus:** +2 AGI
- **Scrap cost:** 50
- **Unlock condition:** Center node (always available from game start)
- **Roster card label:** `[Name] — Scout Hood`
- **Design note:** The starter hat. Available immediately so the system is introduced early without requiring progression.

### 3.2 Knight Helm
- **Class identity:** Frontline bruiser. Aggressive, high-damage.
- **Stat bonus:** +2 STR
- **Scrap cost:** 100
- **Unlock condition:** Ember Flats scouted (Ring 1 Ember node)
- **Roster card label:** `[Name] — Knight Helm`
- **Design note:** First earned hat. Ember Flats is a Starter-tier scout (DC 4) so this is reachable early.

### 3.3 Mage Hood
- **Class identity:** Tactical, high-INT, mission specialist.
- **Stat bonus:** +2 INT
- **Scrap cost:** 100
- **Unlock condition:** Tide Basin scouted (Ring 1 Tide node)
- **Roster card label:** `[Name] — Mage Hood`
- **Design note:** INT-focused missions (Advanced tier) benefit most from this hat.

### 3.4 Commander Cap
- **Class identity:** Squad leader. Broad competence, not a specialist.
- **Stat bonus:** +1 STR, +1 AGI, +1 INT
- **Scrap cost:** 250
- **Unlock condition:** Gale Ridge scouted (Ring 1 Gale node)
- **Roster card label:** `[Name] — Commander Cap`
- **Design note:** Most expensive hat. Broad bonus makes it valuable in any squad without dominating a specific stat. Natural goal for established players.

### 3.5 Future Hats (Deferred)
The following are acknowledged as future content but are not implemented in this sprint. Do not stub, scaffold, or reference them in code.

- Crystal Spire unlock → precision/INT hat (Reagent-tier feel)
- Tundra Shelf unlock → endurance/STR hat (heavy armor feel)
- Ring 2 and Ring 3 unlocks → culture-specific hats TBD

---

## §4 — Equipment Mechanics

### 4.1 Slot
One "Head" slot per operator. An operator may have zero or one hat equipped at any time. There are no other equipment slots in this sprint.

### 4.2 Equipping and Replacing
When an operator equips a new hat while already wearing one, the previous hat is **returned to inventory** — it is not destroyed. The player's Scrap is not refunded on replacement. Hats are reusable and can be moved between operators.

**Rationale:** Destroying hats on replacement would feel punishing and discourage experimentation. A hat inventory (even a simple one) lets the player build a loadout for their squad.

### 4.3 Hat Inventory
The player owns a pool of purchased hats. Multiple copies of the same hat may be owned. Hat inventory is stored on `GameState` and serialized to `save.json`.

### 4.4 Stat Application
Hat bonuses apply inside `Operator::total_stats()` after the stage multiplier and growth factor are applied:

```
final_stat = floor(base * stage_multiplier * growth_factor) + hat_bonus
```

This means hat bonuses are flat additions, not multiplied by stage. A +2 AGI hat gives exactly +2 AGI regardless of operator level. This keeps hat value consistent across the progression curve.

### 4.5 Mission Calculation
`calculate_success_chance()` calls `op.total_stats()` which already incorporates hat bonuses. No changes to the success formula are required — hats feed into it automatically via the stat chain.

---

## §5 — The Quartermaster (Map Sub-Tab)

### 5.1 Location
The Quartermaster is a sub-tab under the Map tab. Map tab sidebar order:

`Zones | Quartermaster`

(If the Map tab currently has no sidebar, add one following the same pattern as the Roster sidebar.)

### 5.2 Shop Layout
The Quartermaster renders available hats as a vertical scrollable list. Each hat card shows:

```
┌─────────────────────────────────┐
│ Knight Helm          100 Scrap  │
│ +2 STR — Bruiser class          │
│ Unlocked: Ember Flats ✓         │
│ Owned: 1    [BUY] [EQUIP →]    │
└─────────────────────────────────┘
```

- Hats for locked nodes are hidden entirely — not shown as locked/grayed, simply absent
- Once a node is scouted, its hat appears in the shop
- `[BUY]` button is disabled if player has insufficient Scrap
- `[EQUIP →]` opens an operator selection view: list of all operators with their current hat slot, tap to equip

### 5.3 Operator Selection for Equip
When the player taps `[EQUIP →]` on a hat they own:
- Show a list of all non-deployed operators
- Each entry shows: Name, current hat (or "No hat equipped"), current AGI/STR/INT
- Tapping an operator equips the hat to them (removes from previous operator if applicable, returns previous hat to inventory)
- A CANCEL button returns to the shop without changes

### 5.4 Scrap Display
Current Scrap balance is shown in the Quartermaster header:

`QUARTERMASTER | Scrap: 45`

---

## §6 — Data Model

### 6.1 Hat struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hat {
    pub id: HatId,
    pub name: &'static str,
    pub str_bonus: u8,
    pub agi_bonus: u8,
    pub int_bonus: u8,
    pub scrap_cost: u32,
    pub unlock_node_id: usize,  // node that must be scouted to unlock
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HatId {
    ScoutHood,
    KnightHelm,
    MageHood,
    CommanderCap,
}
```

### 6.2 Operator hat slot

Add to `Operator` (not `SlimeGenome` — hats are operational equipment, not genetic):

```rust
pub equipped_hat: Option<HatId>,
```

With `#[serde(default)]` so existing saves deserialize without error.

### 6.3 Hat inventory on GameState

```rust
#[serde(default)]
pub hat_inventory: Vec<HatId>,  // all purchased hats, including duplicates
```

### 6.4 total_stats() modification

```rust
pub fn total_stats(&self) -> (u32, u32, u32) {
    let (base_str, base_agi, base_int) = self.computed_base_stats();
    let hat = self.equipped_hat.as_ref()
        .and_then(|id| Hat::from_id(id));
    let str_bonus = hat.map(|h| h.str_bonus as u32).unwrap_or(0);
    let agi_bonus = hat.map(|h| h.agi_bonus as u32).unwrap_or(0);
    let int_bonus = hat.map(|h| h.int_bonus as u32).unwrap_or(0);
    (base_str + str_bonus, base_agi + agi_bonus, base_int + int_bonus)
}
```

---

## §7 — Persistence

### 7.1 SAVE_VERSION
Bump to 11. Migration from v10:
- `equipped_hat` on all operators defaults to `None` via `serde(default)`
- `hat_inventory` on GameState defaults to `vec![]` via `serde(default)`
- No data loss on existing saves

### 7.2 Migration guard

```rust
if loaded.version < 11 {
    // hat fields handled by serde(default) — no explicit migration needed
    loaded.version = 11;
}
```

---

## §8 — Visual Identity

### 8.1 Roster card
The hat name appears as a small label below the operator name on the roster card. If no hat is equipped, no label is shown (do not show "No hat" — absence is sufficient).

```
Echo  Gale  Lv 3
Mage Hood
STR:4 AGI:5 INT:7
```

### 8.2 Future icon layer (deferred)
Emoji or icon suffix on the operator name is acknowledged as future polish. Not implemented in this sprint. Do not add emoji to the roster card in this sprint — the name label is sufficient for MVP.

---

## §9 — What This System Is Not

Explicitly out of scope for this sprint and not to be implemented, stubbed, or scaffolded:

- Weapon slots, armor slots, accessory slots
- Hat upgrade trees or enhancement systems
- Hat durability or degradation
- Class-gating on missions (a mission requiring a specific hat class)
- Hat abilities or passive effects beyond stat bonuses
- Hat crafting or material combination
- Culture-specific hat bonuses
- Ring 2 or Ring 3 hat unlocks (those nodes are not yet accessible)

These are acknowledged as future content. They require their own design documents before implementation.

---

## §10 — Implementation Sprint Scope (G.3)

When the implementation directive is written, the file scope will be:

| File | Change |
|------|--------|
| `src/models.rs` | Add `Hat` struct, `HatId` enum, `Hat::from_id()`, update `total_stats()` |
| `src/persistence.rs` | Add `hat_inventory` to GameState, `equipped_hat` to Operator, SAVE_VERSION 11, migration |
| `src/world_map.rs` | Add `Hat::catalog()` — full list of available hats with unlock conditions |
| `src/ui/map.rs` or `src/ui/mod.rs` | Add Quartermaster sub-tab to Map tab |
| `src/ui/quartermaster.rs` | New file — shop render, equip flow, operator selection |
| `src/ui/manifest.rs` | Add hat name label to roster card |
| `tests/g3_equipment.rs` | New test file, min 8 tests |

---

*RFD IT Services Ltd. | OperatorGame | SDD-036 | March 2026*
