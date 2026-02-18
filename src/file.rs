use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::sync::Mutex;

use crate::output::LogOutput;


/// A thread-safe logger that writes log messages to a file.
///
/// `FileLogger` wraps a `BufWriter<File>` in a `Mutex` to allow
/// safe concurrent logging from multiple threads. Each log message
/// is appended to the specified file.
pub struct FileLogger {
    file: Mutex<BufWriter<File>>,
}

impl FileLogger {
    /// Creates a new `FileLogger` that writes to the specified file path.
    ///
    /// If the file does not exist, it will be created. If it exists,
    /// new log messages will be appended.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to write log messages to.
    ///
    /// # Returns
    ///
    /// * `io::Result<FileLogger>` - Returns a `FileLogger` instance on success,
    ///   or an `io::Error` if the file cannot be opened or created.
    ///
    /// # Example
    ///
    /// ```
    /// let logger = FileLogger::new("app.log").unwrap();
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
     /// Logs a message to the file.
    ///
    /// The message will be written with a newline appended. This
    /// method acquires a lock on the internal mutex to ensure
    /// thread safety. If the mutex is poisoned, it recovers and
    /// continues logging.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write to the log file.
    ///
    /// # Example
    ///
    /// ```
    /// logger.log("This is a log message.");
    /// ```
    fn log(&self, msg : &str) {
        let mut file = match self.file.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let _ = writeln!(file, "{}", msg);
    }
}
