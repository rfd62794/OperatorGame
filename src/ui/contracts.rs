use eframe::egui;
use crate::ui::OperatorApp;
use crate::models::{Mission, MissionTier};

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
                // --- Part 1: Scout Missions ---
                ui.heading("SCOUT MISSIONS");
                ui.label(egui::RichText::new("Establish presence in new territories").small().color(egui::Color32::from_gray(140)));
                ui.add_space(8.0);

                let mut scout_count = 0;
                for mission in &self.state.missions {
                    if mission.is_scout {
                        let unlocked = self.state.unlocked_nodes.contains(&(mission.node_id.unwrap_or(0) as usize));
                        if !unlocked {
                            if self.render_mission_item(ui, mission, selected == Some(mission.id)) {
                                new_selection = if selected == Some(mission.id) { None } else { Some(mission.id) };
                            }
                            ui.add_space(6.0);
                            scout_count += 1;
                        }
                    }
                }
                if scout_count == 0 {
                    ui.label(egui::RichText::new("All known territories scouted.").small().italics().color(egui::Color32::from_gray(100)));
                }

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(8.0);

                // --- Part 2: Available Contracts ---
                ui.heading("AVAILABLE CONTRACTS");
                ui.label(egui::RichText::new("Node-specific operations and high-yield contracts").small().color(egui::Color32::from_gray(140)));
                ui.add_space(8.0);

                let mut contract_count = 0;
                for mission in &self.state.missions {
                    if !mission.is_scout {
                        let can_see = match mission.node_id {
                            None => true,
                            Some(nid) => self.state.unlocked_nodes.contains(&(nid as usize)),
                        };
                        if can_see {
                            if self.render_mission_item(ui, mission, selected == Some(mission.id)) {
                                new_selection = if selected == Some(mission.id) { None } else { Some(mission.id) };
                            }
                            ui.add_space(6.0);
                            contract_count += 1;
                        }
                    }
                }
                if contract_count == 0 {
                    ui.label(egui::RichText::new("No contracts available for unlocked nodes.").small().italics().color(egui::Color32::from_gray(100)));
                }
            });

        if new_selection != selected {
            self.selected_mission = new_selection;
        }
    }

    /// Helper to render a single mission card. Returns true if clicked.
    fn render_mission_item(&self, ui: &mut egui::Ui, mission: &Mission, is_selected: bool) -> bool {
        let mut clicked = false;
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
            .inner_margin(egui::Margin::same(8.0))
            .rounding(egui::Rounding::same(4.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let tier_color = match mission.tier {
                            MissionTier::Starter  => egui::Color32::from_rgb(150, 255, 150),
                            MissionTier::Standard => egui::Color32::from_rgb(150, 200, 255),
                            MissionTier::Advanced => egui::Color32::from_rgb(255, 200, 100),
                            MissionTier::Elite    => egui::Color32::from_rgb(255, 100, 255),
                        };
                        ui.label(egui::RichText::new(format!("{:?}", mission.tier)).color(tier_color).small().strong());
                        ui.label(egui::RichText::new(&mission.name).strong().size(14.0).color(egui::Color32::WHITE));
                        
                        if let Some(nid) = mission.node_id {
                            if let Some(node) = self.state.world_map.nodes.iter().find(|n| n.id as usize == nid) {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(egui::RichText::new(format!("\u{1f4cc} {}", node.name)).small().color(egui::Color32::from_gray(160)));
                                });
                            }
                        }
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
                        
                        let staged_ops: Vec<&crate::models::Operator> = self.state.slimes.iter()
                            .filter(|op| self.staged_operators.contains(&op.genome.id))
                            .collect();
                        
                        let (label, chance) = mission.calculate_success_chance(&staged_ops);
                        let chance_pct = (chance * 100.0) as u32;
                        
                        let chance_color = if chance >= 0.75 {
                            egui::Color32::from_rgb(80, 255, 120) 
                        } else if chance >= 0.50 {
                            egui::Color32::YELLOW 
                        } else {
                            egui::Color32::from_rgb(255, 100, 100)
                        };

                        ui.label(egui::RichText::new(format!("{} - {}%", label, chance_pct)).color(chance_color).strong());

                        let mut t_str = 0u32; let mut t_agi = 0u32; let mut t_int = 0u32;
                        for op in &staged_ops {
                            let (s, a, i, _, _, _) = op.total_stats();
                            t_str += s; t_agi += a; t_int += i;
                        }
                        let (s_val, r_val) = if mission.req_strength >= mission.req_agility && mission.req_strength >= mission.req_intelligence {
                            (t_str, mission.req_strength)
                        } else if mission.req_agility >= mission.req_intelligence {
                            (t_agi, mission.req_agility)
                        } else {
                            (t_int, mission.req_intelligence)
                        };
                        ui.label(egui::RichText::new(format!("| Squad: {} / Req: {}", s_val, r_val)).color(egui::Color32::from_gray(160)).small());

                        ui.label(egui::RichText::new(format!("| {}s | {}", mission.duration_secs, mission.reward)).color(egui::Color32::LIGHT_GRAY));
                    });

                    let btn_label = if is_selected { "▶ SELECTED" } else { "SELECT CONTRACT" };
                    if ui.button(btn_label).clicked() {
                        clicked = true;
                    }
                });
            });
        
        clicked
    }
}
