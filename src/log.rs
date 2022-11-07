//! Logging utilities.

/// How much logging to provide to the user
#[allow(dead_code)]
pub enum LogLevel {
    /// Do not print any information.
    Silent,
    /// Print only steps that contain errors.
    Error,
    /// Print any steps that possibly contain some errors.
    Warning,
    /// Print progress of all steps.
    Info,
    /// Print all output of all steps and what they do.
    Verbose,
}
