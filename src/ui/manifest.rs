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
                ui.label(egui::RichText::new("STATS & GEAR").strong());
                ui.label(egui::RichText::new("ACTIONS").strong());
                ui.end_row();

                let mut equip_action: Option<(uuid::Uuid, usize)> = None;
                let mut unequip_action: Option<(uuid::Uuid, usize)> = None;

                for genome in &self.state.slimes {
                    let is_staged = staged.contains(&genome.id);
                    let is_dispatched = matches!(genome.state, crate::models::SlimeState::Deployed(_));
                    
                    let mut is_injured = false;
                    let mut rtd_msg = String::new();
                    if let crate::models::SlimeState::Injured(until) = genome.state {
                        let remaining = until - Utc::now();
                        if remaining > chrono::Duration::zero() {
                            is_injured = true;
                            let h = remaining.num_hours();
                            let m = remaining.num_minutes() % 60;
                            rtd_msg = format!(" [INJURED — RTD {}h {}m]", h, m);
                        }
                    }

                    let can_stage = !is_dispatched && !is_injured && selected_mission_id.is_some();

                    let (cr, cg, cb) = crate::genetics::culture_display_color(&genome.culture_alleles);
                    let mut color = egui::Color32::from_rgb(cr, cg, cb);
                    
                    if is_injured {
                        // Desaturate for injured slimes
                        color = egui::Color32::from_gray(120);
                    }

                    ui.label(format!("#{}", &genome.id.to_string()[..5]));
                    
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.colored_label(color, egui::RichText::new(&genome.name).strong());
                            
                            if is_injured {
                                ui.label(egui::RichText::new(&rtd_msg)
                                    .small()
                                    .color(egui::Color32::from_rgb(255, 100, 100)));
                            } else {
                                ui.label(egui::RichText::new(genome.genetic_tier().name())
                                    .small()
                                    .color(egui::Color32::from_gray(160)));
                            }
                        });
                        
                        // Culture Tier and Name
                        ui.label(egui::RichText::new(format!("{:?}", genome.dominant_culture()).to_uppercase())
                            .small()
                            .color(color));

                        // Phase B: Culture Spectrum Bar
                        render_culture_spectrum(ui, &genome.culture_alleles.dominant.0, 1.0);
                        
                        // Phase B: Recessive Bar (Gated by Lens)
                        if self.state.lens_unlocked {
                            render_culture_spectrum(ui, &genome.culture_alleles.recessive.0, 0.4);
                        }
                    });

                    ui.label(format!("{:.0} Hz", genome.frequency));

                    ui.vertical(|ui| {
                        let (s, a, i, m, se, t) = genome.total_stats();
                        ui.label(format!("STR: {} | AGI: {} | INT: {}", s, a, i));
                        ui.label(format!("MND: {} | SEN: {} | TEN: {}", m, se, t));
                        
                        ui.horizontal(|ui| {
                            if !genome.equipped_gear.is_empty() {
                                for (idx, gear) in genome.equipped_gear.iter().enumerate() {
                                    if ui.button(format!("[-] {}", gear.name())).on_hover_text("Unequip to Cargo").clicked() {
                                        unequip_action = Some((genome.id, idx));
                                    }
                                }
                            } else {
                                ui.label(egui::RichText::new("No Gear").small().color(egui::Color32::GRAY));
                            }
                        });
                    });

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
                        
                        if is_injured {
                            ui.add_enabled(false, egui::Button::new("INJURED"));
                        } else if is_dispatched {
                            ui.add_enabled(false, egui::Button::new("DISPATCHED"));
                        } else if on_cooldown {
                            // Already handled by the cooldown timer label above, 
                            // but let's show a disabled button for consistency.
                            ui.add_enabled(false, egui::Button::new("COOLDOWN"));
                        } else if selected_mission_id.is_some() {
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
                        } else {
                            ui.add_enabled(false, egui::Button::new("SELECT MISSION"));
                        }

                        ui.menu_button("EQUIP", |ui| {
                            if self.state.inventory.gear_pool.is_empty() {
                                ui.label("No gear in Cargo.");
                            } else {
                                for (idx, gear) in self.state.inventory.gear_pool.iter().enumerate() {
                                    if ui.button(gear.name()).clicked() {
                                        equip_action = Some((genome.id, idx));
                                        ui.close_menu();
                                    }
                                }
                            }
                        });
                    });
                    ui.end_row();
                }

                if let Some((slime_id, eq_idx)) = unequip_action {
                    if let Some(slime) = self.state.slimes.iter_mut().find(|s| s.id == slime_id) {
                        let removed = slime.equipped_gear.remove(eq_idx);
                        self.state.inventory.gear_pool.push(removed);
                    }
                }
                if let Some((slime_id, inv_idx)) = equip_action {
                    if let Some(slime) = self.state.slimes.iter_mut().find(|s| s.id == slime_id) {
                        let gear = self.state.inventory.gear_pool.remove(inv_idx);
                        slime.equipped_gear.push(gear);
                    }
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
        if self.state.slimes.is_empty() && self.state.bank < crate::recruitment::STANDARD_RECRUIT_COST as i64 {
            ui.label(egui::RichText::new("CRITICAL DEBT: Bio-Alliance Emergency Funding granted (+$500)").color(egui::Color32::LIGHT_BLUE));
            if ui.button("ACCEPT BA-GRANTS").clicked() {
                self.state.bank += 500;
            }
            ui.add_space(8.0);
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
                let cost = crate::recruitment::STANDARD_RECRUIT_COST as i64;
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

/// Helper for Sprint 6: Renders a segmented horizontal bar representing the Culture genome.
fn render_culture_spectrum(ui: &mut egui::Ui, expr: &[f32; 9], opacity: f32) {
    let segments = crate::genetics::spectrum_segments(expr, 0.05);
    if segments.is_empty() { return; }

    let height = 4.0;
    let width = 120.0; // Standard manifest width for spectrum
    
    let (rect, _) = ui.allocate_at_least(egui::vec2(width, height), egui::Sense::hover());
    let mut current_x = rect.min.x;

    for (idx, weight) in segments {
        let seg_width = width * weight;
        let seg_rect = egui::Rect::from_min_max(
            egui::pos2(current_x, rect.min.y),
            egui::pos2(current_x + seg_width, rect.max.y)
        );
        
        // Map index to HSL
        let hue = crate::genetics::CULTURE_HUES[idx];
        let sat = crate::genetics::CULTURE_SATURATIONS[idx];
        let (r, g, b) = crate::genetics::hsl_to_rgb(hue, sat, 0.5);
        let color = egui::Color32::from_rgba_unmultiplied(r, g, b, (opacity * 255.0) as u8);
        
        ui.painter().rect_filled(seg_rect, 0.0, color);
        current_x += seg_width;
    }
}
