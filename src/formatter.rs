use crate::record::LogRecord;

pub trait LogFormatter {
    fn format(&self, record: LogRecord) -> String;
}
