use egui;
use crate::ui::OperatorApp;
use crate::models::{Hat, HatId};

impl OperatorApp {
    pub(crate) fn render_quartermaster(&mut self, ui: &mut egui::Ui) {
        let mut action = None;

        ui.vertical(|ui| {
            ui.heading("QUARTERMASTER");
            ui.add_space(8.0);
            ui.label(format!("Available Scrap (Scrap): {}", self.state.inventory.scrap));
            ui.add_space(16.0);

            egui::ScrollArea::vertical()
                .id_source("hat_shop")
                .show(ui, |ui| {
                let catalog = Hat::catalog();
                for hat in catalog {
                    if let Some(a) = self.render_hat_item(ui, hat) {
                        action = Some(a);
                    }
                    ui.add_space(8.0);
                    ui.separator();
                }
            });
        });

        if let Some(a) = action {
            match a {
                HatAction::Buy(id) => {
                    let unlocked = self.state.unlocked_nodes.clone();
                    match self.state.purchase_hat(id, &unlocked) {
                        Ok(_) => {
                            self.status_msg = "Purchased!".to_string();
                            self.persist();
                        }
                        Err(e) => self.status_msg = format!("Error: {}", e),
                    }
                }
                HatAction::Equip(id) => {
                    if let Some(slime_id) = self.selected_slime_id {
                        match self.state.equip_hat(slime_id, id) {
                            Ok(_) => {
                                self.status_msg = "Equipped!".to_string();
                                self.persist();
                            }
                            Err(e) => self.status_msg = format!("Error: {}", e),
                        }
                    } else {
                        self.status_msg = "Select a slime in Roster first.".to_string();
                    }
                }
            }
        }
    }

    fn render_hat_item(&self, ui: &mut egui::Ui, hat: Hat) -> Option<HatAction> {
        let mut action = None;
        let is_unlocked = self.state.unlocked_nodes.contains(&hat.unlock_node_id) || hat.unlock_node_id == 0;
        let is_owned = self.state.hat_inventory.contains(&hat.id) || 
                       self.state.slimes.iter().any(|s| s.equipped_hat == Some(hat.id));

        ui.group(|ui| {
            ui.horizontal(|ui| {
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
                        ui.label(format!("Unlock at {}", hat.unlock_node_id));
                    } else if is_owned {
                        if ui.button("EQUIP").clicked() {
                            action = Some(HatAction::Equip(hat.id));
                        }
                    } else {
                        if ui.button(format!("BUY ({} Scrap)", hat.scrap_cost)).clicked() {
                            action = Some(HatAction::Buy(hat.id));
                        }
                    }
                });
            });
        });
        action
    }
}

enum HatAction {
    Buy(crate::models::HatId),
    Equip(crate::models::HatId),
}
