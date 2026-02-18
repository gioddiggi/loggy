pub mod json;
pub mod plain_text;

pub use json::JsonFormatter;
pub use plain_text::PlainTextFormatter;

use crate::record::LogRecord;

pub trait LogFormatter {
    fn format(&self, record: LogRecord) -> String;
}
