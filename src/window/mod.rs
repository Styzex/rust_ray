//! # Window Module
//!
//! This module provides functionality for creating and managing windows in the game engine.
//!
//! ## Architecture
//!
//! The window module is structured with the following components:
//!
//! - [`SdlWindow`]: The main window implementation using SDL2 and OpenGL
//!
//! ## Usage
//!
//! The main type you'll interact with is [`SdlWindow`], which provides window management
//! and OpenGL context handling.
//!
//! ```rust
//! use your_engine_name::window::SdlWindow;
//!
//! fn main() -> Result<(), String> {
//!     // Create a new window
//!     let window = SdlWindow::new("Game Window", 800, 600)?;
//!     
//!     // Use the window...
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - SDL2-based window management
//! - OpenGL context handling
//! - Event pump integration
//! - VSync support
//! - Mouse grab functionality
//!
//! ## Modules
//!
//! - [`sdl_window`]: Contains the SDL2-based window implementation

pub mod sdl_window;

pub use sdl_window::SdlWindow;
