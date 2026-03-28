# Mobile Emulation Implementation: Verification & Optimization

## What Was Implemented

✅ **Platform Detection** (`src/platform.rs`)
- `is_mobile_emu()` reads `OPERATOR_MOBILE_EMU` env var
- `read_window_insets()` returns Android defaults (48dp top, 56dp bottom) when emulation is ON

✅ **Layout Forcing** (`src/ui/mod.rs`)
- `ResponsiveLayout` forced to `Compact` mode
- Sidebar hidden, Bottom Tabs always visible
- Safe areas carved out from content bounds

✅ **Window Config** (`src/main.rs`)
- 400×800 portrait window when emulating
- 1200×800 desktop when not

✅ **DPI Scaling**
- `pixels_per_point(2.0)` simulates Moto G 2025 display density

✅ **Run Scripts**
- `run_mobile.ps1` for one-command mobile launch
- `run_desktop.ps1` for desktop baseline

---

## Phase 1: Verification Checklist

Run through these before declaring "done":

### Desktop Mode (Baseline)
```powershell
.\run_desktop.ps1
```
- [ ] Window is 1200×800
- [ ] Sidebar is visible and functional
- [ ] No top/bottom safe area margins
- [ ] Buttons are normal size (~32dp)
- [ ] All UI elements render normally

### Mobile Emulation Mode
```powershell
.\run_mobile.ps1
```
- [ ] Window is 400×800 (portrait)
- [ ] Sidebar is completely hidden
- [ ] Bottom Tab bar is visible and all tabs are accessible
- [ ] 48px margin at top (status bar region)
- [ ] 56px margin at bottom (nav bar region)
- [ ] Content respects safe areas (no text clipping)
- [ ] Buttons visibly larger (~44dp, 110px at 2.0 DPI)
- [ ] Touch interactions work (all clicks register)
- [ ] Game state saves/loads identically to desktop mode

### Critical Test: Cross-Mode Compatibility
```powershell
# Test 1: Desktop → Mobile without rebuild
.\run_desktop.ps1
# [Play for 30 seconds, close]
.\run_mobile.ps1
# [Verify same game state loads, UI is mobile]
# [No crashes, no missing saves]
```
**Validates:** Save format is platform-agnostic, consistent gameplay.

---

## Phase 2: Optimization & Polish

### If Touch Scaling Feels Off
**Problem:** 44dp buttons might feel too large or too small.
**Solution:** Adjust the scaling factor in `src/ui/mod.rs`:
```rust
// Current: 44dp ≈ 110px at 2.0 DPI
const MOBILE_BUTTON_SIZE: f32 = 110.0;

// If it's too big:
const MOBILE_BUTTON_SIZE: f32 = 90.0;  // ~36dp

// If it's too small:
const MOBILE_BUTTON_SIZE: f32 = 130.0;  // ~52dp
```
**Test:** Run `.\run_mobile.ps1`, tap buttons with your mouse. Should feel responsive without accidental misclicks.

### If Safe Area Insets Feel Wrong
**Problem:** 48dp top / 56dp bottom might not match your layout.
**Solution:** Adjust in `src/platform.rs`:
```rust
impl SafeArea {
    pub fn android_default() -> Self {
        Self {
            top: 48.0,     // Adjust if needed
            bottom: 56.0,  // Adjust if needed
        }
    }
}
```
**Reference:** Moto G 2025 specs:
- Status bar: 24dp (but we use 48dp for visual buffer)
- Navigation bar: 48dp (we use 56dp for padding)

### If Performance Degrades in Mobile Mode
**Problem:** 2.0 DPI scaling + egui rendering might spike GPU usage.
**Solution:** Profile with `--release` build (already in scripts):
```powershell
# Ensure you're testing with release, not debug
cargo build --release
.\run_mobile.ps1
# Monitor: Should be <5% CPU at idle, <30% during interactions
```

---

## Phase 3: Advanced Features (Optional)

### Feature A: Save File Isolation
**Goal:** Keep mobile and desktop saves separate for testing.
**Implementation Sketch:**
```rust
// In src/main.rs or save module
fn get_save_path() -> PathBuf {
    let filename = if crate::platform::is_mobile_emu() {
        "save_mobile.json"
    } else {
        "save.json"
    };
    PathBuf::from(filename)
}
```
**Benefit:** Test mobile progression independently without corrupting desktop save.

### Feature B: Visual Indicator
**Goal:** Always know which mode you're in (no guessing).
**Implementation Sketch:**
```rust
// In your title bar or top-left corner
if crate::platform::is_mobile_emu() {
    ui.label("📱 MOBILE EMU");  // Or just add to window title
}
```

### Feature C: Rotation Testing (Landscape)
**Goal:** Test 800×400 landscape mode.
**Implementation:**
```powershell
# Add to run_mobile_landscape.ps1
$env:OPERATOR_MOBILE_EMU = "1"
$env:OPERATOR_MOBILE_LANDSCAPE = "1"
cargo run --release
```
Then in `src/main.rs`:
```rust
let (width, height) = if std::env::var("OPERATOR_MOBILE_LANDSCAPE").is_ok() {
    (800.0, 400.0)
} else {
    (400.0, 800.0)
};
```

---

## Phase 4: CI/CD Integration (Future)

Once you're confident, automate testing:
```yaml
# .github/workflows/mobile_emu_test.yml
name: Mobile Emulation Test
on: [push, pull_request]

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - run: cargo test --lib platform::tests
```

---

## Troubleshooting

| Issue | Cause | Fix |
|-------|-------|-----|
| "Window doesn't resize to 400×800" | NativeOptions not applied | Verify `egui::ViewportBuilder::with_inner_size` in `main.rs` |
| "Sidebar still shows in mobile" | Layout mode not forced | Check `ResponsiveLayout::from_width` logic |
| "Safe areas don't work" | UI bounds ignore insets | Wrap content in a container respecting `read_window_insets()` |
| "DPI scaling looks wrong" | pixels_per_point mismatch | Adjust `pixels_per_point(2.0)` in `src/ui/mod.rs` |

---

## Success Metrics

| Metric | Target | Check |
|--------|--------|-------|
| Mobile launch time | <3 sec after `.\run_mobile.ps1` | `time .\run_mobile.ps1` |
| Layout switch accuracy | 100% sidebar hidden | Visual inspection |
| Safe area respect | No text clipping | Bottom bar & status bar |
| Touch scaling | Zero misclicks | Tap 10 buttons |
| State preservation | Pass Desktop→Mobile | Save/Load cycle |
| Performance | No FPS drops | Monitor Task Manager |

---

## Next Steps

1. Run the **verification checklist** (Phase 1 above).
2. Note any failures and reference the **troubleshooting** section.
3. Decide on **optional features** (Phase 3) based on your testing cadence.
4. Integrate into your dev workflow:
   - When working on mobile UI: `.\run_mobile.ps1`
   - When working on desktop features: `.\run_desktop.ps1`

---

## Rollback Plan
If something breaks catastrophically:
```powershell
# Restore to desktop-only build
git checkout src/platform.rs src/ui/mod.rs src/main.rs
cargo clean
cargo build --release
```
*But you shouldn't need this—the changes are surgical and don't touch core game logic.*
