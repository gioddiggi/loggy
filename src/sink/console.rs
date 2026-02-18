use crate::sink::Sink;

/// A sink that prints log messages to standard output (the terminal).
///
/// `Console` is zero-sized and thread-safe because it leverages Rust's
/// internal locking of `stdout` via the `println!` macro.
#[derive(Debug, Default)]
pub struct Console;

impl Console {
    /// Creates a new instance of the `Console` sink.
    pub fn new() -> Self {
        Self
    }
}

impl Sink for Console {
    /// Writes a message to the console followed by a newline.
    ///
    /// This implementation is synchronized; multiple threads can call `write`
    /// safely without interleaving characters, though the order of messages 
    /// from different threads is not guaranteed.
    ///
    /// # Examples
    ///
    /// ```
    /// use loggy::sinks::{Console, Sink};
    /// let sink = Console::new();
    /// sink.write("System initialized.");
    /// ```
    fn write(&self, msg: &str) {
        println!("{}", msg);
    }
}