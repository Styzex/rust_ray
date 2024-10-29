use sdl2::{
    video::{GLContext, SwapInterval, Window},
    EventPump,
};

pub struct SdlWindow {
    pub window: Window,
    pub event_pump: EventPump,
    _gl_context: GLContext, // Keep the context alive as long as the window exists
}

impl SdlWindow {
    /// Makes a new sdl window.
    /// Example usage:
    /// ```
    /// let width: u32 = rendering::SCREEN_WIDTH;
    /// let height: u32 = rendering::SCREEN_HEIGHT;
    /// let title: &str = "Debug window 2D";
    /// let mut sdl_window = SdlWindow::new(title, width, height).unwrap();
    /// ```
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Compatibility);
        gl_attr.set_context_version(4, 3);
        gl_attr.set_double_buffer(true);

        let mut window = video_subsystem
            .window(title, width, height)
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        // Create GL Context and make it current
        let gl_context = window.gl_create_context()?;
        window.gl_make_current(&gl_context)?;

        let _ = sdl.mouse().set_relative_mouse_mode(true);
        window.set_mouse_grab(true);

        video_subsystem.gl_set_swap_interval(SwapInterval::VSync)?;

        // Load GL functions (you might want to use gl crate instead of glu-sys)
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        let event_pump = sdl.event_pump()?;

        Ok(SdlWindow {
            window,
            event_pump,
            _gl_context: gl_context,
        })
    }

    pub fn swap_window(&self) {
        self.window.gl_swap_window();
    }
}

/// Checks for OpenGL errors and displays them in the console.
/// Example:
/// ```
/// OpenGL error: 1283
/// ```
pub fn check_gl_error() {
    unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            eprintln!("OpenGL error: {}", error);
        }
    }
}
