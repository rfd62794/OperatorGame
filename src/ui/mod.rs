/// ui.rs — OPERATOR War Room Dashboard (Tier 3)
///
/// Three-column egui layout:
///   [UNIT ROSTER] | [ACTIVE OPERATIONS] | [AVAILABLE CONTRACTS]
///
/// Polling fix: ctx.request_repaint_after(100ms) — progress bars animate
/// without a background thread. No decrement loop, just wall-clock math.
use std::collections::HashSet;
use std::path::PathBuf;

use chrono::{Duration, Utc};
use crate::geometry::Bounds;
// use crate::render::garden_bridge::{egui_pos_to_point, egui_rect_to_bounds};
use eframe::egui;
use uuid::Uuid;

use crate::garden::Garden;
use crate::log_engine::{format_log_entry, generate_narrative};
use crate::models::{AarOutcome, Deployment, LogEntry, LogOutcome, Mission, SlimeState};
use crate::persistence::{save, GameState};

#[cfg(test)]
mod f1b_loop_tests;

// ---------------------------------------------------------------------------
// Stitch Design System — Color Tokens
// ---------------------------------------------------------------------------

/// Background surface low (panels, active tab fills). #131318
const COLOR_SURFACE_LOW:  egui::Color32 = egui::Color32::from_rgb(19, 19, 24);
/// Surface container highest (separator lines, headers). #25252c
const COLOR_SURFACE_HIGH: egui::Color32 = egui::Color32::from_rgb(37, 37, 44);
/// Primary accent — active/success states. #69fea5
const COLOR_PRIMARY:      egui::Color32 = egui::Color32::from_rgb(105, 254, 165);
/// High-contrast text (inactive labels). #f8f5fd
const COLOR_TEXT:         egui::Color32 = egui::Color32::from_rgb(248, 245, 253);

pub mod cargo;
pub mod contracts;
pub mod manifest;
pub mod ops;
pub mod radar;
pub mod squad;

// ---------------------------------------------------------------------------
// App State
// ---------------------------------------------------------------------------

/// Summary of a resolved AAR result — stored on OperatorApp so the panel
/// persists across tab switches until the player taps DISMISS.
#[derive(Debug, Clone)]
pub struct AarSummary {
    pub mission_name: String,
    pub outcome_label: String,
    pub outcome_color: egui::Color32,
    pub xp_gained: u32,
    pub level_ups: Vec<String>,     // "{name} reached Level {n}"
    pub roll_lines: Vec<String>,    // compact per-roll summary
    pub injured_names: Vec<String>, // names of newly-injured operators
    pub reward: Option<crate::models::ResourceYield>,
}

pub struct OperatorApp {
    state: GameState,
    save_path: PathBuf,
    /// Mission currently targeted for deployment.
    selected_mission: Option<Uuid>,
    /// Operators staged for the next launch. Toggled by clicking roster cards.
    staged_operators: HashSet<Uuid>,
    /// One-line feedback shown at the bottom of the screen.
    status_msg: String,
    /// The living Shepherd's Garden background simulation.
    garden: Garden,
    /// Slime selected via clicking a roster card — opens the detail panel.
    pub selected_slime_id: Option<Uuid>,
    /// Pending AAR result displayed after PROCESS AAR until DISMISS is tapped.
    pub pending_aar: Option<AarSummary>,
    /// Which panel is active on the left: Roster (Manifest) or Incubator.
    pub left_tab: LeftTab,
    /// Which panel is active on the right: Contracts or Radar.
    pub right_tab: RightTab,
    /// Which panel is active in mobile-view (single column).
    pub mobile_tab: MobileTab,
    /// Which bottom tab is active on Android/Compact view.
    pub active_tab: crate::platform::BottomTab,
    // Sub-tab state
    pub roster_sub_tab: crate::platform::RosterSubTab,
    pub missions_sub_tab: crate::platform::MissionsSubTab,
    pub map_sub_tab: crate::platform::MapSubTab,
    pub logs_sub_tab: crate::platform::LogsSubTab,
}

#[derive(PartialEq)]
pub enum LeftTab {
    Manifest,
    Incubator,
    Recruit,
}

#[derive(PartialEq)]
pub enum RightTab {
    Contracts,
    Radar,
    Cargo,
}

#[derive(PartialEq, Clone, Copy)]
pub enum MobileTab {
    Manifest,
    Ops,
    Contracts,
    Radar,
    Cargo,
}

