/// garden.rs — The Shepherd's Garden (Sprint 7)
///
/// The "Meso-View" habitat within the Command Deck. When slimes aren't on
/// expeditions they live here, wandering according to their `SlimeMood`.
///
/// **Sources of truth (rpgCore):**
/// - `apps/slime_breeder/entities/slime.py` — wander timer, zone-based AI,
///   `_update_mood()`, shyness/affection/curiosity force system
///
/// # Personality-Driven Steering (ported from slime.py)
///
/// | Mood     | Behaviour                                        | Source axis         |
/// |----------|--------------------------------------------------|---------------------|
/// | Sleepy   | Hardly moves; decelerates toward (0,0) of rect  | energy < 0.3        |
/// | Shy      | Flees toward nearest wall / corner               | shyness > 0.7       |
/// | Playful  | Seeks other slimes; clusters                     | affection > 0.7     |
/// | Curious  | Drifts toward the cursor / last-click point      | curiosity ratio > 1.2|
/// | Happy    | Random wander; rebounces off walls               | default             |
///
/// # Architecture
///
/// `GardenAgent` holds display state (pos, vel, wander timer) per slime.
/// `Garden` owns a `Vec<GardenAgent>` and is driven by `tick(dt, cursor)`.
/// `draw_garden(painter, rect, &garden, t)` draws all agents — call from the egui `update`.
///
/// The garden is **not serialised** — positions reset on every boot (the
/// "Internal Habitat" reconfigures each session). Only `SlimeGenome` is
/// persistent.

use eframe::egui::{Color32, Painter, Pos2, Rect, Stroke};
use std::collections::HashMap;
use uuid::Uuid;

use crate::genetics::SlimeGenome;
use crate::render::slime::{draw_slime, SlimeMood, SlimeVisual};

// ---------------------------------------------------------------------------
// Constants — ported from slime.py update logic
// ---------------------------------------------------------------------------

/// Shyness radius: if cursor is within this many points, shy slimes flee.
pub const SHYNESS_RADIUS:      f32 = 80.0;
/// Affection zone: slimes notice each other within this radius (Playful).
pub const CLUSTER_RADIUS:      f32 = 120.0;
/// Curious "magnet" radius toward cursor (if distance < CURIOUS_RADIUS).
pub const CURIOUS_RADIUS:      f32 = 200.0;
/// Max speed in points/sec (SPD stat scales within this).
pub const MAX_SPEED_BASE:      f32 = 60.0;
/// Friction multiplier per tick (higher = more slippery).
pub const FRICTION:            f32 = 0.90;
/// Wall bounce margin in points.
pub const WALL_MARGIN:         f32 = 25.0;
/// Wander timer range (secs). Mood modifies this.
pub const WANDER_INTERVAL_MIN: f32 = 1.0;
pub const WANDER_INTERVAL_MAX: f32 = 3.5;

// ---------------------------------------------------------------------------
// GardenAgent — per-slime physics + AI state
// ---------------------------------------------------------------------------

/// Runtime physics + AI state for one slime in the garden.
/// Initialized from a `SlimeGenome`; updated every frame.
#[derive(Debug, Clone)]
pub struct GardenAgent {
    /// Stable reference back to the genome's ID.
    pub genome_id: Uuid,
    /// Current position in garden-local coordinates.
    pub pos:       Pos2,
    /// Current velocity (pts/sec).
    pub vel:       Pos2,
    /// Seconds until next target re-roll.
    pub wander_timer: f32,
    /// The personality axes derived from genome stats (0.0–1.0).
    pub energy:    f32,
    pub shyness:   f32,
    pub affection: f32,
    pub curiosity: f32,
    /// Cached mood — updated each tick.
    pub mood:      SlimeMood,
    /// Current wander target (garden-local).
    pub target:    Option<Pos2>,
    /// Level — for elder crown
    pub level:     u32,
    /// Dispatched (ghost state)
    pub dispatched: bool,
}

impl GardenAgent {
    /// Build a new garden agent from a genome. `spawn` is the initial
    /// position in garden-local coordinates.
    pub fn new(genome: &SlimeGenome, spawn: Pos2, level: u32, dispatched: bool) -> Self {
        // Derive personality axes from genome stats (same formula as SlimeVisual)
        let energy    = (genome.base_spd / 200.0).clamp(0.0, 1.0);
        let shyness   = 1.0 - (genome.base_atk / 200.0).clamp(0.0, 1.0);
        let affection = (genome.base_hp / 250.0).clamp(0.0, 1.0);
        let curiosity = (genome.base_spd / (genome.base_hp + 1.0)).clamp(0.0, 2.0) / 2.0;

        let mood = derive_mood(energy, shyness, affection, curiosity);

        Self {
            genome_id:   genome.id,
            pos:         spawn,
            vel:         Pos2::ZERO,
            wander_timer: 0.0,
            energy,
            shyness,
            affection,
            curiosity,
            mood,
            target:      None,
            level,
            dispatched,
        }
    }

