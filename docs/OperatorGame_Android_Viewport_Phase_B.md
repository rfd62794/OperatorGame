# OperatorGame --- Android Viewport Fix (Phase B)

**Directive Type:** IMPLEMENTATION  
**Scope:** SafeArea wiring + BottomTab rendering  
**Test Floor:** 93 → 105 passing (12 new UI layout tests)  
**Acceptance:** Moto G 2025 portrait mode: status bar & soft buttons no longer clip top/bottom UI  

---

## Goal

Wire the existing `SafeArea` and `LayoutCalculator` infrastructure into `OperatorApp::update()` and render the 4-tab bottom navigation bar at safe screen edges.

---

## Phase B.1: SafeArea Integration into UI Loop

### Task 1.1: Call `read_window_insets()` in OperatorApp::update()

**Location:** `src/ui/mod.rs` → `impl OperatorApp { fn update(&mut self, ctx: &egui::Context) { ... } }`

**Change:**
```rust
// At the START of OperatorApp::update(), before any panel rendering:
let safe_area = crate::platform::read_window_insets();
```

**Rationale:** Capture safe area once per frame. Moto G returns `top: 48, bottom: 56, left: 0, right: 0`.

---

### Task 1.2: Inject SafeArea margins into TopBottomPanel/CentralPanel

**Location:** Same `update()` function. Modify the panel definitions:

**Current Code:**
```rust
egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
    // OPERATOR: COMMAND DECK, etc.
});

egui::CentralPanel::default().show(ctx, |ui| {
    // Main content
});

egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
    // Launch bar (Deploy, etc.)
});
```

**Updated Code:**
```rust
// TOP PANEL — Reserve space for status bar inset
let top_panel_height = 60.0; // Operator header height
egui::TopBottomPanel::top("top_bar")
    .frame(
        egui::Frame::none()
            .inner_margin(egui::Margin {
                left: safe_area.left,
                right: safe_area.right,
                top: safe_area.top,  // ← Push content below status bar
                bottom: 0.0,
            })
    )
    .show(ctx, |ui| {
        // OPERATOR: COMMAND DECK, etc.
    });

// CENTRAL PANEL — Already naturally centered between top and bottom
egui::CentralPanel::default()
    .frame(
        egui::Frame::none()
            .inner_margin(egui::Margin {
                left: safe_area.left,
                right: safe_area.right,
                top: 0.0,
                bottom: 0.0,
            })
    )
    .show(ctx, |ui| {
        // Main content (roster, missions, etc.)
    });

// BOTTOM PANEL — Reserve space for soft button inset
egui::TopBottomPanel::bottom("bottom_bar")
    .frame(
        egui::Frame::none()
            .inner_margin(egui::Margin {
                left: safe_area.left,
                right: safe_area.right,
                top: 0.0,
                bottom: safe_area.bottom,  // ← Push content above soft buttons
            })
    )
    .show(ctx, |ui| {
        // Launch bar (Deploy, etc.)
    });
```

**Rationale:** egui's `Frame::inner_margin()` insets the *content* from the panel edges. This pushes text/buttons away from screen edges, preventing clipping by status/nav bars.

---

## Phase B.2: BottomTab Rendering

### Task 2.1: Define BottomTab Layout Rect

**Location:** `src/platform.rs` → Add to `LayoutCalculator`:

```rust
impl LayoutCalculator {
    pub fn bottom_tab_rect(&self, safe_area: &SafeArea) -> egui::Rect {
        let tab_height = 48.0; // Standard Android bottom nav height
        let screen_height = self.screen_size.y;
        
        // Position tabs above soft button area
        let bottom_y = screen_height - safe_area.bottom - tab_height;
        let top_y = bottom_y;
        
        egui::Rect::from_min_max(
            egui::pos2(safe_area.left, top_y),
            egui::pos2(self.screen_size.x - safe_area.right, screen_height - safe_area.bottom),
        )
    }
}
```

**Rationale:** Calculate exact pixel position for the tab bar *within* the safe area.

---

### Task 2.2: Render BottomTab in UI loop

**Location:** `src/ui/mod.rs` → In `OperatorApp::update()`, **after** the bottom_bar panel:

```rust
// Calculate tab bar position
let layout = crate::platform::LayoutCalculator::new(
    egui::vec2(ctx.screen_rect().width(), ctx.screen_rect().height()),
    &safe_area,
);
let tab_rect = layout.bottom_tab_rect(&safe_area);

// Render tab bar in custom area (not a standard panel)
egui::Area::new("bottom_tabs")
    .fixed_pos(tab_rect.min)
    .show(ctx, |ui| {
        ui.set_min_size(egui::vec2(
            tab_rect.width(),
            tab_rect.height(),
        ));
        
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            
            let tab_width = tab_rect.width() / 4.0; // 4 tabs
            
            for (idx, tab_label) in &[
                ("Roster", BottomTab::Roster),
                ("Missions", BottomTab::Missions),
                ("Map", BottomTab::Map),
                ("Logs", BottomTab::Logs),
            ].iter().enumerate() {
                let is_active = self.active_tab == *tab_label;
                
                let button = egui::Button::new(tab_label.0)
                    .fill(if is_active { 
                        egui::Color32::from_rgb(100, 200, 100) 
                    } else { 
                        egui::Color32::from_rgb(60, 60, 60) 
                    })
                    .min_size(egui::vec2(tab_width, tab_rect.height()));
                
                if ui.add(button).clicked() {
                    self.active_tab = *tab_label;
                }
            }
        });
    });
```

