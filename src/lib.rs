pub mod rendering;
pub mod rrm;
pub mod utilities;
pub mod window;

pub use rendering::{
    map,
    player::draw_player,
    renderer::{render_2d, render_3d},
};
pub use rrm::rrm_support::{map_initialize, MAP_CUBE_SIZE, MAP_DATA, MAP_HEIGHT, MAP_WIDTH};
pub use utilities::opengl::{clear_screen, setup_viewport};
pub use window::sdl_window::{check_gl_error, SdlWindow};
