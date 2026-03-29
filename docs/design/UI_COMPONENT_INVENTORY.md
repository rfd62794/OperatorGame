# OperatorGame UI Component Inventory
**Status:** Phase F.1 Active  
**Purpose:** Provide the Designer/Vision Agent with the exact structural components existing in the MVP Rust codebase prior to visual constraint analysis.

## Global Viewport Hierarchy (Moto G 2025 Mobile View)

1. **Top Status Bar (`TopBottomPanel::top`)**
   - Title: "OPERATOR: COMMAND DECK" 
   - **Metrics:** Bank Balance, Daily Upkeep Forecast, GEL (Biomass), Scrap (Scrap), Reagents.
   - **Health/Meta:** "RESONANCE STRESS" progress bar.

2. **Main Content Area (`CentralPanel`)**
   - Consumes the vast majority of the screen.
   - Dynamic: Switches contents entirely based on the Active Bottom Tab and active Sub-Tab.
   - *Has an internal left-aligned Sidebar (`ui.vertical`) handling sub-tabs (width 80dp/100dp).*

3. **Bottom Tab Bar (`egui::Area` pinned to safe bottom rect)**
   - 4 equal-width Touch Target Slots.
   - Icons + Text labels. Active tab gets a green `#69fea5` left strip.

4. **Combat Log Overlay (Expandable Bottom Panel above tabs)**
   - Header: "── COMBAT LOG ──"
   - Button: "Clear"
   - Scrollable text history of Mission deployments.

---

## Tab 1: Roster (`BottomTab::Roster`)
Controlled by: `src/ui/manifest.rs`

### Sub-tab A: Collection (Default)
Renders: `render_manifest(ui)`
- **Components Expected:** Operator cards (grid/list layout). HP, Mind, Stats, Slime level/XP bars.

### Sub-tab B: Breeding
Renders: `render_incubator(ui)`
- **Components Expected:** Slime-pairing interfaces, genetic combination preview, "Incubate" action button, hatch timers.

---

## Tab 2: Missions (`BottomTab::Missions`)
Controlled by: `src/ui/ops.rs` and `src/ui/contracts.rs`

### Sub-tab A: Active (Default)
Renders: `render_active_ops(ui)`
- **Components Expected:** Ongoing deployments, countdown timers, Squad health preview.

### Sub-tab B: QuestBoard
Renders: `render_contracts(ui)`
- **Components Expected:** Available missions list, Difficulty ratings, Reward cash estimates, "Select/Deploy" action buttons.

---

## Tab 3: Map (`BottomTab::Map`)
Controlled by: `src/ui/world_map.rs` / `src/ui/radar.rs`

### Sub-tab A: Zones (Default)
Renders: `render_radar(ui)`
- **Components Expected:** Visual or list-based area traversal, Startled/Stress metrics by zone.

---

## Tab 4: Logs (`BottomTab::Logs`)

### Sub-tab A: MissionHistory (Default)
Renders: Inline text loop inside `mod.rs`
- **Components Expected:** Colored text readouts of past operations (Green for Victory, Red for Critical Failure).

### Sub-tab B: CultureHistory
Renders: Simple Label
- **Status:** Currently unimplemented (`[TODO] Logs -> Culture History`).
