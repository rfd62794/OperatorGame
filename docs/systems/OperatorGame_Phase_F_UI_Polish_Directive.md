# OperatorGame — Phase F: UI Polish Directive
## Sub-tab Sidebar Styling + Bottom Tab Button Restructuring

**Phase:** F (UI Implementation — Polish Pass)  
**Goal:** Improve sub-tab sidebar visual hierarchy and restructure bottom tab buttons for mobile-standard icon-over-label layout  
**Scope:** Visual styling + small presentation restructuring (no features, no behavior changes)  
**Output:** Styled sidebar + restructured bottom tabs, all tests passing  

---

## Current State (Problems Identified)

**Sub-tab Sidebar Issues:**
- `selectable_label()` is stock egui with no custom styling
- Tiny touch targets (hard to tap on mobile)
- No visual hierarchy (all labels look the same)
- Section headers (`ui.label("Roster")`) are unpolished
- Separator line (`"─────────────"`) is weak and unrefined
- Long labels ("Mission History", "Quest Board", "Culture History") overflow the 80–100dp column width

**Bottom Tab Button Issues:**
- Active tab shows solid green fill (100, 200, 100) — jarring color, no refinement
- Emoji + label rendered flat on one line (no vertical structure)
- No icon-over-label stacking (not a proper mobile nav bar pattern)
- No clear active/inactive state differentiation (beyond green fill)

---

## Design Direction (From Stitch Design System)

**Color Palette:**
- **Base Background:** `#0e0e13` (void/dark)
- **Surface Containers:** `#131318` (panels), `#25252c` (headers)
- **Primary (Active/Success):** `#69fea5` (vibrant green, for active states)
- **Secondary (Warning/Heat):** `#ff8844` (orange, for secondary/warning)
- **Tertiary (Utility):** `#6a9cff` (blue, for secondary systems)
- **Text:** `#f8f5fd` (high contrast white)

**Typography:**
- Headers: Space Grotesk, all-caps, bold (14-16pt)
- Body/Labels: Inter or monospace (12pt)
- Stats: Monospace (10pt)

**Principles:**
- High-density, tactical feel
- Sharp corners (no soft rounded corners)
- Tonal layering (no 1px borders)
- Clear active state feedback
- Touch targets: 44dp minimum (mobile-friendly)

---

## Part A: Sub-Tab Sidebar Styling

### Task A.1: Improve `selectable_label()` Buttons

**Current code pattern (in `src/ui/mod.rs` or sidebar rendering):**
```rust
if ui.selectable_label(self.roster_sub_tab == RosterSubTab::Collection, "Collection") {
    self.roster_sub_tab = RosterSubTab::Collection;
}
```

**Problems:**
- No custom styling (default egui appearance)
- Tiny default size
- No color differentiation
- No hover/active feedback

**Fix:**
Replace stock `selectable_label()` with **custom styled button** that:

1. **Sets button styling before rendering:**
   ```rust
   // Apply custom visuals for sub-tab buttons
   let mut button = egui::Button::new(
       egui::RichText::new("Collection")
           .size(12.0)
           .color(if self.roster_sub_tab == RosterSubTab::Collection {
               egui::Color32::from_rgb(105, 254, 165) // Primary green (#69fea5)
           } else {
               egui::Color32::from_rgb(248, 245, 253) // Text color (#f8f5fd)
           })
   )
   .fill(if self.roster_sub_tab == RosterSubTab::Collection {
       egui::Color32::from_rgb(19, 19, 24) // Surface container low (#131318)
   } else {
       egui::Color32::TRANSPARENT
   })
   .min_size(egui::vec2(70.0, 40.0)) // 44dp minimum for mobile touch
   .stroke(egui::Stroke::NONE)
   .frame(true)
   .rounding(egui::Rounding::ZERO); // Sharp corners per design system
   
   if ui.add(button).clicked() {
       self.roster_sub_tab = RosterSubTab::Collection;
   }
   ```

