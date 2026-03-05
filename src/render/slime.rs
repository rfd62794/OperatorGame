/// render/slime.rs — Procedural Slime Renderer (Sprint 6)
///
/// Pure "Rendered Math" — no .png assets, no external sprites.
/// Every slime is described by `SlimeVisual`, computed from `SlimeGenome`, and drawn
/// each frame via `draw_slime()` using egui's `Painter`.
///
/// **Source of truth (rpgCore):**
/// - `src/shared/rendering/slime_renderer.py` — shapes, patterns, accessories, breathing
/// - `src/apps/slime_breeder/entities/slime.py` — personality axes, mood states
/// - `src/shared/genetics/cultural_base.py` — `CULTURAL_PARAMETERS` (wobble freq, roundness)
///
/// # Visual Layers (drawn in order)
/// 1. Bloom — 3 concentric semi-transparent halos (culture glow color)
/// 2. Body shape — 5 shape types driven by `SlimeShape` (from tier + culture)
/// 3. Pattern overlay — spotted · striped · marbled · iridescent
/// 4. Accessory — crown · glow · shell · crystals · scar
/// 5. Eyes — two dark circles; spacing driven by body width
/// 6. Status badges — dispatched arrow · exhaustion timer · elder crown (level≥10)
/// 7. Selection ring — white outline + culture-colored outer ring

use eframe::egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Vec2};
use std::f32::consts::TAU;

use crate::genetics::{Culture, GeneticTier, LifeStage, SlimeGenome};
use crate::world_map::culture_accent;

// ---------------------------------------------------------------------------
// Cultural parameters — ported from cultural_base.py CULTURAL_PARAMETERS
// ---------------------------------------------------------------------------

/// Animation and shape tendencies for each culture.
/// Ported verbatim from `rpgCore/src/shared/genetics/cultural_base.py`.
#[derive(Debug, Clone, Copy)]
pub struct CultureParams {
    /// Range `(min, max)` — lower = more angular, higher = rounder body.
    pub roundness_range: (f32, f32),
    /// Wobble frequency range `(min, max)` in Hz. Avg used for breathing speed.
    pub wobble_freq: (f32, f32),
    /// Modifier multipliers (relative to baseline 1.0).
    pub hp_mod: f32,
    pub atk_mod: f32,
    pub spd_mod: f32,
}

impl CultureParams {
    pub fn get(c: Culture) -> Self {
        match c {
            Culture::Ember   => Self { roundness_range: (0.2, 0.5), wobble_freq: (1.5, 2.5), hp_mod: 0.8, atk_mod: 1.4, spd_mod: 1.1 },
            Culture::Gale    => Self { roundness_range: (0.3, 0.6), wobble_freq: (2.0, 3.5), hp_mod: 0.9, atk_mod: 0.9, spd_mod: 1.4 },
            Culture::Marsh   => Self { roundness_range: (0.6, 0.9), wobble_freq: (0.5, 1.0), hp_mod: 1.0, atk_mod: 0.9, spd_mod: 1.3 },
            Culture::Crystal => Self { roundness_range: (0.4, 0.7), wobble_freq: (0.3, 0.8), hp_mod: 1.4, atk_mod: 0.8, spd_mod: 0.7 },
            Culture::Tundra  => Self { roundness_range: (0.7, 0.9), wobble_freq: (0.3, 0.6), hp_mod: 1.1, atk_mod: 0.9, spd_mod: 0.8 },
            Culture::Tide    => Self { roundness_range: (0.3, 0.6), wobble_freq: (2.0, 3.5), hp_mod: 1.0, atk_mod: 1.0, spd_mod: 1.2 },
            Culture::Void    => Self { roundness_range: (0.1, 0.9), wobble_freq: (0.1, 3.0), hp_mod: 1.2, atk_mod: 1.2, spd_mod: 1.2 },
        }
    }

    /// Average wobble frequency — used for breathing pulse speed.
    pub fn wobble_avg(self) -> f32 { (self.wobble_freq.0 + self.wobble_freq.1) / 2.0 }
}

