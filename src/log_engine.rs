/// log_engine.rs — Narrative After Action Report generator.
///
/// Derives a `MissionType` from the dominant stat requirement, then
/// pulls weighted flavor text from job-appropriate template pools.
/// No hidden RNG state — caller supplies `&mut impl Rng`.
use rand::seq::SliceRandom;
use rand::Rng;

use crate::models::{AarOutcome, Mission, Operator};

// ---------------------------------------------------------------------------
// Mission flavour classification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionType {
    Assault,  // str-dominant
    Stealth,  // agi-dominant
    Cyber,    // int-dominant
    Balanced, // no clear dominant
}

impl MissionType {
    pub fn from_mission(m: &Mission) -> Self {
        let s = m.req_strength;
        let a = m.req_agility;
        let i = m.req_intelligence;

        if s > a && s > i {
            MissionType::Assault
        } else if a > s && a > i {
            MissionType::Stealth
        } else if i > s && i > a {
            MissionType::Cyber
        } else {
            MissionType::Balanced
        }
    }
}

// ---------------------------------------------------------------------------
// Template pools
// ---------------------------------------------------------------------------

const VICTORY_ASSAULT: &[&str] = &[
    "{op} breached the perimeter before the alarms could sound. Clean extraction.",
    "The door never had a chance. {op} cleared all three floors in under ninety seconds.",
    "Overwhelming force. {op} hit the objective with zero resistance left standing.",
    "The crew moved like a single fist. {op} put the first guard down before he reached the radio.",
    "They called it a fortress. {op} called it Tuesday.",
];

const VICTORY_STEALTH: &[&str] = &[
    "{op} bypassed the heat sensors with seconds to spare. No trace left behind.",
    "Every camera looped, every guard on a scheduled route. {op} vanished before dawn.",
    "The client never knew we were there. Neither did anyone else. Classic {op}.",
    "{op} cut the feed and slipped through the blind spot. Textbook infiltration.",
    "Silent approach, silent departure. The only evidence: the missing file.",
];

const VICTORY_CYBER: &[&str] = &[
    "{op} triggered a cascade failure across the entire network. Their IT team is still rebooting.",
    "The zero-day executed flawlessly. {op} had admin access before the coffee got cold.",
    "Every system, owned. {op} left a ghost signature they'll be chasing for months.",
    "Firewall, intrusion detection, biometrics — {op} walked through all of it.",
    "Forty-seven seconds from first packet to root shell. {op} set a new record.",
];

const VICTORY_BALANCED: &[&str] = &[
    "Every contingency accounted for. The squad executed perfectly.",
    "No single moment of brilliance — just relentless preparation paying off.",
    "The mission ran exactly as planned. In this line of work, that's the best-case scenario.",
    "Clean operation. The client is satisfied. The squad is intact. Good enough.",
];

const FAILURE_INJURY: &[&str] = &[
    "{op} took the hit so the others could fall back. Medics say they'll recover.",
    "The extraction went sideways. {op} caught a ricochet on the way out. Down but not out.",
    "Close — too close. {op} needs time off the line before the next contract.",
    "They almost had it. The final corridor cost {op} a week in recovery.",
    "Intel was bad. {op} walked into a prepared position. We're lucky to be debriefing at all.",
];

const CRITICAL_LINES: &[&str] = &[
    "{op} didn't make it to the extraction point. No further communication.",
    "The last transmission from {op} was static. We sent a recovery team. There was nothing to recover.",
    "{op} bought the squad time to get out. We won't use that lightly.",
    "Standard debrief protocol. {op} is listed as KIA. Their file is closed.",
    "Some contracts cost more than money. {op} paid the full price.",
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Generate a narrative string for the given AAR outcome.
/// `lead_op_name` is the first operator in the squad (used as the {op} token).
pub fn generate_narrative<R: Rng>(
    outcome: &AarOutcome,
    mission: &Mission,
    squad: &[&Operator],
    rng: &mut R,
) -> String {
    let mission_type = MissionType::from_mission(mission);
    let lead = squad
        .first()
        .map(|o| o.name.as_str())
        .unwrap_or("The squad");

    let template = match outcome {
        AarOutcome::Victory { .. } => {
            let pool = match mission_type {
                MissionType::Assault => VICTORY_ASSAULT,
                MissionType::Stealth => VICTORY_STEALTH,
                MissionType::Cyber => VICTORY_CYBER,
                MissionType::Balanced => VICTORY_BALANCED,
            };
            pool.choose(rng).copied().unwrap_or("Mission complete.")
        }
        AarOutcome::Failure { injured_ids } => {
            if injured_ids.is_empty() {
                "The mission failed. The squad retreated intact."
            } else {
                FAILURE_INJURY
                    .choose(rng)
                    .copied()
                    .unwrap_or("The mission failed. Operators are injured.")
            }
        }
        AarOutcome::CriticalFailure { .. } => CRITICAL_LINES
            .choose(rng)
            .copied()
            .unwrap_or("An operator was lost."),
    };

    template.replace("{op}", lead)
}

/// Format a full log entry with mission name, outcome header, and narrative.
pub fn format_log_entry(
    mission_name: &str,
    outcome: &AarOutcome,
    narrative: &str,
) -> String {
    let outcome_label = match outcome {
        AarOutcome::Victory { reward } => format!("✅ VICTORY (+${})", reward),
        AarOutcome::Failure { .. } => "❌ FAILURE".to_string(),
        AarOutcome::CriticalFailure { .. } => "☠ CRITICAL FAILURE".to_string(),
    };

    format!("[{}] {} — {}", mission_name, outcome_label, narrative)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    fn dummy_mission(rs: u32, ra: u32, ri: u32) -> Mission {
        crate::models::Mission::new("Test", rs, ra, ri, 0.1, 60, 100)
    }

    fn dummy_op(name: &str) -> Operator {
        crate::models::Operator::new(name, crate::models::Job::Breacher, 50, 30, 20)
    }

    #[test]
    fn test_mission_type_classification() {
        assert_eq!(MissionType::from_mission(&dummy_mission(80, 20, 10)), MissionType::Assault);
        assert_eq!(MissionType::from_mission(&dummy_mission(10, 80, 20)), MissionType::Stealth);
        assert_eq!(MissionType::from_mission(&dummy_mission(10, 20, 80)), MissionType::Cyber);
        assert_eq!(MissionType::from_mission(&dummy_mission(40, 40, 40)), MissionType::Balanced);
    }

    #[test]
    fn test_narrative_returns_non_empty() {
        let mut rng = SmallRng::seed_from_u64(7);
        let mission = dummy_mission(10, 10, 80);
        let op = dummy_op("Ghost");
        let outcome = AarOutcome::Victory { reward: 1000 };
        let result = generate_narrative(&outcome, &mission, &[&op], &mut rng);
        assert!(!result.is_empty());
        assert!(result.contains("Ghost"), "Operator name should be interpolated");
    }

    #[test]
    fn test_format_log_entry_structure() {
        let entry = format_log_entry("Bank Heist", &AarOutcome::Victory { reward: 500 }, "Great job.");
        assert!(entry.contains("Bank Heist"));
        assert!(entry.contains("VICTORY"));
        assert!(entry.contains("Great job."));
    }
}
