use crate::level::Level;
use crate::output::LogOutput;

pub struct ConsoleLogger{}

impl LogOutput for ConsoleLogger{
    fn log(&self, level: Level, timestamp: &str, message: &str) {
        println!("{} [{:?}] {}", timestamp, level, message);
    }
}