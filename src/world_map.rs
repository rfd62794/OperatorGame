/// world_map.rs — Cell War Planet Graph (Sprint 4 + v2 pressure upgrade Sprint 5)
///
/// Sprint 4: 15-node fixed graph, simplified bleed-and-flip faction tick.
/// Sprint 5: Full RPS_BEATS pressure system + BFS supply chain ported from
///           rpgCore `demos/culture_node_wars.py` (see DESIGN_BLUEPRINT.md §2).
use crate::genetics::Culture;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

pub mod radial;

// ---------------------------------------------------------------------------
// NodeZoneType — what kind of expedition a node supports
// ---------------------------------------------------------------------------

/// The expedition category a node offers. Maps directly to rpgCore ZoneType
/// and the demo engines in `dungeon.rs` / `racing.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeZoneType {
    /// Dungeon crawl — yields Scrap. Min stage: Young.
    Excavation,
    /// Race run — yields Gold. Min stage: Juvenile.
    ScoutingRun,
    /// Sumo/Arena contest — yields Gold. Min stage: Young.
    TerritoryDispute,
    /// Foraging — yields Food. Min stage: Juvenile.
    BioSurvey,
    /// Trade — yields Gold + influence. Min stage: Young.
    MarketLiaison,
    /// Tower Defense — yields Scrap + tech. Min stage: Prime.
    CrashSiteDefence,
}

impl NodeZoneType {
    pub fn label(self) -> &'static str {
        match self {
            NodeZoneType::Excavation       => "Excavation",
            NodeZoneType::ScoutingRun      => "Scouting Run",
            NodeZoneType::TerritoryDispute => "Territory Dispute",
            NodeZoneType::BioSurvey        => "Bio-Survey",
            NodeZoneType::MarketLiaison    => "Market Liaison",
            NodeZoneType::CrashSiteDefence => "Crash Site Defence",
        }
    }

    /// Shepherd utility requirement for this zone type.
    pub fn shepherd_requirement(self) -> ShepherdRequirement {
        match self {
            NodeZoneType::Excavation       => ShepherdRequirement::HeavyLift,
            NodeZoneType::ScoutingRun      => ShepherdRequirement::FastScout,
            NodeZoneType::TerritoryDispute => ShepherdRequirement::CombatReady,
            NodeZoneType::BioSurvey        => ShepherdRequirement::Curious,
            NodeZoneType::MarketLiaison    => ShepherdRequirement::Charismatic,
            NodeZoneType::CrashSiteDefence => ShepherdRequirement::HeavyLift,
        }
    }
}

impl std::fmt::Display for NodeZoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

// ---------------------------------------------------------------------------
// ShepherdRequirement — ADR-014 Pikmin-style obstacle type
// ---------------------------------------------------------------------------

/// What the node requires from the squad — not just raw stats, but a
/// specific capability that the Astronaut must deliberately breed for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShepherdRequirement {
    /// Needs cumulative Massive/Large body size in squad (Scrap mining).
    HeavyLift,
    /// Needs high aggregate SPD (terrain mapping, scouting).
    FastScout,
    /// Needs high combined ATK (territorial fights, sumo arenas).
    CombatReady,
    /// Needs high MND (exploration, environmental reading).
    Curious,
    /// Needs high CHM (trade talks, faction diplomacy).
    Charismatic,
}

impl ShepherdRequirement {
    pub fn label(self) -> &'static str {
        match self {
            ShepherdRequirement::HeavyLift   => "⚙ HEAVY LIFT",
            ShepherdRequirement::FastScout   => "⚡ FAST SCOUT",
            ShepherdRequirement::CombatReady => "⚔ COMBAT READY",
            ShepherdRequirement::Curious     => "🔬 CURIOUS",
            ShepherdRequirement::Charismatic => "💬 CHARISMATIC",
        }
    }

    /// Tip shown in the Profile Card to guide squad composition.
    pub fn squad_tip(self) -> &'static str {
        match self {
            ShepherdRequirement::HeavyLift   => "Send Large/Massive slimes — size scores ATK×mass",
            ShepherdRequirement::FastScout   => "Send high-SPD slimes — Gale culture excels here",
            ShepherdRequirement::CombatReady => "Send high-ATK slimes — Ember culture excels here",
            ShepherdRequirement::Curious     => "Send high-MND slimes — Crystal culture excels here",
            ShepherdRequirement::Charismatic => "Send high-CHM slimes — Tide culture excels here",
        }
    }
}

