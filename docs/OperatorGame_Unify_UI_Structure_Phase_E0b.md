# OperatorGame --- Unify UI Structure (Phase E.0b)

**Directive Type:** REFACTORING (cleanup, no new features)  
**Scope:** Remove desktop 3-column override, enforce 4-tab structure uniformly  
**Test Floor:** 210 → 215 passing (5 new consistency tests)  
**Acceptance:** Desktop and mobile render identical 4-tab layout; sub-tabs functional per tab  

---

## Goal

Eliminate the legacy 3-column desktop override (Manifest | Ops | Contracts) and unify the UI around the 4-tab bottom navigation structure. Both desktop and mobile will use the same tab dispatch logic.

---

## Phase E.0b.1: Identify & Remove Desktop Override

### Task 1.1: Locate Desktop Layout Code

**Location:** `src/ui/mod.rs`

**Find:**
- Any code that checks `ctx.available_rect().width()` or screen size to trigger 3-column layout
- Any conditional that says "if desktop, use 3-column" or similar
- Any logic that bypasses `active_tab` dispatch in favor of hardcoded column rendering

**Report:**
- Line numbers where override exists
- What conditions trigger it
- What content it renders (Manifest, Ops, Contracts modules)

---

### Task 1.2: Remove Desktop Override

**Delete/Comment Out:**
- Any multi-column layout code that's independent of `active_tab`
- Any desktop-specific branching in the CentralPanel render logic

**Pattern to remove (example):**
```rust
// OLD (REMOVE THIS):
if ctx.available_rect().width() > 1200.0 {
    // Desktop 3-column layout
    ui.columns(3, |columns| {
        render_manifest(&mut columns[0], ...);
        render_ops(&mut columns[1], ...);
        render_contracts(&mut columns[2], ...);
    });
} else {
    // Mobile tab dispatch
    match self.active_tab { ... }
}
```

**Replace with:**
```rust
// NEW (UNIFIED):
match self.active_tab {
    BottomTab::Roster => { render_manifest(...); }
    BottomTab::Missions => { render_ops(...); }
    // etc.
}
```

---

## Phase E.0b.2: Verify Tab Dispatch Consistency

### Task 2.1: Map Content to Tabs

**Confirm each tab renders the correct module:**

| Bottom Tab | Module | Render Function | Status |
|-----------|--------|-----------------|--------|
| Roster | manifest.rs | render_manifest() | Should show slimes, stats, breeding UI |
| Missions | ops.rs + contracts.rs | render_active_ops() + render_contracts() | Should show active missions + quest board |
| Map | radar.rs | render_radar() | Should show world map / zones |
| Logs | mod.rs | render_combat_log() | Should show mission history / AAR |

**Verify each render function:**
- Takes `&mut egui::Ui` as parameter
- Does NOT reference screen width or responsive breakpoints
- Does NOT contain its own tab/sub-tab logic (that should be in parent)

---

### Task 2.2: Test Tab Switching

**Manual verification:**
- Click Roster → manifest content appears
- Click Missions → ops + contracts appear (or separate sub-tabs for Active/QuestBoard?)
- Click Map → radar appears
- Click Logs → combat log appears
- Switch back to Roster → manifest is still there (state persists)

**Report:**
- Each tab switches successfully? YES/NO
- Content renders without panics? YES/NO
- Sub-tabs visible (left sidebar)? YES/NO
- Sub-tab selection works? YES/NO

---

## Phase E.0b.3: Sub-Tab Rendering (Verify Phase C Implementation)

### Task 3.1: Vertical Sub-Menu Sidebar

**Confirm:**
- Left sidebar (100dp width) renders below top bar
- Sub-tabs are listed vertically
- Clicking a sub-tab changes state (e.g., Collection → Breeding)
- Sub-tab selection is visually distinct (highlight active)

**Example structure:**
```
┌────────────────────────────────┐
│ Top Bar (stats, resources)      │
├──────────┬──────────────────────┤
│ Sub-Tabs │ Main Content Area    │
│ ─────    │                      │
│ Coll.    │ (Roster tab active)  │
│ Breed.   │ Shows slime list     │
│          │                      │
│          │                      │
├──────────┴──────────────────────┤
│ Bottom Tab Bar: [Roster] [Miss] │
└────────────────────────────────┘
```

**Verify:**
- Sub-tabs render? YES/NO
- Sidebar width correct (100dp)? YES/NO
- Content area resizes when sidebar present? YES/NO
- Each main tab has its own sub-tabs? YES/NO

---

### Task 3.2: Sub-Tab Content Dispatch

**Verify each sub-tab switches content:**

**Roster tab:**
- Collection sub-tab → render_manifest() with collection view
- Breeding sub-tab → render_incubator() with breeding pairs

**Missions tab:**
- Active sub-tab → render_active_ops() (in-progress missions)
- QuestBoard sub-tab → render_contracts() (available missions)

**Map tab:**
- Zones sub-tab → render_radar() (world map)

**Logs tab:**
- MissionHistory sub-tab → render_combat_log()
- CultureHistory sub-tab → render_culture_log() [stub or extend combat_log]

