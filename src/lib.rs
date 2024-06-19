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
    static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Info);
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

pub fn set_log_level(level: LogLevel) {
    let mut log_level = LOG_LEVEL.lock().unwrap();
    *log_level = level;
}

fn should_log(level: LogLevel) -> bool {
    let log_level = LOG_LEVEL.lock().unwrap();
    level <= *log_level
}

pub fn log(level: LogLevel, message: &str) {
    if should_log(level) {
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

pub fn info(message: &str) {
    log(LogLevel::Info, message);
}

pub fn warn(message: &str) {
    log(LogLevel::Warn, message);
}

pub fn error(message: &str) {
    log(LogLevel::Error, message);
}

/*
  ∧,,,∧
(  ̳• · • ̳)
/    づ♡ read if cute
*/
