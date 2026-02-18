/// Trait representing a logging output.
///
/// Types implementing `LogOutput` define how log messages are written
/// or sent to a destination, such as the console, a file, or a network sink.
///
/// # Examples
///
/// **Console output implementation:**
/// ```
/// struct ConsoleLogger;
///
/// impl LogOutput for ConsoleLogger {
///     fn log(&self, msg: &str) {
///         println!("{}", msg);
///     }
/// }
/// ```
///
/// **File output implementation (simplified):**
/// ```
/// use std::fs::OpenOptions;
/// use std::io::Write;
///
/// struct FileLogger {
///     file: std::fs::File,
/// }
///
/// impl LogOutput for FileLogger {
///     fn log(&self, msg: &str) {
///         let _ = writeln!(&self.file, "{}", msg);
///     }
/// }
/// ```
pub trait LogOutput {
    /// Writes a log message to the output destination.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to be logged.
    fn log(&self, msg: &str);
}
