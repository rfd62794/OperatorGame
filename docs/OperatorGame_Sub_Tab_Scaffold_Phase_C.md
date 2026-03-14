# OperatorGame --- Sub-Tab Scaffold (Phase C)

**Directive Type:** IMPLEMENTATION  
**Scope:** Add vertical sub-tab enums + dispatch logic (no content rendering yet)  
**Test Floor:** 145 → 157 passing (12 new scaffold tests)  
**Acceptance:** Sub-tab state tracking + dispatch wired; content stubs render "TODO" placeholders  

---

## Goal

Implement the structural scaffold for vertical sub-tabs under each main tab (Roster | Missions | Map | Logs). Prepare dispatch logic for future content, no rendering yet.

---

## Phase C.1: Define Sub-Tab Enums

### Location: `src/platform.rs` (after BottomTab definition)

Add four new enums:

```rust
/// Roster sub-tabs: genetics, breeding, collection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RosterSubTab {
    Collection,   // All slimes, genetics tree
    Breeding,     // Pair selection, timers, hatch notifications
    // Reserved for future expansion
}

impl Default for RosterSubTab {
    fn default() -> Self {
        Self::Collection
    }
}

/// Missions sub-tabs: active expeditions, quest board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionsSubTab {
    Active,       // Ongoing expeditions, timers, returns
    QuestBoard,   // Available missions, target selection
    // Reserved for future expansion
}

impl Default for MissionsSubTab {
    fn default() -> Self {
        Self::Active
    }
}

/// Map sub-tabs: currently flat, reserved for expansion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapSubTab {
    Zones,        // Zone affinity, resource yields
    // Reserved: Alliances, Trade, Procedural
}

impl Default for MapSubTab {
    fn default() -> Self {
        Self::Zones
    }
}

/// Logs sub-tabs: mission history, culture/trade history, operational records
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogsSubTab {
    MissionHistory,    // AAR outcomes, rolls, narrative
    CultureHistory,    // Culture recruitment, standing changes
    // Reserved: OpLog, CargoLog (if Ops/Cargo restore needed)
}

impl Default for LogsSubTab {
    fn default() -> Self {
        Self::MissionHistory
    }
}
```

---

## Phase C.2: Add Sub-Tab State to OperatorApp

### Location: `src/ui/mod.rs` → `pub struct OperatorApp`

```rust
pub struct OperatorApp {
    // ... existing fields (active_tab, left_tab, right_tab, mobile_tab)
    
    // Sub-tab state (one per main tab)
    roster_sub_tab: RosterSubTab,
    missions_sub_tab: MissionsSubTab,
    map_sub_tab: MapSubTab,
    logs_sub_tab: LogsSubTab,
}
```

### In `OperatorApp::new()`:

```rust
impl OperatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // ... existing init
            active_tab: BottomTab::Roster,
            
            // Initialize sub-tabs to defaults
            roster_sub_tab: RosterSubTab::default(),
            missions_sub_tab: MissionsSubTab::default(),
            map_sub_tab: MapSubTab::default(),
            logs_sub_tab: LogsSubTab::default(),
        }
    }
}
```

---

## Phase C.3: Render Sub-Tab Bar (Vertical)

### Location: `src/ui/mod.rs` → In `OperatorApp::update()`, after BottomTab rendering

Add a helper function first:

```rust
fn render_sub_tabs(ui: &mut egui::Ui, active_main_tab: BottomTab, app: &mut OperatorApp) {
    ui.vertical(|ui| {
        ui.label("─────────────");  // Visual separator
        
        match active_main_tab {
            BottomTab::Roster => {
                ui.label("Roster");
                
                if ui.selectable_label(
                    app.roster_sub_tab == RosterSubTab::Collection,
                    "Collection"
                ).clicked() {
                    app.roster_sub_tab = RosterSubTab::Collection;
                }
                
                if ui.selectable_label(
                    app.roster_sub_tab == RosterSubTab::Breeding,
                    "Breeding"
                ).clicked() {
                    app.roster_sub_tab = RosterSubTab::Breeding;
                }
            }
            
            BottomTab::Missions => {
                ui.label("Missions");
                
                if ui.selectable_label(
                    app.missions_sub_tab == MissionsSubTab::Active,
                    "Active"
                ).clicked() {
                    app.missions_sub_tab = MissionsSubTab::Active;
                }
                
                if ui.selectable_label(
                    app.missions_sub_tab == MissionsSubTab::QuestBoard,
                    "Quest Board"
                ).clicked() {
                    app.missions_sub_tab = MissionsSubTab::QuestBoard;
                }
            }
            
            BottomTab::Map => {
                ui.label("Map");
                
                if ui.selectable_label(
                    app.map_sub_tab == MapSubTab::Zones,
                    "Zones"
                ).clicked() {
                    app.map_sub_tab = MapSubTab::Zones;
                }
            }
            
            BottomTab::Logs => {
                ui.label("Logs");
                
                if ui.selectable_label(
                    app.logs_sub_tab == LogsSubTab::MissionHistory,
                    "Mission History"
                ).clicked() {
                    app.logs_sub_tab = LogsSubTab::MissionHistory;
                }
                
                if ui.selectable_label(
                    app.logs_sub_tab == LogsSubTab::CultureHistory,
                    "Culture History"
                ).clicked() {
                    app.logs_sub_tab = LogsSubTab::CultureHistory;
                }
            }
        }
    });
}
```

