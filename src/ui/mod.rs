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
pub mod quartermaster;
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
    pub level_ups: Vec<String>,              // "{name} reached Level {n}" (combat log)
    pub level_up_events: Vec<crate::persistence::LevelUpEvent>, // G.6 rich events for FIELD PROMOTIONS
    pub roll_lines: Vec<String>,             // compact per-roll summary
    pub injured_names: Vec<String>,          // names of newly-injured operators
    pub reward: Option<crate::models::ResourceYield>,
    pub targets_defeated: usize,
    pub total_targets: usize,
    pub operator_ids: Vec<Uuid>,
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
    /// Non-persisted state for UI feedback: (node_id, start_time)
    pub recently_unlocked_node: Option<(usize, f64)>,
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
            recently_unlocked_node: None,
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
            recently_unlocked_node: None,
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
        if staged_ids.is_empty() {
             return;
        }
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
        let mut rng = rand::thread_rng();
        
        // Use the new centralized resolution logic in GameState (Bug Fix / Refactor)
        let (dep, outcome, level_ups, level_up_events) = match self.state.resolve_deployment(dep_id, &mut rng) {
            Ok(res) => res,
            Err(e) => {
                self.status_msg = format!("\u{26a0} Error: {}", e);
                return;
            }
        };

        let mission = self.state.missions.iter().find(|m| m.id == dep.mission_id)
            .expect("Mission in deployment not found in state");

        // UI Layer: Narrative Generation
        let squad: Vec<&crate::models::Operator> = self.state.slimes.iter()
            .filter(|o| dep.operator_ids.contains(&o.genome.id))
            .collect();
            
        let mut narrative = generate_narrative(&outcome, &mission, &squad.iter().map(|o| &o.genome).collect::<Vec<_>>(), &mut rng);
        if dep.is_emergency {
            narrative.push_str("\nFIELD OPS PROTOCOL \u{00a7}7 ACTIVE: Personnel operating outside approved medical clearance. Deployment authorized with +15 Critical Stress Penalty.");
        }

        // UI Layer: System Logs for Level Ups
        for msg in &level_ups {
            self.state.combat_log.insert(0, LogEntry {
                timestamp: chrono::Utc::now().timestamp() as u64,
                message: format!(">> EXCELLENCE RECOGNIZED: {}", msg),
                outcome: LogOutcome::System,
            });
        }

        // UI Layer: Visual outcome mapping
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

        // UI Layer: Roll Summary
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

        // UI Layer: Injury Summary
        let injured_names: Vec<String> = match &outcome {
            AarOutcome::Failure { injured_ids, .. } | AarOutcome::CriticalFailure { injured_ids, .. } => {
                injured_ids.iter()
                    .filter_map(|id| self.state.slimes.iter().find(|s| s.genome.id == *id))
                    .map(|op| op.name().to_string())
                    .collect()
            }
            _ => vec![],
        };

        // UI Layer: Operational History Log
        let log_message = format_log_entry(&mission.name, &outcome, &narrative);
        self.state.combat_log.insert(0, LogEntry {
            timestamp: chrono::Utc::now().timestamp() as u64,
            message: log_message,
            outcome: log_outcome,
        });
        if self.state.combat_log.len() > 50 { self.state.combat_log.truncate(50); }

        // Store the pending AAR for display
        let (defeated, total) = match outcome {
            AarOutcome::Victory { targets_defeated, total_targets, .. } => (targets_defeated, total_targets),
            AarOutcome::Failure { targets_defeated, total_targets, .. } => (targets_defeated, total_targets),
            AarOutcome::CriticalFailure { targets_defeated, total_targets, .. } => (targets_defeated, total_targets),
        };

        self.pending_aar = Some(AarSummary {
            mission_name: mission.name.clone(),
            outcome_label,
            outcome_color,
            xp_gained: match outcome {
                AarOutcome::Victory { xp_gained, .. } => xp_gained,
                AarOutcome::Failure { xp_gained, .. } => xp_gained,
                AarOutcome::CriticalFailure { xp_gained, .. } => xp_gained,
            },
            level_ups,
            level_up_events,  // G.6 FIELD PROMOTIONS data
            roll_lines,
            injured_names,
            reward: if let AarOutcome::Victory { reward, .. } = &outcome { Some(reward.clone()) } else { None },
            targets_defeated: defeated,
            total_targets: total,
            operator_ids: dep.operator_ids.clone(),
        });

        self.persist();
    }

    pub fn apply_aar_outcome(&mut self, current_time: f64) {
        let aar = self.pending_aar.as_ref().expect("Apply called without pending AAR");
        let mission_id = self.selected_mission.expect("No selected mission");
        let mission = self.state.missions.iter().find(|m| m.id == mission_id).expect("Mission not found");
        
        let outcome = if aar.outcome_label.contains("VICTORY") {
            AarOutcome::Victory {
                reward: aar.reward.clone().unwrap_or_default(),
                success_chance: 1.0, 
                rolls: vec![],
                xp_gained: aar.xp_gained,
                targets_defeated: aar.targets_defeated,
                total_targets: aar.total_targets,
            }
        } else if aar.outcome_label.contains("CRITICAL") {
            AarOutcome::CriticalFailure { 
                rolls: vec![], 
                injured_ids: vec![], 
                xp_gained: 0,
                targets_defeated: aar.targets_defeated,
                total_targets: aar.total_targets,
            }
        } else {
            AarOutcome::Failure { 
                rolls: vec![], 
                injured_ids: vec![], 
                xp_gained: 0,
                targets_defeated: aar.targets_defeated,
                total_targets: aar.total_targets,
            }
        };

        match &outcome {
            AarOutcome::Victory { reward, .. } => {
                self.state.bank += reward.scrap as i64;
                reward.apply_to_inventory(&mut self.state.inventory);

                // Task C.2: Node unlock on scout completion
                if mission.is_scout {
                    if let Some(node_id) = mission.node_id {
                        self.state.unlocked_nodes.insert(node_id);
                        self.recently_unlocked_node = Some((node_id, current_time)); // Pulse starts NOW
                        
                        let node_name = self.state.world_map.nodes.iter()
                            .find(|n| n.id as usize == node_id)
                            .map(|n| n.name.as_str())
                            .unwrap_or("Unknown Node");
                            
                        let culture_name = format!("{:?}", mission.affinity.unwrap_or(crate::genetics::Culture::Void));
                        self.state.combat_log.insert(0, LogEntry {
                            timestamp: chrono::Utc::now().timestamp() as u64,
                            message: format!("Zone unlocked: {} — {} territory now accessible", node_name, culture_name),
                            outcome: LogOutcome::System,
                        });
                    }
                }
                
                let debt_warning = if self.state.bank < 0 { 
                    "\nNOTE: Current operational balance is negative. Deployment authorized under Emergency Continuity Protocol \u{00a7}4.2."
                } else { "" };

                self.status_msg = format!("\u{2705} '{}' \u{2014} VICTORY (+{}).{}", mission.name, reward, debt_warning);
                
                // Play Tide Bowl based on squad Mind
                let squad: Vec<&crate::models::Operator> = self.state.slimes.iter()
                    .filter(|op| aar.operator_ids.contains(&op.genome.id))
                    .collect();
                let avg_mnd: f32 = if squad.is_empty() { 10.0 } else { 
                    squad.iter().map(|op| op.total_stats().2 as f32).sum::<f32>() / squad.len() as f32 
                };

                let stability = (avg_mnd / 20.0).clamp(0.0, 1.0);
                crate::audio::OperatorSynth::play(crate::audio::PlayEvent::TideBowl { 
                    base_freq: crate::audio::BASE_RESONANCE, 
                    stability 
                });
            }
            AarOutcome::Failure { .. } | AarOutcome::CriticalFailure { .. } => {
                let is_crit = matches!(outcome, AarOutcome::CriticalFailure { .. });
                let symbol = if is_crit { "\u{2620}" } else { "\u{274c}" };
                let label = if is_crit { "CRITICAL FAILURE" } else { "FAILURE" };

                self.status_msg = format!("{} '{}' \u{2014} {}. The squad retreated.", symbol, mission.name, label);
                
                let audio_event = if is_crit {
                    crate::audio::PlayEvent::Startled { base_freq: 100.0 }
                } else {
                    crate::audio::PlayEvent::Failure { base_freq: 200.0 }
                };
                crate::audio::OperatorSynth::play(audio_event);
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

        // Redraw every 100ms
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        // Styling
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.panel_fill = egui::Color32::from_rgb(15, 15, 20);
        style.visuals.window_fill = egui::Color32::from_rgb(15, 15, 20);
        style.visuals.override_text_color = Some(egui::Color32::WHITE);
        ctx.set_style(style);

        let layout = crate::platform::ResponsiveLayout::from_width(ctx.screen_rect().width());
        crate::platform::apply_interaction_scale(ctx, layout);

        if cfg!(target_os = "android") || crate::platform::is_mobile_emu() {
            ctx.set_pixels_per_point(2.0);
        }

        // Ticks
        let mut cleared_names = Vec::new();
        for op in self.state.slimes.iter_mut() {
            if let Some(name) = op.tick_recovery() { cleared_names.push(name); }
        }
        for name in cleared_names {
            self.status_msg = format!("{} cleared for duty.", name);
        }

        // 0. Top Status Bar
        egui::TopBottomPanel::top("top_bar")
            .frame(egui::Frame::none().inner_margin(egui::Margin {
                left: safe_area.left, right: safe_area.right, top: safe_area.top, bottom: 0.0,
            }))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("OPERATOR").strong());
                    ui.separator();
                    ui.label(format!("Bank: ${}", self.state.bank));
                    ui.separator();
                    ui.label(format!("Scrap: {}kg", self.state.inventory.scrap));
                    
                    let stress_pct = (self.state.world_map.startled_level / 10.0).clamp(0.0, 1.0);
                    ui.add_space(8.0);
                    ui.add(egui::ProgressBar::new(stress_pct)
                        .fill(egui::Color32::from_rgb(200, 50, 50))
                        .desired_width(100.0)
                    );
                });
            });

        // 1. Launch Bar, Sub-Tab Bar & Main Tab Bar (Bottom) — Compact layout only
        if layout == crate::platform::ResponsiveLayout::Compact {
            egui::TopBottomPanel::bottom("bottom_stack")
                .frame(egui::Frame::none().inner_margin(egui::Margin {
                    left: safe_area.left, right: safe_area.right, top: 0.0, bottom: safe_area.bottom,
                }))
                .show(ctx, |ui| {
                    self.render_launch_bar(ui);

                    // ── Sub-Tab Row ──────────────────────────────────────────
                    // Renders the sub-tabs for the currently active main tab.
                    // Only shown when the active tab has sub-tabs to switch.
                    // Height: 44dp (WCAG 2.5.5 minimum touch target).
                    ui.separator();
                    let sub_tab_h = 44.0;
                    match self.active_tab {
                        crate::platform::BottomTab::Roster => {
                            use crate::platform::RosterSubTab;
                            let sub_tabs = [
                                (RosterSubTab::Collection, "Collect"),
                                (RosterSubTab::Breeding,   "Breed"),
                                (RosterSubTab::Recruit,    "Recruit"),
                                (RosterSubTab::Squad,      "Squad"),
                            ];
                            ui.horizontal(|ui| {
                                let w = ui.available_width() / sub_tabs.len() as f32;
                                for (tab, label) in sub_tabs {
                                    if ui.add_sized(
                                        [w, sub_tab_h],
                                        egui::SelectableLabel::new(self.roster_sub_tab == tab, label)
                                    ).clicked() {
                                        self.roster_sub_tab = tab;
                                        self.selected_slime_id = None; // clear detail view on tab switch
                                    }
                                }
                            });
                        }
                        crate::platform::BottomTab::Missions => {
                            use crate::platform::MissionsSubTab;
                            let sub_tabs = [
                                (MissionsSubTab::Active,     "Active"),
                                (MissionsSubTab::QuestBoard, "Contracts"),
                            ];
                            ui.horizontal(|ui| {
                                let w = ui.available_width() / sub_tabs.len() as f32;
                                for (tab, label) in sub_tabs {
                                    if ui.add_sized(
                                        [w, sub_tab_h],
                                        egui::SelectableLabel::new(self.missions_sub_tab == tab, label)
                                    ).clicked() {
                                        self.missions_sub_tab = tab;
                                    }
                                }
                            });
                        }
                        crate::platform::BottomTab::Map => {
                            use crate::platform::MapSubTab;
                            let sub_tabs = [
                                (MapSubTab::Zones,         "Zones"),
                                (MapSubTab::Quartermaster, "Shop"),
                            ];
                            ui.horizontal(|ui| {
                                let w = ui.available_width() / sub_tabs.len() as f32;
                                for (tab, label) in sub_tabs {
                                    if ui.add_sized(
                                        [w, sub_tab_h],
                                        egui::SelectableLabel::new(self.map_sub_tab == tab, label)
                                    ).clicked() {
                                        self.map_sub_tab = tab;
                                    }
                                }
                            });
                        }
                        crate::platform::BottomTab::Logs => {
                            use crate::platform::LogsSubTab;
                            let sub_tabs = [
                                (LogsSubTab::MissionHistory, "Missions"),
                                (LogsSubTab::CultureHistory, "Culture"),
                            ];
                            ui.horizontal(|ui| {
                                let w = ui.available_width() / sub_tabs.len() as f32;
                                for (tab, label) in sub_tabs {
                                    if ui.add_sized(
                                        [w, sub_tab_h],
                                        egui::SelectableLabel::new(self.logs_sub_tab == tab, label)
                                    ).clicked() {
                                        self.logs_sub_tab = tab;
                                    }
                                }
                            });
                        }
                    }
                    ui.separator();

                    // ── Main Tab Bar ─────────────────────────────────────────
                    ui.horizontal(|ui| {
                        let tabs = [
                            (crate::platform::BottomTab::Roster,   "🧬 Roster"),
                            (crate::platform::BottomTab::Missions, "🚀 Ops"),
                            (crate::platform::BottomTab::Map,      "🗺️ Map"),
                            (crate::platform::BottomTab::Logs,     "📜 Logs"),
                        ];
                        let w = ui.available_width() / tabs.len() as f32;
                        for (tab, label) in tabs {
                            if ui.add_sized([w, 40.0], egui::SelectableLabel::new(self.active_tab == tab, label)).clicked() {
                                self.active_tab = tab;
                            }
                        }
                    });
                });
        }

        // 2. Sidebar (Left) - Only in Standard layout
        if layout == crate::platform::ResponsiveLayout::Standard {
            egui::SidePanel::left("left_sidebar")
                .frame(egui::Frame::none().fill(egui::Color32::from_rgb(19, 19, 24)).inner_margin(8.0))
                .resizable(false)
                .default_width(100.0)
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    self.render_sub_tabs(ui);
                });
        }

        // 3. Central Content
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                match self.active_tab {
                    crate::platform::BottomTab::Roster => match self.roster_sub_tab {
                        crate::platform::RosterSubTab::Collection => {
                            if self.selected_slime_id.is_some() { self.render_slime_detail(ui); }
                            else { self.render_manifest(ui); }
                        }
                        crate::platform::RosterSubTab::Breeding => self.render_incubator(ui),
                        crate::platform::RosterSubTab::Recruit => self.render_recruit(ui),
                        crate::platform::RosterSubTab::Squad => self.render_squad(ui),
                    },
                    crate::platform::BottomTab::Missions => match self.missions_sub_tab {
                        crate::platform::MissionsSubTab::Active => self.render_active_ops(ui),
                        crate::platform::MissionsSubTab::QuestBoard => self.render_contracts(ui),
                    },
                    crate::platform::BottomTab::Map => match self.map_sub_tab {
                        crate::platform::MapSubTab::Zones => self.render_radar(ui),
                        crate::platform::MapSubTab::Quartermaster => {
                            self.render_quartermaster(ui);
                        }
                    },
                    crate::platform::BottomTab::Logs => {
                        self.render_combat_log(ui);
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

impl OperatorApp {
    fn render_sub_tabs(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 4.0);

            match self.active_tab {
                crate::platform::BottomTab::Roster => {
                    sidebar_header(ui, "Roster");

                    if sub_tab_button(ui, "Collection", self.roster_sub_tab == crate::platform::RosterSubTab::Collection) {
                        self.roster_sub_tab = crate::platform::RosterSubTab::Collection;
                    }
                    if sub_tab_button(ui, "Breeding", self.roster_sub_tab == crate::platform::RosterSubTab::Breeding) {
                        self.roster_sub_tab = crate::platform::RosterSubTab::Breeding;
                    }
                    if sub_tab_button(ui, "Recruit", self.roster_sub_tab == crate::platform::RosterSubTab::Recruit) {
                        self.roster_sub_tab = crate::platform::RosterSubTab::Recruit;
                    }
                    if sub_tab_button(ui, "Squad", self.roster_sub_tab == crate::platform::RosterSubTab::Squad) {
                        self.roster_sub_tab = crate::platform::RosterSubTab::Squad;
                    }
                }

                crate::platform::BottomTab::Missions => {
                    sidebar_header(ui, "Missions");
                    if sub_tab_button(ui, "Active", self.missions_sub_tab == crate::platform::MissionsSubTab::Active) {
                        self.missions_sub_tab = crate::platform::MissionsSubTab::Active;
                    }
                    if sub_tab_button(ui, "Quests", self.missions_sub_tab == crate::platform::MissionsSubTab::QuestBoard) {
                        self.missions_sub_tab = crate::platform::MissionsSubTab::QuestBoard;
                    }
                }

                crate::platform::BottomTab::Map => {
                    sidebar_header(ui, "Map");
                    if sub_tab_button(ui, "Zones", self.map_sub_tab == crate::platform::MapSubTab::Zones) {
                        self.map_sub_tab = crate::platform::MapSubTab::Zones;
                    }
                    if sub_tab_button(ui, "Shop", self.map_sub_tab == crate::platform::MapSubTab::Quartermaster) {
                        self.map_sub_tab = crate::platform::MapSubTab::Quartermaster;
                    }
                }

                crate::platform::BottomTab::Logs => {
                    sidebar_header(ui, "LOGS");
                    if sub_tab_button(ui, "Missions", self.logs_sub_tab == crate::platform::LogsSubTab::MissionHistory) {
                        self.logs_sub_tab = crate::platform::LogsSubTab::MissionHistory;
                    }
                    if sub_tab_button(ui, "Culture", self.logs_sub_tab == crate::platform::LogsSubTab::CultureHistory) {
                        self.logs_sub_tab = crate::platform::LogsSubTab::CultureHistory;
                    }
                }
            }
        });
    }

    pub(crate) fn render_combat_log(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("MISSION LOGS");
            ui.add_space(8.0);

            if self.state.combat_log.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(egui::RichText::new("NO LOGS RECORDED").color(egui::Color32::GRAY));
                });
                return;
            }

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .id_source("combat_log_scroll")
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for entry in &self.state.combat_log {
                             egui::Frame::none()
                                .fill(egui::Color32::from_rgb(20, 20, 25))
                                .inner_margin(egui::Margin::same(8.0))
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    
                                    let (icon, color) = match entry.outcome {
                                        crate::models::LogOutcome::Victory => ("✅", egui::Color32::from_rgb(105, 254, 165)),
                                        crate::models::LogOutcome::Failure => ("❌", egui::Color32::from_rgb(254, 165, 105)),
                                        crate::models::LogOutcome::CritFail => ("☠", egui::Color32::from_rgb(255, 80, 80)),
                                        crate::models::LogOutcome::System => ("💾", egui::Color32::LIGHT_BLUE),
                                    };

                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(egui::RichText::new(icon).size(16.0));
                                        ui.label(egui::RichText::new(&entry.message).color(color).small());
                                    });
                                });
                            ui.add_space(4.0);
                        }
                    });
                });
        });
    }
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
