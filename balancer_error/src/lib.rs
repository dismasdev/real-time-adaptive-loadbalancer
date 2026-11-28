pub mod kind;
pub mod error;
pub mod result;
pub  mod code;
pub  mod context;
pub mod conversion;
pub mod tests;


// all the error are in dedicated crate to ensure the whole system has a unified
// error thus the project turns to be low coupled and high cohesion
// as a result when a new component is about to be added it's easy to reuse the errors
