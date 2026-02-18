use crate::formatter::Formatter;
use crate::core::Level;
use crate::core::LogRecord;
use crate::sink::Sink;
use std::process;

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