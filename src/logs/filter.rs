use super::model::{LogEntry, LogLevel};

pub struct LevelLogFilter {
    minimum_level: LogLevel,
}

impl LevelLogFilter {
    pub fn new(minimum_level: LogLevel) -> Self {
        LevelLogFilter { minimum_level }
    }

    pub fn filter<'a>(
        &'a self,
        logs: std::slice::Iter<'a, LogEntry>,
    ) -> impl Iterator<Item = &'a LogEntry> + '_ {
        logs.filter(move |l| l.level >= self.minimum_level)
    }
}