impl OperatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, state: GameState, save_path: PathBuf) -> Self {
        let garden = Garden::from_operators(&state.slimes, Bounds::new(0.0, 0.0, 1000.0, 1000.0));
        Self {
            active_tab: state.active_tab,
            roster_sub_tab: state.roster_sub_tab,
            missions_sub_tab: state.missions_sub_tab,
            map_sub_tab: state.map_sub_tab,
            logs_sub_tab: state.logs_sub_tab,
            state,
            save_path,
            selected_mission: None,
            staged_operators: HashSet::new(),
            status_msg: String::from("Welcome to OPERATOR. Select a contract, then stage your squad."),
            garden,
            selected_slime_id: None,
            pending_aar: None,
            left_tab: LeftTab::Manifest,
            right_tab: RightTab::Contracts,
            mobile_tab: MobileTab::Manifest,
        }
    }

    pub fn new_dummy() -> Self {
        Self::new_from_state(GameState::default(), PathBuf::from("test_save.json"))
    }

    pub fn new_from_state(state: GameState, save_path: PathBuf) -> Self {
        let garden = Garden::from_operators(&state.slimes, Bounds::new(0.0, 0.0, 1000.0, 1000.0));
        Self {
            active_tab: state.active_tab,
            roster_sub_tab: state.roster_sub_tab,
            missions_sub_tab: state.missions_sub_tab,
            map_sub_tab: state.map_sub_tab,
            logs_sub_tab: state.logs_sub_tab,
            state,
            save_path,
            selected_mission: None,
            staged_operators: HashSet::new(),
            status_msg: String::new(),
            garden,
            selected_slime_id: None,
            pending_aar: None,
            left_tab: LeftTab::Manifest,
            right_tab: RightTab::Contracts,
            mobile_tab: MobileTab::Manifest,
        }
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------


    /// Derive `started_at` from `completes_at - mission.duration_secs`.
    /// Avoids storing a redundant field on `Deployment`.
    fn progress_for(&self, dep: &Deployment) -> (f32, i64) {
        let mission = self.state.missions.iter().find(|m| m.id == dep.mission_id);
        let total_secs = mission.map(|m| m.duration_secs).unwrap_or(60) as f64;
        let started_at = dep.completes_at - Duration::seconds(total_secs as i64);
        let elapsed =
            (Utc::now() - started_at).num_milliseconds() as f64 / 1000.0;
        let progress = (elapsed / total_secs).clamp(0.0, 1.0) as f32;
        let remaining = (dep.completes_at - Utc::now()).num_seconds().max(0);
        (progress, remaining)
    }

    fn persist(&mut self) {
        // Sync UI state to GameState before saving
        self.state.active_tab = self.active_tab;
        self.state.roster_sub_tab = self.roster_sub_tab;
        self.state.missions_sub_tab = self.missions_sub_tab;
        self.state.map_sub_tab = self.map_sub_tab;
        self.state.logs_sub_tab = self.logs_sub_tab;

        if let Err(e) = save(&self.state, &self.save_path) {
            eprintln!("Save error: {e}");
        }
    }

    // -----------------------------------------------------------------------
    // Column renderers
    // -----------------------------------------------------------------------




    // -----------------------------------------------------------------------
    // Actions
    // -----------------------------------------------------------------------

    fn launch_mission(&mut self, mission: Mission) {
        // Validate and check for emergency
        let staged_ids: Vec<Uuid> = self.staged_operators.iter().cloned().collect();
        let mut is_emergency = false;
        for id in &staged_ids {
            let op = self.state.slimes.iter().find(|o| o.genome.id == *id);
            if let Some(op) = op {
                if matches!(op.state, SlimeState::Deployed(_)) {
                    self.status_msg = format!("{} is currently deployed elsewhere.", op.name());
                    return;
                }
                // If it's an Injured slime, it's an emergency deployment.
                if let SlimeState::Injured(_) = op.state {
                    is_emergency = true;
                }
            }
        }

        // Mark operators as deployed
        for op in self.state.slimes.iter_mut() {
            if staged_ids.contains(&op.genome.id) {
                op.state = SlimeState::Deployed(mission.id);
            }
        }

        let deployment = Deployment::start(&mission, staged_ids, is_emergency);
        let emergency_note = if is_emergency {
            " EMERGENCY DEPLOYMENT AUTHORIZED: Personnel operating outside approved medical clearance."
        } else { "" };

        self.status_msg = format!(
            "Deployed {} operator(s) on '{}'. ETA: {}s.{}",
            deployment.operator_ids.len(),
            mission.name,
            mission.duration_secs,
            emergency_note
        );

        self.state.world_map.startled_level += 0.05; // ADR-015: Hoot & Holler resonance
        
        // Trigger Ember Chord (Geometric frequency mapping)
        let mut freqs = Vec::new();
        for op_id in &deployment.operator_ids {
            if let Some(op) = self.state.slimes.iter().find(|s| s.genome.id == *op_id) {
                let (s, a, i, _, _, _) = op.total_stats();
                freqs.push(200.0 + (s as f32 * 2.0));
                freqs.push(300.0 + (a as f32 * 2.0));
                freqs.push(400.0 + (i as f32 * 2.0));
            }
        }
        crate::audio::OperatorSynth::play(crate::audio::PlayEvent::EmberChord { frequencies: freqs });

        self.state.deployments.push(deployment);
        self.staged_operators.clear();
        self.selected_mission = None;
        self.persist();
    }

    fn resolve_deployment(&mut self, dep_id: Uuid) {
        let dep_idx = self.state.deployments.iter().position(|d| d.id == dep_id);
        let Some(dep_idx) = dep_idx else { return; };

        let dep = self.state.deployments[dep_idx].clone();
        let mission = self
            .state
            .missions
            .iter()
            .find(|m| m.id == dep.mission_id)
            .cloned();
        let Some(mission) = mission else { return; };

        let squad: Vec<&crate::models::Operator> = self
            .state
            .slimes
            .iter()
            .filter(|o| dep.operator_ids.contains(&o.genome.id))
            .collect();

        let mut rng = rand::thread_rng();
        let mut outcome = dep.resolve(&mission, &squad, &mut rng);
        
        // Calculate stats needed for audio/UI before we drop the immutable borrow (squad)
        let avg_mnd: f32 = squad.iter().map(|s| s.genome.base_mind as f32).sum::<f32>() / squad.len().max(1) as f32;
        let mut narrative = generate_narrative(&outcome, &mission, &squad.iter().map(|o| &o.genome).collect::<Vec<_>>(), &mut rand::thread_rng());
        if dep.is_emergency {
            narrative.push_str("\nFIELD OPS PROTOCOL \u{00a7}7 ACTIVE: Personnel operating outside approved medical clearance. Deployment authorized with +15 Critical Stress Penalty.");
        }
        
        self.state.deployments[dep_idx].resolved = true;

        // Sprint 8/F.1b: Award XP to the squad; capture total and level-ups
        let mut total_xp_gained: u32 = 0;
        let mut level_ups: Vec<String> = Vec::new();
        {
            let mut mut_squad: Vec<&mut crate::models::Operator> = self
                .state
                .slimes
                .iter_mut()
                .filter(|o| dep.operator_ids.contains(&o.genome.id))
                .collect();
                
            let xp_results = dep.award_squad_xp(&mission, &mut mut_squad, &outcome);
            for (id, xp, leveled) in xp_results {
                total_xp_gained += xp;
                if leveled {
                    if let Some(op) = self.state.slimes.iter().find(|s: &&crate::models::Operator| s.genome.id == id) {
                        let msg = format!("{} has reached Level {}!", op.name(), op.level);
                        level_ups.push(msg.clone());
                        // Also push a system log entry
                        let sys_entry = LogEntry {
                            timestamp: chrono::Utc::now().timestamp() as u64,
                            message: format!(">> EXCELLENCE RECOGNIZED: {}", msg),
                            outcome: LogOutcome::System,
                        };
                        self.state.combat_log.insert(0, sys_entry);
                    }
                }
            }
        }

        // Stamp xp_gained on the outcome before injury resolution
        match &mut outcome {
            AarOutcome::Victory { ref mut xp_gained, .. } => *xp_gained = total_xp_gained,
            AarOutcome::Failure { ref mut xp_gained, .. } => *xp_gained = total_xp_gained,
            AarOutcome::CriticalFailure { ref mut xp_gained, .. } => *xp_gained = total_xp_gained,
        }

        // Phase A: Apply injuries (probabilistic)
        let newly_injured = crate::models::apply_outcome_injuries(
            &mut outcome,
            &mut self.state.slimes,
            &dep.operator_ids,
            &mut rand::thread_rng(),
        );
        let newly_injured_ids: Vec<Uuid> = newly_injured.iter().map(|(id, _)| *id).collect();

        // Build the AAR summary for the result panel
        let (outcome_label, outcome_color, log_outcome) = match &outcome {
            AarOutcome::Victory { reward, .. } => (
                format!("VICTORY (+${})", reward),
                egui::Color32::from_rgb(80, 200, 120),
                LogOutcome::Victory,
            ),
            AarOutcome::CriticalFailure { .. } => (
                "CRITICAL FAILURE".to_string(),
                egui::Color32::from_rgb(220, 80, 80),
                LogOutcome::CritFail,
            ),
            AarOutcome::Failure { .. } => (
                "FAILURE".to_string(),
                egui::Color32::from_rgb(220, 180, 80),
                LogOutcome::Failure,
            ),
        };

        // Build roll summary lines
        let roll_lines: Vec<String> = {
            let rolls = match &outcome {
                AarOutcome::Victory { rolls, .. } => rolls,
                AarOutcome::Failure { rolls, .. } => rolls,
                AarOutcome::CriticalFailure { rolls, .. } => rolls,
            };
            let labels = ["STR", "AGI", "INT"];
            rolls.iter().enumerate().map(|(i, r)| {
                let label = labels.get(i).copied().unwrap_or("?");
                let result = if r.success { "HIT" } else { "MISS" };
                let nat = if r.nat_twenty { " (NAT20!)" } else if r.nat_one { " (NAT1)" } else { "" };
                format!("{} {:?}: {} d={}{}", label, r.rolls, result, r.dc, nat)
            }).collect()
        };

        // Injured operator names for the result panel
        let injured_names: Vec<String> = newly_injured.iter()
            .filter_map(|(id, _)| self.state.slimes.iter().find(|s| s.genome.id == *id))
            .map(|op: &crate::models::Operator| op.name().to_string())
            .collect();

        // Push narrative log entry to GameState (persisted)
        let log_message = format_log_entry(&mission.name, &outcome, &narrative);
        let log_entry = LogEntry {
            timestamp: chrono::Utc::now().timestamp() as u64,
            message: log_message,
            outcome: log_outcome,
        };
        self.state.combat_log.insert(0, log_entry);
        if self.state.combat_log.len() > 50 { self.state.combat_log.truncate(50); }

        // Store the pending AAR for display
        self.pending_aar = Some(AarSummary {
            mission_name: mission.name.clone(),
            outcome_label: outcome_label.clone(),
            outcome_color,
            xp_gained: total_xp_gained,
            level_ups,
            roll_lines,
            injured_names,
            reward: if let AarOutcome::Victory { reward, .. } = &outcome { Some(reward.clone()) } else { None },
        });

        match &outcome {
            AarOutcome::Victory { reward, .. } => {
                self.state.bank += reward.scrap as i64;
                reward.apply_to_inventory(&mut self.state.inventory);

                // Task C.2: Node unlock on scout completion
                if mission.is_scout {
                    if let Some(node_id) = mission.node_id {
                        self.state.unlocked_nodes.insert(node_id);
                        
                        let culture_name = format!("{:?}", mission.affinity.unwrap_or(crate::genetics::Culture::Void));
                        self.state.combat_log.insert(0, LogEntry {
                            timestamp: chrono::Utc::now().timestamp() as u64,
                            message: format!("Zone unlocked: {} territory now accessible", culture_name),
                            outcome: LogOutcome::System,
                        });
                    }
                }
                
                let debt_warning = if self.state.bank < 0 { 
                    "\nNOTE: Current operational balance is negative. Deployment authorized under Emergency Continuity Protocol \u{00a7}4.2."
                } else { "" };

                self.status_msg = format!("\u{2705} '{}' \u{2014} VICTORY (+{}).{}", mission.name, reward, debt_warning);
                
                // Play Tide Bowl (Plate Resonance) based on pre-calculated Mind
                let stability = (avg_mnd / 20.0).clamp(0.0, 1.0);
                crate::audio::OperatorSynth::play(crate::audio::PlayEvent::TideBowl { 
                    base_freq: crate::audio::BASE_RESONANCE, 
                    stability 
                });

                for op in self.state.slimes.iter_mut() {
                    if dep.operator_ids.contains(&op.id()) && !newly_injured_ids.contains(&op.id()) {
                        op.state = SlimeState::Idle;
                    }
                }
            }
            AarOutcome::Failure { .. } | AarOutcome::CriticalFailure { .. } => {
                let is_crit = matches!(outcome, AarOutcome::CriticalFailure { .. });
                let symbol = if is_crit { "\u{2620}" } else { "\u{274c}" };
                let label = if is_crit { "CRITICAL FAILURE" } else { "FAILURE" };

                if !newly_injured.is_empty() {
                    let (id, until) = newly_injured[0];
                    let op = self.state.slimes.iter().find(|s| s.genome.id == id);
                    let name = op.map(|s: &crate::models::Operator| s.name()).unwrap_or("Operator");
                    
                    let remaining = until - Utc::now();
                    let h = remaining.num_hours();
                    let m = remaining.num_minutes() % 60;
                    
                    self.status_msg = format!("{} '{}' \u{2014} {}. INCIDENT REPORT: {} sustained injuries. Medical leave approved. RTD estimated {}h {}m.", 
                        symbol, mission.name, label, name, h, m);
                } else {
                    self.status_msg = format!("{} '{}' \u{2014} {}. The squad retreated intact.", symbol, mission.name, label);
                }
                
                let audio_event = if is_crit {
                    crate::audio::PlayEvent::Startled { base_freq: 100.0 }
                } else {
                    crate::audio::PlayEvent::Failure { base_freq: 200.0 }
                };
                crate::audio::OperatorSynth::play(audio_event);
                
                for op in self.state.slimes.iter_mut() {
                    if dep.operator_ids.contains(&op.id()) && !newly_injured_ids.contains(&op.id()) {
                        op.state = SlimeState::Idle;
                    }
                }
            }
        }

        self.persist();
    }
}

