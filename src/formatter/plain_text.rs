use crate::formatter::Formatter;
use super::LogRecord;

/// A log formatter that outputs log records in plain text format.
///
/// `PlainTextFormatter` formats a `LogRecord` into a human-readable string
/// with the timestamp, log level, and message.
///
/// # Example
///
/// ```
/// let formatter = PlainTextFormatter;
/// let record = LogRecord {
///     level: Level::Info,
///     message: "Application started",
///     timestamp: chrono::Utc::now(),
/// };
/// let output = formatter.format(record);
/// println!("{}", output);
/// // Example output: "[2026-02-18T12:34:56+00:00] Info: Application started"
/// ```
pub struct PlainTextFormatter;

impl Formatter for PlainTextFormatter {
    /// Formats a `LogRecord` as a plain text string.
    ///
    /// The output format is:
    /// `[timestamp] level: message`
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to format.
    ///
    /// # Returns
    ///
    /// A `String` containing the log record in plain text format.
    fn format(&self, record: &LogRecord) -> String {
        format!(
            "[{}] {:?}: {}",
            record.timestamp.to_rfc3339(),
            record.level,
            record.message
        )
    }
}
