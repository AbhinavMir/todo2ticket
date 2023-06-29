use std::env;
use std::fs;
use std::io::{self, BufRead};

fn search_directory(directory_path: &str) {
    // Read the directory entries
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // Recursively search subdirectories
                    search_directory(&path.to_string_lossy());
                } else if path.is_file() {
                    // Search file for "TODO"
                    search_file(&path);
                }
            }
        }
    }
}

fn search_file(file_path: &std::path::Path) {
    // Read the file line by line
    if let Ok(file) = fs::File::open(file_path) {
        let reader = io::BufReader::new(file);
        for (line_number, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                // Check if the line contains "TODO"
                if line.contains("@TODO") {
                    println!("{}:{}: {}", file_path.display(), line_number + 1, line);
                }
            }
        }
    }
}
