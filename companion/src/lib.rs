//! Android NativeActivity bootstrap; map preparation and transfer are not implemented.

#[cfg(target_os = "android")]
use android_activity::{AndroidApp, MainEvent, PollEvent};

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    let mut destroyed = false;
    while !destroyed {
        app.poll_events(None, |event| {
            if let PollEvent::Main(MainEvent::Destroy) = event {
                destroyed = true;
            }
        });
    }
}
