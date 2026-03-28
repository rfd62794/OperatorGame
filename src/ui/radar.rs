// src/ui/radar.rs
use eframe::egui;

use crate::ui::OperatorApp;

impl OperatorApp {
    pub(crate) fn render_radar(&mut self, ui: &mut egui::Ui) {
        let screen_rect = ui.ctx().screen_rect();
        let safe_area = crate::platform::read_window_insets();
        let safe_rect = safe_area.apply(screen_rect);
        
        let available_size = ui.available_size();
        if available_size.x < 50.0 || available_size.y < 50.0 {
            return;
        }

        // Map sizing: 640dp diameter base. 
        // We subtract 16dp from the available width to ensure breathing room on the sides.
        let map_dim = (safe_rect.width() - 16.0).min(safe_rect.height()).min(400.0);
        let scale = map_dim / 640.0;
        let scaled_radius = 320.0 * scale;

        // The "bottom tab system" sits at safe_rect.bottom() - TAB_BAR_HEIGHT
        let tab_bar_top = safe_rect.bottom() - crate::platform::TAB_BAR_HEIGHT;
        
        // Target: Horizontally centered ON SCREEN, sitting 18dp above the tab bar (shifted 10px up from G.2)
        let map_center = egui::pos2(
            screen_rect.center().x,
            tab_bar_top - scaled_radius - 18.0
        );

        // We still allocate to let egui know we've taken space, though we draw freehand
        let (_rect, resp) = ui.allocate_at_least(egui::vec2(available_size.x, available_size.y), egui::Sense::hover());
        let painter = ui.painter();

        // Draw Rings
        for r in 1..=3 {
            painter.circle_stroke(
                map_center,
                (r as f32 * 100.0 + 20.0) * scale,
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(100, 100, 100, 50)),
            );
        }

        // Draw Nodes
        for node in &self.state.world_map.nodes {
            let unlocked = self.state.unlocked_nodes.contains(&(node.id as usize));
            let color = if unlocked {
                let [r, g, b, _] = crate::world_map::culture_accent(node.owner);
                egui::Color32::from_rgb(r, g, b)
            } else {
                egui::Color32::from_rgb(80, 80, 80) // Grayscale locked state
            };
            
            let pos = egui::pos2(
                map_center.x + node.position.0 * scale,
                map_center.y + node.position.1 * scale,
            );
            
            // Base radius
            let mut radius = if node.id == 0 { 12.0 } else { 8.0 };
            
            // Pulse if node is contested
            if node.is_contested() {
                let time = ui.ctx().input(|i| i.time as f32);
                radius += (time * 5.0).sin() * 2.0;
            }

            // Task D.2: Premium unlocked pulse (one-time, 2.0s)
            if let Some((pulse_id, start_time)) = self.recently_unlocked_node {
                if pulse_id == node.id && unlocked {
                    let time = ui.ctx().input(|i| i.time);
                    let age = time - start_time;
                    if age > 0.0 && age < 2.0 {
                        let pulse_t = (age / 2.0) as f32;
                        let pulse_radius = radius + (pulse_t * 15.0);
                        let alpha = (1.0 - pulse_t) * 0.6;
                        painter.circle_stroke(pos, pulse_radius, egui::Stroke::new(1.5, color.gamma_multiply(alpha)));
                        ui.ctx().request_repaint(); // Ensure smooth animation during the 2s window
                    }
                }
            }

            painter.circle_filled(pos, radius, color);
            painter.circle_stroke(pos, radius + 2.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
            
            // Mouse hover tooltip
            if let Some(hover) = resp.hover_pos() {
                if (hover - pos).length() < radius + 8.0 {
                    egui::show_tooltip_at_pointer(ui.ctx(), ui.id(), |ui| {
                        ui.label(egui::RichText::new(&node.name).strong().color(color));
                        if !unlocked {
                            ui.colored_label(egui::Color32::LIGHT_GRAY, "[LOCKED - RECON REQUIRED]");
                        }
                        ui.label(format!("Culture: {:?}", node.owner));
                        ui.label(format!("Influence: {:.0}%", node.influence * 100.0));
                        ui.label(format!("DC: {}", node.difficulty_dc));
                        if node.resonance_aura > 0 {
                            ui.colored_label(egui::Color32::YELLOW, format!("Aura: +{}", node.resonance_aura));
                        }
                    });
                }
            }
        }

        // Show empty string to avoid old status string rendering but keep space
    }
}
