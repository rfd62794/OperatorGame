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
            bottom: 40.0,  // Reduced from 56dp per designer feedback
            left:   8.0,   // Gutter for Moto G horizontal clipping
            right:  8.0,   // Gutter for Moto G horizontal clipping
        }
    }

    /// Zero insets for desktop/host — full window is usable.
    pub fn desktop_default() -> Self {
        Self { top: 0.0, bottom: 0.0, left: 0.0, right: 0.0 }
    }

    /// Shrink `screen` by the inset margins, returning the safe usable rect.
    pub fn apply(&self, screen: Rect) -> Rect {
        let min_x = (screen.min.x + self.left).min(screen.max.x);
        let min_y = (screen.min.y + self.top).min(screen.max.y);
        let max_x = (screen.max.x - self.right).max(min_x);
        let max_y = (screen.max.y - self.bottom).max(min_y);
        
        Rect::from_min_max(
            Pos2::new(min_x, min_y),
            Pos2::new(max_x, max_y),
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
            screen_width:  safe_rect.width().max(0.0),
            screen_height: safe_rect.height().max(0.0),
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
    pub fn bottom_tab_rect(&self, _safe_area: &SafeArea) -> egui::Rect {
        // Standard Android bottom nav height — also exported as TAB_BAR_HEIGHT
        let tab_height = TAB_BAR_HEIGHT;
        
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
// Platform/Emulation Detection
// ---------------------------------------------------------------------------

/// Detects if the internal "Mobile Emulation" mode is active.
///
/// Triggered by the `OPERATOR_MOBILE_EMU=1` environment variable. Allows
/// Windows EXE builds to behave identically to Android for testing.
pub fn is_mobile_emu() -> bool {
    std::env::var("OPERATOR_MOBILE_EMU").is_ok()
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

/// On desktop/host, return zero insets unless emulating mobile.
#[cfg(not(target_os = "android"))]
pub fn read_window_insets() -> SafeArea {
    if is_mobile_emu() {
        SafeArea::android_default()
    } else {
        SafeArea::desktop_default()
    }
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
    ///
    /// Forcing: if `is_mobile_emu()` is true, always returns `Compact`.
    pub fn from_width(width: f32) -> Self {
        if is_mobile_emu() || width < 600.0 {
            Self::Compact
        } else {
            Self::Standard
        }
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
// Layout Constants
// ---------------------------------------------------------------------------

/// Maximum width for center-aligned content panels to ensure readability
/// on large screens while maintaining mobile constraints.
pub const MAX_CONTENT_WIDTH: f32 = 600.0;

// ---------------------------------------------------------------------------
// Bottom Tab Bar (Compact / Mobile)
// ---------------------------------------------------------------------------

/// Standard Android bottom navigation bar height in logical pixels.
///
/// Used by `LayoutCalculator::bottom_tab_rect()` and by `render_radar()` to
/// subtract reserved chrome from the map's available draw height.
pub const TAB_BAR_HEIGHT: f32 = 56.0;

/// Tab bar height in logical pixels for the Compact layout.
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

impl Default for BottomTab {
    fn default() -> Self {
        Self::Roster
    }
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
    Recruit,      // Recruitment agency — purchase_recruit / claim_elders_gift
    Squad,        // STAGED squad overview and combined stats
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

/// Map sub-tabs: Zones (Radar) and Quartermaster (Shop)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MapSubTab {
    Zones,          // Zone affinity, resource yields, current radar
    Quartermaster,  // Equipment shop (G.3)
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_area_desktop_default() {
        let insets = SafeArea::desktop_default();
        assert_eq!(insets.top, 0.0, "Desktop mode should have zero top inset");
        assert_eq!(insets.bottom, 0.0, "Desktop mode should have zero bottom inset");
    }

    #[test]
    fn test_safe_area_android_default() {
        let insets = SafeArea::android_default();
        assert_eq!(insets.top, 48.0, "Android mode should have 48dp top inset (status bar)");
        assert_eq!(insets.bottom, 56.0, "Android mode should have 56dp bottom inset (nav bar)");
    }

    #[test]
    fn test_mobile_emu_detection_when_set() {
        std::env::set_var("OPERATOR_MOBILE_EMU", "1");
        assert!(is_mobile_emu(), "is_mobile_emu() should return true when OPERATOR_MOBILE_EMU is set");
        std::env::remove_var("OPERATOR_MOBILE_EMU");
    }

    #[test]
    fn test_mobile_emu_detection_when_unset() {
        std::env::remove_var("OPERATOR_MOBILE_EMU");
        assert!(!is_mobile_emu(), "is_mobile_emu() should return false when unset");
    }

    #[test]
    fn test_apply_shrinks_rect_correctly() {
        let sa   = SafeArea { top: 48.0, bottom: 56.0, left: 0.0, right: 0.0 };
        let full = Rect::from_min_max(Pos2::ZERO, Pos2::new(1080.0, 2400.0));
        let safe = sa.apply(full);
        assert_eq!(safe.min.y, 48.0);
        assert_eq!(safe.max.y, 2400.0 - 56.0);
        assert_eq!(safe.width(), 1080.0);
    }

    #[test]
    fn test_layout_calculator_column_rects_cover_width() {
        let sa     = SafeArea::desktop_default();
        let layout = LayoutCalculator::new(Vec2::new(900.0, 600.0), sa);
        let r0     = layout.column_rect(0, 3);
        let r2     = layout.column_rect(2, 3);
        assert!((r0.min.x - 0.0).abs() < 0.1);
        assert!((r2.max.x - 900.0).abs() < 0.1);
    }

    #[test]
    fn test_primary_action_guard_is_positive() {
        assert!(PRIMARY_ACTION_BOTTOM_GUARD > 0.0);
    }

    #[test]
    fn test_responsive_layout_compact_below_600() {
        assert_eq!(ResponsiveLayout::from_width(599.9), ResponsiveLayout::Compact);
        assert_eq!(ResponsiveLayout::from_width(0.0),   ResponsiveLayout::Compact);
    }

    #[test]
    fn test_responsive_layout_standard_at_600() {
        assert_eq!(ResponsiveLayout::from_width(600.0), ResponsiveLayout::Standard);
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
            assert!(!label.is_empty());
            assert!(label.chars().next().map(|c| c as u32 > 127).unwrap_or(false));
        }
    }

    #[test]
    fn test_inset_width_consistency() {
        let desktop = SafeArea::desktop_default();
        let android = SafeArea::android_default();
        assert_eq!(std::mem::size_of_val(&desktop.top), std::mem::size_of_val(&android.top));
    }

    #[test]
    fn test_inset_values_are_positive() {
        let android = SafeArea::android_default();
        assert!(android.top >= 0.0);
        assert!(android.bottom >= 0.0);
    }

    #[test]
    fn test_safe_area_sum() {
        let android = SafeArea::android_default();
        let total_vertical_padding = android.top + android.bottom;
        assert_eq!(total_vertical_padding, 88.0, "Moto G 2025: 48dp + 40dp = 88dp");
    }

    #[test]
    fn test_read_window_insets_returns_valid_values() {
        let insets = read_window_insets();
        assert!(insets.top.is_finite());
        assert!(insets.bottom.is_finite());
        assert!(insets.top >= 0.0 && insets.top < 200.0);
    }

    #[test]
    fn test_safe_area_clone() {
        let original = SafeArea::android_default();
        let cloned = original.clone();
        assert_eq!(original.top, cloned.top);
        assert_eq!(original.bottom, cloned.bottom);
    }

    #[test]
    fn test_safe_area_debug_output() {
        let insets = SafeArea::android_default();
        let debug_str = format!("{:?}", insets);
        assert!(debug_str.contains("SafeArea") || debug_str.contains("top") || debug_str.contains("48"));
    }

    #[test]
    fn test_asset_provider_stub_returns_err() {
        let provider = ProcAssetProvider;
        let result   = provider.load_bytes("sounds/ember.wav");
        assert!(result.is_err());
    }

    #[test]
    fn test_platform_integrity_gutters() {
        let sa = SafeArea::android_default();
        assert!(sa.left > 0.0);
        assert!(sa.right > 0.0);
        assert_eq!(MAX_CONTENT_WIDTH, 600.0);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_platform_module_compiles() {
        let _ = SafeArea::android_default();
        let _ = SafeArea::desktop_default();
    }

    #[test]
    fn test_no_panic_on_inset_read() {
        let result = std::panic::catch_unwind(|| {
            let _ = read_window_insets();
        });
        assert!(result.is_ok());
    }
}
