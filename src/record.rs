use core::fmt;

use chrono::{DateTime, Utc};

use crate::level::Level;
pub struct LogRecord<'a> {
    pub level: Level,
    pub message: &'a str,
    pub timestamp: DateTime<Utc>
,   
}


impl<'a> fmt::Display for LogRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {:?}: {}",
            self.timestamp.to_rfc3339(),
            self.level,
            self.message
        )
    }
}
