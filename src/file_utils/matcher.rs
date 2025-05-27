use std::path::{Path, PathBuf};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use log::{info, debug, error, trace};
use walkdir::WalkDir;


use crate::config;
use crate::config::cli::LOG_LEVEL;
use crate::utils::g_utils::{iter_pattern_hits, start_spinner, stop_spinner, get_ticks, SpinnerTheme};


//Global Vec's to store DIR and FILE paths seperately 
pub static FILES: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static DIRS: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| Mutex::new(Vec::new()));




// Main function to match patterns against node_modules directories
pub fn matching_pattern(paths: &Vec<PathBuf>) -> Vec<PathBuf>  {
    info!("Matching patterns for {:?} node_modules directories", paths.len());
    println!();
    let spinner = start_spinner("Matching patterns...", get_ticks(SpinnerTheme::PatternMatch));

    #[allow(unused_variables)]
    let mut results: i32 = 0;
    let mut safe_paths_array: Vec<PathBuf> = Vec::with_capacity(paths.len() * 10); // Pre-allocate more space
    let mut pattern_hits: HashMap<String, i32> = HashMap::new();
    
    // Debug sampling settings
    let max_debug_samples = 10; // Maximum number of debug samples to show per pattern
    let mut debug_sample_counts: HashMap<String, i32> = HashMap::new();
    
    match config::config::read_patterns() {
        Ok(config) => {
            // info!("Successfully loaded patterns config");            
            
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
                                    // Sample debug logging
                                    let sample_count = debug_sample_counts.entry(pattern.clone()).or_insert(0);
                                    *sample_count += 1;
                                    
                                    if *sample_count <= max_debug_samples {
                                        debug!("Match #{} for pattern '{}': {}", 
                                              *sample_count, pattern, entry_path.display());
                                    } else if *sample_count == max_debug_samples + 1 {
                                        debug!("Suppressing further debug output for pattern '{}'", pattern);
                                    }
                                    
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
    stop_spinner(spinner, "Done matching patterns");
    println!();
    debug!("safe_paths_array Contains: {} items", safe_paths_array.len());
    // info!("Found {} files which match the `safe` pattern", results);
    debug!("Pattern hit summary:");
    for (pattern, count) in &pattern_hits {
        debug!("  - '{}': {} matches", pattern, count);
    }
    
    if *LOG_LEVEL.lock().unwrap() == "DEBUG" {
        iter_pattern_hits(&pattern_hits);
    }
    safe_paths_array
}

// Enhanced pattern matching function with support for wildcards and case-insensitivity
pub fn match_path_with_pattern(path_str: &str, pattern: &str) -> bool {
    let mut matches = Vec::new();  
    let mut match_result = false;
    let mut match_reason = String::new();
    
    // Case 1: If pattern starts and ends with '/', it's meant to match a directory
    if pattern.starts_with('/') && pattern.ends_with('/') {
        let dir_name = &pattern[1..pattern.len()-1]; // Remove slashes
        
        // Convert to platform-specific path separators
        #[cfg(not(windows))]
        let normalized_path = path_str;
        
        #[cfg(windows)]
        let normalized_path = path_str.replace('\\', "/");
        
        // Check if `/dir_name/` exists in the path as a complete directory segment
        let dir_pattern = format!("/{}/", dir_name);
        match_result = normalized_path.contains(&dir_pattern);
        
        if match_result {
            match_reason = format!("Directory pattern '{}' found in path", dir_pattern);
            matches.push(path_str); // Add to matches vector here
        }
    }
    
    // Case 2: Handle wildcard patterns like "readme*"
    else if pattern.contains('*') {
        let path_lower = path_str.to_lowercase();
        let parts: Vec<&str> = pattern.split('*').collect();
        
        // Simple wildcard matching
        if parts.len() == 2 {
            let prefix = parts[0].to_lowercase();
            let suffix = parts[1].to_lowercase();
            
            // Get the file name for boundary checking
            let file_name = Path::new(path_str)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            if suffix.is_empty() {
                // Pattern like "readme*" - check if it's at the beginning of file name
                // or is a complete path segment
                let is_segment = is_exact_path_segment(&path_lower, &prefix);
                let starts_with = file_name.starts_with(&prefix);
                
                match_result = is_segment || starts_with;
                
                if match_result {
                    if is_segment {
                        match_reason = format!("Prefix '{}' is an exact path segment", prefix);
                    } else {
                        match_reason = format!("Filename '{}' starts with '{}'", file_name, prefix);
                    }
                    matches.push(path_str); // Add to matches vector here
                }
            } else {
                // Pattern like "read*me" - ensure both parts and check boundaries
                let contains_prefix = path_lower.contains(&prefix);
                let contains_suffix = path_lower.contains(&suffix);
                let prefix_before_suffix = path_lower.find(&prefix).unwrap_or(usize::MAX) < 
                                         path_lower.find(&suffix).unwrap_or(usize::MAX);
                let combined = prefix.to_owned() + &suffix;
                let is_segment = is_exact_path_segment(&path_lower, &combined);
                let file_pattern_match = file_name.starts_with(&prefix) && file_name.ends_with(&suffix);
                
                match_result = contains_prefix && 
                               contains_suffix && 
                               prefix_before_suffix &&
                               (is_segment || file_pattern_match);
                
                if match_result {
                    if is_segment {
                        match_reason = format!("Combined pattern '{}{}{}' is an exact path segment", 
                                             prefix, "*", suffix);
                    } else {
                        match_reason = format!("Filename '{}' matches wildcard pattern '{}*{}'", 
                                             file_name, prefix, suffix);
                    }
                    matches.push(path_str); // Add to matches vector here
                }
            }
        }
    }
    
    // Case 3: Check for exact matches
    else {
        let path = Path::new(path_str); 
        // Check if pattern matches the file name exactly
        if let Some(file_name) = path.file_name() {
            if let Some(file_str) = file_name.to_str() {
                if file_str.to_lowercase() == pattern.to_lowercase() {
                    match_result = true;
                    match_reason = format!("Exact filename match: '{}'", file_str);
                    matches.push(path_str); // Add to matches vector here
                }
            }
        }
        
        // Check if pattern matches a file extension exactly
        if !match_result {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if ext_str.to_lowercase() == pattern.to_lowercase() {
                        match_result = true;
                        match_reason = format!("Exact extension match: '.{}'", ext_str);
                        matches.push(path_str); // Add to matches vector here
                    }
                }
            }
        }
        
        // Check if pattern is an exact path segment (directory name)
        if !match_result {
            match_result = is_exact_path_segment(&path_str.to_lowercase(), &pattern.to_lowercase());
            if match_result {
                match_reason = format!("Exact path segment match: '{}'", pattern);
                matches.push(path_str); // Add to matches vector here
            }
        }
    }
    
    // Only log if there's a match
    if match_result {
        debug!("MATCH: '{}' with pattern '{}' - {}", path_str, pattern, match_reason);
    }
    
    // Always call split_by_type with our matches
    split_by_type(matches);
    match_result
}

// Helper function to check if a string is an exact path segment
pub fn is_exact_path_segment(path: &str, segment: &str) -> bool {
    // Normalize path separators for consistent processing
    #[cfg(windows)]
    let normalized_path = path.replace('\\', "/");
    
    #[cfg(not(windows))]
    let normalized_path = path;
    
    // Split the path into segments and check if any segment matches exactly
    let path_segments: Vec<&str> = normalized_path.split('/').collect();
    
    path_segments.iter().any(|&s| s.to_lowercase() == segment.to_lowercase())
}


// Helper function to split files and directories into global array's 
// Arrays in turn will be used to remove files accordingly 
fn split_by_type(path_str: Vec<&str>){
    let mut local_files = Vec::new();
    let mut local_dirs = Vec::new();

    for s in path_str{
        let path = Path::new(s);
        if path.is_file(){
            local_files.push(path.to_path_buf());
        } else if path.is_dir(){
            local_dirs.push(path.to_path_buf());
        }
    }
    FILES.lock().unwrap().extend(local_files);
    DIRS.lock().unwrap().extend(local_dirs);
}