### Then, in the `CentralPanel` content area:

**Current structure (simplified):**
```rust
egui::CentralPanel::default()
    .frame(/* ... margins ... */)
    .show(ctx, |ui| {
        match self.active_tab {
            BottomTab::Roster => {
                ui.label("Roster Content");
            }
            // ... other tabs
        }
    });
```

**Updated structure (with sub-tabs):**
```rust
egui::CentralPanel::default()
    .frame(/* ... margins ... */)
    .show(ctx, |ui| {
        // Horizontal layout: sub-tabs on left, content on right
        ui.horizontal(|ui| {
            // Left sidebar: sub-tab navigation
            ui.vertical(|ui| {
                ui.set_width(100.0);  // Sub-tab menu width
                render_sub_tabs(ui, self.active_tab, self);
            });
            
            // Right side: content area (vertical separator)
            ui.separator();
            
            ui.vertical(|ui| {
                // Content dispatch based on active_tab + active sub_tab
                match self.active_tab {
                    BottomTab::Roster => {
                        match self.roster_sub_tab {
                            RosterSubTab::Collection => {
                                ui.label("[TODO] Roster → Collection");
                            }
                            RosterSubTab::Breeding => {
                                ui.label("[TODO] Roster → Breeding");
                            }
                        }
                    }
                    
                    BottomTab::Missions => {
                        match self.missions_sub_tab {
                            MissionsSubTab::Active => {
                                ui.label("[TODO] Missions → Active");
                            }
                            MissionsSubTab::QuestBoard => {
                                ui.label("[TODO] Missions → Quest Board");
                            }
                        }
                    }
                    
                    BottomTab::Map => {
                        match self.map_sub_tab {
                            MapSubTab::Zones => {
                                ui.label("[TODO] Map → Zones");
                            }
                        }
                    }
                    
                    BottomTab::Logs => {
                        match self.logs_sub_tab {
                            LogsSubTab::MissionHistory => {
                                ui.label("[TODO] Logs → Mission History");
                            }
                            LogsSubTab::CultureHistory => {
                                ui.label("[TODO] Logs → Culture History");
                            }
                        }
                    }
                }
            });
        });
    });
```

---

## Phase C.4: Persistence (Save Sub-Tab State)

### Location: `src/persistence.rs` → `GameState` struct

Add sub-tab state to the saved game:

```rust
pub struct GameState {
    // ... existing fields
    
    // UI state (persisted across sessions)
    pub active_tab: BottomTab,
    pub roster_sub_tab: RosterSubTab,
    pub missions_sub_tab: MissionsSubTab,
    pub map_sub_tab: MapSubTab,
    pub logs_sub_tab: LogsSubTab,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            // ... existing defaults
            active_tab: BottomTab::Roster,
            roster_sub_tab: RosterSubTab::default(),
            missions_sub_tab: MissionsSubTab::default(),
            map_sub_tab: MapSubTab::default(),
            logs_sub_tab: LogsSubTab::default(),
        }
    }
}
```

### Update `OperatorApp::load_game()` and `save_game()`:

```rust
// In OperatorApp::load_game():
self.active_tab = game_state.active_tab;
self.roster_sub_tab = game_state.roster_sub_tab;
self.missions_sub_tab = game_state.missions_sub_tab;
self.map_sub_tab = game_state.map_sub_tab;
self.logs_sub_tab = game_state.logs_sub_tab;

// In OperatorApp::save_game():
game_state.active_tab = self.active_tab;
game_state.roster_sub_tab = self.roster_sub_tab;
game_state.missions_sub_tab = self.missions_sub_tab;
game_state.map_sub_tab = self.map_sub_tab;
game_state.logs_sub_tab = self.logs_sub_tab;
```

---

## Phase C.5: Tests

### Location: `tests/ui_layout.rs` (extend existing test suite)

Add 12 new tests:

