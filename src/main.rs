use loggy_neko::prelude::*;
use loggy_neko::{LogLevel, LOGGER};
use std::{thread, time};

#[derive(Debug)]
struct TestVec {
    array: Vec<String>,
}

fn main() {
    // * Standard use

    LOGGER.set_log_level(LogLevel::Error);

    LOGGER.info("Info message that wont display!");

    LOGGER.set_log_level(LogLevel::Info);

    LOGGER.info("LogLevel set to Info!");
    LOGGER.warn("Warn message!");

    LOGGER.log(LogLevel::Info, "This is an informative message.");

    // * Or using macros

    info!("Info message using macro");

    LOGGER.set_log_level(LogLevel::Error);
    LOGGER.info("LogLevel set to Error!");

    warn!("Warn using macro that wont display!");
    error!("Error using macro");

    LOGGER.set_log_level(LogLevel::Info);
    LOGGER.info("LogLevel set to Info!");
    LOGGER.debug("Debug message!");

    thread::sleep(time::Duration::from_secs(2));

    info!("Hello World!");

    let test_vec = TestVec {
        array: vec!["Hello".to_string(), "World!".to_string()],
    };

    info!(dbg!(format!("{:?}", test_vec)));
    warn!("Whoops!");
    debug!("Debug message via macro!");
    info!("Hello World!");
}
