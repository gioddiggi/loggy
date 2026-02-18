use crate::formatter::LogFormatter;
use crate::level::Level;
use crate::output::LogOutput;
use crate::record::LogRecord;
use std::process;

/// A generic, configurable logger.
///
/// `Logger` allows logging messages at different severity levels
/// (`Debug`, `Info`, `Warn`, `Error`, `Fatal`) to a customizable output
/// with a customizable formatter. The logger only emits messages
/// at or above its configured level.
///
/// # Type Parameters
///
/// * `O` - The output type implementing `LogOutput`, e.g., `ConsoleLogger` or `FileLogger`.
/// * `F` - The formatter type implementing `LogFormatter`, responsible for formatting log records.
pub struct Logger<O: LogOutput, F: LogFormatter> {
    level: Level,
    formatter: F,
    output: O,
}

impl<O: LogOutput, F: LogFormatter> Logger<O, F> {
    /// Creates a new `Logger`.
    ///
    /// # Arguments
    ///
    /// * `level` - The minimum `Level` of messages to log. Messages below this level are ignored.
    /// * `output` - The log output, e.g., console, file, or other sinks.
    /// * `formatter` - The log formatter used to format `LogRecord`s before output.
    ///
    /// # Example
    ///
    /// ```
    /// let logger = Logger::new(Level::Info, ConsoleLogger {}, SimpleFormatter {});
    /// ```
    pub fn new(level: Level, output: O, formatter: F) -> Self {
        Self { level, output, formatter }
    }

    /// Logs a debug-level message.
    pub fn debug(&self, message: impl AsRef<str>) {
        self.log(Level::Debug, message.as_ref());
    }

    /// Logs an info-level message.
    pub fn info(&self, message: impl AsRef<str>) {
        self.log(Level::Info, message.as_ref());
    }

    /// Logs a warning-level message.
    pub fn warn(&self, message: impl AsRef<str>) {
        self.log(Level::Warn, message.as_ref());
    }

    /// Logs an error-level message.
    pub fn error(&self, message: impl AsRef<str>) {
        self.log(Level::Error, message.as_ref());
    }

    /// Logs a fatal-level message and immediately exits the program with status code 1.
    ///
    /// # Note
    ///
    /// This method does not return; it calls `process::exit(1)` after logging.
    pub fn fatal(&self, message: impl AsRef<str>) {
        self.log(Level::Fatal, message.as_ref());
        process::exit(1);
    }

    /// Internal method that handles logging a message if its level is >= the configured logger level.
    ///
    /// # Arguments
    ///
    /// * `level` - The severity level of the message.
    /// * `message` - The message content.
    fn log(&self, level: Level, message: &str) {
        if level >= self.level {
            let record = LogRecord {
                level,
                message,
                timestamp: chrono::Utc::now(),
            };
            self.output.log(self.formatter.format(record).as_str());
        }
    }
}
