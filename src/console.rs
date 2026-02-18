use crate::output::LogOutput;
use crate::record::LogRecord;

pub struct ConsoleLogger{}

impl LogOutput for ConsoleLogger{
    fn log(&self, record: LogRecord) {
        println!("{} [{:?}] {}", record.timestamp, record.level, record.message);
    }
}