//! # Sinks
//! 
//! Sinks are the final destinations for log records. 
//! 
//! This module provides the [`Sink`] trait and several standard implementations 
//! like [`Console`] and [`File`].
//!
//! 

pub mod console;
pub mod file;

// Re-exporting for a flatter, more ergonomic API
pub use self::console::Console;
pub use self::file::File;

/// A trait representing a destination where log messages can be written.
///
/// Implementors of this trait handle the low-level details of writing 
/// to a specific output, such as a terminal, a file, or a network socket.
pub trait Sink: Send + Sync {
    /// Writes a formatted message string to the destination.
    ///
    /// # Errors
    ///
    /// If the underlying write operation fails (e.g., disk is full, 
    /// permission denied), the implementation should handle the error 
    /// according to the library's error policy.
    fn write(&self, message: &str);
}