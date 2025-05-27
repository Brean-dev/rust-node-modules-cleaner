use clap::Parser;
use clap_verbosity_flag::{LogLevel, Verbosity};
use env_logger::fmt::Color;
use log::Level;
use std::io::Write;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use indicatif::{MultiProgress, ProgressDrawTarget};
use indicatif_log_bridge::LogWrapper;
use dialoguer::theme::{ColorfulTheme};


pub static DIALOG_THEME: Lazy<ColorfulTheme> = Lazy::new(ColorfulTheme::default);
// Thread-safe storage for log level
pub static LOG_LEVEL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("INFO")));
// Add global flag for full scan mode
pub static FULL_SCAN: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

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
   
    #[arg(long)]
    pub full: bool,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}


pub fn setup_logger(cli: &Cli) {
    let mp = MultiProgress::with_draw_target(ProgressDrawTarget::stderr());

    let mut builder = env_logger::Builder::new();
    builder
        .filter_level(cli.verbose.log_level_filter())
        .format(|buf, record| {
            let mut style = buf.style();
            match record.level() {
                Level::Error => style.set_color(Color::Red),
                Level::Warn => style.set_color(Color::Yellow),
                Level::Info => style.set_color(Color::Green),
                Level::Debug => style.set_color(Color::Blue),
                Level::Trace => style.set_color(Color::Cyan),
            };
            writeln!(buf, "{} - {}", style.value(record.level()), record.args())
        });

    // Wrap your logger with indicatif-aware bridge
    LogWrapper::new(mp.clone(), Box::new(builder.build()))
        .try_init()
        .expect("Failed to initialize logger");

    *LOG_LEVEL.lock().unwrap() = cli.verbose.log_level_filter().to_string();
    *FULL_SCAN.lock().unwrap() = cli.full;
}

pub fn ask_yes_no(prompt: &str) -> bool {
    dialoguer::Confirm::with_theme(&*DIALOG_THEME)
        .with_prompt(prompt)
        .default(true)
        .interact()
        .unwrap_or(false)
}
