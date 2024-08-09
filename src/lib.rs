/*
*   ██╗      ██████╗  ██████╗  ██████╗██╗   ██╗     ███╗   ██╗███████╗██╗  ██╗ ██████╗
*   ██║     ██╔═══██╗██╔════╝ ██╔════╝╚██╗ ██╔╝     ████╗  ██║██╔════╝██║ ██╔╝██╔═══██╗
*   ██║     ██║   ██║██║  ███╗██║  ███╗╚████╔╝█████╗██╔██╗ ██║█████╗  █████╔╝ ██║   ██║
*   ██║     ██║   ██║██║   ██║██║   ██║ ╚██╔╝ ╚════╝██║╚██╗██║██╔══╝  ██╔═██╗ ██║   ██║
*   ███████╗╚██████╔╝╚██████╔╝╚██████╔╝  ██║        ██║ ╚████║███████╗██║  ██╗╚██████╔╝
*   ╚══════╝ ╚═════╝  ╚═════╝  ╚═════╝   ╚═╝        ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝
*/

use chrono::Local;
use colored::Colorize;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod macros;
pub mod prelude;

lazy_static! {
    pub static ref LOGGER: Logger = Logger::new();
}

// maybe RwLock?
pub struct Logger {
    log_level: Mutex<LogLevel>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
    Error,
    Warn,
    Debug,
    Info,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match *self {
            LogLevel::Error => "[ERRR]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Debug => "[DEBU]",
            LogLevel::Info => "[INFO]",
        }
    }

    fn color(&self) -> colored::ColoredString {
        let text = self.as_str();
        match *self {
            LogLevel::Error => text.bright_red(),
            LogLevel::Warn => text.bright_yellow(),
            LogLevel::Debug => text.bright_black(),
            LogLevel::Info => text.bright_green(),
        }
    }
}

impl Logger {
    fn new() -> Self {
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
    pub fn log<T: AsRef<str>>(&self, level: LogLevel, message: T) {
        if self.should_log(level) {
            let formatted_message = format!(
                "{} {} {}",
                format!("[{}]", Local::now().format("%Y-%m-%d %H:%M:%S")).bright_black(),
                level.color(),
                message.as_ref()
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
    pub fn info<T: AsRef<str>>(&self, message: T) {
        self.log(LogLevel::Info, message.as_ref());
    }

    /// Logs a warn message.
    ///
    /// # Example
    /// ```
    /// LOGGER.warn("This is a warn message.");
    /// ```
    pub fn warn<T: AsRef<str>>(&self, message: T) {
        self.log(LogLevel::Warn, message.as_ref());
    }

    /// Logs an error message.
    ///
    /// # Example
    /// ```
    /// LOGGER.error("This is an error message.");
    /// ```
    pub fn error<T: AsRef<str>>(&self, message: T) {
        self.log(LogLevel::Error, message.as_ref());
    }

    /// Logs a debug message.
    ///
    /// # Example
    /// ```
    /// LOGGER.debug("This is a debug message.");
    /// ```
    pub fn debug<T: AsRef<str>>(&self, message: T) {
        self.log(LogLevel::Debug, message.as_ref());
    }
}

/*
  ∧,,,∧
( ̳• · • ̳)
/ づ♡ read if cute
*/
