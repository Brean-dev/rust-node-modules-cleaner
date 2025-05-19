use std::path::{Path, PathBuf};
use std::time::Instant;
use jwalk::{WalkDirGeneric, DirEntry};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use walkdir::{WalkDir, DirEntry as WalkDirEntry};
use std::collections::HashMap;
use std::cell::RefCell;
use serde::Deserialize;
use clap::Parser;
use log::{error, info, trace, debug};
use once_cell::sync::Lazy;

// Import fxhash if available, otherwise use regular HashMap
#[cfg(feature = "use-fxhash")]
use fxhash::{FxHashMap, FxHashSet};
#[cfg(not(feature = "use-fxhash"))]
use std::collections::{HashMap as FxHashMap, HashSet as FxHashSet};

// Thread-local storage for batching path operations
thread_local! {
    static LOCAL_NODE_MODULES: RefCell<Vec<String>> = RefCell::new(Vec::with_capacity(50));
}

// Original config structures
#[derive(Debug, Deserialize, Clone)]
struct RuleSet {
    patterns: Vec<String>,
    ignore: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Config {
    #[serde(rename = "$default")]
    default: String,
    #[serde(flatten)]
    rules: HashMap<String, RuleSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PaternHits {
    pub patterns: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    //arguments to look for 
    #[arg(short, long)]
    arguments: String,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

static LOG_LEVEL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("INFO")));

// Optimized path ignoring function for jwalk
fn is_ignored<C: jwalk::ClientState>(entry: &jwalk::DirEntry<C>) -> bool {
    let path = entry.path();
    
    // Fast prefix check for common system directories
    let path_str = path.as_os_str().to_string_lossy();
    if path_str.starts_with("/proc/") || path_str.starts_with("/sys/") || 
       path_str.starts_with("/dev/") || path_str.starts_with("/run/") ||
       path_str.starts_with("/efi/") || path_str.starts_with("/usr/") {
        return true;
    }
    
    // Fast component check to avoid repeated iteration
    let mut components_iter = path.components();
    while let Some(component) = components_iter.next() {
        if let std::path::Component::Normal(name) = component {
            let name_str = name.to_string_lossy();
            if name_str == "Projects" || name_str == "opt" || name_str == ".vscode" {
                return true;
            }
        }
    }
    
    false
}



// Your original read_patterns function
fn read_patterns() -> Result<Config, Box<dyn std::error::Error>> {
    let data = include_str!("../patterns.json");
    let config: Config = serde_json::from_str(data)?;
    Ok(config)
}

// Original iteration function using your pattern matcher
fn iter_pattern_hits(hits: &HashMap<String, i32>) {
    let mut items: Vec<_> = hits.iter().collect();
    items.sort_by_key(|&(k, _)| k);

    debug!("Pattern hits:");
    for (k, v) in items {
        debug!("{:<20} {}", k, v);
    }
}

fn matching_pattern(paths: &Vec<PathBuf>) {
    info!("Matching patterns for {:?} node_modules directories", paths.len());
    let mut results: i32 = 0;
    let mut safe_paths_array: Vec<PathBuf> = Vec::with_capacity(paths.len() * 10); // Pre-allocate more space
    let mut pattern_hits: HashMap<String, i32> = HashMap::new();
    
    match read_patterns() {
        Ok(config) => {
            info!("Successfully loaded patterns config");            
            
            // Get the safe ruleset once outside the loop
            if let Some(safe_ruleset) = config.rules.get("safe") {
                // For each node_modules directory
                for node_modules_path in paths {
                    trace!("Walking through directory: {}", node_modules_path.display());
                    
                    // Actually walk through the directory and check each file
                    for entry_result in WalkDir::new(node_modules_path)
                        .into_iter()
                        .filter_map(Result::ok) {
                        
                        let entry_path = entry_result.path();
                        trace!("Checking path: {}", entry_path.display());
                        
                        // Only process files (not directories)
                        if entry_result.file_type().is_file() {
                            let path_str = entry_path.to_str().unwrap_or("");
                            let file_name = entry_path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("");
                            
                            // Check against each pattern
                            for pattern in &safe_ruleset.patterns {
                                // Try to match against full path and file name
                                let matches = match_path_with_pattern(path_str, pattern) || 
                                              match_path_with_pattern(file_name, pattern);
                                
                                if matches {
                                    trace!("Bingo! File {} matches pattern {}", entry_path.display(), pattern);
                                    safe_paths_array.push(entry_path.to_path_buf());
                                    *pattern_hits.entry(pattern.clone()).or_insert(0) += 1;
                                    results += 1;
                                    break; // Move to next file after first match
                                }
                            }
                        }
                    }
                }
            } else {
                error!("No 'safe' ruleset found in configuration");
            }
        },
        Err(e) => {
            error!("Error loading patterns: {}", e);
        }
    }
    
    debug!("safe_paths_array Contains: {} items", safe_paths_array.len());
    info!("Found {} files which match the `safe` pattern", results);
    if *LOG_LEVEL.lock().unwrap() == "DEBUG" {
        iter_pattern_hits(&pattern_hits);
    }
}

// Enhanced pattern matching function with support for wildcards and case-insensitivity
fn match_path_with_pattern(path_str: &str, pattern: &str) -> bool {
    // Case 1: If pattern starts and ends with '/', it's meant to match a directory
    if pattern.starts_with('/') && pattern.ends_with('/') {
        let dir_name = &pattern[1..pattern.len()-1]; // Remove slashes
        
        // Convert to platform-specific path separators
        #[cfg(not(windows))]
        let normalized_path = path_str;
        
        #[cfg(windows)]
        let normalized_path = path_str.replace('\\', "/");
        
        // Check if `/dir_name/` exists in the path
        let dir_pattern = format!("/{}/", dir_name);
        return normalized_path.contains(&dir_pattern);
    }
    
    // Case 2: Handle wildcard patterns like "readme*"
    else if pattern.contains('*') {
        let path_lower = path_str.to_lowercase();
        let parts: Vec<&str> = pattern.split('*').collect();
        
        // Simple wildcard matching
        if parts.len() == 2 {
            let prefix = parts[0].to_lowercase();
            let suffix = parts[1].to_lowercase();
            
            if suffix.is_empty() {
                // Pattern like "readme*" - just check for prefix
                return path_lower.contains(&prefix);
            } else {
                // Pattern like "read*me" - check for both parts in order
                return path_lower.contains(&prefix) && 
                       path_lower.contains(&suffix) &&
                       path_lower.find(&prefix).unwrap_or(usize::MAX) < 
                       path_lower.find(&suffix).unwrap_or(usize::MAX);
            }
        }
    }
    
    // Case 3: Check for exact filename match (case insensitive)
    else {
        let path = Path::new(path_str);
        if let Some(file_name) = path.file_name() {
            if let Some(file_str) = file_name.to_str() {
                if file_str.to_lowercase() == pattern.to_lowercase() {
                    return true;
                }
            }
        }
        
        // Case 4: Check for pattern as substring of path (case insensitive)
        return path_str.to_lowercase().contains(&pattern.to_lowercase());
    }
    
    false
}

// Optimized function to convert string to PathBuf
fn convert_string_to_pathbuf(mutex: &MutexGuard<Vec<String>>) -> Vec<PathBuf> {
    let mut result = Vec::with_capacity(mutex.len());
    for s in mutex.iter() {
        result.push(PathBuf::from(s));
    }
    result
}

// Optimized walker function
fn walk_directories() {
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
    
    // Pre-allocate collections with appropriate initial capacity
    let node_modules_locations = Arc::new(Mutex::new(Vec::with_capacity(2000)));
    let node_modules_locations_clone = Arc::clone(&node_modules_locations);

    // Use HashSet for faster path lookups
    let skip_paths = Arc::new(Mutex::new(std::collections::HashSet::<String>::default()));    

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
            match entry_result {
                Ok(entry) => Some(entry),
                Err(_) => None
            }
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
    
    // Print benchmark results
    info!("----------------------------------------");
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
    info!("Sample of node_modules locations found:");
    let locations = node_modules_locations.lock().unwrap();
    let display_count = std::cmp::min(locations.len(), 10); // Display up to 10 locations
    
    for i in 0..display_count {
        info!("  - {}", locations[i]);
    }
    
    if node_modules_count_print > 10 {
        info!("  ... and {} more", locations.len().to_string());
    }
    info!("Reading patterns!");

    let locations_pathbuff = convert_string_to_pathbuf(&locations);
    matching_pattern(&locations_pathbuff);
}


fn main() {
    let start = Instant::now();
    
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Initialize logger
    let mut builder = env_logger::Builder::from_default_env();
    builder
        .filter_level(cli.verbose.log_level_filter())
        .init();
        
    *LOG_LEVEL.lock().unwrap() = cli.verbose.log_level_filter().to_string();
    
    // Option 1: Use the new optimized walk_directories
    walk_directories();
    
    // Option 2: Use optimized versions of the original functions
    // matching_pattern(&iterate_matching_directories_optimized(&iterate_directories_optimized()));
    
    let elapsed = start.elapsed();
    info!("Total execution time: {:.2?}", elapsed);
}
