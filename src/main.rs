use operator::{persistence::load, ui::OperatorApp};
use eframe::egui;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let mobile_emu = std::env::var("OPERATOR_MOBILE_EMU").is_ok();
    let save_path = PathBuf::from("save.json");
    let state = load(&save_path).unwrap_or_default();

    let window_size = if mobile_emu {
        [400.0, 800.0]  // Portrait mobile aspect ratio
    } else {
        [1200.0, 800.0] // Standard desktop layout
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_title(if mobile_emu { "OPERATOR (Mobile Simulation)" } else { "OPERATOR" }),
        ..Default::default()
    };

    eframe::run_native(
        "OPERATOR",
        options,
        Box::new(|cc| {
            Box::new(OperatorApp::new(cc, state, save_path))
        }),
    )
}