    /// Max speed for this agent.
    fn max_speed(&self) -> f32 {
        MAX_SPEED_BASE * (0.5 + self.energy * 1.5)
    }
}

// ---------------------------------------------------------------------------
// Garden — the wander arena
// ---------------------------------------------------------------------------

/// The full garden simulation state. Call `tick(dt, cursor, rect)` each frame,
/// then `draw_garden(painter, rect, &genomes, &garden, t)`.
pub struct Garden {
    pub agents: Vec<GardenAgent>,
    /// Currently selected agent id (highlights in blue ring).
    pub selected: Option<Uuid>,
}

impl Garden {
    /// Build a garden from the current roster. Agents are placed in a simple
    /// grid layout within the given rect.
    pub fn from_genomes(genomes: &[SlimeGenome], rect: Rect) -> Self {
        let n = genomes.len();
        let agents = genomes.iter().enumerate().map(|(i, g)| {
            // Deterministic grid spawn — evenly placed left-to-right, row-wrapped
            let cols    = (n as f32).sqrt().ceil().max(1.0) as usize;
            let col     = i % cols;
            let row     = i / cols;
            let cell_w  = rect.width()  / (cols as f32 + 1.0);
            let cell_h  = rect.height() / ((n / cols + 1) as f32 + 1.0);
            let x       = rect.min.x + cell_w * (col as f32 + 1.0);
            let y       = rect.min.y + cell_h * (row as f32 + 1.0);
            GardenAgent::new(g, Pos2::new(x, y), 1, false)
        }).collect();
        Self { agents, selected: None }
    }

    // -----------------------------------------------------------------------
    // Tick — personality-driven steering
    // -----------------------------------------------------------------------