// ---------------------------------------------------------------------------
// WorldNode — a single point on the planet map
// ---------------------------------------------------------------------------

/// One expedition site on the planet. Has a name, zone type, cultural owner,
/// influence level, and list of adjacent node IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldNode {
    pub id:           usize,
    pub name:         String,
    pub zone_type:    NodeZoneType,
    /// The Culture faction currently holding this node.
    pub owner:        Culture,
    /// How strongly the owner controls this node (0.0–1.0).
    /// Below 0.3 = contested; above 0.7 = firmly held.
    pub influence:    f32,
    /// Adjacent node IDs (for expansion logic and route display).
    pub adjacent:     Vec<usize>,
    /// Is an expedition currently running on this node?
    pub occupied:     bool,
    // --- Radial Graph ---
    pub ring:         u8,
    pub position:     (f32, f32),
    pub difficulty_dc: u32,
    pub resonance_aura: i32,
}

impl WorldNode {
    pub fn is_contested(&self) -> bool { self.influence < 0.3 }
    pub fn is_controlled(&self) -> bool { self.influence >= 0.7 }

    /// Narrative status badge.
    pub fn status_label(&self) -> &'static str {
        if self.occupied  { "LOCKED — EXPEDITION IN PROGRESS" }
        else if self.is_contested()  { "⚔ CONTESTED" }
        else if self.is_controlled() { "🔒 CONTROLLED" }
        else                         { "〄 FRONTIER" }
    }

    /// True if this node has at least one neighbor owned by a different culture.
    pub fn is_frontier(&self, nodes: &[WorldNode]) -> bool {
        if matches!(self.owner, Culture::Void) { return false; }
        self.adjacent.iter()
            .filter_map(|&id| nodes.get(id))
            .any(|nb| nb.owner != self.owner)
    }
}

// ---------------------------------------------------------------------------
// Faction pressure constants — from culture_node_wars.py (DESIGN_BLUEPRINT §2)
// ---------------------------------------------------------------------------

/// Accumulated pressure required to claim an empty node.
pub const PRESSURE_THRESHOLD: f32 = 60.0;
/// Base pressure exerted per tick by one neighbor node.
pub const PRESSURE_PER_TICK:  f32 = 4.0;
/// Pressure decay multiplier applied each tick to all pressure values.
pub const PRESSURE_DECAY:     f32 = 0.85;
/// Supply decay per tick when a node is unsupplied.
pub const SUPPLY_DECAY:       f32 = 2.0;
/// Minimum strength before a node reverts to Empty.
pub const COLLAPSE_STRENGTH:  f32 = 0.0;
/// Probability of ownership flip when strength ≤ 0.15 and under enemy pressure.
pub const FLIP_CHANCE:        f32 = 0.15;
/// Enemy pressure multiplier threshold before a contested flip can occur.
pub const FLIP_THRESHOLD:     f32 = 1.5;

/// Returns the RPS pressure multiplier when `attacker` pressures `defender`.
/// Matches the `RPS_BEATS` dict from culture_node_wars.py (Philosophy B topology).
///
/// | Result     | Multiplier |
/// |------------|------------|
/// | A beats B  |   ×1.4     |
/// | B beats A  |   ×0.6     |
/// | Neutral    |   ×1.0     |
pub fn rps_factor(attacker: Culture, defender: Culture) -> f32 {
    let beats = |a: Culture, b: Culture| -> bool {
        matches!(
            (a, b),
            (Culture::Ember,   Culture::Gale)    |
            (Culture::Ember,   Culture::Marsh)   |
            (Culture::Gale,    Culture::Tundra)  |
            (Culture::Gale,    Culture::Tide)    |
            (Culture::Marsh,   Culture::Crystal) |
            (Culture::Marsh,   Culture::Gale)    |
            (Culture::Crystal, Culture::Tide)    |
            (Culture::Crystal, Culture::Ember)   |
            (Culture::Tundra,  Culture::Ember)   |
            (Culture::Tundra,  Culture::Crystal) |
            (Culture::Tide,    Culture::Marsh)   |
            (Culture::Tide,    Culture::Tundra)
        )
    };
    if beats(attacker, defender) { 1.4 }
    else if beats(defender, attacker) { 0.6 }
    else { 1.0 }
}

