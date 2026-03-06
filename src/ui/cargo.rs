// src/ui/cargo.rs
use eframe::egui;
use crate::ui::OperatorApp;

impl OperatorApp {
    pub(crate) fn render_cargo(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("── CARGO BAY ──").strong().size(14.0));
        ui.add_space(8.0);
        
        egui::Grid::new("cargo_grid").num_columns(3).spacing([40.0, 16.0]).show(ui, |ui| {
            ui.label(egui::RichText::new("RESOURCE").strong().color(egui::Color32::GRAY));
            ui.label(egui::RichText::new("QUANTITY").strong().color(egui::Color32::GRAY));
            ui.label(egui::RichText::new("UTILITY").strong().color(egui::Color32::GRAY));
            ui.end_row();

            ui.label(egui::RichText::new("Biomass [GEL]").color(egui::Color32::LIGHT_GREEN));
            ui.label(format!("{} L", self.state.inventory.biomass));
            if ui.button("Refine").clicked() {}
            ui.end_row();

            ui.label(egui::RichText::new("Scrap [MTL]").color(egui::Color32::LIGHT_BLUE));
            ui.label(format!("{} kg", self.state.inventory.scrap));
            if ui.button("Repair Ship").clicked() {}
            ui.end_row();
            
            ui.label(egui::RichText::new("Reagents").color(egui::Color32::GOLD));
            ui.label(format!("{} Units", self.state.inventory.reagents));
            if ui.button("Force Mutate").clicked() {}
            ui.end_row();
        });
    }
}
