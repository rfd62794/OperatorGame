# OperatorGame --- Android Viewport & Safe Area Inventory & Analysis

**Directive Type:** SHORT --- Inventory & Analysis (no implementation)  
**Target:** Coding Agent  
**Device:** Moto G 2025 (portrait, primary test target)  
**Scope:** Status quo assessment + blockers identification  
**Outcome:** Written report (not code changes)

---

## Goal

Understand the current state of Android viewport handling in OperatorGame and identify what's blocking proper safe area layout for portrait mode on Moto G 2025.

---

## Phase A: Codebase Inventory

### 1. Manifest & Build Configuration
- **Location:** `android/` directory (or equivalent)
- **Find & Report:**
  - Current `targetSdk` and `minSdk`
  - Window layout mode (fullscreen vs. normal)
  - Any existing `android:windowTranslucentStatus` or `android:fitsSystemWindows` flags
  - Build target architecture (aarch64, armv7, both?)

### 2. JNI / Platform Layer (`src/platform.rs`)
- **Current state of:**
  - `SafeArea` struct definition (fields, default values)
  - `LayoutCalculator` — does it use SafeArea yet?
  - `ResponsiveLayout` enum — what variants exist?
  - `apply_interaction_scale()` — what does it do?
  - `BottomTab` struct — is it positioned absolutely or relative?
  - Any existing JNI binding calls to `android.app.Activity`

### 3. Main UI Loop (`src/main.rs` or equivalent)
- **Find & Report:**
  - How is egui window created? (`eframe::run_native`, `NativeOptions`?)
  - Where is `SafeArea` instantiated and used?
  - Current `egui::CentralPanel` / `egui::TopBottomPanel` layout structure
  - Any hardcoded padding/margin values (search for `px`, `dp`, magic numbers)

### 4. Current Runtime Behavior
- **Simulate / Report:**
  - If you can, run the APK on Moto G or emulator
  - Screenshot: What is currently eaten by status bar and soft buttons?
  - Measure in dp: How much vertical space is lost?
  - What parts of the UI are clipped or unreachable?

---

## Phase B: Analysis & Blockers

### 5. Safe Area Query Mechanism
- **Check if implemented:**
  - Does Android JNI layer query system insets at runtime?
  - Function signature: `get_safe_area() -> SafeArea { top: dp, bottom: dp, left: dp, right: dp }`?
  - Is it called on every frame, or once at startup?
  - Fallback behavior if JNI call fails (mock values for WASM/desktop)?

### 6. Layout Application
- **Identify:**
  - Where does `SafeArea` get applied to the egui layout?
  - Is padding applied to the root `CentralPanel`?
  - Are the 4 tabs (`Roster | Missions | Map | Logs`) currently positioned relative to bottom-safe-area, or hardcoded to screen edge?
  - Current `BottomTab` y-position calculation (is it safe?)

### 7. Responsive Layout Status
- **Document:**
  - What does `ResponsiveLayout` currently support? (portrait-only? landscape ready?)
  - Is there a `calculate_layout(screen_width, screen_height, safe_area) -> LayoutRects` function?
  - Does it return separate rects for: content_area, tabs_area, reserve_height?

### 8. Known Issues & TODOs
- **Search for:**
  - Any `TODO`, `FIXME`, `XXX` comments in `platform.rs`
  - Sprint 4 defer notes (JNI insets mentioned line 159)
  - Any hardcoded Moto G safe area values or assumptions

---

## Phase C: Blocker Summary

### Report Format
Provide a structured summary:

```
VIEWPORT INVENTORY --- OperatorGame Android (Moto G 2025 Portrait)

CURRENT STATE:
- Window mode: [fullscreen | normal | unknown]
- SafeArea implementation: [complete | partial | stub]
- JNI insets query: [yes | no | fallback only]
- BottomTab positioning: [safe-aware | hardcoded | absolute]
- Status bar loss: [X dp] Status bar height observed: [Y dp]
- Soft button loss: [X dp] Soft button height observed: [Y dp]

BLOCKERS:
- [Blocker 1]: Description + impact
- [Blocker 2]: Description + impact
- [Blocker 3]: Description + impact

DESIGN DECISIONS NEEDED:
- Fullscreen mode vs. respect insets?
- Where should the 4-tab bar live (bottom vs. top)?
- Hardcoded safe area values or runtime JNI query?
- Responsive breakpoints for tablet/landscape?

NEXT PHASE READINESS:
- Can implement [specific fix] with [dependencies]
- Blocked on [architectural decision X]
```

---

## Acceptance Criteria

✓ All manifest/config values documented  
✓ All relevant `platform.rs` structures and functions mapped  
✓ SafeArea query mechanism status clear (exists / missing / partial)  
✓ BottomTab positioning strategy identified  
✓ Visual measurement of status/button loss (screenshot + dp values)  
✓ Blocker list ranked by impact (high / medium / low)  
✓ Design decision prompts ready for architect review  

---

## Output Deliverable

**File:** `OperatorGame_Android_Viewport_Analysis.md`

**Sections:**
1. Manifest & Build Configuration (bullet list)
2. Platform Layer Inventory (code references + current implementations)
3. Main UI Loop Layout (structural diagram or pseudocode)
4. SafeArea Query Mechanism (is it implemented? how?)
5. Known TODOs & Deferrals (Sprint 4 notes)
6. Observed Behavior (screenshot notes + measurements in dp)
7. Blocker Summary (structured format above)
8. Design Decision Matrix (fullscreen vs. insets, tabs top vs. bottom, etc.)

**Length:** ~2-3 pages (single-spaced Markdown)

---

## Notes for Agent

- This is **discovery work**, not implementation. No code changes yet.
- If you need to run the APK to get measurements, do so. Screenshot and measure in Android Studio device monitor (dp values visible).
- If source is not available locally, infer from docs and report what's missing.
- Flag any assumptions clearly (e.g., "SafeArea struct assumed to exist in platform.rs, but not found — may be stubbed or named differently").
