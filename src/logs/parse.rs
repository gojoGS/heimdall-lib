use crate::logs::model::LogLevel;

use super::model::LogEntry;

use lazy_static::lazy_static;
use log::debug;
use melody_compiler::compiler;
use regex::Regex;

const DEFAULT_LOG_REGEX: &str = r#"
<start>;

lazy any of <char>;

capture level {
  either {"TRACE"; "DEBUG"; "INFO"; "WARNING"; "ERROR"; "FATAL";}
}

lazy any of <char>;

<end>;
"#;

pub struct LogParser {}

impl LogParser {
    pub fn parse<'a, S>(log: S) -> Vec<LogEntry>
    where
        S: Into<String>,
    {
        let lines: Vec<String> = log.into().lines().map(|l| l.to_string()).collect();
        let mut log_entries: Vec<LogEntry> = Vec::with_capacity(16);

        lazy_static! {
            static ref REGEX_STR: String = compiler(DEFAULT_LOG_REGEX).unwrap();
            static ref REGEX: Regex = Regex::new(&REGEX_STR).unwrap();
        }

        for line in lines {
            let capture = REGEX.captures(&line);

            if capture.is_none() {
                log_entries.push(LogEntry::new(line.to_string(), LogLevel::None));
                continue;
            }

            let level = match capture.unwrap().name("level") {
                Some(level_string) => LogLevel::from_string(level_string),
                None => LogLevel::None,
            };

            let log_entry = LogEntry::new(line.to_string(), level);
            debug!("Created log entry: {:?}", log_entry);
            log_entries.push(log_entry)
        }

        log_entries
    }
}
