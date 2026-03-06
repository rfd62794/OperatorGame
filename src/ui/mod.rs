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
use crate::models::{AarOutcome, Deployment, Mission, Operator, OperatorState};
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

impl OperatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, state: GameState, save_path: PathBuf) -> Self {
        let garden = Garden::from_genomes(&state.slimes, egui::Rect::EVERYTHING);
        Self {
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

    fn persist(&self) {
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
        // Validate all staged operators are still available
        let staged_ids: Vec<Uuid> = self.staged_operators.iter().cloned().collect();
        for id in &staged_ids {
            let op = self.state.roster.iter().find(|o| o.id == *id);
            if let Some(op) = op {
                if !op.is_available() {
                    self.status_msg = format!("{} is no longer available.", op.name);
                    return;
                }
            }
        }

        // Mark operators as deployed
        for op in self.state.roster.iter_mut() {
            if staged_ids.contains(&op.id) {
                op.state = OperatorState::Deployed(mission.id);
            }
        }

        let deployment = Deployment::start(&mission, staged_ids);
        self.status_msg = format!(
            "Deployed {} operator(s) on '{}'. ETA: {}s.",
            deployment.operator_ids.len(),
            mission.name,
            mission.duration_secs,
        );

        self.state.world_map.startled_level += 0.05; // ADR-015: Hoot & Holler resonance
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

        let squad: Vec<&Operator> = self
            .state
            .roster
            .iter()
            .filter(|o| dep.operator_ids.contains(&o.id))
            .collect();

        let mut rng = rand::thread_rng();
        let outcome = dep.resolve(&mission, &squad, &mut rng);

        // Generate narrative BEFORE mutating squad state
        let narrative = generate_narrative(&outcome, &mission, &squad, &mut rng);
        let log_entry = format_log_entry(&mission.name, &outcome, &narrative);
        self.combat_log.insert(0, log_entry); // newest first
        if self.combat_log.len() > 50 { self.combat_log.truncate(50); }

        self.state.deployments[dep_idx].resolved = true;

        match outcome {
            AarOutcome::Victory { reward } => {
                self.state.bank += reward;
                self.status_msg =
                    format!("✅ '{}' — VICTORY! +${} | Bank: ${}", mission.name, reward, self.state.bank);
                for op in self.state.roster.iter_mut() {
                    if dep.operator_ids.contains(&op.id) {
                        op.state = OperatorState::Idle;
                    }
                }
            }
            AarOutcome::Failure { injured_ids } => {
                let recovery = mission.duration_secs * 2;
                let recover_at = Utc::now() + Duration::seconds(recovery as i64);
                self.status_msg =
                    format!("❌ '{}' — FAILURE. Operators injured for {}s.", mission.name, recovery);
                for op in self.state.roster.iter_mut() {
                    if injured_ids.contains(&op.id) {
                        op.state = OperatorState::Injured(recover_at);
                    }
                }
            }
            AarOutcome::CriticalFailure { killed_id } => {
                let name = self
                    .state
                    .roster
                    .iter()
                    .find(|o| o.id == killed_id)
                    .map(|o| o.name.clone())
                    .unwrap_or_default();
                self.status_msg =
                    format!("☠ '{}' — CRITICAL FAILURE! {} is KIA.", mission.name, name);
                self.state.roster.retain(|o| o.id != killed_id);
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
        // Redraw every 100ms — animates progress bars without a background thread.
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        // Make panels completely opaque to prevent the "dim overlay" feel
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.panel_fill = egui::Color32::from_rgb(15, 15, 20); // Solid dark blue/grey
        style.visuals.window_fill = egui::Color32::from_rgb(15, 15, 20);
        style.visuals.override_text_color = Some(egui::Color32::WHITE);
        ctx.set_style(style);

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
                let genome_map = self.state.slimes.iter().map(|g| (g.id, g)).collect();
                draw_garden(ui.painter(), screen_rect, &genome_map, &self.garden, t);
            });
        */

        // Top status bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("OPERATOR: COMMAND DECK")
                        .strong()
                        .size(16.0),
                );
                ui.separator();
                ui.label(format!("Bank: ${}", self.state.bank));
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
        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            self.render_launch_bar(ui);
        });

        // Three-column central panel
        egui::CentralPanel::default().show(ctx, |ui| {
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

// ---------------------------------------------------------------------------
// Free-standing column render helpers (work around borrow-checker in columns)
// ---------------------------------------------------------------------------

fn render_roster_panel(ui: &mut egui::Ui, app: &mut OperatorApp) {
    app.render_roster(ui);
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
