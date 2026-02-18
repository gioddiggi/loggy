use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::level::Level;

/// Represents a single log entry.
///
/// `LogRecord` stores all information about a single log message,
/// including its severity level, content, and timestamp. It is serializable
/// via `serde` for use in structured logging (e.g., JSON output).
///
/// # Type Parameters
///
/// * `'a` - Lifetime of the message string slice.
#[derive(Serialize)]
pub struct LogRecord<'a> {
    /// The severity level of the log message.
    pub level: Level,

    /// The content of the log message.
    pub message: &'a str,

    /// The timestamp when the log message was created.
    /// Uses UTC time via `chrono::Utc`.
    pub timestamp: DateTime<Utc>,
}
