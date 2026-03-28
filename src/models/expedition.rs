// Moved from models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::mission::ResourceYield;
use crate::models::operator::Operator;
use crate::combat::{D20, D20Result, RollMode};

// ---------------------------------------------------------------------------
// Expedition — slime dispatch to the 19-node planet map (Sprint 3)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expedition {
    pub id:          Uuid,
    pub slime_ids:   Vec<Uuid>,
    pub target:      crate::world_map::ExpeditionTarget,
    pub departed_at: DateTime<Utc>,
    /// Wall-clock time when the expedition returns (departed_at + distance*2).
    pub returns_at:  DateTime<Utc>,
    pub resolved:    bool,
}

impl Expedition {
    pub fn launch(slimes: Vec<Uuid>, target: crate::world_map::ExpeditionTarget) -> Self {
        let now      = Utc::now();
        let duration = chrono::Duration::seconds((target.distance_secs * 2) as i64);
        Self {
            id:          Uuid::new_v4(),
            slime_ids:   slimes,
            target,
            departed_at: now,
            returns_at:  now + duration,
            resolved:    false,
        }
    }

    pub fn is_complete(&self) -> bool {
        Utc::now() >= self.returns_at
    }

    pub fn resolve<R: rand::Rng>(
        &self,
        squad:  &[&Operator],
        rng:    &mut R,
    ) -> ExpeditionOutcome {
        let avg_agi: u32 = if squad.is_empty() {
            0
        } else {
            squad.iter().map(|s| s.genome.base_agility).sum::<u32>() / squad.len() as u32
        };

        let coverage = (avg_agi as f64 / 20.0).min(1.0);
        let roll = D20::mission_check(coverage, self.target.danger_level, RollMode::Normal, rng);
        let report = self.generate_report(roll.nat_twenty);

        if roll.nat_twenty {
            ExpeditionOutcome::BonusHaul {
                yield_:  self.target.resource_yield.scaled(1.5),
                roll,
                report,
            }
        } else if roll.success {
            ExpeditionOutcome::Success {
                yield_:  self.target.resource_yield.clone(),
                roll,
                report,
            }
        } else if roll.nat_one {
            let victim = self.slime_ids[rng.gen_range(0..self.slime_ids.len().max(1))];
            ExpeditionOutcome::SlimeInjured {
                slime_id:      victim,
                partial_yield: self.target.resource_yield.scaled(0.25),
                roll,
                report,
            }
        } else {
            ExpeditionOutcome::Failure { roll, report }
        }
    }

    fn generate_report(&self, exceptional: bool) -> String {
        use crate::genetics::Culture;
        let flavor = match self.target.culture {
            Culture::Ember   => if exceptional { "uncovered a smouldering cache no one was meant to find" }
                                else           { "navigated the thermal vents and returned with singed margins" },
            Culture::Tide    => if exceptional { "caught the basin at low ebb and found what the water hides" }
                                else           { "read the tide schedule correctly and returned on schedule" },
            Culture::Orange  => if exceptional { "decoded the amber lattice and returned with warm-harvest surplus" }
                                else           { "traced the amber paths and came back heavy with yield" },
            Culture::Marsh   => if exceptional { "found a submerged cache beneath the root network" }
                                else           { "waded through the delta and surfaced intact" },
            Culture::Teal    => if exceptional { "located a precision cache at depth-zero and extracted cleanly" }
                                else           { "crossed the teal shelf with surgical accuracy" },
            Culture::Crystal => if exceptional { "resonated with the spire lattice and extracted pure nodes" }
                                else           { "navigated the refraction corridors without incident" },
            Culture::Gale    => if exceptional { "rode the updrafts to a ridge no survey had mapped" }
                                else           { "outpaced the storm front and made it back before it turned" },
            Culture::Tundra  => if exceptional { "broke into a preserved vault beneath the permafrost" }
                                else           { "crossed the shelf in the cold window and came back clean" },
            Culture::Frost   => if exceptional { "found the still-point beneath the ice and listened" }
                                else           { "moved through the frost zone without disturbing the ancient layer" },
            Culture::Void    => "returned. No further details.",
        };
        format!("The team reached {} and {}.", self.target.name, flavor)
    }
}

#[derive(Debug, Clone)]
pub enum ExpeditionOutcome {
    BonusHaul    { yield_: ResourceYield, roll: D20Result, report: String },
    Success      { yield_: ResourceYield, roll: D20Result, report: String },
    SlimeInjured { slime_id: Uuid, partial_yield: ResourceYield, roll: D20Result, report: String },
    Failure      {                                           roll: D20Result, report: String },
}
