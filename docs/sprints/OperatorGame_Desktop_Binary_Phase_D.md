# OperatorGame --- Desktop Binary & Type Abstraction (Phase D)

**Directive Type:** IMPLEMENTATION  
**Scope:** Decouple garden.rs from egui types + add desktop binary entry point  
**Test Floor:** 170 → 185 passing (15 new tests)  
**Acceptance:** `cargo run` on desktop <5 seconds; APK build unaffected; garden logic portable  

---

## Goal

Enable fast desktop iteration by (1) abstracting egui math types from garden.rs, (2) adding an explicit desktop binary, and (3) ensuring the game loop runs identically on desktop and Android.

---

## Phase D.1: Define Internal Geometry Types

### Location: `src/geometry.rs` (NEW FILE)

Create platform-agnostic math primitives:

```rust
/// 2D point with f32 precision
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn distance_to(self, other: Point) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn scale(self, factor: f32) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

/// 2D axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Bounds {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min_x
            && point.x <= self.max_x
            && point.y >= self.min_y
            && point.y <= self.max_y
    }

    pub fn center(&self) -> Point {
        Point {
            x: (self.min_x + self.max_x) / 2.0,
            y: (self.min_y + self.max_y) / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let result = p1.add(p2);
        assert_eq!(result, Point::new(4.0, 6.0));
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert!((p1.distance_to(p2) - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_bounds_contains() {
        let b = Bounds::new(0.0, 0.0, 10.0, 10.0);
        assert!(b.contains(Point::new(5.0, 5.0)));
        assert!(!b.contains(Point::new(11.0, 5.0)));
    }

    #[test]
    fn test_bounds_center() {
        let b = Bounds::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(b.center(), Point::new(5.0, 5.0));
    }
}
```

### Add to `src/lib.rs`:

```rust
pub mod geometry;
```

---

## Phase D.2: Update `garden.rs` to Use Internal Types

### Location: `src/garden.rs`

**Replace at the top:**

```rust
// OLD:
use egui::{Pos2, Rect, Vec2};

// NEW:
use crate::geometry::{Point, Bounds};
```

### In `GardenAgent` struct definition:

**OLD:**
```rust
pub struct GardenAgent {
    pub position: Pos2,
    pub velocity: Pos2,
    pub bounds: Rect,
    // ...
}
```

**NEW:**
```rust
pub struct GardenAgent {
    pub position: Point,
    pub velocity: Point,
    pub bounds: Bounds,
    // ...
}
```

### In `tick()` method and all simulation code:

Replace egui calls with internal types:

**OLD:**
```rust
let new_pos = Pos2 {
    x: self.position.x + self.velocity.x * dt,
    y: self.position.y + self.velocity.y * dt,
};
```

**NEW:**
```rust
let new_pos = self.position.add(self.velocity.scale(dt));
```

**OLD:**
```rust
if self.bounds.contains(new_pos) {
    self.position = new_pos;
}
```

**NEW:**
```rust
if self.bounds.contains(new_pos) {
    self.position = new_pos;
}
```

(No change needed — `Bounds::contains` works the same way.)

### Anywhere egui math is inlined, use internal types:

Search for all instances of `Pos2::`, `Vec2::`, `Rect::` and replace:
- `Pos2 { x: a, y: b }` → `Point::new(a, b)`
- `Pos2::distance(p1, p2)` → `p1.distance_to(p2)`
- `rect.min` / `rect.max` → `Bounds` with `min_x, min_y, max_x, max_y` fields

**Expected changes:** ~30 lines across the file.

---

## Phase D.3: Bridge Layer for UI Rendering

### Location: `src/render/mod.rs` or new `src/render/garden_bridge.rs`

Create a **one-way conversion** from internal types to egui for rendering:

