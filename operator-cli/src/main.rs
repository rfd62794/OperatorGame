#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

/// main.rs — OPERATOR entry point.
///
/// Loads GameState → parses CLI → dispatches command → saves state.
/// All missions tick passively via `Deployment::is_complete()`.
use clap::Parser;
use operator::cli::{Cli, Commands};
use operator::genetics::{generate_random, BreedingResolver};
use operator::models::{AarOutcome, Deployment, SlimeState};
use operator::persistence::{load, save, save_path};
use operator::ui::run_gui;
use include_dir::{include_dir, Dir};

#[allow(dead_code)]
static DOCS_DIR: Dir = include_dir!("docs");

#[tokio::main]
async fn main() {
    let path = save_path();
    let mut state = match load(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("FATAL: Could not load save file: {e}");
            std::process::exit(1);
        }
    };

    // Tick injury recovery on every launch — free state cleanup.
    for op in &mut state.slimes {
        op.tick_recovery();
    }

    let cli = Cli::parse();
    let mut rng = rand::thread_rng();

    let command = cli.command.unwrap_or(Commands::Gui);

    match command {

        // Removed Commands::Hire and Commands::Roster

        // -----------------------------------------------------------------------
        Commands::Missions => {
            if state.missions.is_empty() {
                println!("No missions available.");
            } else {
                println!("=== AVAILABLE MISSIONS ({}) ===", state.missions.len());
                for m in &state.missions {
                    println!("  {m}");
                }
            }
        }

        // -----------------------------------------------------------------------
        Commands::Deploy { mission_id_prefix, operator_id_prefixes } => {
            // Resolve mission by short-ID prefix
            let mission = state
                .missions
                .iter()
                .find(|m| m.id.to_string().starts_with(&mission_id_prefix))
                .cloned();

            let Some(mission) = mission else {
                eprintln!("Mission '{}' not found.", mission_id_prefix);
                std::process::exit(1);
            };

            // Resolve operators by prefix, validate availability
            let mut operator_ids = Vec::new();
            let mut squad_display = Vec::new();

            for prefix in &operator_id_prefixes {
                let maybe = state
                    .slimes
                    .iter_mut()
                    .find(|o| o.id.to_string().starts_with(prefix.as_str()));

                match maybe {
                    None => {
                        eprintln!("Operator '{}' not found.", prefix);
                        std::process::exit(1);
                    }
                    Some(op) if !op.is_available() => {
                        eprintln!("Operator '{}' is not available: {}", op.name, op.state);
                        std::process::exit(1);
                    }
                    Some(op) => {
                        op.state = SlimeState::Deployed(mission.id);
                        operator_ids.push(op.id);
                        squad_display.push(op.name.clone());
                    }
                }
            }

            // Preview success rate before locking in
            let squad_refs: Vec<&operator::genetics::SlimeGenome> = state
                .slimes
                .iter()
                .filter(|o| operator_ids.contains(&o.id))
                .collect();
            let rate = mission.calculate_success_rate(&squad_refs);

            let deployment = Deployment::start(&mission, operator_ids);
            let eta = mission.duration_secs;
            println!(
                "DEPLOYED: [{}] → Mission: '{}' | Squad: {} | Success: {:.0}% | ETA: {}s",
                &deployment.id.to_string()[..8],
                mission.name,
                squad_display.join(", "),
                rate * 100.0,
                eta,
            );

            state.world_map.startled_level += 0.05; // ADR-015: Hoot & Holler resonance
            state.deployments.push(deployment);
        }

        // -----------------------------------------------------------------------
        Commands::Aar => {
            let completed: Vec<usize> = state
                .deployments
                .iter()
                .enumerate()
                .filter(|(_, d)| d.is_complete() && !d.resolved)
                .map(|(i, _)| i)
                .collect();

            if completed.is_empty() {
                println!("No deployments ready for AAR.");
            } else {
                for idx in completed {
                    let deployment = &mut state.deployments[idx];
                    let mission = state
                        .missions
                        .iter()
                        .find(|m| m.id == deployment.mission_id)
                        .cloned();

                    let Some(mission) = mission else { continue; };

                    let squad_ids = deployment.operator_ids.clone();
                    let squad: Vec<&operator::genetics::SlimeGenome> = state
                        .slimes
                        .iter()
                        .filter(|o| squad_ids.contains(&o.id))
                        .collect();

                    let outcome = deployment.resolve(&mission, &squad, &mut rng);
                    deployment.resolved = true;

                    println!("=== AAR: {} ===", mission.name);
                    match &outcome {
                        AarOutcome::Victory { reward } => {
                            state.bank += reward;
                            println!("  ✅ VICTORY! +${reward} | Bank: ${}", state.bank);
                            // Return squad to Idle
                            for op in state.slimes.iter_mut() {
                                if squad_ids.contains(&op.id) {
                                    op.state = SlimeState::Idle;
                                }
                            }
                        }
                        AarOutcome::Failure { injured_ids } => {
                            let recovery = mission.duration_secs * 2;
                            let recover_at = chrono::Utc::now()
                                + chrono::Duration::seconds(recovery as i64);
                            println!("  ❌ FAILURE. Operators injured for {}s.", recovery);
                            for op in state.slimes.iter_mut() {
                                if injured_ids.contains(&op.id) {
                                    println!("     ↳ {} is injured.", op.name);
                                    op.state = SlimeState::Injured(recover_at);
                                }
                            }
                        }
                        AarOutcome::CriticalFailure { killed_id } => {
                            println!("  ☠ CRITICAL FAILURE! One operator did not make it out.");
                            if let Some(pos) = state.slimes.iter().position(|o| &o.id == killed_id) {
                                println!("     ↳ {} is KIA.", state.slimes[pos].name);
                                state.slimes.remove(pos);
                            }
                        }
                    }
                }
                // Prune resolved deployments
                state.deployments.retain(|d| !d.resolved);
            }
        }

        // -----------------------------------------------------------------------
        Commands::Status => {
            println!("=== STATUS ===");
            println!("  Bank:        ${}", state.bank);
            println!("  Roster:      {} operator(s)", state.slimes.len());
            println!("  Deployments: {} active", state.deployments.len());

            for d in &state.deployments {
                let remaining = (d.completes_at - chrono::Utc::now())
                    .num_seconds()
                    .max(0);
                let mission_name = state
                    .missions
                    .iter()
                    .find(|m| m.id == d.mission_id)
                    .map(|m| m.name.as_str())
                    .unwrap_or("Unknown");
                println!(
                    "    → [{}] '{}' — ETA: {}s",
                    &d.id.to_string()[..8],
                    mission_name,
                    remaining,
                );
            }
        }

        // -----------------------------------------------------------------------
        Commands::Gui => {
            if let Err(e) = save(&state, &path) {
                eprintln!("WARNING: Could not save before launching GUI: {e}");
            }
            if let Err(e) = run_gui(state, path) {
                eprintln!("GUI error: {e}");
            }
            return;
        }

        // ── Genetics ─────────────────────────────────────────────────────────

        Commands::Slimes => {
            if state.slimes.is_empty() {
                println!("No slimes hatched yet. Use `operator hatch <name> <culture>` to start.");
            } else {
                println!("=== SLIME STABLE ({}) ===", state.slimes.len());
                for slime in &state.slimes {
                    println!("  {slime}");
                }
            }
        }

        Commands::Hatch { name, culture } => {
            let slime = generate_random(culture, &name, &mut rng);
            println!("Hatched: {slime}");
            state.slimes.push(slime);
        }

        Commands::Splice { parent_a_prefix, parent_b_prefix, offspring_name } => {
            let a_idx = state.slimes.iter().position(|s| s.id.to_string().starts_with(&parent_a_prefix));
            let b_idx = state.slimes.iter().position(|s| s.id.to_string().starts_with(&parent_b_prefix));

            let (Some(ai), Some(bi)) = (a_idx, b_idx) else {
                eprintln!("One or both parent IDs not found. Use `operator slimes` to list IDs.");
                std::process::exit(1);
            };
            if ai == bi {
                eprintln!("A slime cannot splice with itself.");
                std::process::exit(1);
            }

            let a = state.slimes[ai].clone();
            let b = state.slimes[bi].clone();

            match BreedingResolver::breed(&a, &b, &offspring_name, &mut rng) {
                Ok(child) => {
                    println!("=== SPLICE RESULT ===");
                    println!("  Parent A: {a}");
                    println!("  Parent B: {b}");
                    println!("  Offspring: {child}");
                    state.slimes.push(child);
                }
                Err(reason) => {
                    eprintln!("Splice failed: {reason}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Incubate => {
            let mut ready = Vec::new();
            
            // Retain incubating slimes that are NOT ready, collect the ready ones
            state.incubating.retain(|inc| {
                if inc.is_ready() {
                    ready.push(inc.genome.clone());
                    false
                } else {
                    true
                }
            });

            if ready.is_empty() {
                println!("No slimes are ready for harvest in the Bio-Incubator.");
            } else {
                println!("Harvested {} slime(s) from the Bio-Incubator:", ready.len());
                for g in &ready {
                    println!("  - {} ({:?} / {:?})", g.name, g.dominant_culture(), g.genetic_tier());
                }
                state.world_map.startled_level += 0.10 * ready.len() as f32; // ADR-015: Hoot & Holler resonance
                state.slimes.extend(ready);
            }
        }
    }


    // CLI path: persist state after every command.
    if let Err(e) = save(&state, &path) {
        eprintln!("WARNING: Could not save state: {e}");
    }
}


