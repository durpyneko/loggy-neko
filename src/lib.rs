/*
*    ██╗      ██████╗  ██████╗  ██████╗██╗   ██╗
*    ██║     ██╔═══██╗██╔════╝ ██╔════╝╚██╗ ██╔╝
*    ██║     ██║   ██║██║  ███╗██║  ███╗╚████╔╝
*    ██║     ██║   ██║██║   ██║██║   ██║ ╚██╔╝
*    ███████╗╚██████╔╝╚██████╔╝╚██████╔╝  ██║
*    ╚══════╝ ╚═════╝  ╚═════╝  ╚═════╝   ╚═╝
*/

use chrono::Local;
use colored::Colorize;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref LOGGER: Logger = Logger::new();
}

/// Logs an info message.
///
/// # Example
/// ```
/// info!("This is an informative message.");
/// ```
#[macro_export]
macro_rules! info {
    ($message:expr) => {
        $crate::LOGGER.info($message);
    };
}

/// Logs a warning message.
///
/// # Example
/// ```
/// warn!("This is a warning message.");
/// ```
#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        $crate::LOGGER.warn($message);
    };
}

/// Logs an error message.
///
/// # Example
/// ```
/// error!("This is an error message.");
/// ```
#[macro_export]
macro_rules! error {
    ($message:expr) => {
        $crate::LOGGER.error($message);
    };
}

// maybe RwLock?
pub struct Logger {
    log_level: Mutex<LogLevel>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match *self {
            LogLevel::Error => "[ERRR]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Info => "[INFO]",
        }
    }

    fn color(&self) -> colored::ColoredString {
        match *self {
            LogLevel::Error => self.as_str().bright_red(),
            LogLevel::Warn => self.as_str().bright_yellow(),
            LogLevel::Info => self.as_str().bright_green(),
        }
    }
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            log_level: Mutex::new(LogLevel::Info),
        }
    }

    /// Set log level.
    ///
    /// # Example
    /// ```
    /// LOGGER.set_log_level(LogLevel::Info);
    /// ```
    pub fn set_log_level(&self, level: LogLevel) {
        let mut log_level = self.log_level.lock().unwrap();
        *log_level = level;
    }

    fn should_log(&self, level: LogLevel) -> bool {
        let log_level = self.log_level.lock().unwrap();
        level <= *log_level
    }

    /// Custom log implementation.
    ///
    /// # Example
    /// ```
    /// LOGGER.log(LogLevel::Info, "This is an informative message.");
    /// ```
    pub fn log(&self, level: LogLevel, message: &str) {
        if self.should_log(level) {
            let formatted_message = format!(
                "{} {} {}",
                format!("[{}]", Local::now().format("%Y-%m-%d %H:%M:%S")).bright_black(),
                level.color(),
                message
            );
            if level == LogLevel::Error {
                eprintln!("{}", formatted_message);
            } else {
                println!("{}", formatted_message);
            }
        }
    }

    /// Logs an info message.
    ///
    /// # Example
    /// ```
    /// LOGGER.info("This is an informative message.");
    /// ```
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Logs a warn message.
    ///
    /// # Example
    /// ```
    /// LOGGER.warn("This is a warn message.");
    /// ```
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Logs an error message.
    ///
    /// # Example
    /// ```
    /// LOGGER.info("This is an error message.");
    /// ```
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

/*
  ∧,,,∧
( ̳• · • ̳)
/ づ♡ read if cute
*/
