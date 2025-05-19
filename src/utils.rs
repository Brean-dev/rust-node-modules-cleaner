use std::collections::HashMap;
use log::debug;

// Function to iterate and display pattern hits
pub fn iter_pattern_hits(hits: &HashMap<String, i32>) {
    let mut items: Vec<_> = hits.iter().collect();
    items.sort_by_key(|&(k, _)| k);

    debug!("Pattern hits:");
    for (k, v) in items {
        debug!("{:<20} {}", k, v);
    }
}
