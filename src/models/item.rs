// Moved from models.rs
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Gear — Industrial Grade Tools
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Gear {
    HeavyVest, // +5 STR
    ScoutFins, // +5 AGI
    DataLens,  // +5 INT
}

impl Gear {
    pub fn name(&self) -> &'static str {
        match self {
            Gear::HeavyVest => "Heavy Vest",
            Gear::ScoutFins => "Scout Fins",
            Gear::DataLens => "Data Lens",
        }
    }

    pub fn stat_bonus(&self) -> (u32, u32, u32) {
        match self {
            Gear::HeavyVest => (5, 0, 0),
            Gear::ScoutFins => (0, 5, 0),
            Gear::DataLens => (0, 0, 5),
        }
    }
}

// ---------------------------------------------------------------------------
// Equipment — Hats (G.3)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum HatId {
    ScoutHood,
    KnightHelm,
    MageHood,
    CommanderCap,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hat {
    pub id: HatId,
    pub name: &'static str,
    pub str_bonus: u8,
    pub agi_bonus: u8,
    pub int_bonus: u8,
    pub scrap_cost: u32,
    pub unlock_node_id: usize,
}

impl Hat {
    pub fn from_id(id: &HatId) -> Hat {
        match id {
            HatId::ScoutHood => Hat {
                id: HatId::ScoutHood,
                name: "Scout Hood",
                str_bonus: 0,
                agi_bonus: 2,
                int_bonus: 0,
                scrap_cost: 50,
                unlock_node_id: 0, // Always available
            },
            HatId::KnightHelm => Hat {
                id: HatId::KnightHelm,
                name: "Knight Helm",
                str_bonus: 2,
                agi_bonus: 0,
                int_bonus: 0,
                scrap_cost: 100,
                unlock_node_id: 10, // Ember Flats
            },
            HatId::MageHood => Hat {
                id: HatId::MageHood,
                name: "Mage Hood",
                str_bonus: 0,
                agi_bonus: 0,
                int_bonus: 2,
                scrap_cost: 100,
                unlock_node_id: 12, // Tide Basin
            },
            HatId::CommanderCap => Hat {
                id: HatId::CommanderCap,
                name: "Commander Cap",
                str_bonus: 1,
                agi_bonus: 1,
                int_bonus: 1,
                scrap_cost: 250,
                unlock_node_id: 11, // Gale Ridge
            },
        }
    }

    pub fn catalog() -> Vec<Hat> {
        vec![
            Self::from_id(&HatId::ScoutHood),
            Self::from_id(&HatId::KnightHelm),
            Self::from_id(&HatId::MageHood),
            Self::from_id(&HatId::CommanderCap),
        ]
    }

    pub fn available_for(unlocked_nodes: &HashSet<usize>) -> Vec<Hat> {
        Self::catalog()
            .into_iter()
            .filter(|h| unlocked_nodes.contains(&h.unlock_node_id))
            .collect()
    }
}
