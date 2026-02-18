use crate::formatter::Formatter;
use super::LogRecord;

/// A log formatter that outputs log records in plain text format.
///
/// Formats a `LogRecord` into a human-readable string with:
/// - Timestamp in RFC3339
/// - Uppercase log level
/// - Message
/// - Extras formatted as `[key=value ...]`
///
/// # Example
///
/// ```rust,ignore
/// let formatter = PlainTextFormatter;
/// let record = LogRecord {
///     level: Level::Info,
///     message: "Application started",
///     timestamp: chrono::Utc::now(),
///     extras: std::collections::HashMap::new(),
/// };
/// let output = formatter.format(&record);
/// println!("{}", output);
/// // Example: "[2026-02-18T12:34:56+00:00] INFO: Application started"
/// ```
pub struct PlainTextFormatter;

impl Formatter for PlainTextFormatter {
    fn format(&self, record: &LogRecord) -> String {
        // Convert extras to key=value pairs
        let extras_str = if record.extras.is_empty() {
            "".to_string()
        } else {
            let pairs: Vec<String> = record
                .extras
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            format!(" [{}]", pairs.join(" "))
        };

        format!(
            "[{}] {}: {}{}",
            record.timestamp.to_rfc3339(),
            record.level.to_string().to_uppercase(),
            record.message,
            extras_str
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use chrono::DateTime;
    use crate::formatter:: {Formatter, PlainTextFormatter};


    const FORMATTER : PlainTextFormatter = PlainTextFormatter;

    #[test]
    fn format_sample_record(){
        let extras = HashMap::from([("service".to_string(), "logging-service".to_string())]);
        let record = crate::LogRecord { 
            level: crate::Level::Info, 
            message: "Hello world", 
            timestamp: DateTime::from_timestamp_millis(1771438594775).unwrap(), 
            extras
        };
        let result = FORMATTER.format(&record);
        let expected = "[2026-02-18T18:16:34.775+00:00] INFO: Hello world [service=logging-service]";
        assert_eq!(result, expected)
    }
}