use thiserror::Error;


#[derive(Debug,Error)]
pub enum BalancerError{
    #[error("the backend is not found")]
    NotFound,
}