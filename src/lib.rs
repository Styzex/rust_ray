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
//! // This is the function for anything that is game specific
//! fn run_game() -> Result<(), String> {
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
//!         // Initialize map
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
//!
//! // Main function with error printing
//! pub fn main() {
//!     if let Err(error) = run_game() {
//!         eprintln!("Error running game: {}", error);
//!         std::process::exit(1);
//!     }
//! }
//! ```
//!
//! # Features
//!
//! - **fake 3D Rendering**: OpenGL-based fake 3D rendering system (Real 3D might be implemented in the future)
//! - **2D Rendering**: OpenGL-based 2D rendering system
//! - **Map Support**: Custom RRM (Rust Ray Map) format for level design
//! - **Input Processing**: Keyboard and mouse input processing
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
//! │   │   └── font.ttf
//! │   └── maps/
//! │       └── your_maps.rrm
//! ├── src/
//! │   ├── main.rs
//! │   └── game_state.rs (Or any other file that the game needs)
//! └── Cargo.toml
//! ```
//!
//! # Dependencies
//!
//! Add these dependencies to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sdl2 = { version = "^0.37.0", features = ["bundled", "static-link"]}
//! rust_ray = "^0.1.35" # Adjust version as needed
//! ```
//!
//! # Architecture
//!
//! The engine is organized into several main modules:
//!
//! - [`rendering`]: Handles all graphics rendering operations
//!   - [`map`]: Map rendering functionality
//!   - [`player`]: Player rendering systems
//!   - [`renderer`]: Core 2D and 3D rendering systems
//!
//! - [`rrm`]: Custom Rust Ray Map (RRM) format support
//!   - [`rrm_support`]: Handles loading and parsing of .rrm map files
//!
//! - [`utilities`]: Common utility functions and helpers
//!   - [`opengl`]: OpenGL utility functions
//!
//! - [`window`]: Window management and creation
//!   - [`sdl_window`]: SDL2 window implementation and OpenGL context management

pub mod rendering;
pub mod rrm;
pub mod utilities;
pub mod window;

pub use rrm::rrm_support::{map_initialize, MAP_CUBE_SIZE, MAP_DATA, MAP_HEIGHT, MAP_WIDTH};
pub use utilities::opengl::{clear_screen, setup_viewport};
pub use window::sdl_window::{check_gl_error, SdlWindow};
