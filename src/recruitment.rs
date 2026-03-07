/// recruitment.rs — The Union Recruitment Agency (Sprint 12)
///
/// Handles purchasing randomized slimes (Tier 0) to build the starting squad,
/// as well as the "Elder's Gift" anti-softlock mechanism.

use rand::Rng;

use crate::genetics::{generate_random, Culture, SlimeGenome};
use crate::persistence::GameState;

pub const STANDARD_RECRUIT_COST: u64 = 25;

/// Generate a new Tier 0 recruit.
/// Randomly selects a Primary culture (Ember, Gale, Marsh) if none specified.
pub fn generate_recruit(culture: Option<Culture>, name: &str) -> SlimeGenome {
    let mut rng = rand::thread_rng();
    let cult = culture.unwrap_or_else(|| {
        match rng.gen_range(0..3) {
            0 => Culture::Ember,
            1 => Culture::Gale,
            _ => Culture::Marsh,
        }
    });

    // Create the genome
    generate_random(cult, name, &mut rng)
}

/// Attempts to purchase a standard recruit for $25.
/// Returns Ok(genome_id) on success, Err if lacking funds.
pub fn purchase_recruit(state: &mut GameState, name: &str) -> Result<uuid::Uuid, String> {
    if state.bank < STANDARD_RECRUIT_COST as i64 {
        return Err("Insufficient credits to hire a new operator.".to_string());
    }

    state.bank -= STANDARD_RECRUIT_COST as i64;
    let genome = generate_recruit(None, name);
    let id = genome.id;
    state.slimes.push(genome);

    Ok(id)
}

/// The Anti-Softlock (ADR-034).
/// Grants a free Void-affinity genome if the roster is completely empty
/// and the player cannot afford a standard recruit.
pub fn claim_elders_gift(state: &mut GameState) -> Result<uuid::Uuid, &'static str> {
    if !state.slimes.is_empty() {
        return Err("The Elder's Gift is only available when you have no slimes.");
    }
    let cost = STANDARD_RECRUIT_COST as i64;
    if state.bank >= cost {
        return Err("You have enough funds to draft a standard recruit.");
    }

    let mut rng = rand::thread_rng();
    let genome = generate_random(Culture::Void, "Elder's Seed", &mut rng);
    let id = genome.id;
    state.slimes.push(genome);

    Ok(id)
}
