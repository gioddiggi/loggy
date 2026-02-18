/// The `formatter` module provides log formatting utilities.
///
/// This module includes concrete formatters (`JsonFormatter` and `PlainTextFormatter`)
/// and the `LogFormatter` trait for custom formatting of log records.

pub mod json;
pub mod plain_text;

pub use json::JsonFormatter;
pub use plain_text::PlainTextFormatter;

use crate::record::LogRecord;

/// Trait for formatting log records into strings.
///
/// Implementors of this trait define how `LogRecord`s are
/// converted into textual representations for output.
///
/// # Example
///
/// ```
/// struct SimpleFormatter;
///
/// impl LogFormatter for SimpleFormatter {
///     fn format(&self, record: LogRecord) -> String {
///         format!("[{}] {}: {}", record.timestamp, record.level as u8, record.message)
///     }
/// }
/// ```
pub trait LogFormatter {
    /// Formats a `LogRecord` into a `String` for output.
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to format.
    ///
    /// # Returns
    ///
    /// A `String` representing the formatted log message.
    fn format(&self, record: LogRecord) -> String;
}