// ---------------------------------------------------------------------------
// SlimeShape — 5 shape types (from slime_renderer.py)
// ---------------------------------------------------------------------------

/// The geometric base shape of the slime body.
/// Derived from GeneticTier + Culture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlimeShape {
    /// Default circle (Tiers 1-2, Marsh/Crystal/Tundra)
    Round,
    /// Square body (Tiers 3-4, high ATK cultures)
    Cubic,
    /// Wide ellipse (Ember — "horizontal streak")
    Elongated,
    /// Hexagon (Crystal — geometric)
    Crystalline,
    /// Sine-warped blob with 12-point boundary (Gale/Tide/Void — amorphous)
    Amorphous,
}

impl SlimeShape {
    pub fn for_genome(genome: &SlimeGenome) -> Self {
        let culture = genome.dominant_culture();
        match (culture, genome.genetic_tier()) {
            (Culture::Crystal, _)                        => SlimeShape::Crystalline,
            (Culture::Ember,   t) if t as u8 >= 3       => SlimeShape::Elongated,
            (Culture::Gale,    _) | (Culture::Tide,  _) => SlimeShape::Amorphous,
            (Culture::Void,    _)                        => SlimeShape::Amorphous,
            (_,                t) if t as u8 >= 4       => SlimeShape::Cubic,
            _                                            => SlimeShape::Round,
        }
    }
}

// ---------------------------------------------------------------------------
// SlimePattern — 4 overlay patterns
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlimePattern {
    None,
    Spotted,
    Striped,
    Marbled,
    Iridescent, // Tier 8 Void forced
}

impl SlimePattern {
    pub fn for_genome(genome: &SlimeGenome) -> Self {
        if genome.genetic_tier() == GeneticTier::Void { return SlimePattern::Iridescent; }
        // Deterministic from genome stats — use sum of bytes as a hash
        let hash = genome.base_hp as u32 + genome.base_atk as u32 * 3 + genome.base_spd as u32 * 7;
        match hash % 4 {
            0 => SlimePattern::None,
            1 => SlimePattern::Spotted,
            2 => SlimePattern::Striped,
            _ => SlimePattern::Marbled,
        }
    }
}

// ---------------------------------------------------------------------------
// SlimeAccessory — 5 visual accessories
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlimeAccessory {
    None,
    Crown,    // Yellow jagged crown
    GlowHalo, // Soft white halo
    Shell,    // Brown arc on back
    Crystals, // Small diamond shards
    Scar,     // Dark diagonal line
}

impl SlimeAccessory {
    pub fn for_genome(genome: &SlimeGenome) -> Self {
        let hash = (genome.base_atk as u32 * 11 + genome.base_spd as u32 * 13) % 12;
        match hash {
            0  => SlimeAccessory::Crown,
            1  => SlimeAccessory::GlowHalo,
            2  => SlimeAccessory::Shell,
            3  => SlimeAccessory::Crystals,
            4  => SlimeAccessory::Scar,
            _  => SlimeAccessory::None, // 7/12 chance of no accessory
        }
    }
}

// ---------------------------------------------------------------------------
// Mood — 5 states from slime.py _update_mood()
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlimeMood {
    Happy,
    Sleepy,  // energy < 0.3
    Shy,     // shyness > 0.7
    Playful, // affection > 0.7
    Curious, // curiosity > 0.7
}

impl SlimeMood {
    pub fn label(self) -> &'static str {
        match self {
            SlimeMood::Happy   => "😊",
            SlimeMood::Sleepy  => "😴",
            SlimeMood::Shy     => "😳",
            SlimeMood::Playful => "😄",
            SlimeMood::Curious => "🔍",
        }
    }
}

// ---------------------------------------------------------------------------
// SlimeVisual — the complete per-frame draw descriptor
// ---------------------------------------------------------------------------

/// Everything needed to draw a slime for one frame.
/// Computed from a `SlimeGenome` + a `t` time value (seconds).
/// **No mutable state here** — derive anew each frame from genome + clock.
pub struct SlimeVisual {
    pub shape:     SlimeShape,
    pub pattern:   SlimePattern,
    pub accessory: SlimeAccessory,
    pub mood:      SlimeMood,

