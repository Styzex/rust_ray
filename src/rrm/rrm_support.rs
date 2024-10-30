//! # Game Engine Map Module
//!
//! This module provides functionality for initializing and managing game maps.
//! It includes functions for reading map data from files and allocating map variables.

// --- Imports ---
use std::path::Path;
use std::*;

// --- Variables ---
// Defaults

/// The size of the map (width and height).
static mut SIZE: i32 = 8;

/// The width of the map in tiles.
pub static mut MAP_WIDTH: usize = 8;

/// The height of the map in tiles.
pub static mut MAP_HEIGHT: usize = 8;

/// The size of each map cube in pixels or units.
pub static mut MAP_CUBE_SIZE: f32 = 64.0;

/// The actual map data, where 0 represents an empty tile and 1 represents a wall.
pub static mut MAP_DATA: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

// --- Logic ---
/// Represents information about a file in the map directory.
pub struct FileInfo {
    /// The name of the file.
    pub name: String,
    /// The full path to the file.
    pub path: String,
}

/// Initializes the map by reading data from files in the specified folder.
///
/// # Arguments
///
/// * `folder_location` - A string slice that holds the path to the folder containing map files.
///
/// # Returns
///
/// * `io::Result<()>` - Ok(()) if successful, or an error if the folder doesn't exist or there's an issue reading files.
pub fn map_initialize(folder_location: &str) -> io::Result<()> {
    let path = Path::new(&folder_location);

    if path.exists() {
        match read_dir_to_string(String::from(folder_location)) {
            Ok(files) => {
                for file_info in files {
                    let path_to_file = Path::new(&file_info.path);
                    if path_to_file.extension().and_then(|s| s.to_str()) == Some("rrm") {
                        read_map_data(path_to_file);
                        // We only need to process one file, so we can break here
                        break;
                    }
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path does not exist",
        ))
    }
}

/// Reads the contents of a directory and returns file information.
///
/// # Arguments
///
/// * `folder_location` - A String that holds the path to the folder to read.
///
/// # Returns
///
/// * `io::Result<Vec<FileInfo>>` - A vector of FileInfo structs if successful, or an error if there's an issue reading the directory.
fn read_dir_to_string(folder_location: String) -> io::Result<Vec<FileInfo>> {
    let path = Path::new(&folder_location);
    let mut file_names = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        file_names.push(FileInfo {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path().to_string_lossy().into_owned(),
        });
    }

    Ok(file_names)
}

/// Reads map data from a file and updates the global map variables.
///
/// # Arguments
///
/// * `path_to_file` - A reference to a Path that points to the map file to read.
///
/// # Safety
///
/// This function uses unsafe code to modify static mutable variables. Ensure that it's called in a single-threaded context or with proper synchronization.
fn read_map_data(path_to_file: &Path) {
    let file_data = fs::read_to_string(path_to_file).unwrap();
    let data = file_data.lines().collect::<Vec<&str>>();

    unsafe {
        // Parse SIZE from first line
        if let Some(size_str) = data[0].split('=').last() {
            SIZE = size_str.trim().parse().unwrap_or(8);
        }

        // Skip first line (SIZE) and parse array lines
        let array_lines = &data[1..];

        // Create a new map array and fill it with the data
        let mut new_map = [[0u8; 8]; 8];
        for (i, line) in array_lines.iter().enumerate() {
            if i >= 8 {
                break;
            }

            let trimmed = line.trim().trim_matches(|c| c == '[' || c == ']');
            let nums: Vec<u8> = trimmed
                .split(',')
                .map(|n| n.trim().parse().unwrap_or(0))
                .collect();

            for (j, &num) in nums.iter().enumerate().take(8) {
                new_map[i][j] = num;
            }
        }
        allocate_variables(new_map);
    }
}

/// Allocates and updates the global map variables with new map data.
///
/// # Arguments
///
/// * `new_map` - A 2D array representing the new map data to be allocated.
///
/// # Safety
///
/// This function uses unsafe code to modify static mutable variables. Ensure that it's called in a single-threaded context or with proper synchronization.
fn allocate_variables(new_map: [[u8; 8]; 8]) {
    unsafe {
        MAP_WIDTH = SIZE as usize;
        MAP_HEIGHT = SIZE as usize;
        MAP_CUBE_SIZE = (SIZE * SIZE) as f32;
        MAP_DATA = new_map; // This line is important
    }
}
