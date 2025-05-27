#![allow(unused_assignments, dead_code)]

mod config;
mod file_utils;
mod utils;

use std::time::Instant;
use clap::Parser;
use log::{info, debug};
use crate::file_utils::fs_utils;
use crate::file_utils::matcher::{DIRS, FILES};

fn main(){
    let start = Instant::now();
    // Parse CLI arguments and set up logging
    let cli = config::cli::Cli::parse();
    config::cli::setup_logger(&cli); 
    
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
    file_utils::remover::remove_file_on_path(files.to_vec());
}


