use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Resource {
    /// Gel: Harvested from Slime synergy; fuels the Incubator.
    Biomass,
    /// Metal: Mined from Orbit/Dungeons; repairs the Ship.
    Scrap,
    /// Rare drops; used to "Tune" DNA during splicing.
    Reagents,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Inventory {
    pub biomass: u64,
    pub scrap: u64,
    pub reagents: u32,
}

impl Inventory {
    pub fn add(&mut self, res: Resource, amount: u64) {
        match res {
            Resource::Biomass => self.biomass = self.biomass.saturating_add(amount),
            Resource::Scrap => self.scrap = self.scrap.saturating_add(amount),
            Resource::Reagents => self.reagents = self.reagents.saturating_add(amount as u32),
        }
    }

    pub fn try_spend(&mut self, res: Resource, amount: u64) -> bool {
        match res {
            Resource::Biomass => {
                if self.biomass >= amount {
                    self.biomass -= amount;
                    true
                } else { false }
            }
            Resource::Scrap => {
                if self.scrap >= amount {
                    self.scrap -= amount;
                    true
                } else { false }
            }
            Resource::Reagents => {
                let amt32 = amount as u32;
                if self.reagents >= amt32 {
                    self.reagents -= amt32;
                    true
                } else { false }
            }
        }
    }
}
