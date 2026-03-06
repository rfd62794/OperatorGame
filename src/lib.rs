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

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use crate::persistence::load;
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

#[cfg(target_os = "android")]
pub mod android_stubs {
    #[no_mangle] pub unsafe extern "C" fn __cxa_pure_virtual() { loop {} }
    #[no_mangle] pub unsafe extern "C" fn __gxx_personality_v0() { loop {} }

    // RTTI Vtables
    #[no_mangle] pub static _ZTVN10__cxxabiv117__class_type_infoE: [usize; 2] = [0, 0];
    #[no_mangle] pub static _ZTVN10__cxxabiv120__si_class_type_infoE: [usize; 2] = [0, 0];
    #[no_mangle] pub static _ZTVN10__cxxabiv121__vmi_class_type_infoE: [usize; 2] = [0, 0];
    #[no_mangle] pub static _ZTVSt12length_error: [usize; 2] = [0, 0];
    #[no_mangle] pub static _ZTVSt9exception: [usize; 2] = [0, 0];

    // RTTI Type Info
    #[no_mangle] pub static _ZTISt9exception: [usize; 2] = [0, 0];
    #[no_mangle] pub static _ZTISt12length_error: [usize; 2] = [0, 0];
    
    // Destructors
    #[no_mangle] pub unsafe extern "C" fn _ZNSt12length_errorD1Ev() { loop {} }
    #[no_mangle] pub unsafe extern "C" fn _ZNSt9exceptionD1Ev() { loop {} }
}
