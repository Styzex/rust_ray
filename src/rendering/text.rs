use glu_sys::*;
use rusttype::{Font, Point, Scale};
use sdl2::pixels::Color;
use std::fs::File;
use std::io::Read;

pub struct TextRenderer {
    font: Font<'static>,
    color: Color,
}

impl TextRenderer {
    pub fn new(font_path: &str, color: Color) -> Result<Self, Box<dyn std::error::Error>> {
        // Read font file
        let mut file = File::open(font_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Parse font
        let font = Font::try_from_vec(buffer).ok_or("Error loading font")?;

        Ok(TextRenderer { font, color })
    }

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

    pub fn render_centered_text(&self, screen_width: f32, y: f32, text: &str, scale: f32) {
        let text_width = self.calculate_text_width(text, scale);
        let x = (screen_width - text_width) / 2.0;
        self.render_text(x, y, text, scale);
    }
}
