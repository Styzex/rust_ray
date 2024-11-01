use glu_sys::*;

/// Sets up the OpenGL viewport.
///
/// # Safety
///
/// This function uses unsafe OpenGL calls and should be used carefully.
///
/// # Arguments
///
/// * `screen_width` - The width of the screen.
/// * `screen_height` - The height of the screen.
pub unsafe fn setup_viewport(screen_width: i32, screen_height: i32) {
    glViewport(0, 0, screen_width, screen_height);
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    gluOrtho2D(0.0, screen_width as f64, screen_height as f64, 0.0);
    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
}

/// Clears the OpenGL screen.
///
/// # Safety
///
/// This function uses unsafe OpenGL calls and should be used carefully.
pub unsafe fn clear_screen() {
    glClearColor(0.0, 0.0, 0.0, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
}