    /// Advance the garden simulation by `dt` seconds.
    ///
    /// - `cursor` — last known cursor position in garden-local space (or None)
    /// - `rect`   — the drawable boundary of the garden UI area
    ///
    /// Ported from `slime.py::update()` + `_get_zone_target()`.
    pub fn tick(&mut self, dt: f32, cursor: Option<Pos2>, rect: Rect) {
        // Snapshot positions for inter-agent queries (don't borrow agents mutably yet)
        let positions: Vec<Pos2> = self.agents.iter().map(|a| a.pos).collect();

        for (idx, agent) in self.agents.iter_mut().enumerate() {
            agent.wander_timer -= dt;

            // --- Mood update every tick ---
            agent.mood = derive_mood(agent.energy, agent.shyness, agent.affection, agent.curiosity);

            let mut force = Pos2::ZERO;

            // ── 1. SHYNESS force (flee cursor) ──────────────────────────────
            if let Some(cur) = cursor {
                let dcur = dist(cur, agent.pos);
                if dcur < SHYNESS_RADIUS && dcur > 0.01 {
                    let flee = normalize(sub(agent.pos, cur));
                    let mag  = (SHYNESS_RADIUS - dcur) / SHYNESS_RADIUS * agent.shyness * 120.0;
                    force = add(force, scale(flee, mag));
                }
            }

            // ── 2. AFFECTION force (cluster toward nearest neighbour) ────────
            if agent.affection > 0.5 {
                let nearest = positions.iter().enumerate()
                    .filter(|(i, _)| *i != idx)
                    .min_by(|(_, a), (_, b)| dist(**a, agent.pos)
                        .partial_cmp(&dist(**b, agent.pos)).unwrap());
                if let Some((_, &nb_pos)) = nearest {
                    let d = dist(nb_pos, agent.pos);
                    if d < CLUSTER_RADIUS && d > 15.0 {
                        let toward = normalize(sub(nb_pos, agent.pos));
                        let mag    = agent.affection * 50.0 * ((CLUSTER_RADIUS - d) / CLUSTER_RADIUS);
                        force = add(force, scale(toward, mag));
                    }
                }
            }

            // ── 3. CURIOSITY force (follow cursor) ───────────────────────────
            if let Some(cur) = cursor {
                let dcur = dist(cur, agent.pos);
                if agent.shyness < 0.5 && agent.curiosity > 0.5 && dcur < CURIOUS_RADIUS && dcur > SHYNESS_RADIUS {
                    let toward = normalize(sub(cur, agent.pos));
                    let mag    = agent.curiosity * 40.0;
                    force = add(force, scale(toward, mag));
                }
            }

            // ── 4. WANDER target (zone-based, mood-driven) ──────────────────
            if agent.wander_timer <= 0.0 {
                let interval = WANDER_INTERVAL_MIN
                    + (1.0 - agent.energy) * (WANDER_INTERVAL_MAX - WANDER_INTERVAL_MIN);
                agent.wander_timer = interval;

                let new_target = match agent.mood {
                    // Shy → flee to nearest wall
                    SlimeMood::Shy => {
                        let cx = rect.center().x;
                        let cy = rect.center().y;
                        let dx = agent.pos.x - cx;
                        let dy = agent.pos.y - cy;
                        let edge_x = if dx > 0.0 { rect.max.x - WALL_MARGIN } else { rect.min.x + WALL_MARGIN };
                        let edge_y = if dy > 0.0 { rect.max.y - WALL_MARGIN } else { rect.min.y + WALL_MARGIN };
                        Some(Pos2::new(edge_x, edge_y))
                    }
                    // Sleepy → barely moves — target stays near current pos
                    SlimeMood::Sleepy => {
                        let jitter_x = (agent.pos.x + 20.0).clamp(rect.min.x + WALL_MARGIN, rect.max.x - WALL_MARGIN);
                        let jitter_y = (agent.pos.y + 10.0).clamp(rect.min.y + WALL_MARGIN, rect.max.y - WALL_MARGIN);
                        Some(Pos2::new(jitter_x, jitter_y))
                    }
                    // Playful → toward center of the pack
                    SlimeMood::Playful => {
                        if positions.is_empty() {
                            None
                        } else {
                            let cx = positions.iter().map(|p| p.x).sum::<f32>() / positions.len() as f32;
                            let cy = positions.iter().map(|p| p.y).sum::<f32>() / positions.len() as f32;
                            Some(Pos2::new(cx, cy))
                        }
                    }
                    // Curious → toward center + slight drift pattern
                    SlimeMood::Curious => {
                        let t_off = (agent.energy * 100.0) as i32 as f32;
                        let cx = rect.center().x + (t_off * 0.05).sin() * rect.width() * 0.3;
                        let cy = rect.center().y + (t_off * 0.07).cos() * rect.height() * 0.3;
                        Some(Pos2::new(
                            cx.clamp(rect.min.x + WALL_MARGIN, rect.max.x - WALL_MARGIN),
                            cy.clamp(rect.min.y + WALL_MARGIN, rect.max.y - WALL_MARGIN),
                        ))
                    }
                    // Happy → random drift within rect (high energy → wide range)
                    SlimeMood::Happy => {
                        let range_x = rect.width()  * 0.4 * (0.3 + agent.energy);
                        let range_y = rect.height() * 0.4 * (0.3 + agent.energy);
                        // Deterministic pseudo-random from pos + timer
                        let px = agent.pos.x + agent.wander_timer * 31.7;
                        let py = agent.pos.y + agent.wander_timer * 17.3;
                        let nx = rect.center().x + (px.sin() * range_x);
                        let ny = rect.center().y + (py.cos() * range_y);
                        Some(Pos2::new(
                            nx.clamp(rect.min.x + WALL_MARGIN, rect.max.x - WALL_MARGIN),
                            ny.clamp(rect.min.y + WALL_MARGIN, rect.max.y - WALL_MARGIN),
                        ))
                    }
                };
                agent.target = new_target;
            }

            // ── 5. Apply target-seeking force ────────────────────────────────
            if let Some(tgt) = agent.target {
                let diff = sub(tgt, agent.pos);
                let d    = mag(diff);
                if d > 8.0 {
                    let dir    = scale(diff, 1.0 / d);
                    let speed  = match agent.mood {
                        SlimeMood::Sleepy => 8.0,
                        SlimeMood::Shy    => 90.0,
                        _                 => 20.0 + agent.energy * 80.0,
                    };
                    force = add(force, scale(dir, speed));
                } else {
                    agent.target = None;
                }
            }

            // ── 6. Integrate velocity ─────────────────────────────────────────
            agent.vel = add(agent.vel, scale(force, dt));
            // Friction
            agent.vel = scale(agent.vel, FRICTION.powf(dt * 30.0));
            // Speed clamp
            let speed = mag(agent.vel);
            if speed > agent.max_speed() {
                agent.vel = scale(agent.vel, agent.max_speed() / speed);
            }
            // Integrate position
            agent.pos = add(agent.pos, scale(agent.vel, dt));

            // ── 7. Wall bounce ────────────────────────────────────────────────
            handle_bounds(&mut agent.pos, &mut agent.vel, rect);
        }
    }

