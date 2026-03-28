# OperatorGame: Mobile Emulation for Windows EXE
## Google Antigravity IDE + Gemini Flash 3 Session Guide

**Goal:** Enable Windows EXE builds to behave like Android (Moto G 2025) for faster iterative testing.

**Trigger:** `OPERATOR_MOBILE_EMU=1` environment variable (no CLI parsing library needed).

**IDE Context:** Google Gemini Flash 3 in Antigravity IDE
- Fast iterations (Flash 3's latency is lower than Sonnet)
- Excellent at systemic codebase navigation
- Strong Rust support
- Copy-paste friendly for multi-file edits

---

## Phase 1: Platform Detection & Safe Area Simulation

### File: `src/platform.rs` (Create or Modify)

**What it does:**
- Detects if mobile emulation is active
- Returns Android-style safe areas (48dp status bar top, 56dp nav bar bottom) when emulation is ON
- Maintains desktop defaults when emulation is OFF

**Key Functions:**
```rust
pub fn is_mobile_emu() -> bool {
    std::env::var("OPERATOR_MOBILE_EMU").is_ok()
}

#[cfg(not(target_os = "android"))]
pub fn read_window_insets() -> SafeArea {
    if is_mobile_emu() {
        SafeArea::android_default()  // 48.0, 56.0
    } else {
        SafeArea::desktop_default()  // 0.0, 0.0
    }
}

#[cfg(target_os = "android")]
pub fn read_window_insets() -> SafeArea {
    // Actual Android inset reading (unchanged)
}
```

**Tests to add:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_emu_detection() {
        // Verify is_mobile_emu() works (may need env setup)
    }

    #[test]
    fn test_safe_area_defaults() {
        let desktop = SafeArea::desktop_default();
        assert_eq!(desktop.top, 0.0);
        assert_eq!(desktop.bottom, 0.0);

        let android = SafeArea::android_default();
        assert_eq!(android.top, 48.0);
        assert_eq!(android.bottom, 56.0);
    }
}
```

---

## Phase 2: UI Layout Forcing (Compact Mode)

### File: `src/ui/mod.rs` or equivalent layout module

**What it does:**
- Forces `LayoutMode::Compact` (Bottom Tabs) when emulation is ON
- Ignores actual window width; mobile layout always wins
- Ensures all content respects safe areas

**Key Changes:**
```rust
// In your update() or layout logic:

pub enum LayoutMode {
    Compact,   // Bottom tabs, single-column (mobile)
    Expanded,  // Sidebar, multi-column (desktop)
}

fn determine_layout(window_width: f32) -> LayoutMode {
    if crate::platform::is_mobile_emu() {
        LayoutMode::Compact  // Always, regardless of width
    } else if window_width < 800.0 {
        LayoutMode::Compact
    } else {
        LayoutMode::Expanded
    }
}

// In your render loop:
fn apply_safe_areas(ui: &mut egui::Ui) {
    let insets = crate::platform::read_window_insets();
    // Add spacing at top and bottom
    // Constrain content to safe region
}
```

**Verification Checklist:**
- [ ] Sidebar disappears when emulation is ON
- [ ] Bottom Tab bar is visible and active
- [ ] Content doesn't render in top 48dp or bottom 56dp zones
- [ ] Buttons scale to 44dp touch targets (check egui button sizing)

---

## Phase 3: Window Configuration

### File: `src/main.rs`

**What it does:**
- Detects `OPERATOR_MOBILE_EMU` and sets window to 400×800 (portrait mobile)
- Passes emulation flag to app state for UI to consume

**Key Changes:**
```rust
fn main() -> Result<(), eframe::Error> {
    let mobile_emu = std::env::var("OPERATOR_MOBILE_EMU").is_ok();
    
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size(if mobile_emu {
            [400.0, 800.0]  // Moto G-like portrait
        } else {
            [1200.0, 800.0] // Desktop default
        });

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "OperatorGame",
        options,
        Box::new(|_| Box::new(MyApp::new(mobile_emu))),
    )
}
```

**Pass to App State:**
```rust
pub struct MyApp {
    mobile_emu: bool,
    // ... rest of state
}

impl MyApp {
    pub fn new(mobile_emu: bool) -> Self {
        Self {
            mobile_emu,
            // ...
        }
    }
}
```

---

## Phase 4: Tooling—Run Script

### File: `run_mobile.ps1` (Windows PowerShell)

**What it does:**
- One-command launch of mobile-emulated EXE
- Sets environment variable, builds, runs

**Content:**
```powershell
# run_mobile.ps1
# Launch OperatorGame in mobile emulation mode

$env:OPERATOR_MOBILE_EMU = "1"
Write-Host "🔧 Building OperatorGame in Mobile Emulation mode..." -ForegroundColor Cyan
cargo run --release

