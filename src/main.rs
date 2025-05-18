use std::path::{PathBuf};
use std::fs::File;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::BufReader;


use walkdir::{WalkDir, DirEntry};

fn is_node_modules(entry: &DirEntry) -> bool {
    entry.file_name() == "node_modules" && entry.file_type().is_dir()
}

#[derive(Debug, Deserialize)]
struct RuleSet{
    patterns: Vec<String>,
    ignore: Vec<String>,
}
#[derive(Debug, Deserialize)]
struct Config{
    #[serde(rename = "$default")]
    default: String,
    #[serde(flatten)]
    rules: HashMap<String, RuleSet>,
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
    match read_patterns() {
        Ok(config) => {
            println!("Successfully loaded patterns config");

            for path in paths {
                //println!("Checking path: {}", path.display());

                // Make sure "safe" exists in the rules map
                if let Some(safe_ruleset) = config.rules.get("safe") {
                    // Iterate through the patterns vector inside the RuleSet
                    for pattern in &safe_ruleset.patterns {
                        if let Some(path_str) = path.to_str() {
                            if path_str.contains(pattern) {
                                //println!("Bingo! Path {} matches pattern {}", path.display(), pattern);
                                safe_paths_array.push(path.to_path_buf());                                
                                results+=1;
                            }
                        }
                    }
                } else {
                    println!("No 'safe' ruleset found in configuration");
                }
            }
        },
        Err(e) => {
            eprintln!("Error loading patterns: {}", e);
        }

    }
    println!("safe_paths_array Contains: {:?} items ", safe_paths_array.len().to_string());
    println!("Found {:?} files which match the `safe` pattern", results);
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
            //println!("{}", entry.path().display());
            matches.push(entry.path().to_path_buf());
            _x+=1;
        }
    }
    println!("Total ammount of directories found: {:?}", _x);
    //println!("The array in question \r {:?}", matches);
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
                eprintln!("Error accessing entry: {}", e);
            }
          }
       }
    }
    return matching_files;
}

fn main() {
    // Triple nested function calling which in turn all itterate in their own way 
    // I know I am cringing too, I will most defintely have a look at this later on, once I have
    // gained more experience with Rust
    matching_pattern(&iterate_matching_directories(&iterate_directories()));
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