/// Per-culture simulation traits.
#[derive(Debug, Clone, Copy)]
pub struct CultureTraits {
    /// Pressure exerted per tick (multiplied on top of base PRESSURE_PER_TICK).
    pub pressure_mult:       f32,
    /// How sensitive the culture is to being cut off from supply.
    pub supply_sensitivity:  f32,
}

impl CultureTraits {
    pub fn for_culture(c: Culture) -> Self {
        match c {
            Culture::Ember   => Self { pressure_mult: 1.3, supply_sensitivity: 1.0 },
            Culture::Gale    => Self { pressure_mult: 1.6, supply_sensitivity: 0.6 },
            Culture::Marsh   => Self { pressure_mult: 0.8, supply_sensitivity: 0.5 },
            Culture::Crystal => Self { pressure_mult: 1.0, supply_sensitivity: 1.0 },
            Culture::Tundra  => Self { pressure_mult: 0.9, supply_sensitivity: 0.3 },
            Culture::Tide    => Self { pressure_mult: 1.2, supply_sensitivity: 1.5 },
            Culture::Void    => Self { pressure_mult: 0.0, supply_sensitivity: 0.0 },
        }
    }
}

/// Maps a `Culture` to a stable array index 0–6 for the pressure buffer.
/// Order matches `Culture::WHEEL`.
#[inline]
fn culture_index(c: Culture) -> usize {
    match c {
        Culture::Ember   => 0,
        Culture::Gale    => 1,
        Culture::Marsh   => 2,
        Culture::Crystal => 3,
        Culture::Tundra  => 4,
        Culture::Tide    => 5,
        Culture::Void    => 6,
    }
}


// ---------------------------------------------------------------------------
// WorldMap — the planet graph
// ---------------------------------------------------------------------------

/// The living planet map: 15 named nodes connected as an undirected graph.
/// Culture factions expand over time, shifting "Frontlines."
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub nodes: Vec<WorldNode>,
    /// Accumulated simulation time since last full faction tick.
    tick_accum: f32,
    pub startled_level: f32, // ADR-015: Hoot & Holler resonance
}

impl WorldMap {
    /// Generate the canonical 19-node planet map.
    ///
    /// Node layout (3 rings):
    ///   Centre:   Crash Site (your base — always Neutral/Void, locked)
    ///   Ring 1:   6 nodes — one per Culture, strongly held
    ///   Ring 2:   6 nodes — frontier/contested, mix of zone types
    ///   Ring 3:   6 nodes — deep wilds, high difficulty
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        let radial_nodes = radial::generate_ripple_map();
        let mut nodes = Vec::new();

