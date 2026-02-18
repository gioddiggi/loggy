use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::{ level::Level};


#[derive(Serialize)]
pub struct LogRecord<'a> {
    pub level: Level,
    pub message: &'a str,
    pub timestamp: DateTime<Utc>,   
}