// ---------------------------------------------------------------------------
// eframe::App implementation
// ---------------------------------------------------------------------------

impl eframe::App for OperatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let safe_area = crate::platform::read_window_insets();

        // Redraw every 100ms — animates progress bars without a background thread.
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        // Make panels completely opaque to prevent the "dim overlay" feel
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.panel_fill = egui::Color32::from_rgb(15, 15, 20); // Solid dark blue/grey
        style.visuals.window_fill = egui::Color32::from_rgb(15, 15, 20);
        style.visuals.override_text_color = Some(egui::Color32::WHITE);
        ctx.set_style(style);

        // Responsive DPI Scaling (ADR-041)
        if cfg!(target_os = "android") {
            ctx.set_pixels_per_point(2.0); // Mobile default density
        }

        // Phase C: Tick operator recovery & clearance notifications
        let mut cleared_names = Vec::new();
        for op in self.state.slimes.iter_mut() {
            if let Some(name) = op.tick_recovery() {
                cleared_names.push(name);
            }
        }
        for name in cleared_names {
            let msg = format!("{} has been cleared for deployment by Medical.", name);
            let entry = LogEntry {
                timestamp: chrono::Utc::now().timestamp() as u64,
                message: msg.clone(),
                outcome: LogOutcome::System,
            };
            self.state.combat_log.insert(0, entry);
            self.status_msg = format!("{} cleared for duty.", name);
        }

        // Sprint 7B: Tick daily upkeep
        let (deducted, idle_count) = self.state.apply_daily_upkeep(Utc::now());
        if deducted > 0 {
            let msg = format!("Deducted ${} in maintenance costs for {} idle operator(s).", deducted, idle_count);
            let entry = LogEntry {
                timestamp: chrono::Utc::now().timestamp() as u64,
                message: msg,
                outcome: LogOutcome::System,
            };
            self.state.combat_log.insert(0, entry);
            self.persist();
        }

        // Sprint 8: Refresh mission pool
        if self.state.refresh_missions_if_needed(Utc::now()) {
            let msg = format!("MISSION POOL REFRESHED: New contracts available for {} UTC.", Utc::now().date_naive());
            let entry = LogEntry {
                timestamp: chrono::Utc::now().timestamp() as u64,
                message: msg,
                outcome: LogOutcome::System,
            };
            self.state.combat_log.insert(0, entry);
            self.persist();
        }

        /*
        // Background Garden (Temporarily disabled due to UI layering issues)
        let t = ctx.input(|i| i.time as f32);
        let dt = ctx.input(|i| i.stable_dt).min(0.1);
        let cursor = ctx.input(|i| i.pointer.hover_pos()).map(egui_pos_to_point);
        let screen_rect = ctx.screen_rect();

        // Advance garden simulation
        self.garden.tick(dt, cursor, egui_rect_to_bounds(screen_rect));

        // Intercept clicks in empty space for selecting garden slimes
        if ctx.input(|i| i.pointer.primary_clicked()) && !ctx.wants_pointer_input() {
            if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                if let Some(id) = self.garden.handle_click(egui_pos_to_point(pos)) {
                    self.selected_slime_id = Some(id);
                    // Switch to Manifest to show the card
                    self.left_tab = LeftTab::Manifest;
                } else {
                    self.selected_slime_id = None;
                }
            }
        }

        // Draw garden layer beneath UI
        egui::Area::new(egui::Id::new("garden_bg"))
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                let operator_map = self.state.slimes.iter().map(|op| (op.id(), op)).collect();
                crate::garden::draw_garden(ui.painter(), screen_rect, &operator_map, &self.garden, t);
            });
        */

        // Top status bar
        egui::TopBottomPanel::top("top_bar")
            .frame(
                egui::Frame::none()
                    .inner_margin(egui::Margin {
                        left: safe_area.left,
                        right: safe_area.right,
                        top: safe_area.top,
                        bottom: 0.0,
                    })
            )
            .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("OPERATOR: COMMAND DECK")
                        .strong()
                        .size(16.0),
                );
                ui.separator();
                ui.separator();
                let idle_count = self.state.slimes.iter()
                    .filter(|s| matches!(s.state, crate::models::SlimeState::Idle))
                    .count() as i64;
                let forecast = idle_count * crate::persistence::UPKEEP_PER_DAY;
                ui.vertical(|ui| {
                    ui.label(format!("Bank: ${}", self.state.bank));
                    if forecast > 0 {
                        ui.label(egui::RichText::new(format!("Est. Upkeep: -${}/day", forecast))
                            .small()
                            .color(egui::Color32::from_gray(140)));
                    }
                });
                ui.separator();
                ui.label(format!("GEL: {}L", self.state.inventory.biomass));
                ui.separator();
                ui.label(format!("MTL: {}kg", self.state.inventory.scrap));
                ui.separator();
                ui.label(format!("Reagents: {}", self.state.inventory.reagents));
                ui.separator();
                
                // Stress Bar
                let stress_pct = (self.state.world_map.startled_level / 10.0).clamp(0.0, 1.0);
                ui.add_space(16.0);
                ui.label(egui::RichText::new("RESONANCE STRESS:").color(egui::Color32::YELLOW));
                ui.add(egui::ProgressBar::new(stress_pct)
                    .fill(egui::Color32::from_rgb(200, 50, 50))
                    .desired_width(120.0)
                    .show_percentage()
                );
            });
        });

        // 1. Launch Bar (Outer-most bottom)
        egui::TopBottomPanel::bottom("bottom_bar")
            .frame(
                egui::Frame::none()
                    .inner_margin(egui::Margin {
                        left: safe_area.left,
                        right: safe_area.right,
                        top: 0.0,
                        bottom: safe_area.bottom,
                    })
            )
            .show(ctx, |ui| {
                self.render_launch_bar(ui);
            });

        // 2. Navigation Tab Bar — Stitch Design (Outer-middle)
        egui::TopBottomPanel::bottom("bottom_tabs")
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(19, 19, 24)) // Forced opaque for layering
                    .inner_margin(egui::Margin {
                        left: safe_area.left,
                        right: safe_area.right,
                        top: 0.0,
                        bottom: 0.0,
                    })
            )
            .show(ctx, |ui| {
                ui.set_height(crate::platform::TAB_BAR_HEIGHT);
                
                // Top border line separating the bar from content
                let (rect, _) = ui.allocate_at_least(ui.available_size(), egui::Sense::hover());
                ui.painter().hline(
                    rect.min.x..=rect.max.x,
                    rect.min.y,
                    egui::Stroke::new(1.0, COLOR_SURFACE_HIGH),
                );

                let tabs = [
                    (crate::platform::BottomTab::Roster,   "🧬", "Roster"),
                    (crate::platform::BottomTab::Missions, "🚀", "Missions"),
                    (crate::platform::BottomTab::Map,      "🗺️", "Map"),
                    (crate::platform::BottomTab::Logs,     "📜", "Logs"),
                ];

                let tab_w = rect.width() / tabs.len() as f32;
                let tab_h = crate::platform::TAB_BAR_HEIGHT;

                for (i, (tab, icon, label)) in tabs.iter().enumerate() {
                    let is_active = self.active_tab == *tab;
                    let slot_rect = egui::Rect::from_min_size(
                        egui::pos2(rect.min.x + i as f32 * tab_w, rect.min.y),
                        egui::vec2(tab_w, tab_h)
                    );

                    if is_active {
                        ui.painter().rect_filled(slot_rect, egui::Rounding::ZERO, egui::Color32::from_rgb(45, 55, 75));
                        let accent_rect = egui::Rect::from_min_size(
                            egui::pos2(slot_rect.min.x, slot_rect.max.y - 4.0),
                            egui::vec2(tab_w, 4.0),
                        );
                        ui.painter().rect_filled(accent_rect, egui::Rounding::ZERO, COLOR_PRIMARY);
                    }

                    ui.allocate_ui_at_rect(slot_rect, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(6.0);
                            ui.label(egui::RichText::new(*icon).size(18.0).color(egui::Color32::WHITE));
                            ui.add_space(2.0);
                            ui.label(egui::RichText::new(*label).size(10.0).color(if is_active { COLOR_PRIMARY } else { COLOR_TEXT }));
                        });
                    });

                    let click_resp = ui.interact(slot_rect, egui::Id::new(format!("tab_{}", label)), egui::Sense::click());
                    if click_resp.clicked() { self.active_tab = *tab; }
                }
            });

        // 3. Last Action Log (Inner-most bottom / sits above tabs)
        if self.active_tab == crate::platform::BottomTab::Missions {
            if self.pending_aar.is_none() && !self.state.combat_log.is_empty() {
                egui::TopBottomPanel::bottom("combat_log_panel")
                    .resizable(true)
                    .min_height(40.0)
                    .max_height(120.0)
                    .frame(
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(20, 20, 25)) // Forced opaque
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                    )
                    .show(ctx, |ui| {
                        ui.label(egui::RichText::new("── LAST ACTION ──").strong());
                        if let Some(entry) = self.state.combat_log.first() {
                            let color = match entry.outcome {
                                LogOutcome::Victory  => egui::Color32::from_rgb(80, 200, 120),
                                LogOutcome::CritFail => egui::Color32::from_rgb(220, 80, 80),
                                LogOutcome::Failure  => egui::Color32::from_rgb(220, 180, 80),
                                LogOutcome::System   => egui::Color32::from_rgb(160, 160, 180),
                            };
                            ui.colored_label(color, &entry.message);
                        }
                    });
            }
        }

        // 4. Sidebar navigation (Sub-tabs)
        egui::SidePanel::left("left_sidebar")
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(19, 19, 24)) // Consistent opaque surface
                    .inner_margin(egui::Margin {
                        left: safe_area.left,
                        right: 0.0,
                        top: 0.0,
                        bottom: 0.0,
                    })
            )
            .resizable(false)
            .default_width(if ctx.screen_rect().width() < 450.0 { 80.0 } else { 100.0 })
            .show(ctx, |ui| {
                ui.add_space(8.0);
                render_sub_tabs(ui, self.active_tab, self);
            });

        // 5. Main Content Area (CentralPanel fills the remaining gap perfectly)
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .inner_margin(egui::Margin {
                        left: 4.0, // Breathing room after separator
                        right: safe_area.right,
                        top: 0.0,
                        bottom: 0.0,
                    })
            )
            .show(ctx, |ui| {
                // Map tab handles its own positioning
                if self.active_tab == crate::platform::BottomTab::Map {
                    match self.map_sub_tab {
                        crate::platform::MapSubTab::Zones => {
                            self.render_radar(ui);
                        }
                    }
                } else {
                    // Unified 4-tab content area
                    match self.active_tab {
                        crate::platform::BottomTab::Roster => match self.roster_sub_tab {
                            crate::platform::RosterSubTab::Collection => {
                                // If a slime is selected, show detail view; otherwise card grid
                                if self.selected_slime_id.is_some() {
                                    self.render_slime_detail(ui);
                                } else {
                                    // Force Roster (internal ScrollArea) to fill available gap
                                    ui.allocate_ui_at_rect(ui.available_rect_before_wrap(), |ui| {
                                        self.render_manifest(ui);
                                    });
                                }
                            }
                            crate::platform::RosterSubTab::Breeding => {
                                self.render_incubator(ui);
                            }
                            crate::platform::RosterSubTab::Recruit => {
                                self.render_recruit(ui);
                            }
                            crate::platform::RosterSubTab::Squad => {
                                self.render_squad(ui);
                            }
                        },
                        crate::platform::BottomTab::Missions => match self.missions_sub_tab {
                            crate::platform::MissionsSubTab::Active => {
                                self.render_active_ops(ui);
                            }
                            crate::platform::MissionsSubTab::QuestBoard => {
                                self.render_contracts(ui);
                            }
                        },
                        crate::platform::BottomTab::Logs => match self.logs_sub_tab {
                            crate::platform::LogsSubTab::MissionHistory => {
                                egui::ScrollArea::vertical()
                                    .id_source("logs_scroll")
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        if self.state.combat_log.is_empty() {
                                            ui.label(egui::RichText::new("No mission history. Deploy your first squad to begin.").color(egui::Color32::GRAY).italics());
                                        } else {
                                            for entry in &self.state.combat_log {
                                                let color = match entry.outcome {
                                                    LogOutcome::Victory  => egui::Color32::from_rgb(100, 220, 100),
                                                    LogOutcome::CritFail => egui::Color32::from_rgb(220, 80, 80),
                                                    LogOutcome::Failure  => egui::Color32::from_rgb(220, 180, 80),
                                                    LogOutcome::System   => egui::Color32::from_rgb(160, 160, 180),
                                                };
                                                ui.colored_label(color, &entry.message);
                                            }
                                        }
                                    });
                            }
                            crate::platform::LogsSubTab::CultureHistory => {
                                ui.label(egui::RichText::new("Awaiting deployment and culture synchronization...").italics().color(egui::Color32::GRAY));
                            }
                        },
                        _ => {}
                    }
                }
            });
    }
}