        for rn in radial_nodes {
            let name = match rn.ring {
                0 => "Hidden Meadow",
                1 => match rn.culture {
                    Culture::Ember => "Ember Caldera",
                    Culture::Gale => "Gale Ridge",
                    Culture::Tide => "Tide Basin",
                    _ => "Heartlands Node",
                },
                2 => match rn.culture {
                    Culture::Marsh => "Mire Delta",
                    Culture::Crystal => "Prism Market",
                    Culture::Tundra => "Frost Caves",
                    _ => "Wilds Node",
                },
                _ => "Forbidden Reaches",
            };

            let zone_type = match rn.ring {
                0 => NodeZoneType::CrashSiteDefence,
                1 => NodeZoneType::TerritoryDispute,
                2 => NodeZoneType::BioSurvey,
                _ => NodeZoneType::Excavation,
            };

            let influence = if rn.ring <= 1 {
                0.85
            } else {
                rng.gen_range(0.2..0.6)
            };

            let resonance_aura = if rn.ring == 0 { 2 } else { 0 };

            // Simplified ring-based adjacency mapping
            let mut adj = Vec::new();
            if rn.id == 0 {
                adj = vec![1, 2, 3, 4, 5, 6];
            } else {
                let ring = rn.ring as u32;
                let i = rn.id % 10;
                
                let _current_base = ring * 10;
                let left = if i == 0 { 5 } else { i - 1 };
                let right = if i == 5 { 0 } else { i + 1 };
                // Remap pseudo-IDs to dense vector indices for faction tick
                // Since ring 1 is 1-6, ring 2 is 7-12, ring 3 is 13-18
                let dense_id = |r: u32, idx: u32| -> usize {
                    if r == 0 { 0 } else { ( (r - 1) * 6 + idx + 1 ) as usize }
                };

                adj.push(dense_id(ring, left));
                adj.push(dense_id(ring, right));

                if ring > 1 {
                    adj.push(dense_id(ring - 1, i));
                } else {
                    adj.push(0);
                }
                if ring < 3 {
                    adj.push(dense_id(ring + 1, i));
                }
            }

            nodes.push(WorldNode {
                id: nodes.len(), // Force dense ID sequence
                name: name.to_string(),
                zone_type,
                owner: rn.culture,
                influence,
                adjacent: adj,
                occupied: false,
                ring: rn.ring,
                position: rn.position,
                difficulty_dc: rn.difficulty_dc,
                resonance_aura,
            });
        }

