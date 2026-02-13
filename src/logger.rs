use crate::level::Level;
use crate::output::LogOutput;
use std::process;

pub struct Logger<O: LogOutput> {
    level : Level,
    output : O,
}

impl <O: LogOutput> Logger<O> {
    pub fn new(level : Level, output : O) -> Self {
        Self{ level, output }
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
        if self.level <= level {
            self.output.log(level, message);
        }
    }
}