# OperatorGame Mobile Emulation: Quick Reference

## TL;DR

```powershell
# Mobile (Android-like)
.\run_mobile.ps1

# Desktop (normal)
.\run_desktop.ps1
```

---

## One-Command Testing Workflows

### Rapid Mobile Iteration
```powershell
# Edit UI → Save → Run mobile
.\run_mobile.ps1
# [App launches in 400×800, safe areas active]
# [Play for 30 sec, close]
# [Repeat]
```
**Feedback loop:** ~5 seconds

### Baseline Comparison
```powershell
# Test both modes back-to-back
.\run_desktop.ps1
# [Play 30 sec]
# [Close]
.\run_mobile.ps1
# [Play 30 sec]
# [Verify same behavior]
```
**Validates:** No state corruption, consistent gameplay

### Cross-Save Testing
```powershell
# Desktop mode
.\run_desktop.ps1
# [Play, save game]
# [Close]

# Switch to mobile (same save.json)
.\run_mobile.ps1
# [Verify save loaded, layout adapted]
# [No crashes]
```
**Validates:** Save format is platform-agnostic

---

## What Each Script Does

### `run_mobile.ps1`
```powershell
$env:OPERATOR_MOBILE_EMU = "1"
cargo run --release
```

**Results:**
- Window: 400×800 (portrait)
- Layout: Bottom Tabs (sidebar hidden)
- Safe Areas: 48px top, 56px bottom
- DPI: 2.0 (Moto G 2025 density)

### `run_desktop.ps1`
```powershell
# No env var set
cargo run --release
```

**Results:**
- Window: 1200×800 (landscape)
- Layout: Sidebar + content
- Safe Areas: None
- DPI: 1.0 (normal)

---

## Environment Variable Reference

| Var | Value | Effect |
|-----|-------|--------|
| `OPERATOR_MOBILE_EMU` | `1` | Enable mobile simulation |
| `OPERATOR_MOBILE_LANDSCAPE` | `1` | 800×400 portrait (optional, future) |

**How to use manually:**
```powershell
$env:OPERATOR_MOBILE_EMU = "1"
cargo run --release

# Or one-liner:
$env:OPERATOR_MOBILE_EMU="1"; cargo run --release
```

---

## Tweaking Mobile Behavior

### Adjust Safe Area Insets
**File:** `src/platform.rs`
```rust
pub fn android_default() -> Self {
    Self {
        top: 48.0,      // ← Change this
        bottom: 56.0,   // ← Or this
    }
}
```

### Adjust Touch Button Size
**File:** `src/ui/mod.rs`
```rust
const MOBILE_BUTTON_SIZE: f32 = 110.0;  // ← Change to 90 or 130
```

### Adjust Window Size
**File:** `src/main.rs`
```rust
let (width, height) = if mobile_emu {
    (400.0, 800.0)  // ← Change portrait ratio
} else {
    (1200.0, 800.0)
};
```

### Adjust DPI Scaling
**File:** `src/ui/mod.rs`
```rust
ui.set_pixels_per_point(2.0);  // ← Try 1.5, 2.5, etc.
```

---

## Common Issues & Fixes

| Issue | Fix |
|-------|-----|
| Window not 400×800 | Check `egui::ViewportBuilder` in main.rs |
| Sidebar shows in mobile | Verify `is_mobile_emu()` in layout logic |
| Content bleeds into safe areas | Wrap UI in bounds-respecting container |
| Buttons too small/big | Adjust `MOBILE_BUTTON_SIZE` constant |
| Save doesn't load between modes | Ensure save.json path is same for both |

**Full troubleshooting:** See `MOBILE_EMU_VERIFICATION.md`

---

## Performance Baseline

After launching `.\run_mobile.ps1`, check:

```
Idle:             <5% CPU
Gameplay:         <30% CPU
Menu interactions: <20% CPU
```

If exceeded, profile with:
```powershell
cargo build --release
# Then use Windows Task Manager or profiler
```

---

## Testing Checklist (Before Commit)

```powershell
# 1. Run mobile
.\run_mobile.ps1
# [ ] Window is 400×800
# [ ] Sidebar hidden, bottom tabs visible
# [ ] Play 2 minutes without issues
# [ ] Close normally

# 2. Run desktop
.\run_desktop.ps1
# [ ] Window is 1200×800
# [ ] Sidebar visible, content normal
# [ ] Play 2 minutes without issues
# [ ] Close normally

# 3. Verify code
cargo check --lib
# [ ] No warnings or errors

# 4. Commit
git add src/
git commit -m "Feature: [description]"
```

---

## Workflow Integration

### For Mobile UI Work
```powershell
# In your IDE:
# 1. Edit src/ui/mod.rs
# 2. Ctrl+S to save
# 3. Alt+Tab to PowerShell
# 4. .\run_mobile.ps1
# 5. See changes instantly (400×800 view)
# 6. Close, repeat
```

### For Desktop Features
```powershell
# In your IDE:
# 1. Edit src/main.rs or game logic
# 2. Ctrl+S to save
# 3. .\run_desktop.ps1
# 4. Test full desktop experience
# 5. Close, repeat
```

### For Cross-Platform Validation
```powershell
# Before shipping UI changes:
# 1. .\run_mobile.ps1 → Play 2 min
# 2. .\run_desktop.ps1 → Play 2 min
# 3. .\run_mobile.ps1 again → Load saved state
# 4. Verify: No crashes, state intact, both layouts work
```

---

## File Locations

```
OperatorGame/
├── src/
│   ├── platform.rs          ← Env var detection, safe areas
│   ├── ui/mod.rs            ← Layout forcing, touch scaling
│   ├── main.rs              ← Window size configuration
│   └── ...
├── run_mobile.ps1           ← Mobile launcher (one-click)
├── run_desktop.ps1          ← Desktop launcher (one-click)
├── Cargo.toml
└── save.json                ← Shared save file (both modes)
```

---

## Advanced: Manual Launch

If scripts don't work:

```powershell
# Mobile mode
$env:OPERATOR_MOBILE_EMU = "1"
cargo run --release

# Desktop mode
$env:OPERATOR_MOBILE_EMU = ""
cargo run --release
```

---

## Success Criteria

You're done when:

✅ `.\run_mobile.ps1` launches in ~3 seconds  
✅ UI behaves identically to actual Android build  
✅ Sidebar hidden, bottom tabs visible  
✅ Safe areas respected (no content clipping)  
✅ Buttons feel mobile-sized (44dp)  
✅ Save/load works between modes  
✅ Zero crashes during gameplay  

---

## Next: Optional Enhancements

1. **Save file isolation:** Separate `save_mobile.json` from `save.json`
2. **Landscape testing:** Add `run_mobile_landscape.ps1` for 800×400
3. **Visual indicator:** Add "📱 MOBILE EMU" label to title bar
4. **Automated tests:** Add `cargo test --lib platform` verification

See `MOBILE_EMU_VERIFICATION.md` for implementation details.

---

**Questions?** Refer back to `MOBILE_EMU_PLAN.md` for deep dives on any component.
