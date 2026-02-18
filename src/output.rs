use crate::{record::LogRecord};
pub trait LogOutput {
    fn log(&self, record:LogRecord);
}

