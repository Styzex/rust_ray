//! # Player Module
//!
//! This module provides functionality for rendering the player in a 2D environment.

use glu_sys::*;
use std::*;

/// Draws the player on the screen.
///
/// # Arguments
///
/// * `player_x` - The player's x-coordinate.
/// * `player_y` - The player's y-coordinate.
/// * `player_angle` - The player's viewing angle.
///
/// # Safety
///
/// This function uses unsafe OpenGL calls and should be used carefully.
pub fn draw_player(player_x: f32, player_y: f32, player_angle: f32) {
    let player_x = player_x as i32;
    let player_y = player_y as i32;

    // Calculate the angle to the mouse pointer
    let delta_x = player_angle.cos();
    let delta_y = player_angle.sin();

    unsafe {
        glColor3f(1.0, 1.0, 1.0);
        glPointSize(16.0);
        glBegin(GL_POINTS);
        glVertex2i(player_x, player_y);
        glEnd();

        glLineWidth(3.00);
        glBegin(GL_LINES);
        glVertex2i(player_x, player_y);
        glVertex2i(player_x + (delta_x as i32), player_y + (delta_y as i32));
        glEnd();
    }
}
