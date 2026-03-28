# Mobile Emulation: Quick-Reference Cheat Sheet

## 🚀 One-Click Launch

| Command | Action | Layout | Mode |
|---------|--------|--------|------|
| `.\run_mobile.ps1` | Launch Mobile | Bottom Tabs | Compact |
| `.\run_desktop.ps1` | Launch Desktop | Sidebar Nav | Standard |

---

## 🏗️ Manual Toggle (One-Liner)

```powershell
# Mobile
$env:OPERATOR_MOBILE_EMU="1"; cargo run --release

# Desktop
$env:OPERATOR_MOBILE_EMU=""; cargo run --release
```

---

## 🔧 Tweak Guide

| Feature | Target File | Constant/Function | Recommendation |
|---------|-------------|-------------------|----------------|
| Insets | `src/platform.rs` | `SafeArea::android_default()` | Status: 48, Nav: 56 |
| Scaling | `src/ui/mod.rs` | `ctx.set_pixels_per_point(2.0)` | Try 1.5 or 2.5 |
| Buttons | `src/ui/mod.rs` | `INTERACT_SIZE` (via scaling) | 44.0 (Touch Safe) |
| Window | `src/main.rs` | `viewport.with_inner_size()` | Portrait: [400, 800] |

---

## 🧬 Verify Your UI

Check these 4 items whenever you change a screen:
1. **[ ] Safe Zones**: Is text touching the 48px top or 56px bottom?
2. **[ ] Hidden Sidebar**: Does the sidebar accidentally appear?
3. **[ ] Tab Bar Access**: Can you reach the "Logs" tab easily?
4. **[ ] Button Targets**: Are the main buttons big enough (44dp)?

---

## 📜 Key Files Reference

- **`src/platform.rs`**: The brain of detection and safe zones.
- **`src/ui/mod.rs`**: Layout forcing and touch target scaling.
- **`src/main.rs`**: Window size and title configuration.
- **`run_mobile.ps1`**: PowerShell launcher.