```rust
/// Convert internal Point to egui::Pos2 for rendering
pub fn point_to_egui(p: &crate::geometry::Point) -> egui::Pos2 {
    egui::Pos2 { x: p.x, y: p.y }
}

/// Convert internal Bounds to egui::Rect for rendering
pub fn bounds_to_egui(b: &crate::geometry::Bounds) -> egui::Rect {
    egui::Rect::from_min_max(
        egui::Pos2 {
            x: b.min_x,
            y: b.min_y,
        },
        egui::Pos2 {
            x: b.max_x,
            y: b.max_y,
        },
    )
}

/// Render a GardenAgent using egui::Painter
pub fn draw_garden_agent(
    painter: &egui::Painter,
    agent: &crate::garden::GardenAgent,
    color: egui::Color32,
) {
    let egui_pos = point_to_egui(&agent.position);
    painter.circle_filled(egui_pos, 5.0, color);
}
```

### In `src/ui/mod.rs`, update garden rendering:

**OLD:**
```rust
// Directly accessing egui types in garden.rs
painter.circle_filled(agent.position, 5.0, color);
```

**NEW:**
```rust
// Use bridge layer
crate::render::draw_garden_agent(&painter, &agent, color);
```

---

## Phase D.4: Add Desktop Binary Entry Point

### Location: `src/main.rs` (CREATE if missing, or update if exists)

```rust
use operator::{models::GameState, ui::OperatorApp};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "OperatorGame",
        options,
        Box::new(|cc| {
            let app = OperatorApp::new(cc);
            Ok(Box::new(app))
        }),
    )
}
```

### Add to `Cargo.toml`:

```toml
[[bin]]
name = "operatorgame"
path = "src/main.rs"
```

(Ensure the `[lib]` section still exists and exports `operator`.)

---

## Phase D.5: Update `Cargo.toml` Metadata

### Verify/Update root `Cargo.toml`:

```toml
[package]
name = "operator"
version = "0.1.0"
edition = "2021"

[lib]
name = "operator"
path = "src/lib.rs"
crate-type = ["rlib", "cdylib"]  # ← Keep both for Android + desktop

[[bin]]
name = "operatorgame"
path = "src/main.rs"
required-features = []  # ← No Android JNI needed for desktop

[dependencies]
egui = "0.27"
eframe = "0.27"
# ... other deps

[target.'cfg(target_os = "android")']
dependencies = []  # Android-specific deps if needed
```

---

## Phase D.6: Tests for New Geometry Types + Integration

### Location: `tests/geometry_integration.rs` (NEW FILE)

```rust
use operator::geometry::{Point, Bounds};

#[test]
fn test_point_arithmetic() {
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(3.0, 4.0);
    assert_eq!(p1.add(p2), Point::new(4.0, 6.0));
    assert_eq!(p2.sub(p1), Point::new(2.0, 2.0));
}

#[test]
fn test_point_scale() {
    let p = Point::new(2.0, 3.0);
    assert_eq!(p.scale(2.0), Point::new(4.0, 6.0));
}

#[test]
fn test_bounds_operations() {
    let b = Bounds::new(0.0, 0.0, 100.0, 100.0);
    assert_eq!(b.width(), 100.0);
    assert_eq!(b.height(), 100.0);
    assert!(b.contains(Point::new(50.0, 50.0)));
    assert!(!b.contains(Point::new(150.0, 50.0)));
}

#[test]
fn test_garden_agent_with_internal_types() {
    use operator::garden::GardenAgent;
    
    let agent = GardenAgent {
        position: Point::new(10.0, 10.0),
        velocity: Point::new(1.0, 1.0),
        bounds: Bounds::new(0.0, 0.0, 100.0, 100.0),
        ..Default::default()
    };
    
    // Simulate one tick (dt = 0.016 seconds)
    let new_pos = agent.position.add(agent.velocity.scale(0.016));
    assert!(agent.bounds.contains(new_pos));
}

#[test]
fn test_render_bridge_conversions() {
    use operator::geometry::{Point, Bounds};
    use operator::render::garden_bridge::{point_to_egui, bounds_to_egui};
    
    let p = Point::new(100.0, 200.0);
    let egui_p = point_to_egui(&p);
    assert_eq!(egui_p.x, 100.0);
    assert_eq!(egui_p.y, 200.0);
    
    let b = Bounds::new(0.0, 0.0, 100.0, 100.0);
    let egui_b = bounds_to_egui(&b);
    assert!(egui_b.width() == 100.0);
    assert!(egui_b.height() == 100.0);
}

#[test]
fn test_desktop_binary_compiles() {
    // This test just verifies that src/main.rs compiles.
    // Actual execution is manual: `cargo run`
}

// Add 10 more integration tests covering:
// - Garden tick() with new types
// - Bounds collision detection
// - Point velocity application
// - Render bridge layer conversions
// - Desktop vs. Android feature detection
```

