// src/ui/manifest.rs
use chrono::Utc;
use eframe::egui;

use crate::ui::OperatorApp;

impl OperatorApp {
    pub(crate) fn render_manifest(&mut self, ui: &mut egui::Ui) {
        if self.state.slimes.is_empty() {
            ui.label("No slimes in the Bio-Manifest. Go to the Incubator or use `operator hatch`.");
            return;
        }

        let staged = self.staged_operators.clone();
        let selected_mission_id = self.selected_mission;

        egui::Grid::new("manifest_grid")
            .num_columns(6)
            .spacing([12.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                // Header
                ui.label(egui::RichText::new("ID").strong());
                ui.label(egui::RichText::new("CULTURE").strong());
                ui.label(egui::RichText::new("NAME").strong());
                ui.label(egui::RichText::new("FREQ").strong());
                ui.label(egui::RichText::new("EFFICIENCY").strong());
                ui.label(egui::RichText::new("ACTIONS").strong());
                ui.end_row();

                for genome in &self.state.slimes {
                    let is_staged = staged.contains(&genome.id);
                    let is_dispatched = false; // TODO Sync actual dispatch state
                    let can_stage = !is_dispatched && selected_mission_id.is_some();

                    let [cr, cg, cb, _] = crate::world_map::culture_accent(genome.dominant_culture());
                    let color = egui::Color32::from_rgb(cr, cg, cb);

                    ui.label(format!("#{}", &genome.id.to_string()[..5]));
                    ui.colored_label(color, format!("{:?}", genome.dominant_culture()).to_uppercase());
                    ui.label(egui::RichText::new(&genome.name).strong());
                    ui.label(format!("{:.0} Hz", genome.frequency));

                    let eff = (genome.base_hp / 100.0).clamp(0.0, 1.0) as f32;
                    ui.add(egui::ProgressBar::new(eff).desired_width(80.0));

                    // Actions
                    ui.horizontal(|ui| {
                        let mut on_cooldown = false;
                        if let Some(cooldown) = genome.synthesis_cooldown_until {
                            let secs = (cooldown - Utc::now()).num_seconds().max(0);
                            if secs > 0 {
                                ui.label(egui::RichText::new(format!("{}s", secs)).color(egui::Color32::RED));
                                on_cooldown = true;
                            }
                        }
                        
                        if !on_cooldown && can_stage {
                            let label = if is_staged { "✓ STAGED" } else { "DISPATCH" };
                            if ui.button(label).clicked() {
                                if is_staged {
                                    self.staged_operators.remove(&genome.id);
                                } else if self.staged_operators.len() < 3 {
                                    self.staged_operators.insert(genome.id);
                                } else {
                                    self.status_msg = "Max squad size is 3 slimes.".to_string();
                                }
                            }
                        }

                        if ui.button("SAMPLE").clicked() { /* Force mutation placeholder */ }
                    });
                    ui.end_row();
                }
            });
    }

    pub(crate) fn render_incubator(&mut self, ui: &mut egui::Ui) {
        if self.state.incubating.is_empty() {
            ui.label(egui::RichText::new("Incubator is empty.").italics().color(egui::Color32::GRAY));
            ui.add_space(8.0);
            ui.label("Use the command line to splice slimes:");
            ui.code("operator splice <parent_a> <parent_b> <offspring_name>");
            return;
        }

        let mut harvested = Vec::new();

        // Draw multiple test tubes
        for inc in &self.state.incubating {
            let ready = inc.is_ready();
            let rem = inc.remaining_secs();
            
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(20, 30, 40, 150))
                .inner_margin(egui::Margin::same(8.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("🧪").size(24.0));
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(format!("Synthesizing: {}", inc.genome.name)).strong());
                            if ready {
                                ui.colored_label(egui::Color32::from_rgb(100, 255, 100), "READY FOR HARVEST");
                                if ui.button("Harvest").clicked() {
                                    harvested.push(inc.genome.id);
                                }
                            } else {
                                ui.label(format!("Time remaining: {}s", rem));
                            }
                        });
                    });
                });
            ui.add_space(6.0);
        }

        if !harvested.is_empty() {
            // Move genomes from incubating to slimes
            let mut new_slimes = Vec::new();
            self.state.incubating.retain(|inc| {
                if harvested.contains(&inc.genome.id) {
                    new_slimes.push(inc.genome.clone());
                    false
                } else {
                    true
                }
            });
            self.state.world_map.startled_level += 0.10 * new_slimes.len() as f32; // ADR-015: Hoot & Holler resonance
            self.state.slimes.extend(new_slimes);
            
            // Re-sync garden to ensure new slimes wander immediately
            self.garden = crate::garden::Garden::from_genomes(&self.state.slimes, ui.max_rect());
            self.persist();
        }
    }

    pub(crate) fn render_recruit(&mut self, ui: &mut egui::Ui) {
        ui.heading("RECRUITMENT AGENCY");
        ui.add_space(8.0);
        
        // Anti-Softlock (ADR-034): Elder's Gift
        if self.state.slimes.is_empty() && self.state.bank < crate::recruitment::STANDARD_RECRUIT_COST {
            ui.group(|ui| {
                ui.heading(egui::RichText::new("EMERGENCY DIRECTIVE").color(egui::Color32::RED));
                ui.label("Roster empty. Insufficient funds. The Union cannot deploy.");
                ui.add_space(4.0);
                ui.label(egui::RichText::new("The Meadow offers a seed of the Void to restart the cycle.").italics());
                
                ui.add_space(8.0);
                if ui.button("RESONATE WITH MEADOW (FREE)").clicked() {
                    match crate::recruitment::claim_elders_gift(&mut self.state) {
                        Ok(id) => {
                            self.status_msg = format!("Granted Elder's Gift: Void Slime #{}", &id.to_string()[..5]);
                            self.persist();
                            self.garden = crate::garden::Garden::from_genomes(&self.state.slimes, ui.max_rect());
                        }
                        Err(e) => self.status_msg = e.to_string(),
                    }
                }
            });
            return;
        }

        // Standard Draft
        ui.group(|ui| {
            ui.heading("Standard Draft");
            ui.label("Requisition a new Tier 0 operator. Culture is randomized (Ember, Gale, or Marsh).");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                let cost = crate::recruitment::STANDARD_RECRUIT_COST;
                if self.state.bank >= cost {
                    if ui.button(format!("DRAFT NEW RECRUIT (${})", cost)).clicked() {
                        let name_pool = ["Rookie", "Spark", "Dusty", "Echo", "Jumper", "Mute"];
                        let r_name = name_pool[rand::random::<usize>() % name_pool.len()];
                        
                        match crate::recruitment::purchase_recruit(&mut self.state, r_name) {
                            Ok(id) => {
                                self.status_msg = format!("Drafted new recruit: {} #{}", r_name, &id.to_string()[..5]);
                                self.persist();
                                self.garden = crate::garden::Garden::from_genomes(&self.state.slimes, ui.max_rect());
                            }
                            Err(e) => self.status_msg = e.to_string(),
                        }
                    }
                } else {
                    ui.add_enabled(false, egui::Button::new(format!("DRAFT NEW RECRUIT (${})", cost)));
                    ui.label(egui::RichText::new("INSUFFICIENT FUNDS").color(egui::Color32::RED));
                }
            });
        });
    }
}