# Optional: After run, you can add cleanup or logging
Write-Host "✓ Mobile emulation session closed." -ForegroundColor Green
```

**Usage:**
```powershell
.\run_mobile.ps1
```

### File: `run_desktop.ps1` (Optional—for comparison)

```powershell
# run_desktop.ps1
# Launch OperatorGame in standard desktop mode

Write-Host "🖥️ Building OperatorGame in Desktop mode..." -ForegroundColor Cyan
cargo run --release
```

---

## Phase 5: Touch Target Scaling (Optional but Recommended)

### File: `src/ui/mod.rs` or UI helper module

**What it does:**
- Scales button/interactive areas to 44dp (mobile minimum touch target)
- Applied only when mobile emulation is ON

**Implementation Sketch:**
```rust
pub fn button_size_for_platform() -> f32 {
    if crate::platform::is_mobile_emu() {
        44.0 * 2.5  // 44dp ≈ 110px at typical DPI
    } else {
        32.0  // Desktop default
    }
}

// In UI code:
if ui.button("Action").clicked() {
    // handle click
}
```

---

## Verification Plan (Manual Testing)

### Checklist Before Deployment

#### Desktop Mode (Baseline)
- [ ] Run `.\run_desktop.ps1`
- [ ] Window is ~1200×800 (or your default)
- [ ] Sidebar is visible
- [ ] Buttons are normal size
- [ ] No insets at top/bottom

#### Mobile Emulation Mode
- [ ] Run `.\run_mobile.ps1`
- [ ] Window is 400×800 (portrait)
- [ ] **Sidebar disappears**, Bottom Tab bar is visible
- [ ] 48px margin at top (status bar simulation)
- [ ] 56px margin at bottom (nav bar simulation)
- [ ] Content doesn't bleed into margin zones
- [ ] Buttons are noticeably larger (44dp touch targets)
- [ ] All interactions work identically to desktop

#### Save File Isolation (Optional)
- [ ] Desktop mode and mobile emulation share `save.json` (recommended for now)
- [ ] Or: Implement `is_mobile_emu() ? "save_mobile.json" : "save.json"` for isolation

---

## How to Use This in Antigravity IDE + Gemini Flash 3

### Recommended Workflow

1. **Open OperatorGame project in Antigravity IDE**
   
2. **Copy this plan into a session prompt:**
   ```
   I want to implement mobile emulation for my Egui-based OperatorGame Windows EXE.
   Use OPERATOR_MOBILE_EMU=1 environment variable to trigger:
   - Safe area simulation (48dp top, 56dp bottom)
   - Forced compact layout (bottom tabs always)
   - Mobile-sized window (400×800)
   - Touch scaling (44dp targets)
   
   [Paste the implementation plan above]
   
   Start with Phase 1 (platform.rs). I'll guide you through each phase.
   ```

3. **Let Gemini Flash 3 navigate your codebase:**
   - It will ask for your current SafeArea struct, LayoutMode enum, etc.
   - Provide file snippets as needed
   - Flash 3 excels at multi-file refactoring

4. **Iterative Verification:**
   - After each phase, test with `.\run_mobile.ps1`
   - Gemini Flash 3 will suggest adjustments based on what breaks

### Pro Tips for Antigravity IDE + Flash 3

- **Don't paste huge files:** Give Flash 3 the relevant 50-100 line sections
- **Use cargo check early:** After Phase 1 & 2, run `cargo check --lib` to catch borrow checker issues
- **Test incrementally:** Don't wait until all 5 phases are done; verify Phase 1 works before Phase 2
- **Ask for explanations:** Flash 3 is good at explaining Rust idioms if you need clarity

---

## Expected Timeline

- **Phase 1 (Platform Detection):** 10–15 min
- **Phase 2 (Layout Forcing):** 15–20 min
- **Phase 3 (Window Config):** 5–10 min
- **Phase 4 (Run Scripts):** 2–3 min
- **Phase 5 (Touch Scaling):** 10–15 min
- **Testing & Iteration:** 10–20 min

**Total:** ~1–1.5 hours for full implementation + verification

---

## Success Criteria

You know you're done when:

✅ `.\run_mobile.ps1` launches your game in a 400×800 window
✅ Bottom tabs always show (sidebar hidden)
✅ Safe areas are visually respected
✅ Buttons are larger and touch-friendly
✅ Game state/saves work identically to desktop mode
✅ You can test Android UX changes in ~10 seconds (rebuild + run cycle)

---

## Fallback: Minimal MVP

If the full plan feels too large, here's the **minimum viable emulation:**

```rust
// src/platform.rs (minimal)
pub fn is_mobile_emu() -> bool {
    std::env::var("OPERATOR_MOBILE_EMU").is_ok()
}

// src/main.rs (minimal)
fn main() -> Result<(), eframe::Error> {
    let mobile_emu = std::env::var("OPERATOR_MOBILE_EMU").is_ok();
    let size = if mobile_emu { [400.0, 800.0] } else { [1200.0, 800.0] };
    // ... rest of setup
}
```

This alone gives you instant window size switching. Add the layout forcing next.