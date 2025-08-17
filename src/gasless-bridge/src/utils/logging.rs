use ic_cdk::api::time;

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct Logger;

impl Logger {
    pub fn debug(message: &str) {
        Self::log(LogLevel::Debug, message);
    }
    
    pub fn info(message: &str) {
        Self::log(LogLevel::Info, message);
    }
    
    pub fn warn(message: &str) {
        Self::log(LogLevel::Warn, message);
    }
    
    pub fn error(message: &str) {
        Self::log(LogLevel::Error, message);
    }
    
    fn log(level: LogLevel, message: &str) {
        let timestamp = time();
        let level_str = match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };
        
        ic_cdk::println!("[{}] {}: {}", timestamp, level_str, message);
    }
}

// Convenience macros
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        crate::utils::logging::Logger::debug(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        crate::utils::logging::Logger::info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        crate::utils::logging::Logger::warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        crate::utils::logging::Logger::error(&format!($($arg)*))
    };
}
