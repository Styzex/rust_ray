pub mod map;
pub mod player;
pub mod renderer;

pub use player::draw_player;
pub use renderer::{gl_clear_screen, render_2d, render_3d, setup_viewport};