use serde::Serialize;

#[derive(Debug, PartialEq, PartialOrd,Serialize)]
pub enum Level{
    Debug,
    Info,
    Warn,
    Error,
    Fatal
}
