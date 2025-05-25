use std::collections::HashMap;
use log::debug;
use spinners::{Spinner, Spinners};
use std::sync::Mutex; 
use once_cell::sync::Lazy;


pub static SPINNER: Lazy<Mutex<Option<Spinner>>> = Lazy::new(|| {
    Mutex::new(None)
});

// Function to iterate and display pattern hits
pub fn iter_pattern_hits(hits: &HashMap<String, i32>) {
    let mut items: Vec<_> = hits.iter().collect();
    items.sort_by_key(|&(k, _)| k);

    debug!("Pattern hits:");
    for (k, v) in items {
        debug!("{:<20} {}", k, v);
    }
}


    // Helper function to start the spinner with optional custom message
pub fn start_spinner(message: Option<String>) -> () {
    let msg = message.unwrap_or("".into());
    let mut spinner_guard = SPINNER.lock().unwrap();
    print!("\x1b[2K\r");
    println!("\n");
    *spinner_guard = Some(Spinner::new(Spinners::Hearts, msg));
}

    // Helper function to stop the spinner
pub fn stop_spinner() -> () {
    let mut spinner_guard = SPINNER.lock().unwrap();
    if let Some(mut spinner) = spinner_guard.take() {
       spinner.stop_with_symbol("\x1b[32m🗸\x1b[0m"); 
    }
}

