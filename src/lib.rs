//! A modern game engine built in Rust with OpenGL and SDL2.
//!
//! This game engine provides a modular architecture for building 2D and 3D games,
//! with components for rendering, window management, and a custom map format (RRM).
//!
//! # Getting Started
//!
//! To create a basic game using this engine, you'll need to:
//! 1. Set up a window
//! 2. Create a game state manager
//! 3. Implement the main game loop
//!
//! Here's a minimal example:
//!
//! ```rust
//! use rust_ray::{SdlWindow, map_initialize, check_gl_error};
//! use sdl2::event::Event;
//!
//! fn main() -> Result<(), String> {
//!     // 1. Initialize window and game state
//!     let window = SdlWindow::new("My Game", 1280, 720)?;
//!     
//!     // 2. Set up initial game state
//!     let player_x = 1280.0 / 4.0;
//!     let player_y = 720.0 / 4.0;
//!     let game_state = GameStateManager::new(
//!         1280,     // screen_width
//!         720,      // screen_height
//!         player_x, // initial player x
//!         player_y, // initial player y
//!         0.0,      // initial player angle
//!         0.0,      // initial mouse x
//!         0.0       // initial mouse y
//!     )?;
//!
//!     // 3. Run game loop
//!     'running: loop {
//!         // Handle events
//!         for event in window.event_pump.poll_iter() {
//!             match event {
//!                 Event::Quit { .. } => break 'running,
//!                 _ => {
//!                     if game_state.update(&event) {
//!                         break 'running;
//!                     }
//!                 }
//!             }
//!         }
//!
//!         // Initialize map (if using RRM maps)
//!         map_initialize("./assets/maps")?;
//!
//!         // Render current frame
//!         game_state.render();
//!         window.swap_window();
//!         check_gl_error();
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
//! - **State Management**: Built-in game state system (MainMenu, Playing, Paused, etc.)
//! - **3D Rendering**: OpenGL-based 3D rendering system
//! - **Map Support**: Custom RRM (Rust Ray Map) format for level design
//! - **Input Handling**: Keyboard and mouse input processing
//! - **Window Management**: SDL2-based window handling with OpenGL context
//!
//! # Project Structure
//!
//! To use this engine, your project should have the following structure:
//!
//! ```text
//! my_game/
//! ├── assets/
//! │   ├── font/
//! │   │   └── Mario.ttf  # Or your preferred font
//! │   └── maps/
//! │       └── your_maps.rrm
//! ├── src/
//! │   ├── main.rs
//! │   └── game_state.rs
//! └── Cargo.toml
//! ```
//!
//! # Dependencies
//!
//! Add these dependencies to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sdl2 = "0.35"
//! rust_ray = { path = "../rust_ray" }  # Adjust path as needed
//! ```
//!
//! # Architecture
//!
//! The engine is organized into several main modules:
//!
//! - [`rendering`]: Handles all graphics rendering operations
//!   - [`rendering::map`]: Map rendering functionality
//!   - [`rendering::player`]: Player rendering systems
//!   - [`rendering::renderer`]: Core 2D and 3D rendering systems
//!
//! - [`rrm`]: Custom Rust Ray Map (RRM) format support
//!   - [`rrm::rrm_support`]: Handles loading and parsing of .rrm map files
//!
//! - [`utilities`]: Common utility functions and helpers
//!   - [`utilities::opengl`]: OpenGL utility functions
//!
//! - [`window`]: Window management and creation
//!   - [`window::sdl_window`]: SDL2 window implementation and OpenGL context management

pub mod rendering;
pub mod rrm;
pub mod utilities;
pub mod window;

pub use window::sdl_window::{check_gl_error, SdlWindow};
