# OperatorGame --- UI Layout Audit & Responsive Sizing Fix

**Directive Type:** ANALYSIS + QUICK FIX (no new features)  
**Goal:** Identify UI sizing issues, fix layout bugs, enable smart responsive sizing  
**Output:** Audit report + fixed layout code  

---

## Phase A: Layout Audit

### Task A.1: Screen Real Estate Inventory

**On Moto G 2025 (portrait mode):**
- Total screen: 412dp width × 1900dp height (approximate usable area)
- Status bar: 48dp (top, reserved by Android)
- Soft buttons: 56dp (bottom, reserved by Android)
- **Available to app:** 412dp × ~1796dp

**Current layout allocation:**
- Top bar (OPERATOR: COMMAND DECK): X dp height
- Sidebar (sub-tabs): 100dp width (fixed)
- Content area: (412 - 100) = 312dp width
- Bottom tab bar: X dp height

**Report:**
```
AVAILABLE SPACE (Portrait, Moto G):
- Screen: 412dp wide × 1796dp tall (safe area)
- Top bar: [measure current height] dp
- Bottom bar: [measure current height] dp
- Sidebar: 100dp wide (fixed) | [measure height]
- Content area: 312dp wide × [measure height] tall

ISSUE #1: Map Nodes Hidden
- Where do they appear in the UI?
- Are they behind a panel? Off-screen? Clipped?
- What resolution does radar.rs expect?

ISSUE #2: Card Grid Sizing
- Current card dimensions: [measure width × height]
- Cards per row on 312dp: [count]
- Is this optimal, or too cramped/too sparse?

ISSUE #3: Sidebar vs. Content
- Sidebar takes 100dp, leaves 312dp for content.
- Is 100dp too wide? Does it crush content on narrow screens?
- Should sidebar be collapsible or resizable?
```

---

### Task A.2: Panel Layout Trace

**In `src/ui/mod.rs`, document:**

1. **Top Panel** (`TopBottomPanel::top`)
   - Current height (hardcoded or calculated?)
   - Safe area margin applied? (safe_area.top)
   - Content: What's rendered? (header, resource counters, etc.)

2. **Central Panel** (`CentralPanel`)
   - Current structure:
     ```
     Horizontal layout:
     - Left: Sidebar (100dp) → Vertical sub-tabs
     - Right: Content area → Match on active_tab
     ```
   - Is content area width calculated? Fixed?
   - Is there padding/margin around content?

3. **Bottom Panel** (`TopBottomPanel::bottom`)
   - Current height (hardcoded or calculated?)
   - Safe area margin applied? (safe_area.bottom)
   - Content: 4-tab buttons

**Report findings:**
```
TOP PANEL:
- Height: [X] dp (hardcoded | calculated)
- Safe area margin: YES/NO
- Content: [description]

CENTRAL PANEL:
- Layout: Horizontal (sidebar | content)
- Sidebar width: 100dp (fixed)
- Content width: [calculated or fixed]?
- Padding/margin: [X] dp

BOTTOM PANEL:
- Height: [X] dp (hardcoded | calculated)
- Safe area margin: YES/NO
- Tab button sizing: [X] dp each
```

---

### Task A.3: Map Rendering Issue (radar.rs)

**Locate the map rendering:**
- Is `render_radar()` called in the Map tab?
- What does it render? (egui::Painter? Custom canvas?)
- What are the expected dimensions? (width, height)
- Is it positioned with `egui::Area` or within `CentralPanel`?

**If using egui::Area:**
```rust
egui::Area::new("map")
    .fixed_pos([X, Y])
    .show(ctx, |ui| {
        // render_radar() called here?
    });
```

**Check:**
- Is the Area positioned off-screen? (negative X/Y, or beyond screen bounds)
- Is the Area width/height too small or too large?
- Is the Area behind another panel (z-order issue)?

**Report:**
```
MAP RENDERING:
- Function: render_radar() in [file]
- Called from: Map tab in OperatorApp::update()
- Layout: egui::Area | CentralPanel | Custom?
- Dimensions: Width X, Height Y (hardcoded | calculated)
- Position: [X, Y] (fixed | calculated)
- Visible: YES/NO (If NO, why? Off-screen, clipped, z-order?)
```

---

## Phase B: Responsive Sizing Strategy

### Task B.1: Define Breakpoints

**Mobile-first responsive design:**

```
Breakpoint 1 (Ultra-Narrow: <300dp):
  - Sidebar: 80dp (smaller)
  - Content: Very constrained

Breakpoint 2 (Narrow: 300-400dp):
  - Sidebar: 100dp
  - Content: 200-300dp
  - Cards: 1-2 per row

Breakpoint 3 (Standard: 400-600dp):
  - Sidebar: 100dp (or collapsible)
  - Content: 300-500dp
  - Cards: 2-3 per row

Breakpoint 4 (Wide: 600-800dp):
  - Sidebar: 100dp (or side-by-side layout change)
  - Content: 500-700dp
  - Cards: 3-4 per row

Breakpoint 5 (Desktop: >800dp):
  - Sidebar: 100dp or wider
  - Content: Fill remaining
  - Cards: 4+ per row
```

