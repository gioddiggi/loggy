use crate::formatter::LogFormatter;
use crate::record::LogRecord;

pub struct PlainTextFormatter;

impl LogFormatter for PlainTextFormatter {
    fn format(&self, record: LogRecord) -> String {
        format!(
            "[{}] {:?}: {}",
            record.timestamp.to_rfc3339(),
            record.level,
            record.message
        )
    }
}
