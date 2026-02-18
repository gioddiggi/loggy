use crate::output::LogOutput;

pub struct ConsoleLogger{}

impl LogOutput for ConsoleLogger{
    fn log(&self, msg: &str) {
        println!("{}",msg);
    }
}