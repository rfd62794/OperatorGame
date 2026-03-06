pub mod cli;
pub mod combat;
pub mod dice;
pub mod dungeon;
pub mod garden;
pub mod genetics;
pub mod inventory;
pub mod recruitment;
pub mod log_engine;
pub mod models;
pub mod persistence;
pub mod racing;
pub mod render;
pub mod ui;
pub mod world_map;
pub mod audio;

/// Sovereign Stub: Resolves the missing C++ ABI symbol caused by the Oboe/cpal dependency.
/// This prevents the 'dlopen failed: cannot locate symbol "__cxa_pure_virtual"' crash on boot.
#[no_mangle]
pub extern "C" fn __cxa_pure_virtual() {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use crate::persistence::{load, save_path};
    use crate::ui::OperatorApp;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    std::env::set_var("RUST_BACKTRACE", "full");
    
    // ADR-042 Revision: Use the system-provided internal data path instead of hardcoding.
    let mut path = app.internal_data_path().unwrap_or_else(|| std::path::PathBuf::from("/data/local/tmp"));
    path.push("save.json");
    
    let state = load(&path).unwrap_or_default();

    let mut options = eframe::NativeOptions::default();
    
    // The 0.27 way to pass the app handle:
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    eframe::run_native(
        "OPERATOR",
        options,
        Box::new(|cc| Box::new(OperatorApp::new(cc, state, path))),
    ).expect("Failed to run on Android");
}
