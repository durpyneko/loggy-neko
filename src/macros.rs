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

/// Logs a debug message.
///
/// # Example
/// ```
/// debug!("This is a debug message.");
/// ```
#[macro_export]
macro_rules! debug {
    ($message:expr) => {
        $crate::LOGGER.debug($message);
    };
}
