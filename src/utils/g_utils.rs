use std::collections::HashMap;
use log::debug;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub enum SpinnerTheme {
    FileWalker,
    SearchScan,
    PatternMatch,
}
    // let ticks = &[];

pub fn get_ticks(theme: SpinnerTheme) -> &'static [&'static str] {
    match theme {
        SpinnerTheme::FileWalker => &[
            "📁", "📁", "📁", 
            "📂", "📂", "📂",  
            "📁", "📁", "📁",
            "📂", "📂", "📂",
        ],
        SpinnerTheme::SearchScan => &[
            "🔍", "🔍", "🔍",
            "🔎", "🔎", "🔎",
            "📡", "📡", "📡",
            "📶", "📶", "📶",
        ],
        SpinnerTheme::PatternMatch => &[
            "🎯", "🎯", "🎯",
            "🕵️", "🕵️", "🕵️",
            "🔎", "🔎", "🔎",
            "💡", "💡", "💡",
        ],
    }
}


// Function to iterate and display pattern hits
pub fn iter_pattern_hits(hits: &HashMap<String, i32>) {
    let mut items: Vec<_> = hits.iter().collect();
    items.sort_by_key(|&(k, _)| k);

    debug!("Pattern hits:");
    for (k, v) in items {
        debug!("{:<20} {}", k, v);
    }
}


pub fn start_spinner(message: &str, ticks: &[&str]) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();

    spinner.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(ticks),
    );

    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

pub fn stop_spinner(spinner: ProgressBar, final_message: &str) {
    spinner.finish_and_clear(); // Clears spinner line completely
    println!("✔️  {}", final_message); 
}

fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

