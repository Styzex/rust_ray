use crate::rrm::{MAP_CUBE_SIZE, MAP_DATA, MAP_HEIGHT, MAP_WIDTH};
use glu_sys::*;
use std::*;

/// renders the map from the rrm file
pub unsafe fn draw_map_2d() {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if MAP_DATA[y][x] == 1 {
                glColor3f(1.0, 1.0, 1.0);
            } else {
                glColor4f(0.0, 0.0, 0.0, 0.0);
            }

            let x_offset = ((x as f32) * MAP_CUBE_SIZE) as i32;
            let y_offset = ((y as f32) * MAP_CUBE_SIZE) as i32;
            let map_cube_size_i32 = MAP_CUBE_SIZE as i32;

            glBegin(GL_QUADS);
            glVertex2i(x_offset + 1, y_offset + 1);
            glVertex2i(x_offset + 1, y_offset + map_cube_size_i32 - 1);
            glVertex2i(
                x_offset + map_cube_size_i32 - 1,
                y_offset + map_cube_size_i32 - 1,
            );
            glVertex2i(x_offset + map_cube_size_i32 - 1, y_offset + 1);
            glEnd();
        }
    }
}
