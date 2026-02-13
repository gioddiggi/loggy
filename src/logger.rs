use crate::level::Level;
use std::process;

pub struct Logger {
    level : Level
}

impl Logger {
    pub fn new(level:Level) -> Self {
        Self{ level }
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

    fn log(&self, level:Level, message : &str){
        if self.level <= level {
            println!("[{:?}] {}", level, message);
        }
    }
}