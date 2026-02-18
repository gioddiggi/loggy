use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::sync::Mutex;

use crate::output::LogOutput;
use crate::record::LogRecord;

pub struct FileLogger {
    file: Mutex<BufWriter<File>>,
}

impl FileLogger {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;

        Ok(Self {
            file: Mutex::new(BufWriter::new(file)),
        })
    }
}

impl LogOutput for FileLogger {
    fn log(&self, record : LogRecord) {
        let mut file = match self.file.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let _ = writeln!(file, "{} [{:?}] {}", record.timestamp, record.level, record.message);
    }
}