    // -----------------------------------------------------------------------
    // Click handling — select an agent in garden space
    // -----------------------------------------------------------------------

    /// Handle a click at `click_pos` (gallery-local). Returns the `genome_id`
    /// of the agent clicked, or `None`.
    pub fn handle_click(&mut self, click_pos: Pos2) -> Option<Uuid> {
        // Hit-radius: fixed 25pts (covers all life stages)
        for agent in &self.agents {
            if dist(click_pos, agent.pos) < 28.0 {
                self.selected = Some(agent.genome_id);
                return Some(agent.genome_id);
            }
        }
        self.selected = None;
        None
    }
}

// ---------------------------------------------------------------------------
// draw_garden — render all agents
// ---------------------------------------------------------------------------

/// Draw all slimes in the garden.
///
/// - `painter`  — egui Painter restricted to the garden rect
/// - `rect`     — garden boundary (for background tint)
/// - `genomes`  — map from genome_id → SlimeGenome (used for SlimeVisual)
/// - `garden`   — the live simulation state
/// - `t`        — current time in seconds (from `ctx.input(|i| i.time as f32)`)
pub fn draw_garden(
    painter:  &Painter,
    rect:     Rect,
    genomes:  &HashMap<Uuid, &SlimeGenome>,
    garden:   &Garden,
    t:        f32,
) {
    // Garden background
    painter.rect_filled(rect, 6.0, Color32::from_rgba_unmultiplied(10, 16, 26, 220));
    painter.rect_stroke(rect, 6.0, Stroke::new(1.0, Color32::from_rgb(30, 45, 70)));

    // Draw each agent
    for agent in &garden.agents {
        let Some(&genome) = genomes.get(&agent.genome_id) else { continue };
        let vis      = SlimeVisual::from_genome(genome, t, agent.level, agent.dispatched);
        let selected = garden.selected == Some(agent.genome_id);
        draw_slime(painter, agent.pos, &vis, selected);

        // Mood label (tiny emoji below the slime)
        let label_pos = Pos2::new(agent.pos.x, agent.pos.y + vis.radius + 10.0);
        painter.text(
            label_pos,
            eframe::egui::Align2::CENTER_CENTER,
            agent.mood.label(),
            eframe::egui::FontId::proportional(10.0),
            Color32::from_rgba_unmultiplied(220, 220, 220, 140),
        );
    }

    // "INTERNAL HABITAT" watermark
    painter.text(
        Pos2::new(rect.max.x - 8.0, rect.min.y + 6.0),
        eframe::egui::Align2::RIGHT_TOP,
        "INTERNAL HABITAT",
        eframe::egui::FontId::monospace(8.0),
        Color32::from_rgba_unmultiplied(40, 80, 120, 100),
    );
}

