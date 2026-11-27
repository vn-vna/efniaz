//! Android NativeActivity bootstrap; map preparation and transfer are not implemented.

#[cfg(target_os = "android")]
use android_activity::{AndroidApp, MainEvent, PollEvent};

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        "EFNIAZ Companion",
        eframe::NativeOptions {
            android_app: Some(app),
            ..eframe::NativeOptions::default()
        },
        Box::new(|_| Ok(Box::new(app::CompanionApp::new()))),
    )?;

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn windows_main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        "EFNIAZ Companion",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(app::CompanionApp::new()))),
    )?;

    Ok(())
}

pub(crate) mod app;