    /// Main body color (culture-derived + tier overlay).
    pub body_color:    Color32,
    /// Inner gradient highlight (body_color brightened ×1.2).
    pub inner_color:   Color32,
    /// Pattern overlay color (shifted from body).
    pub pattern_color: Color32,
    /// Bloom/glow ring color (culture accent, semi-transparent).
    pub glow_color:    Color32,

    /// Base radius in UI points before breathing.
    pub base_radius:   f32,
    /// Breathing-animated radius = base_radius × (1 + pulse).
    pub radius:        f32,
    /// Wobble phase offset (time × wobble_freq × TAU).
    pub wobble_phase:  f32,
    /// Wobble amplitude as fraction of radius.
    pub wobble_amp:    f32,

    /// Alpha: 1.0 = normal, 0.55 = dispatched ghost.
    pub alpha:         f32,
    /// Show elder crown (slime level indicator ≥ 10).
    pub is_elder:      bool,
    /// Show dispatched arrow indicator.
    pub is_dispatched: bool,
}

impl SlimeVisual {
    /// Compute the visual descriptor from the genome snapshot.
    ///
    /// - `t` = seconds since app start (from `ctx.input(|i| i.time)`)
    /// - `level` = current slime level (for elder crown check)
    /// - `dispatched` = whether the slime is on an expedition
    pub fn from_genome(genome: &SlimeGenome, t: f32, level: u32, dispatched: bool) -> Self {
        let culture   = genome.dominant_culture();
        let tier      = genome.genetic_tier();
        let params    = CultureParams::get(culture);
        let accent    = culture_accent(culture);

        // --- Size from LifeStage ---
        let base_radius = match genome.life_stage() {
            LifeStage::Hatchling =>  9.0,
            LifeStage::Juvenile  => 15.0,
            LifeStage::Young     => 22.0,
            LifeStage::Prime     => 30.0,
            LifeStage::Veteran   => 38.0,
            LifeStage::Elder     => 48.0,
        };

        // --- Breathing pulse (ported from slime_renderer.py line 28-30) ---
        let pulse_speed = 3.0 * params.wobble_avg();
        let pulse       = (t * pulse_speed).sin() * 0.05;
        let radius      = base_radius * (1.0 + pulse);

        // --- Wobble for amorphous shape ---
        let wobble_phase = t * params.wobble_avg() * TAU;
        let wobble_amp   = 0.22; // 22% of radius, as in Python

        // --- Body color (culture → tier modification) ---
        let [r0, g0, b0, _] = accent;
        let body_color = apply_tier_color(r0, g0, b0, tier, t);
        let inner_color = brighten(body_color, 50);
        let pattern_color = shift_hue(body_color);
        let glow_color  = Color32::from_rgba_unmultiplied(r0, g0, b0, 55);

        // --- Personality-derived mood (from slime.py _update_mood) ---
        // We derive mock "personality axes" from genome stats deterministically
        let energy    = (genome.base_spd / 200.0).clamp(0.0, 1.0);
        let shyness   = 1.0 - (genome.base_atk / 200.0).clamp(0.0, 1.0);
        let affection = (genome.base_hp / 250.0).clamp(0.0, 1.0);
        let curiosity_stat = genome.base_spd / (genome.base_hp + 1.0);
        let mood = if energy < 0.3 { SlimeMood::Sleepy }
            else if shyness > 0.7  { SlimeMood::Shy }
            else if affection > 0.7 { SlimeMood::Playful }
            else if curiosity_stat > 1.2 { SlimeMood::Curious }
            else { SlimeMood::Happy };

        Self {
            shape:         SlimeShape::for_genome(genome),
            pattern:       SlimePattern::for_genome(genome),
            accessory:     SlimeAccessory::for_genome(genome),
            mood,
            body_color,
            inner_color,
            pattern_color,
            glow_color,
            base_radius,
            radius,
            wobble_phase,
            wobble_amp,
            alpha:         if dispatched { 0.55 } else { 1.0 },
            is_elder:      level >= 10,
            is_dispatched: dispatched,
        }
    }
}

