#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    None,
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn from_string<'a, S>(level: S) -> Self
    where
        S: Into<&'a str>,
    {
        match level.into() {
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARNING" => LogLevel::Warning,
            "ERROR" => LogLevel::Error,
            _ => LogLevel::None,
        }
    }
}

#[derive(Debug)]
pub struct LogEntry {
    pub message: String,
    pub level: LogLevel,
}

impl LogEntry {
    pub fn new<S>(msg: S, level: LogLevel) -> LogEntry
    where
        S: Into<String>,
    {
        LogEntry {
            message: msg.into(),
            level,
        }
    }
}
