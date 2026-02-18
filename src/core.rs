use chrono::{DateTime, Utc};
use serde::Serialize;

/// Defines the severity levels for log messages.
/// 
/// Levels are ordered by priority: `Debug` < `Info` < `Warn` < `Error` < `Fatal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

/// A snapshot of a single logging event.
///
/// This struct captures all necessary metadata at the moment of the log call.
/// It uses a lifetime `'a` to borrow the message string, avoiding 
/// unnecessary memory allocations during the logging pipeline.
#[derive(Debug, Serialize)]
pub struct LogRecord<'a> {
    /// The severity level of this record.
    pub level: Level,
    /// The log message content.
    pub message: &'a str,
    /// The exact time (UTC) the record was created.
    pub timestamp: DateTime<Utc>,
}

impl<'a> LogRecord<'a> {
    /// Creates a new `LogRecord` with the current UTC timestamp.
    pub fn new(level: Level, message: &'a str) -> Self {
        Self {
            level,
            message,
            timestamp: Utc::now(),
        }
    }
}