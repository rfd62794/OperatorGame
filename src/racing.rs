/// racing.rs — Race Track Engine (Sprint 3)
///
/// Port of rpgCore `shared/racing/race_engine.py` + `race_track.py`.
///
/// This module is "dormant" — the physics and state run in Rust but are not
/// rendered until the Egui zoom-in layer is wired in a future sprint.
/// All structs are Serialize + Deserialize so an active race session can be
/// saved to GameState mid-run.
///
/// The render layer (terrain colours, camera, speed lines) lives in Python
/// for now and will be ported to Egui as part of the "Scouting Run" milestone.
use rand::Rng;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// TerrainType — track surface
// ---------------------------------------------------------------------------

/// Surface types from rpgCore `race_track.py`.
/// Each terrain applies a speed multiplier when the racer is inside the zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    /// Default surface — no modifier.
    Grass,
    /// Slows to 70% speed.
    Water,
    /// 80% speed, random stumble chance.
    Rock,
    /// 80% speed, suppresses jump.
    Mud,
}

impl TerrainType {
    /// Speed multiplier applied to a racer's velocity while inside this terrain.
    pub fn speed_multiplier(self) -> f32 {
        match self {
            TerrainType::Grass => 1.00,
            TerrainType::Water => 0.70,
            TerrainType::Rock  => 0.80,
            TerrainType::Mud   => 0.80,
        }
    }

    /// True if jumping is suppressed on this terrain (Mud).
    pub fn suppresses_jump(self) -> bool {
        self == TerrainType::Mud
    }
}

impl std::fmt::Display for TerrainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerrainType::Grass => write!(f, "Grass"),
            TerrainType::Water => write!(f, "Water"),
            TerrainType::Rock  => write!(f, "Rock"),
            TerrainType::Mud   => write!(f, "Mud"),
        }
    }
}

// ---------------------------------------------------------------------------
// TerrainZone — a stretch of the track with a given surface
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainZone {
    pub start_dist:   f32,
    pub end_dist:     f32,
    pub terrain_type: TerrainType,
}

impl TerrainZone {
    pub fn contains(&self, dist: f32) -> bool {
        dist >= self.start_dist && dist < self.end_dist
    }
}

// ---------------------------------------------------------------------------
// RaceTrack — procedural track generation
// ---------------------------------------------------------------------------

/// A linear race track built from a sequence of TerrainZones.
/// `num_laps` stores how many times racers must complete the full track.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceTrack {
    pub zones:     Vec<TerrainZone>,
    pub lap_dist:  f32,   // length of a single lap in world units
    pub num_laps:  u32,
}

impl RaceTrack {
    /// Total race distance (all laps combined).
    pub fn total_dist(&self) -> f32 {
        self.lap_dist * self.num_laps as f32
    }

    /// Generate a procedural track.
    ///
    /// Rules from rpgCore `race_track.py`:
    /// - Track has `num_zones` segments per lap
    /// - First and last segment are always Grass (clean start/finish)
    /// - Middle segments randomly assigned; Water/Rock/Mud appear at ~25% each
    pub fn generate<R: Rng>(
        track_length: f32,
        num_zones:    usize,
        num_laps:     u32,
        rng:          &mut R,
    ) -> Self {
        let zone_len  = track_length / num_zones as f32;
        let mut zones = Vec::with_capacity(num_zones);

        for i in 0..num_zones {
            let start = i as f32 * zone_len;
            let end   = start + zone_len;
            let terrain = if i == 0 || i == num_zones - 1 {
                // Start/finish always Grass
                TerrainType::Grass
            } else {
                // Weighted random: 50% Grass, ~17% each hazard
                match rng.gen_range(0..6) {
                    0 | 1 | 2 => TerrainType::Grass,
                    3          => TerrainType::Water,
                    4          => TerrainType::Rock,
                    _          => TerrainType::Mud,
                }
            };
            zones.push(TerrainZone { start_dist: start, end_dist: end, terrain_type: terrain });
        }

        Self { zones, lap_dist: track_length, num_laps }
    }

