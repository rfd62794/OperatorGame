// src/ui/squad.rs
use eframe::egui;
use crate::ui::OperatorApp;

impl OperatorApp {
    pub(crate) fn render_squad(&mut self, ui: &mut egui::Ui) {
        let staged_ops: Vec<&crate::models::Operator> = self.state.slimes.iter()
            .filter(|op| self.staged_operators.contains(&op.genome.id))
            .collect();

        egui::ScrollArea::vertical()
            .id_source("squad_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(8.0);
                    ui.heading(egui::RichText::new("STAGED SQUAD").color(egui::Color32::WHITE).strong());
                    ui.label(egui::RichText::new("Current offensive/defensive capability for deployment.").small());
                    ui.add_space(12.0);

                    if staged_ops.is_empty() {
                        ui.label(egui::RichText::new("No operators staged.").italics().color(egui::Color32::GRAY));
                        ui.label("Go to the Collection tab and select operators to build your squad.");
                        return;
                    }

                    // Totals Row (at top for easy reading / transparency)
                    let mut total_str = 0u32;
                    let mut total_agi = 0u32;
                    let mut total_int = 0u32;

                    for op in &staged_ops {
                        let (s, a, i, _, _, _) = op.total_stats();
                        total_str += s;
                        total_agi += a;
                        total_int += i;
                    }

                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(20, 50, 40)) // Subtle green dark highlight
                        .inner_margin(egui::Margin::same(10.0))
                        .rounding(egui::Rounding::same(4.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("SQUAD TOTALS:").strong().color(egui::Color32::WHITE));
                                ui.separator();
                                ui.label(egui::RichText::new(format!("STR: {}", total_str)).strong().color(egui::Color32::LIGHT_GRAY));
                                ui.label(egui::RichText::new(format!("AGI: {}", total_agi)).strong().color(egui::Color32::LIGHT_GRAY));
                                ui.label(egui::RichText::new(format!("INT: {}", total_int)).strong().color(egui::Color32::LIGHT_GRAY));
                            });
                        });

                    ui.add_space(12.0);
                    ui.label(egui::RichText::new("CURRENT ROSTER IN SQUAD").small().strong().color(egui::Color32::from_gray(140)));
                    ui.add_space(4.0);

                    for op in staged_ops {
                        let (s, a, i, _, _, _) = op.total_stats();
                        let culture = op.genome.dominant_culture();
                        let (r, g, b) = crate::genetics::culture_display_color_standalone(culture);

                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(25, 25, 35))
                            .inner_margin(egui::Margin::same(8.0))
                            .rounding(egui::Rounding::same(4.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new(&op.genome.name).strong().color(egui::Color32::WHITE));
                                            ui.colored_label(egui::Color32::from_rgb(r, g, b), format!("[{:?}]", culture));
                                            ui.label(format!("Lvl {}", op.level));
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new(format!("S:{}", s)).small());
                                            ui.label(egui::RichText::new(format!("A:{}", a)).small());
                                            ui.label(egui::RichText::new(format!("I:{}", i)).small());
                                        });
                                    });
                                    
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("REMOVE").clicked() {
                                            self.staged_operators.remove(&op.genome.id);
                                        }
                                    });
                                });
                            });
                        ui.add_space(4.0);
                    }

                    ui.add_space(16.0);
                    if ui.button(egui::RichText::new("CLEAR SQUAD").color(egui::Color32::LIGHT_RED)).clicked() {
                        self.staged_operators.clear();
                    }
                });
            });
    }
}
