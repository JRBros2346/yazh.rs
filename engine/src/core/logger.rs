use std::fmt;

pub const WARN: bool = true;
pub const INFO: bool = true;
#[cfg(debug_assertions)]
pub const DEBUG: bool = true;
#[cfg(debug_assertions)]
pub const TRACE: bool = true;

// Disable debug and trace logging for release build.
#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;
#[cfg(not(debug_assertions))]
pub const TRACE: bool = false;

#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
impl fmt::Display for LogLevel {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "[ {:>5} ]",
            match self {
                Self::Fatal => "Fatal",
                Self::Error => "Error",
                Self::Warn => "Warn",
                Self::Info => "Info",
                Self::Debug => "Debug",
                Self::Trace => "Trace",
            }
        )
    }
}

// TODO: Remove this.
#[allow(dead_code)]
fn initialize() -> bool {
    // TODO: create log file.
    true
}

// TODO: Remove this.
#[allow(dead_code)]
fn shutdown() {
    // TODO: cleanup logging/write queued entries.
}

#[macro_export]
macro_rules! logoutput {
    ($level:expr, $format:expr, $($format_args:expr),*) => {
        let is_error = $level < $crate::core::logger::LogLevel::Warn;

        // TODO: Platform specific
        if is_error {
            std::eprintln!("{}: {}", $level, std::format!($format, $($format_args),*));
        } else {
            std::println!("{}: {}", $level, std::format!($format, $($format_args),*));
        }
    };
}
pub use logoutput;

// Logs a fatal-level message
#[macro_export(local_inner_macros)]
macro_rules! LogFatal {
    ($format:expr, $($format_args:expr),*) => {
        logoutput!($crate::core::logger::LogLevel::Fatal, $format, $($format_args),*);
    };
}

// Logs a error-level message
#[macro_export(local_inner_macros)]
macro_rules! LogError {
    ($format:expr, $($format_args:expr),*) => {
        logoutput!($crate::core::logger::LogLevel::Error, $format, $($format_args),*);
    };
}

// Logs a warn-level message
#[macro_export(local_inner_macros)]
macro_rules! LogWarn {
    ($format:expr, $($format_args:expr),*) => {
        if $crate::core::logger::WARN {
            logoutput!($crate::core::logger::LogLevel::Warn, $format, $($format_args),*);
        }
    };
}

// Logs a info-level message
#[macro_export(local_inner_macros)]
macro_rules! LogInfo {
    ($format:expr, $($format_args:expr),*) => {
        if $crate::core::logger::INFO {
            logoutput!($crate::core::logger::LogLevel::Info, $format, $($format_args),*);
        }
    };
}

// Logs a debug-level message
#[macro_export(local_inner_macros)]
macro_rules! LogDebug {
    ($format:expr, $($format_args:expr),*) => {
        if $crate::core::logger::DEBUG {
            logoutput!($crate::core::logger::LogLevel::Debug, $format, $($format_args),*);
        }
    };
}

// Logs a trace-level message
#[macro_export(local_inner_macros)]
macro_rules! LogTrace {
    ($format:expr, $($format_args:expr),*) => {
        if $crate::core::logger::TRACE {
            let level = $crate::core::logger::LogLevel::Trace;
            logoutput!(level, $format, $($format_args),*);
        }
    };
}

pub use {LogDebug, LogError, LogFatal, LogInfo, LogTrace, LogWarn};
