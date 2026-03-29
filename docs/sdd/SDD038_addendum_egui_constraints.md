# SDD-038 Addendum: Known egui Constraints (§10)

**Added:** March 2026  
**Reason:** Documented during SDD-038 implementation after repeated layout failures  
**Authority:** These constraints apply to egui 0.27.2 as used in this project  

---

## §10 — Known egui Constraints and Safe Patterns

This section documents layout patterns that were attempted and failed during SDD-038 implementation, along with the safe alternatives. Future agents working in `src/ui/` must read this section before writing any layout code.

---

### 10.1 — Mutable Variable Capture Across Nested Closures

**The problem:**
egui widgets accept closures. When you nest closures (e.g., a Frame closure containing a columns closure), Rust's borrow checker allows compilation but mutations of outer variables from inner closures may not propagate correctly at runtime.

**Specifically broken:**
```rust
let mut stage_clicked = false;

egui::Frame::none().show(ui, |ui| {          // closure 1
    ui.columns(2, |cols| {                    // closure 2 — nested
        if cols[1].button("STAGE").clicked() {
            stage_clicked = true;            // mutation may not propagate
        }
    });
});

// stage_clicked may still be false here
```

**Safe pattern:**
Keep mutable result variables in the same closure scope where they're mutated. Do not nest more than one closure level when capturing mutable variables.

```rust
let mut stage_clicked = false;

egui::Frame::none().show(ui, |ui| {
    // Row 1: name
    ui.horizontal(|ui| {
        ui.label(&genome.name);
    });
    // Row 1b: buttons — separate closure, same level
    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
        if ui.small_button("STAGE").clicked() {
            stage_clicked = true;   // same closure level as Frame — works
        }
    });
});
```

---

### 10.2 — `ui.set_width()` vs `ui.set_max_width()`

**The problem:**
`ui.set_width(380.0)` forces egui to allocate exactly 380dp regardless of actual available space. If the parent container provides less than 380dp, content overflows off-screen silently.

**Rule:**
- Use `ui.set_max_width(value)` — caps at value but uses less if needed
- Never use `ui.set_width(hardcoded_value)` in production UI code
- If you need exact sizing, use `ui.available_width()` as the source

**Correct:**
```rust
ui.set_max_width(380.0);  // caps at 380, uses less if needed
```

**Incorrect:**
```rust
ui.set_width(380.0);  // forces 380 even if only 240dp available — causes overflow
```

---

### 10.3 — `ui.columns(2)` Width Calculation

**The problem:**
`ui.columns(2)` splits available width equally. But "available width" inside a Frame with `inner_margin(8.0)` is `frame_width - 16dp` (8dp left + 8dp right). If `frame_width` was forced with `set_width(380.0)`, columns get `(380 - 16) / 2 = 182dp` each — but if the screen only has 240dp available, the second column renders off-screen.

**Rule:**
Do not combine `ui.set_width(hardcoded)` with `ui.columns(n)`. Either use `set_max_width` or calculate column widths from `ui.available_width()`.

---

### 10.4 — `add_sized` Centering Behavior

**The problem:**
`ui.add_sized([width, height], widget)` centers the widget within the allocated space by default. For a `Label`, this produces centered text, not left-aligned text.

**If you need left-aligned text with a fixed width:**
```rust
ui.horizontal(|ui| {
    ui.set_width(name_width);
    ui.label(text);  // left-aligned by default in horizontal layout
});
```

Not:
```rust
ui.add_sized([name_width, 18.0], egui::Label::new(text));  // centers text
```

---

### 10.5 — Right-to-Left Layout Inside Horizontal

**The problem:**
When `with_layout(right_to_left)` is placed after labels in a `ui.horizontal` block, the labels have already consumed available space. The right-to-left block gets zero or negative remaining width and renders nothing.

**Safe pattern:**
Use two separate UI calls rather than nesting right-to-left inside horizontal:
```rust
// Name row
ui.horizontal(|ui| { ui.label(name); });

// Buttons row — separate, right aligned
ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
    if ui.small_button("STAGE").clicked() { stage_clicked = true; }
});
```

This produces two rows (name above, buttons below-right) rather than one row. This is the accepted tradeoff per ADR-039.

---

### 10.6 — Frame Inner Margin Accounting

**The problem:**
A Frame with `inner_margin(8.0)` reduces available width by 16dp (8dp each side). Code that doesn't account for this will overflow.

**The math for this project:**
- Content area: 396dp
- Side gutter: 8dp each side = 16dp
- Card width: 380dp
- Frame inner margin: 8dp each side = 16dp
- Actual usable width inside card frame: **364dp**

Any layout inside a card frame must work within 364dp, not 380dp.

---

### 10.7 — Emoji Rendering on Android

**The problem:**
egui uses its own font rendering. Many emoji characters render as boxes (□) or garbled text on Android because the embedded font doesn't include emoji glyphs.

**Rule:**
Do not use emoji in button labels or UI text. Use ASCII alternatives:
- ✓ → "OK" or "DONE"  
- ➕ → "+"  
- ▶ → ">"  
- ◀ → "<"  
- 🎩 → "HAT:"  

Emoji may work in non-interactive labels on desktop but will break on Android.

---

### 10.8 — `apply_interaction_scale` Side Effects

**The problem:**
`apply_interaction_scale()` in `platform.rs` sets minimum interact sizes to 44dp for the Compact layout. This causes every button, label, and selectable to occupy at least 44dp of vertical space, making cards feel "giant" on mobile.

**Current status:** Commented out in `mod.rs` as of SDD-038 implementation. Do not re-enable without designer approval.

**Location:** `src/platform.rs` — function exists but is not called.

---

*RFD IT Services Ltd. | OperatorGame | SDD-038 §10 Addendum | March 2026*