// ---------------------------------------------------------------------------
// draw_slime — the main draw function
// ---------------------------------------------------------------------------

/// Draw a slime at `center` using the provided `Painter`.
///
/// **Call once per frame per visible slime.** Pass `ctx.input(|i| i.time as f32)` as `t`.
///
/// ```rust
/// let vis = SlimeVisual::from_genome(&genome, t, level, dispatched);
/// draw_slime(&painter, center, &vis, selected);
/// ```
pub fn draw_slime(painter: &Painter, center: Pos2, vis: &SlimeVisual, selected: bool) {
    let r   = vis.radius;
    let a   = vis.alpha;

    // 1. BLOOM — 3 concentric halos (largest first)
    for i in (1..=3u8).rev() {
        let halo_r     = r + (i as f32) * 4.0 + 2.0;
        let halo_alpha = ((40 - i as u8 * 10) as f32 * a) as u8;
        let [gr, gg, gb, _] = vis.glow_color.to_array();
        painter.circle_filled(center, halo_r, Color32::from_rgba_unmultiplied(gr, gg, gb, halo_alpha));
    }

    // 2. BODY SHAPE
    draw_body(painter, center, r, vis);

    // 3. INNER GRADIENT (smaller concentric circle, brighter)
    let grad_r = r * 0.65;
    painter.circle_filled(center, grad_r, with_alpha(vis.inner_color, (200.0 * a) as u8));

    // 4. PATTERN OVERLAY
    draw_pattern(painter, center, r, vis);

    // 5. ACCESSORY
    draw_accessory(painter, center, r, vis);

    // 6. EYES — two small dark circles
    let eye_off = r * 0.32;
    let eye_r   = (r / 10.0).max(1.5);
    let eye_col = with_alpha(Color32::from_rgb(15, 10, 20), (235.0 * a) as u8);
    painter.circle_filled(Pos2::new(center.x - eye_off, center.y - eye_off * 0.9), eye_r, eye_col);
    painter.circle_filled(Pos2::new(center.x + eye_off, center.y - eye_off * 0.9), eye_r, eye_col);
    // Pupils (tiny white highlight)
    let pupil_r   = eye_r * 0.35;
    let pupil_col = with_alpha(Color32::WHITE, (180.0 * a) as u8);
    painter.circle_filled(Pos2::new(center.x - eye_off + 1.0, center.y - eye_off * 0.9 - 1.0), pupil_r, pupil_col);
    painter.circle_filled(Pos2::new(center.x + eye_off + 1.0, center.y - eye_off * 0.9 - 1.0), pupil_r, pupil_col);

    // 6.5 ELDER CROWN — golden crown polygon for level ≥ 10
    if vis.is_elder {
        draw_elder_crown(painter, center, r, (220.0 * a) as u8);
    }

    // 7. SELECTION RING
    if selected {
        painter.circle_stroke(center, r + 3.0, Stroke::new(2.0, Color32::WHITE));
        painter.circle_stroke(center, r + 5.5, Stroke::new(1.0, with_alpha(vis.body_color, (180.0 * a) as u8)));
    }

    // 8. DISPATCHED ARROW (top-right corner)
    if vis.is_dispatched {
        let tip   = Pos2::new(center.x + r * 0.7, center.y - r * 0.7);
        let arrow_col = Color32::from_rgba_unmultiplied(200, 200, 200, 180);
        painter.line_segment([tip, Pos2::new(tip.x - 8.0, tip.y)], Stroke::new(1.5, arrow_col));
        painter.line_segment([tip, Pos2::new(tip.x, tip.y + 8.0)], Stroke::new(1.5, arrow_col));
    }
}

// ---------------------------------------------------------------------------
// Body shape drawing
// ---------------------------------------------------------------------------

