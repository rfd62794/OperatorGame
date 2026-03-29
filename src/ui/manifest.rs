use crate::ui::{OperatorApp, CONTENT_WIDTH, SIDE_GUTTER, CARD_INNER_MARGIN, CARD_GAP};
use crate::render::garden_bridge::egui_rect_to_bounds;
use eframe::egui;

impl OperatorApp {
    pub(crate) fn render_manifest(&mut self, ui: &mut egui::Ui) {
        if self.state.slimes.is_empty() {
            ui.label("No slimes in the Bio-Manifest. Go to the Incubator or use `operator hatch`.");
            return;
        }

        let staged = self.staged_operators.clone();
        let selected_mission_id = self.selected_mission;
        let mut toggle_stage: Option<uuid::Uuid> = None;

        let card_width = 380.0; // SDD-038 ┬º4: CONTENT_WIDTH - (SIDE_GUTTER * 2)

        // Use a vertical layout for the card list inside the scroll area
        egui::ScrollArea::vertical()
            .id_source("roster_scroll")
            .auto_shrink([false, false]) // Fill available space
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, CARD_GAP);
                    
                    for op in &self.state.slimes {
                        let (stage_clicked, card_clicked, hat_clicked) = render_operator_card(ui, op, &staged, selected_mission_id, card_width);
                        if stage_clicked {
                            toggle_stage = Some(op.genome.id);
                        }
                        if card_clicked {
                            self.selected_slime_id = Some(op.genome.id);
                        }
                        if hat_clicked {
                            self.selected_slime_id = Some(op.genome.id);
                            self.active_tab = crate::platform::BottomTab::Map;
                            self.map_sub_tab = crate::platform::MapSubTab::Quartermaster;
                        }
                    }
                });
            });

        if let Some(id) = toggle_stage {
            if self.staged_operators.contains(&id) {
                self.staged_operators.remove(&id);
            } else if self.staged_operators.len() < 3 {
                self.staged_operators.insert(id);
            } else {
                self.status_msg = "Max squad size is 3 slimes.".to_string();
            }
        }
    }

    pub(crate) fn render_incubator(&mut self, ui: &mut egui::Ui) {
        if self.state.incubating.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(egui::RichText::new("INCUBATOR OFFLINE").strong().size(20.0).color(egui::Color32::GRAY));
                ui.add_space(8.0);
                ui.label("No active synthesis detected.");
                ui.add_space(20.0);
                
                if ui.button(egui::RichText::new("Γ₧ò SELECT BREEDING PAIR").strong()).clicked() {
                    self.status_msg = "Breeding UI requires Phase G.1 (Dice Drama) integration.".to_string();
                }
                ui.add_space(8.0);
                ui.label(egui::RichText::new("(Select donors from Bio-Manifest to begin)").small().color(egui::Color32::from_gray(120)));
            });
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
                    ui.horizontal(|ui| { // TODO: reflow if clips on narrow
                        ui.label(egui::RichText::new("≡ƒº¬").size(24.0));
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(format!("Synthesizing: {}", inc.operator.name())).strong());
                            if ready {
                                ui.colored_label(egui::Color32::from_rgb(100, 255, 100), "READY FOR HARVEST");
                                if ui.button("Harvest").clicked() {
                                    harvested.push(inc.operator.genome.id);
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
                if harvested.contains(&inc.operator.genome.id) {
                    new_slimes.push(inc.operator.clone());
                    false
                } else {
                    true
                }
            });
            self.state.world_map.startled_level += 0.10 * new_slimes.len() as f32; // ADR-015: Hoot & Holler resonance
            self.state.slimes.extend(new_slimes);
            
            // Re-sync garden to ensure new slimes wander immediately
            self.garden = crate::garden::Garden::from_operators(&self.state.slimes, egui_rect_to_bounds(ui.max_rect()));
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
                            self.garden = crate::garden::Garden::from_operators(&self.state.slimes, egui_rect_to_bounds(ui.max_rect()));
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
                                self.garden = crate::garden::Garden::from_operators(&self.state.slimes, egui_rect_to_bounds(ui.max_rect()));
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

    pub(crate) fn render_slime_detail(&mut self, ui: &mut egui::Ui) {
        if let Some(id) = self.selected_slime_id {
            if let Some(op) = self.state.slimes.iter().find(|s| s.genome.id == id) {
                // Task D.3 Render slide-in detail panel
                // Header area: wrap if name is long
                ui.horizontal_wrapped(|ui| {
                    if ui.button("ΓùÇ Back").clicked() {
                        self.selected_slime_id = None;
                    }
                    ui.label(egui::RichText::new(&op.genome.name).strong().size(18.0));
                });
                ui.separator();
                
                ui.vertical(|ui| {
                    ui.set_max_width(ui.available_width());
                    
                    ui.heading("VITAL STATISTICS");
                    egui::Grid::new("slime_stats_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Level:");
                            ui.label(format!("{} (XP: {}/{})", op.level, op.total_xp, op.xp_to_next()));
                            ui.end_row();

                            ui.label("Base HP:");
                            ui.label(format!("{}", op.genome.base_hp));
                            ui.end_row();

                            ui.label("Base Mind:");
                            ui.label(format!("{}", op.genome.base_mind));
                            ui.end_row();
                        });
                    
                    ui.add_space(8.0);
                    
                    ui.heading("CULTURAL GENOME");
                    ui.add(egui::Label::new(egui::RichText::new(format!("Dominance: {:?}", op.genome.dominant_culture()))).wrap(true));
                    ui.add(egui::Label::new(egui::RichText::new(format!("Pattern: {:?}", op.genome.pattern))).wrap(true));
                    
                    let expr = op.genome.culture_alleles.dominant.0;
                    render_culture_spectrum(ui, &expr, 0.8);
                });
            }
        } else {
            self.selected_slime_id = None;
        }
    }
}

