use crate::gfx;

pub fn initialize_graphics(
    app: &android_activity::AndroidApp,
    graphics: &mut Option<gfx::Graphics>,
) {
    drop(graphics.take());
    match gfx::Graphics::new(&app) {
        Ok(gpu) => {
            if let Err(err) = gpu.draw() {
                eprintln!("First OpenGL ES frame failed: {err}")
            }
            *graphics = Some(gpu)
        }

        Err(err) => {
            eprint!("Cannot initialize OpenGL ES: {err}");
        }
    }
}

pub fn draw_frame(graphics: &Option<gfx::Graphics>) {
    if let Some(gpu) = graphics {
        if let Err(err) = gpu.draw() {
            eprint!("GPU Draw error: {err}");
        }
    }
}

pub fn destroy_graphics(graphics: &mut Option<gfx::Graphics>) {
    drop(graphics.take());
}
pub fn cleanup(destroyed: &mut bool) {
    *destroyed = true;
}
