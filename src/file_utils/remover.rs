use std::path::PathBuf;

use crate::config::cli::ask_yes_no;
#[allow(unused_imports)]
use log::{info, debug, warn};

pub fn remove_file_on_path(paths: Vec<PathBuf>){
    if ask_yes_no("About to permanently remove files from your system, would you like to proceed?"){
        println!("{}", paths.len());
    }else {
        warn!("User has aborted");
    }    
}
