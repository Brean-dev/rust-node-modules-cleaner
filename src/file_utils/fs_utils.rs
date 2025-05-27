use std::path::{PathBuf};
use std::time::Instant;
use jwalk::WalkDirGeneric;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::{HashSet};
use std::cell::RefCell;
use log::info;

use super::matcher;


use crate::cli;
use crate::utils;
use crate::utils::g_utils::{start_spinner, stop_spinner, get_ticks, SpinnerTheme};

// Thread-local storage for batching path operations
thread_local! {
    static LOCAL_NODE_MODULES: RefCell<Vec<String>> = RefCell::new(Vec::with_capacity(50));
}

// Optimized path ignoring function for jwalk
pub fn is_ignored<C: jwalk::ClientState>(entry: &jwalk::DirEntry<C>) -> bool {
    let path = entry.path();
   
    if !(*cli::FULL_SCAN.lock().unwrap()) {
        // Fast prefix check for common system directories
        let path_str = path.as_os_str().to_string_lossy();
        if path_str.starts_with("/proc/") || path_str.starts_with("/sys/") || 
            path_str.starts_with("/dev/") || path_str.starts_with("/run/") ||
            path_str.starts_with("/efi/") || path_str.starts_with("/usr/") {
            return true;
        }

        // Fast component check to avoid repeated iteration
        let components_iter = path.components();
        for component in components_iter {
            if let std::path::Component::Normal(name) = component {
                let name_str = name.to_string_lossy();
                if name_str == "Projects" || name_str == "opt" || name_str == ".vscode" {
                    return true;
                }
            }
        }
    }
    
    false
}

// Optimized function to convert string to PathBuf
pub fn convert_string_to_pathbuf(mutex: &MutexGuard<Vec<String>>) -> Vec<PathBuf> {
    let mut result = Vec::with_capacity(mutex.len());
    for s in mutex.iter() {
        result.push(PathBuf::from(s));
    }
    result
}

