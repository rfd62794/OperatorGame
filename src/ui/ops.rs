// src/ui/ops.rs

use eframe::egui;
use uuid::Uuid;

use crate::ui::OperatorApp;

impl OperatorApp {
    pub(crate) fn render_active_ops(&mut self, ui: &mut egui::Ui) {
        // Render AAR Result panel if one is pending
        if let Some(aar) = self.pending_aar.clone() {
            ui.label(egui::RichText::new("── AFTER ACTION REPORT ──").strong().size(16.0));
            ui.add_space(8.0);

            egui::Frame::none()
                .fill(egui::Color32::from_rgb(20, 20, 25))
                .stroke(egui::Stroke::new(1.0, aar.outcome_color))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.set_max_width(ui.available_width());
                    ui.add(egui::Label::new(egui::RichText::new(&aar.mission_name).color(egui::Color32::WHITE)).wrap(true));
                    ui.add(egui::Label::new(egui::RichText::new(&aar.outcome_label).size(18.0).strong().color(aar.outcome_color)).wrap(true));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(format!("OBJECTIVES: {} / {} TARGETS ELIMINATED", aar.targets_defeated, aar.total_targets))
                        .color(egui::Color32::from_rgb(180, 180, 180))
                        .size(11.0));
                    ui.add_space(8.0);

                    // ScrollArea contains all log content — DISMISS button is NOT inside it
                    egui::ScrollArea::vertical()
                        .max_height(280.0)
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            if let Some(reward) = &aar.reward {
                                ui.label(egui::RichText::new(format!("REWARDS: {}", reward)).strong().color(egui::Color32::from_rgb(255, 215, 0)));
                                ui.add_space(4.0);
                            }
                            ui.label(format!("Squad Experience Gained: {}", aar.xp_gained));

                            // ── FIELD PROMOTIONS ─────────────────────────────────────
                            // G.6: Only rendered when at least one operator leveled up.
                            // Stage transitions (e.g. Hatchling→Juvenile) rendered in gold.
                            // Same-stage level-ups rendered in cyan.
                            if !aar.level_up_events.is_empty() {
                                ui.add_space(8.0);
                                ui.label(egui::RichText::new("── FIELD PROMOTIONS ────────────────────")
                                    .strong()
                                    .color(egui::Color32::from_rgb(100, 200, 255))
                                    .size(12.0));

                                for evt in &aar.level_up_events {
                                    ui.add_space(4.0);

                                    // Gold = stage transition (biologically significant moment)
                                    // Cyan = same-stage level-up (operational improvement)
                                    let name_color = if evt.stage_transition {
                                        egui::Color32::from_rgb(255, 215, 80)
                                    } else {
                                        egui::Color32::from_rgb(140, 220, 255)
                                    };

                                    // Row 1: ⬆ Name   Lv X → Y   STAGE_OLD → STAGE_NEW
                                    let header = if evt.stage_transition {
                                        format!(
                                            "⬆ {:12}  Lv {} → {}    {} → {}",
                                            evt.operator_name,
                                            evt.old_level, evt.new_level,
                                            evt.old_stage.to_string().to_uppercase(),
                                            evt.new_stage.to_string().to_uppercase(),
                                        )
                                    } else {
                                        format!(
                                            "⬆ {:12}  Lv {} → {}    [{}]",
                                            evt.operator_name,
                                            evt.old_level, evt.new_level,
                                            evt.new_stage.to_string().to_uppercase(),
                                        )
                                    };
                                    ui.add(egui::Label::new(
                                        egui::RichText::new(header)
                                            .strong()
                                            .color(name_color)
                                            .size(12.0)
                                    ).wrap(true));

                                    // Row 2: Stat delta line
                                    let d = &evt.stat_delta;
                                    let fmt_delta = |v: i32| if v >= 0 { format!("+{}", v) } else { format!("{}", v) };
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "   STR {} ({})   AGI {} ({})   INT {} ({})",
                                            fmt_delta(d.str_change), fmt_delta(d.str_change),
                                            fmt_delta(d.agi_change), fmt_delta(d.agi_change),
                                            fmt_delta(d.int_change), fmt_delta(d.int_change),
                                        ))
                                        .color(egui::Color32::from_rgb(180, 220, 180))
                                        .size(11.0)
                                    );
                                }

                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("────────────────────────────────────────")
                                    .color(egui::Color32::from_rgb(60, 80, 100))
                                    .size(11.0));
                            }
                            // ── END FIELD PROMOTIONS ─────────────────────────────────

                            if !aar.injured_names.is_empty() {
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("CASUALTIES:").strong().color(egui::Color32::from_rgb(255, 100, 100)));
                                for inj in &aar.injured_names {
                                    ui.label(format!(" • {} (Medical Leave)", inj));
                                }
                            }

                            ui.add_space(8.0);
                            ui.collapsing("View Encounter Dice Rolls", |ui| {
                                for roll in &aar.roll_lines {
                                    ui.label(roll);
                                }
                            });
                        });

                    // DISMISS button is OUTSIDE the ScrollArea — always visible
                    ui.add_space(12.0);
                    if ui.button(egui::RichText::new("ACKNOWLEDGE & DISMISS").size(14.0)).clicked() {
                        self.pending_aar = None;
                    }
                });
            return;
        }

        ui.label(egui::RichText::new("── ACTIVE OPERATIONS ──").strong().size(14.0));
        ui.add_space(4.0);

        if self.state.deployments.is_empty() {
            ui.label("No active operations.");
            return;
        }

        let mut to_resolve: Vec<Uuid> = vec![];

        egui::ScrollArea::vertical()
            .id_source("active_ops_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
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
                            let is_orphan = mission_name.contains("[ORPHANED]");

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&mission_name).strong());
                                if is_orphan {
                                    ui.label(egui::RichText::new("⚠️ ORPHANED").color(egui::Color32::from_rgb(255, 100, 100)).small());
                                }
                            });

                            if is_orphan {
                                ui.label(egui::RichText::new("Save corruption detected. This operation can still be resolved, but original details are lost.").small().color(egui::Color32::GRAY));
                            }

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
            });

        for dep_id in to_resolve {
            self.resolve_deployment(dep_id);
        }
        self.state.deployments.retain(|d| !d.resolved);
    }

    pub(crate) fn render_launch_bar(&mut self, ui: &mut egui::Ui) {
        ui.separator();

        if self.pending_aar.is_some() {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Awaiting commander acknowledgment of AAR...").color(egui::Color32::YELLOW));
            });
            return;
        }

        ui.horizontal_wrapped(|ui| {
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