        Self { nodes, tick_accum: 0.0, startled_level: 0.0 }
    }

    /// Advance the faction simulation by `dt` seconds.
    /// Influence bleeds across edges every ~60 real-world seconds.
    pub fn tick_factions<R: Rng>(&mut self, dt: f32, rng: &mut R) {
        self.tick_accum += dt;
        if self.tick_accum < 60.0 { return; } // only tick every 60 seconds
        self.tick_accum = 0.0;

        let node_count = self.nodes.len();
        // Collect adjacency + owner + influence snapshot to avoid borrow issues
        let snapshot: Vec<(usize, Culture, f32)> = self.nodes.iter()
            .map(|n| (n.id, n.owner, n.influence))
            .collect();

        for i in 0..node_count {
            if self.nodes[i].occupied { continue; } // expeditions freeze faction movement

            let adj = self.nodes[i].adjacent.clone();
            for &adj_id in &adj {
                if adj_id >= node_count { continue; }
                let (_, adj_owner, adj_influence) = snapshot[adj_id];

                // Stronger adjacent node bleeds 1–3% influence per tick
                if adj_owner != self.nodes[i].owner && adj_influence > self.nodes[i].influence {
                    let bleed = rng.gen_range(0.01f32..0.03);
                    let decay = (self.nodes[i].influence - bleed).max(0.05);
                    self.nodes[i].influence = decay;

                    // Flip owner if influence collapses below threshold
                    if self.nodes[i].influence < 0.2 && rng.gen_bool(0.3) {
                        self.nodes[i].owner = adj_owner;
                        self.nodes[i].influence = 0.25; // reset to frontier strength
                    }
                }
            }
        }
    }

    /// Returns all non-occupied, non-Crash-Site nodes available for dispatch.
    pub fn available_nodes(&self) -> Vec<&WorldNode> {
        self.nodes.iter()
            .filter(|n| n.id != 0 && !n.occupied)
            .collect()
    }

    /// Look up a node by ID.
    pub fn node(&self, id: usize) -> Option<&WorldNode> {
        self.nodes.get(id)
    }

    pub fn node_mut(&mut self, id: usize) -> Option<&mut WorldNode> {
        self.nodes.get_mut(id)
    }

    /// Influence summary for the status bar: {Culture: node_count}.
    pub fn influence_summary(&self) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();
        for node in &self.nodes {
            *map.entry(node.owner.name().to_string()).or_insert(0) += 1;
        }
        map
    }

    // -----------------------------------------------------------------------
    // v2: Full RPS Pressure System (Sprint 5)
    // -----------------------------------------------------------------------

    /// Full pressure tick — ported from `culture_node_wars.py::update()`.
    /// Runs on a 60-second real-world accumulator (same as `tick_factions`).
    ///
    /// Phases (matching the Python blueprint exactly):
    /// 1. **Pressure Phase** — each owned node pushes pressure onto neighbours.
    /// 2. **Claim Phase** — empty nodes flip when pressure ≥ THRESHOLD.
    /// 3. **Contest Phase** — owned nodes weaken under heavy enemy pressure; can flip.
    /// 4. **Supply Phase** — BFS from strongest node per culture; unsupplied nodes decay.
    pub fn tick_pressure<R: Rng>(&mut self, dt: f32, rng: &mut R) {
        self.tick_accum += dt;
        if self.tick_accum < 60.0 { return; }
        self.tick_accum = 0.0;

        let n = self.nodes.len();

        // ── 1. PRESSURE PHASE ──
        // Snapshot owned nodes to avoid mid-loop mutation confusion.
        // pressure[node_id][culture_idx] → accumulated pressure value.
        // We encode Culture as u8 index: Ember=0 Gale=1 Marsh=2 Crystal=3 Tundra=4 Tide=5 Void=6
        let mut pressure: Vec<[f32; 7]> = vec![[0.0f32; 7]; n];

        for i in 0..n {
            let owner = self.nodes[i].owner;
            if matches!(owner, Culture::Void) { continue; }
            let traits  = CultureTraits::for_culture(owner);
            let pmult   = traits.pressure_mult;
            let strength = self.nodes[i].influence;

            let adj = self.nodes[i].adjacent.clone();
            for &nb_id in &adj {
                if nb_id >= n { continue; }
                let nb_owner = self.nodes[nb_id].owner;
                if matches!(nb_owner, Culture::Void) { continue; }

                let factor = if nb_owner == owner {
                    continue; // same culture — no pressure needed
                } else {
                    rps_factor(owner, nb_owner)
                };
                let contrib = PRESSURE_PER_TICK * pmult * strength * factor;
                pressure[nb_id][culture_index(owner)] += contrib;
            }
        }

        // ── 2. CLAIM PHASE (empty nodes) + 3. CONTEST PHASE (owned nodes) ──
        for i in 0..n {
            if self.nodes[i].occupied { continue; }
            let cur = self.nodes[i].owner;
            if matches!(cur, Culture::Void) { continue; }

            let row = pressure[i];
            let cur_idx = culture_index(cur);

            // ── CLAIM — node is currently unowned (we treat Void as "empty" for BFS purposes;
            //    nodes never start Void except Crash Site, but can collapse to Void via supply)
            // ── CONTEST — enemy pressure exceeds the flip threshold
            let enemy_max = Culture::WHEEL.iter()
                .enumerate()
                .filter(|(_, &c)| c != cur && !matches!(c, Culture::Void))
                .map(|(idx, _)| row[idx])
                .fold(0.0f32, f32::max);

            if enemy_max > PRESSURE_THRESHOLD * FLIP_THRESHOLD {
                self.nodes[i].influence = (self.nodes[i].influence - 0.08).max(0.0);
                if self.nodes[i].influence <= 0.15 && rng.gen::<f32>() < FLIP_CHANCE {
                    // Find the winning enemy culture by highest pressure
                    if let Some((winner_idx, _)) = Culture::WHEEL.iter()
                        .enumerate()
                        .filter(|(_, &c)| c != cur && !matches!(c, Culture::Void))
                        .max_by(|(a, _), (b, _)| row[*a].partial_cmp(&row[*b]).unwrap())
                    {
                        self.nodes[i].owner     = Culture::WHEEL[winner_idx];
                        self.nodes[i].influence = 0.25;
                    }
                }
            }

            // Pressure decay on all enemy rows
            let _ = cur_idx; // used for future claim-phase expansion
        }

        // ── 4. SUPPLY PHASE ──
        let supplied = self.compute_supply();
        for node in &mut self.nodes {
            if matches!(node.owner, Culture::Void) || node.occupied { continue; }
            if supplied.contains(&node.id) {
                node.influence = (node.influence + 0.02).min(1.0);
            } else {
                let sens = CultureTraits::for_culture(node.owner).supply_sensitivity;
                node.influence = (node.influence - SUPPLY_DECAY * 0.01 * sens).max(0.0);
                if node.influence <= COLLAPSE_STRENGTH {
                    node.owner    = Culture::Void;
                    node.influence = 0.0;
                }
            }
        }
    }

    /// BFS supply chain — returns the set of node IDs reachable from each
    /// culture's "capitol" (highest-influence node) through same-culture edges.
    ///
    /// Ported from `culture_node_wars.py::compute_supply()` and `find_capitols()`.
    pub fn compute_supply(&self) -> HashSet<usize> {
        let mut supplied = HashSet::new();

        // Find the highest-influence node per culture (the "capitol").
        let mut capitols: HashMap<usize, usize> = HashMap::new();
        for node in &self.nodes {
            if matches!(node.owner, Culture::Void) { continue; }
            let cidx = culture_index(node.owner);
            let best = capitols.entry(cidx).or_insert(node.id);
            if node.influence > self.nodes[*best].influence { *best = node.id; }
        }

        // BFS from each capitol through same-culture nodes.
        for (_cidx, &cap_id) in &capitols {
            let cult = self.nodes[cap_id].owner;
            let mut visited = HashSet::new();
            let mut queue   = VecDeque::new();
            visited.insert(cap_id);
            queue.push_back(cap_id);
            while let Some(cur) = queue.pop_front() {
                supplied.insert(cur);
                for &nb_id in &self.nodes[cur].adjacent {
                    if nb_id < self.nodes.len()
                        && !visited.contains(&nb_id)
                        && self.nodes[nb_id].owner == cult
                    {
                        visited.insert(nb_id);
                        queue.push_back(nb_id);
                    }
                }
            }
        }

        supplied
    }
}

