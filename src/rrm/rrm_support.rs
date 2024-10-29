// --- Imports ---
use std::fs::*;
use std::path::Path;
use std::*;

// --- Variables ---
// Defaults
static mut SIZE: i32 = 8;
pub static mut MAP_WIDTH: usize = 8;
pub static mut MAP_HEIGHT: usize = 8;
pub static mut MAP_CUBE_SIZE: f32 = 64.0;
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
struct FileInfo {
    name: String,
    path: String,
}

pub fn map_initialize(folder_location: &str) -> io::Result<()> {
    // Added return type
    let path = Path::new(&folder_location);

    if path.exists() {
        match read_dir_to_string(String::from(folder_location)) {
            Ok(files) => {
                // Need to handle the Result explicitly
                for file_info in files {
                    // Changed variable name to avoid confusion with struct
                    // You can filter by extension here if needed
                    let path_to_file = Path::new(&file_info.path); // Access struct field
                    if let Ok(content) = read_to_string(path_to_file) {
                        read_map_data(path_to_file);
                    }
                }
                Ok(()) // Return success
            }
            Err(e) => Err(e), // Handle the error case
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path does not exist",
        ))
    }
}

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

            if nums.len() == 8 {
                new_map[i].copy_from_slice(&nums);
            }
        }
        allocate_variables(new_map);
    }
}

fn allocate_variables(new_map: [[u8; 8]; 8]) {
    unsafe {
        MAP_WIDTH = SIZE as usize;
        MAP_HEIGHT = SIZE as usize;
        MAP_CUBE_SIZE = (SIZE * SIZE) as f32;
        MAP_DATA = new_map;
    }
}