    /// Returns the terrain at a given distance along a single lap.
    pub fn terrain_at(&self, lap_dist: f32) -> TerrainType {
        let d = lap_dist % self.lap_dist;
        self.zones.iter()
            .find(|z| z.contains(d))
            .map(|z| z.terrain_type)
            .unwrap_or(TerrainType::Grass)
    }
}

// ---------------------------------------------------------------------------
// RacerState — per-racer runtime state
// ---------------------------------------------------------------------------

/// Runtime state for a single racer in the RaceEngine.
/// Stores physics, lap counts, and finish status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacerState {
    /// UUID string of the SlimeGenome this racer represents.
    pub slime_id:       String,
    pub name:           String,
    /// Base speed from genome (units/sec, pre-terrain).
    pub base_speed:     f32,
    /// Current velocity (terrain-modified).
    pub velocity:       f32,
    /// Total distance travelled.
    pub distance:       f32,
    /// How many full laps completed.
    pub laps_complete:  u32,
    /// True once the racer has crossed the finish line.
    pub finished:       bool,
    /// Finish rank (1 = winner). None until finished.
    pub rank:           Option<u32>,
    /// Current jump height (0.0 = on ground).
    pub jump_height:    f32,
    /// Whether the racer is mid-jump.
    pub is_jumping:     bool,
    /// Cooldown before next jump is allowed (seconds).
    pub jump_cooldown:  f32,
}

impl RacerState {
    pub fn new(slime_id: impl Into<String>, name: impl Into<String>, base_speed: f32) -> Self {
        Self {
            slime_id:      slime_id.into(),
            name:          name.into(),
            base_speed,
            velocity:      base_speed,
            distance:      0.0,
            laps_complete: 0,
            finished:      false,
            rank:          None,
            jump_height:   0.0,
            is_jumping:    false,
            jump_cooldown: 0.0,
        }
    }

    /// Progress as a 0.0–1.0 fraction of the total race.
    pub fn progress(&self, track: &RaceTrack) -> f32 {
        let total = track.total_dist();
        if total > 0.0 { (self.distance / total).min(1.0) } else { 0.0 }
    }
}

// ---------------------------------------------------------------------------
// RaceEngine — tick loop
// ---------------------------------------------------------------------------

/// Drives a set of racers through a RaceTrack frame by frame.
///
/// Ported from rpgCore `race_engine.py::RaceEngine`.
/// Rendering is intentionally absent — this is the "logic core" only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceEngine {
    pub track:       RaceTrack,
    pub racers:      Vec<RacerState>,
    finish_count:    u32, // how many have finished (for rank assignment)
}

impl RaceEngine {
    pub fn new(track: RaceTrack, racers: Vec<RacerState>) -> Self {
        Self { track, racers, finish_count: 0 }
    }

    /// Advance the race by `dt` seconds. Returns finish events.
    pub fn tick<R: Rng>(&mut self, dt: f32, rng: &mut R) -> Vec<RaceEvent> {
        let mut events = Vec::new();

        for racer in &mut self.racers {
            if racer.finished { continue; }

            // Current terrain at racer's single-lap position
            let terrain_mult = self.track.terrain_at(racer.distance).speed_multiplier();

            // Rock: 15% random stumble (halves velocity for this tick)
            let rock_stumble = self.track.terrain_at(racer.distance) == TerrainType::Rock
                && rng.gen_bool(0.15);

            let effective_mult = if rock_stumble { terrain_mult * 0.5 } else { terrain_mult };
            racer.velocity = racer.base_speed * effective_mult;

            // Jump cooldown
            if racer.jump_cooldown > 0.0 {
                racer.jump_cooldown -= dt;
            }

            // Advance distance
            racer.distance += racer.velocity * dt;

            // Lap tracking
            let laps_done = (racer.distance / self.track.lap_dist) as u32;
            if laps_done > racer.laps_complete {
                racer.laps_complete = laps_done;
            }

            // Finish check
            if racer.distance >= self.track.total_dist() && !racer.finished {
                racer.finished = true;
                self.finish_count += 1;
                racer.rank = Some(self.finish_count);
                events.push(RaceEvent::Finished {
                    slime_id: racer.slime_id.clone(),
                    rank:     self.finish_count,
                });
            }
        }

        events
    }