// Main directory walker function
pub fn walk_directories() {
    let start = Instant::now();
    
    let root_path = "/";

    // Stats tracking with pre-allocated capacity
    let file_count = Arc::new(AtomicUsize::new(0));
    let dir_count = Arc::new(AtomicUsize::new(0));
    let node_modules_count = Arc::new(AtomicUsize::new(0));
    let ignored_count = Arc::new(AtomicUsize::new(0));

    // Determine thread count
    let num_threads = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4);
    
    info!("Using {:?} threads for traversal starting from {:?}", num_threads, root_path);
    println!();
    let spinner = start_spinner("Walking file tree...", get_ticks(SpinnerTheme::FileWalker));
    // utils::g_utils::start_spinner(Some("Walking through your file system!".to_string()));
    // Pre-allocate collections with appropriate initial capacity
    let node_modules_locations = Arc::new(Mutex::new(Vec::with_capacity(2000)));
    let node_modules_locations_clone = Arc::clone(&node_modules_locations);

    // Use HashSet for faster path lookups
    let skip_paths = Arc::new(Mutex::new(HashSet::<String>::default()));    
    let skip_paths_clone = Arc::clone(&skip_paths);
    
    // Configure walker
    let walker = WalkDirGeneric::<((), ())>::new(root_path)
        .skip_hidden(false)
        .follow_links(false)
        .sort(false)
        .parallelism(jwalk::Parallelism::RayonNewPool(num_threads));
    
    // Clone counters
    let file_count_clone = Arc::clone(&file_count);
    let dir_count_clone = Arc::clone(&dir_count);
    let node_modules_count_clone = Arc::clone(&node_modules_count);
    let ignored_count_clone = Arc::clone(&ignored_count);
    
    // Process walker
    let processed = walker
        .into_iter()
        .filter_map(|entry_result| {
            // Skip errors
            entry_result.ok()
        })
        .filter_map(|entry| {
            // Fast filter for ignored paths
            if is_ignored(&entry) {
                ignored_count_clone.fetch_add(1, Ordering::Relaxed);
                return None;
            }
            
            // Check if path should be skipped (inside node_modules)
            let path = entry.path();
            let path_str = path.to_string_lossy();
            
            // Fast check of skip paths
            let should_skip = {
                let skip_set = skip_paths.lock().unwrap();
                // Iterate through ancestors to find if any parent is in the skip set
                let mut current = Some(path.as_path());
                let mut should_skip = false;
                while let Some(p) = current {
                    if let Some(p_str) = p.to_str() {
                        if skip_set.contains(p_str) && path_str != p_str {
                            should_skip = true;
                            break;
                        }
                    }
                    current = p.parent();
                }
                should_skip
            };
            
            if should_skip {
                None
            } else {
                Some(entry)
            }
        })
        .filter_map(|entry| {
            if entry.file_type.is_dir() {
                dir_count_clone.fetch_add(1, Ordering::Relaxed);
                
                // Check if node_modules directory
                if entry.file_name.to_string_lossy() == "node_modules" {
                    node_modules_count_clone.fetch_add(1, Ordering::Relaxed);
                    let path = entry.path().to_string_lossy().to_string();
                    
                    // Use thread-local storage to batch updates
                    LOCAL_NODE_MODULES.with(|local_paths| {
                        let mut paths = local_paths.borrow_mut();
                        paths.push(path);
                        
                        // Only lock the global collections when we have enough items
                        if paths.len() >= 20 {
                            // Batch update skip paths
                            {
                                let mut skip_set = skip_paths_clone.lock().unwrap();
                                for path in paths.iter() {
                                    skip_set.insert(path.clone());
                                }
                            }
                            
                            // Batch update locations
                            {
                                let mut locations = node_modules_locations_clone.lock().unwrap();
                                locations.extend(paths.drain(..));
                            }
                        }
                    });
                }
            } else {
                file_count_clone.fetch_add(1, Ordering::Relaxed);
            }
            Some(())
        })
        .count();
    
    // Flush any remaining items in thread-local storage
    LOCAL_NODE_MODULES.with(|local_paths| {
        let paths = local_paths.borrow();
        if !paths.is_empty() {
            // Update skip paths
            {
                let mut skip_set = skip_paths_clone.lock().unwrap();
                for path in paths.iter() {
                    skip_set.insert(path.clone());
                }
            }
            
            // Update locations
            {
                let mut locations = node_modules_locations_clone.lock().unwrap();
                locations.extend(paths.iter().cloned());
            }
        }
    });
    
    let elapsed = start.elapsed();
    stop_spinner(spinner, "Done walking");
    println!();
    // Print benchmark results
    info!("Traversal completed in {:.2?}", elapsed);
    info!("Directories scanned: {}", dir_count.load(Ordering::Relaxed));
    info!("Files scanned: {}", file_count.load(Ordering::Relaxed));
    info!("node_modules directories found: {}", node_modules_count.load(Ordering::Relaxed));
    info!("Paths ignored: {}", ignored_count.load(Ordering::Relaxed));
    info!("Total entries processed: {}", processed);
    
    // Calculate and print processing speed
    let total_entries = dir_count.load(Ordering::Relaxed) + file_count.load(Ordering::Relaxed);
    let speed = if elapsed.as_secs_f64() > 0.0 {
        total_entries as f64 / elapsed.as_secs_f64()
    } else {
        total_entries as f64 // Avoid division by zero
    };
    info!("Processing speed: {:.2} entries/sec", speed);
    
    // Calculate and print node_modules finding speed
    let node_modules_count_print = node_modules_count.load(Ordering::Relaxed);
    let node_modules_speed = if elapsed.as_secs_f64() > 0.0 {
        node_modules_count_print as f64 / elapsed.as_secs_f64()
    } else {
        node_modules_count_print as f64 // Avoid division by zero
    };
    info!("node_modules finding speed: {:.2} node_modules/sec", node_modules_speed);
    
    // Print a sample of found node_modules locations
    info!("Sample of node_modules locations found:\n");
    let locations = node_modules_locations.lock().unwrap();
    let display_count = std::cmp::min(locations.len(), 10); // Display up to 10 locations
    
    for i in 0..display_count {
        info!("  - {}", locations[i]);
    }
    
    if node_modules_count_print > 10 {
        info!("  ... and {} more \n", locations.len());
    }
    info!("Reading patterns!");

    let locations_pathbuff = convert_string_to_pathbuf(&locations);
    //TODO: Returning an Vec<> from this function call will allow me to allocate and fill an big
    //array only once, instead of mutating it often. Implement split_by_type(paths: Vec<Path>) ->
    //Vec<PathBuf>, Vec<PathBuf> DIR AND FILES Vec
    
    let mut matched_paths: Vec<PathBuf> = Vec::new();

    matched_paths= matcher::matching_pattern(&locations_pathbuff);
    
    match utils::read_size::get_paths_size(&matched_paths) {
        Ok((bytes, mb)) => info!("Total node_modules size: {} bytes ({:.2} MB)", bytes, mb),
        Err(err) => info!("Error calculating node_modules size: {}", err),
    }
    
}
