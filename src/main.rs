use std::path::{PathBuf};
use std::fs::File;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::BufReader;
use walkdir::{WalkDir, DirEntry};
use clap::Parser;
use env_logger::Builder;
use log::{error, info, trace, debug, LevelFilter};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Deserialize, Clone)]
struct RuleSet{
    patterns: Vec<String>,
    ignore: Vec<String>,
}
#[derive(Debug, Deserialize, Clone)]
struct Config{
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
struct Cli{
//arguments to look for 
    #[arg(short, long)]
    arguments: String,
     #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
static LOG_LEVEL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("INFO")));


fn is_node_modules(entry: &DirEntry) -> bool {
    entry.file_name() == "node_modules" && entry.file_type().is_dir()
}

// Read pattern from patterns.json 
// Returns the JSON as a struct ::Config 
// Calling, read_patterns -> returns Config 
// config.rules.get("safe") returns 
// "safe": {
//       "patterns": [
//            "readme*",
//            ".npmignore",
//            "license",

fn read_patterns() -> Result<Config, Box<dyn std::error::Error>>{
    let current_dir = std::env::current_dir()?;
    let patterns_path = current_dir.join("patterns.json");
    let file = File::open(&patterns_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}


// matches patterns from an Path array 
// Requires Vec<PathBuf> array of Paths 
// Has read_patterns nested in it
// Itterates through PathBuf array and matches all the patterns 
// Pushes the patterns into an array 'safe_paths_array' Vec<PathBuf> 

fn matching_pattern(paths: &Vec<PathBuf>) {
    let mut results: i32 = 0;
    let mut safe_paths_array: Vec<PathBuf> = Vec::new();
        let mut pattern_hits: HashMap<String, i32> = HashMap::new();
    match read_patterns() {
        Ok(config) => {
            info!("Successfully loaded patterns config");            
                for path in paths {
                trace!("Checking path: {}", path.display());

                // Make sure "safe" exists in the rules map
                if let Some(safe_ruleset) = config.rules.get("safe") {
                    // Iterate through the patterns vector inside the RuleSet
                    for pattern in &safe_ruleset.patterns {
                        if let Some(path_str) = path.to_str() {
                            if path_str.contains(pattern) {
                                trace!("Bingo! Path {} matches pattern {}", path.display(), pattern);
                                safe_paths_array.push(path.to_path_buf());                                
                                *pattern_hits.entry(pattern.clone()).or_insert(0) += 1;
                                results+=1;
                            }
                        }
                    }
                } else {
                    error!("No 'safe' ruleset found in configuration");
                }
            }
        },
        Err(e) => {
            error!("Error loading patterns: {}", e);
        }

    }
    debug!("safe_paths_array Contains: {:?} items ", safe_paths_array.len().to_string());
    info!("Found {:?} files which match the `safe` pattern", results);
    if *LOG_LEVEL.lock().unwrap() == "DEBUG"{
        iter_pattern_hits(&pattern_hits);
    }
}


fn iter_pattern_hits(hits: &HashMap<String, i32>){
    let mut items: Vec<_> = hits.iter().collect();
    items.sort_by_key(|&(k, _)| k);

    debug!("Pattern hits:");
    for (k, v) in items {
        debug!("{:<20} {}", k, v);
    }

}
// Itterates through all directories from root '/'
// Uses built in WalkDir for some extra speed(god knows we need that when itterating the file
// system)
// If 'is_node_module()' returns true it pushes it into a new array 
fn iterate_directories() -> Vec<PathBuf>{
    let mut _x: i32 = 0;
    let mut matches: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new("/")
        .into_iter()
        .filter_entry(|e| !is_ignored(e))
        .filter_map(Result::ok)
    {
        if is_node_modules(&entry) {
            trace!("{}", entry.path().display());
            matches.push(entry.path().to_path_buf());
            _x+=1;
        }
    }
    info!("Total ammount of directories found: {:?}", _x);
    trace!("The array in question \r {:?}", matches);
    return matches;
}


// itterates through the node_module searching for all files, storing them into an new array. 
// for the sake of optimizing this I will revisit this and probally check patterns at the same time
fn iterate_matching_directories (directories: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut matching_files: Vec<PathBuf> = Vec::new();

    for i in 0..directories.len(){
       for entry_result in WalkDir::new(&directories[i]){
        match entry_result{
            Ok(entry) => {
                matching_files.push(entry.path().to_path_buf());
            },
            Err(e) => {
                error!("Error accessing entry: {}", e);
            }
          }
       }
    }
    return matching_files;
}



// Ignoring the following paths, either at root 
// starts_with will catch any file system in root I.e. /efi 
// iter.().any() will catch any regex in the full path no mather where the directory is in the path
// string
//
// This being Linux it is sensitive to capital letters
fn is_ignored(entry: &DirEntry) -> bool {
    let path = entry.path();
    // Add other paths to ignore as needed
    path.starts_with("/proc")
        || path.starts_with("/sys")
        || path.starts_with("/dev")
        || path.starts_with("/run")
        || path.starts_with("/efi")
        || path.iter().any(|comp| comp == "Projects")
        || path.iter().any(|comp| comp == "opt")
        || path.iter().any(|comp| comp == ".vscode")
        || path.starts_with("/usr")
}


fn main() {
    // Triple nested function calling which in turn all itterate in their own way 
    // I know I am cringing too, I will most defintely have a look at this later on, once I have
    // gained more experience with Rust
    let cli = Cli::parse();
    let mut builder = Builder::from_default_env();
    builder
        .filter_level(cli.verbose.log_level_filter())
        .init();
    *LOG_LEVEL.lock().unwrap() = cli.verbose.log_level_filter().to_string();
    matching_pattern(&iterate_matching_directories(&iterate_directories()));

}
