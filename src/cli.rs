use clap::Parser;
use clap_verbosity_flag::{LogLevel, Verbosity};
use env_logger::fmt::Color;
use log::Level;
use std::io::Write;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Thread-safe storage for log level
pub static LOG_LEVEL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("INFO")));

#[derive(Debug, Clone, Copy)]
pub struct InfoLevel;
impl LogLevel for InfoLevel {
    fn default() -> Option<Level> {
        Some(Level::Info)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // Make arguments optional
    #[arg(short, long, required = false)]
    pub arguments: Option<String>,
    
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

pub fn setup_logger(cli: &Cli) {
    // Initialize logger with custom format
    let mut builder = env_logger::Builder::from_default_env();
    builder
        .filter_level(cli.verbose.log_level_filter())
        .format(|buf, record| {
            let mut style = buf.style();
            
            // Color based on log level
            match record.level() {
                Level::Error => style.set_color(Color::Red),
                Level::Warn => style.set_color(Color::Yellow),
                Level::Info => style.set_color(Color::Green),
                Level::Debug => style.set_color(Color::Blue),
                Level::Trace => style.set_color(Color::Cyan),
            };
            
            writeln!(
                buf,
                "{} - {}",
                style.value(record.level()),
                record.args()
            )
        })
        .init();
        
    *LOG_LEVEL.lock().unwrap() = cli.verbose.log_level_filter().to_string();
}