/// Renders an individual operator as a card (Phase F.1).
/// Returns true if the [STAGE] button was clicked, and true if the card was tapped.
fn render_operator_card(
    ui: &mut egui::Ui,
    op: &crate::models::Operator,
    staged: &std::collections::HashSet<uuid::Uuid>,
    _selected_mission_id: Option<uuid::Uuid>,
    card_width: f32,
) -> (bool, bool, bool) {
    let mut stage_clicked = false;
    let mut card_clicked = false;
    let mut hat_clicked = false;
    let genome = &op.genome;
    let is_staged = staged.contains(&genome.id);
    let (cr, cg, cb) = crate::genetics::culture_display_color(&genome.culture_alleles);
    let color = egui::Color32::from_rgb(cr, cg, cb);
    
    // Card Frame
    let frame_color = if is_staged {
        egui::Color32::from_rgb(30, 50, 40) // Subtle green for staged
    } else {
        egui::Color32::from_rgb(26, 26, 34) // Panel background
    };

    let _response = egui::Frame::none()
        .fill(frame_color)
        .stroke(egui::Stroke::new(1.0, if is_staged { egui::Color32::GREEN } else { egui::Color32::from_gray(60) }))
        .inner_margin(egui::Margin::same(8.0))
        .rounding(egui::Rounding::same(4.0))
        .show(ui, |ui| {
            ui.set_max_width(380.0); // SDD-038 ┬º4 width enforcement
            ui.set_max_width(380.0);
            
            // Row 1: Header
            ui.columns(2, |cols| {
                // Left column: name
                cols[0].label(
                    egui::RichText::new(&genome.name)
                        .strong()
                        .size(14.0)
                        .color(color)
                );
                // Right column: buttons
                cols[1].with_layout(
                    egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button(">").clicked() { card_clicked = true; }
                        let is_injured = matches!(op.state, crate::models::SlimeState::Injured(_));
                        let is_dispatched = matches!(op.state, crate::models::SlimeState::Deployed(_));
                        if is_injured {
                            ui.add_enabled(false, egui::Button::new("INJURED").small());
                        } else if is_dispatched {
                            ui.add_enabled(false, egui::Button::new("DEPLOYED").small());
                        } else {
                            let lbl = if is_staged { "STAGED" } else { "STAGE" };
                            if ui.small_button(lbl).clicked() { stage_clicked = true; }
                        }
                    }
                );
            });

            // Row 2: Lv | Stage Label | Pattern (┬º3: 11pt sub-status)
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Lv: {}", op.level)).size(11.0).color(egui::Color32::from_gray(160)));
                ui.add_space(4.0);
                
                let stage = op.life_stage();
                let stage_color = match stage {
                    crate::genetics::LifeStage::Hatchling => egui::Color32::from_rgb(160, 160, 160),
                    crate::genetics::LifeStage::Juvenile  => egui::Color32::from_rgb(140, 200, 140),
                    crate::genetics::LifeStage::Young     => egui::Color32::from_rgb(100, 200, 180),
                    crate::genetics::LifeStage::Prime     => egui::Color32::from_rgb(220, 180,  80),
                    crate::genetics::LifeStage::Veteran   => egui::Color32::from_rgb(200, 140,  60),
                    crate::genetics::LifeStage::Elder     => egui::Color32::from_rgb(180, 120, 220),
                };
                ui.label(egui::RichText::new(stage.to_string().to_uppercase())
                    .size(11.0)
                    .color(stage_color));
                
                ui.add_space(4.0);
                ui.label(egui::RichText::new(format!("{:?}", genome.pattern)).size(11.0).color(egui::Color32::from_gray(120)));
            });

            // Row 3: XP bar (4dp height, no percentage)
            let needed = op.xp_to_next().max(1) as f32;
            let current_tier = (op.total_xp as f32) % needed;
            let xp_pct = (current_tier / needed).clamp(0.0, 1.0);
            ui.add(egui::ProgressBar::new(xp_pct).desired_height(4.0)); // SDD-038 ┬º4

            // Row 4: Vitals (┬º3: 11pt stats row)
            let (s, a, i, _, _, _) = op.total_stats();
            let hp = op.genome.base_hp;
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("STR:{}", s)).size(11.0).color(egui::Color32::from_gray(180)));
                ui.add_space(4.0);
                ui.label(egui::RichText::new(format!("AGI:{}", a)).size(11.0).color(egui::Color32::from_gray(180)));
                ui.add_space(4.0);
                ui.label(egui::RichText::new(format!("INT:{}", i)).size(11.0).color(egui::Color32::from_gray(180)));
                ui.add_space(8.0);
                ui.label(egui::RichText::new(format!("HP: {:.0}", hp)).size(11.0).color(egui::Color32::LIGHT_GRAY));
            });

            // Row 5: Hat action (┬º3: 12pt buttons)
            ui.horizontal(|ui| {
                if let Some(hat_id) = op.equipped_hat {
                    let catalog = crate::models::Hat::catalog();
                    if let Some(hat) = catalog.iter().find(|h| h.id == hat_id) {
                        if ui.button(egui::RichText::new(format!("≡ƒÄ⌐ {}", hat.name))
                            .size(12.0)
                            .color(egui::Color32::from_rgb(220, 220, 100))).clicked() 
                        {
                            hat_clicked = true;
                        }
                    }
                } else {
                    if ui.button(egui::RichText::new("Γ₧ò EQUIP HAT").size(12.0).color(egui::Color32::GRAY)).clicked() {
                        hat_clicked = true;
                    }
                }
            });
        });

    (stage_clicked, card_clicked, hat_clicked)
}

/// Helper for Sprint 6: Renders a segmented horizontal bar representing the Culture genome.
fn render_culture_spectrum(ui: &mut egui::Ui, expr: &[f32; 9], opacity: f32) {
    let segments = crate::genetics::spectrum_segments(expr, 0.05);
    if segments.is_empty() { return; }

    let height = 4.0;
    let width = ui.available_width().min(120.0); // Limit to 120 but shrink if needed
    
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
