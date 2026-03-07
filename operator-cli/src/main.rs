#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

/// main.rs — OPERATOR entry point.
///
/// Loads GameState → parses CLI → dispatches command → saves state.
/// All missions tick passively via `Deployment::is_complete()`.
use clap::Parser;
use operator::cli::{Cli, Commands, ExpeditionAction};
use operator::genetics::{generate_random, BreedingResolver};
use operator::models::{AarOutcome, Deployment, Expedition, ExpeditionOutcome, SlimeState};
use operator::persistence::{load, save, save_path};
use operator::ui::run_gui;
use operator::world_map::{seed_expedition_targets};
use include_dir::{include_dir, Dir};

#[allow(dead_code)]
static DOCS_DIR: Dir = include_dir!("docs");

#[tokio::main]
async fn main() {
    let path = save_path();
    let mut state: operator::persistence::GameState = match load(&path) {
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
                    .find(|o| o.id().to_string().starts_with(prefix.as_str()));

                match maybe {
                    None => {
                        eprintln!("Operator '{}' not found.", prefix);
                        std::process::exit(1);
                    }
                    Some(op) if !op.is_available() => {
                        eprintln!("Operator '{}' is not available: {}", op.name(), op.state);
                        std::process::exit(1);
                    }
                    Some(op) => {
                        op.state = SlimeState::Deployed(mission.id);
                        operator_ids.push(op.id());
                        squad_display.push(op.name().to_string());
                    }
                }
            }

            // Preview success rate before locking in
            let squad_refs: Vec<&operator::models::Operator> = state
                .slimes
                .iter()
                .filter(|o| operator_ids.contains(&o.id()))
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

            // Trigger Ember Chord (Geometric frequency mapping)
            let mut freqs = Vec::new();
            for op_id in &deployment.operator_ids {
                if let Some(op) = state.slimes.iter().find(|s| s.id() == *op_id) {
                    let (s, a, i, _, _, _) = op.total_stats();
                    freqs.push(200.0 + (s as f32 * 2.0));
                    freqs.push(300.0 + (a as f32 * 2.0));
                    freqs.push(400.0 + (i as f32 * 2.0));
                }
            }
            operator::audio::OperatorSynth::play(operator::audio::PlayEvent::EmberChord { frequencies: freqs });

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
                    let squad: Vec<&operator::models::Operator> = state
                        .slimes
                        .iter()
                        .filter(|o| squad_ids.contains(&o.id()))
                        .collect();

                    let outcome = deployment.resolve(&mission, &squad, &mut rng);
                    deployment.resolved = true;

                    println!("=== AAR: {} ===", mission.name);
                    let stat_labels = ["STR", "AGI", "INT"];
                    match &outcome {
                        AarOutcome::Victory { reward, success_rate, rolls } => {
                            // Print per-stat D20 roll results
                            for (label, roll) in stat_labels.iter().zip(rolls.iter()) {
                                println!("  {} check: {}", label, roll.narrative());
                            }
                            let pass = rolls.iter().filter(|r| r.success).count();
                            println!("  Result: VICTORY ({}/3 checks passed)", pass);

                            state.bank += reward;
                            println!("  ✅ +${reward} | Bank: ${}", state.bank);
                            
                            // Play Tide Bowl (Plate Resonance) based on total Mind of squad
                            let avg_mnd: f32 = squad.iter().map(|s| s.genome.base_mind as f32).sum::<f32>() / squad.len().max(1) as f32;
                            let stability = (avg_mnd / 20.0).clamp(0.0, 1.0);
                            operator::audio::OperatorSynth::play(operator::audio::PlayEvent::TideBowl { 
                                base_freq: operator::audio::BASE_RESONANCE, 
                                stability 
                            });

                            // Return squad to Idle
                            for op in state.slimes.iter_mut() {
                                if squad_ids.contains(&op.id()) {
                                    op.state = SlimeState::Idle;
                                }
                            }
                        }
                        AarOutcome::Failure { injured_ids, rolls } => {
                            for (label, roll) in stat_labels.iter().zip(rolls.iter()) {
                                println!("  {} check: {}", label, roll.narrative());
                            }
                            let pass = rolls.iter().filter(|r| r.success).count();
                            println!("  Result: FAILURE ({}/3 checks passed)", pass);

                            let recovery = mission.duration_secs * 2;
                            let recover_at = chrono::Utc::now()
                                + chrono::Duration::seconds(recovery as i64);
                            println!("  ❌ Operators injured for {}s.", recovery);
                            
                            operator::audio::OperatorSynth::play(operator::audio::PlayEvent::Failure { base_freq: 200.0 });

                            for op in state.slimes.iter_mut() {
                                if injured_ids.contains(&op.id()) {
                                    println!("     ↳ {} is injured.", op.name());
                                    op.state = SlimeState::Injured(recover_at);
                                }
                            }
                        }
                        AarOutcome::CriticalFailure { injured_ids, rolls } => {
                            for (label, roll) in stat_labels.iter().zip(rolls.iter()) {
                                println!("  {} check: {}", label, roll.narrative());
                            }
                            println!("  Result: CRITICAL FAILURE (0/3 checks passed)");
                            println!("  ☠ One operator did not make it out.");
                            
                            operator::audio::OperatorSynth::play(operator::audio::PlayEvent::Startled { base_freq: 100.0 });

                            for &id in &injured_ids {
                                if let Some(pos) = state.slimes.iter().position(|o| o.id() == id) {
                                    println!("     ↳ {} is injured (Critical).", state.slimes[pos].name());
                                    // Set injured state if not already handled by apply_outcome_injuries logic
                                    // (Actually apply_outcome_injuries handles it, but let's be safe or just log)
                                }
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
            state.slimes.push(operator::models::Operator::new(slime));
        }

        Commands::Splice { parent_a_prefix, parent_b_prefix, offspring_name } => {
            let a_idx = state.slimes.iter().position(|s| s.id().to_string().starts_with(&parent_a_prefix));
            let b_idx = state.slimes.iter().position(|s| s.id().to_string().starts_with(&parent_b_prefix));

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
                    state.slimes.push(operator::models::Operator::new(child));
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
            state.incubating.retain(|inc: &operator::persistence::IncubatingGenome| {
                if inc.is_ready() {
                    ready.push(inc.operator.genome.clone());
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
                state.slimes.extend(ready.into_iter().map(operator::models::Operator::new));
            }
        }

        // -----------------------------------------------------------------------
        Commands::Expedition { action } => match action {

            // ── LIST ─────────────────────────────────────────────────────────
            ExpeditionAction::List => {
                let targets = seed_expedition_targets();
                println!("=== EXPEDITION TARGETS ({}) ===", targets.len());
                for t in &targets {
                    println!(
                        "  [{:?}] {:<16} | DC {:.0}% | {}s round-trip | 🌱{}  ⚙{}  🧪{}",
                        t.culture,
                        t.name,
                        t.danger_level * 100.0,
                        t.distance_secs * 2,
                        t.resource_yield.biomass,
                        t.resource_yield.scrap,
                        t.resource_yield.reagents,
                    );
                }

                let active: Vec<_> = state.active_expeditions.iter()
                    .filter(|e| !e.resolved)
                    .collect();
                if active.is_empty() {
                    println!("\nNo active expeditions.");
                } else {
                    println!("\n=== ACTIVE EXPEDITIONS ({}) ===", active.len());
                    for exp in active {
                        let remaining = (exp.returns_at - chrono::Utc::now()).num_seconds().max(0);
                        let status = if exp.is_complete() { "✅ READY" } else { &format!("ETA {}s", remaining) };
                        println!(
                            "  {} → {} | {} slime(s) | {}",
                            exp.target.name,
                            exp.target.name,
                            exp.slime_ids.len(),
                            status,
                        );
                    }
                }
            }

            // ── LAUNCH ───────────────────────────────────────────────────────
            ExpeditionAction::Launch { target_name, slime_id_prefixes } => {
                let targets = seed_expedition_targets();
                let target_name_lower = target_name.to_lowercase();
                let target = targets.into_iter()
                    .find(|t| t.name.to_lowercase().contains(&target_name_lower));

                let Some(target) = target else {
                    eprintln!("No expedition target matching '{}'. Run `operator expedition list`.", target_name);
                    std::process::exit(1);
                };

                // Resolve slime IDs by prefix
                let mut slime_ids = Vec::new();
                for prefix in &slime_id_prefixes {
                    let id = state.slimes.iter()
                        .find(|s| s.id().to_string().starts_with(prefix.as_str()))
                        .map(|s| s.id());
                    match id {
                        Some(id) => slime_ids.push(id),
                        None => {
                            eprintln!("Slime '{}' not found. Use `operator slimes` to list IDs.", prefix);
                            std::process::exit(1);
                        }
                    }
                }

                let exp = Expedition::launch(slime_ids, target.clone());
                println!(
                    "🚀 Launched {} slime(s) to {}. Returns in {}s.",
                    exp.slime_ids.len(),
                    exp.target.name,
                    target.distance_secs * 2,
                );
                state.active_expeditions.push(exp);
            }

            // ── RETURN ───────────────────────────────────────────────────────
            ExpeditionAction::Return => {
                let completed: Vec<_> = state.active_expeditions.iter()
                    .enumerate()
                    .filter(|(_, e)| !e.resolved && e.is_complete())
                    .map(|(i, _)| i)
                    .collect();

                if completed.is_empty() {
                    println!("No expeditions have returned yet.");
                } else {
                    for &idx in &completed {
                        let exp = &state.active_expeditions[idx];
                        let squad: Vec<&operator::models::Operator> = state.slimes.iter()
                            .filter(|s| exp.slime_ids.contains(&s.id()))
                            .collect();
                        let outcome = exp.resolve(&squad, &mut rng);

                        println!("\n=== EXPEDITION RETURN: {} ===", exp.target.name);

                        match &outcome {
                            ExpeditionOutcome::BonusHaul { yield_, roll, report } => {
                                println!("  AGI check: {}", roll.narrative());
                                println!();
                                println!("  \"{}\"\n", report);
                                println!("  ⚡ CRITICAL HAUL — Resources at 1.5×:");
                                println!("    🌱 Biomass  +{}", yield_.biomass);
                                println!("    ⚙️  Scrap    +{}", yield_.scrap);
                                println!("    🧪 Reagents +{}", yield_.reagents);
                                yield_.apply_to_inventory(&mut state.inventory);
                            }
                            ExpeditionOutcome::Success { yield_, roll, report } => {
                                println!("  AGI check: {}", roll.narrative());
                                println!();
                                println!("  \"{}\"\n", report);
                                println!("  Resources recovered:");
                                println!("    🌱 Biomass  +{}", yield_.biomass);
                                println!("    ⚙️  Scrap    +{}", yield_.scrap);
                                println!("    🧪 Reagents +{}", yield_.reagents);
                                yield_.apply_to_inventory(&mut state.inventory);
                            }
                            ExpeditionOutcome::SlimeInjured { slime_id, partial_yield, roll, report } => {
                                let name = state.slimes.iter()
                                    .find(|s| s.id() == *slime_id)
                                    .map(|s| s.name())
                                    .unwrap_or("Unknown");
                                println!("  AGI check: {}", roll.narrative());
                                println!();
                                println!("  \"{}\"\n", report);
                                println!("  ⚠️  {} was injured — partial return (0.25×):", name);
                                println!("    🌱 Biomass  +{}", partial_yield.biomass);
                                println!("    ⚙️  Scrap    +{}", partial_yield.scrap);
                                println!("    🧪 Reagents +{}", partial_yield.reagents);
                                // Mark slime as injured (recovery = distance_secs)
                                let recover_at = chrono::Utc::now()
                                    + chrono::Duration::seconds(exp.target.distance_secs as i64);
                                if let Some(s) = state.slimes.iter_mut().find(|s| s.id() == *slime_id) {
                                    s.state = SlimeState::Injured(recover_at);
                                }
                                partial_yield.apply_to_inventory(&mut state.inventory);
                            }
                            ExpeditionOutcome::Failure { roll, report } => {
                                println!("  AGI check: {}", roll.narrative());
                                println!();
                                println!("  \"{}\"\n", report);
                                println!("  ❌ FAILURE — no resources recovered.");
                            }
                        }

                        println!(
                            "\n  Cargo Bay: 🌱 {} | ⚙️  {} | 🧪 {}",
                            state.inventory.biomass,
                            state.inventory.scrap,
                            state.inventory.reagents,
                        );
                    }

                    // Mark resolved
                    for &idx in &completed {
                        state.active_expeditions[idx].resolved = true;
                    }
                }
            }
        }
    }

    // CLI path: persist state after every command.
    if let Err(e) = save(&state, &path) {
        eprintln!("WARNING: Could not save state: {e}");
    }
}