impl Default for WorldMap {
    /// Generates a canonical planet map with seed 2025 so existing saves
    /// get a deterministic fresh map when the field is absent.
    fn default() -> Self {
        use rand::SeedableRng;
        let mut rng = rand::rngs::SmallRng::seed_from_u64(2025);
        Self::generate(&mut rng)
    }
}


// ---------------------------------------------------------------------------
// Profile Card — pure-data display helper (no egui dependency)
// ---------------------------------------------------------------------------

/// Text-mode profile card for a slime. Rendered by ui.rs into an egui frame.
/// Centralises the display logic so it can be unit-tested without egui.
pub struct SlimeProfileCard {
    pub name:           String,
    pub id_short:       String,
    pub dominant:       Culture,
    pub tier_label:     String,
    pub stage_label:    String,
    pub hp:             f32,
    pub atk:            f32,
    pub spd:            f32,
    /// Culture color as RGBA for the dot/stripe accent.
    pub accent_color:   [u8; 4],
    pub status:         SlimeCardStatus,
    pub cooldown_secs:  Option<i64>, // Cellular Exhaustion remaining
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlimeCardStatus {
    Idle,
    Incubating { remaining_secs: i64 },
    Deployed   { node_name: String },
}

impl SlimeProfileCard {
    /// Build a profile card from a SlimeGenome reference.
    pub fn from_genome(genome: &crate::genetics::SlimeGenome) -> Self {
        let dominant = genome.dominant_culture();
        let accent   = culture_accent(dominant);
        let cooldown = genome.exhaustion_remaining()
            .map(|d| d.num_seconds().max(0));

        Self {
            name:          genome.name.clone(),
            id_short:      genome.id.to_string()[..8].to_string(),
            dominant,
            tier_label:    genome.genetic_tier().to_string(),
            stage_label:   genome.life_stage().to_string(),
            hp:            genome.base_hp,
            atk:           genome.base_atk,
            spd:           genome.base_spd,
            accent_color:  accent,
            status:        SlimeCardStatus::Idle,
            cooldown_secs: cooldown,
        }
    }

