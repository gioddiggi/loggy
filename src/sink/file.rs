use std::fs::{File as StdFile, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::sync::Mutex;
use crate::sink::Sink;

/// A thread-safe sink that appends log messages to a file.
///
/// `File` uses a [`Mutex`] to synchronize writes from multiple threads and a
/// [`BufWriter`] to minimize expensive system calls to the disk.
pub struct File {
    inner: Mutex<BufWriter<StdFile>>,
}

impl File {
    /// Creates a new `File` sink at the specified path.
    /// 
    /// If the file exists, messages are appended to the end. If it does not exist,
    /// it will be created.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the file cannot be opened or created with 
    /// append permissions.
    pub fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;

        Ok(Self {
            inner: Mutex::new(BufWriter::new(file)),
        })
    }
}

impl Sink for File {
    /// Writes a message to the file followed by a newline.
    ///
    /// This method acquires a lock on the internal [`Mutex`]. If a thread panics 
    /// while holding the lock, this method will recover the "poisoned" lock to 
    /// ensure logging continues.
    fn write(&self, msg: &str) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let _ = writeln!(guard, "{}", msg);
    }
}