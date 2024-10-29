use glu_sys::*;
use std::f32::consts::PI;
use std::*;

use super::map::draw_map_2d;
use super::player::draw_player;
use crate::rrm::rrm_support::{MAP_CUBE_SIZE, MAP_DATA, MAP_HEIGHT, MAP_WIDTH};

// Constants
const FOV: f32 = PI / 3.0; // 60 degrees field of view
const RAY_STEP: f32 = 0.1;

pub fn render_2d(
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    screen_width: i32,
    screen_height: i32,
) {
    // let delta_x = mouse_x - player_x;
    // let delta_y = mouse_y - player_y;

    unsafe {
        // OpenGL
        setup_viewport(screen_width, screen_height);
        gl_clear_screen();

        // My own functions
        draw_player(player_x, player_y, player_angle);
        draw_map_2d();
    }
}

pub fn render_3d(
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    screen_width: i32,
    screen_height: i32,
) {
    // let delta_x = mouse_x - player_x;
    // let delta_y = mouse_y - player_y;

    unsafe {
        // OpenGL
        setup_viewport(screen_width, screen_height);
        gl_clear_screen();

        // My own functions
        let rays = draw_rays_3d(player_x, player_y, player_angle, screen_width);
        render_3d_walls(
            rays,
            player_x,
            player_y,
            player_angle,
            screen_width,
            screen_height,
        );
    }
}

pub unsafe fn setup_viewport(screen_width: i32, screen_height: i32) {
    glViewport(0, 0, screen_width, screen_height);
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    gluOrtho2D(0.0, screen_width as f64, screen_height as f64, 0.0);
    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
}

pub unsafe fn gl_clear_screen() {
    glClearColor(0.0, 0.0, 0.0, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
}

unsafe fn draw_rays_3d(
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    screen_width: i32,
) -> Vec<(f32, f32)> {
    let num_rays: usize = screen_width as usize;
    let angle_increment = FOV / num_rays as f32;
    let mut rays = Vec::with_capacity(num_rays);

    for i in 0..num_rays {
        let ray_angle = player_angle - (FOV / 2.0) + (i as f32 * angle_increment);
        let (ray_x, ray_y) = cast_ray(player_x, player_y, ray_angle);
        rays.push((ray_x, ray_y));
    }

    rays
}

unsafe fn cast_ray(player_x: f32, player_y: f32, ray_angle: f32) -> (f32, f32) {
    let delta_x = ray_angle.cos();
    let delta_y = ray_angle.sin();
    let mut ray_x = player_x;
    let mut ray_y = player_y;

    loop {
        ray_x += delta_x * RAY_STEP;
        ray_y += delta_y * RAY_STEP;

        let map_x = ray_x as usize / MAP_CUBE_SIZE as usize;
        let map_y = ray_y as usize / MAP_CUBE_SIZE as usize;

        if map_x >= MAP_WIDTH || map_y >= MAP_HEIGHT || MAP_DATA[map_y][map_x] == 1 {
            break;
        }
    }

    (ray_x, ray_y)
}

unsafe fn render_3d_walls(
    rays: Vec<(f32, f32)>,
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    screen_width: i32,
    screen_height: i32,
) {
    let num_rays: usize = screen_width as usize;
    let slice_width = screen_width as f32 / num_rays as f32;

    for (i, (ray_x, ray_y)) in rays.iter().enumerate() {
        let ray_angle = player_angle - (FOV / 2.0) + (i as f32 * FOV / num_rays as f32);
        let distance = ((ray_x - player_x).powi(2) + (ray_y - player_y).powi(2)).sqrt();
        let perpendicular_distance: f32 = (distance * (ray_angle - player_angle).cos()) as f32;

        // Calculate wall height
        let wall_height: f32 = (screen_height as f32 / perpendicular_distance) * MAP_CUBE_SIZE;

        unsafe {
            glBegin(GL_QUADS);
            glColor3f(0.0, 1.0, 0.0); // Or use the wall texture color here

            let start_x = i as f32 * slice_width;
            let end_x = start_x + slice_width;

            glTexCoord2f(start_x / screen_width as f32, 0.5);
            glVertex3f(
                start_x,
                (screen_height as f32 / 2.0) - (wall_height / 2.0),
                0.0,
            );

            glTexCoord2f(end_x / screen_width as f32, 0.5);
            glVertex3f(
                end_x,
                (screen_height as f32 / 2.0) - (wall_height / 2.0),
                0.0,
            );

            glTexCoord2f(end_x / screen_width as f32, 1.0);
            glVertex3f(
                end_x,
                (screen_height as f32 / 2.0) + (wall_height / 2.0),
                0.0,
            );

            glTexCoord2f(start_x / screen_width as f32, 1.0);
            glVertex3f(
                start_x,
                (screen_height as f32 / 2.0) + (wall_height / 2.0),
                0.0,
            );

            glEnd();
        }
    }
}
