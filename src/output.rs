use crate::level::Level;
pub trait LogOutput {
    fn log(&self, level: Level, timestamp: &str, message: &str);
}

