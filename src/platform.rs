/// platform.rs — Android-aware safe area and layout foundation.
///
/// Provides `SafeArea` (system UI inset reservation) and `LayoutCalculator`
/// (screen geometry gated on safe bounds). All new UI panels in Sprint 3+
/// use this module so they are never obscured by status/nav bars.
///
/// ADR ref: Sprint 3 Android UI Inset Mandate (Moto G 2025 3-button nav).
use egui::{Pos2, Rect, Vec2};

// ---------------------------------------------------------------------------
// Safe Area — system UI inset reservation
// ---------------------------------------------------------------------------

/// System UI inset reservation for all platforms, in logical pixels (dp).
///
/// On Android, the status bar, soft nav bar, and any notch/cutout consume
/// real screen edge space that egui does not automatically exclude from its
/// rendering surface. `SafeArea` captures these margins so every panel and
/// button is placed inside the truly usable region.
#[derive(Debug, Clone, Copy)]
pub struct SafeArea {
    /// Status bar height (24–48dp typical, more on notch devices).
    pub top: f32,
    /// Soft nav bar (3-button ≈ 48dp; gesture ≈ 20dp; home indicator on gesture nav).
    pub bottom: f32,
    /// Side cutout/notch padding.
    pub left: f32,
    /// Side cutout/notch padding.
    pub right: f32,
}

impl SafeArea {
    /// Conservative fallback for Moto G 2025 with 3-button soft nav (default).
    ///
    /// Runtime inset reading will replace this in Sprint 4 when the JNI
    /// `WindowInsetsCompat.systemBars()` call is wired in.
    pub fn android_default() -> Self {
        Self {
            top:    48.0,
            bottom: 56.0,  // 48dp nav bar + 8dp primary action guard
            left:   0.0,
            right:  0.0,
        }
    }

    /// Zero insets for desktop/host — full window is usable.
    pub fn desktop_default() -> Self {
        Self { top: 0.0, bottom: 0.0, left: 0.0, right: 0.0 }
    }

    /// Shrink `screen` by the inset margins, returning the safe usable rect.
    pub fn apply(&self, screen: Rect) -> Rect {
        Rect::from_min_max(
            Pos2::new(screen.min.x + self.left,  screen.min.y + self.top),
            Pos2::new(screen.max.x - self.right, screen.max.y - self.bottom),
        )
    }
}

// ---------------------------------------------------------------------------
// Primary Action Guard
// ---------------------------------------------------------------------------

/// Minimum additional padding (dp) between any primary action button
/// (Deploy, Launch, Confirm) and the safe area's bottom edge.
///
/// Effective bottom clearance = `safe_area.bottom + PRIMARY_ACTION_BOTTOM_GUARD`.
/// On Moto G 2025: 56 + 8 = 64dp above raw screen bottom.
pub const PRIMARY_ACTION_BOTTOM_GUARD: f32 = 8.0;

// ---------------------------------------------------------------------------
// Layout Calculator
// ---------------------------------------------------------------------------

/// Wraps a safe-area-gated screen rect and provides geometry helpers for
/// column/panel layout. All UI panels built in Sprint 3+ should be constructed
/// through `LayoutCalculator` rather than from raw `ctx.screen_rect()`.
#[derive(Debug, Clone, Copy)]
pub struct LayoutCalculator {
    /// Usable width after insets.
    pub screen_width:  f32,
    /// Usable height after insets.
    pub screen_height: f32,
    /// Top-left origin of the safe area (offset from raw screen top-left).
    pub origin: Pos2,
}

impl LayoutCalculator {
    /// Construct from raw screen dimensions and a `SafeArea`.
    ///
    /// # Example
    /// ```ignore
    /// let safe = read_window_insets();
    /// let layout = LayoutCalculator::new(ctx.screen_rect().size(), safe);
    /// ```
    pub fn new(screen_size: Vec2, safe_area: SafeArea) -> Self {
        let safe_rect = safe_area.apply(
            Rect::from_min_size(Pos2::ZERO, screen_size)
        );
        Self {
            screen_width:  safe_rect.width(),
            screen_height: safe_rect.height(),
            origin:        safe_rect.min,
        }
    }

    /// Rect for a proportional column within the safe area.
    ///
    /// `col_index` is 0-based; `num_cols` is the total column count.
    pub fn column_rect(&self, col_index: usize, num_cols: usize) -> Rect {
        let col_w = self.screen_width / num_cols as f32;
        let x0 = self.origin.x + col_index as f32 * col_w;
        Rect::from_min_size(
            Pos2::new(x0, self.origin.y),
            Vec2::new(col_w, self.screen_height),
        )
    }

    /// Bottom y-coordinate where primary action buttons must sit above.
    /// = safe area bottom edge (already excludes nav bar) - guard padding.
    pub fn primary_action_y_max(&self) -> f32 {
        self.origin.y + self.screen_height - PRIMARY_ACTION_BOTTOM_GUARD
    }
}

// ---------------------------------------------------------------------------
// Android Runtime Inset Hook
// ---------------------------------------------------------------------------

/// Read system bar insets and return the corresponding `SafeArea`.
///
/// **Sprint 3**: returns the conservative static fallback.
/// **Sprint 4+**: wire the `#[cfg(target_os = "android")]` branch to a JNI
/// call via `WindowInsetsCompat.systemBars()` to get live device values:
///
/// ```ignore
/// // TODO Sprint 4+: replace with JNI WindowInsetsCompat call
/// // let insets = env.call_method(activity, "getSystemInsets", "()Landroid/graphics/Insets;", &[])?;
/// // SafeArea { top: insets.top as f32, bottom: insets.bottom as f32, .. }
/// ```
#[cfg(target_os = "android")]
pub fn read_window_insets() -> SafeArea {
    // TODO Sprint 4+: JNI call to WindowInsetsCompat.systemBars()
    SafeArea::android_default()
}

/// On desktop/host, no insets are needed.
#[cfg(not(target_os = "android"))]
pub fn read_window_insets() -> SafeArea {
    SafeArea::desktop_default()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn android_default_has_nonzero_bottom() {
        let sa = SafeArea::android_default();
        assert!(sa.bottom > 0.0, "Android default must reserve nav bar space");
    }

    #[test]
    fn desktop_default_is_zero() {
        let sa = SafeArea::desktop_default();
        assert_eq!(sa.top,    0.0);
        assert_eq!(sa.bottom, 0.0);
        assert_eq!(sa.left,   0.0);
        assert_eq!(sa.right,  0.0);
    }

    #[test]
    fn apply_shrinks_rect_correctly() {
        let sa   = SafeArea { top: 48.0, bottom: 56.0, left: 0.0, right: 0.0 };
        let full = Rect::from_min_max(Pos2::ZERO, Pos2::new(1080.0, 2400.0));
        let safe = sa.apply(full);
        assert_eq!(safe.min.y, 48.0);
        assert_eq!(safe.max.y, 2400.0 - 56.0);
        assert_eq!(safe.width(), 1080.0);
    }

    #[test]
    fn layout_calculator_column_rects_cover_width() {
        let sa     = SafeArea::desktop_default();
        let layout = LayoutCalculator::new(Vec2::new(900.0, 600.0), sa);
        let r0     = layout.column_rect(0, 3);
        let r2     = layout.column_rect(2, 3);
        // All three columns together should span the full width
        assert!((r0.min.x - 0.0).abs() < 0.1);
        assert!((r2.max.x - 900.0).abs() < 0.1);
    }

    #[test]
    fn primary_action_guard_is_positive() {
        assert!(PRIMARY_ACTION_BOTTOM_GUARD > 0.0);
    }
}
