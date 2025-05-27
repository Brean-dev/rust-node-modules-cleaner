use crate::config::cli::ask_yes_no;
use crate::utils::read_size::{get_paths_size};
use std::fs;
use std::path::PathBuf;
use log::{info, error, warn};

pub fn remove_file_on_path(files: Vec<PathBuf>, dirs: Vec<PathBuf>, debug_mode: bool) {
    // Combine all paths for a single size calculation
    let mut all_paths = Vec::with_capacity(files.len() + dirs.len());
    all_paths.extend(files.iter().cloned());
    all_paths.extend(dirs.iter().cloned());

    match get_paths_size(&all_paths) {
        Ok((total_bytes, total_mb)) => {
            info!("Total target size: {} bytes ({:.2} MB)", total_bytes, total_mb);
            info!("Files: {}, Directories: {}", files.len(), dirs.len());
        }
        Err(e) => {
            error!("Failed to calculate total size: {}", e);
        }
    }

    if debug_mode {
        warn!("Debug mode is ON. No files will be deleted.");
        for file in &files {
            // info!("[DEBUG] Would delete file: {}", file.display());
        }
        for dir in &dirs {
            // info!("[DEBUG] Would delete directory: {}", dir.display());
        }
        return;
    }

    if ask_yes_no("About to permanently remove files and directories from your system. Proceed?") {
        for file in files {
            if file.is_file() {
                match fs::remove_file(&file) {
                    Ok(_) => info!("Removed file: {}", file.display()),
                    Err(e) => error!("Failed to remove file {}: {}", file.display(), e),
                }
            } else {
                warn!("Not a valid file: {}", file.display());
            }
        }

        for dir in dirs {
            if dir.is_dir() {
                match fs::remove_dir_all(&dir) {
                    Ok(_) => info!("Removed directory: {}", dir.display()),
                    Err(e) => error!("Failed to remove directory {}: {}", dir.display(), e),
                }
            } else {
                warn!("Not a valid directory: {}", dir.display());
            }
        }
    } else {
        warn!("User aborted deletion.");
    }
}

