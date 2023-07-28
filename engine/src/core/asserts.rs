use crate::defines::*;

use super::logger;

// Disable assertions by commenting out the below line.
pub const ASSERTIONS: bool = true;

pub fn report_assertion_failure(expression: &str, message: &str, file: &str, line: u32) {
    logger::logoutput!(logger::LogLevel::Fatal, "Assertion Failure: {}, message: '{}', in file: {}, line: {}", expression, message, file, line);
}

pub fn debug_break() {

}

#[macro_export]
macro_rules! Assert {
    ($expr:expr) => {
        if $crate::core::asserts::ASSERTIONS {
            if $expr {
            } else {
                $crate::core::asserts::report_assertion_failure(std::stringify!($expr), "", std::file!(), std::line!());
                $crate::core::asserts::debug_break();
            }
        }
    };
    ($expr:expr, $message:expr) => {
        if $crate::core::asserts::ASSERTIONS {
            if $expr {
            } else {
                $crate::core::asserts::report_assertion_failure(std::stringify!($expr), $message, std::file!(), std::line!());
                $crate::core::asserts::debug_break();
            }
        }
    };
}

#[cfg_attr(debug_assertions, macro_export)]
macro_rules! AssertDebug {
    ($expr:expr) => {
        if $crate::core::asserts::ASSERTIONS {
            if $expr {
            } else {
                $crate::core::asserts::report_assertion_failure(std::stringify!($expr), "", std::file!(), std::line!());
                $crate::core::asserts::debug_break();
            }
        }
    };
    ($expr:expr, $message:expr) => {
        if $crate::core::asserts::ASSERTIONS {
            if $expr {
            } else {
                $crate::core::asserts::report_assertion_failure($expr, $message, std::file!(), std::line!());
                $crate::core::asserts::debug_break();
            }
        }
    };
}
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! AssertDebug {
    ($($args:expr),*) => {
        
    };
}
pub use {Assert, AssertDebug};
