use serde_json::json;
use crate::formatter::LogFormatter;
use super::LogRecord;
pub struct JsonFormatter;

impl<'a> LogFormatter for JsonFormatter {
    fn format(&self, record: LogRecord) -> String {
        let obj = json!({
            "level": format!("{:?}", record.level),
            "message": record.message,
            "timestamp": record.timestamp.to_rfc3339()
        });
        obj.to_string()
    }
}
