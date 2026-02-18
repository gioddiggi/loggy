use crate::output::LogOutput;
use crate::record::LogRecord;

pub struct ConsoleLogger{}

impl LogOutput for ConsoleLogger{
    fn log(&self, msg: &str) {
        println!("{}",msg);
    }
}