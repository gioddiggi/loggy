use crate::level::Level;
use std::process;

pub struct Logger {
    level : Level
}


impl Logger {
    pub fn new(level:Level) -> Self {
        return Self{ level };
    }

    pub fn debug(&self, message:&str){
        self.log(Level::Debug, message);
    }
    pub fn info(&self, message:&str){
        self.log(Level::Info, message);
    }
    pub fn warn(&self, message:&str){
        self.log(Level::Warn, message);
    }
    pub fn error(&self, message:&str){
        self.log(Level::Error, message);
    }
    pub fn fatal(&self, message:&str){
        self.log(Level::Fatal, message);
        process::exit(1);
    }

    fn log(&self, level:Level, message : &str){
        if self.level <= level {
            println!("[{:?}] {}", level, message);
        }
    }
}
