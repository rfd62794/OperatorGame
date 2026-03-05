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
use rand::Rng;
use uuid::Uuid;

use crate::models::{AarOutcome, Deployment, Mission, Operator, OperatorState};
use crate::persistence::{save, GameState};

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
}

impl OperatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, state: GameState, save_path: PathBuf) -> Self {
        Self {
            state,
            save_path,
            selected_mission: None,
            staged_operators: HashSet::new(),
            status_msg: String::from("Welcome to OPERATOR. Select a contract, then stage your squad."),
        }
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Staged operators as references — used for the live success% preview.
    fn staged_squad<'a>(&'a self) -> Vec<&'a Operator> {
        self.state
            .roster
            .iter()
            .filter(|o| self.staged_operators.contains(&o.id))
            .collect()
    }

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
        ui.label(egui::RichText::new("── UNIT ROSTER ──").strong().size(14.0));
        ui.add_space(4.0);

        if self.state.roster.is_empty() {
            ui.label("No operators. Use the CLI to hire: `operator hire <name> <job>`");
            return;
        }

        let staged = self.staged_operators.clone();
        let selected_mission_id = self.selected_mission;

        // Lookup the selected mission for live success preview
        let selected_mission = selected_mission_id
            .and_then(|id| self.state.missions.iter().find(|m| m.id == id).cloned());

        for op in self.state.roster.iter_mut() {
            // Tick recovery passively
            op.tick_recovery();

            let is_staged = staged.contains(&op.id);
            let can_stage = op.is_available() && selected_mission_id.is_some();

            let (state_color, state_label) = match &op.state {
                OperatorState::Idle => (egui::Color32::from_rgb(80, 200, 120), "IDLE"),
                OperatorState::Deployed(_) => (egui::Color32::from_rgb(255, 200, 0), "DEPLOYED"),
                OperatorState::Injured(until) => {
                    let secs = (*until - Utc::now()).num_seconds().max(0);
                    let _ = secs; // used in label below
                    (egui::Color32::from_rgb(220, 80, 80), "INJURED")
                }
            };

            let frame_color = if is_staged {
                egui::Color32::from_rgb(40, 80, 60)
            } else {
                egui::Color32::from_rgb(30, 30, 40)
            };

            egui::Frame::none()
                .fill(frame_color)
                .inner_margin(egui::Margin::same(6.0))
                .rounding(egui::Rounding::same(4.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.colored_label(state_color, "●");
                        ui.label(egui::RichText::new(&op.name).strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.small(egui::RichText::new(state_label).color(state_color));
                        });
                    });

                    let (s, a, i) = op.effective_stats();
                    ui.label(
                        egui::RichText::new(format!(
                            "{} | STR:{} AGI:{} INT:{}",
                            op.job, s, a, i
                        ))
                        .small()
                        .color(egui::Color32::GRAY),
                    );

                    // Injury countdown
                    if let OperatorState::Injured(until) = op.state {
                        let secs = (until - Utc::now()).num_seconds().max(0);
                        ui.label(
                            egui::RichText::new(format!("  Recovery in: {}s", secs))
                                .small()
                                .color(egui::Color32::from_rgb(220, 80, 80)),
                        );
                    }

                    if can_stage {
                        let label = if is_staged { "✓ STAGED" } else { "+ STAGE" };
                        if ui.small_button(label).clicked() {
                            if is_staged {
                                self.staged_operators.remove(&op.id);
                            } else if self.staged_operators.len() < 3 {
                                self.staged_operators.insert(op.id);
                            } else {
                                self.status_msg =
                                    "Max squad size is 3 operators.".to_string();
                            }
                        }
                    }
                });
            ui.add_space(4.0);
        }

        // Live success% preview
        if let Some(mission) = &selected_mission {
            let squad: Vec<&Operator> = self
                .state
                .roster
                .iter()
                .filter(|o| staged.contains(&o.id))
                .collect();
            if !squad.is_empty() {
                let rate = mission.calculate_success_rate(&squad);
                ui.separator();
                let color = if rate >= 0.7 {
                    egui::Color32::from_rgb(80, 200, 120)
                } else if rate >= 0.4 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::from_rgb(220, 80, 80)
                };
                ui.colored_label(
                    color,
                    format!("Success if deployed: {:.0}%", rate * 100.0),
                );
            }
        }
    }

    fn render_active_ops(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("── ACTIVE OPERATIONS ──").strong().size(14.0));
        ui.add_space(4.0);

        if self.state.deployments.is_empty() {
            ui.label("No active operations.");
            return;
        }

        let mut to_resolve: Vec<Uuid> = vec![];

        for dep in &self.state.deployments {
            if dep.resolved { continue; }

            let mission_name = self
                .state
                .missions
                .iter()
                .find(|m| m.id == dep.mission_id)
                .map(|m| m.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            let (progress, remaining_secs) = self.progress_for(dep);

            egui::Frame::none()
                .fill(egui::Color32::from_rgb(25, 35, 50))
                .inner_margin(egui::Margin::same(6.0))
                .rounding(egui::Rounding::same(4.0))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&mission_name).strong());

                    if progress < 1.0 {
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .show_percentage()
                                .animate(true),
                        );
                        ui.small(format!("ETA: {}s", remaining_secs));
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(80, 200, 120),
                            "✅ COMPLETE — Awaiting AAR",
                        );
                        if ui.button("⚡ PROCESS AAR").clicked() {
                            to_resolve.push(dep.id);
                        }
                    }

                    // Squad IDs
                    ui.small(
                        egui::RichText::new(format!(
                            "Squad: {} operator(s)",
                            dep.operator_ids.len()
                        ))
                        .color(egui::Color32::GRAY),
                    );
                });
            ui.add_space(4.0);
        }

        // Process any AAR clicks
        for dep_id in to_resolve {
            self.resolve_deployment(dep_id);
        }
        self.state.deployments.retain(|d| !d.resolved);
    }

    fn render_contracts(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("── AVAILABLE CONTRACTS ──").strong().size(14.0));
        ui.add_space(4.0);

        if self.state.missions.is_empty() {
            ui.label("No contracts available.");
            return;
        }

        let selected = self.selected_mission;
        let mut new_selection = selected;

        for mission in &self.state.missions {
            let is_selected = selected == Some(mission.id);
            let frame_color = if is_selected {
                egui::Color32::from_rgb(20, 50, 80)
            } else {
                egui::Color32::from_rgb(25, 25, 35)
            };

            egui::Frame::none()
                .fill(frame_color)
                .stroke(egui::Stroke::new(
                    if is_selected { 1.5 } else { 0.0 },
                    egui::Color32::from_rgb(50, 130, 200),
                ))
                .inner_margin(egui::Margin::same(6.0))
                .rounding(egui::Rounding::same(4.0))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&mission.name).strong());

                    ui.horizontal(|ui| {
                        ui.small(format!("STR:{}", mission.req_strength));
                        ui.small(format!("AGI:{}", mission.req_agility));
                        ui.small(format!("INT:{}", mission.req_intelligence));
                    });

                    ui.horizontal(|ui| {
                        let diff_color = if mission.difficulty < 0.3 {
                            egui::Color32::from_rgb(80, 200, 120)
                        } else if mission.difficulty < 0.5 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::from_rgb(220, 80, 80)
                        };
                        ui.colored_label(
                            diff_color,
                            format!("Diff: {:.0}%", mission.difficulty * 100.0),
                        );
                        ui.small(format!("| {}s | ${}", mission.duration_secs, mission.reward));
                    });

                    let btn_label = if is_selected {
                        "✓ SELECTED"
                    } else {
                        "SELECT CONTRACT"
                    };
                    if ui.button(btn_label).clicked() {
                        new_selection = if is_selected { None } else { Some(mission.id) };
                        self.staged_operators.clear();
                    }
                });
            ui.add_space(4.0);
        }

        self.selected_mission = new_selection;
    }

    fn render_launch_bar(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            if let Some(mission_id) = self.selected_mission {
                let mission = self
                    .state
                    .missions
                    .iter()
                    .find(|m| m.id == mission_id)
                    .cloned();

                if let Some(ref mission) = mission {
                    let staged_count = self.staged_operators.len();
                    ui.label(format!(
                        "Target: {}  |  Squad: {} operator(s)  |  ",
                        mission.name, staged_count
                    ));

                    let can_launch = staged_count >= 1 && staged_count <= 3;
                    let launch_btn = ui.add_enabled(
                        can_launch,
                        egui::Button::new(egui::RichText::new("🚀 LAUNCH MISSION").strong()),
                    );

                    if launch_btn.clicked() {
                        self.launch_mission(mission.clone());
                    }
                }
            } else {
                ui.label("← Select a contract to begin.");
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(&self.status_msg.clone())
                        .small()
                        .color(egui::Color32::GRAY),
                );
            });
        });
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
}

// ---------------------------------------------------------------------------
// eframe::App implementation
// ---------------------------------------------------------------------------

impl eframe::App for OperatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Redraw every 100ms — animates progress bars without a background thread.
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        // Top status bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("OPERATOR: WAR ROOM")
                        .strong()
                        .size(16.0),
                );
                ui.separator();
                ui.label(format!("Bank: ${}", self.state.bank));
                ui.separator();
                ui.label(format!("Roster: {}", self.state.roster.len()));
                ui.separator();
                ui.label(format!(
                    "Active Ops: {}",
                    self.state.deployments.iter().filter(|d| !d.resolved).count()
                ));
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
                            render_contracts_panel(ui, self);
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

fn render_contracts_panel(ui: &mut egui::Ui, app: &mut OperatorApp) {
    app.render_contracts(ui);
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
        Box::new(move |cc| Ok(Box::new(OperatorApp::new(cc, state, save_path)))),
    )
}
