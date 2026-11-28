//! this modules defines the error type and error messages returned from the commandline 
//! when things go wrong 

use crate::kind::ErrorKind;
use thiserror::Error;





#[derive(Debug,Error)]
pub enum BalancerError{
    #[error("the backend is not found")]
    NotFound,

    #[error("no healthy backends are available")]
    NoHealthyBackends,

    #[error("backend registry is empty")]
    RegistryEmpty,

    #[error("configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("proxy error: {0}")]
    ProxyError(String),

}

impl BalancerError {
    // this function maps the kind of errors to the error kind/
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::NotFound => ErrorKind::Notfound,
            Self::NoHealthyBackends => ErrorKind::NoBackend,
            Self::IoError(_) => ErrorKind::Io,
            Self::ProxyError(_) => ErrorKind::Proxy,
            Self::RegistryEmpty => ErrorKind::NoBackend,
            Self::ConfigError(_) => ErrorKind::Config,
        }

    }
    
}