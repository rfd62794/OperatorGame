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

        egui::ScrollArea::vertical()
            .id_source("missions_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
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
                            ui.horizontal(|ui| {
                                let tier_color = match mission.tier {
                                    crate::models::MissionTier::Starter  => egui::Color32::from_rgb(150, 255, 150),
                                    crate::models::MissionTier::Standard => egui::Color32::from_rgb(150, 200, 255),
                                    crate::models::MissionTier::Advanced => egui::Color32::from_rgb(255, 200, 100),
                                    crate::models::MissionTier::Elite    => egui::Color32::from_rgb(255, 100, 255),
                                };
                                ui.label(egui::RichText::new(format!("{:?}", mission.tier)).color(tier_color).small().strong());
                                ui.label(egui::RichText::new(&mission.name).strong().size(14.0).color(egui::Color32::WHITE));
                            });

                            ui.horizontal_wrapped(|ui| {
                                if let Some(aff) = mission.affinity {
                                    let (r, g, b) = crate::genetics::culture_display_color_standalone(aff);
                                    ui.colored_label(egui::Color32::from_rgb(r, g, b), format!("[{aff:?}]"));
                                }
                                ui.label(egui::RichText::new(format!("STR:{}", mission.req_strength)).color(egui::Color32::LIGHT_GRAY));
                                ui.label(egui::RichText::new(format!("AGI:{}", mission.req_agility)).color(egui::Color32::LIGHT_GRAY));
                                ui.label(egui::RichText::new(format!("INT:{}", mission.req_intelligence)).color(egui::Color32::LIGHT_GRAY));
                            });

                            ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new(format!("DC: {}", mission.base_dc)).color(egui::Color32::LIGHT_GRAY));
                                
                                // Show real-time success chance based on STAGED operators
                                let staged_ops: Vec<&crate::models::Operator> = self.state.slimes.iter()
                                    .filter(|op| self.staged_operators.contains(&op.genome.id))
                                    .collect();
                                
                                let (label, chance) = mission.calculate_success_chance(&staged_ops);
                                let chance_pct = (chance * 100.0) as u32;
                                
                                let chance_color = if chance >= 0.75 {
                                    egui::Color32::from_rgb(80, 255, 120) // Success
                                } else if chance >= 0.50 {
                                    egui::Color32::YELLOW // Risky
                                } else {
                                    egui::Color32::from_rgb(255, 100, 100) // Danger
                                };

                                ui.label(egui::RichText::new(format!("{} - {}%", label, chance_pct)).color(chance_color).strong());

                                ui.label(egui::RichText::new(format!("| {}s | ${}", mission.duration_secs, mission.reward)).color(egui::Color32::LIGHT_GRAY));
                            });

                            let btn_label = if is_selected {
                                "▶ SELECTED"
                            } else {
                                "SELECT CONTRACT"
                            };
                            if ui.button(btn_label).clicked() {
                                new_selection = if is_selected { None } else { Some(mission.id) };
                                // Task D: DO NOT clear staged_operators here. 
                                // Allow them to persist if the user is swapping contracts to compare odds.
                            }
                        });
                    ui.add_space(4.0);
                }
            });

        self.selected_mission = new_selection;
    }
}
