//! this module attached context to the errors
//! this help in error navigation especially when errors bubble
//! up the function stacks
//!  the errorContext struct shows exactly where the error happened.

use core::fmt;

use crate::error::BalancerError;

pub struct ErrorContext {
    pub source: BalancerError,
    pub location: &'static str, // example "selector:select"
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:  {}", self.location, self.source)
    }
}
