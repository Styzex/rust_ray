//! # Rendering Module
//!
//! A comprehensive rendering system that provides both 2D and 3D rendering capabilities using OpenGL.
//! The module supports map rendering, player rendering, raycasting-based 3D views, and text rendering.
//!
//! ## Components
//!
//! - [`map`]: Handles rendering of 2D map layouts
//! - [`player`]: Player entity rendering in both 2D and 3D contexts
//! - [`renderer`]: Core rendering system with support for both 2D and 3D scenes
//! - [`text`]: Text rendering system using custom fonts
//!
//! ## Features
//!
//! - Ray-casting based 3D rendering with configurable FOV
//! - 2D overhead map view for debugging
//! - Custom text rendering with TrueType font support
//! - Perspective-correct wall rendering
//! - Debug rendering mode for development
//!
//! ## Example
//!
//! ```rust
//! use rust_ray::rendering::{
//!     render_2d,
//!     render_3d,
//!     TextRenderer,
//! };
//! use sdl2::pixels::Color;
//!
//! // Initialize text renderer
//! let text_renderer = TextRenderer::new("assets/fonts/your_font.ttf", Color::RGB(255, 255, 255))?;
//!
//! // Render a frame
//! render_3d(player_x, player_y, player_angle, screen_width, screen_height);
//!
//! // Or render in 2D mode
//! render_2d(player_x, player_y, player_angle, screen_width, screen_height);
//! ```

pub mod map;
pub mod player;
pub mod renderer;
pub mod text;

pub use player::draw_player;
pub use renderer::{render_2d, render_3d};
pub use text::TextRenderer;