// ---------------------------------------------------------------------------
// Free-standing column render helpers (work around borrow-checker in columns)
// ---------------------------------------------------------------------------


/// Render a single styled sub-tab button for the sidebar.
///
/// - **Active:** dark surface fill (#131318) + primary green text (#69fea5)
/// - **Inactive:** transparent fill + high-contrast white text (#f8f5fd)
/// - Minimum size: 70×40dp (44dp touch target)
/// - Sharp corners, no stroke (Stitch design system)
fn sub_tab_button(ui: &mut egui::Ui, label: &str, is_active: bool) -> bool {
    let text_color = if is_active { COLOR_PRIMARY } else { COLOR_TEXT };
    let fill_color = if is_active { COLOR_SURFACE_LOW } else { egui::Color32::TRANSPARENT };

    let btn = egui::Button::new(
        egui::RichText::new(label)
            .size(11.0)
            .color(text_color),
    )
    .fill(fill_color)
    .stroke(egui::Stroke::NONE)
    .rounding(egui::Rounding::ZERO)
    .min_size(egui::vec2(70.0, 40.0));

    ui.add(btn).clicked()
}

/// Render a styled section header for the sidebar.
///
/// All-caps bold label in primary green, preceded by spacing and
/// followed by a thin surface-high separator line.
fn sidebar_header(ui: &mut egui::Ui, title: &str) {
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(title)
            .size(13.0)
            .color(COLOR_PRIMARY)
            .strong(),
    );
    // Thin colored separator line via the painter
    let sep_rect = ui.available_rect_before_wrap();
    ui.painter().hline(
        sep_rect.min.x..=sep_rect.max.x,
        sep_rect.min.y,
        egui::Stroke::new(1.0, COLOR_SURFACE_HIGH),
    );
    ui.add_space(6.0);
}

