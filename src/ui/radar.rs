// src/ui/radar.rs
use eframe::egui;

use crate::ui::OperatorApp;

impl OperatorApp {
    pub(crate) fn render_radar(&mut self, ui: &mut egui::Ui) {
        let frame_size = ui.available_size();
        let scale = (frame_size.x.min(frame_size.y) / 800.0).clamp(0.4, 1.0);
        
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(frame_size.x, frame_size.x), egui::Sense::hover());
        let painter = ui.painter();
        
        let map_center = rect.center();

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
            let [r, g, b, _] = crate::world_map::culture_accent(node.owner);
            let color = egui::Color32::from_rgb(r, g, b);
            
            let pos = egui::pos2(
                map_center.x + node.position.0 * scale,
                map_center.y + node.position.1 * scale,
            );
            
            // Base radius
            let mut radius = if node.id == 0 { 12.0 } else { 8.0 };
            
            // Pulse if node is contested or startled level is high
            if node.is_contested() {
                let time = ui.ctx().input(|i| i.time as f32);
                radius += (time * 5.0).sin() * 2.0;
            }

            painter.circle_filled(pos, radius, color);
            painter.circle_stroke(pos, radius + 2.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
            
            // Mouse hover tooltip
            if let Some(hover) = resp.hover_pos() {
                if (hover - pos).length() < radius + 4.0 {
                    egui::show_tooltip_at_pointer(ui.ctx(), ui.id(), |ui| {
                        ui.label(egui::RichText::new(&node.name).strong().color(color));
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