**Question for agent:** What screen widths are we actually testing on? (Desktop window, Moto G, tablet?)

---

### Task B.2: Smart Card Sizing

**Current card grid:**
- Cards rendered with `horizontal_wrapped`
- Fixed card size? Or responsive?

**New strategy:**
```rust
// Calculate available width for cards
let available_width = ui.available_width(); // Accounts for sidebar
let card_width = calculate_card_width(available_width);
let cards_per_row = (available_width / card_width).floor() as usize;

// Render cards
ui.horizontal_wrapped(|ui| {
    ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
    ui.set_width(available_width);
    
    for op in &self.state.slimes {
        if ui.available_width() < card_width {
            ui.end_row();
        }
        // render_operator_card with calculated width
    }
});

fn calculate_card_width(available_width: f32) -> f32 {
    // Return card width that fits 2-4 per row based on available space
    match available_width {
        w if w < 250.0 => 120.0,  // Ultra-narrow: 1-2 cards
        w if w < 400.0 => 140.0,  // Narrow: 2-3 cards
        w if w < 600.0 => 160.0,  // Standard: 3-4 cards
        _ => 180.0,                // Wide: 4+ cards
    }
}
```

**Questions:**
- Should card height also be responsive, or fixed?
- Should font size scale? (10pt on mobile, 12pt on desktop?)
- Should sidebar width be fixed (100dp) or responsive?

---

### Task B.3: Panel Spacing & Margins

**Current spacing (guesses):**
- Top bar height: 60dp?
- Bottom bar height: 48dp?
- Sidebar width: 100dp?
- Content padding: 8dp?

**Audit & optimize:**
```rust
// Top bar
let top_bar_height = 60.0;
let top_safe_margin = safe_area.top;

// Sidebar
let sidebar_width = 100.0;

// Content area
let content_width = ui.available_width() - sidebar_width;
let content_height = ui.available_height(); // Remaining after top/bottom

// Bottom bar
let bottom_bar_height = 48.0;
let bottom_safe_margin = safe_area.bottom;

// Calculate card grid height
let usable_height = screen_height - top_bar_height - bottom_bar_height - top_safe_margin - bottom_safe_margin;
```

---

## Phase C: Fix Map Rendering

### Task C.1: Relocate Map Rendering

**If map is hidden, likely causes:**
1. **Positioned off-screen:** Area fixed_pos is wrong
2. **Clipped by panel:** CentralPanel has size constraints
3. **Z-order behind:** Another panel rendered on top

**Fix strategy:**
- Ensure `render_radar()` is called *inside* the Map tab's content area
- Use `ui.available_rect()` to get the correct bounds for drawing
- Don't use `egui::Area` with fixed positioning; use the current panel's UI context

**Updated pattern:**
```rust
BottomTab::Map => {
    ui.vertical(|ui| {
        match self.map_sub_tab {
            MapSubTab::Zones => {
                // Map should render within this panel context
                render_radar(ui, &self.state.world_map);
                // NOT: egui::Area::new(...).fixed_pos(...).show(...)
            }
        }
    });
}
```

---

## Deliverables

**File:** `OperatorGame_UI_Layout_Audit_Report.md`

**Sections:**
1. Screen Real Estate Inventory (Moto G portrait dimensions)
2. Current Layout Breakdown (top/central/bottom panels, sidebar, content)
3. Map Rendering Issue Analysis (where is it, why is it hidden?)
4. Card Grid Sizing Analysis (cards per row, responsiveness)
5. Recommended Responsive Breakpoints (for different screen widths)
6. Smart Sizing Strategy (calculated card width, responsive font, etc.)
7. Panel Spacing Audit (heights, widths, margins, safe area application)

**And/Or: Code Changes (if simple fixes identified)**

---

## Acceptance Criteria

✓ Screen real estate mapped (available width × height per breakpoint)  
✓ Current panel layout documented (top, central, bottom, sidebar)  
✓ Map rendering issue identified (off-screen, clipped, z-order, etc.)  
✓ Card grid sizing analyzed (current vs. optimal cards per row)  
✓ Responsive breakpoints defined (ultra-narrow, narrow, standard, wide, desktop)  
✓ Smart sizing formulas provided (card width calculation, responsive logic)  
✓ Panel spacing optimized (heights, widths, margins reconciled)  
✓ Map rendering fix proposed (relocate, reposition, or restructure)  
✓ All issues documented with recommendations  

---

## Notes for Agent

- This is **audit + light fix**, not a full refactor
- Focus on identifying problems, not solving everything at once
- If fixes require changes to multiple files, flag them but don't implement yet—we'll plan a "Layout Fix Phase" after this audit
- Test on both desktop (large window) and mobile (narrow window simulation) if possible
- Screenshot current state vs. proposed state if possible

---

## Next Step

Once audit is complete, we'll either:
1. **Apply quick fixes** (if issues are simple: reposition map, adjust card widths, etc.)
2. **Plan a dedicated "Layout Refinement Phase"** (if major restructuring is needed)

Report back with findings.
