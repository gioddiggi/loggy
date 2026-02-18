use crate::formatter::LogFormatter;
use crate::level::Level;
use crate::output::LogOutput;
use crate::record::LogRecord;
use std::process;

pub struct Logger<O: LogOutput, F: LogFormatter> {
    level : Level,
    formatter: F,
    output : O,
}

impl <O: LogOutput, F: LogFormatter> Logger<O, F> {
    pub fn new(level : Level, output : O, formatter : F) -> Self {
        Self{ level, output, formatter}
    }

    pub fn debug(&self, message: impl AsRef<str>){
        self.log(Level::Debug, message.as_ref());
    }
    pub fn info(&self, message: impl AsRef<str>){
        self.log(Level::Info, message.as_ref());
    }
    pub fn warn(&self, message: impl AsRef<str>){
        self.log(Level::Warn, message.as_ref());
    }
    pub fn error(&self, message:impl AsRef<str>){
        self.log(Level::Error, message.as_ref());
    }
    pub fn fatal(&self, message:impl AsRef<str>){
        self.log(Level::Fatal, message.as_ref());
        process::exit(1);
    }

    fn log(&self, level : Level, message:&str){
        if level >= self.level {
            let record = LogRecord {
                level,
                message,
                timestamp: chrono::Utc::now(),
            };
            self.output.log(self.formatter.format(record).as_str());
        }
    }
}