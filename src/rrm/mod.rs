//! # Map System (.rrm format)
//!
//! The engine uses a custom map format (.rrm) for defining game maps. Map files follow this structure:
//! ```rrm
//! SIZE=8
//! [1, 1, 1, 1, 1, 1, 1, 1,],
//! [1, 0, 0, 0, 0, 0, 0, 1,],
//! [1, 0, 0, 0, 0, 1, 0, 1,],
//! [1, 1, 1, 1, 0, 1, 1, 1,],
//! [1, 0, 0, 1, 0, 0, 0, 1,],
//! [1, 0, 0, 0, 0, 0, 0, 1,],
//! [1, 0, 0, 1, 0, 0, 0, 1,],
//! [1, 1, 1, 1, 1, 1, 1, 1,],
//! ```

pub mod rrm_support;

pub use rrm_support::map_initialize;
pub use rrm_support::{MAP_CUBE_SIZE, MAP_DATA, MAP_HEIGHT, MAP_WIDTH};
