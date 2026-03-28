# SDD-043: Equipment System & Combat Logs

**Status:** Draft / Directive
**Sprint:** G.3 & G.4
**Context:** Mobile-first feature velocity enabled by Windows Emulation.

## 🎯 Primary Directive: G.3 Equipment System

### Phase 1: Architecture & Data Model
**Objective:** Scaffold the data structures and mobile-first UI flow (400×800 portrait).

1. **Inspection:**
   - Check `src/models.rs` or `src/persistence.rs` for existing item/equipment stubs.
   - Scaffold `src/equipment/` if needed.
2. **UI Flow Design:**
   - **Screen 1: Slots**: Vertical list (Head, Chest, Hands, Feet, Accessory) with 44dp+ tap targets.
   - **Screen 2: Inventory**: Filtered grid/list for item selection.
   - **Screen 3: Detail**: Comparison view with "Equip/Unequip" actions.
3. **State Machine:**
   - `Slots` → tap empty slot → `Inventory` (filtered) → tap item → `Detail` → `Equip` → `Slots`.

### Phase 2: Core Data & Save Integration
**Objective:** Persistence and serialization logic.

```rust
pub struct Equipment {
    pub slots: HashMap<EquipSlot, Option<Item>>,
    pub inventory: Vec<Item>,
}

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EquipSlot { Head, Chest, Hands, Feet, Accessory }

pub struct Item {
    pub id: u32,
    pub name: String,
    pub stats: ItemStats,
}
```

- **Persistence:** Ensure `Equipment` is part of `GameState` and saves to `save.json`.
- **Validation:** Test cross-platform save integrity using `.\run_mobile.ps1` and `.\run_desktop.ps1`.

### Phase 3: Minimal Viable UI
**Objective:** High-fidelity egui implementation for compact and standard layouts.

- **Slots View:** 5 vertical boxes, 12px gaps, clear "Empty" states.
- **Inventory Grid:** Scrollable list with icon placeholders and condensed stats.
- **Detail View:** Full stat breakdown with primary action buttons (Green Equip / Red Unequip).

---

## 🎯 Secondary Directive: G.4 Combat Logs

### Phase 1: Data Model & UI Layout
**Objective:** Narrative and tactical feedback system.

1. **Log Structure:**
```rust
pub struct CombatLogEntry {
    pub timestamp: u64,
    pub actor: String,
    pub action: String,
    pub target: String,
    pub result: String,
    pub outcome: LogOutcome, // Victory, Failure, Neutral
}
```

2. **UI Wireframe:**
   - Scrollable newest-first list.
   - Monospace font for alignment.
   - Color coding: Green (Heal/Buff), Red (Damage/Debuff), Gray (Miss/System).

3. **Integration:**
   - Combat resolution logic must emit these entries to `GameState`.
   - Persistence in `save.json`.

---

## 🔁 Workflow Cadence (The 5-Second Loop)

1. **Code:** Minor edit (2 min).
2. **Mobile Test:** `.\run_mobile.ps1` (Check 400×800 layout).
3. **Desktop Test:** `.\run_desktop.ps1` (Check 1200×800 regression).
4. **Iterate:** Fix clipping/sizing immediately.
5. **Commit:** Only when both modes pass visual inspection.

---

## 🎨 Design Constraints

- ✅ **Mobile-First:** Design for 400×800 first; let standard layout be the secondary.
- ✅ **Touch Safety:** 44dp (approx 110px at 2.0 DPI) minimum targets.
- ✅ **Verticality:** Favor vertical scrolling over horizontal swiping.
- ❌ **No Hover:** Avoid tooltips or states that rely on mouse-over.

---

## 📋 Success Criteria

- [ ] Equipment serializes to `save.json`.
- [ ] Equip/Unequip flow is functional in mobile emulation.
- [ ] UI is responsive across both build profiles.
- [ ] Combat logs persist and scroll correctly.
- [ ] All tests in `src/platform.rs` pass.
