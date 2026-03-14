use operator::{models::GameState, ui::OperatorApp, persistence::load};
use eframe::egui;
use std::path::PathBuf;

fn main() -> eframe::Result {
    let save_path = PathBuf::from("save.json");
    let state = load(&save_path).unwrap_or_default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("OPERATOR"),
        ..Default::default()
    };

    eframe::run_native(
        "OPERATOR",
        options,
        Box::new(|cc| {
            Ok(Box::new(OperatorApp::new(cc, state, save_path)))
        }),
    )
}
