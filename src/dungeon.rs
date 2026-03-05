/// dungeon.rs — Dungeon Track Engine (Sprint 3)
///
/// Port of rpgCore `shared/dungeon/dungeon_engine.py` + `dungeon_track.py`.
///
/// This module is "dormant" — the math and state run fully in Rust but are not
/// rendered until the Egui "zoom-in" layer is wired in a future sprint.
/// All structs are Serialize + Deserialize so an active expedition can be
/// saved to GameState if the player closes the Command Deck mid-run.
use rand::Rng;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// DungeonZoneType — the five node categories
// ---------------------------------------------------------------------------

/// Event types that can appear along a dungeon track.
/// Mirrors `DungeonZoneType` from rpgCore `dungeon_track.py`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DungeonZoneType {
    /// Hostile encounter — party stops, combat resolves before advancing.
    Combat,
    /// Rest node — party pauses 2s, heals 10% HP per slime.
    Rest,
    /// Trap — instant effect, party does not stop.
    Trap,
    /// Loot cache — party stops, loot rolled from depth-scaled table.
    Treasure,
    /// Terminal encounter — must win to complete the run.
    Boss,
}

impl std::fmt::Display for DungeonZoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DungeonZoneType::Combat   => write!(f, "Combat"),
            DungeonZoneType::Rest     => write!(f, "Rest"),
            DungeonZoneType::Trap     => write!(f, "Trap"),
            DungeonZoneType::Treasure => write!(f, "Treasure"),
            DungeonZoneType::Boss     => write!(f, "Boss"),
        }
    }
}

// ---------------------------------------------------------------------------
// DungeonZone — a single node on the track
// ---------------------------------------------------------------------------

/// A named section of the dungeon track with a type and position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonZone {
    /// Distance along track where this zone begins (world units).
    pub start_dist: f32,
    /// Distance along track where this zone ends.
    pub end_dist:   f32,
    /// What kind of event triggers when the party enters this zone.
    pub zone_type:  DungeonZoneType,
    /// True once the party has resolved this zone and may proceed.
    pub resolved:   bool,
}

impl DungeonZone {
    pub fn new(start: f32, end: f32, zone_type: DungeonZoneType) -> Self {
        Self { start_dist: start, end_dist: end, zone_type, resolved: false }
    }

    pub fn contains(&self, dist: f32) -> bool {
        dist >= self.start_dist && dist <= self.end_dist
    }
}

// ---------------------------------------------------------------------------
// DungeonTrack — the full node map
// ---------------------------------------------------------------------------

/// A generated dungeon floor: a linear sequence of zones with a total length.
/// Analogous to a RaceTrack but event-driven instead of speed-driven.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonTrack {
    pub zones:        Vec<DungeonZone>,
    pub total_length: f32,
    /// Floor depth (1 = shallow, N = deep). Scales loot and encounter difficulty.
    pub depth:        u32,
}

impl DungeonTrack {
    /// Generate a procedural dungeon floor for the given depth.
    ///
    /// Layout rules (ported from rpgCore `dungeon_track.py`):
    /// - depth 1: 3 nodes  [Combat, Treasure, Boss]
    /// - depth 2: 4 nodes  [Combat, Rest|Trap, Combat, Boss]
    /// - depth 3: 5 nodes  [Combat, Trap, Rest, Treasure, Boss]
    /// - depth 4+: 5 nodes [Combat, Boss-lite, Trap, Treasure, Boss]
    pub fn generate<R: Rng>(depth: u32, rng: &mut R) -> Self {
        let node_length = 200.0f32; // world units per zone
        let gap         = 50.0f32;  // gap between zones

        let layout: Vec<DungeonZoneType> = match depth {
            1 => vec![
                DungeonZoneType::Combat,
                DungeonZoneType::Treasure,
                DungeonZoneType::Boss,
            ],
            2 => vec![
                DungeonZoneType::Combat,
                if rng.gen_bool(0.5) { DungeonZoneType::Rest } else { DungeonZoneType::Trap },
                DungeonZoneType::Combat,
                DungeonZoneType::Boss,
            ],
            3 => vec![
                DungeonZoneType::Combat,
                DungeonZoneType::Trap,
                DungeonZoneType::Rest,
                DungeonZoneType::Treasure,
                DungeonZoneType::Boss,
            ],
            _ => vec![
                DungeonZoneType::Combat,
                DungeonZoneType::Combat,
                DungeonZoneType::Trap,
                DungeonZoneType::Treasure,
                DungeonZoneType::Boss,
            ],
        };

        let mut zones   = Vec::with_capacity(layout.len());
        let mut cursor  = 0.0f32;
        for zone_type in layout {
            let start = cursor;
            let end   = cursor + node_length;
            zones.push(DungeonZone::new(start, end, zone_type));
            cursor = end + gap;
        }

        Self { zones, total_length: cursor, depth }
    }

