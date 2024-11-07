//! # SDL Window Module
//!
//! This module provides functionality for creating and managing an SDL window with OpenGL context.
//!
//! ## Features
//!
//! - OpenGL 4.3 compatibility profile
//! - Double buffering support
//! - VSync support
//! - Mouse grab functionality
//! - Event handling through SDL2
//!
//! ## Example
//!
//! ```rust
//! use your_engine_name::window::SdlWindow;
//!
//! fn main() -> Result<(), String> {
//!     let width = 800;
//!     let height = 600;
//!     let window = SdlWindow::new("Game Window", width, height)?;
//!     Ok(())
//! }
//! ```

extern crate sdl2;

use sdl2::{
    video::{GLContext, SwapInterval, Window},
    EventPump,
};

/// A window implementation using SDL2 with OpenGL context.
///
/// This struct manages the lifecycle of an SDL window along with its OpenGL context
/// and event handling capabilities. It provides a safe wrapper around SDL2's window
/// functionality with game-focused features enabled by default.
///
/// # Fields
///
/// * `window` - The underlying SDL window instance
/// * `event_pump` - SDL event pump for handling window and input events
/// * `_gl_context` - OpenGL context (kept alive through struct ownership)
///
/// # Example
///
/// ```rust
/// use your_engine_name::window::SdlWindow;
///
/// fn init_window() -> Result<SdlWindow, String> {
///     let window = SdlWindow::new("Game Window", 800, 600)?;
///     Ok(window)
/// }
/// ```
pub struct SdlWindow {
    /// The SDL window.
    pub window: Window,
    /// The SDL event pump for handling events.
    pub event_pump: EventPump,
    _gl_context: GLContext, // Keep the context alive as long as the window exists
}

impl SdlWindow {
    /// Creates a new SDL window with OpenGL context.
    ///
    /// This function initializes SDL2, sets up the video subsystem with OpenGL support,
    /// and creates a window with the specified parameters. It also configures several
    /// game-specific settings like mouse grab and VSync.
    ///
    /// # Arguments
    ///
    /// * `title` - The window title that will appear in the title bar
    /// * `width` - The width of the window in pixels
    /// * `height` - The height of the window in pixels
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `SdlWindow` if successful, or an error
    /// message if initialization fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_engine_name::window::SdlWindow;
    ///
    /// let window = SdlWindow::new("Game Window", 1920, 1080)?;
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - SDL2 initialization fails
    /// - Video subsystem initialization fails
    /// - Window creation fails
    /// - OpenGL context creation fails
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
            .map_err(|err| err.to_string())?;

        let gl_context = window.gl_create_context()?;
        window.gl_make_current(&gl_context)?;

        sdl.mouse().set_relative_mouse_mode(true);
        window.set_mouse_grab(true);

        video_subsystem.gl_set_swap_interval(SwapInterval::VSync)?;

        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        let event_pump = sdl.event_pump()?;

        Ok(SdlWindow {
            window,
            event_pump,
            _gl_context: gl_context,
        })
    }

    /// Swaps the window's OpenGL buffers.
    ///
    /// This method should be called after rendering each frame to display
    /// the rendered content on screen. It synchronizes with the monitor's
    /// refresh rate if VSync is enabled.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn render_loop(window: &SdlWindow) {
    ///     // Render frame...
    ///     window.swap_window();
    /// }
    /// ```
    pub fn swap_window(&self) {
        self.window.gl_swap_window();
    }
}

/// Checks for OpenGL errors and prints them to stderr.
///
/// This utility function should be used during development to catch and
/// diagnose OpenGL-related issues. It checks the OpenGL error flag and
/// prints any errors it finds.
///
/// # Safety
///
/// This function uses unsafe code to interact with OpenGL through FFI.
///
/// # Examples
///
/// ```rust
/// // After performing OpenGL operations
/// check_gl_error();
/// ```
pub fn check_gl_error() {
    unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            eprintln!("OpenGL error: {}", error);
        }
    }
}