2. **Active state indicators:**
   - **Active:** Full background fill (#131318), text color primary green (#69fea5)
   - **Inactive:** Transparent background, text color white (#f8f5fd)
   - **Hover:** Subtle background color shift (slightly lighter surface)

3. **Spacing & layout:**
   - Padding: 8dp (0.5rem) inside each button
   - Gap between buttons: 4dp
   - Minimum button height: 44dp (mobile-friendly touch target)

**Apply to all sub-tabs:**
- Roster: Collection, Breeding Tank, (future Detail)
- Missions: Active, Quest Board, (future History)
- Map: Zones, (future Resources, Anomalies)
- Logs: Mission History, Culture History

---

### Task A.2: Refine Section Headers

**Current code:**
```rust
ui.label("Roster"); // Plain, unpolished
ui.label("─────────────"); // Weak separator
```

**Fix:**
1. **Section header styling:**
   ```rust
   ui.label(
       egui::RichText::new("ROSTER")
           .size(14.0)
           .color(egui::Color32::from_rgb(105, 254, 165)) // Primary green (#69fea5)
           .weight(egui::FontWeight::Bold)
   );
   ```
   - All-caps (per design system)
   - Larger, bolder font (14pt, bold)
   - Primary green color (visual emphasis)

2. **Separator refinement:**
   ```rust
   ui.separator(); // Use egui's built-in separator instead of text
   // Or, if you want custom styling:
   let sep_rect = ui.available_rect_before_wrap();
   let painter = ui.painter();
   painter.hline(
       sep_rect.x_range(),
       sep_rect.top,
       egui::Stroke::new(1.0, egui::Color32::from_rgb(37, 37, 44)) // Surface container highest (#25252c)
   );
   ui.add_space(4.0);
   ```
   - Use a proper horizontal line (not text characters)
   - Color: Surface container highest (#25252c)
   - Spacing: 4dp above and below

3. **Spacing around section:**
   - 12dp above section header
   - 8dp below separator
   - 12dp below last sub-tab in section

---

### Task A.3: Handle Label Overflow

**Current problem:**
Long labels like "Mission History" and "Quest Board" overflow the 80–100dp sidebar width.

**Solutions (pick one):**

**Option 1: Truncate with ellipsis (recommended for mobile)**
```rust
let max_width = 90.0; // Sidebar width minus padding
ui.label(
    egui::RichText::new("Mission History")
        .size(11.0) // Slightly smaller to fit
        .color(text_color)
);
```
- Reduce font size slightly (11pt instead of 12pt)
- Use monospace or condensed font if available

**Option 2: Abbreviate labels**
```rust
// Instead of "Mission History", use "Missions"
// Instead of "Quest Board", use "Quests"
// Instead of "Culture History", use "Cultures"
```

**Option 3: Wrap text (less recommended for nav)**
- Wraps long labels to two lines
- Takes up more vertical space
- Less clean for a sidebar

**Recommendation:** Use **Option 1** (slight font reduction) or **Option 2** (abbreviations). Test on 80dp width and confirm legibility.

---

## Part B: Bottom Tab Button Restructuring

### Task B.1: Restructure Tab Button Layout (Icon-Over-Label)

**Current code pattern (in `src/ui/mod.rs`, bottom panel tab rendering):**
```rust
if ui.button("🗂️ Roster").clicked() {
    self.active_tab = BottomTab::Roster;
}
```

**Problem:**
- Emoji + label on one line (flat)
- No visual hierarchy between icon and text
- Doesn't match mobile design standard (icon above, label below)

**Fix:**
Replace flat button with **vertical stacked layout:**

```rust
let tab_width = 80.0; // Width per tab (adjust based on available space)
let tab_height = 56.0; // Standard mobile tab bar height

let is_active = self.active_tab == BottomTab::Roster;

// Create a vertical button container
let button_response = ui.allocate_response(
    egui::vec2(tab_width, tab_height),
    egui::Sense::click()
);

// Paint the background (active or inactive)
let bg_color = if is_active {
    egui::Color32::from_rgb(19, 19, 24) // Surface container low (#131318)
} else {
    egui::Color32::TRANSPARENT
};
ui.painter().rect_filled(button_response.rect, egui::Rounding::ZERO, bg_color);

// Paint the top accent strip (active tabs only)
if is_active {
    let accent_width = 3.0;
    let accent_rect = egui::Rect {
        min: button_response.rect.min,
        max: egui::pos2(button_response.rect.min.x + accent_width, button_response.rect.max.y),
    };
    ui.painter().rect_filled(
        accent_rect,
        egui::Rounding::ZERO,
        egui::Color32::from_rgb(105, 254, 165) // Primary green (#69fea5)
    );
}

// Render stacked content: icon above, label below
ui.vertical(|ui| {
    ui.add_space(4.0); // Top padding
    
    // Icon (emoji, 18pt)
    ui.label(
        egui::RichText::new("🗂️")
            .size(18.0)
            .color(egui::Color32::WHITE)
    );
    
    ui.add_space(2.0); // Gap between icon and label
    
    // Label (text, 10pt)
    ui.label(
        egui::RichText::new("Roster")
            .size(10.0)
            .color(if is_active {
                egui::Color32::from_rgb(105, 254, 165) // Primary green (#69fea5)
            } else {
                egui::Color32::from_rgb(248, 245, 253) // Text color (#f8f5fd)
            })
    );
    
    ui.add_space(4.0); // Bottom padding
});

// Handle click
if button_response.clicked() {
    self.active_tab = BottomTab::Roster;
}
```

**Key changes:**
1. **Vertical stacking:** Icon (18pt) above, label (10pt) below, with 2dp gap
2. **Active state:** Left accent strip (3dp wide, primary green) + background fill
3. **Inactive state:** Transparent background, white text
4. **Touch target:** 80dp × 56dp (standard mobile nav bar size)
5. **Alignment:** Center icon and label horizontally

---

### Task B.2: Apply to All Bottom Tabs

**Tabs to style:**
1. **Roster** (🗂️)
2. **Missions** (🎯 or ⚔️)
3. **Map** (🗺️)
4. **Logs** (📋)

**Each tab follows the same pattern:**
- Icon (emoji, 18pt, white)
- Label (10pt, green if active, white if inactive)
- Left accent strip (3dp, primary green) if active
- Background fill (#131318) if active, transparent if inactive

---

### Task B.3: Color & Feedback Refinement

**Active tab styling:**
- Background: `#131318` (surface container low)
- Left accent strip: `#69fea5` (primary green, 3dp wide, full height)
- Label text: `#69fea5` (primary green)
- Icon: `#ffffff` (white)

**Inactive tab styling:**
- Background: Transparent
- Icon: `#ffffff` (white)
- Label text: `#f8f5fd` (light white)

**Hover state (optional, for desktop):**
- Subtle background shift: slightly darker gray
- No accent strip

---

## Part C: Implementation Details

### File Structure
- **Main changes:** `src/ui/mod.rs` (sidebar rendering + bottom tab rendering)
- **Possibly:** Extract sidebar button rendering to a helper function if it becomes repetitive
- **No changes needed:** Geometry, platform, garden, or core logic files

### Code Patterns to Update
1. Replace all `ui.selectable_label()` calls in sidebar with custom styled buttons
2. Replace flat emoji+label buttons at bottom with vertical stacked layout
3. Apply consistent color tokens throughout (use hex values or define constants)

### Testing Checklist
- [ ] Sub-tab buttons render with correct size (44dp+ touch target)
- [ ] Active sub-tab shows clear visual feedback (background fill + color change)
- [ ] Section headers are visible and properly spaced
- [ ] Long labels (Mission History, Culture History) fit within sidebar width (80-100dp)
- [ ] Bottom tab buttons stack icon over label (not side-by-side)
- [ ] Active bottom tab shows left accent strip + background
- [ ] All text is legible (high contrast, readable font sizes)
- [ ] Spacing is consistent (8dp padding, 4dp gaps)
- [ ] No jarring color transitions
- [ ] All tests pass (`cargo test`)

---

## Acceptance Criteria

✓ Sub-tab sidebar buttons styled with custom appearance (not stock `selectable_label`)  
✓ Sub-tab buttons have 44dp+ touch targets (mobile-friendly)  
✓ Active sub-tab shows clear color differentiation (primary green text + background fill)  
✓ Section headers are all-caps, bold, primary green (#69fea5)  
✓ Separators are refined (proper lines, not text characters)  
✓ Long labels fit within sidebar width without overflow  
✓ Bottom tab buttons restructured to icon-over-label stacking  
✓ Bottom tab buttons show left accent strip when active (3dp, primary green)  
✓ Active bottom tab shows background fill (#131318) + primary green text/accent  
✓ Inactive bottom tab shows transparent background + white text  
✓ All colors match design system (dark industrial + vibrant accents)  
✓ Sharp corners throughout (no soft rounded corners)  
✓ Spacing consistent (8dp padding, 4dp gaps)  
✓ All tests passing  
✓ No functionality changes, only visual/presentation restructuring  

---

## Notes for Agent

- This is **visual polish only** — no features, no behavior changes
- Small structural change (button layout) is acceptable within this scope
- Focus on mobile-first design (44dp touch targets, stacked nav bars)
- Use the Stitch design system colors as reference
- Test on both desktop (wide window) and mobile (narrow) to ensure responsive appearance
- If you need to define color constants, consider adding them to a theme module (future cleanup)
- Report any blockers or design conflicts immediately

---

## Success Looks Like

When complete, the UI will:
1. **Sidebar:** Clear visual hierarchy, easy-to-tap buttons, properly spaced labels
2. **Bottom tabs:** Look like a proper mobile nav bar (icon stacked above label, clear active state)
3. **Overall:** Professional, polished, dark industrial + vibrant accent colors
4. **Feel:** Tactical and dense (appropriate for a stats-heavy game)

---

## Next Phases (Post-F)

- F.2: Roster Tab Breeding UI (incubator timer, hatch notifications)
- F.3: Missions Tab (quest board, dispatch, AAR display)
- F.4: Map & Logs Tabs (zone rings, culture timeline)
- Phase E: Core Loop Balance (breeding, dispatch, leveling mechanics)

---

## Deliverable

**File:** `src/ui/mod.rs`  
**Changes:**
- Sidebar sub-tab button rendering (replace `selectable_label` with styled buttons)
- Section header styling (all-caps, bold, primary green)
- Separator refinement (proper lines)
- Bottom tab button restructuring (icon-over-label stacking)
- Color application throughout

**Tests:** All passing, zero regressions

---
