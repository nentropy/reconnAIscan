//! # Color Logger
//! 
//! `color_logger` is a simple logging library that provides colored, bolded log messages
//! and graceful error handling using the `thiserror` crate. It is built on top of the
//! `log` and `env_logger` crates for logging best practices.
//!
//! ## Example
//!
//! ```rust
//! use color_logger::{init_logger, log_info};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     init_logger()?;
//!     log_info("This is an info message");
//!     Ok(())
//! }
//! ```

use log::{Level, info, warn, error};
use env_logger::Env;
use colored::{Colorize, Color};
use thiserror::Error;
use std::io::Write;  // Corrected: Added missing semicolon

// Initialize the logger with the environment settings.
// This should be called at the start of your application.
pub fn init_logger() -> Result<(), ColorLoggerError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level = record.level();
            let target = record.target();
            let mut style = buf.style();

            match level {
                Level::Error => style.env_logger::fmt::Color("Red").set_bold(true),
                Level::Warn => style.env_logger::fmt::Color::Yellow).set_bold(true),
                Level::Info => style.env_logger::fmt::Color::Green).set_bold(true),
                Level::Debug => style.env_logger::fmt::Color::Blue).set_bold(true),
                Level::Trace => style.env_logger::fmt::Color::White).set_bold(true),
            };

            writeln!(buf, "{}: {}: {}", style.value(level), target, record.args())
                .map_err(|_| ColorLoggerError::InitError)  // Handle potential errors in writing to the buffer
        })?
        .init();

    Ok(())
}

/// Custom error type for the Color Logger library.
#[derive(Error, Debug)]
pub enum ColorLoggerError {
    #[error("Failed to initialize logger")]
    InitError,
}

/// Log an info message.
pub fn log_info(message: &str) {
    info!("{}", message.green().bold());  // Corrected: Use text color instead of background color
}

/// Log a warning message.
pub fn log_warn(message: &str) {
    warn!("{}", message.yellow().bold());  // Corrected: Use text color instead of background color
}

/// Log an error message.
pub fn log_error(message: &str) {
    error!("{}", message.red().bold());  // Corrected: Use text color instead of background color
}