    /// Returns the zone that contains `dist`, if any.
    pub fn zone_at(&self, dist: f32) -> Option<usize> {
        self.zones.iter().position(|z| z.contains(dist))
    }
}

// ---------------------------------------------------------------------------
// DungeonEvent — emitted by DungeonEngine::tick()
// ---------------------------------------------------------------------------

/// Events emitted when a party enters a new zone. The caller handles resolution
/// (e.g. dispatch a D20 check, roll loot, reduce HP from a trap).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DungeonEvent {
    CombatEncounter { zone_index: usize },
    RestEncountered { zone_index: usize },
    TrapTriggered   { zone_index: usize },
    TreasureFound   { zone_index: usize },
    BossEncountered { zone_index: usize },
    PathComplete,
}

// ---------------------------------------------------------------------------
// PartyState — runtime scrolling state
// ---------------------------------------------------------------------------

/// The party's position and pause state as they scroll through the dungeon.
/// Serialized so a mid-run state can be saved to GameState and resumed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyState {
    /// How far along the track the party has travelled.
    pub distance:     f32,
    /// Units per second the party advances when not paused.
    pub speed:        f32,
    /// Whether the party is paused (waiting for event resolution).
    pub paused:       bool,
    /// Why the party is paused ("combat" | "rest" | "treasure" | "boss")
    pub pause_reason: String,
    /// Index into `DungeonTrack.zones` of the current zone, if inside one.
    pub zone_index:   Option<usize>,
    /// True once the party has reached `DungeonTrack.total_length`.
    pub finished:     bool,
    /// Countdown for automatic rest completion (in seconds).
    pub rest_timer:   f32,
}

