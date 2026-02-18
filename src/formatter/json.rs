use serde_json::json;
use crate::formatter::Formatter;
use super::LogRecord;

/// A log formatter that outputs log records in JSON format.
///
/// `JsonFormatter` converts a `LogRecord` into a JSON string
/// with fields for `level`, `message`, and `timestamp`.
///
/// # Example
///
/// ```
/// let formatter = JsonFormatter;
/// let record = LogRecord {
///     level: Level::Info,
///     message: "Hello, world!",
///     timestamp: chrono::Utc::now(),
/// };
/// let json_string = formatter.format(record);
/// println!("{}", json_string);
/// ```
pub struct JsonFormatter;

impl<'a> Formatter for JsonFormatter {
    /// Formats a `LogRecord` as a JSON string.
    ///
    /// The output includes the following fields:
    /// - `"level"`: The log level as a string (e.g., `"Info"`).
    /// - `"message"`: The log message.
    /// - `"timestamp"`: The timestamp in RFC 3339 format.
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to format.
    ///
    /// # Returns
    ///
    /// A `String` containing the log record serialized as JSON.
    fn format(&self, record: &LogRecord) -> String {
        let obj = json!({
            "level": format!("{:?}", record.level),
            "message": record.message,
            "timestamp": record.timestamp.to_rfc3339(),
            "extras" : record.extras
        });
        obj.to_string()
    }
}
