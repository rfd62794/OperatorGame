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

    /// Calculate the rectangle for the 4-tab bottom navigation bar.
    ///
    /// Sits immediately above the soft button inset within the safe area.
    pub fn bottom_tab_rect(&self, safe_area: &SafeArea) -> egui::Rect {
        let tab_height = 48.0; // Standard Android bottom nav height
        
        // Position tabs above soft button area at the bottom of the safe area.
        // Origin.y + screen_height is the bottom of the safe rect.
        let bottom_y = self.origin.y + self.screen_height;
        let top_y = bottom_y - tab_height;
        
        egui::Rect::from_min_max(
            egui::pos2(self.origin.x, top_y),
            egui::pos2(self.origin.x + self.screen_width, bottom_y),
        )
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
// Responsive Layout
// ---------------------------------------------------------------------------

/// Breakpoint-driven layout mode. Determines column count and interaction scale.
///
/// Mobile-first: `Compact` is the Android default. `Standard` activates when
/// the safe area width meets the 600dp threshold for tablet/desktop.
///
/// Sprint 4: drives whether the bottom tab bar or the 3-column side-panel
/// layout is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveLayout {
    /// width < 600dp — mobile single-column + bottom tab bar.
    Compact,
    /// width ≥ 600dp — desktop/tablet 3-column layout.
    Standard,
}

impl ResponsiveLayout {
    /// Derive layout from the safe-area usable width.
    pub fn from_width(width: f32) -> Self {
        if width < 600.0 { Self::Compact } else { Self::Standard }
    }
}

/// Apply platform-appropriate egui interaction sizes to the current frame.
///
/// **Compact (mobile)**: 44dp minimum touch targets (fat-finger safe, WCAG 2.5.5).
/// **Standard (desktop)**: 28dp — precision mouse-work.
///
/// Call once per frame before rendering any panels.
pub fn apply_interaction_scale(ctx: &egui::Context, layout: ResponsiveLayout) {
    ctx.style_mut(|s| {
        s.spacing.interact_size = match layout {
            ResponsiveLayout::Compact  => egui::Vec2::splat(44.0),
            ResponsiveLayout::Standard => egui::Vec2::splat(28.0),
        };
        // Item spacing — breathe on mobile, tighter on desktop
        s.spacing.item_spacing = match layout {
            ResponsiveLayout::Compact  => egui::Vec2::new(8.0, 6.0),
            ResponsiveLayout::Standard => egui::Vec2::new(8.0, 4.0),
        };
    });
}

// ---------------------------------------------------------------------------
// Bottom Tab Bar (Compact / Mobile)
// ---------------------------------------------------------------------------

/// Tab bar height in logical pixels for the Compact layout.
///
/// Sits immediately above the `safe_area.bottom` boundary:
/// `tab_bar_top_y = safe_height - BOTTOM_TAB_BAR_HEIGHT`
pub const BOTTOM_TAB_BAR_HEIGHT: f32 = 56.0;

/// The four primary navigation destinations in Compact (mobile) layout.
///
/// Sprint 4: rendered as an icon + label bar pinned to the safe-area bottom.
/// Sprint 3.5: constants defined only — no rendering yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BottomTab {
    /// 🧬 Bio-Manifest — slime roster and genome browser.
    Roster,
    /// 🚀 Deploy — mission and expedition dispatch.
    Missions,
    /// 🗺️ Planet — world map and expedition targets.
    Map,
    /// 📜 AAR — after action reports and log history.
    Logs,
}

impl BottomTab {
    /// Display label for the tab bar button.
    pub fn label(self) -> &'static str {
        match self {
            BottomTab::Roster   => "🧬 Roster",
            BottomTab::Missions => "🚀 Missions",
            BottomTab::Map      => "🗺️ Map",
            BottomTab::Logs     => "📜 Logs",
        }
    }
}

/// Roster sub-tabs: genetics, breeding, collection
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RosterSubTab {
    Collection,   // All slimes, genetics tree
    Breeding,     // Pair selection, timers, hatch notifications
    // Reserved for future expansion
}

