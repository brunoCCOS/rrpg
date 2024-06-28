use core::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct LogMessage {
    level: LogLevel,
    message: String,
    timestamp: u64,
}

impl LogMessage {
    fn new(level: LogLevel, message: &str) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        LogMessage {
            level,
            message: message.to_string(),
            timestamp: since_the_epoch.as_secs(),
        }
    }

    fn printlog(&self) {
        println!("{} {}: {} ", self.level, self.timestamp, self.message)
    }
}

pub struct Logger {
    messages: Vec<LogMessage>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            messages: Vec::new(),
        }
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        let log_message = LogMessage::new(level, message);
        log_message.printlog();
        self.messages.push(log_message);
    }

    fn debug(&mut self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    fn info(&mut self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    fn warning(&mut self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    fn error(&mut self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    fn critical(&mut self, message: &str) {
        self.log(LogLevel::Critical, message);
    }

    fn display(&self) {
        for msg in &self.messages {
            msg.printlog();
        }
    }
}