fn draw_body(painter: &Painter, center: Pos2, r: f32, vis: &SlimeVisual) {
    let a   = vis.alpha;
    let col = with_alpha(vis.body_color, (255.0 * a) as u8);

    match vis.shape {
        SlimeShape::Round => {
            painter.circle_filled(center, r, col);
        }
        SlimeShape::Cubic => {
            let rect = Rect::from_center_size(center, Vec2::splat(r * 2.0));
            painter.rect_filled(rect, 4.0, col);
        }
        SlimeShape::Elongated => {
            let w = r * 1.55;
            let h = r * 0.85;
            let pts = ellipse_points(center, w, h, 18);
            painter.add(Shape::convex_polygon(pts, col, Stroke::NONE));
        }
        SlimeShape::Crystalline => {
            let pts = regular_polygon(center, r, 6, 0.0);
            painter.add(Shape::convex_polygon(pts, col, Stroke::NONE));
        }
        SlimeShape::Amorphous => {
            let pts = amorphous_points(center, r, vis.wobble_phase, vis.wobble_amp);
            painter.add(Shape::convex_polygon(pts, col, Stroke::NONE));
        }
    }

    // Body ring
    let ring_col = with_alpha(brighten(vis.body_color, -30), (120.0 * a) as u8);
    painter.circle_stroke(center, r, Stroke::new(1.0, ring_col));
}

// ---------------------------------------------------------------------------
// Pattern overlay drawing
// ---------------------------------------------------------------------------

fn draw_pattern(painter: &Painter, center: Pos2, r: f32, vis: &SlimeVisual) {
    let a    = vis.alpha;
    let pc   = with_alpha(vis.pattern_color, (160.0 * a) as u8);
    let seed = vis.body_color.r() as f32 * 0.7 + vis.body_color.g() as f32 * 0.3;

    match vis.pattern {
        SlimePattern::None => {}
        SlimePattern::Spotted => {
            // 4 small spot circles
            for i in 0u8..4 {
                let off_x = (i as f32 * 1.5 + seed * 0.01).sin() * r * 0.4;
                let off_y = (i as f32 * 1.5 + seed * 0.01).cos() * r * 0.4;
                let spot_r = (r / 6.0).max(2.0);
                painter.circle_filled(Pos2::new(center.x + off_x, center.y + off_y), spot_r, pc);
            }
        }
        SlimePattern::Striped => {
            // 3 horizontal lines
            let lw = (r / 8.0).max(1.0);
            for y_off in [-r / 3.0, 0.0, r / 3.0] {
                painter.line_segment(
                    [Pos2::new(center.x - r + 2.0, center.y + y_off),
                     Pos2::new(center.x + r - 2.0, center.y + y_off)],
                    Stroke::new(lw, pc),
                );
            }
        }
        SlimePattern::Marbled => {
            // Swirl arc (approximated as pentagon arc)
            let swirl_pts: Vec<Pos2> = (0..=8).map(|i| {
                let angle = i as f32 * std::f32::consts::PI / 8.0;
                Pos2::new(center.x + (r * 0.5) * angle.cos(), center.y + (r * 0.5) * angle.sin())
            }).collect();
            for w in swirl_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(2.0, pc));
            }
        }
        SlimePattern::Iridescent => {
            // Offset second halo — Void tier signature
            let iri_col = with_alpha(vis.pattern_color, (80.0 * a) as u8);
            painter.circle_filled(Pos2::new(center.x + 3.0, center.y + 3.0), r * 0.96, iri_col);
        }
    }
}

// ---------------------------------------------------------------------------
// Accessory drawing
// ---------------------------------------------------------------------------