impl Default for RosterSubTab {
    fn default() -> Self {
        Self::Collection
    }
}

/// Missions sub-tabs: active expeditions, quest board
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MissionsSubTab {
    Active,       // Ongoing expeditions, timers, returns
    QuestBoard,   // Available missions, target selection
    // Reserved for future expansion
}

impl Default for MissionsSubTab {
    fn default() -> Self {
        Self::Active
    }
}

/// Map sub-tabs: currently flat, reserved for expansion
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MapSubTab {
    Zones,        // Zone affinity, resource yields
    // Reserved: Alliances, Trade, Procedural
}

impl Default for MapSubTab {
    fn default() -> Self {
        Self::Zones
    }
}

/// Logs sub-tabs: mission history, culture/trade history, operational records
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LogsSubTab {
    MissionHistory,    // AAR outcomes, rolls, narrative
    CultureHistory,    // Culture recruitment, standing changes
    // Reserved: OpLog, CargoLog (if Ops/Cargo restore needed)
}

impl Default for LogsSubTab {
    fn default() -> Self {
        Self::MissionHistory
    }
}

// ---------------------------------------------------------------------------
// Asset Provider — forward-looking abstraction
// ---------------------------------------------------------------------------

/// Abstraction over asset loading for Web (HTTP/WASM), Mobile (APK assets),
/// and Desktop (filesystem).
///
/// **Sprint 3.5**: stub only. Required when cymatics/audio assets land.
/// **Future sprint**: filesystem, HTTP, and APK implementations.
pub trait AssetProvider: Send + Sync {
    /// Load raw bytes for the given asset path.
    ///
    /// Path is relative and cross-platform (e.g. `"sounds/ember.wav"`).
    fn load_bytes(&self, path: &str) -> Result<Vec<u8>, String>;
}

/// Fully procedural provider — no file I/O, no assets required.
///
/// Used when everything is generated at runtime (e.g. procedural cymatics).
pub struct ProcAssetProvider;

impl AssetProvider for ProcAssetProvider {
    fn load_bytes(&self, _path: &str) -> Result<Vec<u8>, String> {
        Err("Procedural engine: no file assets required".into())
    }
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

    // Phase D — Sprint 3.5 tests -------------------------------------------

    #[test]
    fn test_responsive_layout_compact_below_600() {
        assert_eq!(ResponsiveLayout::from_width(599.9), ResponsiveLayout::Compact);
        assert_eq!(ResponsiveLayout::from_width(0.0),   ResponsiveLayout::Compact);
        assert_eq!(ResponsiveLayout::from_width(375.0), ResponsiveLayout::Compact,
            "375dp (iPhone SE width) must be Compact");
    }

    #[test]
    fn test_responsive_layout_standard_at_600() {
        assert_eq!(ResponsiveLayout::from_width(600.0), ResponsiveLayout::Standard);
        assert_eq!(ResponsiveLayout::from_width(1280.0), ResponsiveLayout::Standard,
            "Desktop width must be Standard");
    }

    #[test]
    fn test_bottom_tab_labels_non_empty() {
        let tabs = [
            BottomTab::Roster,
            BottomTab::Missions,
            BottomTab::Map,
            BottomTab::Logs,
        ];
        for tab in tabs {
            let label = tab.label();
            assert!(!label.is_empty(), "{:?} must have a non-empty label", tab);
            // Ensure the label has a unicode icon (first char above ASCII)
            assert!(
                label.chars().next().map(|c| c as u32 > 127).unwrap_or(false),
                "{:?} label should start with an emoji icon", tab
            );
        }
    }

    #[test]
    fn test_asset_provider_stub_returns_err() {
        let provider = ProcAssetProvider;
        let result   = provider.load_bytes("sounds/ember.wav");
        assert!(result.is_err(), "ProcAssetProvider must always return Err");
        let msg = result.unwrap_err();
        assert!(!msg.is_empty(), "Error message must not be empty");
    }
}
