#[link(name = "EGL")]
#[link(name = "GLESv2")]
unsafe extern "C" {
    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    fn glClear(mask: u32);
}

const GL_COLOR_BUFFER_BIT: u32 = 0x4000;

pub struct Graphics {
    display: khronos_egl::Display,
    context: khronos_egl::Context,
    surface: khronos_egl::Surface,
}

impl Graphics {
    pub fn new(app: &android_activity::AndroidApp) -> Result<Self, Box<dyn std::error::Error>> {
        let window = app
            .native_window()
            .ok_or("No android window found for application")?;

        let display = unsafe { khronos_egl::API.get_display(khronos_egl::DEFAULT_DISPLAY) }
            .ok_or("Display not found")?;
        let (context, surface) = Self::initialize_opengl_es(window, display)?;

        Ok(Self {
            display,
            context,
            surface,
        })
    }

    pub fn draw(&self) -> Result<(), khronos_egl::Error> {
        unsafe {
            glClearColor(0.05, 0.25, 0.85, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);
        }
        khronos_egl::API.swap_buffers(self.display, self.surface)
    }

    fn initialize_opengl_es(
        window: android_activity::ndk::native_window::NativeWindow,
        display: khronos_egl::Display,
    ) -> Result<(khronos_egl::Context, khronos_egl::Surface), Box<dyn std::error::Error>> {
        khronos_egl::API.initialize(display)?;
        khronos_egl::API.bind_api(khronos_egl::OPENGL_ES_API)?;

        #[rustfmt::skip]
        let config = khronos_egl::API
            .choose_first_config(
                display,
                &[
                    khronos_egl::SURFACE_TYPE,      khronos_egl::WINDOW_BIT,
                    khronos_egl::RENDERABLE_TYPE,   khronos_egl::OPENGL_ES2_BIT,
                    khronos_egl::RED_SIZE,          8,
                    khronos_egl::GREEN_SIZE,        8,
                    khronos_egl::BLUE_SIZE,         8,
                    khronos_egl::NONE,
                ],
            )?
            .ok_or("No configuration selected for OpenGL ES")?;

        let visual =
            khronos_egl::API.get_config_attrib(display, config, khronos_egl::NATIVE_VISUAL_ID)?;
        window.set_buffers_geometry(0, 0, Some(visual.into()))?;
        let context = khronos_egl::API.create_context(
            display,
            config,
            None,
            &[khronos_egl::CONTEXT_CLIENT_VERSION, 2, khronos_egl::NONE],
        )?;
        let surface = unsafe {
            khronos_egl::API.create_window_surface(
                display,
                config,
                window.ptr().as_ptr().cast(),
                None,
            )
        }?;
        khronos_egl::API.make_current(display, Some(surface), Some(surface), Some(context))?;
        khronos_egl::API.swap_interval(display, 1)?; // Enable VSync
        Ok((context, surface))
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        let _ = khronos_egl::API.make_current(self.display, None, None, None);
        let _ = khronos_egl::API.destroy_surface(self.display, self.surface);
        let _ = khronos_egl::API.destroy_context(self.display, self.context);
        let _ = khronos_egl::API.terminate(self.display);
    }
}
