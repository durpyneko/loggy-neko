use loggy::{error, info, log, set_log_level, warn, LogLevel};

#[derive(Debug)]
struct TestVec {
    array: Vec<String>,
}

// Example
fn test() {
    set_log_level(LogLevel::Info);

    info("Log level set to Info. Will log: Info, Warn, Error");

    let test_vec = TestVec {
        array: vec!["Hello".to_string(), "World!".to_string()],
    };

    info(dbg!(format!("{:?}", test_vec).as_str()));

    set_log_level(LogLevel::Warn);
    warn("Log level set to Warn. Will log: Warn, Error");
    info("This Info won't log");
    error("Error message.")
}

fn main() {
    set_log_level(LogLevel::Error);

    log(LogLevel::Error, "Log level set to Error. Only errors log");
    log(LogLevel::Info, "This Info won't log.");
    log(LogLevel::Warn, "This Warn won't log.");

    test();
}
