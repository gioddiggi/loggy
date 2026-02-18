use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::sink::Sink;
use std::process;
use crate::formatter::Formatter;


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


/// A generic, configurable logger.
///
/// `Logger` orchestrates the logging process by filtering messages by [`Level`],
/// formatting them using a [`Formatter`], and sending the final string to a [`Sink`].
pub struct Logger<S: Sink, F: Formatter> {
    min_level: Level,
    sink: S,
    formatter: F,
}

impl<S: Sink, F: Formatter> Logger<S, F> {
    /// Creates a new `Logger` instance.
    pub fn new(min_level: Level, sink: S, formatter: F) -> Self {
        Self {
            min_level,
            sink,
            formatter,
        }
    }

    /// Logs a message at the `Debug` level.
    pub fn debug(&self, message: impl AsRef<str>) {
        self.dispatch(Level::Debug, message.as_ref());
    }

    /// Logs a message at the `Info` level.
    pub fn info(&self, message: impl AsRef<str>) {
        self.dispatch(Level::Info, message.as_ref());
    }

    /// Logs a message at the `Warn` level.
    pub fn warn(&self, message: impl AsRef<str>) {
        self.dispatch(Level::Warn, message.as_ref());
    }

    /// Logs a message at the `Error` level.
    pub fn error(&self, message: impl AsRef<str>) {
        self.dispatch(Level::Error, message.as_ref());
    }

    /// Logs a message at the `Fatal` level and terminates the process.
    pub fn fatal(&self, message: impl AsRef<str>) {
        self.dispatch(Level::Fatal, message.as_ref());
        process::exit(1);
    }

    /// Private helper to filter, format, and write the log message.
    ///
    /// This ensures that expensive formatting and I/O only occur if the
    /// message meets the minimum severity threshold.
    fn dispatch(&self, level: Level, message: &str) {
        if level >= self.min_level {
            // LogRecord is created here to capture the timestamp at the moment of logging
            let record = LogRecord::new(level, message);
            let formatted_msg = self.formatter.format(&record);
            self.sink.write(&formatted_msg);
        }
    }
}