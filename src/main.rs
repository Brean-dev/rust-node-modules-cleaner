#![allow(unused_assignments, dead_code)]
// Mods
mod config;
mod file_utils;
mod tui;
mod utils;
// Native crates
use crate::config::cli;
use crate::file_utils::fs_utils;
use crate::file_utils::matcher::{DIRS, FILES};
// Non native crates
use clap::Parser;
use log::{debug, error, info};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // Parse CLI arguments and set up logging
    let cli = config::cli::Cli::parse();
    config::cli::setup_logger(&cli);

    if !*cli::TUI_MODE.lock().unwrap() {
        // Do the actual work
        fs_utils::walk_directories();

        let dirs = DIRS.lock().unwrap();
        let files = FILES.lock().unwrap();
        let elapsed = start.elapsed();

        debug!("Found {} directories", dirs.len());
        debug!("Found {} files\n", files.len());

        // Show up to 10 entries from the files collection
        let entries_to_show = std::cmp::min(10, files.len());
        info!("Showing first {} file entries:", entries_to_show);
        for (i, file) in files.iter().take(entries_to_show).enumerate() {
            info!("  {}. {}", i + 1, file.display());
        }
        println!("\n");
        info!("Total execution time: {:.2?}", elapsed);
        file_utils::remover::remove_file_on_path(
            files.to_vec(),
            dirs.to_vec(),
            cli.debug.unwrap_or(false),
        );
    } else {
        if let Err(e) = tui::engine::run_tui() {
            error!("TUI error: {}", e);
        }
    }
}
