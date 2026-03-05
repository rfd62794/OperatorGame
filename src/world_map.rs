/// world_map.rs — Cell War Planet Graph (Sprint 4)
///
/// The "Meso Layer" of the multiscale state machine.
/// Replaces the flat mission list with a living graph of 15 named nodes,
/// each owned by a Culture faction with an influence level (0.0–1.0).
///
/// As time passes, factions expand — `tick_factions(dt)` shifts influence,
/// creating "Frontlines" that change which node types are available.
///
/// The Astronaut picks nodes on this map as expedition targets, not abstract
/// mission names. Culture-zone advantage (ADR-006) is derived from the
/// node's current owner vs the slime squad's dominant culture.
use crate::genetics::Culture;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

impl WorldMap {
    /// Generate the canonical 15-node planet map.
    ///
    /// Node layout (3 rings):
    ///   Centre:   Crash Site (your base — always Neutral/Void, locked)
    ///   Ring 1:   6 nodes — one per Culture, strongly held
    ///   Ring 2:   8 nodes — frontier/contested, mix of zone types
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        let mut nodes = vec![
            // ID 0: Crash Site — the Astronaut's base. Not a dispatchable node.
            WorldNode {
                id: 0, name: "Crash Site".into(),
                zone_type: NodeZoneType::CrashSiteDefence,
                owner: Culture::Void, influence: 1.0,
                adjacent: vec![1, 2, 3, 4, 5, 6],
                occupied: false,
            },
            // Ring 1: Cultural heartlands — each culture firmly holds one nearby node
            WorldNode {
                id: 1, name: "Ember Caldera".into(),
                zone_type: NodeZoneType::TerritoryDispute,
                owner: Culture::Ember, influence: 0.85,
                adjacent: vec![0, 2, 6, 7, 8],
                occupied: false,
            },
            WorldNode {
                id: 2, name: "Gale Ridge".into(),
                zone_type: NodeZoneType::ScoutingRun,
                owner: Culture::Gale, influence: 0.80,
                adjacent: vec![0, 1, 3, 8, 9],
                occupied: false,
            },
            WorldNode {
                id: 3, name: "Marsh Wetlands".into(),
                zone_type: NodeZoneType::BioSurvey,
                owner: Culture::Marsh, influence: 0.82,
                adjacent: vec![0, 2, 4, 9, 10],
                occupied: false,
            },
            WorldNode {
                id: 4, name: "Crystal Spire".into(),
                zone_type: NodeZoneType::MarketLiaison,
                owner: Culture::Crystal, influence: 0.79,
                adjacent: vec![0, 3, 5, 10, 11],
                occupied: false,
            },
            WorldNode {
                id: 5, name: "Tundra Shelf".into(),
                zone_type: NodeZoneType::Excavation,
                owner: Culture::Tundra, influence: 0.83,
                adjacent: vec![0, 4, 6, 11, 12],
                occupied: false,
            },
            WorldNode {
                id: 6, name: "Tide Basin".into(),
                zone_type: NodeZoneType::BioSurvey,
                owner: Culture::Tide, influence: 0.81,
                adjacent: vec![0, 1, 5, 12, 13],
                occupied: false,
            },
            // Ring 2: Frontier nodes — mix of contested cultures and zone types
            WorldNode {
                id: 7, name: "Scorched Ruins".into(),
                zone_type: NodeZoneType::Excavation,
                owner: Culture::Ember, influence: rng.gen_range(0.2..0.6),
                adjacent: vec![1, 8, 14],
                occupied: false,
            },
            WorldNode {
                id: 8, name: "Storm Pass".into(),
                zone_type: NodeZoneType::ScoutingRun,
                owner: Culture::Gale, influence: rng.gen_range(0.2..0.6),
                adjacent: vec![1, 2, 7, 9],
                occupied: false,
            },
            WorldNode {
                id: 9, name: "Mire Delta".into(),
                zone_type: NodeZoneType::BioSurvey,
                owner: Culture::Marsh, influence: rng.gen_range(0.2..0.6),
                adjacent: vec![2, 3, 8, 10],
                occupied: false,
            },
            WorldNode {
                id: 10, name: "Prism Market".into(),
                zone_type: NodeZoneType::MarketLiaison,
                owner: Culture::Crystal, influence: rng.gen_range(0.2..0.6),
                adjacent: vec![3, 4, 9, 11],
                occupied: false,
            },
            WorldNode {
                id: 11, name: "Frost Caves".into(),
                zone_type: NodeZoneType::Excavation,
                owner: Culture::Tundra, influence: rng.gen_range(0.2..0.6),
                adjacent: vec![4, 5, 10, 12],
                occupied: false,
            },
            WorldNode {
                id: 12, name: "Kelp Forest".into(),
                zone_type: NodeZoneType::TerritoryDispute,
                owner: Culture::Tide, influence: rng.gen_range(0.2..0.6),
                adjacent: vec![5, 6, 11, 13],
                occupied: false,
            },
            WorldNode {
                id: 13, name: "Deep Seabed".into(),
                zone_type: NodeZoneType::Excavation,
                owner: Culture::Tide, influence: rng.gen_range(0.15..0.5),
                adjacent: vec![6, 12, 14],
                occupied: false,
            },
            WorldNode {
                id: 14, name: "Ash Wastes".into(),
                zone_type: NodeZoneType::TerritoryDispute,
                owner: Culture::Ember, influence: rng.gen_range(0.15..0.5),
                adjacent: vec![7, 13],
                occupied: false,
            },
        ];

        // Randomise influence slightly so the map starts with some asymmetry
        for node in nodes.iter_mut().skip(1) {
            let jitter = rng.gen_range(-0.05f32..0.05);
            node.influence = (node.influence + jitter).clamp(0.05, 1.0);
        }

        Self { nodes, tick_accum: 0.0 }
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
        use crate::genetics::{GeneticTier, LifeStage};

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
        assert_eq!(map.nodes.len(), 15);
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
        assert_eq!(total, 15);
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
