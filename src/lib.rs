pub mod core;      // Contains Level and LogRecord
pub mod sink;      // Contains Sink trait and impls
pub mod formatter; // Contains Formatter trait and impls
pub mod logger;    // The main Logger engine

pub use crate::core::{Level, LogRecord};
pub use crate::logger::Logger;