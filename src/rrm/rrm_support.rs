//! # Game Engine Map Module
//!
//! This module provides functionality for initializing and managing game maps.
//! It includes functions for reading map data from files and allocating map variables.

use std::path::Path;
use std::*;

/// The size of the map (width and height).
static mut SIZE: i32 = 8;

/// The width of the map in tiles.
pub static mut MAP_WIDTH: usize = 8;

/// The height of the map in tiles.
pub static mut MAP_HEIGHT: usize = 8;

/// The size of each map cube in pixels or units.
pub static mut MAP_CUBE_SIZE: f32 = 64.0;

/// Your maps data, where 0 represents an empty tile and 1 represents a wall.
pub static mut MAP_DATA: Vec<Vec<usize>> = Vec::new();

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
    unsafe {
        MAP_DATA = vec![vec![0; SIZE as usize]; SIZE as usize];
    }

    let path = Path::new(&folder_location);

    if path.exists() {
        match read_dir_to_string(String::from(folder_location)) {
            Ok(files) => {
                for file_info in files {
                    let path_to_file = Path::new(&file_info.path);
                    if path_to_file.extension().and_then(|s| s.to_str()) == Some("rrm") {
                        read_map_data(path_to_file);
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
pub fn read_dir_to_string(folder_location: String) -> io::Result<Vec<FileInfo>> {
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
pub fn read_map_data(path_to_file: &Path) {
    let file_data = fs::read_to_string(path_to_file).unwrap();
    let data = file_data.lines().collect::<Vec<&str>>();

    unsafe {
        if let Some(size_str) = data[0].split('=').last() {
            SIZE = size_str.trim().parse().unwrap_or(8);
        }

        let array_lines = &data[1..];

        let mut new_map = vec![vec![0u8; SIZE as usize]; SIZE as usize];
        for (i, line) in array_lines.iter().enumerate() {
            if i >= SIZE as usize {
                break;
            }

            let trimmed = line.trim().trim_matches(|c| c == '[' || c == ']');
            let nums: Vec<u8> = trimmed
                .split(',')
                .map(|n| n.trim().parse().unwrap_or(0))
                .collect();
            for (j, &nums) in nums.iter().enumerate().take(SIZE as usize) {
                new_map[i][j] = nums;
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
pub fn allocate_variables(new_map: Vec<Vec<u8>>) {
    unsafe {
        MAP_WIDTH = SIZE as usize;
        MAP_HEIGHT = SIZE as usize;

        MAP_DATA = new_map
            .iter()
            .map(|row| row.iter().map(|&val| val as usize).collect())
            .collect();
    }
}
