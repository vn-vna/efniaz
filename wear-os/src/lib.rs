//! Wear OS NativeActivity bootstrap; rendering and navigation are not implemented.

#[cfg(target_os = "android")]
use android_activity::{AndroidApp, MainEvent, PollEvent};

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
#[rustfmt::skip]
fn android_main(app: AndroidApp) {
    let mut graphics = None;
    let mut destroyed = false;

    println!("Initialized watch native app");
    while !destroyed {
        app.poll_events(None, |event| match event {
            PollEvent::Main(MainEvent::InitWindow { .. }) => android::initialize_graphics(&app, &mut graphics),
            PollEvent::Main(MainEvent::RedrawNeeded { .. }) => android::draw_frame(&graphics),
            PollEvent::Main(MainEvent::TerminateWindow { .. }) => android::destroy_graphics(&mut graphics),
            PollEvent::Main(MainEvent::Destroy) => android::cleanup(&mut destroyed),
            _ => {}
        });
    }
}

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
mod gfx;

#[cfg(target_os = "android")]
mod streets;
