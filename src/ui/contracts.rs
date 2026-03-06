// src/ui/contracts.rs
use eframe::egui;
use crate::ui::OperatorApp;

impl OperatorApp {
    pub(crate) fn render_contracts(&mut self, ui: &mut egui::Ui) {
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
}
