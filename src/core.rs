use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::sink::Sink;
use std::fmt::{self, Display};
use std::process;
use std::collections::HashMap;
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
    pub extras : HashMap<String, String>
}


impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Level::Debug => "DEBUG",
            Level::Info  => "INFO",
            Level::Warn  => "WARN",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        };
        write!(f, "{}", s)
    }
}

impl<'a> LogRecord<'a> {
    /// Creates a new `LogRecord` with the current UTC timestamp.
    pub fn new(level: Level, message: &'a str, extras : HashMap<String, String>) -> Self {
        Self {
            level,
            message,
            timestamp: Utc::now(),
            extras
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
    extras: HashMap<String, String>
}

impl<S: Sink, F: Formatter> Logger<S, F> {
    /// Creates a new `Logger` instance.
    pub fn new(min_level: Level, sink: S, formatter: F) -> Self {
        Self {
            min_level,
            sink,
            formatter,
            extras: HashMap::new()
        }
    }


    /// Adds an extra key–value pair to the logger.
    ///
    /// Both the key and value can be any type that can be converted into a
    /// [`String`], such as `&str`, `String`, or the result of `format!`.
    ///
    /// This method takes ownership of the provided values. If a `String` is
    /// passed, it will be moved without additional allocation. If a `&str` is
    /// passed, it will be copied into a new `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut logger = Logger::new();
    ///
    /// logger.add_extra("user_id", "42");
    ///
    /// let request_id = String::from("abc-123");
    /// logger.add_extra("request_id", request_id);
    ///
    /// logger.add_extra("duration_ms", format!("{}", 150));
    /// ```
    pub fn add_extra(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.extras.insert(k.into(), v.into());
    }

    /// Removes an extra field from the logger by key.
    ///
    /// If the key exists in the `extras` map, the corresponding
    /// entry is removed. If the key does not exist, this method
    /// does nothing.
    ///
    /// # Arguments
    ///
    /// * `k` - The key of the extra field to remove.
    ///
    /// # Example
    ///
    /// ```
    /// logger.remove_extra("request_id");
    /// ```
    pub fn remove_extra(&mut self, k: &str){
        self.extras.remove(k);
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
            let record = LogRecord::new(level, message, self.extras.clone());
            let formatted_msg = self.formatter.format(&record);
            self.sink.write(&formatted_msg);
        }
    }
}