    /// One-liner stat summary for compact display.
    pub fn stat_line(&self) -> String {
        format!(
            "HP:{:.0}  ATK:{:.0}  SPD:{:.0}  │  {}  {}",
            self.hp, self.atk, self.spd,
            self.tier_label, self.stage_label
        )
    }

    /// Status badge string.
    pub fn status_badge(&self) -> String {
        match &self.status {
            SlimeCardStatus::Idle => {
                if let Some(secs) = self.cooldown_secs {
                    format!("🧬 EXHAUSTED ({}s)", secs)
                } else {
                    "✅ IDLE".into()
                }
            }
            SlimeCardStatus::Incubating { remaining_secs } =>
                format!("🔬 INCUBATING ({}s)", remaining_secs),
            SlimeCardStatus::Deployed { node_name } =>
                format!("🚀 DEPLOYED → {}", node_name),
        }
    }
}

/// Culture → RGBA accent colour for Profile Cards.
pub fn culture_accent(culture: Culture) -> [u8; 4] {
    match culture {
        Culture::Ember   => [220,  80,  40, 255], // deep red-orange
        Culture::Gale    => [120, 200, 255, 255], // sky blue
        Culture::Marsh   => [ 80, 160,  80, 255], // deep green
        Culture::Crystal => [180, 140, 255, 255], // violet
        Culture::Tundra  => [160, 220, 255, 255], // ice blue
        Culture::Tide    => [ 40, 160, 200, 255], // ocean teal
        Culture::Void    => [100, 100, 100, 255], // dark grey
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

    fn rng() -> SmallRng { SmallRng::seed_from_u64(55) }

    #[test]
    fn map_has_15_nodes() {
        let map = WorldMap::generate(&mut rng());
        assert_eq!(map.nodes.len(), 19);
    }

    #[test]
    fn crash_site_is_node_0_void() {
        let map = WorldMap::generate(&mut rng());
        assert_eq!(map.nodes[0].id, 0);
        assert_eq!(map.nodes[0].owner, Culture::Void);
    }

    #[test]
    fn ring1_nodes_have_high_influence() {
        let map = WorldMap::generate(&mut rng());
        for id in 1..=6 {
            assert!(map.nodes[id].influence > 0.6,
                "Ring 1 node {} should start firmly held", id);
        }
    }

    #[test]
    fn available_nodes_excludes_crash_site() {
        let map = WorldMap::generate(&mut rng());
        assert!(map.available_nodes().iter().all(|n| n.id != 0));
    }

    #[test]
    fn available_nodes_excludes_occupied() {
        let mut map = WorldMap::generate(&mut rng());
        map.nodes[1].occupied = true;
        assert!(map.available_nodes().iter().all(|n| n.id != 1));
    }

    #[test]
    fn influence_summary_counts_all_nodes() {
        let map   = WorldMap::generate(&mut rng());
        let total: usize = map.influence_summary().values().sum();
        assert_eq!(total, 19);
    }

    #[test]
    fn tick_factions_does_not_crash() {
        let mut map = WorldMap::generate(&mut rng());
        let mut r   = rng();
        // fast-forward 90 seconds to trigger a tick
        map.tick_factions(90.0, &mut r);
    }

    #[test]
    fn shepherd_requirement_label_non_empty() {
        use NodeZoneType::*;
        for zone in [Excavation, ScoutingRun, TerritoryDispute, BioSurvey, MarketLiaison, CrashSiteDefence] {
            assert!(!zone.shepherd_requirement().label().is_empty());
        }
    }

    #[test]
    fn culture_accent_all_cultures_covered() {
        for c in Culture::WHEEL {
            let acc = culture_accent(c);
            assert_eq!(acc[3], 255, "Accent must be fully opaque");
        }
    }
}
