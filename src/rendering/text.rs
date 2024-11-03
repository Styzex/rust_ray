//! Text rendering module for game engine.
//!
//! This module provides functionality for rendering text in OpenGL using rusttype.
//! It supports basic text rendering operations including centered text alignment
//! and custom fonts.

use glu_sys::*;
use rusttype::{Font, Point, Scale};
use sdl2::pixels::Color;
use std::fs::File;
use std::io::Read;

/// A text rendering system that handles font loading and text drawing operations.
///
/// The `TextRenderer` provides an interface for rendering text in an OpenGL context
/// using the rusttype library for font handling. It supports custom fonts and colors.
///
/// # Examples
///
/// ```
/// use sdl2::pixels::Color;
///
/// let renderer = TextRenderer::new("assets/fonts/arial.ttf", Color::RGB(255, 255, 255))?;
///
/// // Render text at specific coordinates
/// renderer.render_text(100.0, 100.0, "Hello World!", 16.0);
///
/// // Render centered text
/// renderer.render_centered_text(800.0, 100.0, "Centered Text", 24.0);
/// ```
pub struct TextRenderer {
    /// The loaded font used for rendering
    font: Font<'static>,
    /// The color used for rendering text
    color: Color,
}

impl TextRenderer {
    /// Creates a new TextRenderer instance with the specified font and color.
    ///
    /// # Arguments
    ///
    /// * `font_path` - Path to the font file to be loaded
    /// * `color` - The color to use for rendering text
    ///
    /// # Returns
    ///
    /// Returns a Result containing either the new TextRenderer instance or an error
    /// if the font loading fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The font file cannot be opened
    /// * The font file cannot be read
    /// * The font data is invalid or corrupted
    pub fn new(font_path: &str, color: Color) -> Result<Self, Box<dyn std::error::Error>> {
        // Read font file
        let mut file = File::open(font_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Parse font
        let font = Font::try_from_vec(buffer).ok_or("Error loading font")?;

        Ok(TextRenderer { font, color })
    }

    /// Calculates the total width of a text string when rendered with the given scale.
    ///
    /// # Arguments
    ///
    /// * `text` - The text string to measure
    /// * `scale` - The font scale to use for measurement
    ///
    /// # Returns
    ///
    /// Returns the width of the text in pixels as a f32.
    fn calculate_text_width(&self, text: &str, scale: f32) -> f32 {
        let scale = Scale::uniform(scale);

        // Sum up the actual width of each glyph
        text.chars()
            .map(|c| {
                let glyph = self.font.glyph(c).scaled(scale);
                glyph.h_metrics().advance_width
            })
            .sum()
    }

    /// Renders text at the specified coordinates with the given font size.
    ///
    /// This method renders text using OpenGL point primitives. The text is rendered
    /// using the current font and color settings.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate where the text should start
    /// * `y` - The y coordinate where the text should start
    /// * `text` - The text string to render
    /// * `font_size` - The size of the font to use for rendering
    ///
    /// # Safety
    ///
    /// This method uses unsafe OpenGL calls and should be called only when there is
    /// a valid OpenGL context.
    pub fn render_text(&self, x: f32, y: f32, text: &str, font_size: f32) {
        unsafe {
            // Set text color
            glColor3f(
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
            );

            let scale = Scale::uniform(font_size);
            let v_metrics = self.font.v_metrics(scale);
            let glyphs: Vec<_> = self
                .font
                .layout(
                    text,
                    scale,
                    Point {
                        x,
                        y: y + v_metrics.ascent,
                    },
                )
                .collect();

            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // Draw the glyph's outline
                    glyph.draw(|x, y, v| {
                        // You might want to implement more sophisticated rendering here
                        if v > 0.5 {
                            glBegin(gl::POINTS);
                            glVertex2f(
                                bounding_box.min.x as f32 + x as f32,
                                bounding_box.min.y as f32 + y as f32,
                            );
                            glEnd();
                        }
                    });
                }
            }
        }
    }

    /// Renders text centered horizontally on the screen at the specified y coordinate.
    ///
    /// # Arguments
    ///
    /// * `screen_width` - The total width of the screen or rendering area
    /// * `y` - The y coordinate where the text should be rendered
    /// * `text` - The text string to render
    /// * `scale` - The font scale to use for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// renderer.render_centered_text(800.0, 100.0, "Game Over", 32.0);
    /// ```
    pub fn render_centered_text(&self, screen_width: f32, y: f32, text: &str, scale: f32) {
        let text_width = self.calculate_text_width(text, scale);
        let x = (screen_width - text_width) / 2.0;
        self.render_text(x, y, text, scale);
    }
}
