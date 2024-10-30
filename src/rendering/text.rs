use std::ffi::c_void;

use gl::*;
use glu::GLsizei;
use glu::GLuint;
use glu_sys::*;
use sdl2::surface::Surface;

pub struct TextRenderer {
    texture_id: GLuint,
}

impl TextRenderer {
    pub fn new() -> Self {
        let mut texture_id = 0;
        unsafe {
            glActiveTexture(TEXTURE0);
            glGenTextures(1, &mut texture_id);
            glBindTexture(TEXTURE_2D, texture_id);
            glTexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            glTexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
        }
        TextRenderer { texture_id }
    }

    pub fn render(text_surface: &Surface) {
        unsafe {
            let texture_id = 0;
            glDisable(TEXTURE_2D);
            glBindTexture(TEXTURE_2D, texture_id);

            // Ensure proper alignment and size for our texture
            let width = (text_surface.width() as usize / 4 * 4) as GLsizei;
            let height = (text_surface.height() as usize / 4 * 4) as GLsizei;

            let pixels = text_surface.with_lock(|pixels| pixels.as_ptr() as *const c_void);

            glTexImage2D(
                TEXTURE_2D,
                0,
                RGBA as i32,
                width,
                height,
                0,
                RGBA,
                UNSIGNED_BYTE,
                pixels,
            );

            // Draw the textured quad directly onto your screen
            glBegin(QUADS);
            glTexCoord2f(0.0, 0.0);
            glVertex3f(-(text_surface.width() as f32) / 2.0 + 100.0, -200.0, 0.0);
            glTexCoord2f(1.0, 0.0);
            glVertex3f((text_surface.width() as f32) / 2.0 + 100.0, -200.0, 0.0);
            glTexCoord2f(1.0, 1.0);
            glVertex3f(
                text_surface.width() as f32 / 2.0 + 100.0,
                text_surface.height() as f32 / 2.0,
                0.0,
            );
            glTexCoord2f(0.0, 1.0);
            glVertex3f(
                -(text_surface.width() as f32) / 2.0 + 100.0,
                text_surface.height() as f32 / 2.0,
                0.0,
            );
            glEnd();
        }
    }

    pub fn cleanup(&mut self) {
        unsafe {
            glDisable(TEXTURE_2D);
            glBindTexture(TEXTURE_2D, self.texture_id);
            glDeleteTextures(1, &self.texture_id);
        }
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        TextRenderer::new()
    }
}