fn draw_accessory(painter: &Painter, center: Pos2, r: f32, vis: &SlimeVisual) {
    let a = vis.alpha;
    match vis.accessory {
        SlimeAccessory::None => {}
        SlimeAccessory::Crown => {
            // Jagged crown: 5-point zigzag above head
            let cy = center.y - r - 2.0;
            let pts = vec![
                Pos2::new(center.x - 10.0, cy),
                Pos2::new(center.x - 5.0,  cy - 10.0),
                Pos2::new(center.x,         cy - 4.0),
                Pos2::new(center.x + 5.0,  cy - 10.0),
                Pos2::new(center.x + 10.0, cy),
            ];
            let col = with_alpha(Color32::from_rgb(255, 215, 0), (220.0 * a) as u8);
            for w in pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(2.0, col));
            }
        }
        SlimeAccessory::GlowHalo => {
            // Large soft glow ring
            let halo_col = with_alpha(Color32::from_rgb(255, 255, 200), (40.0 * a) as u8);
            painter.circle_filled(center, r + 10.0, halo_col);
        }
        SlimeAccessory::Shell => {
            // Brown semicircle arc on the back (top half)
            let shell_col = with_alpha(Color32::from_rgb(139, 69, 19), (200.0 * a) as u8);
            let arc_pts: Vec<Pos2> = (0..=8).map(|i| {
                let angle = std::f32::consts::PI * i as f32 / 8.0;
                Pos2::new(center.x + r * angle.cos(), center.y - r * angle.sin())
            }).collect();
            for w in arc_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(3.0, shell_col));
            }
        }
        SlimeAccessory::Crystals => {
            // 2 small diamond shards above
            let crystal_col = with_alpha(Color32::from_rgb(100, 200, 255), (210.0 * a) as u8);
            for i in 0i32..2 {
                let cx = center.x + (i * 15 - 7) as f32;
                let cy = center.y - r - 5.0;
                let diamond = vec![
                    Pos2::new(cx,       cy - 5.0),
                    Pos2::new(cx + 5.0, cy),
                    Pos2::new(cx,       cy + 5.0),
                    Pos2::new(cx - 5.0, cy),
                ];
                painter.add(Shape::convex_polygon(diamond, crystal_col, Stroke::NONE));
            }
        }
        SlimeAccessory::Scar => {
            // Dark diagonal line across face
            let scar_col = with_alpha(Color32::from_rgb(50, 5, 5), (200.0 * a) as u8);
            painter.line_segment(
                [Pos2::new(center.x - r * 0.5, center.y),
                 Pos2::new(center.x + r * 0.5, center.y + r * 0.45)],
                Stroke::new(2.0, scar_col),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Elder crown (level ≥ 10) — ported from slime_renderer.py line 185-191
// ---------------------------------------------------------------------------

fn draw_elder_crown(painter: &Painter, center: Pos2, r: f32, alpha: u8) {
    let c_y  = center.y - r - 5.0;
    let cx   = center.x;
    let pts  = vec![
        Pos2::new(cx - 12.0, c_y),
        Pos2::new(cx - 6.0,  c_y - 12.0),
        Pos2::new(cx,         c_y - 4.0),
        Pos2::new(cx + 6.0,  c_y - 12.0),
        Pos2::new(cx + 12.0, c_y),
    ];
    let fill_col  = Color32::from_rgba_unmultiplied(255, 215, 0, alpha);
    let rim_col   = Color32::from_rgba_unmultiplied(200, 160, 0, alpha);
    // Fill base as filled polygon
    painter.add(Shape::convex_polygon(pts.clone(), fill_col, Stroke::new(1.5, rim_col)));
    // Rim lines
    for w in pts.windows(2) {
        painter.line_segment([w[0], w[1]], Stroke::new(1.5, rim_col));
    }
}

// ---------------------------------------------------------------------------
// Geometry helpers
// ---------------------------------------------------------------------------

/// Regular n-gon vertices centered at `center`.
fn regular_polygon(center: Pos2, r: f32, sides: u32, phase: f32) -> Vec<Pos2> {
    (0..sides).map(|i| {
        let angle = phase + i as f32 * TAU / sides as f32;
        Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
    }).collect()
}

/// Ellipse points centered at `center` with semi-axes `rx`/`ry`.
fn ellipse_points(center: Pos2, rx: f32, ry: f32, steps: u32) -> Vec<Pos2> {
    (0..steps).map(|i| {
        let angle = i as f32 * TAU / steps as f32;
        Pos2::new(center.x + rx * angle.cos(), center.y + ry * angle.sin())
    }).collect()
}

/// Sine-warped blob boundary — the "amorphous" shape from slime_renderer.py.
/// `phase` = current wobble phase, `amp` = 0..1 fraction of radius.
fn amorphous_points(center: Pos2, r: f32, phase: f32, amp: f32) -> Vec<Pos2> {
    (0..12u32).map(|i| {
        let angle  = i as f32 * TAU / 12.0;
        let wobble = (phase + i as f32).sin() * r * amp;
        Pos2::new(center.x + (r + wobble) * angle.cos(),
                  center.y + (r + wobble) * angle.sin())
    }).collect()
}

// ---------------------------------------------------------------------------
// Color helpers
// ---------------------------------------------------------------------------

/// Apply tier-based color effect (ported from slime_renderer.py _apply_tier_effects).
/// - Tier 8 (Void): slow hue-cycling across all channels
/// - Tier 7 (Liminal): subtle blue-shift
/// - Tiers 1-6: base color unchanged
fn apply_tier_color(r: u8, g: u8, b: u8, tier: GeneticTier, t: f32) -> Color32 {
    match tier {
        GeneticTier::Void => {
            // 360°/60s = 6°/s; 3 sub-ranges of 120°
            let hue_off = (t * 6.0) as u32 % 120;
            let (nr, ng, nb) = if hue_off < 40 {
                (r.saturating_add(2), g.saturating_sub(1), b)
            } else if hue_off < 80 {
                (r, g.saturating_add(2), b.saturating_sub(1))
            } else {
                (r.saturating_sub(1), g, b.saturating_add(2))
            };
            Color32::from_rgb(nr, ng, nb)
        }
        GeneticTier::Liminal => {
            // Subtle blue pulse
            let pulse = ((t * 1.5).sin() * 0.12 + 1.0).max(0.0);
            Color32::from_rgb(
                (r as f32 * (1.0 - pulse * 0.08)) as u8,
                (g as f32 * (1.0 - pulse * 0.04)) as u8,
                (b as f32 * (1.0 + pulse * 0.15)).min(255.0) as u8,
            )
        }
        _ => Color32::from_rgb(r, g, b),
    }
}

/// Brighten by `delta` (positive = lighter, negative = darker).
fn brighten(c: Color32, delta: i16) -> Color32 {
    let clamp = |v: u8, d: i16| (v as i16 + d).clamp(0, 255) as u8;
    Color32::from_rgb(clamp(c.r(), delta), clamp(c.g(), delta), clamp(c.b(), delta))
}

/// Shift hue approximately by rotating channels — simple but effective.
fn shift_hue(c: Color32) -> Color32 {
    Color32::from_rgb(c.g(), c.b(), c.r())
}

/// Replace the alpha channel of a Color32.
fn with_alpha(c: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha)
}

// ---------------------------------------------------------------------------
// draw_slime_card — compact profile card renderer for the Bio-Manifest column
// ---------------------------------------------------------------------------

/// Renders a compact inline slime card within `rect` using egui's Painter.
/// Used in the Command Deck's Bio-Manifest roster column.
///
/// Layout:
/// ```
/// [slime_blob ∅28]  SlimeName           EMBER / PRIME
///                   HP:112 ATK:89 SPD:94
///                   ✅ IDLE   [Shepherd: ⚡ FAST SCOUT]
/// ```
pub fn draw_slime_card(
    painter: &Painter,
    rect: Rect,
    genome: &SlimeGenome,
    t: f32,
    level: u32,
    dispatched: bool,
    selected: bool,
) {
    // Blob preview — left side
    let blob_center = Pos2::new(rect.min.x + 30.0, rect.center().y);
    let vis         = SlimeVisual::from_genome(genome, t, level, dispatched);
    draw_slime(painter, blob_center, &vis, selected);

    // Card background — culture tint
    let [cr, cg, cb, _] = culture_accent(genome.dominant_culture());
    let tint = Color32::from_rgba_unmultiplied(cr, cg, cb, 18);
    painter.rect_filled(rect, 4.0, tint);
    if selected {
        painter.rect_stroke(rect, 4.0, Stroke::new(1.5, Color32::from_rgba_unmultiplied(cr, cg, cb, 180)));
    }
}