**Rationale:** Use `egui::Area` to position the tab bar at safe coordinates, separate from the standard panel system.

---

## Phase B.3: State & Navigation

### Task 3.1: Add active_tab to OperatorApp

**Location:** `src/ui/mod.rs` → `pub struct OperatorApp`:

```rust
pub struct OperatorApp {
    // ... existing fields
    active_tab: BottomTab,  // ← Add this
}

impl OperatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // ... existing init
            active_tab: BottomTab::Roster,  // ← Default to Roster
        }
    }
}
```

**Rationale:** Track which tab is selected so buttons visually respond.

---

### Task 3.2: Wire tab selection to content display

**Location:** `src/ui/mod.rs` → In `CentralPanel` render block:

```rust
egui::CentralPanel::default()
    .frame( /* ... margins ... */ )
    .show(ctx, |ui| {
        match self.active_tab {
            BottomTab::Roster => {
                ui.label("Roster Content");
                // Render roster (existing code)
            }
            BottomTab::Missions => {
                ui.label("Missions Content");
                // Render missions
            }
            BottomTab::Map => {
                ui.label("Map Content");
                // Render map
            }
            BottomTab::Logs => {
                ui.label("Logs Content");
                // Render logs
            }
        }
    });
```

**Rationale:** Central panel content switches based on active tab.

---

## Test Floor

**Before:** 93 passing, 0 failing  
**Target:** 105 passing, 0 failing (12 new tests)

### New Tests Required:

1. **test_safe_area_margins_applied** — Verify `Frame::inner_margin()` correctly applies safe_area insets
2. **test_bottom_tab_rect_calculation** — Verify `LayoutCalculator::bottom_tab_rect()` positions tabs above soft buttons
3. **test_tab_bar_rendering** — Verify `egui::Area` renders 4 tabs at correct positions
4. **test_tab_selection_state** — Verify clicking tab changes `self.active_tab`
5. **test_content_switches_on_tab** — Verify CentralPanel content changes with active_tab
6. **test_safe_area_zero_insets_desktop** — Verify zero insets on desktop don't break layout
7. **test_moto_g_insets_hardcoded** — Verify hardcoded `top: 48, bottom: 56` is applied on Android
8. **test_tab_bar_respects_left_right_insets** — Verify tab bar doesn't clip on devices with side notches
9. **test_responsive_layout_with_safe_area** — Verify Compact/Standard layouts still work with insets
10. **test_top_panel_margin_below_status_bar** — Verify top panel content is below status bar
11. **test_bottom_panel_margin_above_soft_buttons** — Verify bottom panel content is above soft buttons
12. **test_landscape_rotation_recalculates_insets** — Verify safe_area updates on orientation change

---

## Acceptance Criteria

✓ `read_window_insets()` called once per frame in `OperatorApp::update()`  
✓ Top/Bottom/Central panels use `Frame::inner_margin()` with safe_area values  
✓ BottomTab renders as 4-button horizontal bar at safe screen bottom  
✓ Tab selection changes content in CentralPanel  
✓ Moto G 2025: Status bar no longer clips "OPERATOR: COMMAND DECK" text  
✓ Moto G 2025: Soft buttons no longer clip bottom panel content  
✓ All 12 new tests pass  
✓ Test floor: 105 / 105 (zero regressions)  
✓ Code compiles to aarch64 + armv7 without warnings  

---

## Notes for Agent

- **Do not implement JNI** yet. Keep using `SafeArea::android_default()` hardcoded values (they're correct for Moto G).
- **egui::Area is the right tool** for the tab bar because standard panels can't be sized/positioned manually.
- **Margin vs. Padding:** `Frame::inner_margin()` is *content* margin. We want this, not outer_margin.
- **Test on device:** After build, run on Moto G 2025 in portrait. The header and launch bar should no longer be clipped.
- **Landscape deferral:** Rotation handling is deferred to Sprint 4 (Task 3.2 test covers it, but implementation can wait).

---

## Deliverables

1. **Updated `src/ui/mod.rs`** — SafeArea integration + BottomTab rendering + tab state
2. **Updated `src/platform.rs`** — Add `LayoutCalculator::bottom_tab_rect()`
3. **New test suite** — 12 tests in `tests/ui_layout.rs` (new file)
4. **APK build** — Test on Moto G 2025 (visual validation)

---

## Completion Checklist

- [ ] SafeArea call added to `OperatorApp::update()`
- [ ] Frame margins applied to all 3 panels (top/central/bottom)
- [ ] BottomTab layout rect calculated in LayoutCalculator
- [ ] BottomTab renders as 4-button Area
- [ ] Tab selection wires to content display
- [ ] All 12 tests passing
- [ ] Test floor: 105 / 105
- [ ] APK builds and runs on Moto G
- [ ] Status bar / soft button clipping visually resolved
