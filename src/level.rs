use serde::Serialize;


/// Represents the severity level of a log message.
///
/// `Level` is commonly used to categorize log messages from
/// least to most critical. It derives `Debug`, `PartialEq`,
/// `PartialOrd`, and `Serialize` for easy comparison, sorting,
/// and serialization.
#[derive(Debug, PartialEq, PartialOrd,Serialize)]
pub enum Level{
    Debug,
    Info,
    Warn,
    Error,
    Fatal
}
