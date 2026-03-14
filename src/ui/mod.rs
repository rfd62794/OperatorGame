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
use eframe::egui;
use uuid::Uuid;

use crate::garden::Garden;
use crate::log_engine::{format_log_entry, generate_narrative};
use crate::models::{AarOutcome, Deployment, Mission, SlimeState};
use crate::persistence::{save, GameState};

pub mod cargo;
pub mod contracts;
pub mod manifest;
pub mod ops;
pub mod radar;

// ---------------------------------------------------------------------------
// App State
// ---------------------------------------------------------------------------

pub struct OperatorApp {
    state: GameState,
    save_path: PathBuf,
    /// Mission currently targeted for deployment.
    selected_mission: Option<Uuid>,
    /// Operators staged for the next launch. Toggled by clicking roster cards.
    staged_operators: HashSet<Uuid>,
    /// One-line feedback shown at the bottom of the screen.
    status_msg: String,
    /// Scrollable narrative log. New entries prepended (newest first).
    combat_log: Vec<String>,
    /// The living Shepherd's Garden background simulation.
    garden: Garden,
    /// Slime selected via clicking the garden.
    pub selected_slime_id: Option<Uuid>,
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
        let garden = Garden::from_operators(&state.slimes, egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1000.0, 1000.0)));
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
            combat_log: Vec::new(),
            garden,
            selected_slime_id: None,
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

    fn render_roster(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.left_tab, LeftTab::Manifest, "BIO-MANIFEST");
            ui.selectable_value(&mut self.left_tab, LeftTab::Recruit, "RECRUIT");
            
            if self.state.tech_tier < 1 {
                ui.add_enabled(false, egui::SelectableLabel::new(false, "INCUBATOR (Req. Tier 1)"));
            } else {
                ui.selectable_value(&mut self.left_tab, LeftTab::Incubator, "INCUBATOR");
            }
        });
        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        match self.left_tab {
            LeftTab::Manifest => self.render_manifest(ui),
            LeftTab::Incubator => self.render_incubator(ui),
            LeftTab::Recruit => self.render_recruit(ui),
        }
    }



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
            narrative.push_str("\nFIELD OPS PROTOCOL §7 ACTIVE: Personnel operating outside approved medical clearance. Deployment authorized with +15 Critical Stress Penalty.");
        }
        
        // Drop the immutable borrow by finishing use of squad
        let log_entry = format_log_entry(&mission.name, &outcome, &narrative);
        self.combat_log.insert(0, log_entry); // newest first
        if self.combat_log.len() > 50 { self.combat_log.truncate(50); }

        self.state.deployments[dep_idx].resolved = true;

        // Sprint 8: Award XP to the squad
        {
            let mut mut_squad: Vec<&mut crate::models::Operator> = self
                .state
                .slimes
                .iter_mut()
                .filter(|o| dep.operator_ids.contains(&o.genome.id))
                .collect();
                
            let xp_results = dep.award_squad_xp(&mission, &mut mut_squad, &outcome);
            for (id, _xp, leveled) in xp_results {
                if leveled {
                    if let Some(op) = self.state.slimes.iter().find(|s| s.genome.id == id) {
                        let msg = format!(">> EXCELLENCE RECOGNIZED: {} has reached Level {}!", op.name(), op.level);
                        self.combat_log.insert(0, msg);
                    }
                }
            }
        }

        // Phase A: Apply injuries (probabilistic)
        // This requires &mut self.state.slimes
        let newly_injured = crate::models::apply_outcome_injuries(
            &mut outcome,
            &mut self.state.slimes,
            &dep.operator_ids,
            &mut rand::thread_rng(),
        );
        let newly_injured_ids: Vec<Uuid> = newly_injured.iter().map(|(id, _)| *id).collect();

        match outcome {
            AarOutcome::Victory { reward, success_rate, .. } => {
                self.state.bank += reward as i64;
                
                let debt_warning = if self.state.bank < 0 { 
                    "\nNOTE: Current operational balance is negative. Deployment authorized under Emergency Continuity Protocol §4.2."
                } else { "" };

                self.status_msg = format!("✅ '{}' — VICTORY (+${}).{}", mission.name, reward, debt_warning);
                
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
                let symbol = if is_crit { "☠" } else { "❌" };
                let label = if is_crit { "CRITICAL FAILURE" } else { "FAILURE" };

                if !newly_injured.is_empty() {
                    let (id, until) = newly_injured[0];
                    let op = self.state.slimes.iter().find(|s| s.genome.id == id);
                    let name = op.map(|s| s.name()).unwrap_or("Operator");
                    
                    let remaining = until - Utc::now();
                    let h = remaining.num_hours();
                    let m = remaining.num_minutes() % 60;
                    
                    self.status_msg = format!("{} '{}' — {}. INCIDENT REPORT: {} sustained injuries. Medical leave approved. RTD estimated {}h {}m.", 
                        symbol, mission.name, label, name, h, m);
                } else {
                    self.status_msg = format!("{} '{}' — {}. The squad retreated intact.", symbol, mission.name, label);
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
    fn render_right_column(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.right_tab, RightTab::Contracts, "CONTRACTS");
            ui.selectable_value(&mut self.right_tab, RightTab::Radar, "RADAR");
            ui.selectable_value(&mut self.right_tab, RightTab::Cargo, "CARGO BAY");
        });
        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);
        
        match self.right_tab {
            RightTab::Contracts => self.render_contracts(ui),
            RightTab::Radar => self.render_radar(ui),
            RightTab::Cargo => self.render_cargo(ui),
        }
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
            self.combat_log.insert(0, msg);
            self.status_msg = format!("{} cleared for duty.", name);
        }

        // Sprint 7B: Tick daily upkeep
        let (deducted, idle_count) = self.state.apply_daily_upkeep(Utc::now());
        if deducted > 0 {
            let msg = format!("Deducted ${} in maintenance costs for {} idle operator(s).", deducted, idle_count);
            self.combat_log.insert(0, msg);
            self.persist();
        }

        // Sprint 8: Refresh mission pool
        if self.state.refresh_missions_if_needed(Utc::now()) {
            let msg = format!("MISSION POOL REFRESHED: New contracts available for {} UTC.", Utc::now().date_naive());
            self.combat_log.insert(0, msg);
            self.persist();
        }

        // Background Garden
        let _t = ctx.input(|i| i.time as f32);
        let _cursor = ctx.input(|i| i.pointer.hover_pos());
        let _screen_rect = ctx.screen_rect();

        /*
        // --- Garden temporarily disabled for "Industrial Pivot" UI Focus ---

        // Advance garden simulation
        self.garden.tick(0.1, cursor, screen_rect);

        // Intercept clicks in empty space for selecting garden slimes
        if ctx.input(|i| i.pointer.primary_clicked()) && !ctx.wants_pointer_input() {
            if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                if let Some(id) = self.garden.handle_click(pos) {
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
            ui.horizontal(|ui| {
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

        // Combat log panel — sits above the launch bar
        egui::TopBottomPanel::bottom("combat_log_panel")
            .resizable(true)
            .min_height(80.0)
            .max_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("── COMBAT LOG ──").strong());
                    if !self.combat_log.is_empty() {
                        if ui.small_button("Clear").clicked() {
                            self.combat_log.clear();
                        }
                    }
                });
                egui::ScrollArea::vertical()
                    .id_source("combat_log_scroll")
                    .stick_to_bottom(false)
                    .show(ui, |ui| {
                        if self.combat_log.is_empty() {
                            ui.label(
                                egui::RichText::new("No actions yet. Deploy a squad to begin.")
                                    .color(egui::Color32::GRAY)
                                    .italics(),
                            );
                        } else {
                            for entry in &self.combat_log {
                                let color = if entry.contains("VICTORY") {
                                    egui::Color32::from_rgb(80, 200, 120)
                                } else if entry.contains("CRITICAL") {
                                    egui::Color32::from_rgb(220, 80, 80)
                                } else {
                                    egui::Color32::YELLOW
                                };
                                ui.colored_label(color, entry);
                            }
                        }
                    });
            });

        // Bottom launch / status bar
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

        // Bottom Navigation Tab Bar (Phase B)
        let layout = crate::platform::LayoutCalculator::new(
            egui::vec2(ctx.screen_rect().width(), ctx.screen_rect().height()),
            safe_area,
        );
        let tab_rect = layout.bottom_tab_rect(&safe_area);

        egui::Area::new(egui::Id::new("bottom_tabs"))
            .fixed_pos(tab_rect.min)
            .show(ctx, |ui| {
                ui.set_min_size(egui::vec2(tab_rect.width(), tab_rect.height()));
                
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                    let tab_width = tab_rect.width() / 4.0;
                    
                    let tabs = [
                        (crate::platform::BottomTab::Roster, "🧬 Roster"),
                        (crate::platform::BottomTab::Missions, "🚀 Missions"),
                        (crate::platform::BottomTab::Map, "🗺️ Map"),
                        (crate::platform::BottomTab::Logs, "📜 Logs"),
                    ];

                    for (_idx, (tab, label)) in tabs.iter().enumerate() {
                        let is_active = self.active_tab == *tab;
                        let button = egui::Button::new(*label)
                            .fill(if is_active { 
                                egui::Color32::from_rgb(100, 200, 100) 
                            } else { 
                                egui::Color32::from_rgb(60, 60, 60) 
                            })
                            .min_size(egui::vec2(tab_width, tab_rect.height()));
                        
                        if ui.add(button).clicked() {
                            self.active_tab = *tab;
                        }
                    }
                });
            });

        // Three-column central panel (Desktop) or Tab-view (Mobile)
        let is_mobile = ctx.screen_rect().width() < 800.0 || cfg!(target_os = "android");

        if is_mobile {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::none()
                        .inner_margin(egui::Margin {
                            left: safe_area.left,
                            right: safe_area.right,
                            top: 0.0,
                            bottom: 0.0,
                        })
                )
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        // Left sidebar: sub-tab navigation
                        ui.vertical(|ui| {
                            ui.set_width(100.0);
                            render_sub_tabs(ui, self.active_tab, self);
                        });

                        ui.separator();

                        ui.vertical(|ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                match self.active_tab {
                                    crate::platform::BottomTab::Roster => match self.roster_sub_tab {
                                        crate::platform::RosterSubTab::Collection => {
                                            ui.label("[TODO] Roster → Collection");
                                        }
                                        crate::platform::RosterSubTab::Breeding => {
                                            ui.label("[TODO] Roster → Breeding");
                                        }
                                    },
                                    crate::platform::BottomTab::Missions => match self.missions_sub_tab {
                                        crate::platform::MissionsSubTab::Active => {
                                            ui.label("[TODO] Missions → Active");
                                        }
                                        crate::platform::MissionsSubTab::QuestBoard => {
                                            ui.label("[TODO] Missions → Quest Board");
                                        }
                                    },
                                    crate::platform::BottomTab::Map => match self.map_sub_tab {
                                        crate::platform::MapSubTab::Zones => {
                                            ui.label("[TODO] Map → Zones");
                                        }
                                    },
                                    crate::platform::BottomTab::Logs => match self.logs_sub_tab {
                                        crate::platform::LogsSubTab::MissionHistory => {
                                            ui.label("[TODO] Logs → Mission History");
                                        }
                                        crate::platform::LogsSubTab::CultureHistory => {
                                            ui.label("[TODO] Logs → Culture History");
                                        }
                                    },
                                }
                            });
                        });
                    });
                });
        } else {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::none()
                        .inner_margin(egui::Margin {
                            left: safe_area.left,
                            right: safe_area.right,
                            top: 0.0,
                            bottom: 0.0,
                        })
                )
                .show(ctx, |ui| {
                // Collect the three column contents; egui columns take a closure
                // so we must render them inside the callback.
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.columns(3, |cols| {
                        // We can't call &mut self methods twice in the same borrow,
                        // so we forward rendering through standalone functions
                        // that accept &mut OperatorApp.
                        egui::ScrollArea::vertical()
                            .id_source("roster_scroll")
                            .show(&mut cols[0], |ui| {
                                // Safety: columns are non-overlapping &mut [egui::Ui]
                                // We render each column via a closure scope.
                                render_roster_panel(ui, self);
                            });
                        egui::ScrollArea::vertical()
                            .id_source("ops_scroll")
                            .show(&mut cols[1], |ui| {
                                render_ops_panel(ui, self);
                            });
                        egui::ScrollArea::vertical()
                            .id_source("contracts_scroll")
                            .show(&mut cols[2], |ui| {
                                render_right_panel(ui, self);
                            });
                    });
                });
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Free-standing column render helpers (work around borrow-checker in columns)
// ---------------------------------------------------------------------------

fn render_roster_panel(ui: &mut egui::Ui, app: &mut OperatorApp) {
    app.render_roster(ui);
}

fn render_sub_tabs(
    ui: &mut egui::Ui,
    active_main_tab: crate::platform::BottomTab,
    app: &mut OperatorApp,
) {
    ui.vertical(|ui| {
        ui.label("─────────────"); // Visual separator

        match active_main_tab {
            crate::platform::BottomTab::Roster => {
                ui.label("Roster");

                if ui
                    .selectable_label(
                        app.roster_sub_tab == crate::platform::RosterSubTab::Collection,
                        "Collection",
                    )
                    .clicked()
                {
                    app.roster_sub_tab = crate::platform::RosterSubTab::Collection;
                }

                if ui
                    .selectable_label(
                        app.roster_sub_tab == crate::platform::RosterSubTab::Breeding,
                        "Breeding",
                    )
                    .clicked()
                {
                    app.roster_sub_tab = crate::platform::RosterSubTab::Breeding;
                }
            }

            crate::platform::BottomTab::Missions => {
                ui.label("Missions");

                if ui
                    .selectable_label(
                        app.missions_sub_tab == crate::platform::MissionsSubTab::Active,
                        "Active",
                    )
                    .clicked()
                {
                    app.missions_sub_tab = crate::platform::MissionsSubTab::Active;
                }

                if ui
                    .selectable_label(
                        app.missions_sub_tab == crate::platform::MissionsSubTab::QuestBoard,
                        "Quest Board",
                    )
                    .clicked()
                {
                    app.missions_sub_tab = crate::platform::MissionsSubTab::QuestBoard;
                }
            }

            crate::platform::BottomTab::Map => {
                ui.label("Map");

                if ui
                    .selectable_label(
                        app.map_sub_tab == crate::platform::MapSubTab::Zones,
                        "Zones",
                    )
                    .clicked()
                {
                    app.map_sub_tab = crate::platform::MapSubTab::Zones;
                }
            }

            crate::platform::BottomTab::Logs => {
                ui.label("Logs");

                if ui
                    .selectable_label(
                        app.logs_sub_tab == crate::platform::LogsSubTab::MissionHistory,
                        "Mission History",
                    )
                    .clicked()
                {
                    app.logs_sub_tab = crate::platform::LogsSubTab::MissionHistory;
                }

                if ui
                    .selectable_label(
                        app.logs_sub_tab == crate::platform::LogsSubTab::CultureHistory,
                        "Culture History",
                    )
                    .clicked()
                {
                    app.logs_sub_tab = crate::platform::LogsSubTab::CultureHistory;
                }
            }
        }
    });
}

fn render_ops_panel(ui: &mut egui::Ui, app: &mut OperatorApp) {
    app.render_active_ops(ui);
}

fn render_right_panel(ui: &mut egui::Ui, app: &mut OperatorApp) {
    app.render_right_column(ui);
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