fn render_sub_tabs(
    ui: &mut egui::Ui,
    active_main_tab: crate::platform::BottomTab,
    app: &mut OperatorApp,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 4.0); // 4dp gap between buttons

        match active_main_tab {
            crate::platform::BottomTab::Roster => {
                sidebar_header(ui, "Roster");

                if sub_tab_button(
                    ui,
                    "Collection",
                    app.roster_sub_tab == crate::platform::RosterSubTab::Collection,
                ) {
                    app.roster_sub_tab = crate::platform::RosterSubTab::Collection;
                }

                if sub_tab_button(
                    ui,
                    "Breeding",
                    app.roster_sub_tab == crate::platform::RosterSubTab::Breeding,
                ) {
                    app.roster_sub_tab = crate::platform::RosterSubTab::Breeding;
                }

                if sub_tab_button(
                    ui,
                    "Recruit",
                    app.roster_sub_tab == crate::platform::RosterSubTab::Recruit,
                ) {
                    app.roster_sub_tab = crate::platform::RosterSubTab::Recruit;
                }

                if sub_tab_button(
                    ui,
                    "Squad",
                    app.roster_sub_tab == crate::platform::RosterSubTab::Squad,
                ) {
                    app.roster_sub_tab = crate::platform::RosterSubTab::Squad;
                }
            }

            crate::platform::BottomTab::Missions => {
                sidebar_header(ui, "Missions");

                if sub_tab_button(
                    ui,
                    "Active",
                    app.missions_sub_tab == crate::platform::MissionsSubTab::Active,
                ) {
                    app.missions_sub_tab = crate::platform::MissionsSubTab::Active;
                }

                // Abbreviated: "Quest Board" → "Quests" to fit 80–100dp column
                if sub_tab_button(
                    ui,
                    "Quests",
                    app.missions_sub_tab == crate::platform::MissionsSubTab::QuestBoard,
                ) {
                    app.missions_sub_tab = crate::platform::MissionsSubTab::QuestBoard;
                }
            }

            crate::platform::BottomTab::Map => {
                sidebar_header(ui, "Map");

                if sub_tab_button(
                    ui,
                    "Zones",
                    app.map_sub_tab == crate::platform::MapSubTab::Zones,
                ) {
                    app.map_sub_tab = crate::platform::MapSubTab::Zones;
                }
            }

            crate::platform::BottomTab::Logs => {
                sidebar_header(ui, "LOGS");

                // Abbreviated: "Mission History" → "Missions" to fit 80–100dp column
                if sub_tab_button(
                    ui,
                    "Missions",
                    app.logs_sub_tab == crate::platform::LogsSubTab::MissionHistory,
                ) {
                    app.logs_sub_tab = crate::platform::LogsSubTab::MissionHistory;
                }

                // Abbreviated: "Culture History" → "Culture" to fit 80–100dp column
                if sub_tab_button(
                    ui,
                    "Culture",
                    app.logs_sub_tab == crate::platform::LogsSubTab::CultureHistory,
                ) {
                    app.logs_sub_tab = crate::platform::LogsSubTab::CultureHistory;
                }
            }
        }
    });
}


// ---------------------------------------------------------------------------
// Entry point helper called from main.rs
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
pub fn run_gui(state: GameState, save_path: std::path::PathBuf) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("OPERATOR: War Room")
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "OPERATOR: War Room",
        options,
        Box::new(move |cc| Box::new(OperatorApp::new(cc, state, save_path))),
    )
}
