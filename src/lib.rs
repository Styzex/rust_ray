pub mod rendering;
pub mod rrm;
pub mod window;

pub use rendering::{
    map,
    player::draw_player,
    renderer::{gl_clear_screen, render_2d, render_3d, setup_viewport},
};
pub use rrm::rrm_support::{map_initialize, MAP_CUBE_SIZE, MAP_DATA, MAP_HEIGHT, MAP_WIDTH};
pub use window::sdl_window::{check_gl_error, SdlWindow};
