use crate::config::parse_settings::{get_all_settings, get_setting};
use log::{error, info, warn};
use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub static CONFIG_CHECK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub struct ConfigValidation {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn validate_startup_config() -> ConfigValidation {
    info!("Checking startup settings");
    let mut validation = ConfigValidation {
        valid: true,
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    let _all_settings = get_all_settings();
    for (key, value) in _all_settings {
        info!("{} | {}", key, value);
    }

    if let Some(log_level) = get_setting("log_level") {
        if !["debug", "info", "warn", "error"].contains(&log_level.as_str()) {
            validation
                .errors
                .push(format!("Invalid log_level: {}", log_level));
            validation.valid = false;
        }
    }

    if let Some(debug) = get_setting("debug") {
        if !["true", "false"].contains(&debug.as_str()) {
            validation.errors.push(format!(
                "Debug has to be either true or false, currently it is {}",
                debug
            ));
            validation.valid = false;
        }
    }

    if let Some(custom_pattern_location) = get_setting("custom_pattern_location") {
        let path = Path::new(&custom_pattern_location);
        // Check if the path exists
        if !path.exists() {
            let _wrn_msg = format!(
                "Custom pattern location does not exist: {}",
                custom_pattern_location
            );
            _validation_warning(&_wrn_msg, &mut validation);
        }
        // Optionally, check if it's a file (not a directory)
        else if !path.is_file() {
            let _err_msg = format!(
                "Custom pattern location is not a file: {}",
                custom_pattern_location
            );

            _validation_warning(&_err_msg, &mut validation);
        }
        // Optionally, check if the file is readable
        else if let Err(e) = std::fs::File::open(path) {
            let _err_msg = format!(
                "Cannot read custom pattern location {}: {}",
                custom_pattern_location, e
            );
            _validation_error(&_err_msg, &mut validation);
        }
    }
    *CONFIG_CHECK.lock().unwrap() = true;
    validation
}

static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn set_logger_initialized() {
    LOGGER_INITIALIZED.store(true, Ordering::Relaxed);
}

fn _validation_error(message: &str, validation: &mut ConfigValidation) {
    if LOGGER_INITIALIZED.load(Ordering::Relaxed) {
        error!("{}", message);
    } else {
        eprintln!("ERROR: {}", message);
    }
    validation.errors.push(message.to_string());
    validation.valid = false;
}

fn _validation_warning(message: &str, validation: &mut ConfigValidation) {
    if LOGGER_INITIALIZED.load(Ordering::Relaxed) {
        warn!("{}", message);
    } else {
        eprintln!("WARNING: {}", message);
    }
    validation.warnings.push(message.to_string());
}