// ---------------------------------------------------------------------------
// Garden unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genetics::{Culture, SlimeGenome};

    fn test_genome(hp: f32, atk: f32, spd: f32) -> SlimeGenome {
        SlimeGenome {
            base_hp:  hp,
            base_atk: atk,
            base_spd: spd,
            ..SlimeGenome::default()
        }
    }

    fn test_rect() -> Rect {
        Rect::from_min_size(Pos2::ZERO, eframe::egui::Vec2::new(400.0, 300.0))
    }

    #[test]
    fn agent_spawns_within_rect() {
        let g    = test_genome(100.0, 100.0, 100.0);
        let rect = test_rect();
        let agent = GardenAgent::new(&g, rect.center(), 1, false);
        assert!(rect.contains(agent.pos));
    }

    #[test]
    fn mood_sleepy_when_low_energy() {
        // SPD/200 → energy; HP=250, ATK=1 → energy = 20/200 = 0.10
        let g    = test_genome(250.0, 1.0, 20.0);
        let rect = test_rect();
        let a    = GardenAgent::new(&g, rect.center(), 1, false);
        assert_eq!(a.mood, SlimeMood::Sleepy);
    }

    #[test]
    fn mood_shy_when_low_atk() {
        // shyness = 1 - (atk/200); atk=5 → shyness=0.975 > 0.7  → Shy
        // energy = spd/200 = 100/200 = 0.5 (not Sleepy)
        let g = test_genome(100.0, 5.0, 100.0);
        let a = GardenAgent::new(&g, test_rect().center(), 1, false);
        assert_eq!(a.mood, SlimeMood::Shy);
    }

    #[test]
    fn mood_playful_when_high_hp() {
        // affection = hp/250 > 0.7 when hp > 175
        // Need energy >= 0.3 (spd >= 60), shyness <= 0.7 (atk >= 60)
        let g = test_genome(200.0, 100.0, 100.0);
        let a = GardenAgent::new(&g, test_rect().center(), 1, false);
        assert_eq!(a.mood, SlimeMood::Playful);
    }

    #[test]
    fn tick_moves_agent() {
        let g    = test_genome(80.0, 150.0, 180.0); // high SPD → energetic Happy
        let rect = test_rect();
        let mut garden = Garden::from_genomes(&[g], rect);
        let pos_before = garden.agents[0].pos;
        // Tick several times to let wander timer fire + movement accumulate
        for _ in 0..60 { garden.tick(0.016, None, rect); }
        let pos_after = garden.agents[0].pos;
        // Not necessarily moved (could have zero wander target) — just assert in bounds
        assert!(rect.contains(pos_after), "Agent escaped bounds: {:?}", pos_after);
        let _ = pos_before;
    }

    #[test]
    fn garden_from_genomes_places_all() {
        let gs: Vec<SlimeGenome> = (0..6).map(|_| SlimeGenome::default()).collect();
        let rect = test_rect();
        let g = Garden::from_genomes(&gs, rect);
        assert_eq!(g.agents.len(), 6);
        for a in &g.agents { assert!(rect.contains(a.pos)); }
    }

    #[test]
    fn click_selects_agent() {
        let g = test_genome(100.0, 100.0, 100.0);
        let rect = test_rect();
        let mut garden = Garden::from_genomes(&[g], rect);
        let pos = garden.agents[0].pos;
        let hit = garden.handle_click(pos);
        assert!(hit.is_some());
        assert_eq!(garden.selected, hit);
    }

    #[test]
    fn click_miss_deselects() {
        let g = test_genome(100.0, 100.0, 100.0);
        let rect = test_rect();
        let mut garden = Garden::from_genomes(&[g], rect);
        garden.selected = Some(garden.agents[0].genome_id);
        let hit = garden.handle_click(Pos2::new(390.0, 290.0)); // far corner
        assert!(hit.is_none());
        assert_eq!(garden.selected, None);
    }
}

// ---------------------------------------------------------------------------
// Math helpers (no nalgebra dependency — keep it simple)
// ---------------------------------------------------------------------------

#[inline] fn add(a: Pos2, b: Pos2)  -> Pos2 { Pos2::new(a.x + b.x, a.y + b.y) }
#[inline] fn sub(a: Pos2, b: Pos2)  -> Pos2 { Pos2::new(a.x - b.x, a.y - b.y) }
#[inline] fn scale(a: Pos2, s: f32) -> Pos2 { Pos2::new(a.x * s, a.y * s) }
#[inline] fn mag(a: Pos2)           -> f32  { (a.x * a.x + a.y * a.y).sqrt() }
#[inline] fn dist(a: Pos2, b: Pos2) -> f32  { mag(sub(a, b)) }
#[inline] fn normalize(a: Pos2)     -> Pos2 {
    let m = mag(a);
    if m > 0.001 { scale(a, 1.0 / m) } else { Pos2::ZERO }
}

fn handle_bounds(pos: &mut Pos2, vel: &mut Pos2, rect: Rect) {
    let margin = WALL_MARGIN;
    if pos.x < rect.min.x + margin { pos.x = rect.min.x + margin; vel.x = vel.x.abs() * 0.5; }
    if pos.x > rect.max.x - margin { pos.x = rect.max.x - margin; vel.x = -vel.x.abs() * 0.5; }
    if pos.y < rect.min.y + margin { pos.y = rect.min.y + margin; vel.y = vel.y.abs() * 0.5; }
    if pos.y > rect.max.y - margin { pos.y = rect.max.y - margin; vel.y = -vel.y.abs() * 0.5; }
}

fn derive_mood(energy: f32, shyness: f32, affection: f32, curiosity: f32) -> SlimeMood {
    if energy < 0.3       { SlimeMood::Sleepy }
    else if shyness > 0.7 { SlimeMood::Shy }
    else if affection > 0.7 { SlimeMood::Playful }
    else if curiosity > 0.6 { SlimeMood::Curious }
    else                  { SlimeMood::Happy }
}
