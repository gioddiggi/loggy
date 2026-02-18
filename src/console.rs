use crate::output::LogOutput;
/// `ConsoleLogger` implements the `LogOutput` trait and prints
/// log messages using `println!`. This logger is inherently thread-safe
/// for typical use because `println!` handles stdout locking internally.
pub struct ConsoleLogger{}

impl LogOutput for ConsoleLogger{
    /// Logs a message to the console.
    ///
    /// This method simply prints the message to standard output,
    /// appending a newline automatically.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to print to the console.
    ///
    /// # Example
    ///
    /// ```
    /// let logger = ConsoleLogger {};
    /// logger.log("Hello, world!");
    /// ```
    fn log(&self, msg: &str) {
        println!("{}",msg);
    }
}