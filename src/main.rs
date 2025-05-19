mod cli;
mod config;
mod fs_utils;
mod matcher;
mod utils;

use std::time::Instant;
use clap::Parser;
use log::info;

fn main() {
    let start = Instant::now();
    
    // Parse CLI arguments and set up logging
    let cli = cli::Cli::parse();
    cli::setup_logger(&cli);
    
    // Example of using the optional argument
    if let Some(args) = &cli.arguments {
        info!("Using provided arguments: {}", args);
    }
    
    // Walk directories to find node_modules
    fs_utils::walk_directories();
    
    let elapsed = start.elapsed();
    info!("Total execution time: {:.2?}", elapsed);
}
