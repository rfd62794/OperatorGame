use egui;
use crate::ui::OperatorApp;
use crate::models::{Hat, HatId};

impl OperatorApp {
    pub(crate) fn render_quartermaster(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("QUARTERMASTER");
            ui.add_space(8.0);
            ui.label(format!("Available MTL (Scrap): {}kg", self.state.inventory.scrap));
            ui.add_space(16.0);

            egui::ScrollArea::vertical()
                .id_source("hat_shop")
                .show(ui, |ui| {
                let catalog = Hat::catalog();
                for hat in catalog {
                    self.render_hat_item(ui, hat);
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);
                }
            });
        });
    }

    fn render_hat_item(&mut self, ui: &mut egui::Ui, hat: Hat) {
        let is_unlocked = self.state.unlocked_nodes.contains(&hat.unlock_node_id) || hat.unlock_node_id == 0;
        let is_owned = self.state.hat_inventory.contains(&hat.id) || 
                       self.state.slimes.iter().any(|s| s.equipped_hat == Some(hat.id));

        ui.group(|ui| {
            ui.horizontal(|ui| {
                // Icon (Emoji)
                ui.label(egui::RichText::new("🎩").size(24.0));
                
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(hat.name).strong());
                        if !is_unlocked {
                            ui.label(egui::RichText::new(" [LOCKED]").color(egui::Color32::GRAY));
                        } else if is_owned {
                            ui.label(egui::RichText::new(" [OWNED]").color(egui::Color32::from_rgb(100, 200, 100)));
                        }
                    });

                    let mut stats = Vec::new();
                    if hat.str_bonus > 0 { stats.push(format!("+{} STR", hat.str_bonus)); }
                    if hat.agi_bonus > 0 { stats.push(format!("+{} AGI", hat.agi_bonus)); }
                    if hat.int_bonus > 0 { stats.push(format!("+{} INT", hat.int_bonus)); }
                    ui.label(stats.join(", "));
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !is_unlocked {
                        ui.label(format!("Unlock at Node {}", hat.unlock_node_id));
                    } else if is_owned {
                        if ui.button("EQUIP").clicked() {
                            self.status_msg = format!("Select an operator to equip {}.", hat.name);
                            // TODO: Open a modal/overlay for operator selection if needed,
                            // or just use the currently selected slime if one is selected.
                            if let Some(slime_id) = self.selected_slime_id {
                                if let Err(e) = self.state.equip_hat(slime_id, hat.id) {
                                    self.status_msg = format!("Error: {}", e);
                                } else {
                                    self.status_msg = format!("Equipped {}!", hat.name);
                                    self.persist();
                                }
                            } else {
                                self.status_msg = "Please select an operator in the Roster first.".to_string();
                            }
                        }
                    } else {
                        let btn_text = format!("BUY ({} Scrap)", hat.scrap_cost);
                        if ui.button(btn_text).clicked() {
                            match self.state.purchase_hat(hat.id, &self.state.unlocked_nodes) {
                                Ok(_) => {
                                    self.status_msg = format!("Purchased {}!", hat.name);
                                    self.persist();
                                }
                                Err(e) => {
                                    self.status_msg = format!("Purchase failed: {}", e);
                                }
                            }
                        }
                    }
                });
            });
        });
    }
}
