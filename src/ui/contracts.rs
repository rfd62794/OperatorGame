// src/ui/contracts.rs
use eframe::egui;
use crate::ui::OperatorApp;

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

        self.selected_mission = new_selection;
    }
}
