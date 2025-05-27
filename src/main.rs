#![allow(unused_assignments, dead_code)]

mod cli;
mod config;
mod fs_utils;
mod matcher;
mod utils;
mod reader;


use std::time::Instant;
use clap::Parser;
use log::info;
use crate::matcher::{DIRS, FILES};

fn main() {
    let start = Instant::now();
    // Parse CLI arguments and set up logging
    let cli = cli::Cli::parse();
    cli::setup_logger(&cli);
    
    // Example of using the optional argument
    if let Some(args) = &cli.arguments {
        info!("Using provided arguments: {}", args);
    }
    
    
    
    // Do the actual work
    fs_utils::walk_directories();
    

    let dirs = DIRS.lock().unwrap();    
    let files = FILES.lock().unwrap();    
    let elapsed = start.elapsed();
    info!("Found {} directories", dirs.len());
    info!("Found {} files\n", files.len());
    
    // Show up to 10 entries from the files collection
    let entries_to_show = std::cmp::min(10, files.len());
    info!("Showing first {} file entries:", entries_to_show);
    for (i, file) in files.iter().take(entries_to_show).enumerate() {
        info!("  {}. {}", i + 1, file.display());
    }
    println!("\n");
    info!("Total execution time: {:.2?}", elapsed); 
}