```rust
#[test]
fn test_roster_sub_tab_default() {
    let app = create_test_app();
    assert_eq!(app.roster_sub_tab, RosterSubTab::Collection);
}

#[test]
fn test_roster_sub_tab_switch_breeding() {
    let mut app = create_test_app();
    app.roster_sub_tab = RosterSubTab::Breeding;
    assert_eq!(app.roster_sub_tab, RosterSubTab::Breeding);
}

#[test]
fn test_missions_sub_tab_default() {
    let app = create_test_app();
    assert_eq!(app.missions_sub_tab, MissionsSubTab::Active);
}

#[test]
fn test_missions_sub_tab_switch_quest_board() {
    let mut app = create_test_app();
    app.missions_sub_tab = MissionsSubTab::QuestBoard;
    assert_eq!(app.missions_sub_tab, MissionsSubTab::QuestBoard);
}

#[test]
fn test_map_sub_tab_default() {
    let app = create_test_app();
    assert_eq!(app.map_sub_tab, MapSubTab::Zones);
}

#[test]
fn test_logs_sub_tab_default() {
    let app = create_test_app();
    assert_eq!(app.logs_sub_tab, LogsSubTab::MissionHistory);
}

#[test]
fn test_logs_sub_tab_switch_culture_history() {
    let mut app = create_test_app();
    app.logs_sub_tab = LogsSubTab::CultureHistory;
    assert_eq!(app.logs_sub_tab, LogsSubTab::CultureHistory);
}

#[test]
fn test_sub_tab_persistence_serialize() {
    let mut game_state = GameState::default();
    game_state.roster_sub_tab = RosterSubTab::Breeding;
    let json = serde_json::to_string(&game_state).unwrap();
    let restored: GameState = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.roster_sub_tab, RosterSubTab::Breeding);
}

#[test]
fn test_sub_tab_persistence_deserialize() {
    let json = r#"{"roster_sub_tab": "Breeding", ...}"#;
    let game_state: GameState = serde_json::from_str(json).unwrap();
    assert_eq!(game_state.roster_sub_tab, RosterSubTab::Breeding);
}

#[test]
fn test_sub_tabs_independent_per_main_tab() {
    let mut app = create_test_app();
    app.active_tab = BottomTab::Roster;
    app.roster_sub_tab = RosterSubTab::Breeding;
    
    app.active_tab = BottomTab::Missions;
    app.missions_sub_tab = MissionsSubTab::QuestBoard;
    
    // Switch back to Roster — sub-tab should persist
    app.active_tab = BottomTab::Roster;
    assert_eq!(app.roster_sub_tab, RosterSubTab::Breeding);
}

#[test]
fn test_render_sub_tabs_roster() {
    // Verify render_sub_tabs produces correct UI buttons for Roster
    // (This is a UI snapshot test; may use egui testing tools)
}

#[test]
fn test_render_sub_tabs_missions() {
    // Verify render_sub_tabs produces correct UI buttons for Missions
}

#[test]
fn test_render_sub_tabs_logs() {
    // Verify render_sub_tabs produces correct UI buttons for Logs
}
```

---

## Test Floor

**Before:** 145 passing  
**Target:** 157 passing (12 new scaffold tests)

---

## Acceptance Criteria

✓ All 4 sub-tab enums defined (`RosterSubTab`, `MissionsSubTab`, `MapSubTab`, `LogsSubTab`)  
✓ Each enum has `Default` implementation  
✓ Sub-tab state added to `OperatorApp` struct (4 fields)  
✓ Sub-tab state persisted in `GameState` (serialization/deserialization)  
✓ `render_sub_tabs()` function produces vertical menu per main tab  
✓ Content dispatch wired: main_tab + sub_tab → "[TODO]" placeholder  
✓ Sub-tabs persist when switching between main tabs  
✓ All 12 new tests passing  
✓ Test floor: 157 / 157 (zero regressions)  
✓ Code compiles to aarch64 + armv7 without warnings  

---

## Notes for Agent

- **No content rendering yet.** Just "[TODO] Tab → SubTab" labels.
- **Sub-tab state persists.** If user selects Roster → Breeding, then switches to Missions, then back to Roster, they should see Breeding still selected.
- **Enums are extensible.** Reserved comments in each enum mark where future sub-tabs can be added (e.g., `// Reserved: Alliances, Trade, Procedural` in MapSubTab).
- **UI layout:** Left sidebar (vertical sub-tabs) + right content area (horizontal separator between them). Width is fixed at 100dp for now; can be made responsive later.
- **Serialization:** Sub-tab state should serialize cleanly (serde will derive this automatically on the enums).

---

## Deliverables

1. **Updated `src/platform.rs`** — 4 new sub-tab enums with defaults
2. **Updated `src/ui/mod.rs`** — OperatorApp sub-tab fields + render_sub_tabs() helper + content dispatch
3. **Updated `src/persistence.rs`** — GameState sub-tab fields + load/save logic
4. **Extended `tests/ui_layout.rs`** — 12 new scaffold tests
5. **APK build** — Verify on Moto G 2025 (visual: left sidebar with sub-menus, content area with TODO stubs)

---

## Completion Checklist

- [ ] All 4 sub-tab enums defined with defaults
- [ ] Sub-tab fields added to OperatorApp
- [ ] Sub-tab fields added to GameState
- [ ] render_sub_tabs() function implemented
- [ ] Content dispatch wired (main_tab + sub_tab → placeholder)
- [ ] Sub-tab persistence logic added (load/save)
- [ ] All 12 scaffold tests passing
- [ ] Test floor: 157 / 157
- [ ] APK builds and runs on Moto G
- [ ] Sub-tab navigation visually functional (selection state, persistence)
