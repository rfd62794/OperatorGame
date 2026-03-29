use eframe::egui;
use crate::ui::{OperatorApp, COLOR_PRIMARY, COLOR_TEXT, COLOR_SURFACE_LOW, COLOR_SURFACE_HIGH};

/// Render a single styled sub-tab button for the sidebar.
pub fn sub_tab_button(ui: &mut egui::Ui, label: &str, is_active: bool) -> bool {
    let text_color = if is_active { COLOR_PRIMARY } else { COLOR_TEXT };
    let fill_color = if is_active { COLOR_SURFACE_LOW } else { egui::Color32::TRANSPARENT };

    // SIDEBAR: font size 15.0 — do not reduce without designer approval
    let btn = egui::Button::new(
        egui::RichText::new(label)
            .size(15.0)
            .color(text_color),
    )
    .fill(fill_color)
    .stroke(egui::Stroke::NONE)
    .rounding(egui::Rounding::ZERO);

    ui.add(btn).clicked()
}

/// Render a styled section header for the sidebar.
pub fn sidebar_header(ui: &mut egui::Ui, title: &str) {
    ui.add_space(8.0);
    // SIDEBAR: font size 15.0 — do not reduce without designer approval
    ui.label(
        egui::RichText::new(title)
            .size(15.0)
            .color(COLOR_PRIMARY)
            .strong(),
    );
    // Thin colored separator line via the painter
    let sep_rect = ui.available_rect_before_wrap();
    ui.painter().hline(
        sep_rect.min.x..=sep_rect.max.x,
        sep_rect.min.y,
        egui::Stroke::new(1.0, COLOR_SURFACE_HIGH),
    );
    ui.add_space(6.0);
}

impl OperatorApp {
    pub(crate) fn render_sub_tabs(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 4.0);

            match self.active_tab {
                crate::platform::BottomTab::Roster => {
                    sidebar_header(ui, "Roster");

                    if sub_tab_button(ui, "Collection", self.roster_sub_tab == crate::platform::RosterSubTab::Collection) {
                        self.roster_sub_tab = crate::platform::RosterSubTab::Collection;
                        self.status_msg.clear();
                    }
                    if sub_tab_button(ui, "Breeding", self.roster_sub_tab == crate::platform::RosterSubTab::Breeding) {
                        self.roster_sub_tab = crate::platform::RosterSubTab::Breeding;
                        self.status_msg.clear();
                    }
                    if sub_tab_button(ui, "Recruit", self.roster_sub_tab == crate::platform::RosterSubTab::Recruit) {
                        self.roster_sub_tab = crate::platform::RosterSubTab::Recruit;
                        self.status_msg.clear();
                    }
                    if sub_tab_button(ui, "Squad", self.roster_sub_tab == crate::platform::RosterSubTab::Squad) {
                        self.roster_sub_tab = crate::platform::RosterSubTab::Squad;
                        self.status_msg.clear();
                    }
                }

                crate::platform::BottomTab::Missions => {
                    sidebar_header(ui, "Missions");
                    if sub_tab_button(ui, "Active", self.missions_sub_tab == crate::platform::MissionsSubTab::Active) {
                        self.missions_sub_tab = crate::platform::MissionsSubTab::Active;
                        self.status_msg.clear();
                    }
                    if sub_tab_button(ui, "Quests", self.missions_sub_tab == crate::platform::MissionsSubTab::QuestBoard) {
                        self.missions_sub_tab = crate::platform::MissionsSubTab::QuestBoard;
                        self.status_msg.clear();
                    }
                }

                crate::platform::BottomTab::Map => {
                    sidebar_header(ui, "Map");
                    if sub_tab_button(ui, "Zones", self.map_sub_tab == crate::platform::MapSubTab::Zones) {
                        self.map_sub_tab = crate::platform::MapSubTab::Zones;
                        self.status_msg.clear();
                    }
                    if sub_tab_button(ui, "Shop", self.map_sub_tab == crate::platform::MapSubTab::Quartermaster) {
                        self.map_sub_tab = crate::platform::MapSubTab::Quartermaster;
                        self.status_msg.clear();
                    }
                }

                crate::platform::BottomTab::Logs => {
                    sidebar_header(ui, "LOGS");
                    if sub_tab_button(ui, "Missions", self.logs_sub_tab == crate::platform::LogsSubTab::MissionHistory) {
                        self.logs_sub_tab = crate::platform::LogsSubTab::MissionHistory;
                        self.status_msg.clear();
                    }
                    if sub_tab_button(ui, "Culture", self.logs_sub_tab == crate::platform::LogsSubTab::CultureHistory) {
                        self.logs_sub_tab = crate::platform::LogsSubTab::CultureHistory;
                        self.status_msg.clear();
                    }
                }
            }
        });
    }
}