---

## Test Floor

**Before:** 170 passing  
**Target:** 185 passing (15 new tests)

---

## Acceptance Criteria

✓ `geometry.rs` created with `Point` and `Bounds` types (tested)  
✓ `garden.rs` imports removed: no `egui::Pos2`, `egui::Rect`, `egui::Vec2`  
✓ All `garden.rs` simulation logic uses internal `Point` / `Bounds`  
✓ Render bridge layer (`garden_bridge.rs` or equivalent) converts to egui for UI  
✓ `src/main.rs` created and links to `OperatorApp`  
✓ `Cargo.toml` declares `[[bin]]` with `operatorgame` entry  
✓ Desktop binary compiles: `cargo build --release`  
✓ Desktop binary runs: `cargo run` launches window, game loop works  
✓ Android APK still builds: `cargo apk build --release --target=aarch64-linux-android`  
✓ All 15 new tests passing  
✓ Test floor: 185 / 185 (zero regressions)  
✓ Code compiles to desktop + aarch64 + armv7 without warnings  

---

## Iteration Speed Validation

**After Phase D completes, user should verify:**

1. Desktop iteration: `cargo run` → window opens in <5 seconds
2. Code change → recompile → <5 seconds (hot reload not implemented, but rebuild is fast)
3. APK build still works: `cargo apk build --release --target=aarch64-linux-android`
4. No regressions: Game state, persistence, garden simulation identical on desktop vs. Android

---

## Notes for Agent

- **One-way bridge.** Internal types don't know about egui. Egui conversion happens only in `render/`.
- **No circular imports.** `geometry.rs` is a leaf node; `garden.rs` imports it; `render/` imports both.
- **Keep garden.rs simulation pure.** All tick(), collision, velocity logic uses internal types. UI rendering is separate.
- **Desktop binary is simple.** Just load `OperatorApp` and run. No platform-specific code needed.
- **Feature flags optional for now.** Desktop can use egui without issue; Android uses the same.

---

## Deliverables

1. **New `src/geometry.rs`** — `Point`, `Bounds` with math methods
2. **Updated `src/garden.rs`** — Replaces egui types with internal geometry
3. **New `src/render/garden_bridge.rs`** (or in `src/render/mod.rs`) — Conversion layer
4. **New/Updated `src/main.rs`** — Desktop binary entry point
5. **Updated `src/lib.rs`** — Declares `geometry` module, re-exports for tests
6. **Updated `Cargo.toml`** — Declares `[[bin]]` for desktop
7. **New `tests/geometry_integration.rs`** — 15 integration tests
8. **Build verification** — Desktop + Android compile, no regressions

---

## Completion Checklist

- [ ] `geometry.rs` created with Point/Bounds (4 methods each, tested)
- [ ] `garden.rs` refactored to use internal types (all 30 lines)
- [ ] Render bridge layer created (point_to_egui, bounds_to_egui, draw_garden_agent)
- [ ] `src/main.rs` created (eframe window, OperatorApp init)
- [ ] `Cargo.toml` updated (lib + bin declared)
- [ ] All 15 new tests passing
- [ ] Test floor: 185 / 185
- [ ] Desktop binary runs: `cargo run` opens window
- [ ] Android APK builds: `cargo apk build --release --target=aarch64-linux-android`
- [ ] No code warnings (all targets)
- [ ] Iteration speed validated (<5 seconds code-to-screen)