    pub fn is_finished(&self) -> bool {
        self.racers.iter().all(|r| r.finished)
    }

    /// Sorted leaderboard by distance descending (leader first).
    pub fn leaderboard(&self) -> Vec<&RacerState> {
        let mut refs: Vec<_> = self.racers.iter().collect();
        refs.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
        refs
    }
}

/// Events emitted by `RaceEngine::tick()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaceEvent {
    /// A racer crossed the finish line.
    Finished { slime_id: String, rank: u32 },
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    fn rng() -> SmallRng { SmallRng::seed_from_u64(7) }

    #[test]
    fn track_first_last_zone_is_grass() {
        let track = RaceTrack::generate(3000.0, 8, 3, &mut rng());
        assert_eq!(track.zones.first().unwrap().terrain_type, TerrainType::Grass);
        assert_eq!(track.zones.last().unwrap().terrain_type,  TerrainType::Grass);
    }

    #[test]
    fn total_dist_equals_lap_times_laps() {
        let track = RaceTrack::generate(3000.0, 8, 3, &mut rng());
        assert!((track.total_dist() - 9000.0).abs() < 1.0);
    }

    #[test]
    fn terrain_at_start_is_grass() {
        let track = RaceTrack::generate(3000.0, 8, 3, &mut rng());
        assert_eq!(track.terrain_at(0.0), TerrainType::Grass);
    }

    #[test]
    fn grass_speed_multiplier_is_1() {
        assert!((TerrainType::Grass.speed_multiplier() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn water_slows_below_grass() {
        assert!(TerrainType::Water.speed_multiplier() < TerrainType::Grass.speed_multiplier());
    }

    #[test]
    fn all_racers_finish_given_enough_ticks() {
        let track = RaceTrack::generate(600.0, 4, 1, &mut rng());
        let racers = vec![
            RacerState::new("a", "Alpha", 80.0),
            RacerState::new("b", "Beta",  70.0),
        ];
        let mut engine = RaceEngine::new(track, racers);
        let mut r = rng();
        for _ in 0..1000 {
            engine.tick(0.5, &mut r);
            if engine.is_finished() { break; }
        }
        assert!(engine.is_finished(), "All racers must finish within 1000 ticks");
    }

    #[test]
    fn faster_racer_wins() {
        let track = RaceTrack::generate(600.0, 2, 1, &mut rng()); // all Grass, no hazard
        let racers = vec![
            RacerState::new("slow", "Slow", 50.0),
            RacerState::new("fast", "Fast", 100.0),
        ];
        let mut engine = RaceEngine::new(track, racers);
        let mut r = rng();
        for _ in 0..1000 { engine.tick(0.5, &mut r); if engine.is_finished() { break; } }
        let winner = engine.racers.iter().find(|r| r.rank == Some(1)).unwrap();
        assert_eq!(winner.slime_id, "fast");
    }

    #[test]
    fn leaderboard_sorted_by_distance() {
        let track = RaceTrack::generate(600.0, 4, 1, &mut rng());
        let racers = vec![
            RacerState::new("a", "A", 80.0),
            RacerState::new("b", "B", 60.0),
        ];
        let mut engine = RaceEngine::new(track, racers);
        let mut r = rng();
        engine.tick(1.0, &mut r);
        let board = engine.leaderboard();
        assert!(board[0].distance >= board[1].distance);
    }
}