impl Default for PartyState {
    fn default() -> Self {
        Self {
            distance:     0.0,
            speed:        60.0, // units/second
            paused:       false,
            pause_reason: String::new(),
            zone_index:   None,
            finished:     false,
            rest_timer:   0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// DungeonEngine — the tick loop
// ---------------------------------------------------------------------------

/// Drives the party through a DungeonTrack, emitting events when zones are
/// entered. The caller is responsible for resolving events and calling
/// `engine.resume()` when an encounter finishes.
///
/// Ported from rpgCore `dungeon_engine.py::DungeonEngine`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonEngine {
    pub track: DungeonTrack,
    pub party: PartyState,
    /// Number of slimes in the party (for HP heal calculations etc).
    pub party_size: usize,
}

impl DungeonEngine {
    pub fn new(track: DungeonTrack, party_size: usize) -> Self {
        Self { track, party: PartyState::default(), party_size }
    }

    /// Advance the party by `dt` seconds. Returns all events that fired.
    /// If the party is paused, only rest auto-resume is processed.
    pub fn tick(&mut self, dt: f32) -> Vec<DungeonEvent> {
        let mut events = Vec::new();

        // Auto-resume rest timer
        if self.party.paused && self.party.pause_reason == "rest" {
            self.party.rest_timer -= dt;
            if self.party.rest_timer <= 0.0 {
                self.resume();
            }
            return events;
        }

        if self.party.paused || self.party.finished {
            return events;
        }

        self.party.distance += self.party.speed * dt;

        // Check for zone entry
        if let Some(idx) = self.track.zone_at(self.party.distance) {
            if Some(idx) != self.party.zone_index && !self.track.zones[idx].resolved {
                self.party.zone_index = Some(idx);
                if let Some(event) = self.enter_zone(idx) {
                    events.push(event);
                }
            }
        }

        // Check for path completion
        if self.party.distance >= self.track.total_length && !self.party.finished {
            self.party.finished = true;
            events.push(DungeonEvent::PathComplete);
        }

        events
    }

    fn enter_zone(&mut self, idx: usize) -> Option<DungeonEvent> {
        let zone_type = self.track.zones[idx].zone_type;
        match zone_type {
            DungeonZoneType::Combat => {
                self.party.paused       = true;
                self.party.pause_reason = "combat".into();
                Some(DungeonEvent::CombatEncounter { zone_index: idx })
            }
            DungeonZoneType::Rest => {
                self.party.paused       = true;
                self.party.pause_reason = "rest".into();
                self.party.rest_timer   = 2.0; // 2 seconds auto-heal
                Some(DungeonEvent::RestEncountered { zone_index: idx })
            }
            DungeonZoneType::Trap => {
                // Traps don't pause — instant effect
                self.track.zones[idx].resolved = true;
                Some(DungeonEvent::TrapTriggered { zone_index: idx })
            }
            DungeonZoneType::Treasure => {
                self.party.paused       = true;
                self.party.pause_reason = "treasure".into();
                Some(DungeonEvent::TreasureFound { zone_index: idx })
            }
            DungeonZoneType::Boss => {
                self.party.paused       = true;
                self.party.pause_reason = "boss".into();
                Some(DungeonEvent::BossEncountered { zone_index: idx })
            }
        }
    }

    /// Call after the caller has resolved the current encounter.
    /// Marks the zone as resolved and un-pauses the party.
    pub fn resume(&mut self) {
        if let Some(idx) = self.party.zone_index {
            self.track.zones[idx].resolved = true;
        }
        self.party.paused       = false;
        self.party.pause_reason = String::new();
    }

    /// Progress as a 0.0–1.0 fraction of the track completed.
    pub fn progress(&self) -> f32 {
        if self.track.total_length > 0.0 {
            (self.party.distance / self.track.total_length).min(1.0)
        } else {
            0.0
        }
    }

    /// Count how many zones of each type remain unresolved.
    pub fn remaining_encounters(&self) -> usize {
        self.track.zones.iter().filter(|z| !z.resolved).count()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    fn rng() -> SmallRng { SmallRng::seed_from_u64(99) }

    #[test]
    fn depth1_track_has_three_nodes() {
        let track = DungeonTrack::generate(1, &mut rng());
        assert_eq!(track.zones.len(), 3);
    }

    #[test]
    fn depth1_last_zone_is_boss() {
        let track = DungeonTrack::generate(1, &mut rng());
        assert_eq!(track.zones.last().unwrap().zone_type, DungeonZoneType::Boss);
    }

    #[test]
    fn depth3_track_has_five_nodes() {
        let track = DungeonTrack::generate(3, &mut rng());
        assert_eq!(track.zones.len(), 5);
    }

    #[test]
    fn zones_do_not_overlap() {
        let track = DungeonTrack::generate(3, &mut rng());
        let mut prev_end = 0.0f32;
        for zone in &track.zones {
            assert!(zone.start_dist >= prev_end, "Zones must not overlap");
            prev_end = zone.end_dist;
        }
    }

    #[test]
    fn tick_advances_party_when_unpaused() {
        let track = DungeonTrack::generate(1, &mut rng());
        let total = track.total_length;
        let mut engine = DungeonEngine::new(track, 2);
        let _ = engine.tick(1.0); // 1 second at 60 units/s = 60 units
        assert!(engine.party.distance > 0.0 && engine.party.distance < total);
    }

    #[test]
    fn entering_combat_zone_pauses_party() {
        let track = DungeonTrack::generate(1, &mut rng());
        let mut engine = DungeonEngine::new(track, 2);
        // Run until we hit the first zone
        let mut events = Vec::new();
        for _ in 0..200 {
            events.extend(engine.tick(0.1));
            if engine.party.paused { break; }
        }
        assert!(engine.party.paused, "Party must pause on first zone entry");
        assert!(events.iter().any(|e| matches!(e, DungeonEvent::CombatEncounter { .. }
            | DungeonEvent::BossEncountered { .. }
            | DungeonEvent::TreasureFound { .. }
            | DungeonEvent::RestEncountered { .. })));
    }

    #[test]
    fn resume_unpauses_party() {
        let track = DungeonTrack::generate(1, &mut rng());
        let mut engine = DungeonEngine::new(track, 2);
        // Advance until paused
        for _ in 0..200 { engine.tick(0.1); if engine.party.paused { break; } }
        engine.resume();
        assert!(!engine.party.paused);
    }

    #[test]
    fn path_complete_event_fires() {
        let track = DungeonTrack::generate(1, &mut rng());
        let mut engine = DungeonEngine::new(track, 2);
        // Fast-forward to end (skip all pauses by auto-resuming)
        let mut complete = false;
        for _ in 0..5000 {
            let events = engine.tick(0.5);
            if engine.party.paused { engine.resume(); }
            if events.contains(&DungeonEvent::PathComplete) { complete = true; break; }
        }
        assert!(complete, "PathComplete event must fire when party reaches end");
    }

    #[test]
    fn progress_is_clamped_0_to_1() {
        let track = DungeonTrack::generate(1, &mut rng());
        let engine = DungeonEngine::new(track, 2);
        assert!(engine.progress() >= 0.0 && engine.progress() <= 1.0);
    }
}