**Report:**
```
Roster:
  - Collection renders: YES/NO
  - Breeding renders: YES/NO
Missions:
  - Active renders: YES/NO
  - QuestBoard renders: YES/NO
Map:
  - Zones renders: YES/NO
Logs:
  - MissionHistory renders: YES/NO
  - CultureHistory renders: YES/NO
```

---

## Phase E.0b.4: Layout & Safe Area

### Task 4.1: Verify Safe Area Margins

**Confirm:**
- Top panel inset by `safe_area.top` (below status bar)
- Bottom tab bar inset by `safe_area.bottom` (above soft buttons)
- Central panel respects both insets
- Left sidebar respects top inset

**Visual check on Moto G portrait:**
- "OPERATOR: COMMAND DECK" text is not clipped by status bar ✓
- Bottom tabs are not clipped by soft buttons ✓
- Sidebar doesn't overlap top bar ✓

---

### Task 4.2: Verify Responsive Sizing

**On desktop (wide screen):**
- Left sidebar is 100dp (or resizable?)
- Content area fills remaining width
- Tabs stay at bottom
- Layout is readable without horizontal scroll

**On mobile (narrow screen):**
- Same layout (unified)
- Text is readable (font size appropriate)
- Buttons are touch-friendly (44dp minimum)

---

## Phase E.0b.5: Tests

### Location: `tests/ui_structure.rs` (NEW or extend existing)

```rust
#[test]
fn test_4_tabs_render_without_desktop_override() {
    let app = create_test_app();
    // Verify active_tab dispatch works for all 4 tabs
    // (This is a structural test, not a visual test)
}

#[test]
fn test_tab_switch_persists_state() {
    let mut app = create_test_app();
    app.active_tab = BottomTab::Roster;
    app.roster_sub_tab = RosterSubTab::Breeding;
    
    // Switch to Missions, then back to Roster
    app.active_tab = BottomTab::Missions;
    app.active_tab = BottomTab::Roster;
    
    // Roster sub-tab should still be Breeding
    assert_eq!(app.roster_sub_tab, RosterSubTab::Breeding);
}

#[test]
fn test_sub_tabs_independent_per_main_tab() {
    let mut app = create_test_app();
    
    app.active_tab = BottomTab::Roster;
    app.roster_sub_tab = RosterSubTab::Breeding;
    
    app.active_tab = BottomTab::Missions;
    app.missions_sub_tab = MissionsSubTab::QuestBoard;
    
    app.active_tab = BottomTab::Roster;
    // Should still be Breeding, not QuestBoard
    assert_eq!(app.roster_sub_tab, RosterSubTab::Breeding);
}

#[test]
fn test_safe_area_applied_to_panels() {
    // Verify safe_area margins are applied to top/bottom panels
}

#[test]
fn test_sidebar_width_100dp() {
    // Verify left sidebar renders at 100dp width
}

#[test]
fn test_no_3_column_override_on_desktop() {
    // Verify 3-column logic is removed or disabled
}
```

---

## Test Floor

**Before:** 210 passing  
**Target:** 215 passing (5 new structure tests)

---

## Acceptance Criteria

✓ No 3-column desktop override code remains (or is fully disabled)  
✓ All 4 tabs (Roster | Missions | Map | Logs) dispatch content via `active_tab`  
✓ Sub-tabs render vertically on left sidebar (100dp width)  
✓ Sub-tabs switch content correctly (Collection ↔ Breeding, Active ↔ QuestBoard, etc.)  
✓ Sub-tab state persists when switching main tabs  
✓ Safe area margins applied to top bar (below status bar)  
✓ Safe area margins applied to bottom bar (above soft buttons)  
✓ Desktop and mobile layouts are identical (same code path)  
✓ Left sidebar doesn't overlap top bar  
✓ Central content area is readable on both desktop and mobile  
✓ All 5 new tests passing  
✓ Test floor: 215 / 215 (zero regressions)  
✓ Code compiles without warnings  
✓ Visual verification on Moto G (no clipping, all tabs clickable)  

---

## Notes for Agent

- **Don't build new features.** This is cleanup only.
- **Safe area is already wired** (from Phase B). Just verify it's applied to the right panels.
- **Sub-tabs are scaffolded** (from Phase C). Just verify they render and switch correctly.
- **No content changes.** manifest.rs, ops.rs, etc. stay as-is. We're just changing *how* they're dispatched.

---

## Deliverables

1. **Refactored `src/ui/mod.rs`** — Remove desktop override, unified tab dispatch
2. **Updated `tests/ui_structure.rs`** — 5 new consistency tests
3. **Visual verification report** — Screenshot from desktop showing 4-tab layout
4. **APK build** — Test on Moto G (confirm no regressions from Phase B/C)

---

## Completion Checklist

- [ ] 3-column override removed/disabled from ui/mod.rs
- [ ] All 4 tabs dispatch via active_tab match statement
- [ ] Left sidebar renders 100dp width with sub-tabs
- [ ] Sub-tabs switch content correctly
- [ ] Sub-tab state persists across main tab switches
- [ ] Safe area margins verified (top + bottom)
- [ ] All 5 new tests passing
- [ ] Test floor: 215 / 215
- [ ] Desktop and mobile layouts identical
- [ ] Visual check on Moto G (no clipping, clickable tabs)
