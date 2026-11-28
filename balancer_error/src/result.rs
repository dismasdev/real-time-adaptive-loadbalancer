//! this module creates a common Result type that has 
//! return type of Result<T,BalancerError>


use crate::error::BalancerError;
pub type BalancerResult<T> = Result<T,BalancerError>;