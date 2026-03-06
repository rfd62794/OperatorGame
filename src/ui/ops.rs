// src/ui/ops.rs

use eframe::egui;
use uuid::Uuid;

use crate::ui::OperatorApp;

impl OperatorApp {
    pub(crate) fn render_active_ops(&mut self, ui: &mut egui::Ui) {
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

    pub(crate) fn render_launch_bar(&mut self, ui: &mut egui::Ui) {
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
}
