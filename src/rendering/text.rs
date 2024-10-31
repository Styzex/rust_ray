extern crate sdl2;

use glu_sys::*;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::*;

pub struct BitmapFont {
    char_width: u32,
    char_height: u32,
    char_map: HashMap<char, (u32, u32)>,
    color: Color,
}

impl BitmapFont {
    pub fn new(char_width: u32, char_height: u32, color: Color) -> Self {
        let mut char_map = HashMap::new();
        let chars = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        let cols = 16; // Assuming a fixed width of 16 characters per row

        for (i, c) in chars.chars().enumerate() {
            let x = (i as u32 % cols) * char_width;
            let y = (i as u32 / cols) * char_height;
            char_map.insert(c, (x, y));
        }

        BitmapFont {
            char_width,
            char_height,
            char_map,
            color,
        }
    }

    pub fn render_text(&self, x: f32, y: f32, text: &str, scale: f32) {
        unsafe {
            // Set the color for the text
            glColor3f(
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
            );

            for (i, c) in text.chars().enumerate() {
                if let Some(&(char_x, char_y)) = self.char_map.get(&c) {
                    let scaled_width = self.char_width as f32 * scale;
                    let scaled_height = self.char_height as f32 * scale;

                    // Set the position for the character
                    let pos_x = x + (i as f32 * scaled_width);
                    let pos_y = y;

                    // Render the character as a textured quad or a rectangle
                    glBegin(GL_QUADS);
                    glTexCoord2f(
                        char_x as f32 / (self.char_width * 16) as f32,
                        char_y as f32 / (self.char_height * 16) as f32,
                    );
                    glVertex2f(pos_x, pos_y);
                    glTexCoord2f(
                        char_x as f32 / (self.char_width * 16) as f32,
                        (char_y + self.char_height) as f32 / (self.char_height * 16) as f32,
                    );
                    glVertex2f(pos_x, pos_y + scaled_height);
                    glTexCoord2f(
                        (char_x + self.char_width) as f32 / (self.char_width * 16) as f32,
                        (char_y + self.char_height) as f32 / (self.char_height * 16) as f32,
                    );
                    glVertex2f(pos_x + scaled_width, pos_y + scaled_height);
                    glTexCoord2f(
                        (char_x + self.char_width) as f32 / (self.char_width * 16) as f32,
                        char_y as f32 / (self.char_height * 16) as f32,
                    );
                    glVertex2f(pos_x + scaled_width, pos_y);
                    glEnd();
                }
            }
        }
    }
}
